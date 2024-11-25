use std::collections::HashMap;

use reader::{class_file::ClassFile, class_path_manager::{self, ClassPathManager}};
use typed_arena::Arena;

use crate::{class::{InstanceKlassDesc, InstanceKlassRef, Klass}, method_area::MethodArea};


struct BootstrapClassLoader<'a> {
    class_path_manager: ClassPathManager,
    classes: HashMap<String, InstanceKlassRef<'a>>,
    method_area: MethodArea<'a>
}

impl<'a> BootstrapClassLoader<'a> {
    pub fn new(paths: &str) -> BootstrapClassLoader<'_> {
        let mut class_path_manager = ClassPathManager::new();
        class_path_manager.add_class_paths(paths);
        let method_area = MethodArea::new();
        BootstrapClassLoader {
            class_path_manager,
            classes: HashMap::new(),
            method_area
        } 
    }

    pub fn load(&'a mut self, class_name: &str) -> InstanceKlassRef<'a> {
        if self.classes.contains_key(class_name) {
            return self.classes.get(class_name).unwrap();
        }
        let class_file = self.class_path_manager.search_class(class_name).expect("msg");
        let instance_klass_ref = self.method_area.allocate_instance_klass(class_file);
        self.classes.insert(String::from(class_name), instance_klass_ref);
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
        let klass_ref = cl.load("Main");
        println!("{:?}", klass_ref);
    }
}
