use crate::jvm_thread::Frame;
use crate::error::JvmError;
use crate::vm::Vm;

pub fn exec_aload_0(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    // aload_0 - 加载this引用
    let value = frame.local_vars.get_obj_ref(0);
    frame.stack.push_obj_ref(value);
    Ok(())
} 