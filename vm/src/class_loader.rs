use std::{array, cell::RefCell, collections::HashMap, path::Component};

use reader::{class_file::ClassFile, class_path_manager::{self, ClassPathManager}};
use typed_arena::Arena;

use crate::{class::{self, ArrayKlassRef, ComponentType, InstanceKlassDesc, InstanceKlassRef, Klass}, method_area::MethodArea};


pub struct BootstrapClassLoader<'klass> {
    class_path_manager: ClassPathManager,
    classes: RefCell<HashMap<String, Klass<'klass>>>
}

pub fn calculate_dimension(class_name: &str) -> usize {
    if !class_name.starts_with("[") {
        return 0;
    }
    1 + calculate_dimension(&class_name[1..])
}

impl<'klass> BootstrapClassLoader<'klass> {
    pub fn new(paths: &str) -> BootstrapClassLoader {
        let mut class_path_manager = ClassPathManager::new();
        class_path_manager.add_class_paths(paths);
        BootstrapClassLoader {
            class_path_manager,
            classes: RefCell::new(HashMap::new())
        } 
    }

    pub fn load(&self, class_name: &str, method_area: &'klass MethodArea<'klass,'_>) -> Klass<'klass> {
        if self.classes.borrow().contains_key(class_name) {
            return self.classes.borrow().get(class_name).unwrap().clone();
        }
        let klass = self.do_load(class_name, method_area);
        self.classes.borrow_mut().insert(String::from(class_name), klass);
        klass
    }

    pub fn load_instance_klass(&self, class_name: &str, method_area: &'klass MethodArea<'klass,'_>) -> InstanceKlassRef<'_, 'klass, '_> {
        if self.classes.borrow().contains_key(class_name) {
            let klass = self.classes.borrow().get(class_name).unwrap().clone();
            if let Klass::Instance(instance_klass) = klass {
                return instance_klass;
            }
        }
        self.do_load_instance(class_name, method_area)
    }

    pub fn load_array_klass(&self, class_name: &str, method_area: &'klass MethodArea<'klass,'_>) -> ArrayKlassRef<'klass> {
        if self.classes.borrow().contains_key(class_name) {
            let klass = self.classes.borrow().get(class_name).unwrap().clone();
            if let Klass::Array(array_klass) = klass {
                return array_klass;
            }
        } 
        self.do_load_array(class_name, method_area)
    }

    fn do_load(&self, class_name: &str, method_area: &'klass MethodArea<'klass,'_>) -> Klass<'klass> {
        if class_name.starts_with("[") {
           return Klass::Array(self.do_load_array(class_name, method_area));
        }
        Klass::Instance(self.do_load_instance(class_name, method_area))
    }

    pub fn do_load_instance(&self, class_name: &str, method_area: &'klass MethodArea<'klass,'_>) -> InstanceKlassRef<'klass> {
        let class_file = self.class_path_manager.search_class(class_name).expect("msg");
        let instance_klass_ref = method_area.allocate_instance_klass(class_file);
        instance_klass_ref.link();
        instance_klass_ref 
    }

    pub fn do_load_array(&self, class_name: &str, method_area: &'klass MethodArea<'klass,'_>) -> ArrayKlassRef<'klass>{
        let dimension = calculate_dimension(class_name);
        if dimension == 1 {
            method_area.allocate_array_klass(1, self.do_load_component_type(&class_name[1..], method_area))
        }else {
            method_area.allocate_array_klass(dimension, ComponentType::Array(self.do_load_array(&class_name[1..], method_area)))
        }
    }

    fn do_load_component_type(&self, class_name: &str, method_area: &'klass MethodArea<'klass,'_>) -> ComponentType<'klass> {
        if class_name.starts_with("L") {
            let instance_klass = self.do_load_instance(&class_name[1..], method_area);
            ComponentType::Object(instance_klass)
        }else {
            let primitive_type = class_name.chars().next().expect("No more chars");
            match primitive_type {
                'B' => ComponentType::Byte,
                'Z' => ComponentType::Boolean,
                'S' => ComponentType::Short,
                'C' => ComponentType::Char,
                'I' => ComponentType::Int,
                'J' => ComponentType::Long,
                'F' => ComponentType::Float,
                'D' => ComponentType::Double,
                'V' => ComponentType::Void,
                _ => panic!("Unknown primitive type")
            }
        }
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

    #[test]
    fn link_main_class() {
        let mut cl = BootstrapClassLoader::new("resources/test");
        let method_area = MethodArea::new();
        let klass_ref = cl.load("Main", &method_area);
        println!("{:?}", klass_ref)
    }

    #[test]
    fn load_object_array() {
        let mut cl = BootstrapClassLoader::new("resources/test");
        let method_area = MethodArea::new();
        let klass_ref = cl.load("[[LMain", &method_area);
        println!("{:?}", klass_ref)
    }

    #[test]
    fn load_primitive_array() {
        let mut cl = BootstrapClassLoader::new("resources/test");
        let method_area = MethodArea::new();
        let klass_ref = cl.load("[[B", &method_area);
        println!("{:?}", klass_ref) 
    }
}
