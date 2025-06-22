use crate::error::JvmError;
use crate::jvm_thread::Frame;
use crate::JvmValue;
use crate::heap::RawPtr;
use crate::jvm_log;
use reader::constant_pool::ConstantPool;

pub fn exec_sipush(frame: &mut Frame, code: &[u8], _vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let high = code[frame.pc] as i16;
    let low = code[frame.pc + 1] as i16;
    frame.pc += 2;
    let value = ((high << 8) | (low & 0xFF)) as i16;
    frame.stack.push_int(value as i32);
    Ok(())
}

pub fn exec_ldc(frame: &mut Frame, code: &[u8], mut vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let index = code[frame.pc] as usize;
    frame.pc += 1;
    let cp = &frame.method.constant_pool;
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

pub fn exec_ldc_w(frame: &mut Frame, code: &[u8], mut vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
    frame.pc += 2;
    let cp = &frame.method.constant_pool;
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

pub fn exec_ldc2_w(frame: &mut Frame, code: &[u8], _vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
    frame.pc += 2;
    let cp = &frame.method.constant_pool;
    match &cp[index - 1] {
        reader::constant_pool::CpInfo::Long { high_bytes, low_bytes, .. } => {
            let value = ((*high_bytes as u64) << 32) | (*low_bytes as u64);
            jvm_log!("ldc2_w long: {} (推入两个32位值)", value);
            // 先推高32位，再推低32位
            frame.stack.push_int(*high_bytes as i32);
            frame.stack.push_int(*low_bytes as i32);
        }
        reader::constant_pool::CpInfo::Double { high_bytes, low_bytes, .. } => {
            let bits = ((*high_bytes as u64) << 32) | (*low_bytes as u64);
            let value = f64::from_bits(bits);
            jvm_log!("ldc2_w double: {} (推入两个32位值)", value);
            // 先推高32位，再推低32位
            frame.stack.push_int(*high_bytes as i32);
            frame.stack.push_int(*low_bytes as i32);
        }
        _ => {
            return Err(JvmError::IllegalStateError(format!("ldc2_w: 常量池索引{}不是long/double", index)));
        }
    }
    Ok(())
} 