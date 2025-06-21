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
                // 这里应该从对象中提取字符串内容
                // 简化实现：直接打印对象引用
                jvm_log!("[Native] Printing object reference: {:?}", ptr);
                println!("[Object: {:?}]", ptr);
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
    // 这里需要从String对象中提取字符数组
    // 简化实现：返回一个占位符
    Ok(format!("String[{}]", ptr.0 as usize))
}
