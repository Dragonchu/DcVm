use crate::jvm_thread::Frame;
use crate::error::JvmError;
use crate::vm::Vm;
use crate::JvmValue;
use crate::jvm_log;

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

/// if_icmpeq 指令 - 如果两个int值相等则跳转
pub fn exec_if_icmpeq(frame: &mut Frame, code: &[u8], _vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let offset = ((code[frame.pc] as i16) << 8 | code[frame.pc + 1] as i16) as i32;
    frame.pc += 2;
    
    let value2 = frame.stack.pop_int();
    let value1 = frame.stack.pop_int();
    
    jvm_log!("if_icmpeq: {} == {} ?", value1, value2);
    
    if value1 == value2 {
        frame.pc = (frame.pc as i32 + offset) as usize;
        jvm_log!("if_icmpeq: 跳转到 {}", frame.pc);
    }
    
    Ok(())
}

/// if_icmpne 指令 - 如果两个int值不相等则跳转
pub fn exec_if_icmpne(frame: &mut Frame, code: &[u8], _vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let offset = ((code[frame.pc] as i16) << 8 | code[frame.pc + 1] as i16) as i32;
    frame.pc += 2;
    
    let value2 = frame.stack.pop_int();
    let value1 = frame.stack.pop_int();
    
    jvm_log!("if_icmpne: {} != {} ?", value1, value2);
    
    if value1 != value2 {
        frame.pc = (frame.pc as i32 + offset) as usize;
        jvm_log!("if_icmpne: 跳转到 {}", frame.pc);
    }
    
    Ok(())
}

/// tableswitch 指令 - 表跳转
pub fn exec_tableswitch(frame: &mut Frame, code: &[u8], _vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    // 对齐到4字节边界
    let padding = (4 - (frame.pc % 4)) % 4;
    frame.pc += padding;
    
    // 读取default offset
    let default_offset = ((code[frame.pc] as i32) << 24 | 
                         (code[frame.pc + 1] as i32) << 16 | 
                         (code[frame.pc + 2] as i32) << 8 | 
                         (code[frame.pc + 3] as i32)) as i32;
    frame.pc += 4;
    
    // 读取low和high值
    let low = ((code[frame.pc] as i32) << 24 | 
               (code[frame.pc + 1] as i32) << 16 | 
               (code[frame.pc + 2] as i32) << 8 | 
               (code[frame.pc + 3] as i32)) as i32;
    frame.pc += 4;
    
    let high = ((code[frame.pc] as i32) << 24 | 
                (code[frame.pc + 1] as i32) << 16 | 
                (code[frame.pc + 2] as i32) << 8 | 
                (code[frame.pc + 3] as i32)) as i32;
    frame.pc += 4;
    
    // 读取跳转表
    let num_cases = (high - low + 1) as usize;
    let mut jump_table = Vec::with_capacity(num_cases);
    
    for _ in 0..num_cases {
        let offset = ((code[frame.pc] as i32) << 24 | 
                     (code[frame.pc + 1] as i32) << 16 | 
                     (code[frame.pc + 2] as i32) << 8 | 
                     (code[frame.pc + 3] as i32)) as i32;
        jump_table.push(offset);
        frame.pc += 4;
    }
    
    let key = frame.stack.pop_int();
    jvm_log!("tableswitch: key={}, low={}, high={}", key, low, high);
    
    // 查找匹配的case
    if key >= low && key <= high {
        let index = (key - low) as usize;
        let offset = jump_table[index];
        frame.pc = (frame.pc as i32 + offset) as usize;
        jvm_log!("tableswitch: 跳转到case {}, offset={}", key, offset);
    } else {
        frame.pc = (frame.pc as i32 + default_offset) as usize;
        jvm_log!("tableswitch: 跳转到default, offset={}", default_offset);
    }
    
    Ok(())
} 