use crate::jvm_thread::Frame;
use crate::error::JvmError;
use crate::vm::Vm;
use crate::JvmValue;
use crate::heap::RawPtr;
use crate::jvm_log;
use reader::constant_pool::ConstantPool;

pub fn exec_getstatic(frame: &mut Frame, code: &[u8], mut vm: Option<&mut Vm>, method: &crate::method::Method) -> Result<(), JvmError> {
    let index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
    frame.pc += 2;
    jvm_log!("getstatic {}", index);
    
    // 从常量池获取字段引用
    let cp = &method.constant_pool;
    if let reader::constant_pool::CpInfo::FieldRef { class_index, name_and_type_index, .. } = &cp[index - 1] {
        // 获取类名
        let class_name = if let reader::constant_pool::CpInfo::Class { name_index, .. } = &cp[(*class_index - 1) as usize] {
            cp.get_utf8_string(*name_index)
        } else {
            return Err(JvmError::IllegalStateError("Invalid class reference".to_string()));
        };
        
        // 获取字段名和描述符
        let name_and_type = if let reader::constant_pool::CpInfo::NameAndType { name_index, descriptor_index, .. } = &cp[(*name_and_type_index - 1) as usize] {
            let field_name = cp.get_utf8_string(*name_index);
            let field_desc = cp.get_utf8_string(*descriptor_index);
            (field_name, field_desc)
        } else {
            return Err(JvmError::IllegalStateError("Invalid name and type reference".to_string()));
        };
        
        jvm_log!("Getting static field: {}.{}", class_name, name_and_type.0);
        
        // 处理System.out字段
        if class_name == "java/lang/System" && name_and_type.0 == "out" {
            // 创建一个假的PrintStream对象引用（简化实现）
            let fake_ptr = RawPtr(std::ptr::null_mut());
            frame.stack.push_obj_ref(fake_ptr);
            jvm_log!("[Pushed System.out object]");
        } else {
            // 其他静态字段，从VM的静态字段存储中获取
            if let Some(ref mut vm) = vm {
                if let Some(field_value) = vm.get_static_field(&class_name, &name_and_type.0) {
                    match field_value {
                        JvmValue::Int(value) => {
                            jvm_log!("[Getting static field {}.{} = {}]", class_name, name_and_type.0, value);
                            frame.stack.push_int(*value as i32);
                        }
                        JvmValue::Long(value) => {
                            jvm_log!("[Getting static field {}.{} = {}]", class_name, name_and_type.0, value);
                            frame.stack.push_int((*value & 0xFFFF_FFFF) as i32);
                        }
                        JvmValue::Float(value) => {
                            let float_value = f32::from_bits(*value as u32);
                            jvm_log!("[Getting static field {}.{} = {}]", class_name, name_and_type.0, float_value);
                            frame.stack.push_int(float_value.to_bits() as i32);
                        }
                        JvmValue::Double(value) => {
                            let double_value = f64::from_bits(*value);
                            jvm_log!("[Getting static field {}.{} = {}]", class_name, name_and_type.0, double_value);
                            // 推入高32位
                            frame.stack.push_int((*value >> 32) as i32);
                            // 推入低32位
                            frame.stack.push_int((*value & 0xFFFF_FFFF) as i32);
                        }
                        JvmValue::Boolean(value) => {
                            jvm_log!("[Getting static field {}.{} = {}]", class_name, name_and_type.0, value);
                            frame.stack.push_int(*value as i32);
                        }
                        JvmValue::Char(value) => {
                            jvm_log!("[Getting static field {}.{} = {}]", class_name, name_and_type.0, value);
                            frame.stack.push_int(*value as i32);
                        }
                        JvmValue::ObjRef(ptr) => {
                            jvm_log!("[Getting static field {}.{} = {:?}]", class_name, name_and_type.0, ptr);
                            frame.stack.push_obj_ref(*ptr);
                        }
                        JvmValue::Null => {
                            jvm_log!("[Getting static field {}.{} = null]", class_name, name_and_type.0);
                            frame.stack.push_obj_ref(RawPtr(std::ptr::null_mut()));
                        }
                        _ => {
                            jvm_log!("[Getting static field {}.{} = unsupported type]", class_name, name_and_type.0);
                            frame.stack.push_obj_ref(RawPtr(std::ptr::null_mut()));
                        }
                    }
                } else {
                    // 字段不存在，根据字段类型推入默认值
                    match name_and_type.1.as_str() {
                        "I" | "S" | "B" | "Z" => {
                            jvm_log!("[Static field {}.{} not found, pushing 0]", class_name, name_and_type.0);
                            frame.stack.push_int(0);
                        }
                        "J" => {
                            jvm_log!("[Static field {}.{} not found, pushing 0L]", class_name, name_and_type.0);
                            frame.stack.push_int(0);
                            frame.stack.push_int(0);
                        }
                        "F" => {
                            jvm_log!("[Static field {}.{} not found, pushing 0.0f]", class_name, name_and_type.0);
                            frame.stack.push_int(0);
                        }
                        "D" => {
                            jvm_log!("[Static field {}.{} not found, pushing 0.0]", class_name, name_and_type.0);
                            frame.stack.push_int(0);
                            frame.stack.push_int(0);
                        }
                        "C" => {
                            jvm_log!("[Static field {}.{} not found, pushing '\\0']", class_name, name_and_type.0);
                            frame.stack.push_int(0);
                        }
                        _ => {
                            jvm_log!("[Static field {}.{} not found, pushing null]", class_name, name_and_type.0);
                            frame.stack.push_obj_ref(RawPtr(std::ptr::null_mut()));
                        }
                    }
                }
            } else {
                // 从操作数栈弹出值，如果栈为空则使用默认值
                let value = if frame.stack.is_values_empty() && frame.stack.is_obj_refs_empty() {
                    // 静态初始化时栈为空，使用字段的默认值
                    jvm_log!("[Static init: stack empty, using default value for {}.{}]", class_name, name_and_type.0);
                    0 // 默认值
                } else {
                    frame.stack.pop_int()
                };
            }
        }
    } else {
        return Err(JvmError::IllegalStateError(format!("getstatic: 常量池索引{}不是字段引用", index)));
    }
    Ok(())
}

pub fn exec_putstatic(frame: &mut Frame, code: &[u8], mut vm: Option<&mut Vm>, method: &crate::method::Method) -> Result<(), JvmError> {
    let index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
    frame.pc += 2;
    jvm_log!("putstatic {}", index);
    
    // 从常量池获取字段引用
    let cp = &method.constant_pool;
    if let reader::constant_pool::CpInfo::FieldRef { class_index, name_and_type_index, .. } = &cp[index - 1] {
        // 获取类名
        let class_name = if let reader::constant_pool::CpInfo::Class { name_index, .. } = &cp[(*class_index - 1) as usize] {
            cp.get_utf8_string(*name_index)
        } else {
            return Err(JvmError::IllegalStateError("Invalid class reference".to_string()));
        };
        
        // 获取字段名和描述符
        let name_and_type = if let reader::constant_pool::CpInfo::NameAndType { name_index, descriptor_index, .. } = &cp[(*name_and_type_index - 1) as usize] {
            let field_name = cp.get_utf8_string(*name_index);
            let field_desc = cp.get_utf8_string(*descriptor_index);
            (field_name, field_desc)
        } else {
            return Err(JvmError::IllegalStateError("Invalid name and type reference".to_string()));
        };
        
        jvm_log!("Setting static field: {}.{}", class_name, name_and_type.0);
        
        // 根据字段类型从栈中弹出值
        let field_value = match name_and_type.1.as_str() {
            "I" | "S" | "B" | "Z" => {
                if frame.stack.is_values_empty() {
                    jvm_log!("[Static init: stack empty, using default value for {}.{}]", class_name, name_and_type.0);
                    JvmValue::Int(0)
                } else {
                    JvmValue::Int(frame.stack.pop_int() as u32)
                }
            }
            "J" => {
                if frame.stack.is_values_empty() {
                    jvm_log!("[Static init: stack empty, using default value for {}.{}]", class_name, name_and_type.0);
                    JvmValue::Long(0)
                } else {
                    let low = frame.stack.pop_int() as u32 as u64;
                    let high = frame.stack.pop_int() as u32 as u64;
                    JvmValue::Long((high << 32) | (low & 0xFFFF_FFFF))
                }
            }
            "F" => {
                if frame.stack.is_values_empty() {
                    jvm_log!("[Static init: stack empty, using default value for {}.{}]", class_name, name_and_type.0);
                    JvmValue::Float(0)
                } else {
                    JvmValue::Float((frame.stack.pop_int() as u32) as u64)
                }
            }
            "D" => {
                if frame.stack.is_values_empty() {
                    jvm_log!("[Static init: stack empty, using default value for {}.{}]", class_name, name_and_type.0);
                    JvmValue::Double(0)
                } else {
                    let low = frame.stack.pop_int() as u32 as u64;
                    let high = frame.stack.pop_int() as u32 as u64;
                    JvmValue::Double((high << 32) | (low & 0xFFFF_FFFF))
                }
            }
            "C" => {
                if frame.stack.is_values_empty() {
                    jvm_log!("[Static init: stack empty, using default value for {}.{}]", class_name, name_and_type.0);
                    JvmValue::Char(0)
                } else {
                    JvmValue::Char(frame.stack.pop_int() as u16)
                }
            }
            desc if desc.starts_with("L") || desc.starts_with("[") => {
                if frame.stack.is_obj_refs_empty() {
                    jvm_log!("[Static init: stack empty, using default value for {}.{}]", class_name, name_and_type.0);
                    JvmValue::Null
                } else {
                    JvmValue::ObjRef(frame.stack.pop_obj_ref())
                }
            }
            _ => {
                jvm_log!("[putstatic] Unsupported field type: {}", name_and_type.1);
                JvmValue::Null
            }
        };
        
        // 使用VM的静态字段存储功能
        if let Some(ref mut vm) = vm {
            vm.set_static_field(&class_name, &name_and_type.0, field_value);
            jvm_log!("Setting static field {}.{}", class_name, name_and_type.0);
        }
    } else {
        return Err(JvmError::IllegalStateError(format!("putstatic: 常量池索引{}不是字段引用", index)));
    }
    Ok(())
} 