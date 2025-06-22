use crate::jvm_thread::Frame;
use crate::error::JvmError;
use crate::vm::Vm;

// pop 指令
pub fn exec_pop(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    // 根据栈顶内容的类型来决定弹出什么
    if !frame.stack.is_values_empty() {
        frame.stack.pop_int();
    } else if !frame.stack.is_obj_refs_empty() {
        frame.stack.pop_obj_ref();
    }
    Ok(())
}

// pop2 指令
pub fn exec_pop2(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    // 弹出两个值或一个long/double
    if !frame.stack.is_values_empty() {
        frame.stack.pop_int();
        if !frame.stack.is_values_empty() {
            frame.stack.pop_int();
        }
    } else if !frame.stack.is_obj_refs_empty() {
        frame.stack.pop_obj_ref();
        if !frame.stack.is_obj_refs_empty() {
            frame.stack.pop_obj_ref();
        }
    }
    Ok(())
}

// dup 指令
pub fn exec_dup(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    crate::jvm_log!("dup");
    // 优先检查对象引用栈，然后检查值栈
    if !frame.stack.is_obj_refs_empty() {
        let val = frame.stack.peek_obj_ref();
        frame.stack.push_obj_ref(val);
    } else if !frame.stack.is_values_empty() {
        let val = frame.stack.peek_int();
        frame.stack.push_int(val);
    } else {
        return Err(JvmError::IllegalStateError("dup: 栈为空".to_string()));
    }
    Ok(())
}

// dup_x1 指令
pub fn exec_dup_x1(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    // 复制栈顶值并插入到栈顶第二个值下面
    if !frame.stack.is_values_empty() {
        let value1 = frame.stack.pop_int();
        let value2 = frame.stack.pop_int();
        frame.stack.push_int(value1);
        frame.stack.push_int(value2);
        frame.stack.push_int(value1);
    } else if !frame.stack.is_obj_refs_empty() {
        let value1 = frame.stack.pop_obj_ref();
        let value2 = frame.stack.pop_obj_ref();
        frame.stack.push_obj_ref(value1);
        frame.stack.push_obj_ref(value2);
        frame.stack.push_obj_ref(value1);
    }
    Ok(())
}

// dup_x2 指令
pub fn exec_dup_x2(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    // 复制栈顶值并插入到栈顶第三个值下面
    if !frame.stack.is_values_empty() {
        let value1 = frame.stack.pop_int();
        let value2 = frame.stack.pop_int();
        let value3 = frame.stack.pop_int();
        frame.stack.push_int(value1);
        frame.stack.push_int(value3);
        frame.stack.push_int(value2);
        frame.stack.push_int(value1);
    } else if !frame.stack.is_obj_refs_empty() {
        let value1 = frame.stack.pop_obj_ref();
        let value2 = frame.stack.pop_obj_ref();
        let value3 = frame.stack.pop_obj_ref();
        frame.stack.push_obj_ref(value1);
        frame.stack.push_obj_ref(value3);
        frame.stack.push_obj_ref(value2);
        frame.stack.push_obj_ref(value1);
    }
    Ok(())
}

// dup2 指令
pub fn exec_dup2(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    // 复制栈顶两个值
    if !frame.stack.is_values_empty() {
        let value1 = frame.stack.pop_int();
        if !frame.stack.is_values_empty() {
            let value2 = frame.stack.pop_int();
            frame.stack.push_int(value2);
            frame.stack.push_int(value1);
            frame.stack.push_int(value2);
            frame.stack.push_int(value1);
        } else {
            frame.stack.push_int(value1);
            frame.stack.push_int(value1);
        }
    } else if !frame.stack.is_obj_refs_empty() {
        let value1 = frame.stack.pop_obj_ref();
        if !frame.stack.is_obj_refs_empty() {
            let value2 = frame.stack.pop_obj_ref();
            frame.stack.push_obj_ref(value2);
            frame.stack.push_obj_ref(value1);
            frame.stack.push_obj_ref(value2);
            frame.stack.push_obj_ref(value1);
        } else {
            frame.stack.push_obj_ref(value1);
            frame.stack.push_obj_ref(value1);
        }
    }
    Ok(())
}

// swap 指令
pub fn exec_swap(frame: &mut Frame, _code: &[u8], _vm: Option<&mut Vm>) -> Result<(), JvmError> {
    frame.stack.swap_top_two_ints();
    Ok(())
} 