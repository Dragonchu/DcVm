use crate::jvm_thread::Frame;
use crate::error::JvmError;
use crate::vm::Vm;

// iconst 指令族
pub fn exec_iconst_m1(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    frame.stack.push_int(-1);
    Ok(())
}

pub fn exec_iconst_0(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    frame.stack.push_int(0);
    Ok(())
}

pub fn exec_iconst_1(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    frame.stack.push_int(1);
    Ok(())
}

pub fn exec_iconst_2(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    frame.stack.push_int(2);
    Ok(())
}

pub fn exec_iconst_3(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    frame.stack.push_int(3);
    Ok(())
}

pub fn exec_iconst_4(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    frame.stack.push_int(4);
    Ok(())
}

pub fn exec_iconst_5(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    frame.stack.push_int(5);
    Ok(())
}

// bipush 指令
pub fn exec_bipush(frame: &mut Frame, code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let byte = code[frame.pc] as i8;
    frame.pc += 1;
    frame.stack.push_int(byte as i32);
    Ok(())
}

// sipush 指令
pub fn exec_sipush(frame: &mut Frame, code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let high = code[frame.pc] as i16;
    let low = code[frame.pc + 1] as i16;
    frame.pc += 2;
    let value = ((high << 8) | (low & 0xFF)) as i16;
    frame.stack.push_int(value as i32);
    Ok(())
}

// aconst_null 指令
pub fn exec_aconst_null(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    frame.stack.push_null();
    Ok(())
} 