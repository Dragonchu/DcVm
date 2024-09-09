use crate::classpath::system::SystemDictionary;
use crate::oop::klass::Klass;
use std::sync::Arc;

pub enum ClassLoader {
    BootstrapClassLoader,
    ExtensionClassLoader,
    ApplicationClassLoader,
}

pub trait BaseClassLoader {
    fn load_class(&self, class_name: &str) -> Option<Arc<Klass>>;
}

pub struct BootstrapClassLoader {}

impl BootstrapClassLoader {
    pub fn new() -> Self {
        BootstrapClassLoader {}
    }
}

impl BootstrapClassLoader {
    fn do_load_class(&self, class_name: &str) -> Option<Arc<Klass>> {
        if class_name.starts_with('[') {
            let mut dimension = 0;
            let class_name_chars = class_name.chars().collect::<Vec<char>>(); 
            while class_name_chars.get(dimension + 1) == Some(&'[') {
                dimension += 1;
            }
            if dimension == 1 {
                let component = &class_name[2..class_name.len() - 1];
                let component_klass = self.load_class(component);
                if let Some(klass) = component_klass {
                    //todo
                }
            }
        } else {
            //todo
        }
        None
    }
}

impl BaseClassLoader for BootstrapClassLoader {
    fn load_class(&self, class_name: &str) -> Option<Arc<Klass>> {
        //todo 加锁
        if let Some(klass) = SystemDictionary::get().find(class_name) {
            return Some(klass);
        }
        None
    }
}
