use crate::jvm_thread::Frame;
use crate::error::JvmError;
use crate::vm::Vm;
use crate::JvmValue;
use crate::heap::RawPtr;
use crate::jvm_log;

pub fn exec_invokevirtual(frame: &mut Frame, code: &[u8], pc: &mut usize, vm: Option<&mut Vm>, method: &crate::method::Method) -> Result<(), JvmError> {
    let index = ((code[*pc] as u16) << 8 | code[*pc + 1] as u16) as usize;
    *pc += 2;
    jvm_log!("invokevirtual {}", index);
    
    // 从常量池获取方法引用
    let cp = &method.constant_pool;
    if let reader::constant_pool::CpInfo::MethodRef { class_index, name_and_type_index, .. } = &cp[index - 1] {
        // 获取类名
        let class_name = if let reader::constant_pool::CpInfo::Class { name_index, .. } = &cp[(*class_index - 1) as usize] {
            cp.get_utf8_string(*name_index)
        } else {
            return Err(JvmError::IllegalStateError("Invalid class reference".to_string()));
        };
        
        // 获取方法名和描述符
        let name_and_type = if let reader::constant_pool::CpInfo::NameAndType { name_index, descriptor_index, .. } = &cp[(*name_and_type_index - 1) as usize] {
            let method_name = cp.get_utf8_string(*name_index);
            let method_desc = cp.get_utf8_string(*descriptor_index);
            (method_name, method_desc)
        } else {
            return Err(JvmError::IllegalStateError("Invalid name and type reference".to_string()));
        };
        
        jvm_log!("[Virtual method call: {}.{}]", class_name, name_and_type.0);
        
        // 1. 解析参数类型
        let desc = &name_and_type.1;
        let mut param_types = Vec::new();
        let mut chars = desc.chars();
        if chars.next() == Some('(') {
            let mut buf = String::new();
            let mut in_obj = false;
            while let Some(c) = chars.next() {
                if c == ')' { break; }
                buf.push(c);
                if c == 'L' { in_obj = true; }
                if in_obj && c == ';' {
                    param_types.push(buf.clone());
                    buf.clear();
                    in_obj = false;
                } else if !in_obj && (c == 'I' || c == 'J' || c == 'F' || c == 'D' || c == 'Z' || c == 'B' || c == 'C' || c == 'S' || c == '[') {
                    param_types.push(buf.clone());
                    buf.clear();
                }
            }
        }
        let param_count = param_types.len();
        
        // 2. 弹出参数（注意顺序，先弹最后一个参数）
        let mut args: Vec<JvmValue> = Vec::with_capacity(param_count);
        for p in param_types.iter().rev() {
            if p.starts_with("L") || p.starts_with("[") {
                if !frame.stack.is_obj_refs_empty() {
                    args.push(JvmValue::ObjRef(frame.stack.pop_obj_ref()));
                } else {
                    // 如果栈为空，使用null引用
                    args.push(JvmValue::ObjRef(RawPtr(std::ptr::null_mut())));
                }
            } else {
                if !frame.stack.is_values_empty() {
                    args.push(JvmValue::Int(frame.stack.pop_int() as u32));
                } else {
                    // 如果栈为空，使用默认值0
                    args.push(JvmValue::Int(0));
                }
            }
        }
        args.reverse(); // 恢复正确的参数顺序
        
        // 3. 弹出this引用
        let this_ref = if !frame.stack.is_obj_refs_empty() {
            frame.stack.pop_obj_ref()
        } else {
            // 如果没有this引用，检查是否是特殊的静态调用
            RawPtr(std::ptr::null_mut())
        };
        
        // 4. 处理特殊方法调用
        if class_name == "java/io/PrintStream" && name_and_type.0 == "println" {
            // 处理System.out.println调用
            if let Some(JvmValue::Int(value)) = args.get(0) {
                println!("{}", value);
            } else if let Some(JvmValue::ObjRef(_)) = args.get(0) {
                println!("[Object]");
            } else {
                println!("[Unknown type]");
            }
        } else {
            // 其他方法调用，这里需要实际的方法分发逻辑
            jvm_log!("[Method call not implemented: {}.{}]", class_name, name_and_type.0);
        }
    } else {
        return Err(JvmError::IllegalStateError(format!("invokevirtual: 常量池索引{}不是方法引用", index)));
    }
    
    Ok(())
}

pub fn exec_invokespecial(frame: &mut Frame, code: &[u8], pc: &mut usize, vm: Option<&mut Vm>, method: &crate::method::Method) -> Result<(), JvmError> {
    let index = ((code[*pc] as u16) << 8 | code[*pc + 1] as u16) as usize;
    *pc += 2;
    jvm_log!("invokespecial {}", index);
    
    // 这里只做骨架，后续迁移完整逻辑
    // 例如：let index = ...; 解析参数，弹栈，分发等
    Err(JvmError::Unimplemented("invokespecial静态分发未实现".to_string()))
}

pub fn exec_invokestatic(frame: &mut Frame, code: &[u8], pc: &mut usize, vm: Option<&mut Vm>, method: &crate::method::Method) -> Result<(), JvmError> {
    let index = ((code[*pc] as u16) << 8 | code[*pc + 1] as u16) as usize;
    *pc += 2;
    jvm_log!("invokestatic {}", index);
    
    // 这里只做骨架，后续迁移完整逻辑
    Err(JvmError::Unimplemented("invokestatic静态分发未实现".to_string()))
} 