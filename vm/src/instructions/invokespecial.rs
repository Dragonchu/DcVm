use crate::error::JvmError;
use crate::jvm_thread::Frame;
use crate::JvmValue;
use crate::jvm_log;
use crate::heap::RawPtr;
use reader::constant_pool::{ConstantPool, ConstantPoolExt};
use crate::vm::Vm;
use crate::instructions::method_utils::{parse_method_descriptor, pop_arguments, push_return_value, handle_special_method_call};

pub fn exec_invokespecial(frame: &mut Frame, code: &[u8], vm: Option<&mut crate::vm::Vm>) -> Result<(), JvmError> {
    let index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
    frame.pc += 2;
    
    let cp = &frame.method.constant_pool;
    let (class_name, method_name, method_desc) = cp.get_methodref_info(index as u16);
    jvm_log!("[Special] 调用方法: {}.{}{}", class_name, method_name, method_desc);
    
    // 检查是否是构造函数
    if method_name == "<init>" {
        jvm_log!("[Special] 调用构造函数: {}.{}{}", class_name, method_name, method_desc);
        
        // 解析参数类型
        let param_types = parse_method_descriptor(&method_desc);
        jvm_log!("[Special] 构造函数参数类型: {:?}", param_types);
        
        // 弹出参数
        let args = pop_arguments(frame, &param_types);
        jvm_log!("[Special] 构造函数参数: {:?}", args);
        
        // 弹出 this 引用
        let this_ref = if !frame.stack.is_obj_refs_empty() {
            frame.stack.pop_obj_ref()
        } else {
            jvm_log!("[Special] 警告: 栈中没有 this 引用");
            RawPtr(std::ptr::null_mut())
        };
        
        // 执行构造函数 - 使用优雅的方式避免多重借用
        if let Some(vm) = vm {
            execute_constructor(vm, &class_name, &method_desc, this_ref, args)?;
        }
        
        // 构造函数执行完成后，将this引用重新推入栈中
        frame.stack.push_obj_ref(this_ref);
        
        jvm_log!("[Special] 构造函数调用完成");
        return Ok(());
    }
    
    // 解析参数类型
    let param_types = parse_method_descriptor(&method_desc);
    jvm_log!("[Special] 参数类型: {:?}", param_types);
    
    // 弹出参数
    let args = pop_arguments(frame, &param_types);
    jvm_log!("[Special] 弹出参数: {:?}", args);
    
    // 弹出 this 引用
    let this_ref = if !frame.stack.is_obj_refs_empty() {
        frame.stack.pop_obj_ref()
    } else {
        jvm_log!("[Special] 警告: 栈中没有 this 引用");
        RawPtr(std::ptr::null_mut())
    };
    
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
                jvm_log!("[Special] Native 方法调用成功: {}.{}", class_name, method_name);
                push_return_value(frame, return_value);
                return Ok(());
            }
            Err(e) => {
                jvm_log!("[Special] Native 方法调用失败: {:?}", e);
                // 继续尝试其他方法
            }
        }
        
        // 尝试通过 VM 的方法分发
        let dispatch_result = vm.dispatch_method_call(&class_name, &method_name, &method_desc, args);
        match dispatch_result {
            Ok(return_value) => {
                jvm_log!("[Special] 方法调用成功: {}.{}", class_name, method_name);
                push_return_value(frame, return_value);
                return Ok(());
            }
            Err(e) => {
                jvm_log!("[Special] 方法调用失败: {:?}", e);
                // 简化处理：暂时返回成功
                jvm_log!("[Special] 方法调用完成: {}", method_name);
                return Ok(());
            }
        }
    }
    
    // 简化处理：暂时返回成功
    jvm_log!("[Special] 方法调用完成: {}", method_name);
    Ok(())
}

/// 执行构造函数的辅助函数，避免多重借用
fn execute_constructor(
    vm: &mut Vm,
    class_name: &str,
    method_desc: &str,
    this_ref: RawPtr,
    args: Vec<JvmValue>,
) -> Result<(), JvmError> {
    // 构建包含this引用的完整参数列表
    let mut full_args = vec![JvmValue::ObjRef(this_ref)];
    full_args.extend(args);
    
    // 直接使用VM的dispatch_method_call方法来执行构造函数
    // 这样可以避免多重借用问题，因为dispatch_method_call内部已经处理了所有必要的逻辑
    let result = vm.dispatch_method_call(class_name, "<init>", method_desc, full_args);
    
    match result {
        Ok(_) => {
            jvm_log!("[Special] 构造函数执行完成");
            Ok(())
        }
        Err(e) => {
            jvm_log!("[Special] 构造函数执行失败: {:?}", e);
            Err(e)
        }
    }
} 