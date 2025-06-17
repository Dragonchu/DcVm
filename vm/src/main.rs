use vm::heap::RawPtr;
use vm::jvm_thread::JvmThread;
use vm::vm::Vm;
use vm::error::JvmError;

fn main() -> Result<(), JvmError> {
    // 初始化JVM
    let mut vm = Vm::new("resources/test");

    // 加载主类
    let main_class = vm.load("TestProgram")?;

    // 获取main方法
    let main_method = main_class
        .get_method("main", "([Ljava/lang/String;)V")
        .ok_or_else(|| JvmError::ClassNotFoundError("main method not found".to_string()))?;

    // 创建主线程
    let mut java_main_thread = JvmThread::new(1024, 1024);

    // 准备参数
    let args: Vec<RawPtr> = vec![];

    // 调用main方法
    java_main_thread.invoke(None, main_method.clone(), main_class, args, &mut vm);

    Ok(())
}