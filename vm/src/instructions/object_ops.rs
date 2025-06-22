use crate::error::JvmError;
use crate::jvm_thread::Frame;
use crate::vm::Vm;
use crate::jvm_log;
use reader::constant_pool::{ConstantPool, ConstantPoolExt};
use crate::JvmValue;
use crate::heap::RawPtr;

pub fn exec_new(frame: &mut Frame, code: &[u8], vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
    frame.pc += 2;
    let cp = &frame.method.constant_pool;
    let class_name = cp.get_class_name(index as u16);
    jvm_log!("[New] 创建对象: {}", class_name);
    
    if let Some(vm) = vm {
        // 加载类，分配对象
        let klass = vm.load(&class_name)?;
        if let crate::class::Klass::Instance(instance_klass) = &klass {
            let obj_ptr = vm.heap.borrow_mut().alloc_object(instance_klass)
                .map_err(|e| JvmError::IllegalStateError(format!("alloc_object失败: {:?}", e)))?;
            frame.stack.push_obj_ref(obj_ptr);
            jvm_log!("[New] 推入对象引用: {:?}", obj_ptr);
        } else {
            return Err(JvmError::IllegalStateError(format!("new: {} 不是实例类", class_name)));
        }
    } else {
        // 没有VM实例，创建一个假的对象引用
        let fake_obj_ptr = crate::heap::RawPtr(Box::into_raw(Box::new(())) as *mut u8);
        frame.stack.push_obj_ref(fake_obj_ptr);
        jvm_log!("[New] 推入对象引用: {:?}", fake_obj_ptr);
    }
    Ok(())
}

pub fn exec_getfield(frame: &mut Frame, code: &[u8], vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
    frame.pc += 2;
    jvm_log!("getfield {}", index);
    if frame.stack.is_obj_refs_empty() {
        return Err(JvmError::IllegalStateError("getfield: 栈无对象引用".to_string()));
    }
    let obj_ref = frame.stack.pop_obj_ref();
    let cp = &frame.method.constant_pool;
    let (class_name, field_name, field_desc) = cp.get_fieldref_info(index as u16);
    jvm_log!("[GetField] 访问字段: {}.{}{}", class_name, field_name, field_desc);
    if obj_ref.is_null() {
        return Err(JvmError::NullPointerError("getfield: 对象引用为null".to_string()));
    }
    if let Some(vm) = vm {
        let klass = vm.load(&class_name)?;
        if let crate::class::Klass::Instance(instance_klass) = &klass {
            let fields = instance_klass.get_instance_fields();
            let mut field_offset = None;
            for f in fields.iter() {
                if f.get_name() == field_name {
                    field_offset = Some(f.get_offset());
                    break;
                }
            }
            if let Some(offset) = field_offset {
                let value = vm.heap.borrow().get_field(obj_ref, offset, &field_desc);
                match value {
                    JvmValue::Int(v) => frame.stack.push_int(v as i32),
                    JvmValue::Long(v) => frame.stack.push_int(v as i32),
                    JvmValue::Float(v) => frame.stack.push_int(v as i32),
                    JvmValue::Double(v) => frame.stack.push_int(v as i32),
                    JvmValue::Char(v) => frame.stack.push_int(v as i32),
                    JvmValue::ObjRef(ptr) => frame.stack.push_obj_ref(ptr),
                    JvmValue::Null => frame.stack.push_obj_ref(RawPtr(std::ptr::null_mut())),
                    _ => frame.stack.push_obj_ref(RawPtr(std::ptr::null_mut())),
                }
            } else {
                return Err(JvmError::IllegalStateError(format!("getfield: 找不到字段 {}", field_name)));
            }
        }
    }
    Ok(())
}

pub fn exec_putfield(frame: &mut Frame, code: &[u8], vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
    frame.pc += 2;
    let cp = &frame.method.constant_pool;
    match &cp[index - 1] {
        reader::constant_pool::CpInfo::FieldRef { class_index, name_and_type_index, .. } => {
            let class_name = cp.get_class_name(*class_index);
            let name_and_type = &cp[*name_and_type_index as usize - 1];
            match name_and_type {
                reader::constant_pool::CpInfo::NameAndType { name_index, descriptor_index, .. } => {
                    let field_name = cp.get_utf8_string(*name_index);
                    let field_desc = cp.get_utf8_string(*descriptor_index);
                    jvm_log!("[PutField] 设置字段: {}.{}{}", class_name, field_name, field_desc);
                    
                    // 先弹出对象引用，确保栈中有对象引用
                    let obj_ref = if !frame.stack.is_obj_refs_empty() {
                        frame.stack.pop_obj_ref()
                    } else {
                        jvm_log!("[PutField] 警告: 栈中没有对象引用，使用null");
                        RawPtr(std::ptr::null_mut())
                    };
                    
                    if obj_ref.is_null() {
                        return Err(JvmError::NullPointerError("putfield: 对象引用为null".to_string()));
                    }
                    
                    // 弹出值
                    let value = match field_desc.as_str() {
                        "I" | "S" | "B" | "Z" => {
                            if !frame.stack.is_values_empty() {
                                JvmValue::Int(frame.stack.pop_int() as u32)
                            } else {
                                jvm_log!("[PutField] Stack empty for {}.{}, using default", class_name, field_name);
                                JvmValue::Int(0)
                            }
                        },
                        "J" => {
                            if frame.stack.is_values_empty() {
                                jvm_log!("[PutField] Stack empty for {}.{}, using default", class_name, field_name);
                                JvmValue::Long(0)
                            } else {
                                let low = frame.stack.pop_int() as u32 as u64;
                                let high = if !frame.stack.is_values_empty() {
                                    frame.stack.pop_int() as u32 as u64
                                } else {
                                    0
                                };
                                JvmValue::Long((high << 32) | (low & 0xFFFF_FFFF))
                            }
                        },
                        "F" => {
                            if !frame.stack.is_values_empty() {
                                JvmValue::Float((frame.stack.pop_int() as u32) as u64)
                            } else {
                                jvm_log!("[PutField] Stack empty for {}.{}, using default", class_name, field_name);
                                JvmValue::Float(0)
                            }
                        },
                        "D" => {
                            if frame.stack.is_values_empty() {
                                jvm_log!("[PutField] Stack empty for {}.{}, using default", class_name, field_name);
                                JvmValue::Double(0)
                            } else {
                                let low = frame.stack.pop_int() as u32 as u64;
                                let high = if !frame.stack.is_values_empty() {
                                    frame.stack.pop_int() as u32 as u64
                                } else {
                                    0
                                };
                                JvmValue::Double((high << 32) | (low & 0xFFFF_FFFF))
                            }
                        },
                        "C" => {
                            if !frame.stack.is_values_empty() {
                                JvmValue::Char(frame.stack.pop_int() as u16)
                            } else {
                                jvm_log!("[PutField] Stack empty for {}.{}, using default", class_name, field_name);
                                JvmValue::Char(0)
                            }
                        },
                        desc if desc.starts_with("L") || desc.starts_with("[") => {
                            if !frame.stack.is_obj_refs_empty() {
                                JvmValue::ObjRef(frame.stack.pop_obj_ref())
                            } else {
                                jvm_log!("[PutField] Stack empty for {}.{}, using null", class_name, field_name);
                                JvmValue::Null
                            }
                        },
                        _ => {
                            jvm_log!("[PutField] Unsupported field type: {}", field_desc);
                            JvmValue::Null
                        },
                    };
                    
                    // 获取字段偏移并设置字段值
                    if let Some(vm) = vm {
                        let klass = vm.load(&class_name)?;
                        if let crate::class::Klass::Instance(instance_klass) = &klass {
                            let fields = instance_klass.get_instance_fields();
                            let mut field_offset = None;
                            for f in fields.iter() {
                                if f.get_name() == field_name {
                                    field_offset = Some(f.get_offset());
                                    break;
                                }
                            }
                            if let Some(offset) = field_offset {
                                jvm_log!("[PutField] Setting field {} at offset {}", field_name, offset);
                                vm.heap.borrow_mut().put_field(obj_ref, offset, value);
                            } else {
                                jvm_log!("[PutField] Field not found: {}.{}", class_name, field_name);
                                return Err(JvmError::IllegalStateError(format!("putfield: 找不到字段 {}", field_name)));
                            }
                        }
                    }
                    Ok(())
                }
                _ => Err(JvmError::IllegalStateError("Invalid NameAndType in constant pool".to_string())),
            }
        }
        _ => Err(JvmError::IllegalStateError("Invalid FieldRef in constant pool".to_string())),
    }
} 