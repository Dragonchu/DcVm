use crate::JvmValue;
use crate::heap::RawPtr;
use crate::error::JvmError;
use crate::jvm_log;

/// Native方法注册表
pub struct NativeMethodRegistry {
    methods: std::collections::HashMap<String, Box<dyn NativeMethod>>,
}

impl NativeMethodRegistry {
    pub fn new() -> Self {
        let mut registry = NativeMethodRegistry {
            methods: std::collections::HashMap::new(),
        };
        
        // 注册System.out.println方法
        registry.register("java/io/PrintStream.println", Box::new(SystemOutPrintln));
        
        // 注册StringBuilder方法
        registry.register("java/lang/StringBuilder.toString", Box::new(StringBuilderToString));
        registry.register("java/lang/StringBuilder.append", Box::new(StringBuilderAppend));
        
        // 注册Object方法
        registry.register("java/lang/Object.toString", Box::new(ObjectToString));
        // 注册Object.registerNatives空实现
        registry.register("java/lang/Object.registerNatives", Box::new(ObjectRegisterNatives));
        
        registry
    }
    
    pub fn register(&mut self, name: &str, method: Box<dyn NativeMethod>) {
        self.methods.insert(name.to_string(), method);
    }
    
    pub fn get(&self, name: &str) -> Option<&Box<dyn NativeMethod>> {
        self.methods.get(name)
    }
}

/// Native方法trait
pub trait NativeMethod {
    fn invoke(&self, args: Vec<JvmValue>) -> Result<Option<JvmValue>, JvmError>;
}

/// System.out.println实现
pub struct SystemOutPrintln;

impl NativeMethod for SystemOutPrintln {
    fn invoke(&self, args: Vec<JvmValue>) -> Result<Option<JvmValue>, JvmError> {
        jvm_log!("[Native] System.out.println called with {} arguments", args.len());
        for (i, arg) in args.iter().enumerate() {
            jvm_log!("[Native] args[{}] = {:?}", i, arg);
        }
        if args.len() <= 1 {
            println!();
            return Ok(None);
        }
        // 跳过this，处理第一个实际参数
        match &args[1] {
            JvmValue::ObjRef(ptr) => {
                jvm_log!("[Native] Processing object reference: {:?}", ptr);
                // 尝试从字符串对象中提取字符串内容
                match extract_string_content(*ptr) {
                    Ok(string_content) => {
                        jvm_log!("[Native] Printing string: {}", string_content);
                        println!("{}", string_content);
                    }
                    Err(e) => {
                        jvm_log!("[Native] Failed to extract string content: {:?}", e);
                        jvm_log!("[Native] Printing object reference: {:?}", ptr);
                        println!("[Object: {:?}]", ptr);
                    }
                }
            }
            JvmValue::Int(value) => {
                jvm_log!("[Native] Printing int value: {}", value);
                println!("{}", value);
            }
            JvmValue::Long(value) => {
                jvm_log!("[Native] Printing long value: {}", value);
                println!("{}", value);
            }
            JvmValue::Float(value) => {
                let float_value = f32::from_bits(*value as u32);
                jvm_log!("[Native] Printing float value: {}", float_value);
                println!("{}", float_value);
            }
            JvmValue::Double(value) => {
                let double_value = f64::from_bits(*value);
                jvm_log!("[Native] Printing double value: {}", double_value);
                println!("{}", double_value);
            }
            JvmValue::Boolean(value) => {
                jvm_log!("[Native] Printing boolean value: {}", value);
                println!("{}", value);
            }
            JvmValue::Char(value) => {
                if let Some(ch) = char::from_u32(*value as u32) {
                    jvm_log!("[Native] Printing char value: {}", ch);
                    println!("{}", ch);
                } else {
                    jvm_log!("[Native] Printing invalid char: {}", value);
                    println!("[Invalid char: {}]", value);
                }
            }
            JvmValue::Null => {
                jvm_log!("[Native] Printing null");
                println!("null");
            }
            _ => {
                jvm_log!("[Native] Printing unsupported type");
                println!("[Unsupported type]");
            }
        }
        Ok(None)
    }
}

/// StringBuilder.toString实现
pub struct StringBuilderToString;

impl NativeMethod for StringBuilderToString {
    fn invoke(&self, args: Vec<JvmValue>) -> Result<Option<JvmValue>, JvmError> {
        jvm_log!("[Native] StringBuilder.toString called");
        
        // StringBuilder.toString()没有参数，this引用在调用时会被弹出
        // 这里我们返回一个假的字符串对象引用
        // 在实际实现中，应该从StringBuilder对象中提取内容并创建String对象
        let fake_string_ptr = RawPtr(std::ptr::null_mut());
        Ok(Some(JvmValue::ObjRef(fake_string_ptr)))
    }
}

/// StringBuilder.append实现
pub struct StringBuilderAppend;

impl NativeMethod for StringBuilderAppend {
    fn invoke(&self, args: Vec<JvmValue>) -> Result<Option<JvmValue>, JvmError> {
        jvm_log!("[Native] StringBuilder.append called with {} arguments", args.len());
        
        // StringBuilder.append()返回this引用以支持链式调用
        // 这里我们返回一个假的对象引用
        let fake_this_ptr = RawPtr(std::ptr::null_mut());
        Ok(Some(JvmValue::ObjRef(fake_this_ptr)))
    }
}

/// Object.toString实现
pub struct ObjectToString;

impl NativeMethod for ObjectToString {
    fn invoke(&self, args: Vec<JvmValue>) -> Result<Option<JvmValue>, JvmError> {
        jvm_log!("[Native] Object.toString called");
        
        // Object.toString()返回对象的字符串表示
        // 这里我们返回一个假的字符串对象引用
        let fake_string_ptr = RawPtr(std::ptr::null_mut());
        Ok(Some(JvmValue::ObjRef(fake_string_ptr)))
    }
}

/// Object.registerNatives实现
pub struct ObjectRegisterNatives;

impl NativeMethod for ObjectRegisterNatives {
    fn invoke(&self, _args: Vec<JvmValue>) -> Result<Option<JvmValue>, JvmError> {
        // 什么都不做，直接返回
        Ok(None)
    }
}

/// 从字符串对象中提取字符串内容的辅助函数
pub fn extract_string_content(ptr: RawPtr) -> Result<String, JvmError> {
    if ptr.is_null() {
        return Err(JvmError::IllegalStateError("Null pointer".to_string()));
    }
    
    // 获取对象头部
    let header_size = std::mem::size_of::<crate::heap::Header>();
    
    // 尝试从String对象的value字段中提取字符数组
    // String对象的内存布局：Header + value字段(指向char[])
    unsafe {
        // 读取value字段（假设value字段在偏移量0处）
        let value_field_offset = header_size;
        let value_ptr = ptr.0.add(value_field_offset) as *mut RawPtr;
        let char_array_ptr = *value_ptr;
        
        if char_array_ptr.is_null() {
            return Err(JvmError::IllegalStateError("String value field is null".to_string()));
        }
        
        // 从char[]数组中提取字符串内容
        extract_string_from_char_array(char_array_ptr)
    }
}

/// 从字符数组中提取字符串内容
fn extract_string_from_char_array(arr_ptr: RawPtr) -> Result<String, JvmError> {
    if arr_ptr.is_null() {
        return Err(JvmError::IllegalStateError("Null char array pointer".to_string()));
    }
    
    unsafe {
        let header_size = std::mem::size_of::<crate::heap::Header>();
        
        // 读取数组长度（在header之后）
        let length_ptr = arr_ptr.0.add(header_size) as *mut usize;
        let length = *length_ptr;
        
        // 读取字符数据（在长度之后）
        let chars_start = arr_ptr.0.add(header_size + 8); // 8字节存储length
        let mut chars = Vec::with_capacity(length);
        
        for i in 0..length {
            let char_ptr = chars_start.add(i * 2) as *mut u16; // char是2字节
            let char_value = *char_ptr;
            chars.push(char_value);
        }
        
        // 将UTF-16字符转换为String
        match String::from_utf16(&chars) {
            Ok(s) => Ok(s),
            Err(_) => Err(JvmError::IllegalStateError("Invalid UTF-16 string".to_string())),
        }
    }
}
