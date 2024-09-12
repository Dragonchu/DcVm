use std::sync::Arc;

use crate::oop::{
    klass::{Klass, ObjArrayKlass, TypeArrayKlass},
    reflection,
};

use super::class_path_manager::ClassPathManager;

pub trait ClassLoader {
    fn load_class(&self, class_name: &str) -> Option<Klass>;
}

pub struct BaseClassLoader {
    pub class_path_manager: ClassPathManager,
}

struct BootStrapClassLoader {
    base: BaseClassLoader,
}

impl ClassLoader for BaseClassLoader {
    fn load_class(&self, class_name: &str) -> Option<Klass> {
        if class_name.starts_with('[') {
            let mut dimension = 0;
            while class_name.chars().nth(dimension).unwrap() == '[' {
                dimension += 1;
            }
            if dimension == 1 {
                if class_name.chars().nth(1).unwrap() == 'L' {
                    let component = class_name[1..].to_string();
                    let component_class = self.clone().load_class(&component);
                    if let Some(Klass::InstanceKlass(component_type)) = component_class {
                        let object_array_klass =
                            Klass::ObjArrayKlass(Arc::new(ObjArrayKlass::one_dimension(
                                self.clone(),
                                dimension,
                                component_type.clone(),
                            )));
                        return Some(object_array_klass);
                    }
                    return None;
                }
                let component_type = reflection::primitive_type_to_value_type_no_wrap(
                    class_name.chars().nth(1).unwrap(),
                );
                let type_array_klass = Klass::TypeArrayKlass(Arc::new(
                    TypeArrayKlass::one_dimension(self.clone(), dimension, component_type),
                ));
                return Some(type_array_klass);
            }
            let down_type_name = class_name[1..].to_string();
            let down_type = self.clone().load_class(&down_type_name);
            match down_type {
                Some(Klass::ObjArrayKlass(down_type)) => {
                    return Some(Klass::ObjArrayKlass(Arc::new(
                        ObjArrayKlass::multi_dimension(self.clone(), down_type.clone()),
                    )));
                }
                Some(Klass::TypeArrayKlass(down_type)) => {
                    return Some(Klass::TypeArrayKlass(
                        Arc::new(
                        TypeArrayKlass::multi_dimension(
                        self.clone(),
                        down_type.clone(),
                    ))));
                }
                _ => return None,
            }
        }
        None
    }
}

fn new_base_class_loader(){
    let cl = BaseClassLoader {
        class_path_manager: ClassPathManager::new(),
    };
    BaseClassLoader::load_class(Arc::new(cl), "main");

}
