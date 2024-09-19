use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::oops::{
    klass::{
        instance_klass::{InstanceKlass, InstanceKlassRef},
        klass::{Klass, KlassRef, ObjectArrayKlass, TypeArrayKlass},
    },
    reflection,
};

use super::class_path_manager::{ClassPathManager, CLASS_PATH_MANGER};

#[derive(Debug, Clone)]
pub struct ClassNotFoundError;

type Result<T> = std::result::Result<T, ClassNotFoundError>;

pub trait ClassLoader: Debug {
    fn load_class(self: Rc<Self>, class_name: &str) -> Result<KlassRef>;
}

#[derive(Debug)]
pub struct BootStrapClassLoader {
    parent: Option<Rc<dyn ClassLoader>>,
}

impl ClassLoader for BootStrapClassLoader {
    fn load_class(self: Rc<Self>, class_name: &str) -> Result<KlassRef> {
        if let Some(cl) = &self.parent {
            ClassLoader::load_class(cl.to_owned(), class_name)
        } else {
            self.do_load_class(class_name)
        }
    }
}

impl BootStrapClassLoader {
    pub fn new() -> BootStrapClassLoader {
        BootStrapClassLoader { parent: None }
    }

    fn do_load_class(self: Rc<Self>, class_name: &str) -> Result<KlassRef> {
        if self.is_array_class(class_name) {
            self.do_load_array_class(class_name)
        } else {
            self.do_load_instance_class(class_name)
        }
    }

    fn do_load_array_class(self: Rc<Self>, class_name: &str) -> Result<KlassRef> {
        let dimension = self.calculate_arr_dimension(class_name);
        if dimension == 1 {
            self.do_load_one_dimension_array_class(class_name)
        } else {
            self.do_load_multi_dimension_array_class(class_name)
        }
    }

    fn do_load_one_dimension_array_class(self: Rc<Self>, class_name: &str) -> Result<KlassRef> {
        if class_name.chars().nth(1).unwrap() == 'L' {
            self.do_load_object_array_class(1, &class_name[2..])
        } else {
            Ok(self.do_load_type_array_class(1, class_name.chars().nth(1).unwrap()))
        }
    }

    fn do_load_multi_dimension_array_class(self: Rc<Self>, class_name: &str) -> Result<KlassRef> {
        let down_type = self.clone().load_class(&class_name[1..])?;
        if let Klass::ObjectArrayKlass(object_array_klass_ref) = down_type {
            Ok(Klass::ObjectArrayKlass(Rc::new(
                ObjectArrayKlass::recurese_create(self.clone(), object_array_klass_ref.clone()),
            )))
        } else if let Klass::TypeArrayKlass(type_array_klass_ref) = down_type {
            Ok(Klass::TypeArrayKlass(Rc::new(
                TypeArrayKlass::recurese_create(self.clone(), type_array_klass_ref.clone()),
            )))
        } else {
            Err(ClassNotFoundError)
        }
    }

    fn do_load_object_array_class(
        self: Rc<Self>,
        dimension: usize,
        down_type_name: &str,
    ) -> Result<KlassRef> {
        let down_type = self.clone().load_class(down_type_name)?;
        if let Klass::InstanceKlass(instance_klass_ref) = down_type {
            Ok(Klass::ObjectArrayKlass(Rc::new(ObjectArrayKlass::new(
                self.clone(),
                dimension,
                instance_klass_ref.clone(),
            ))))
        } else {
            Err(ClassNotFoundError)
        }
    }

    fn do_load_type_array_class(
        self: Rc<Self>,
        dimension: usize,
        primitive_type: char,
    ) -> KlassRef {
        let component_type = reflection::primitive_type_to_value_type_no_wrap(primitive_type);
        Klass::TypeArrayKlass(Rc::new(TypeArrayKlass::new(
            self.clone(),
            dimension,
            component_type,
        )))
    }

    fn do_load_instance_class(self: Rc<Self>, class_name: &str) -> Result<KlassRef> {
        if let Ok(class_file) = CLASS_PATH_MANGER.lock().unwrap().search_class(class_name) {
            Ok(Klass::InstanceKlass(InstanceKlassRef {
                layout: InstanceKlass::new(Box::new(class_file), self.clone()),
            }))
        } else {
            Err(ClassNotFoundError)
        }
    }

    fn calculate_arr_dimension(&self, class_name: &str) -> usize {
        let mut dimension = 0;
        for c in class_name.chars() {
            if c == '[' {
                dimension += 1;
            } else {
                break;
            }
        }
        dimension
    }

    fn is_array_class(&self, class_name: &str) -> bool {
        class_name.starts_with('[')
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_bootstrap_class_loader() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test");
        let mut class_path_manager = CLASS_PATH_MANGER.lock().unwrap();
        class_path_manager.add_class_path(d.to_str().unwrap());
        let class_loader = Rc::new(BootStrapClassLoader::new());
        let class_name = "Main";
        let klass = class_loader.load_class(class_name).unwrap();
        print!("{:?}", klass);
    }

    #[test]
    fn test_bootstrap_class_loader_array() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test");
        let mut class_path_manager = CLASS_PATH_MANGER.lock().unwrap();
        class_path_manager.add_class_path(d.to_str().unwrap());
        let class_loader = Rc::new(BootStrapClassLoader::new());
        let class_name = "[Ljava/lang/String;";
        let klass = class_loader.load_class(class_name).unwrap();
    }

    #[test]
    fn test_bootstrap_class_loader_multi_array() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test");
        let mut class_path_manager = CLASS_PATH_MANGER.lock().unwrap();
        class_path_manager.add_class_path(d.to_str().unwrap());
        let class_loader = Rc::new(BootStrapClassLoader::new());
        let class_name = "[[[LString";
        let klass = class_loader.load_class(class_name).unwrap();
        print!("{:?}", klass);
    }

    #[test]
    fn test_bootstrap_class_loader_primitive_array() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test");
        let mut class_path_manager = CLASS_PATH_MANGER.lock().unwrap();
        class_path_manager.add_class_path(d.to_str().unwrap());
        let class_loader = Rc::new(BootStrapClassLoader::new());
        let class_name = "[I";
        let klass = class_loader.load_class(class_name).unwrap();
        print!("{:?}", klass);
    }
}
