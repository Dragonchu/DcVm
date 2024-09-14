use std::sync::Arc;

use crate::oop::{
    klass::{Klass, ObjArrayKlass, TypeArrayKlass},
    reflection,
};

use super::class_path_manager::ClassPathManager;

pub type ClassLoaderRef = Option<Arc<ClassLoader>>;

struct ClassLoader {
    class_path_manager: ClassPathManager,
    parent: ClassLoaderRef,
}

impl ClassLoader {
    fn load_class(self: Arc<Self>, class_name: &str) -> Option<Klass> {
        if class_name.starts_with('[') {
            let mut dimension = 0;
            while class_name.chars().nth(dimension).unwrap() == '[' {
                dimension += 1;
            }
            if dimension == 1 {
                if class_name.chars().nth(1).unwrap() == 'L' {
                    let component = class_name[1..].to_string();
                    let component_class = self.load_class(&component);
                    if let Some(Klass::InstanceKlass(component_type)) = component_class {
                        let object_array_klass =
                            Klass::ObjArrayKlass(ObjArrayKlass::one_dimension(
                                Some(self.clone()),
                                dimension,
                                &component_type,
                            ));
                        return Some(object_array_klass);
                    }
                    return None;
                }
                let component_type = reflection::primitive_type_to_value_type_no_wrap(
                    class_name.chars().nth(1).unwrap(),
                );
                let type_array_klass = Klass::TypeArrayKlass(TypeArrayKlass::one_dimension(
                    Some(self.clone()),
                    dimension,
                    component_type,
                ));
                return Some(type_array_klass);
            }
            let down_type_name = class_name[1..].to_string();
            let down_type = self.clone().load_class(&down_type_name);
            match down_type {
                Some(Klass::ObjArrayKlass(down_type)) => {
                    return Some(Klass::ObjArrayKlass(ObjArrayKlass::multi_dimension(
                        Some(self.clone()),
                        down_type,
                    )));
                }
                Some(Klass::TypeArrayKlass(down_type)) => {
                    return Some(Klass::TypeArrayKlass(TypeArrayKlass::multi_dimension(
                        Some(self.clone()),
                        down_type,
                    )));
                }
                _ => return None,
            }
        }
        None
    }
}
