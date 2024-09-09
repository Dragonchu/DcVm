use crate::frame::Frame;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use lazy_static::lazy_static;
use crate::classpath::class_loader::BootstrapClassLoader;
use crate::native::java_lang_class::java::lang::Class;

lazy_static! {
    static ref MIRRORS: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(vec![]));
}

pub struct JavaMainThread {
    _frames: Vec<Frame>,
    _mirrors: Vec<String>,
    _main_class_name: String,
    _arguments: Vec<String>,
}

impl JavaMainThread {
    pub fn new(main_class_name: String, arguments: Vec<String>) -> Self {
        JavaMainThread {
            _frames: vec![],
            _mirrors: vec![],
            _main_class_name: main_class_name,
            _arguments: arguments,
        }
    }
    pub fn start(mut self) {
        let handler = thread::Builder::new()
            .name("JavaMainThread".to_string())
            .spawn(move || {
                self.run();
                self.on_destroy();
            })
            .expect("Fail to start JavaMainThread");
        handler.join().expect("Fail to run JavaMainThread");
    }

    fn run(&mut self) {
        let cl = BootstrapClassLoader::new();
        let class = Class::new();
         
        
    }

    fn on_destroy(&self) {}
}

pub struct Threads {}
