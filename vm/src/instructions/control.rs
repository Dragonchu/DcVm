use crate::jvm_thread::Frame;
use crate::error::JvmError;
use crate::vm::Vm;

// goto 指令
pub fn exec_goto(frame: &mut Frame, code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let offset = ((code[frame.pc] as i16) << 8 | (code[frame.pc + 1] as i16)) as i32;
    frame.pc += 2;
    frame.pc = (frame.pc as i32 + offset - 3) as usize; // -3 是因为我们已经读取了opcode和offset
    Ok(())
}

// ifeq 指令
pub fn exec_ifeq(frame: &mut Frame, code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let offset = ((code[frame.pc] as i16) << 8 | (code[frame.pc + 1] as i16)) as i32;
    frame.pc += 2;
    let value = frame.stack.pop_int();
    if value == 0 {
        frame.pc = (frame.pc as i32 + offset - 3) as usize;
    }
    Ok(())
}

// ifne 指令
pub fn exec_ifne(frame: &mut Frame, code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let offset = ((code[frame.pc] as i16) << 8 | (code[frame.pc + 1] as i16)) as i32;
    frame.pc += 2;
    let value = frame.stack.pop_int();
    if value != 0 {
        frame.pc = (frame.pc as i32 + offset - 3) as usize;
    }
    Ok(())
}

// ifge 指令
pub fn exec_ifge(frame: &mut Frame, code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let offset = ((code[frame.pc] as i16) << 8 | (code[frame.pc + 1] as i16)) as i32;
    frame.pc += 2;
    let value = frame.stack.pop_int();
    if value >= 0 {
        frame.pc = (frame.pc as i32 + offset - 3) as usize;
    }
    Ok(())
}

// return 指令
pub fn exec_return(_frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    // 这里可以直接返回Ok(())，实际弹栈在jvm_thread里处理
    Ok(())
} 