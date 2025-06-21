use crate::heap::{AllocError, Heap, RawPtr};
use crate::{class_loader::BootstrapClassLoader, jvm_thread::JvmThread, };
use crate::class::Klass;
use crate::error::JvmError;
use crate::JvmValue;
use crate::native_method::{NativeMethodRegistry, NativeMethod};
use std::collections::HashMap;
use reader::constant_pool::ConstantPool;

pub struct Vm {
    pub heap: Heap,
    class_loader: BootstrapClassLoader,
    // 静态字段存储: (类名, 字段名) -> 值
    static_fields: HashMap<(String, String), JvmValue>,
    // Native方法注册表
    native_methods: NativeMethodRegistry,
}
impl Vm {
    pub fn new(paths: &str) -> Vm {
        Vm {
            class_loader: BootstrapClassLoader::new(paths),
            heap: Heap::with_maximum_memory(1024 * 1024),
            static_fields: HashMap::new(),
            native_methods: NativeMethodRegistry::new(),
        }
    }
    
    pub fn load(&mut self, class_name: &str) -> Result<Klass, JvmError> {
        self.class_loader.load(class_name, &mut self.heap)
    }

    pub fn alloc_array(&mut self, klass: &Klass, length: usize) -> Result<RawPtr, AllocError> {
        match klass {
            crate::class::Klass::Array(k) => self.heap.alloc_array(k, length),
            _ => Err(AllocError::BadRequest),
        }
    }
    
    pub fn alloc_object(&mut self, klass: &Klass) -> Result<RawPtr, AllocError> {
        match klass {
            crate::class::Klass::Instance(k) => self.heap.alloc_object(k),
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
        // 1. 加载 String 类
        let string_klass = self.load("java/lang/String").map_err(|_| AllocError::BadRequest)?;
        // 2. 分配 char[] 数组
        let char_array_klass = self.load("[C").map_err(|_| AllocError::BadRequest)?;
        let chars: Vec<u16> = string_content.encode_utf16().collect();
        let arr_ptr = self.alloc_array(&char_array_klass, chars.len())?;
        // 写入字符内容
        for (i, ch) in chars.iter().enumerate() {
            let addr = unsafe { arr_ptr.0.add(std::mem::size_of::<crate::heap::Header>() + 8 + i * 2) };
            unsafe { *(addr as *mut u16) = *ch; }
        }
        // 3. 分配 String 对象
        let str_ptr = self.alloc_object(&string_klass)?;
        // 4. 设置 value 字段
        let value_field = match &string_klass {
            crate::class::Klass::Instance(k) => k.get_instance_fields().iter().enumerate()
                .find(|(_, f)| f.get_name() == "value" && f.get_descriptor() == "[C")
                .map(|(i, _)| i * 8)
                .ok_or(AllocError::BadRequest)?,
            _ => return Err(AllocError::BadRequest),
        };
        self.heap.put_field(str_ptr, value_field, crate::JvmValue::ObjRef(arr_ptr));
        Ok(str_ptr)
    }
}




