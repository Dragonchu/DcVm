use crate::error::JvmError;
use crate::jvm_thread::Frame;
use crate::vm::Vm;
use crate::jvm_log;

/// if_icmpge 指令 - 如果第一个int值大于等于第二个int值则跳转
pub fn exec_if_icmpge(frame: &mut Frame, code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    let offset = ((code[frame.pc] as i16) << 8 | (code[frame.pc + 1] as i16)) as i32;
    frame.pc += 2;
    
    let value2 = frame.stack.pop_int();
    let value1 = frame.stack.pop_int();
    
    jvm_log!("if_icmpge: {} >= {} ?", value1, value2);
    
    if value1 >= value2 {
        frame.pc = (frame.pc as i32 + offset - 3) as usize;
        jvm_log!("if_icmpge: 跳转到 {}", frame.pc);
    }
    
    Ok(())
}

/// areturn 指令 - 返回对象引用
pub fn exec_areturn(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    jvm_log!("areturn");
    // 这里可以直接返回Ok(())，实际弹栈在jvm_thread里处理
    Ok(())
} 