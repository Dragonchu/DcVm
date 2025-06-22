use crate::error::JvmError;
use crate::jvm_thread::Frame;
use crate::vm::Vm;
use crate::jvm_log;
use reader::constant_pool::{ConstantPool, ConstantPoolExt};

pub fn exec_new(frame: &mut Frame, code: &[u8], vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
    frame.pc += 2;
    let cp = &frame.method.constant_pool;
    let class_name = cp.get_class_name(index as u16);
    jvm_log!("[New] 创建对象: {}", class_name);
    // 简化处理：暂时返回成功
    Ok(())
}

pub fn exec_getfield(frame: &mut Frame, code: &[u8], _vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
    frame.pc += 2;
    jvm_log!("getfield {}", index);
    // 简化实现：弹出对象引用，推入一个假值
    if !frame.stack.is_obj_refs_empty() {
        let _obj_ref = frame.stack.pop_obj_ref();
        // 假设字段是int，推入0
        frame.stack.push_int(0);
    } else {
        return Err(JvmError::IllegalStateError("getfield: 栈无对象引用".to_string()));
    }
    Ok(())
}

pub fn exec_putfield(frame: &mut Frame, code: &[u8], vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
    frame.pc += 2;
    
    let cp = &frame.method.constant_pool;
    match &cp[index - 1] {
        reader::constant_pool::CpInfo::FieldRef { class_index, name_and_type_index, .. } => {
            let class_name = cp.get_utf8_string(*class_index);
            let name_and_type = &cp[*name_and_type_index as usize - 1];
            match name_and_type {
                reader::constant_pool::CpInfo::NameAndType { name_index, descriptor_index, .. } => {
                    let field_name = cp.get_utf8_string(*name_index);
                    let field_desc = cp.get_utf8_string(*descriptor_index);
                    
                    jvm_log!("[PutField] 设置字段: {}.{}{}", class_name, field_name, field_desc);
                    
                    // 简化处理：暂时返回成功
                    Ok(())
                }
                _ => Err(JvmError::IllegalStateError("Invalid NameAndType in constant pool".to_string())),
            }
        }
        _ => Err(JvmError::IllegalStateError("Invalid FieldRef in constant pool".to_string())),
    }
} 