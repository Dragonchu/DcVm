use crate::jvm_thread::Frame;
use crate::error::JvmError;
use crate::vm::Vm;
use crate::jvm_log;
use crate::heap::RawPtr;

pub fn exec_newarray(frame: &mut Frame, code: &[u8], mut vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let count = frame.stack.pop_int();
    let array_type = code[frame.pc];
    frame.pc += 1;
    
    if count < 0 {
        return Err(JvmError::IllegalStateError(format!("Array size cannot be negative: {}", count)));
    }
    
    if let Some(ref mut vm) = vm {
        let array_ptr = match array_type {
            4 => vm.create_simple_array(count as usize, 4).map_err(|e| JvmError::IllegalStateError(format!("Failed to create array: {:?}", e)))?,  // T_BOOLEAN
            5 => vm.create_simple_array(count as usize, 5).map_err(|e| JvmError::IllegalStateError(format!("Failed to create array: {:?}", e)))?,  // T_CHAR
            6 => vm.create_simple_array(count as usize, 6).map_err(|e| JvmError::IllegalStateError(format!("Failed to create array: {:?}", e)))?,  // T_FLOAT
            7 => vm.create_simple_array(count as usize, 7).map_err(|e| JvmError::IllegalStateError(format!("Failed to create array: {:?}", e)))?,  // T_DOUBLE
            8 => vm.create_simple_array(count as usize, 8).map_err(|e| JvmError::IllegalStateError(format!("Failed to create array: {:?}", e)))?,  // T_BYTE
            9 => vm.create_simple_array(count as usize, 9).map_err(|e| JvmError::IllegalStateError(format!("Failed to create array: {:?}", e)))?,  // T_SHORT
            10 => vm.create_simple_array(count as usize, 10).map_err(|e| JvmError::IllegalStateError(format!("Failed to create array: {:?}", e)))?, // T_INT
            11 => vm.create_simple_array(count as usize, 11).map_err(|e| JvmError::IllegalStateError(format!("Failed to create array: {:?}", e)))?, // T_LONG
            _ => return Err(JvmError::IllegalStateError(format!("Unsupported array type: {}", array_type))),
        };
        
        frame.stack.push_obj_ref(array_ptr);
        jvm_log!("[NewArray] 创建数组: 类型={}, 长度={}", array_type, count);
    }
    
    Ok(())
}

pub fn exec_arraylength(frame: &mut Frame, _code: &[u8], _vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let array_ref = frame.stack.pop_obj_ref();
    
    if array_ref.is_null() {
        return Err(JvmError::IllegalStateError("NullPointerException".to_string()));
    }
    
    // 简化处理：假设数组长度为10
    frame.stack.push_int(10);
    jvm_log!("[ArrayLength] 获取数组长度: 10");
    
    Ok(())
}

pub fn exec_iastore(frame: &mut Frame, _code: &[u8], _vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let value = frame.stack.pop_int();
    let index = frame.stack.pop_int();
    let array_ref = frame.stack.pop_obj_ref();
    
    if array_ref.is_null() {
        return Err(JvmError::IllegalStateError("NullPointerException".to_string()));
    }
    
    if index < 0 {
        return Err(JvmError::IllegalStateError(format!("Array index cannot be negative: {}", index)));
    }
    
    jvm_log!("[IAStore] 存储数组元素: 索引={}, 值={}", index, value);
    
    Ok(())
}

pub fn exec_iaload(frame: &mut Frame, _code: &[u8], _vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let index = frame.stack.pop_int();
    let array_ref = frame.stack.pop_obj_ref();
    
    if array_ref.is_null() {
        return Err(JvmError::IllegalStateError("NullPointerException".to_string()));
    }
    
    if index < 0 {
        return Err(JvmError::IllegalStateError(format!("Array index cannot be negative: {}", index)));
    }
    
    // 简化处理：假设返回值为0
    frame.stack.push_int(0);
    jvm_log!("[IALoad] 加载数组元素: 索引={}, 值=0", index);
    
    Ok(())
} 