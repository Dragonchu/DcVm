use crate::JvmValue;
use crate::heap::RawPtr;
use crate::error::JvmError;
use crate::jvm_log;
use std::collections::HashMap;

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
pub trait NativeMethod: Send + Sync {
    fn invoke(&self, args: Vec<JvmValue>, vm: &mut crate::vm::Vm) -> Result<Option<JvmValue>, JvmError>;
}

/// System.out.println实现
#[derive(Clone)]
pub struct SystemOutPrintln;

unsafe impl Send for SystemOutPrintln {}
unsafe impl Sync for SystemOutPrintln {}

impl NativeMethod for SystemOutPrintln {
    fn invoke(&self, args: Vec<JvmValue>, vm: &mut crate::vm::Vm) -> Result<Option<JvmValue>, JvmError> {
        jvm_log!("[Native] System.out.println called with {} arguments", args.len());
        for (i, arg) in args.iter().enumerate() {
            jvm_log!("[Native] args[{}] = {:?}", i, arg);
        }
        // 打印内容
        if args.len() >= 2 {
            // 第一个参数是this引用，第二个参数是要打印的对象
            let value = &args[1];
            match value {
                JvmValue::ObjRef(ptr) => {
                    if ptr.is_null() {
                        println!("null");
                    } else {
                        // 添加调试日志
                        let string_map = vm.string_map.borrow();
                        jvm_log!("[Native] System.out.println: string_map contains {} entries", string_map.len());
                        jvm_log!("[Native] System.out.println: looking for ptr {:?}", ptr);
                        for (key, val) in string_map.iter() {
                            jvm_log!("[Native] System.out.println: string_map entry: {:?} -> '{}'", key, val);
                        }
                        
                        if let Some(s) = string_map.get(ptr) {
                            jvm_log!("[Native] System.out.println: found string in map: '{}'", s);
                            println!("{}", s);
                        } else if (ptr.0 as usize) % 8 == 0 && (ptr.0 as usize) > 0x1000 {
                            // 仅对对齐且非伪造指针尝试解码
                            match extract_string_content(*ptr) {
                                Ok(s) => {
                                    jvm_log!("[Native] System.out.println: extracted string content: '{}'", s);
                                    println!("{}", s);
                                },
                                Err(e) => {
                                    jvm_log!("[Native] System.out.println: failed to extract string content: {:?}", e);
                                    println!("[Object]");
                                }
                            }
                        } else {
                            jvm_log!("[Native] System.out.println: ptr not aligned or too small: {:?}", ptr);
                            println!("[Object]");
                        }
                    }
                }
                JvmValue::Int(v) => println!("{}", v),
                JvmValue::Long(v) => println!("{}", v),
                JvmValue::Boolean(v) => println!("{}", *v != 0),
                JvmValue::Float(v) => println!("{}", v),
                JvmValue::Double(v) => println!("{}", v),
                JvmValue::Null => println!("null"),
                _ => println!("[Unsupported]"),
            }
        } else if args.len() == 1 {
            // 只有一个参数（this引用），打印空行
            println!();
        } else {
            println!();
        }
        Ok(None)
    }
}

/// StringBuilder.append实现
#[derive(Clone)]
pub struct StringBuilderAppend;

unsafe impl Send for StringBuilderAppend {}
unsafe impl Sync for StringBuilderAppend {}

impl NativeMethod for StringBuilderAppend {
    fn invoke(&self, args: Vec<JvmValue>, vm: &mut crate::vm::Vm) -> Result<Option<JvmValue>, JvmError> {
        jvm_log!("[Native] StringBuilder.append called with {} arguments", args.len());
        
        if args.len() < 2 {
            jvm_log!("[Native] StringBuilder.append: insufficient arguments");
            return Ok(Some(JvmValue::Null));
        }
        
        let this_ptr = match &args[0] { 
            JvmValue::ObjRef(ptr) => *ptr, 
            _ => {
                jvm_log!("[Native] StringBuilder.append: first argument is not an object reference");
                return Ok(Some(JvmValue::Null));
            }
        };
        
        if this_ptr.is_null() {
            jvm_log!("[Native] StringBuilder.append: this reference is null");
            return Ok(Some(JvmValue::Null));
        }
        
        let value = &args[1];
        let mut map = vm.string_builder_map.borrow_mut();
        let entry = map.entry(this_ptr).or_insert_with(String::new);
        
        match value {
            JvmValue::Int(v) => entry.push_str(&v.to_string()),
            JvmValue::Long(v) => entry.push_str(&v.to_string()),
            JvmValue::Boolean(v) => entry.push_str(&(v != &0).to_string()),
            JvmValue::ObjRef(ptr) => {
                if ptr.is_null() {
                    entry.push_str("null");
                } else if let Some(s) = vm.string_map.borrow().get(ptr) {
                    entry.push_str(s);
                } else if (ptr.0 as usize) % 8 == 0 && (ptr.0 as usize) > 0x1000 {
                    if let Ok(s) = extract_string_content(*ptr) {
                        entry.push_str(&s);
                    } else {
                        entry.push_str("[Object]");
                    }
                } else {
                    entry.push_str("[Object]");
                }
            }
            JvmValue::Null => entry.push_str("null"),
            _ => entry.push_str("[Unsupported]"),
        }
        
        Ok(Some(JvmValue::ObjRef(this_ptr)))
    }
}

/// StringBuilder.toString实现
#[derive(Clone)]
pub struct StringBuilderToString;

unsafe impl Send for StringBuilderToString {}
unsafe impl Sync for StringBuilderToString {}

impl NativeMethod for StringBuilderToString {
    fn invoke(&self, args: Vec<JvmValue>, vm: &mut crate::vm::Vm) -> Result<Option<JvmValue>, JvmError> {
        jvm_log!("[Native] StringBuilder.toString called with {} arguments", args.len());
        
        // 第一个参数应该是this引用
        if args.is_empty() {
            jvm_log!("[Native] StringBuilder.toString called with no arguments");
            return Ok(Some(JvmValue::Null));
        }
        
        let this_ptr = match &args[0] { 
            JvmValue::ObjRef(ptr) => *ptr, 
            _ => {
                jvm_log!("[Native] StringBuilder.toString: first argument is not an object reference");
                return Ok(Some(JvmValue::Null));
            }
        };
        
        if this_ptr.is_null() {
            jvm_log!("[Native] StringBuilder.toString: this reference is null");
            return Ok(Some(JvmValue::Null));
        }
        
        // 先获取内容，然后释放借用
        let content = {
            let map = vm.string_builder_map.borrow();
            map.get(&this_ptr).cloned()
        };
        
        if let Some(content) = content {
            jvm_log!("[Native] StringBuilder.toString: found content: '{}'", content);
            
            // 清空StringBuilder的内容
            {
                let mut map = vm.string_builder_map.borrow_mut();
                if let Some(sb_content) = map.get_mut(&this_ptr) {
                    sb_content.clear();
                }
            }
            
            // 创建新的字符串对象
            let string_ptr = match vm.create_string_object(&content) {
                Ok(ptr) => ptr,
                Err(_) => {
                    jvm_log!("[Native] StringBuilder.toString: failed to create string object");
                    return Ok(Some(JvmValue::Null));
                }
            };
            return Ok(Some(JvmValue::ObjRef(string_ptr)));
        }
        
        jvm_log!("[Native] StringBuilder.toString: no content found for this pointer");
        Ok(Some(JvmValue::Null))
    }
}

/// Object.toString实现
#[derive(Clone)]
pub struct ObjectToString;

unsafe impl Send for ObjectToString {}
unsafe impl Sync for ObjectToString {}

impl NativeMethod for ObjectToString {
    fn invoke(&self, args: Vec<JvmValue>, vm: &mut crate::vm::Vm) -> Result<Option<JvmValue>, JvmError> {
        jvm_log!("[Native] Object.toString called");
        let s = format!("Object@{:x}", args.get(0).and_then(|v| if let JvmValue::ObjRef(ptr) = v { Some(ptr.0 as usize) } else { None }).unwrap_or(0));
        let string_ptr = RawPtr(Box::into_raw(Box::new(())) as *mut u8);
        vm.string_map.borrow_mut().insert(string_ptr, s);
        Ok(Some(JvmValue::ObjRef(string_ptr)))
    }
}

/// Object.registerNatives实现
#[derive(Clone)]
pub struct ObjectRegisterNatives;

unsafe impl Send for ObjectRegisterNatives {}
unsafe impl Sync for ObjectRegisterNatives {}

impl NativeMethod for ObjectRegisterNatives {
    fn invoke(&self, _args: Vec<JvmValue>, _vm: &mut crate::vm::Vm) -> Result<Option<JvmValue>, JvmError> {
        Ok(None)
    }
}

/// 从Java String对象中提取字符串内容
fn extract_string_content(ptr: RawPtr) -> Result<String, JvmError> {
    if ptr.is_null() {
        return Err(JvmError::NullPointerError("String pointer is null".to_string()));
    }
    
    // 安全地解引用指针
    unsafe {
        // 假设Java String对象的内存布局：
        // - 对象头（8字节）
        // - value字段（指向char数组的指针）
        // - 其他字段...
        
        let obj_ptr = ptr.0 as *const u8;
        
        // 读取value字段（假设在偏移量8处）
        let value_offset = 8;
        let value_ptr = obj_ptr.add(value_offset) as *const *const u8;
        let char_array_ptr = *value_ptr;
        
        if char_array_ptr.is_null() {
            return Err(JvmError::NullPointerError("String value field is null".to_string()));
        }
        
        // 读取char数组的长度（假设在偏移量8处）
        let length_offset = 8;
        let length_ptr = char_array_ptr.add(length_offset) as *const i32;
        let length = *length_ptr;
        
        if length < 0 || length > 1000000 {
            return Err(JvmError::IllegalStateError("Invalid string length".to_string()));
        }
        
        // 读取char数组的数据（假设在偏移量12处）
        let data_offset = 12;
        let data_ptr = char_array_ptr.add(data_offset) as *const u16;
        
        // 将UTF-16字符转换为Rust字符串
        let chars: Vec<u16> = std::slice::from_raw_parts(data_ptr, length as usize).to_vec();
        match String::from_utf16(&chars) {
            Ok(s) => Ok(s),
            Err(_) => Err(JvmError::IllegalStateError("Invalid UTF-16 encoding".to_string())),
        }
    }
}
