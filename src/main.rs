use crate::java_thread::JavaMainThread;

mod java_thread;
mod frame;
mod stack;
mod method;
mod types;
mod oop;
mod native;
mod classpath;
mod classfile;

fn main() {
    let mut app_threads: Vec<&JavaMainThread> = Vec::new();
    
    let main_thread = JavaMainThread::new("main".to_string(), vec![]);
    app_threads.push(&main_thread);
    main_thread.start();
}
