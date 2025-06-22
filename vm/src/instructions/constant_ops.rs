use crate::jvm_thread::Frame;
use crate::error::JvmError;
use crate::vm::Vm;
use crate::jvm_log;

pub fn exec_ldc(frame: &mut Frame, code: &[u8], pc: &mut usize, vm: Option<&mut Vm>, method: &crate::method::Method) -> Result<(), JvmError> {
    let index = code[*pc] as usize;
    *pc += 1;
    let cp = &method.constant_pool;
    match &cp[index - 1] {
        reader::constant_pool::CpInfo::Integer { bytes, .. } => {
            let value = *bytes as i32;
            frame.stack.push_int(value);
        }
        reader::constant_pool::CpInfo::Float { bytes, .. } => {
            let value = f32::from_bits(*bytes);
            frame.stack.push_int(value.to_bits() as i32);
        }
        reader::constant_pool::CpInfo::String { string_index, .. } => {
            let s = cp.get_utf8_string(*string_index);
            jvm_log!("ldc string: {}", s);
            // 创建字符串对象并推入栈
            if let Some(ref mut vm) = vm {
                match vm.create_string_object(&s) {
                    Ok(string_ptr) => {
                        // 将对象引用推入栈
                        frame.stack.push_obj_ref(string_ptr);
                    }
                    Err(e) => {
                        return Err(JvmError::IllegalStateError(format!("Failed to create string object: {:?}", e)));
                    }
                }
            } else {
                panic!("ldc指令需要有效的VM引用以创建字符串对象，但vm为None");
            }
        }
        _ => {
            return Err(JvmError::IllegalStateError(format!("ldc: 常量池索引{}类型不支持", index)));
        }
    }
    Ok(())
}

pub fn exec_ldc_w(frame: &mut Frame, code: &[u8], pc: &mut usize, vm: Option<&mut Vm>, method: &crate::method::Method) -> Result<(), JvmError> {
    let index = ((code[*pc] as u16) << 8 | code[*pc + 1] as u16) as usize;
    *pc += 2;
    let cp = &method.constant_pool;
    match &cp[index - 1] {
        reader::constant_pool::CpInfo::Integer { bytes, .. } => {
            let value = *bytes as i32;
            frame.stack.push_int(value);
        }
        reader::constant_pool::CpInfo::Float { bytes, .. } => {
            let value = f32::from_bits(*bytes);
            frame.stack.push_int(value.to_bits() as i32);
        }
        reader::constant_pool::CpInfo::String { string_index, .. } => {
            let s = cp.get_utf8_string(*string_index);
            jvm_log!("ldc_w string: {}", s);
            // 创建字符串对象并推入栈
            if let Some(ref mut vm) = vm {
                match vm.create_string_object(&s) {
                    Ok(string_ptr) => {
                        // 将对象引用推入栈
                        frame.stack.push_obj_ref(string_ptr);
                    }
                    Err(e) => {
                        return Err(JvmError::IllegalStateError(format!("Failed to create string object: {:?}", e)));
                    }
                }
            } else {
                panic!("ldc_w指令需要有效的VM引用以创建字符串对象，但vm为None");
            }
        }
        _ => {
            return Err(JvmError::IllegalStateError(format!("ldc_w: 常量池索引{}类型不支持", index)));
        }
    }
    Ok(())
}

pub fn exec_ldc2_w(frame: &mut Frame, code: &[u8], pc: &mut usize, _vm: Option<&mut Vm>, method: &crate::method::Method) -> Result<(), JvmError> {
    let index = ((code[*pc] as u16) << 8 | code[*pc + 1] as u16) as usize;
    *pc += 2;
    let cp = &method.constant_pool;
    match &cp[index - 1] {
        reader::constant_pool::CpInfo::Long { high_bytes, low_bytes, .. } => {
            let value = ((*high_bytes as u64) << 32) | (*low_bytes as u64);
            jvm_log!("ldc2_w long: {} (推入两个32位值)", value);
            // 先推高32位，再推低32位
            frame.stack.push_int((*high_bytes) as i32);
            frame.stack.push_int((*low_bytes) as i32);
        }
        reader::constant_pool::CpInfo::Double { high_bytes, low_bytes, .. } => {
            let bits = ((*high_bytes as u64) << 32) | (*low_bytes as u64);
            let value = f64::from_bits(bits);
            jvm_log!("ldc2_w double: {} (推入两个32位值)", value);
            // 先推高32位，再推低32位
            frame.stack.push_int((*high_bytes) as i32);
            frame.stack.push_int((*low_bytes) as i32);
        }
        _ => {
            return Err(JvmError::IllegalStateError(format!("ldc2_w: 常量池索引{}不是long/double", index)));
        }
    }
    Ok(())
} 