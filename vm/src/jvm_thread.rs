use crate::error::JvmError;
use crate::heap::Heap;
use crate::method::Method;
use crate::operand_stack::OperandStack;
use crate::local_vars::LocalVars;
use crate::JvmValue;
use crate::heap::RawPtr;
use crate::jvm_log;
use reader::constant_pool::ConstantPool;

pub struct JvmThread {
    pub pc: usize,
    pub stack: OperandStack,
    pub local_vars: LocalVars,
}

impl JvmThread {
    pub fn new(max_stack: usize, max_locals: usize) -> Self {
        JvmThread {
            pc: 0,
            stack: OperandStack::new(max_stack),
            local_vars: LocalVars::new(max_locals),
        }
    }

    pub fn execute(&mut self, method: &Method, heap: &mut Heap, vm: &mut crate::vm::Vm) -> Result<(), JvmError> {
        let code = method.get_code();
        while self.pc < code.len() {
            let opcode = code[self.pc];
            self.pc += 1;

            match opcode {
                0x00 => (), // nop
                0x01 => self.stack.push_null(),
                0x02 => self.stack.push_int(-1),
                0x03 => self.stack.push_int(0),
                0x04 => self.stack.push_int(1),
                0x05 => self.stack.push_int(2),
                0x06 => self.stack.push_int(3),
                0x07 => self.stack.push_int(4),
                0x08 => self.stack.push_int(5),
                0x10 => {
                    let byte = code[self.pc] as i8;
                    self.pc += 1;
                    self.stack.push_int(byte as i32);
                }
                0x11 => {
                    // sipush
                    let high = code[self.pc] as i16;
                    let low = code[self.pc + 1] as i16;
                    self.pc += 2;
                    let value = ((high << 8) | (low & 0xFF)) as i16;
                    self.stack.push_int(value as i32);
                }
                0x15 => {
                    let index = code[self.pc] as usize;
                    self.pc += 1;
                    let value = self.local_vars.get_int(index);
                    self.stack.push_int(value);
                }
                0x1a => {
                    let value = self.local_vars.get_int(0);
                    self.stack.push_int(value);
                }
                0x1b => {
                    let value = self.local_vars.get_int(1);
                    self.stack.push_int(value);
                }
                0x1c => {
                    let value = self.local_vars.get_int(2);
                    self.stack.push_int(value);
                }
                0x1d => {
                    let value = self.local_vars.get_int(3);
                    self.stack.push_int(value);
                }
                0x2a => {
                    let value = self.local_vars.get_int(0);
                    self.stack.push_int(value);
                }
                0x2b => {
                    let value = self.local_vars.get_int(1);
                    self.stack.push_int(value);
                }
                0x2c => {
                    let value = self.local_vars.get_int(2);
                    self.stack.push_int(value);
                }
                0x2d => {
                    let value = self.local_vars.get_int(3);
                    self.stack.push_int(value);
                }
                0x36 => {
                    let index = code[self.pc] as usize;
                    self.pc += 1;
                    let value = self.stack.pop_int();
                    self.local_vars.set_int(index, value);
                }
                0x3b => {
                    let value = self.stack.pop_int();
                    self.local_vars.set_int(0, value);
                }
                0x3c => {
                    let value = self.stack.pop_int();
                    self.local_vars.set_int(1, value);
                }
                0x3d => {
                    let value = self.stack.pop_int();
                    self.local_vars.set_int(2, value);
                }
                0x3e => {
                    let value = self.stack.pop_int();
                    self.local_vars.set_int(3, value);
                }
                0x4b => {
                    let obj_ref = self.stack.pop_obj_ref();
                }
                0x4c => {
                    let obj_ref = self.stack.pop_obj_ref();
                }
                0x4d => {
                    let obj_ref = self.stack.pop_obj_ref();
                }
                0x4e => {
                    let obj_ref = self.stack.pop_obj_ref();
                }
                0x60 => {
                    let v2 = self.stack.pop_int();
                    let v1 = self.stack.pop_int();
                    self.stack.push_int(v1 + v2);
                }
                0x64 => {
                    let v2 = self.stack.pop_int();
                    let v1 = self.stack.pop_int();
                    self.stack.push_int(v1 - v2);
                }
                0x68 => {
                    let v2 = self.stack.pop_int();
                    let v1 = self.stack.pop_int();
                    self.stack.push_int(v1 * v2);
                }
                0x6c => {
                    let v2 = self.stack.pop_int();
                    let v1 = self.stack.pop_int();
                    if v2 == 0 {
                        return Err(JvmError::ArithmeticError("Division by zero".to_string()));
                    }
                    self.stack.push_int(v1 / v2);
                }
                0x84 => {
                    let index = code[self.pc] as usize;
                    let const_val = code[self.pc + 1] as i8;
                    self.pc += 2;
                    let value = self.local_vars.get_int(index);
                    self.local_vars.set_int(index, value + const_val as i32);
                }
                0x99 => {
                    let high = code[self.pc] as i16;
                    let low = code[self.pc + 1] as i16;
                    let offset = ((high << 8) | (low & 0xFF)) as i16;
                    self.pc += 2;
                    let value = self.stack.pop_int();
                    if value == 0 {
                        self.pc = ((self.pc as isize) + (offset as isize) - 3) as usize;
                    }
                }
                0x9a => {
                    let high = code[self.pc] as i16;
                    let low = code[self.pc + 1] as i16;
                    let offset = ((high << 8) | (low & 0xFF)) as i16;
                    self.pc += 2;
                    let value = self.stack.pop_int();
                    if value != 0 {
                        self.pc = ((self.pc as isize) + (offset as isize) - 3) as usize;
                    }
                }
                0x9f => {
                    let high = code[self.pc] as i16;
                    let low = code[self.pc + 1] as i16;
                    let offset = ((high << 8) | (low & 0xFF)) as i16;
                    self.pc += 2;
                    let value = self.stack.pop_int();
                    if value >= 0 {
                        self.pc = ((self.pc as isize) + (offset as isize) - 3) as usize;
                    }
                }
                0xa7 => {
                    let high = code[self.pc] as i16;
                    let low = code[self.pc + 1] as i16;
                    let offset = ((high << 8) | (low & 0xFF)) as i16;
                    self.pc += 2;
                    self.pc = ((self.pc as isize) + (offset as isize) - 3) as usize;
                }
                0xac => {
                    return Ok(());
                }
                0xb1 => return Ok(()), // return
                0xb2 => {
                    // getstatic
                    let index = ((code[self.pc] as u16) << 8 | code[self.pc + 1] as u16) as usize;
                    self.pc += 2;
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
                            self.stack.push_obj_ref(fake_ptr);
                            jvm_log!("[Pushed System.out object]");
                        } else {
                            // 其他静态字段，简化处理
                            let fake_ptr = RawPtr(std::ptr::null_mut());
                            self.stack.push_obj_ref(fake_ptr);
                        }
                    } else {
                        return Err(JvmError::IllegalStateError(format!("getstatic: 常量池索引{}不是字段引用", index)));
                    }
                }
                0xb3 => {
                    // putstatic
                    let index = ((code[self.pc] as u16) << 8 | code[self.pc + 1] as u16) as usize;
                    self.pc += 2;
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
                        
                        // 从操作数栈弹出值
                        let value = self.stack.pop_int();
                        
                        // 使用VM的静态字段存储功能
                        vm.set_static_field(&class_name, &name_and_type.0, JvmValue::Int(value as u32));
                        jvm_log!("Setting static field {}.{} = {}", class_name, name_and_type.0, value);
                    } else {
                        return Err(JvmError::IllegalStateError(format!("putstatic: 常量池索引{}不是字段引用", index)));
                    }
                }
                0x12 => {
                    // ldc
                    let index = code[self.pc] as usize;
                    self.pc += 1;
                    let cp = &method.constant_pool;
                    match &cp[index - 1] {
                        reader::constant_pool::CpInfo::Integer { bytes, .. } => {
                            let value = *bytes as i32;
                            self.stack.push_int(value);
                        }
                        reader::constant_pool::CpInfo::Float { bytes, .. } => {
                            let value = f32::from_bits(*bytes);
                            self.stack.push_int(value.to_bits() as i32);
                        }
                        reader::constant_pool::CpInfo::String { string_index, .. } => {
                            let s = cp.get_utf8_string(*string_index);
                            jvm_log!("ldc string: {}", s);
                            // 创建字符串对象并推入栈
                            match vm.create_string_object(&s) {
                                Ok(string_ptr) => {
                                    // 将对象引用推入栈
                                    self.stack.push_obj_ref(string_ptr);
                                }
                                Err(e) => {
                                    return Err(JvmError::IllegalStateError(format!("Failed to create string object: {:?}", e)));
                                }
                            }
                        }
                        _ => {
                            return Err(JvmError::IllegalStateError(format!("ldc: 常量池索引{}类型不支持", index)));
                        }
                    }
                }
                0x13 => {
                    // ldc_w
                    let index = ((code[self.pc] as u16) << 8 | code[self.pc + 1] as u16) as usize;
                    self.pc += 2;
                    let cp = &method.constant_pool;
                    match &cp[index - 1] {
                        reader::constant_pool::CpInfo::Integer { bytes, .. } => {
                            let value = *bytes as i32;
                            self.stack.push_int(value);
                        }
                        reader::constant_pool::CpInfo::Float { bytes, .. } => {
                            let value = f32::from_bits(*bytes);
                            self.stack.push_int(value.to_bits() as i32);
                        }
                        reader::constant_pool::CpInfo::String { string_index, .. } => {
                            let s = cp.get_utf8_string(*string_index);
                            jvm_log!("ldc_w string: {}", s);
                            // 创建字符串对象并推入栈
                            match vm.create_string_object(&s) {
                                Ok(string_ptr) => {
                                    // 将对象引用推入栈
                                    self.stack.push_obj_ref(string_ptr);
                                }
                                Err(e) => {
                                    return Err(JvmError::IllegalStateError(format!("Failed to create string object: {:?}", e)));
                                }
                            }
                        }
                        _ => {
                            return Err(JvmError::IllegalStateError(format!("ldc_w: 常量池索引{}类型不支持", index)));
                        }
                    }
                }
                0x14 => {
                    // ldc2_w
                    let index = ((code[self.pc] as u16) << 8 | code[self.pc + 1] as u16) as usize;
                    self.pc += 2;
                    let cp = &method.constant_pool;
                    match &cp[index - 1] {
                        reader::constant_pool::CpInfo::Long { high_bytes, low_bytes, .. } => {
                            let value = ((*high_bytes as u64) << 32) | (*low_bytes as u64);
                            jvm_log!("ldc2_w long: {} (仅低32位入栈)", value);
                            self.stack.push_int((value & 0xFFFF_FFFF) as i32);
                        }
                        reader::constant_pool::CpInfo::Double { high_bytes, low_bytes, .. } => {
                            let bits = ((*high_bytes as u64) << 32) | (*low_bytes as u64);
                            let value = f64::from_bits(bits);
                            jvm_log!("ldc2_w double: {} (推入两个32位值)", value);
                            // 推入高32位
                            self.stack.push_int((*high_bytes) as i32);
                            // 推入低32位
                            self.stack.push_int((*low_bytes) as i32);
                        }
                        _ => {
                            return Err(JvmError::IllegalStateError(format!("ldc2_w: 常量池索引{}不是long/double", index)));
                        }
                    }
                }
                0xb6 => {
                    // invokevirtual
                    let index = ((code[self.pc] as u16) << 8 | code[self.pc + 1] as u16) as usize;
                    self.pc += 2;
                    jvm_log!("invokevirtual {}", index);
                    
                    // 从常量池获取方法引用
                    let cp = &method.constant_pool;
                    if let reader::constant_pool::CpInfo::MethodRef { class_index, name_and_type_index, .. } = &cp[index - 1] {
                        // 获取类名
                        let class_name = if let reader::constant_pool::CpInfo::Class { name_index, .. } = &cp[(*class_index - 1) as usize] {
                            cp.get_utf8_string(*name_index)
                        } else {
                            return Err(JvmError::IllegalStateError("Invalid class reference".to_string()));
                        };
                        
                        // 获取方法名和描述符
                        let name_and_type = if let reader::constant_pool::CpInfo::NameAndType { name_index, descriptor_index, .. } = &cp[(*name_and_type_index - 1) as usize] {
                            let method_name = cp.get_utf8_string(*name_index);
                            let method_desc = cp.get_utf8_string(*descriptor_index);
                            (method_name, method_desc)
                        } else {
                            return Err(JvmError::IllegalStateError("Invalid name and type reference".to_string()));
                        };
                        
                        jvm_log!("Calling virtual method: {}.{}", class_name, name_and_type.0);
                        
                        // 检查是否是PrintStream.println调用
                        if class_name == "java/io/PrintStream" && name_and_type.0 == "println" {
                            // 从操作数栈弹出参数
                            let mut args = Vec::new();
                            
                            // 根据描述符解析参数
                            let descriptor = &name_and_type.1;
                            if descriptor.starts_with("(Ljava/lang/String;)V") {
                                // println(String)
                                // 检查是否有对象引用可以弹出
                                if !self.stack.is_obj_refs_empty() {
                                    let obj_ref = self.stack.pop_obj_ref();
                                    args.push(JvmValue::ObjRef(obj_ref));
                                }
                            } else if descriptor.starts_with("(I)V") {
                                // println(int)
                                let value = self.stack.pop_int();
                                args.push(JvmValue::Int(value as u32));
                            } else if descriptor.starts_with("(D)V") {
                                // println(double)
                                // 弹出两个int值组成double
                                let low = self.stack.pop_int();
                                let high = self.stack.pop_int();
                                let double_bits = ((high as u64) << 32) | (low as u64 & 0xFFFF_FFFF);
                                args.push(JvmValue::Double(double_bits));
                            } else if descriptor.starts_with("(Z)V") {
                                // println(boolean)
                                let value = self.stack.pop_int();
                                args.push(JvmValue::Boolean(value as u8));
                            } else if descriptor.starts_with("(C)V") {
                                // println(char)
                                let value = self.stack.pop_int();
                                args.push(JvmValue::Char(value as u16));
                            } else if descriptor.starts_with("()V") {
                                // println() - 无参数
                            } else {
                                // 其他类型，简化处理
                                // 检查是否有值可以弹出
                                if !self.stack.is_values_empty() {
                                    let value = self.stack.pop_int();
                                    args.push(JvmValue::Int(value as u32));
                                } else if !self.stack.is_obj_refs_empty() {
                                    let obj_ref = self.stack.pop_obj_ref();
                                    args.push(JvmValue::ObjRef(obj_ref));
                                }
                            }
                            
                            // 弹出this引用（PrintStream对象）
                            if !self.stack.is_obj_refs_empty() {
                                let _this_ref = self.stack.pop_obj_ref();
                            }
                            
                            // 调用native方法
                            match vm.call_native_method("java/lang/System", "out.println", args) {
                                Ok(_) => {
                                    jvm_log!("[Native method call successful]");
                                }
                                Err(e) => {
                                    return Err(JvmError::IllegalStateError(format!("Native method call failed: {:?}", e)));
                                }
                            }
                        } else {
                            // 其他虚方法调用
                            jvm_log!("[Virtual method call: {}.{}]", class_name, name_and_type.0);
                        }
                    } else {
                        return Err(JvmError::IllegalStateError(format!("invokevirtual: 常量池索引{}不是方法引用", index)));
                    }
                }
                0xbb => {
                    // new
                    let index = ((code[self.pc] as u16) << 8 | code[self.pc + 1] as u16) as usize;
                    self.pc += 2;
                    jvm_log!("new {}", index);
                    
                    // 从常量池获取类引用
                    let cp = &method.constant_pool;
                    if let reader::constant_pool::CpInfo::Class { name_index, .. } = &cp[index - 1] {
                        let class_name = cp.get_utf8_string(*name_index);
                        jvm_log!("Creating new object of class: {}", class_name);
                        
                        // 1. 加载类（如果还没有加载）
                        let klass = match vm.load(&class_name) {
                            Ok(k) => k,
                            Err(e) => {
                                return Err(JvmError::ClassNotFoundError(format!("Failed to load class {}: {:?}", class_name, e)));
                            }
                        };
                        
                        // 2. 在堆上分配内存
                        let obj_ptr = match vm.alloc_object(&klass) {
                            Ok(ptr) => ptr,
                            Err(e) => {
                                return Err(JvmError::IllegalStateError(format!("Failed to allocate object: {:?}", e)));
                            }
                        };
                        
                        // 3. 初始化对象字段为默认值
                        if let crate::class::Klass::Instance(instance_klass) = &klass {
                            let fields = instance_klass.get_instance_fields();
                            for (i, field) in fields.iter().enumerate() {
                                let default_value = field.get_default();
                                let field_offset = i * 8; // 8字节对齐
                                vm.heap.put_field(obj_ptr, field_offset, default_value);
                            }
                        }
                        
                        // 4. 将对象引用推入操作数栈
                        self.stack.push_obj_ref(obj_ptr);
                        jvm_log!("[Created object: {:?}]", obj_ptr);
                    } else {
                        return Err(JvmError::IllegalStateError(format!("new: 常量池索引{}不是类引用", index)));
                    }
                }
                0x59 => {
                    // dup
                    jvm_log!("dup");
                    // 复制栈顶元素
                    if !self.stack.is_values_empty() {
                        let value = self.stack.peek_int();
                        self.stack.push_int(value);
                    } else if !self.stack.is_obj_refs_empty() {
                        let obj_ref = self.stack.peek_obj_ref();
                        self.stack.push_obj_ref(obj_ref);
                    } else {
                        return Err(JvmError::IllegalStateError("dup: 栈为空".to_string()));
                    }
                }
                0xb7 => {
                    // invokespecial
                    let index = ((code[self.pc] as u16) << 8 | code[self.pc + 1] as u16) as usize;
                    self.pc += 2;
                    jvm_log!("invokespecial {}", index);
                    
                    // 从常量池获取方法引用
                    let cp = &method.constant_pool;
                    if let reader::constant_pool::CpInfo::MethodRef { class_index, name_and_type_index, .. } = &cp[index - 1] {
                        // 获取类名
                        let class_name = if let reader::constant_pool::CpInfo::Class { name_index, .. } = &cp[(*class_index - 1) as usize] {
                            cp.get_utf8_string(*name_index)
                        } else {
                            return Err(JvmError::IllegalStateError("Invalid class reference".to_string()));
                        };
                        
                        // 获取方法名和描述符
                        let name_and_type = if let reader::constant_pool::CpInfo::NameAndType { name_index, descriptor_index, .. } = &cp[(*name_and_type_index - 1) as usize] {
                            let method_name = cp.get_utf8_string(*name_index);
                            let method_desc = cp.get_utf8_string(*descriptor_index);
                            (method_name, method_desc)
                        } else {
                            return Err(JvmError::IllegalStateError("Invalid name and type reference".to_string()));
                        };
                        
                        jvm_log!("Calling special method: {}.{}", class_name, name_and_type.0);
                        
                        // 检查是否是构造函数调用
                        if name_and_type.0 == "<init>" {
                            jvm_log!("[Constructor call: {}.<init>]", class_name);
                            // 弹出this引用（对象实例）
                            if !self.stack.is_obj_refs_empty() {
                                let _this_ref = self.stack.pop_obj_ref();
                                jvm_log!("[Popped this reference for constructor]");
                            }
                        } else {
                            // 其他特殊方法调用
                            jvm_log!("[Special method call: {}.{}]", class_name, name_and_type.0);
                            // 弹出this引用
                            if !self.stack.is_obj_refs_empty() {
                                let _this_ref = self.stack.pop_obj_ref();
                            }
                        }
                    } else {
                        return Err(JvmError::IllegalStateError(format!("invokespecial: 常量池索引{}不是方法引用", index)));
                    }
                }
                0x57 => {
                    if !self.stack.is_values_empty() {
                        self.stack.pop_int();
                    } else if !self.stack.is_obj_refs_empty() {
                        self.stack.pop_obj_ref();
                    }
                }
                0x5f => {
                    self.stack.swap_top_two_ints();
                }
                0xb4 => {
                    // getfield
                    let index = ((code[self.pc] as u16) << 8 | code[self.pc + 1] as u16) as usize;
                    self.pc += 2;
                    jvm_log!("getfield {}", index);
                    // 简化实现：弹出对象引用，推入一个假值
                    if !self.stack.is_obj_refs_empty() {
                        let _obj_ref = self.stack.pop_obj_ref();
                        // 假设字段是int，推入0
                        self.stack.push_int(0);
                    } else {
                        return Err(JvmError::IllegalStateError("getfield: 栈无对象引用".to_string()));
                    }
                }
                0xb5 => {
                    // putfield
                    let index = ((code[self.pc] as u16) << 8 | code[self.pc + 1] as u16) as usize;
                    self.pc += 2;
                    jvm_log!("putfield {}", index);
                    // 简化实现：弹出值和对象引用
                    if !self.stack.is_obj_refs_empty() {
                        let _value = self.stack.pop_int();
                        let _obj_ref = self.stack.pop_obj_ref();
                    } else {
                        return Err(JvmError::IllegalStateError("putfield: 栈无对象引用".to_string()));
                    }
                }
                0xb8 => {
                    // invokestatic
                    let index = ((code[self.pc] as u16) << 8 | code[self.pc + 1] as u16) as usize;
                    self.pc += 2;
                    jvm_log!("invokestatic {}", index);
                    // 简化实现：直接返回
                    // 实际应查找方法并执行
                }
                _ => return Err(JvmError::IllegalStateError(format!("Unknown opcode: 0x{:x}", opcode))),
            }
        }
        Ok(())
    }

    pub fn invoke(
        &mut self,
        _receiver: Option<crate::heap::RawPtr>,
        method: crate::method::Method,
        _class: crate::class::Klass,
        _args: Vec<crate::heap::RawPtr>,
        vm: &mut crate::vm::Vm,
    ) {
        jvm_log!("[JVM] 开始执行方法: {}.{}", method.get_name(), method.get_descriptor());
        let mut heap = crate::heap::Heap::with_maximum_memory(1024);
        match self.execute(&method, &mut heap, vm) {
            Ok(_) => {
                jvm_log!("[JVM] 方法执行完成");
            }
            Err(e) => {
                jvm_log!("[JVM] 方法执行失败: {:?}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_method(code: Vec<u8>, max_stack: usize, max_locals: usize) -> Method {
        Method::new(
            "test".to_string(),
            "()V".to_string(),
            0,
            code,
            max_stack,
            max_locals,
        )
    }

    #[test]
    fn test_iconst_instructions() {
        let mut heap = Heap::with_maximum_memory(1024);
        let mut vm = crate::vm::Vm::new("resources/test");
        let mut thread = JvmThread::new(10, 10);
        
        // 测试 iconst_0 到 iconst_5
        let code = vec![0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let method = create_test_method(code, 10, 10);
        
        thread.execute(&method, &mut heap, &mut vm).unwrap();
        assert_eq!(thread.stack.pop_int(), 5);
        assert_eq!(thread.stack.pop_int(), 4);
        assert_eq!(thread.stack.pop_int(), 3);
        assert_eq!(thread.stack.pop_int(), 2);
        assert_eq!(thread.stack.pop_int(), 1);
        assert_eq!(thread.stack.pop_int(), 0);
    }

    #[test]
    fn test_bipush() {
        let mut heap = Heap::with_maximum_memory(1024);
        let mut vm = crate::vm::Vm::new("resources/test");
        let mut thread = JvmThread::new(10, 10);
        
        let code = vec![0x10, 42]; // bipush 42
        let method = create_test_method(code, 10, 10);
        
        thread.execute(&method, &mut heap, &mut vm).unwrap();
        assert_eq!(thread.stack.pop_int(), 42);
    }

    #[test]
    fn test_arithmetic_instructions() {
        let mut heap = Heap::with_maximum_memory(1024);
        let mut vm = crate::vm::Vm::new("resources/test");
        let mut thread = JvmThread::new(10, 10);
        
        // 测试加法、减法、乘法
        let code = vec![
            0x10, 10,  // bipush 10
            0x10, 5,   // bipush 5
            0x60,      // iadd
            0x10, 3,   // bipush 3
            0x61,      // isub
            0x10, 2,   // bipush 2
            0x62,      // imul
        ];
        let method = create_test_method(code, 10, 10);
        
        thread.execute(&method, &mut heap, &mut vm).unwrap();
        assert_eq!(thread.stack.pop_int(), 24); // ((10 + 5) - 3) * 2
    }

    #[test]
    fn test_division_by_zero() {
        let mut heap = Heap::with_maximum_memory(1024);
        let mut vm = crate::vm::Vm::new("resources/test");
        let mut thread = JvmThread::new(10, 10);
        
        let code = vec![
            0x10, 10,  // bipush 10
            0x10, 0,   // bipush 0
            0x63,      // idiv
        ];
        let method = create_test_method(code, 10, 10);
        
        match thread.execute(&method, &mut heap, &mut vm) {
            Err(JvmError::ArithmeticError(_)) => (),
            _ => panic!("Expected ArithmeticError"),
        }
    }

    #[test]
    fn test_local_variables() {
        let mut heap = Heap::with_maximum_memory(1024);
        let mut vm = crate::vm::Vm::new("resources/test");
        let mut thread = JvmThread::new(10, 10);
        
        let code = vec![
            0x10, 42,  // bipush 42
            0x36, 0,   // istore_0
            0x15, 0,   // iload_0
            0x10, 1,   // bipush 1
            0x60,      // iadd
            0x36, 1,   // istore_1
        ];
        let method = create_test_method(code, 10, 10);
        
        thread.execute(&method, &mut heap, &mut vm).unwrap();
        assert_eq!(thread.local_vars.get_int(0), 42);
        assert_eq!(thread.local_vars.get_int(1), 43);
    }

    #[test]
    fn test_conditional_jump() {
        let mut heap = Heap::with_maximum_memory(1024);
        let mut vm = crate::vm::Vm::new("resources/test");
        let mut thread = JvmThread::new(10, 10);
        
        let code = vec![
            0x10, 0,   // bipush 0
            0x99, 0, 3, // ifeq +3
            0x10, 1,   // bipush 1 (should be skipped)
            0x10, 2,   // bipush 2
        ];
        let method = create_test_method(code, 10, 10);
        
        thread.execute(&method, &mut heap, &mut vm).unwrap();
        assert_eq!(thread.stack.pop_int(), 2);
    }

    #[test]
    fn test_static_field_storage() {
        let mut heap = Heap::with_maximum_memory(1024);
        let mut vm = crate::vm::Vm::new("resources/test");
        let mut thread = JvmThread::new(10, 10);
        
        // 测试静态字段存储
        // 这里我们模拟一个简单的静态字段设置
        vm.set_static_field("TestClass", "staticField", JvmValue::Int(42));
        
        // 验证静态字段值
        let field_value = vm.get_static_field("TestClass", "staticField");
        assert!(field_value.is_some());
        if let Some(JvmValue::Int(value)) = field_value {
            assert_eq!(*value, 42);
        } else {
            panic!("Expected Int value");
        }
    }
}
