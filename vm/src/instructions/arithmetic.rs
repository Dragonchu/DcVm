use crate::jvm_thread::Frame;
use crate::error::JvmError;
use crate::vm::Vm;

pub fn exec_iadd(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let b = frame.stack.pop_int();
    let a = frame.stack.pop_int();
    frame.stack.push_int(a + b);
    Ok(())
}

pub fn exec_isub(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let b = frame.stack.pop_int();
    let a = frame.stack.pop_int();
    frame.stack.push_int(a - b);
    Ok(())
}

pub fn exec_imul(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let b = frame.stack.pop_int();
    let a = frame.stack.pop_int();
    frame.stack.push_int(a * b);
    Ok(())
}

pub fn exec_idiv(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let b = frame.stack.pop_int();
    let a = frame.stack.pop_int();
    
    if b == 0 {
        return Err(JvmError::ArithmeticError("Division by zero".to_string()));
    }
    
    frame.stack.push_int(a / b);
    Ok(())
}

pub fn exec_irem(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let v2 = frame.stack.pop_int();
    let v1 = frame.stack.pop_int();
    if v2 == 0 {
        return Err(JvmError::ArithmeticError("Modulo by zero".to_string()));
    }
    frame.stack.push_int(v1 % v2);
    Ok(())
}

pub fn exec_ineg(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let v = frame.stack.pop_int();
    frame.stack.push_int(-v);
    Ok(())
}

pub fn exec_iand(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let v2 = frame.stack.pop_int();
    let v1 = frame.stack.pop_int();
    frame.stack.push_int(v1 & v2);
    Ok(())
}

pub fn exec_ior(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let v2 = frame.stack.pop_int();
    let v1 = frame.stack.pop_int();
    frame.stack.push_int(v1 | v2);
    Ok(())
}

pub fn exec_ixor(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let v2 = frame.stack.pop_int();
    let v1 = frame.stack.pop_int();
    frame.stack.push_int(v1 ^ v2);
    Ok(())
}

pub fn exec_ishl(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let v2 = frame.stack.pop_int();
    let v1 = frame.stack.pop_int();
    frame.stack.push_int(v1 << (v2 & 0x1F));
    Ok(())
}

pub fn exec_ishr(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let v2 = frame.stack.pop_int();
    let v1 = frame.stack.pop_int();
    frame.stack.push_int(v1 >> (v2 & 0x1F));
    Ok(())
}

pub fn exec_iushr(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let v2 = frame.stack.pop_int();
    let v1 = frame.stack.pop_int();
    frame.stack.push_int((v1 as u32 >> (v2 & 0x1F)) as i32);
    Ok(())
} 