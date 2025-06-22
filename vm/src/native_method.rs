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
        registry.register("java/lang/System.out.println", Box::new(SystemOutPrintln));
        
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
        
        if args.is_empty() {
            println!();
            return Ok(None);
        }
        
        // 处理第一个参数（通常是字符串）
        match &args[0] {
            JvmValue::ObjRef(ptr) => {
                // 尝试从字符串对象中提取字符串内容
                match extract_string_content(*ptr) {
                    Ok(string_content) => {
                        jvm_log!("[Native] Printing string: {}", string_content);
                        println!("{}", string_content);
                    }
                    Err(_) => {
                        // 如果无法提取字符串内容，则打印对象引用
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
                // 将u16转换为char，需要处理UTF-16编码
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
        
        Ok(None) // println返回void
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
