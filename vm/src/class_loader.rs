use std::{cell::RefCell, collections::HashMap};

use crate::class::Klass;
use reader::class_path_manager::ClassPathManager;
use reader::constant_pool::ConstantPool;

pub struct BootstrapClassLoader {
    class_path_manager: ClassPathManager,
    classes: RefCell<HashMap<String, Klass>>,
}

impl BootstrapClassLoader {
    pub fn new(paths: &str) -> BootstrapClassLoader {
        let mut class_path_manager = ClassPathManager::new();
        class_path_manager.add_class_paths(paths);
        BootstrapClassLoader {
            class_path_manager,
            classes: RefCell::new(HashMap::new()),
        }
    }

    pub fn load(&self, class_name: &str) -> Klass {
        if self.classes.borrow().contains_key(class_name) {
            return self.classes.borrow().get(class_name).unwrap().clone();
        }
        let klass = self.do_load(class_name).clone();
        self.classes
            .borrow_mut()
            .insert(String::from(class_name), klass.clone());
        klass
    }

    fn do_load(&self, class_name: &str) -> Klass {
        let class_file = self
            .class_path_manager
            .search_class(class_name)
            .expect("class not found");
        Klass::new(&class_file)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_main_class() {
        let mut cl = BootstrapClassLoader::new("resources/test");
        let klass_ref = cl.load("Main");
        println!("{:?}", klass_ref);
    }

}
