use crate::error::JvmError;
use crate::heap::Heap;
use crate::method::Method;
use crate::operand_stack::OperandStack;
use crate::local_vars::LocalVars;
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

    pub fn execute(&mut self, method: &Method, heap: &mut Heap) -> Result<(), JvmError> {
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
                    let index = code[self.pc] as usize;
                    self.pc += 1;
                    let value = self.local_vars.get_int(index);
                    self.stack.push_int(value);
                }
                0x1b => {
                    let index = code[self.pc] as usize;
                    self.pc += 1;
                    let value = self.local_vars.get_int(index);
                    self.stack.push_int(value);
                }
                0x36 => {
                    let index = code[self.pc] as usize;
                    self.pc += 1;
                    let value = self.stack.pop_int();
                    self.local_vars.set_int(index, value);
                }
                0x3c => {
                    let index = code[self.pc] as usize;
                    self.pc += 1;
                    let value = self.stack.pop_int();
                    self.local_vars.set_int(index, value);
                }
                0x3d => {
                    let index = code[self.pc] as usize;
                    self.pc += 1;
                    let value = self.stack.pop_int();
                    self.local_vars.set_int(index, value);
                }
                0x60 => {
                    let v2 = self.stack.pop_int();
                    let v1 = self.stack.pop_int();
                    self.stack.push_int(v1 + v2);
                }
                0x61 => {
                    let v2 = self.stack.pop_int();
                    let v1 = self.stack.pop_int();
                    self.stack.push_int(v1 - v2);
                }
                0x62 => {
                    let v2 = self.stack.pop_int();
                    let v1 = self.stack.pop_int();
                    self.stack.push_int(v1 * v2);
                }
                0x63 => {
                    let v2 = self.stack.pop_int();
                    let v1 = self.stack.pop_int();
                    if v2 == 0 {
                        return Err(JvmError::ArithmeticError("Division by zero".to_string()));
                    }
                    self.stack.push_int(v1 / v2);
                }
                0x64 => {
                    let v2 = self.stack.pop_int();
                    let v1 = self.stack.pop_int();
                    if v2 == 0 {
                        return Err(JvmError::ArithmeticError("Division by zero".to_string()));
                    }
                    self.stack.push_int(v1 % v2);
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
                0xb1 => return Ok(()), // return
                0xb2 => {
                    // invokestatic
                    let index = ((code[self.pc] as u16) << 8 | code[self.pc + 1] as u16) as usize;
                    self.pc += 2;
                    println!("invokestatic {}", index);
                }
                0xb3 => {
                    // putstatic
                    let index = ((code[self.pc] as u16) << 8 | code[self.pc + 1] as u16) as usize;
                    self.pc += 2;
                    println!("putstatic {}", index);
                    // TODO: 实现静态字段设置逻辑
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
                            println!("ldc string: {}", s);
                            // TODO: 支持字符串对象入栈
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
                            println!("ldc_w string: {}", s);
                            // TODO: 支持字符串对象入栈
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
                            println!("ldc2_w long: {} (仅低32位入栈)", value);
                            self.stack.push_int((value & 0xFFFF_FFFF) as i32);
                        }
                        reader::constant_pool::CpInfo::Double { high_bytes, low_bytes, .. } => {
                            let bits = ((*high_bytes as u64) << 32) | (*low_bytes as u64);
                            let value = f64::from_bits(bits);
                            println!("ldc2_w double: {} (仅低32位入栈)", value);
                            self.stack.push_int((bits & 0xFFFF_FFFF) as i32);
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
                    println!("invokevirtual {}", index);
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
        let mut heap = crate::heap::Heap::with_maximum_memory(1024);
        let _ = self.execute(&method, &mut heap);
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
        let mut thread = JvmThread::new(10, 10);
        
        // 测试 iconst_0 到 iconst_5
        let code = vec![0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let method = create_test_method(code, 10, 10);
        
        thread.execute(&method, &mut heap).unwrap();
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
        let mut thread = JvmThread::new(10, 10);
        
        let code = vec![0x10, 42]; // bipush 42
        let method = create_test_method(code, 10, 10);
        
        thread.execute(&method, &mut heap).unwrap();
        assert_eq!(thread.stack.pop_int(), 42);
    }

    #[test]
    fn test_arithmetic_instructions() {
        let mut heap = Heap::with_maximum_memory(1024);
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
        
        thread.execute(&method, &mut heap).unwrap();
        assert_eq!(thread.stack.pop_int(), 24); // ((10 + 5) - 3) * 2
    }

    #[test]
    fn test_division_by_zero() {
        let mut heap = Heap::with_maximum_memory(1024);
        let mut thread = JvmThread::new(10, 10);
        
        let code = vec![
            0x10, 10,  // bipush 10
            0x10, 0,   // bipush 0
            0x63,      // idiv
        ];
        let method = create_test_method(code, 10, 10);
        
        match thread.execute(&method, &mut heap) {
            Err(JvmError::ArithmeticError(_)) => (),
            _ => panic!("Expected ArithmeticError"),
        }
    }

    #[test]
    fn test_local_variables() {
        let mut heap = Heap::with_maximum_memory(1024);
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
        
        thread.execute(&method, &mut heap).unwrap();
        assert_eq!(thread.local_vars.get_int(0), 42);
        assert_eq!(thread.local_vars.get_int(1), 43);
    }

    #[test]
    fn test_conditional_jump() {
        let mut heap = Heap::with_maximum_memory(1024);
        let mut thread = JvmThread::new(10, 10);
        
        let code = vec![
            0x10, 0,   // bipush 0
            0x99, 0, 3, // ifeq +3
            0x10, 1,   // bipush 1 (should be skipped)
            0x10, 2,   // bipush 2
        ];
        let method = create_test_method(code, 10, 10);
        
        thread.execute(&method, &mut heap).unwrap();
        assert_eq!(thread.stack.pop_int(), 2);
    }
}
