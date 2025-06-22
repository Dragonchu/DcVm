use crate::jvm_thread::Frame;
use crate::JvmValue;
use crate::jvm_log;
use crate::heap::RawPtr;

/// 解析方法描述符，提取参数类型
pub fn parse_method_descriptor(descriptor: &str) -> Vec<String> {
    let mut param_types = Vec::new();
    let mut chars = descriptor.chars().peekable();
    
    // 跳过开头的 '('
    if chars.next() != Some('(') {
        return param_types;
    }
    
    while let Some(&c) = chars.peek() {
        if c == ')' { break; }
        let mut buf = String::new();
        // 处理数组类型
        while let Some('[') = chars.peek() {
            buf.push(chars.next().unwrap());
        }
        // 处理对象类型
        if let Some('L') = chars.peek() {
            while let Some(ch) = chars.next() {
                buf.push(ch);
                if ch == ';' { break; }
            }
            param_types.push(buf);
        } else if let Some(ch) = chars.peek() {
            // 基本类型
            if "BCDFIJSZ".contains(*ch) {
                buf.push(chars.next().unwrap());
                param_types.push(buf);
            } else {
                // 非法字符，跳过
                chars.next();
            }
        }
    }
    
    param_types
}

/// 从栈中弹出参数
pub fn pop_arguments(frame: &mut Frame, param_types: &[String]) -> Vec<JvmValue> {
    let mut args: Vec<JvmValue> = Vec::with_capacity(param_types.len());
    
    // 从后往前弹出参数（栈的顺序）
    for param_type in param_types.iter().rev() {
        match param_type.as_str() {
            "I" | "S" | "B" | "Z" => {
                if !frame.stack.is_values_empty() {
                    args.push(JvmValue::Int(frame.stack.pop_int() as u32));
                } else {
                    args.push(JvmValue::Int(0));
                }
            }
            "J" => {
                if !frame.stack.is_values_empty() {
                    let low = frame.stack.pop_int() as u32 as u64;
                    let high = if !frame.stack.is_values_empty() {
                        frame.stack.pop_int() as u32 as u64
                    } else {
                        0
                    };
                    args.push(JvmValue::Long((high << 32) | (low & 0xFFFF_FFFF)));
                } else {
                    args.push(JvmValue::Long(0));
                }
            }
            "F" => {
                if !frame.stack.is_values_empty() {
                    args.push(JvmValue::Float(frame.stack.pop_int() as u64));
                } else {
                    args.push(JvmValue::Float(0));
                }
            }
            "D" => {
                if !frame.stack.is_values_empty() {
                    let low = frame.stack.pop_int() as u32 as u64;
                    let high = if !frame.stack.is_values_empty() {
                        frame.stack.pop_int() as u32 as u64
                    } else {
                        0
                    };
                    args.push(JvmValue::Double((high << 32) | (low & 0xFFFF_FFFF)));
                } else {
                    args.push(JvmValue::Double(0));
                }
            }
            desc if desc.starts_with("L") || desc.starts_with("[") => {
                if !frame.stack.is_obj_refs_empty() {
                    args.push(JvmValue::ObjRef(frame.stack.pop_obj_ref()));
                } else {
                    args.push(JvmValue::Null);
                }
            }
            _ => {
                jvm_log!("[Method] 未知参数类型: {}", param_type);
                args.push(JvmValue::Null);
            }
        }
    }
    
    // 恢复正确的参数顺序
    args.reverse();
    args
}

/// 将返回值推入栈中
pub fn push_return_value(frame: &mut Frame, return_value: Option<JvmValue>) {
    if let Some(value) = return_value {
        match value {
            JvmValue::Int(v) => frame.stack.push_int(v as i32),
            JvmValue::Long(v) => {
                frame.stack.push_int((v >> 32) as i32);
                frame.stack.push_int((v & 0xFFFF_FFFF) as i32);
            }
            JvmValue::Float(v) => frame.stack.push_int(v as i32),
            JvmValue::Double(v) => {
                frame.stack.push_int((v >> 32) as i32);
                frame.stack.push_int((v & 0xFFFF_FFFF) as i32);
            }
            JvmValue::ObjRef(ptr) => frame.stack.push_obj_ref(ptr),
            JvmValue::Null => frame.stack.push_obj_ref(RawPtr(std::ptr::null_mut())),
            _ => frame.stack.push_int(0),
        }
    }
}

/// 处理特殊方法调用
pub fn handle_special_method_call(
    class_name: &str, 
    method_name: &str, 
    args: &[JvmValue],
    frame: &mut Frame
) -> Option<bool> {
    match (class_name, method_name) {
        ("java/io/PrintStream", "println") => {
            // 处理 System.out.println 调用
            if let Some(arg) = args.get(0) {
                match arg {
                    JvmValue::Null => println!("null"),
                    JvmValue::Int(v) => println!("{}", v),
                    JvmValue::Long(v) => println!("{}", v),
                    JvmValue::Boolean(v) => println!("{}", v != &0),
                    JvmValue::ObjRef(ptr) => {
                        if ptr.0.is_null() {
                            println!("null");
                        } else {
                            // 尝试从 VM 的 string_map 取内容
                            // 这里只能打印 [Object]，具体内容由 native_method.rs 负责
                            println!("[Object]");
                        }
                    }
                    _ => println!("[Unsupported type]"),
                }
            } else {
                println!();
            }
            Some(true)
        }
        ("java/lang/System", "currentTimeMillis") => {
            // 处理 System.currentTimeMillis() 调用
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            frame.stack.push_int((current_time >> 32) as i32);
            frame.stack.push_int((current_time & 0xFFFF_FFFF) as i32);
            jvm_log!("[Method] System.currentTimeMillis() 返回: {}", current_time);
            Some(true)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::method::Method;
    use crate::operand_stack::OperandStack;
    use crate::local_vars::LocalVars;

    #[test]
    fn test_parse_method_descriptor() {
        // 测试无参数方法
        assert_eq!(parse_method_descriptor("()V"), Vec::<String>::new());
        
        // 测试单个参数
        assert_eq!(parse_method_descriptor("(I)V"), vec!["I"]);
        assert_eq!(parse_method_descriptor("(Ljava/lang/String;)V"), vec!["Ljava/lang/String;"]);
        
        // 测试多个参数
        assert_eq!(parse_method_descriptor("(II)V"), vec!["I", "I"]);
        assert_eq!(parse_method_descriptor("(ILjava/lang/String;Z)V"), vec!["I", "Ljava/lang/String;", "Z"]);
        
        // 测试数组参数
        assert_eq!(parse_method_descriptor("([I)V"), vec!["[I"]);
        assert_eq!(parse_method_descriptor("([Ljava/lang/String;)V"), vec!["[Ljava/lang/String;"]);
    }

    #[test]
    fn test_pop_arguments() {
        let method = Method::new("test".to_string(), "()V".to_string(), 0, vec![], 10, 10);
        let mut frame = Frame {
            pc: 0,
            stack: OperandStack::new(10),
            local_vars: LocalVars::new(10),
            method,
        };
        
        // 压入一些测试数据
        frame.stack.push_int(42);
        frame.stack.push_int(100);
        frame.stack.push_obj_ref(RawPtr(std::ptr::null_mut()));
        
        // 测试弹出参数
        let param_types = vec!["I".to_string(), "I".to_string(), "Ljava/lang/Object;".to_string()];
        let args = pop_arguments(&mut frame, &param_types);
        
        assert_eq!(args.len(), 3);
        assert_eq!(args[0], JvmValue::Int(42));
        assert_eq!(args[1], JvmValue::Int(100));
        assert_eq!(args[2], JvmValue::ObjRef(RawPtr(std::ptr::null_mut())));
    }

    #[test]
    fn test_push_return_value() {
        let method = Method::new("test".to_string(), "()V".to_string(), 0, vec![], 10, 10);
        let mut frame = Frame {
            pc: 0,
            stack: OperandStack::new(10),
            local_vars: LocalVars::new(10),
            method,
        };
        
        // 测试推送整数值
        push_return_value(&mut frame, Some(JvmValue::Int(42)));
        assert_eq!(frame.stack.pop_int(), 42);
        
        // 测试推送长整数值
        push_return_value(&mut frame, Some(JvmValue::Long(0x123456789ABCDEF0)));
        assert_eq!(frame.stack.pop_int(), 0x9ABCDEF0u32 as i32);
        assert_eq!(frame.stack.pop_int(), 0x12345678u32 as i32);
        
        // 测试推送对象引用
        let test_ptr = RawPtr(std::ptr::null_mut());
        push_return_value(&mut frame, Some(JvmValue::ObjRef(test_ptr)));
        assert_eq!(frame.stack.pop_obj_ref(), test_ptr);
    }

    #[test]
    fn test_handle_special_method_call() {
        let method = Method::new("test".to_string(), "()V".to_string(), 0, vec![], 10, 10);
        let mut frame = Frame {
            pc: 0,
            stack: OperandStack::new(10),
            local_vars: LocalVars::new(10),
            method,
        };
        
        // 测试 println 调用
        let args = vec![JvmValue::Int(42)];
        let result = handle_special_method_call("java/io/PrintStream", "println", &args, &mut frame);
        assert_eq!(result, Some(true));
        
        // 测试 currentTimeMillis 调用
        let args = vec![];
        let result = handle_special_method_call("java/lang/System", "currentTimeMillis", &args, &mut frame);
        assert_eq!(result, Some(true));
        // 验证栈上有两个值（long 类型）
        assert!(!frame.stack.is_values_empty());
        
        // 测试未知方法
        let args = vec![JvmValue::Int(42)];
        let result = handle_special_method_call("unknown/Class", "unknownMethod", &args, &mut frame);
        assert_eq!(result, None);
    }
} 