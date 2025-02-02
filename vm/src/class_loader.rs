use std::{cell::RefCell, collections::HashMap};

use crate::class::{ArrayKlass, ComponentType, InstanceKlass, Klass};
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
        let klass = if class_name.starts_with('[') {
            Klass::Array(self.do_load_array(class_name))
        } else {
            Klass::Instance(self.do_load_instance(class_name))
        };
        self.classes
            .borrow_mut()
            .insert(String::from(class_name), klass.clone());
        klass
    }

    fn do_load_array(&self, class_name: &str) -> ArrayKlass {
        let dimension_size = class_name
            .chars()
            .into_iter()
            .take_while(|&ch| ch == '[')
            .count();
        let element_type = self.load_element_type(&class_name[1..]);
        Klass::new_array(dimension_size, element_type)
    }

    fn load_element_type(&self, element_type: &str) -> ComponentType {
        match element_type.chars().next().unwrap() {
            '[' => {
                let array_klass = self.do_load_array(element_type);
                ComponentType::Array(Box::new(array_klass))
            }
            'L' => {
                let instance_klass = self.do_load_instance(element_type);
                ComponentType::Object(Box::new(instance_klass))
            }
            'B' => ComponentType::Byte,
            'Z' => ComponentType::Boolean,
            'S' => ComponentType::Short,
            'C' => ComponentType::Char,
            'I' => ComponentType::Int,
            'J' => ComponentType::Long,
            'F' => ComponentType::Float,
            'D' => ComponentType::Double,
            'V' => ComponentType::Void,
            _ => panic!("Unknown element type {}", element_type),
        }
    }

    fn do_load_instance(&self, class_name: &str) -> InstanceKlass {
        if let Some(r_class_name) = class_name.strip_prefix('L') {
           return self.do_load_instance(r_class_name);
        }
        let class_file = self
            .class_path_manager
            .search_class(class_name)
            .unwrap_or_else(|_| panic!("class {} not found", class_name));
        Klass::new_instance(&class_file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_main_class() {
        let mut cl = BootstrapClassLoader::new("resources/test");
        let klass_ref = cl.load("LMain");
        println!("{:?}", klass_ref);
    }
    
    #[test]
    fn load_array_class() {
        let mut cl = BootstrapClassLoader::new("resources/test");
        let klass_ref = cl.load("[[D");
        println!("{:?}", klass_ref);
    }
}
