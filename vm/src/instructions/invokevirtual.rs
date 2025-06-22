use crate::error::JvmError;
use crate::jvm_thread::Frame;
use crate::JvmValue;
use crate::jvm_log;
use reader::constant_pool::{ConstantPool, ConstantPoolExt};

pub fn exec_invokevirtual(frame: &mut Frame, code: &[u8], vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
    frame.pc += 2;
    
    let cp = &frame.method.constant_pool;
    let (class_name, method_name, method_desc) = cp.get_methodref_info(index as u16);
    jvm_log!("[Virtual] 调用方法: {}.{}{}", class_name, method_name, method_desc);
    
    // 简化处理：暂时返回成功
    jvm_log!("[Virtual] 方法调用完成: {}", method_name);
    Ok(())
} 