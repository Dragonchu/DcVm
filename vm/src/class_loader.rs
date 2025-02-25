use crate::class::{ArrayKlass, ComponentType, InstanceKlass, Klass};
use reader::class_path_manager::ClassPathManager;
use reader::constant_pool::ConstantPool;
use std::cell::Cell;
use std::{cell::RefCell, collections::HashMap};
use crate::class::Klass::Instance;
use crate::heap::Heap;

pub struct BootstrapClassLoader {
    class_path_manager: ClassPathManager,
    classes: RefCell<HashMap<usize, Klass>>,
    name_id: RefCell<HashMap<String, usize>>,
    nxt_id: Cell<usize>,
}

impl BootstrapClassLoader {
    pub fn new(paths: &str) -> BootstrapClassLoader {
        let mut class_path_manager = ClassPathManager::new();
        class_path_manager.add_class_paths(paths);
        BootstrapClassLoader {
            class_path_manager,
            classes: RefCell::new(HashMap::new()),
            name_id: RefCell::new(HashMap::new()),
            nxt_id: Cell::new(0),
        }
    }

    pub fn load(&self, class_name: &str, heap: &mut Heap) -> Klass {
        if self.name_id.borrow().contains_key(class_name) {
            return self
                .classes
                .borrow()
                .get(self.name_id.borrow().get(class_name).unwrap())
                .unwrap()
                .clone();
        }
        let klass = if class_name.starts_with('[') {
            Klass::Array(self.do_load_array(class_name, heap))
        } else {
            Klass::Instance(self.do_load_instance(class_name, heap))
        };
        let class_id = klass.get_class_id();
        self.name_id
            .borrow_mut()
            .insert(String::from(class_name), class_id);
        self.classes.borrow_mut().insert(class_id, klass.clone());
        self.nxt_id.set(self.nxt_id.get() + 1);
        klass
    }

    fn do_load_array(&self, class_name: &str, heap: &mut Heap) -> ArrayKlass {
        let dimension_size = class_name
            .chars()
            .into_iter()
            .take_while(|&ch| ch == '[')
            .count();
        let element_type = self.load_element_type(&class_name[1..], heap);
        Klass::new_array(dimension_size, element_type, self.nxt_id.get())
    }

    fn load_element_type(&self, element_type: &str, heap: &mut Heap) -> ComponentType {
        match element_type.chars().next().unwrap() {
            '[' => {
                let array_klass = self.do_load_array(element_type, heap);
                ComponentType::Array(Box::new(array_klass))
            }
            'L' => {
                let instance_klass = self.do_load_instance(element_type, heap);
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

    fn do_load_instance(&self, class_name: &str, heap: &mut Heap) -> InstanceKlass {
        if let Some(r_class_name) = class_name.strip_prefix('L') {
            return self.do_load_instance(r_class_name, heap);
        }
        if let Some(r_class_name) = class_name.strip_suffix(';') {
            return self.do_load_instance(r_class_name, heap);
        }
        let class_file = self
            .class_path_manager
            .search_class(class_name)
            .unwrap_or_else(|_| panic!("class {} not found", class_name));
        InstanceKlass::of(&class_file, self.nxt_id.get(), heap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_main_class() {
        let mut cl = BootstrapClassLoader::new("resources/test");
        let mut heap = Heap::with_maximum_memory(1024);
        let klass_ref = cl.load("LMain;", &mut heap);
        println!("{:?}", klass_ref);
    }

    #[test]
    fn load_array_class() {
        let mut cl = BootstrapClassLoader::new("resources/test");
        let mut heap = Heap::with_maximum_memory(1024);
        let klass_ref = cl.load("[[D", &mut heap);
        println!("{:?}", klass_ref);
    }
    
    #[test]
    fn get_main_method() {
        let mut cl = BootstrapClassLoader::new("resources/test");
        let mut heap = Heap::with_maximum_memory(1024);
        let klass_ref = cl.load("LMain;", &mut heap);
        let method = klass_ref.get_method("main", "([Ljava/lang/String;)V");
        println!("{:?}", method);
    }
}
