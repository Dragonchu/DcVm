use crate::oop::{
    klass::{Klass, ObjArrayKlass, TypeArrayKlass},
    reflection,
};

use super::class_path_manager::ClassPathManager;

pub trait ClassLoader {
    fn load_class(&self, class_name: &str) -> Option<Klass>;
}

struct BaseClassLoader {
    class_path_manager: ClassPathManager,
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
                    let component_class = self.load_class(&component);
                    if let Some(Klass::InstanceKlass(component_class)) = component_class {
                        let object_array_klass = Klass::ObjArrayKlass(
                            ObjArrayKlass::one_dimension(self, dimension, component_class),
                        );
                        return Some(object_array_klass);
                    }
                    return None;
                }
                let component_type = reflection::primitive_type_to_value_type_no_wrap(
                    class_name.chars().nth(1).unwrap(),
                );
                let type_array_klass = Klass::TypeArrayKlass(TypeArrayKlass::one_dimension(
                    self,
                    dimension,
                    component_type,
                ));
                return Some(type_array_klass);
            }
            let down_type_name = class_name[1..].to_string();
            let down_type = self.load_class(&down_type_name);
            match down_type {
                Some(Klass::ObjArrayKlass(down_type)) => {
                    return Some(Klass::ObjArrayKlass(ObjArrayKlass::multi_dimension(
                        self,
                        down_type.dimension + 1,
                        down_type.component_type,
                        down_type,
                    )));
                }
                Some(Klass::TypeArrayKlass(down_type)) => {
                    return Some(Klass::TypeArrayKlass(TypeArrayKlass::multi_dimension(
                        self,
                        down_type.dimension + 1,
                        down_type.component_type,
                        down_type,
                    )));
                }
                _ => return None,
            }
        }
        None
    }
}
