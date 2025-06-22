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

pub struct Vm {
    pub heap: RefCell<Heap>,
    class_loader: RefCell<BootstrapClassLoader>,
    // 静态字段存储: (类名, 字段名) -> 值
    static_fields: HashMap<(String, String), JvmValue>,
    // Native方法注册表
    native_methods: NativeMethodRegistry,
    pub string_builder_map: RefCell<HashMap<crate::heap::RawPtr, String>>,
}
impl Vm {
    pub fn new(paths: &str) -> Vm {
        Vm {
            class_loader: RefCell::new(BootstrapClassLoader::new(paths)),
            heap: RefCell::new(Heap::with_maximum_memory(1024 * 1024)),
            static_fields: HashMap::new(),
            native_methods: NativeMethodRegistry::new(),
            string_builder_map: RefCell::new(HashMap::new()),
        }
    }
    
    pub fn load(&mut self, class_name: &str) -> Result<Klass, JvmError> {
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
            (*class_loader_ptr).initialize_class(class_name, &mut *heap_ptr, self as *mut Vm)?;
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
    pub fn get_static_field(&self, class_name: &str, field_name: &str) -> Option<&JvmValue> {
        self.static_fields.get(&(class_name.to_string(), field_name.to_string()))
    }
    
    /// 调用native方法
    pub fn call_native_method(&self, class_name: &str, method_name: &str, args: Vec<JvmValue>) -> Result<Option<JvmValue>, JvmError> {
        let full_name = format!("{}.{}", class_name, method_name);
        jvm_log!("[Native] call_native_method key: {}", full_name);
        if let Some(native_method) = self.native_methods.get(&full_name) {
            jvm_log!("[Native] native method found for key: {}", full_name);
            native_method.invoke(args)
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

        // 4. 对于Java方法，创建新的执行帧
        jvm_log!("[Dispatch] Calling Java method: {}.{}", class_name, method_name);
        
        // 这里应该创建新的执行帧并执行Java方法
        // 由于这需要访问JvmThread，我们将在解释器中处理Java方法调用
        Err(JvmError::IllegalStateError("Java method execution not implemented in dispatch".to_string()))
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




