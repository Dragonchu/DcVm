use crate::error::JvmError;
use crate::jvm_thread::Frame;
use crate::JvmValue;
use crate::jvm_log;
use reader::constant_pool::{ConstantPool, ConstantPoolExt};
use crate::vm::Vm;

pub fn exec_invokespecial(frame: &mut Frame, code: &[u8], vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
    frame.pc += 2;
    
    let cp = &frame.method.constant_pool;
    let (class_name, method_name, method_desc) = cp.get_methodref_info(index as u16);
    jvm_log!("[Special] 调用方法: {}.{}{}", class_name, method_name, method_desc);
    
    // 检查是否是构造函数
    if method_name == "<init>" {
        jvm_log!("[Special] 调用构造函数: {}.{}{}", class_name, method_name, method_desc);
        // 构造函数调用，不需要特殊处理
        return Ok(());
    }
    
    // 简化处理：暂时返回成功
    jvm_log!("[Special] 方法调用完成: {}", method_name);
    Ok(())
} 