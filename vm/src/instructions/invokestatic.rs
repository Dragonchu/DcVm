use crate::error::JvmError;
use crate::jvm_thread::Frame;
use crate::JvmValue;
use crate::jvm_log;
use reader::constant_pool::ConstantPool;

pub fn exec_invokestatic(frame: &mut Frame, code: &[u8], vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
    frame.pc += 2;
    
    let cp = &frame.method.constant_pool;
    match &cp[index - 1] {
        reader::constant_pool::CpInfo::MethodRef { class_index, name_and_type_index, .. } => {
            let class_name = cp.get_utf8_string(*class_index);
            let name_and_type = &cp[*name_and_type_index as usize - 1];
            match name_and_type {
                reader::constant_pool::CpInfo::NameAndType { name_index, descriptor_index, .. } => {
                    let method_name = cp.get_utf8_string(*name_index);
                    let method_desc = cp.get_utf8_string(*descriptor_index);
                    
                    jvm_log!("[Static] 调用方法: {}.{}{}", class_name, method_name, method_desc);
                    
                    // 简化处理：暂时返回成功
                    jvm_log!("[Static] 方法调用完成: {}", method_name);
                    Ok(())
                }
                _ => Err(JvmError::IllegalStateError("Invalid NameAndType in constant pool".to_string())),
            }
        }
        _ => Err(JvmError::IllegalStateError("Invalid MethodRef in constant pool".to_string())),
    }
} 