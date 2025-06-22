use crate::jvm_thread::Frame;
use crate::error::JvmError;
use crate::vm::Vm;

pub fn exec_putfield(frame: &mut Frame, _code: &[u8], _pc: &mut usize, _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    // 这里只做骨架，后续迁移完整逻辑
    Err(JvmError::Unimplemented("putfield静态分发未实现".to_string()))
} 