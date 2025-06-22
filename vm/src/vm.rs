use crate::heap::{AllocError, Heap, RawPtr};
use crate::{class_loader::BootstrapClassLoader, jvm_thread::JvmThread, };
use crate::class::Klass;
use crate::error::JvmError;
use crate::JvmValue;
use crate::native_method::{NativeMethodRegistry, NativeMethod};
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
        let mut class_loader = self.class_loader.borrow_mut();
        let mut heap = self.heap.borrow_mut();
        let klass = class_loader.load(class_name, &mut heap)?;
        class_loader.initialize_class(class_name, &mut heap)?;
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
        if let Some(native_method) = self.native_methods.get(&full_name) {
            native_method.invoke(args)
        } else {
            Err(JvmError::IllegalStateError(format!("Native method not found: {}", full_name)))
        }
    }
    
    /// 创建字符串对象
    pub fn create_string_object(&mut self, string_content: &str) -> Result<RawPtr, AllocError> {
        // 简化实现：直接创建字符串对象，不依赖加载完整的String类
        self.create_simple_string_object(string_content)
    }
    
    /// 创建简化的字符串对象（不依赖String类加载）
    fn create_simple_string_object(&mut self, string_content: &str) -> Result<RawPtr, AllocError> {
        // 1. 创建字符数组
        let chars: Vec<u16> = string_content.encode_utf16().collect();
        let char_array_ptr = self.create_char_array(&chars)?;
        
        // 2. 创建简化的String对象
        // String对象的内存布局：Header + value字段(指向char[])
        let header_size = std::mem::size_of::<crate::heap::Header>();
        let total_size = header_size + 8; // 8字节存储value字段
        
        // 分配内存
        let layout = std::alloc::Layout::from_size_align(total_size, 8).unwrap();
        let ptr = unsafe { std::alloc::alloc_zeroed(layout) };
        
        if ptr.is_null() {
            return Err(AllocError::OOM);
        }
        
        let string_ptr = RawPtr(ptr);
        
        // 初始化头部（简化版本）
        unsafe {
            let header_ptr = ptr as *mut crate::heap::Header;
            *header_ptr = crate::heap::Header::new()
                .with_class_id(0) // 使用0表示String类
                .with_state(crate::heap::GcState::Unmarked)
                .with_size(total_size);
        }
        
        // 设置value字段指向字符数组
        unsafe {
            let value_field_ptr = ptr.add(header_size) as *mut RawPtr;
            *value_field_ptr = char_array_ptr;
        }
        
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




