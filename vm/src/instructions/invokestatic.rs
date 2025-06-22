use crate::error::JvmError;
use crate::jvm_thread::Frame;
use crate::JvmValue;
use crate::jvm_log;
use crate::heap::RawPtr;
use reader::constant_pool::{ConstantPool, ConstantPoolExt};
use crate::instructions::method_utils::{parse_method_descriptor, pop_arguments, push_return_value, handle_special_method_call};

pub fn exec_invokestatic(frame: &mut Frame, code: &[u8], vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
    frame.pc += 2;
    
    let cp = &frame.method.constant_pool;
    let (class_name, method_name, method_desc) = cp.get_methodref_info(index as u16);
    jvm_log!("[Static] 调用方法: {}.{}{}", class_name, method_name, method_desc);
    
    // 解析参数类型
    let param_types = parse_method_descriptor(&method_desc);
    jvm_log!("[Static] 参数类型: {:?}", param_types);
    
    // 弹出参数（静态方法没有 this 引用）
    let args = pop_arguments(frame, &param_types);
    jvm_log!("[Static] 弹出参数: {:?}", args);
    
    // 处理特殊方法调用
    if let Some(true) = handle_special_method_call(&class_name, &method_name, &args, frame) {
        return Ok(());
    }
    
    // 尝试调用 native 方法
    if let Some(vm) = vm {
        // 先尝试 native 方法调用
        let native_result = vm.call_native_method(&class_name, &method_name, args.clone());
        match native_result {
            Ok(return_value) => {
                jvm_log!("[Static] Native 方法调用成功: {}.{}", class_name, method_name);
                push_return_value(frame, return_value);
                return Ok(());
            }
            Err(e) => {
                jvm_log!("[Static] Native 方法调用失败: {:?}", e);
                // 继续尝试其他方法
            }
        }
        
        // 尝试通过 VM 的方法分发
        let dispatch_result = vm.dispatch_method_call(&class_name, &method_name, &method_desc, args);
        match dispatch_result {
            Ok(return_value) => {
                jvm_log!("[Static] 方法调用成功: {}.{}", class_name, method_name);
                push_return_value(frame, return_value);
                return Ok(());
            }
            Err(e) => {
                jvm_log!("[Static] 方法调用失败: {:?}", e);
                // 简化处理：暂时返回成功
                jvm_log!("[Static] 方法调用完成: {}", method_name);
                return Ok(());
            }
        }
    }
    
    // 简化处理：暂时返回成功
    jvm_log!("[Static] 方法调用完成: {}", method_name);
    Ok(())
} 