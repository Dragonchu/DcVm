use vm::heap::RawPtr;
use vm::jvm_thread::JvmThread;
use vm::vm::Vm;
use vm::error::JvmError;
use std::env;

fn main() -> Result<(), JvmError> {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    
    // 检查参数
    if args.len() != 2 {
        println!("用法: {} <测试文件路径>", args[0]);
        println!("示例: {} test/TestProgram", args[0]);
        return Err(JvmError::IllegalStateError("参数错误".to_string()));
    }
    
    let test_path = &args[1];
    let class_name = if test_path.ends_with(".class") {
        // 如果输入的是.class文件，提取类名
        let path = std::path::Path::new(test_path);
        let stem = path.file_stem().unwrap().to_str().unwrap();
        format!("L{};", stem)
    } else if test_path.ends_with(".java") {
        // 如果输入的是.java文件，提取类名
        let path = std::path::Path::new(test_path);
        let stem = path.file_stem().unwrap().to_str().unwrap();
        format!("L{};", stem)
    } else {
        // 如果输入的是类名或路径，直接使用
        if test_path.contains('/') || test_path.contains('\\') {
            // 路径形式，转换为JVM格式
            let path = std::path::Path::new(test_path);
            let stem = path.file_stem().unwrap().to_str().unwrap();
            format!("L{};", stem)
        } else {
            // 类名形式，添加JVM格式
            format!("L{};", test_path)
        }
    };
    
    // 获取类文件所在目录
    let class_dir = if test_path.ends_with(".class") || test_path.ends_with(".java") {
        std::path::Path::new(test_path).parent().unwrap().to_str().unwrap()
    } else {
        // 默认在test目录下查找
        "test"
    };
    
    println!("[JVM] 加载类: {}", class_name);
    println!("[JVM] 类路径: {}", class_dir);
    
    // 初始化JVM
    let mut vm = Vm::new(class_dir);

    // 加载主类
    let main_class = vm.load(&class_name)?;

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