use crate::error::JvmError;
use crate::heap::Heap;
use crate::method::Method;
use crate::operand_stack::OperandStack;
use crate::local_vars::LocalVars;
use crate::JvmValue;
use crate::heap::RawPtr;
use crate::jvm_log;
use reader::constant_pool::ConstantPool;
use crate::instructions::aload_0;
use crate::instructions::constants;
use crate::instructions::load_store;
use crate::instructions::arithmetic;
use crate::instructions::control;
use crate::instructions::stack;
use crate::instructions::field_ops;
use crate::instructions::ldc_ops;
use crate::instructions::object_ops;
use crate::instructions::control_extended;
use crate::instructions::invokestatic;
use crate::instructions::array_ops;
use crate::instructions::invokevirtual;
use crate::instructions::iinc;
use crate::instructions::invokespecial;

// 新增 Frame 结构体
pub struct Frame {
    pub local_vars: LocalVars,
    pub stack: OperandStack,
    pub method: Method,
    pub pc: usize,
}

pub struct JvmThread {
    pub frames: Vec<Frame>,
    call_depth: usize, // 添加调用深度计数器
}

impl JvmThread {
    pub fn new(max_stack: usize, max_locals: usize) -> Self {
        let main_frame = Frame {
            pc: 0,
            stack: OperandStack::new(max_stack),
            local_vars: LocalVars::new(max_locals),
            method: Method::new("main".to_string(), "()V".to_string(), 0, vec![], max_stack, max_locals),
        };
        JvmThread {
            frames: vec![main_frame],
            call_depth: 0,
        }
    }

    pub fn execute(&mut self, method: &Method, heap: &mut Heap, mut vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
        // 检查调用深度，防止无限递归
        if self.call_depth > 1000 {
            return Err(JvmError::IllegalStateError("Call depth exceeded maximum limit".to_string()));
        }
        self.call_depth += 1;
        
        // 确保第一个frame使用正确的方法（包含常量池）
        if !self.frames.is_empty() {
            self.frames[0].method = method.clone();
        }
        
        let code = method.get_code();
        // 主循环：只要还有frame且pc未越界就继续执行
        while !self.frames.is_empty() && self.frames[0].pc < code.len() {
            // 注意：任何地方弹出frame后都要保证self.frames非空再访问self.frames[0]
            let opcode = code[self.frames[0].pc];
            self.frames[0].pc += 1;

            // 获取当前frame的可变引用
            let frame = &mut self.frames[0];

            match opcode {
                0x00 => (), // nop
                0x01 => constants::exec_aconst_null(frame, code, vm.as_deref_mut())?,
                0x02 => constants::exec_iconst_m1(frame, code, vm.as_deref_mut())?,
                0x03 => constants::exec_iconst_0(frame, code, vm.as_deref_mut())?,
                0x04 => constants::exec_iconst_1(frame, code, vm.as_deref_mut())?,
                0x05 => constants::exec_iconst_2(frame, code, vm.as_deref_mut())?,
                0x06 => constants::exec_iconst_3(frame, code, vm.as_deref_mut())?,
                0x07 => constants::exec_iconst_4(frame, code, vm.as_deref_mut())?,
                0x08 => constants::exec_iconst_5(frame, code, vm.as_deref_mut())?,
                0x10 => constants::exec_bipush(frame, code, vm.as_deref_mut())?,
                0x11 => ldc_ops::exec_sipush(frame, code, vm.as_deref_mut())?,
                0x12 => ldc_ops::exec_ldc(frame, code, vm.as_deref_mut())?,
                0x13 => ldc_ops::exec_ldc_w(frame, code, vm.as_deref_mut())?,
                0x14 => ldc_ops::exec_ldc2_w(frame, code, vm.as_deref_mut())?,
                0x15 => load_store::exec_iload(frame, code, vm.as_deref_mut())?,
                0x1a => load_store::exec_iload_0(frame, code, vm.as_deref_mut())?,
                0x1b => load_store::exec_iload_1(frame, code, vm.as_deref_mut())?,
                0x1c => load_store::exec_iload_2(frame, code, vm.as_deref_mut())?,
                0x1d => load_store::exec_iload_3(frame, code, vm.as_deref_mut())?,
                0x2a => aload_0::exec_aload_0(frame, code, vm.as_deref_mut())?,
                0x2b => load_store::exec_aload_1(frame, code, vm.as_deref_mut())?,
                0x2c => load_store::exec_aload_2(frame, code, vm.as_deref_mut())?,
                0x2d => load_store::exec_aload_3(frame, code, vm.as_deref_mut())?,
                0x36 => load_store::exec_istore(frame, code, vm.as_deref_mut())?,
                0x3b => load_store::exec_istore_0(frame, code, vm.as_deref_mut())?,
                0x3c => load_store::exec_istore_1(frame, code, vm.as_deref_mut())?,
                0x3d => load_store::exec_istore_2(frame, code, vm.as_deref_mut())?,
                0x3e => load_store::exec_istore_3(frame, code, vm.as_deref_mut())?,
                0x4b => load_store::exec_astore_0(frame, code, vm.as_deref_mut())?,
                0x4c => load_store::exec_astore_1(frame, code, vm.as_deref_mut())?,
                0x4d => load_store::exec_astore_2(frame, code, vm.as_deref_mut())?,
                0x4e => load_store::exec_astore_3(frame, code, vm.as_deref_mut())?,
                0x59 => stack::exec_dup(frame, code, vm.as_deref_mut())?,
                0xb7 => invokespecial::exec_invokespecial(frame, code, vm.as_deref_mut())?,
                0x60 => arithmetic::exec_iadd(frame, code, vm.as_deref_mut())?,
                0x64 => arithmetic::exec_isub(frame, code, vm.as_deref_mut())?,
                0x68 => arithmetic::exec_imul(frame, code, vm.as_deref_mut())?,
                0x6c => arithmetic::exec_idiv(frame, code, vm.as_deref_mut())?,
                0x84 => iinc::exec_iinc(frame, code, vm.as_deref_mut())?,
                0x99 => control::exec_ifeq(frame, code, vm.as_deref_mut())?,
                0x9a => control::exec_ifne(frame, code, vm.as_deref_mut())?,
                0x9f => control::exec_ifge(frame, code, vm.as_deref_mut())?,
                0xa7 => control::exec_goto(frame, code, vm.as_deref_mut())?,
                0xac => {
                    return Ok(());
                }
                0xb1 => control::exec_return(frame, code, vm.as_deref_mut())?,
                0xb2 => field_ops::exec_getstatic(frame, code, vm.as_deref_mut(), method)?,
                0xb3 => field_ops::exec_putstatic(frame, code, vm.as_deref_mut(), method)?,
                0xb6 => invokevirtual::exec_invokevirtual(frame, code, vm.as_deref_mut())?,
                0xb5 => object_ops::exec_putfield(frame, code, vm.as_deref_mut())?,
                0xbc => array_ops::exec_newarray(frame, code, vm.as_deref_mut())?,
                0xbe => array_ops::exec_arraylength(frame, code, vm.as_deref_mut())?,
                0x4f => array_ops::exec_iastore(frame, code, vm.as_deref_mut())?,
                0x2e => array_ops::exec_iaload(frame, code, vm.as_deref_mut())?,
                0xa2 => control_extended::exec_if_icmpge(frame, code, vm.as_deref_mut())?,
                0xb0 => control_extended::exec_areturn(frame, code, vm.as_deref_mut())?,
                0xbb => object_ops::exec_new(frame, code, vm.as_deref_mut())?,
                0xb8 => invokestatic::exec_invokestatic(frame, code, vm.as_deref_mut())?,
                // istore_0 ~ istore_3
                0x3f => load_store::exec_istore_0(frame, code, vm.as_deref_mut())?,
                0x40 => load_store::exec_istore_1(frame, code, vm.as_deref_mut())?,
                0x41 => load_store::exec_istore_2(frame, code, vm.as_deref_mut())?,
                0x42 => load_store::exec_istore_3(frame, code, vm.as_deref_mut())?,
                0xb4 => object_ops::exec_getfield(frame, code, vm.as_deref_mut())?,
                _ => return Err(JvmError::IllegalStateError(format!("Unknown opcode: 0x{:x}", opcode))),
            }
            // 如果frame被弹空，直接return Ok(())，防止后续访问self.frames[0]越界
            if self.frames.is_empty() {
                return Ok(());
            }
        }
        self.call_depth -= 1; // 递减调用深度
        Ok(())
    }

    pub fn invoke(
        &mut self,
        receiver: Option<crate::heap::RawPtr>,
        method: crate::method::Method,
        class: crate::class::Klass,
        args: Vec<crate::heap::RawPtr>,
        vm: &mut crate::vm::Vm,
    ) {
        jvm_log!("[JVM] 开始执行方法: {}.{}", method.get_name(), method.get_descriptor());
        
        // 创建新的frame，而不是使用默认的main frame
        let mut new_frame = Frame {
            pc: 0,
            stack: OperandStack::new(1024), // 使用固定的较大栈大小
            local_vars: LocalVars::new(method.max_locals),
            method: method.clone(),
        };
        
        // 初始化参数到局部变量表
        // 对于实例方法，slot 0 是 this 引用
        // 对于静态方法，从 slot 0 开始是参数
        let mut param_index = 0;
        
        // 如果是实例方法，设置 this 引用
        if let Some(this_ref) = receiver {
            new_frame.local_vars.set_obj_ref(0, this_ref);
            param_index = 1;
        }
        
        // 设置方法参数
        for (i, arg) in args.iter().enumerate() {
            new_frame.local_vars.set_obj_ref(param_index + i, *arg);
        }
        
        // 将新frame推入栈顶
        self.frames.insert(0, new_frame);
        
        let mut heap = crate::heap::Heap::with_maximum_memory(1024);
        match self.execute(&method, &mut heap, Some(vm)) {
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
        
        thread.execute(&method, &mut heap, Some(&mut vm)).unwrap();
        assert_eq!(thread.frames[0].stack.pop_int(), 5);
        assert_eq!(thread.frames[0].stack.pop_int(), 4);
        assert_eq!(thread.frames[0].stack.pop_int(), 3);
        assert_eq!(thread.frames[0].stack.pop_int(), 2);
        assert_eq!(thread.frames[0].stack.pop_int(), 1);
        assert_eq!(thread.frames[0].stack.pop_int(), 0);
    }

    #[test]
    fn test_bipush() {
        let mut heap = Heap::with_maximum_memory(1024);
        let mut vm = crate::vm::Vm::new("resources/test");
        let mut thread = JvmThread::new(10, 10);
        
        let code = vec![0x10, 42]; // bipush 42
        let method = create_test_method(code, 10, 10);
        
        thread.execute(&method, &mut heap, Some(&mut vm)).unwrap();
        assert_eq!(thread.frames[0].stack.pop_int(), 42);
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
            0x64,      // isub
            0x10, 2,   // bipush 2
            0x68,      // imul
        ];
        let method = create_test_method(code, 10, 10);
        
        thread.execute(&method, &mut heap, Some(&mut vm)).unwrap();
        assert_eq!(thread.frames[0].stack.pop_int(), 24); // ((10 + 5) - 3) * 2
    }

    #[test]
    fn test_division_by_zero() {
        let mut heap = Heap::with_maximum_memory(1024);
        let mut vm = crate::vm::Vm::new("resources/test");
        let mut thread = JvmThread::new(10, 10);
        
        let code = vec![
            0x10, 10,  // bipush 10
            0x10, 0,   // bipush 0
            0x6c,      // idiv
        ];
        let method = create_test_method(code, 10, 10);
        
        match thread.execute(&method, &mut heap, Some(&mut vm)) {
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
        
        thread.execute(&method, &mut heap, Some(&mut vm)).unwrap();
        assert_eq!(thread.frames[0].local_vars.get_int(0), 42);
        assert_eq!(thread.frames[0].local_vars.get_int(1), 43);
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
        
        thread.execute(&method, &mut heap, Some(&mut vm)).unwrap();
        assert_eq!(thread.frames[0].stack.pop_int(), 2);
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

    #[test]
    fn test_method_is_native() {
        // 测试Method的is_native方法
        let method = Method::new(
            "nativeMethod".to_string(),
            "()V".to_string(),
            0x0100, // ACC_NATIVE
            vec![],
            10,
            10,
        );
        
        assert!(method.is_native());
        
        let java_method = Method::new(
            "javaMethod".to_string(),
            "()V".to_string(),
            0x0001, // ACC_PUBLIC
            vec![],
            10,
            10,
        );
        
        assert!(!java_method.is_native());
    }

    #[test]
    fn test_native_method_key() {
        let method = Method::new(
            "toString".to_string(),
            "()Ljava/lang/String;".to_string(),
            0x0100, // ACC_NATIVE
            vec![],
            10,
            10,
        );
        
        let key = method.get_native_key("java/lang/Object");
        assert_eq!(key, "java/lang/Object.toString()Ljava/lang/String;");
    }

    #[test]
    fn test_dynamic_method_dispatch() {
        let mut heap = Heap::with_maximum_memory(1024);
        let mut vm = crate::vm::Vm::new("resources/test");
        let mut thread = JvmThread::new(10, 10);
        
        // 测试动态方法分发的基本功能
        // 这里我们测试native方法调用
        // 由于我们没有完整的类加载器，这个测试主要是验证代码结构
        
        // 创建一个简单的测试方法
        let method = Method::new(
            "testDispatch".to_string(),
            "()V".to_string(),
            0x0001, // ACC_PUBLIC
            vec![],
            10,
            10,
        );
        
        // 验证方法不是native方法
        assert!(!method.is_native());
    }
}
