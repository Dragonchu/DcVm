use crate::jvm_thread::Frame;
use crate::error::JvmError;
use crate::vm::Vm;
use crate::jvm_log;
use crate::heap::RawPtr;
use std::collections::HashMap;

// 全局数组存储，用于模拟数组操作
static mut ARRAY_STORAGE: Option<HashMap<RawPtr, Vec<i32>>> = None;

fn get_array_storage() -> &'static mut HashMap<RawPtr, Vec<i32>> {
    unsafe {
        if ARRAY_STORAGE.is_none() {
            ARRAY_STORAGE = Some(HashMap::new());
        }
        ARRAY_STORAGE.as_mut().unwrap()
    }
}

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
        
        // 为int数组初始化存储
        if array_type == 10 { // T_INT
            let storage = get_array_storage();
            storage.insert(array_ptr, vec![0; count as usize]);
        }
        
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
    
    // 从存储中获取数组长度
    let storage = get_array_storage();
    let length = if let Some(array_data) = storage.get(&array_ref) {
        array_data.len() as i32
    } else {
        10 // 默认长度
    };
    
    frame.stack.push_int(length);
    jvm_log!("[ArrayLength] 获取数组长度: {}", length);
    
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
    
    // 存储数组元素
    let mut storage = get_array_storage();
    if let Some(array_data) = storage.get_mut(&array_ref) {
        if (index as usize) < array_data.len() {
            array_data[index as usize] = value;
            jvm_log!("[IAStore] 存储数组元素: 索引={}, 值={}", index, value);
        } else {
            return Err(JvmError::IllegalStateError(format!("Array index out of bounds: {}", index)));
        }
    } else {
        // 如果数组不在存储中，创建一个新的
        let mut new_array = vec![0; (index + 1) as usize];
        new_array[index as usize] = value;
        storage.insert(array_ref, new_array);
        jvm_log!("[IAStore] 创建新数组并存储元素: 索引={}, 值={}", index, value);
    }
    
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
    
    // 从存储中获取数组元素
    let storage = get_array_storage();
    let value = if let Some(array_data) = storage.get(&array_ref) {
        if (index as usize) < array_data.len() {
            array_data[index as usize]
        } else {
            return Err(JvmError::IllegalStateError(format!("Array index out of bounds: {}", index)));
        }
    } else {
        0 // 默认值
    };
    
    frame.stack.push_int(value);
    jvm_log!("[IALoad] 加载数组元素: 索引={}, 值={}", index, value);
    
    Ok(())
} 