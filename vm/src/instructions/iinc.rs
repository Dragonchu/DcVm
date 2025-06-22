use crate::error::JvmError;
use crate::jvm_thread::Frame;

pub fn exec_iinc(frame: &mut Frame, code: &[u8], _vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let index = code[frame.pc] as usize;
    let const_val = code[frame.pc + 1] as i8;
    frame.pc += 2;
    let value = frame.local_vars.get_int(index);
    frame.local_vars.set_int(index, value + const_val as i32);
    Ok(())
} 