use crate::jvm_thread::Frame;
use crate::error::JvmError;
use crate::vm::Vm;

// iload 指令族
pub fn exec_iload(frame: &mut Frame, code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let index = code[frame.pc] as usize;
    frame.pc += 1;
    let value = frame.local_vars.get_int(index);
    frame.stack.push_int(value);
    Ok(())
}

pub fn exec_iload_0(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let value = frame.local_vars.get_int(0);
    frame.stack.push_int(value);
    Ok(())
}

pub fn exec_iload_1(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let value = frame.local_vars.get_int(1);
    frame.stack.push_int(value);
    Ok(())
}

pub fn exec_iload_2(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let value = frame.local_vars.get_int(2);
    frame.stack.push_int(value);
    Ok(())
}

pub fn exec_iload_3(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let value = frame.local_vars.get_int(3);
    frame.stack.push_int(value);
    Ok(())
}

// istore 指令族
pub fn exec_istore(frame: &mut Frame, code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let index = code[frame.pc] as usize;
    frame.pc += 1;
    let value = frame.stack.pop_int();
    frame.local_vars.set_int(index, value);
    Ok(())
}

pub fn exec_istore_0(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let value = frame.stack.pop_int();
    frame.local_vars.set_int(0, value);
    Ok(())
}

pub fn exec_istore_1(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let value = frame.stack.pop_int();
    frame.local_vars.set_int(1, value);
    Ok(())
}

pub fn exec_istore_2(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let value = frame.stack.pop_int();
    frame.local_vars.set_int(2, value);
    Ok(())
}

pub fn exec_istore_3(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let value = frame.stack.pop_int();
    frame.local_vars.set_int(3, value);
    Ok(())
}

// aload 指令族
pub fn exec_aload(frame: &mut Frame, code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let index = code[frame.pc] as usize;
    frame.pc += 1;
    let value = frame.local_vars.get_obj_ref(index);
    frame.stack.push_obj_ref(value);
    Ok(())
}

pub fn exec_aload_0(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let value = frame.local_vars.get_obj_ref(0);
    frame.stack.push_obj_ref(value);
    Ok(())
}

pub fn exec_aload_1(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let value = frame.local_vars.get_obj_ref(1);
    frame.stack.push_obj_ref(value);
    Ok(())
}

pub fn exec_aload_2(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let value = frame.local_vars.get_obj_ref(2);
    frame.stack.push_obj_ref(value);
    Ok(())
}

pub fn exec_aload_3(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let value = frame.local_vars.get_obj_ref(3);
    frame.stack.push_obj_ref(value);
    Ok(())
}

// astore 指令族
pub fn exec_astore(frame: &mut Frame, code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let index = code[frame.pc] as usize;
    frame.pc += 1;
    let value = frame.stack.pop_obj_ref();
    frame.local_vars.set_obj_ref(index, value);
    Ok(())
}

pub fn exec_astore_0(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let value = frame.stack.pop_obj_ref();
    frame.local_vars.set_obj_ref(0, value);
    Ok(())
}

pub fn exec_astore_1(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let value = frame.stack.pop_obj_ref();
    frame.local_vars.set_obj_ref(1, value);
    Ok(())
}

pub fn exec_astore_2(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let value = frame.stack.pop_obj_ref();
    frame.local_vars.set_obj_ref(2, value);
    Ok(())
}

pub fn exec_astore_3(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let value = frame.stack.pop_obj_ref();
    frame.local_vars.set_obj_ref(3, value);
    Ok(())
} 