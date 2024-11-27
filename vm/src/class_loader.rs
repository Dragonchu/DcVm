use std::{cell::RefCell, collections::HashMap};

use reader::{class_file::ClassFile, class_path_manager::{self, ClassPathManager}};
use typed_arena::Arena;

use crate::{class::{InstanceKlassDesc, InstanceKlassRef, Klass}, method_area::MethodArea};


pub struct BootstrapClassLoader<'a> {
    class_path_manager: ClassPathManager,
    classes: RefCell<HashMap<String, InstanceKlassRef<'a>>>
}

impl<'a> BootstrapClassLoader<'a> {
    pub fn new(paths: &str) -> BootstrapClassLoader<'_> {
        let mut class_path_manager = ClassPathManager::new();
        class_path_manager.add_class_paths(paths);
        BootstrapClassLoader {
            class_path_manager,
            classes: RefCell::new(HashMap::new())
        } 
    }

    pub fn load(&'a self, class_name: &str, method_area: &'a MethodArea<'a>) -> InstanceKlassRef<'a> {
        if self.classes.borrow().contains_key(class_name) {
            return self.classes.borrow().get(class_name).unwrap();
        }
        let class_file = self.class_path_manager.search_class(class_name).expect("msg");
        let instance_klass_ref = method_area.allocate_instance_klass(class_file);
        self.classes.borrow_mut().insert(String::from(class_name), instance_klass_ref);
        instance_klass_ref 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn parse_main_class() {
        let mut cl = BootstrapClassLoader::new("resources/test");
        let method_area = MethodArea::new();
        let klass_ref = cl.load("Main", &method_area);
        println!("{:?}", klass_ref);
    }
}
