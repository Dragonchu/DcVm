use crate::heap::{AllocError, Heap, RawPtr};
use crate::{class_loader::BootstrapClassLoader, jvm_thread::JvmThread, };
use crate::class::Klass;
use crate::error::JvmError;
use crate::JvmValue;
use crate::native_method::{NativeMethodRegistry, NativeMethod};
use crate::jvm_log;
use std::collections::HashMap;
use reader::constant_pool::ConstantPool;
use std::cell::RefCell;
use std::sync::Once;

// 全局VM单例
static mut VM_INSTANCE: Option<Vm> = None;
static INIT: Once = Once::new();

pub struct Vm {
    pub heap: RefCell<Heap>,
    class_loader: RefCell<BootstrapClassLoader>,
    // 静态字段存储: (类名, 字段名) -> 值
    static_fields: HashMap<(String, String), JvmValue>,
    // Native方法注册表
    native_methods: NativeMethodRegistry,
    pub string_builder_map: RefCell<HashMap<crate::heap::RawPtr, String>>,
    pub string_map: RefCell<HashMap<crate::heap::RawPtr, String>>,
}

impl Vm {
    /// 获取全局VM实例
    pub fn instance() -> &'static mut Vm {
        unsafe {
            INIT.call_once(|| {
                VM_INSTANCE = Some(Vm::new("."));
            });
            VM_INSTANCE.as_mut().unwrap()
        }
    }
    
    /// 初始化VM（设置类路径）
    pub fn initialize(class_path: &str) {
        unsafe {
            INIT.call_once(|| {
                VM_INSTANCE = Some(Vm::new(class_path));
            });
        }
    }

    pub fn new(paths: &str) -> Vm {
        Vm {
            class_loader: RefCell::new(BootstrapClassLoader::new(paths)),
            heap: RefCell::new(Heap::with_maximum_memory(1024 * 1024)),
            static_fields: HashMap::new(),
            native_methods: NativeMethodRegistry::new(),
            string_builder_map: RefCell::new(HashMap::new()),
            string_map: RefCell::new(HashMap::new()),
        }
    }
    
    pub fn load(&mut self, class_name: &str) -> Result<Klass, JvmError> {
        // 先获取self裸指针
        let self_ptr = self as *mut Vm;
        // 先借用class_loader和heap，load一次，clone结果
        let (klass, class_loader_ptr, heap_ptr): (Klass, *mut crate::class_loader::BootstrapClassLoader, *mut crate::heap::Heap);
        {
            let mut class_loader = self.class_loader.borrow_mut();
            let mut heap = self.heap.borrow_mut();
            klass = class_loader.load(class_name, &mut heap)?.clone();
            class_loader_ptr = &mut *class_loader as *mut crate::class_loader::BootstrapClassLoader;
            heap_ptr = &mut *heap as *mut crate::heap::Heap;
        }
        // 用裸指针调用initialize_class，避免self多重借用
        unsafe {
            (*class_loader_ptr).initialize_class(class_name, &mut *heap_ptr, Some(&mut *self_ptr))?;
        }
        Ok(klass)
    }

    pub fn alloc_array(&mut self, klass: &Klass, length: usize) -> Result<RawPtr, AllocError> {
        match klass {
            crate::class::Klass::Array(k) => self.heap.borrow_mut().alloc_array(k, length),
            _ => Err(AllocError::BadRequest),
        }
    }
    
    pub fn alloc_object(&mut self, klass: &Klass) -> Result<RawPtr, AllocError> {
        match klass {
            crate::class::Klass::Instance(k) => self.heap.borrow_mut().alloc_object(k),
            _ => Err(AllocError::BadRequest),
        }
    }
    
    /// 设置静态字段值
    pub fn set_static_field(&mut self, class_name: &str, field_name: &str, value: JvmValue) {
        self.static_fields.insert((class_name.to_string(), field_name.to_string()), value);
    }
    
    /// 获取静态字段值
    pub fn get_static_field(&self, class_name: &str, field_name: &str) -> Option<JvmValue> {
        self.static_fields.get(&(class_name.to_string(), field_name.to_string())).cloned()
    }
    
    /// 调用native方法
    pub fn call_native_method(&mut self, class_name: &str, method_name: &str, args: Vec<JvmValue>) -> Result<Option<JvmValue>, JvmError> {
        let full_name = format!("{}.{}", class_name, method_name);
        jvm_log!("[Native] call_native_method key: {}", full_name);
        // 先取出方法引用，避免self多重借用
        let native_method = self.native_methods.get(&full_name).map(|m| &**m as *const dyn NativeMethod);
        if let Some(native_method_ptr) = native_method {
            let native_method: &dyn NativeMethod = unsafe { &*native_method_ptr };
            jvm_log!("[Native] native method found for key: {}", full_name);
            native_method.invoke(args, self)
        } else {
            jvm_log!("[Native] native method NOT found for key: {}", full_name);
            Err(JvmError::IllegalStateError(format!("Native method not found: {}", full_name)))
        }
    }
    
    /// 通用的方法调用分发函数
    pub fn dispatch_method_call(&mut self, class_name: &str, method_name: &str, descriptor: &str, args: Vec<JvmValue>) -> Result<Option<JvmValue>, JvmError> {
        // 1. 尝试加载类
        let klass = match self.load(class_name) {
            Ok(k) => k,
            Err(e) => {
                return Err(JvmError::ClassNotFoundError(format!("Failed to load class {}: {:?}", class_name, e)));
            }
        };

        // 2. 在类中查找方法
        let method = match klass.lookup_method(method_name, descriptor, self) {
            Some(m) => m,
            None => {
                return Err(JvmError::IllegalStateError(format!("Method {}.{}{} not found", class_name, method_name, descriptor)));
            }
        };

        // 3. 检查是否为native方法
        if method.is_native() {
            jvm_log!("[Dispatch] Calling native method: {}.{}", class_name, method_name);
            return self.call_native_method(class_name, method_name, args);
        }

        // 4. 对于Java方法，创建新的执行帧并执行
        jvm_log!("[Dispatch] Calling Java method: {}.{}", class_name, method_name);

        // 解析参数类型
        let param_types = crate::instructions::method_utils::parse_method_descriptor(descriptor);
        // 判断是否静态方法（ACC_STATIC = 0x0008）
        let is_static = (method.access_flags & 0x0008) != 0;
        
        // 使用更大的栈大小防止栈溢出
        let mut thread = crate::jvm_thread::JvmThread::new(65536, method.max_locals);
        thread.frames.clear();
        
        // 构造Frame
        let mut frame = crate::jvm_thread::Frame {
            local_vars: crate::local_vars::LocalVars::new(method.max_locals),
            stack: crate::operand_stack::OperandStack::new(65536), // 使用更大的栈大小
            method: method.clone(),
            pc: 0,
        };
        
        let mut arg_index = 0;
        // 实例方法第一个参数是this
        if !is_static {
            if let Some(JvmValue::ObjRef(this_ref)) = args.get(0) {
                frame.local_vars.set_obj_ref(0, *this_ref);
                arg_index = 1;
            }
        }
        // 其余参数
        for (i, param) in param_types.iter().enumerate() {
            let arg = args.get(arg_index + i).cloned().unwrap_or(JvmValue::Null);
            match param.as_str() {
                "I" | "Z" | "B" | "C" | "S" => {
                    if let JvmValue::Int(v) = arg {
                        frame.local_vars.set_int(arg_index + i, v as i32);
                    }
                }
                "J" => {
                    if let JvmValue::Long(v) = arg {
                        // long 拆成两个i32存储，低位在前
                        frame.local_vars.set_int(arg_index + i, (v & 0xFFFF_FFFF) as i32);
                        frame.local_vars.set_int(arg_index + i + 1, (v >> 32) as i32);
                    }
                }
                "F" => {
                    if let JvmValue::Float(v) = arg {
                        frame.local_vars.set_int(arg_index + i, v as i32);
                    }
                }
                "D" => {
                    if let JvmValue::Double(v) = arg {
                        frame.local_vars.set_int(arg_index + i, (v & 0xFFFF_FFFF) as i32);
                        frame.local_vars.set_int(arg_index + i + 1, (v >> 32) as i32);
                    }
                }
                s if s.starts_with("L") || s.starts_with("[") => {
                    if let JvmValue::ObjRef(ptr) = arg {
                        frame.local_vars.set_obj_ref(arg_index + i, ptr);
                    }
                }
                _ => {}
            }
        }
        thread.frames.push(frame);
        
        // 执行方法
        let code = method.get_code();
        let mut ret: Option<JvmValue> = None;
        
        // 添加执行步数限制，防止无限循环
        let mut step_count = 0;
        let max_steps = 10000; // 最大执行步数
        
        while !thread.frames.is_empty() && thread.frames[0].pc < code.len() && step_count < max_steps {
            step_count += 1;
            let opcode = code[thread.frames[0].pc];
            thread.frames[0].pc += 1;
            let frame = &mut thread.frames[0];
            
            match opcode {
                0xac => { // ireturn
                    if !frame.stack.is_values_empty() {
                        let v = frame.stack.pop_int();
                        ret = Some(JvmValue::Int(v as u32));
                    }
                    thread.frames.pop();
                    break;
                }
                0xad => { // lreturn
                    // TODO: long
                    thread.frames.pop();
                    break;
                }
                0xae => { // freturn
                    // TODO: float
                    thread.frames.pop();
                    break;
                }
                0xaf => { // dreturn
                    // TODO: double
                    thread.frames.pop();
                    break;
                }
                0xb0 => { // areturn
                    if !frame.stack.is_obj_refs_empty() {
                        let v = frame.stack.pop_obj_ref();
                        ret = Some(JvmValue::ObjRef(v));
                    }
                    thread.frames.pop();
                    break;
                }
                0xb1 => { // return
                    thread.frames.pop();
                    break;
                }
                _ => {
                    // 复用主线程的VM和heap，直接调用execute方法单步执行
                    // 这里直接复用frame和VM即可
                    // 由于execute_one不存在，直接复用frame和VM的指令分发
                    // 这里直接调用主线程的execute方法更安全
                    // 但此处为单步，直接match分发
                    match opcode {
                        0x00 => (), // nop
                        0x01 => crate::instructions::constants::exec_aconst_null(frame, code, Some(self))?,
                        0x02 => crate::instructions::constants::exec_iconst_m1(frame, code, Some(self))?,
                        0x03 => crate::instructions::constants::exec_iconst_0(frame, code, Some(self))?,
                        0x04 => crate::instructions::constants::exec_iconst_1(frame, code, Some(self))?,
                        0x05 => crate::instructions::constants::exec_iconst_2(frame, code, Some(self))?,
                        0x06 => crate::instructions::constants::exec_iconst_3(frame, code, Some(self))?,
                        0x07 => crate::instructions::constants::exec_iconst_4(frame, code, Some(self))?,
                        0x08 => crate::instructions::constants::exec_iconst_5(frame, code, Some(self))?,
                        0x10 => crate::instructions::constants::exec_bipush(frame, code, Some(self))?,
                        0x11 => crate::instructions::ldc_ops::exec_sipush(frame, code, Some(self))?,
                        0x12 => crate::instructions::ldc_ops::exec_ldc(frame, code, Some(self))?,
                        0x13 => crate::instructions::ldc_ops::exec_ldc_w(frame, code, Some(self))?,
                        0x14 => crate::instructions::ldc_ops::exec_ldc2_w(frame, code, Some(self))?,
                        0x15 => crate::instructions::load_store::exec_iload(frame, code, Some(self))?,
                        0x1a => crate::instructions::load_store::exec_iload_0(frame, code, Some(self))?,
                        0x1b => crate::instructions::load_store::exec_iload_1(frame, code, Some(self))?,
                        0x1c => crate::instructions::load_store::exec_iload_2(frame, code, Some(self))?,
                        0x1d => crate::instructions::load_store::exec_iload_3(frame, code, Some(self))?,
                        0x2a => crate::instructions::aload_0::exec_aload_0(frame, code, Some(self))?,
                        0x2b => crate::instructions::load_store::exec_aload_1(frame, code, Some(self))?,
                        0x2c => crate::instructions::load_store::exec_aload_2(frame, code, Some(self))?,
                        0x2d => crate::instructions::load_store::exec_aload_3(frame, code, Some(self))?,
                        0x36 => crate::instructions::load_store::exec_istore(frame, code, Some(self))?,
                        0x3b => crate::instructions::load_store::exec_istore_0(frame, code, Some(self))?,
                        0x3c => crate::instructions::load_store::exec_istore_1(frame, code, Some(self))?,
                        0x3d => crate::instructions::load_store::exec_istore_2(frame, code, Some(self))?,
                        0x3e => crate::instructions::load_store::exec_istore_3(frame, code, Some(self))?,
                        0x4b => crate::instructions::load_store::exec_astore_0(frame, code, Some(self))?,
                        0x4c => crate::instructions::load_store::exec_astore_1(frame, code, Some(self))?,
                        0x4d => crate::instructions::load_store::exec_astore_2(frame, code, Some(self))?,
                        0x4e => crate::instructions::load_store::exec_astore_3(frame, code, Some(self))?,
                        0x59 => crate::instructions::stack::exec_dup(frame, code, Some(self))?,
                        0xb7 => crate::instructions::invokespecial::exec_invokespecial(frame, code, Some(self))?,
                        0x60 => crate::instructions::arithmetic::exec_iadd(frame, code, Some(self))?,
                        0x64 => crate::instructions::arithmetic::exec_isub(frame, code, Some(self))?,
                        0x68 => crate::instructions::arithmetic::exec_imul(frame, code, Some(self))?,
                        0x6c => crate::instructions::arithmetic::exec_idiv(frame, code, Some(self))?,
                        0x84 => crate::instructions::iinc::exec_iinc(frame, code, Some(self))?,
                        0x99 => crate::instructions::control::exec_ifeq(frame, code, Some(self))?,
                        0x9a => crate::instructions::control::exec_ifne(frame, code, Some(self))?,
                        0x9f => crate::instructions::control::exec_ifge(frame, code, Some(self))?,
                        0xa7 => crate::instructions::control::exec_goto(frame, code, Some(self))?,
                        0xb2 => crate::instructions::field_ops::exec_getstatic(frame, code, Some(self), &method)?,
                        0xb3 => crate::instructions::field_ops::exec_putstatic(frame, code, Some(self), &method)?,
                        0xb6 => crate::instructions::invokevirtual::exec_invokevirtual(frame, code, Some(self))?,
                        0xb5 => crate::instructions::object_ops::exec_putfield(frame, code, Some(self))?,
                        0xbc => crate::instructions::array_ops::exec_newarray(frame, code, Some(self))?,
                        0xbe => crate::instructions::array_ops::exec_arraylength(frame, code, Some(self))?,
                        0x4f => crate::instructions::array_ops::exec_iastore(frame, code, Some(self))?,
                        0x2e => crate::instructions::array_ops::exec_iaload(frame, code, Some(self))?,
                        0xa2 => crate::instructions::control_extended::exec_if_icmpge(frame, code, Some(self))?,
                        0xb0 => crate::instructions::control_extended::exec_areturn(frame, code, Some(self))?,
                        0xbb => crate::instructions::object_ops::exec_new(frame, code, Some(self))?,
                        0xb8 => crate::instructions::invokestatic::exec_invokestatic(frame, code, Some(self))?,
                        0x3f => crate::instructions::load_store::exec_istore_0(frame, code, Some(self))?,
                        0x40 => crate::instructions::load_store::exec_istore_1(frame, code, Some(self))?,
                        0x41 => crate::instructions::load_store::exec_istore_2(frame, code, Some(self))?,
                        0x42 => crate::instructions::load_store::exec_istore_3(frame, code, Some(self))?,
                        0xb4 => crate::instructions::object_ops::exec_getfield(frame, code, Some(self))?,
                        _ => return Err(JvmError::IllegalStateError(format!("Unknown opcode: 0x{:x}", opcode))),
                    }
                }
            }
        }
        
        // 检查是否因为步数限制而退出
        if step_count >= max_steps {
            return Err(JvmError::IllegalStateError(format!("Method execution exceeded maximum steps: {}", max_steps)));
        }
        
        Ok(ret)
    }
    
    /// 创建字符串对象
    pub fn create_string_object(&mut self, string_content: &str) -> Result<RawPtr, AllocError> {
        // 简化实现：直接创建字符串对象，不依赖加载完整的String类
        self.create_simple_string_object(string_content)
    }
    
    /// 创建简化的字符串对象（不依赖String类加载）
    fn create_simple_string_object(&mut self, string_content: &str) -> Result<RawPtr, AllocError> {
        jvm_log!("[String] Creating string object for: '{}'", string_content);
        
        // 1. 创建字符数组
        let chars: Vec<u16> = string_content.encode_utf16().collect();
        jvm_log!("[String] Encoded to {} UTF-16 chars", chars.len());
        
        let char_array_ptr = match self.create_char_array(&chars) {
            Ok(ptr) => {
                jvm_log!("[String] Created char array: {:?}", ptr);
                ptr
            }
            Err(e) => {
                jvm_log!("[String] Failed to create char array: {:?}", e);
                return Err(e);
            }
        };
        
        // 2. 创建简化的String对象
        // String对象的内存布局：Header + value字段(指向char[])
        let header_size = std::mem::size_of::<crate::heap::Header>();
        let total_size = header_size + 8; // 8字节存储value字段
        
        jvm_log!("[String] Allocating string object: header_size={}, total_size={}", header_size, total_size);
        
        // 分配内存
        let layout = std::alloc::Layout::from_size_align(total_size, 8).unwrap();
        let ptr = unsafe { std::alloc::alloc_zeroed(layout) };
        
        if ptr.is_null() {
            jvm_log!("[String] Memory allocation failed");
            return Err(AllocError::OOM);
        }
        
        let string_ptr = RawPtr(ptr);
        jvm_log!("[String] Allocated string object: {:?}", string_ptr);
        
        // 初始化头部（简化版本）
        unsafe {
            let header_ptr = ptr as *mut crate::heap::Header;
            *header_ptr = crate::heap::Header::new()
                .with_class_id(0) // 使用0表示String类
                .with_state(crate::heap::GcState::Unmarked)
                .with_size(total_size);
            jvm_log!("[String] Initialized header");
        }
        
        // 设置value字段指向字符数组
        unsafe {
            let value_field_ptr = ptr.add(header_size) as *mut RawPtr;
            *value_field_ptr = char_array_ptr;
            jvm_log!("[String] Set value field to char array: {:?}", char_array_ptr);
        }
        
        jvm_log!("[String] Successfully created string object: {:?}", string_ptr);
        
        // 将字符串对象注册到string_map中，以便System.out.println能够正确显示
        self.string_map.borrow_mut().insert(string_ptr, string_content.to_string());
        
        Ok(string_ptr)
    }
    
    /// 创建字符数组
    fn create_char_array(&mut self, chars: &[u16]) -> Result<RawPtr, AllocError> {
        let header_size = std::mem::size_of::<crate::heap::Header>();
        let total_size = header_size + 8 + chars.len() * 2; // 8字节存储length，每个char 2字节
        
        // 分配内存
        let layout = std::alloc::Layout::from_size_align(total_size, 8).unwrap();
        let ptr = unsafe { std::alloc::alloc_zeroed(layout) };
        
        if ptr.is_null() {
            return Err(AllocError::OOM);
        }
        
        let array_ptr = RawPtr(ptr);
        
        // 初始化头部
        unsafe {
            let header_ptr = ptr as *mut crate::heap::Header;
            *header_ptr = crate::heap::Header::new()
                .with_class_id(1) // 使用1表示char[]类
                .with_state(crate::heap::GcState::Unmarked)
                .with_size(total_size);
        }
        
        // 设置数组长度
        unsafe {
            let length_ptr = ptr.add(header_size) as *mut usize;
            *length_ptr = chars.len();
        }
        
        // 写入字符数据
        for (i, &ch) in chars.iter().enumerate() {
            unsafe {
                let char_ptr = ptr.add(header_size + 8 + i * 2) as *mut u16;
                *char_ptr = ch;
            }
        }
        
        Ok(array_ptr)
    }
    
    /// 创建简单的数组对象（用于newarray指令）
    pub fn create_simple_array(&mut self, length: usize, array_type: u8) -> Result<RawPtr, AllocError> {
        let header_size = std::mem::size_of::<crate::heap::Header>();
        
        // 根据数组类型确定元素大小
        let element_size = match array_type {
            4 => 1,  // T_BOOLEAN
            5 => 2,  // T_CHAR
            6 => 4,  // T_FLOAT
            7 => 8,  // T_DOUBLE
            8 => 1,  // T_BYTE
            9 => 2,  // T_SHORT
            10 => 4, // T_INT
            11 => 8, // T_LONG
            _ => return Err(AllocError::BadRequest),
        };
        
        let total_size = header_size + 8 + length * element_size; // 8字节存储length
        
        // 分配内存
        let layout = std::alloc::Layout::from_size_align(total_size, 8).unwrap();
        let ptr = unsafe { std::alloc::alloc_zeroed(layout) };
        
        if ptr.is_null() {
            return Err(AllocError::OOM);
        }
        
        let array_ptr = RawPtr(ptr);
        
        // 初始化头部
        unsafe {
            let header_ptr = ptr as *mut crate::heap::Header;
            *header_ptr = crate::heap::Header::new()
                .with_class_id(array_type as usize) // 使用数组类型作为类ID
                .with_state(crate::heap::GcState::Unmarked)
                .with_size(total_size);
        }
        
        // 设置数组长度
        unsafe {
            let length_ptr = ptr.add(header_size) as *mut usize;
            *length_ptr = length;
        }
        
        Ok(array_ptr)
    }
}




