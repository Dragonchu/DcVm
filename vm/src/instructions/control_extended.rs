use crate::error::JvmError;
use crate::jvm_thread::Frame;

pub fn exec_if_icmpge(frame: &mut Frame, code: &[u8], _vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let high = code[frame.pc] as i16;
    let low = code[frame.pc + 1] as i16;
    let offset = ((high << 8) | (low & 0xFF)) as i16;
    frame.pc += 2;
    let v2 = frame.stack.pop_int();
    let v1 = frame.stack.pop_int();
    if v1 >= v2 {
        frame.pc = ((frame.pc as isize) + (offset as isize) - 3) as usize;
    }
    Ok(())
}

pub fn exec_areturn(frame: &mut Frame, _code: &[u8], _vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    crate::jvm_log!("areturn");
    if !frame.stack.is_obj_refs_empty() {
        let obj_ref = frame.stack.pop_obj_ref();
        // 弹出当前frame
        // 注意：这里需要特殊处理，因为我们需要在调用方处理frame的弹出
        // 这里只是标记需要返回，实际的frame弹出在调用方处理
        crate::jvm_log!("[Returning object reference to caller]");
        // 直接return Ok(())，让调用方处理
        return Ok(());
    } else {
        return Err(JvmError::IllegalStateError("areturn: 栈无对象引用".to_string()));
    }
} 