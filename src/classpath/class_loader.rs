use std::rc::Rc;

use crate::oops::{
    klass::{InstanceKlass, KlassRef, ObjectArrayKlass, TypeArrayKlass},
    reflection,
};

use super::class_path_manager::ClassPathManager;

#[derive(Debug, Clone)]
pub struct ClassNotFoundError;

type Result<T> = std::result::Result<T, ClassNotFoundError>;

pub trait ClassLoader {
    fn load_class(
        self: Rc<Self>,
        class_name: &str,
        class_path_manager: ClassPathManager,
    ) -> Result<KlassRef>;
}

pub struct BootStrapClassLoader {
    parent: Option<Rc<dyn ClassLoader>>,
}

impl ClassLoader for BootStrapClassLoader {
    fn load_class(
        self: Rc<Self>,
        class_name: &str,
        class_path_manager: ClassPathManager,
    ) -> Result<KlassRef> {
        if let Some(cl) = &self.parent {
            ClassLoader::load_class(cl.to_owned(), class_name, class_path_manager)
        } else {
            self.do_load_class(class_name, class_path_manager)
        }
    }
}

impl BootStrapClassLoader {
    pub fn new() -> BootStrapClassLoader {
        BootStrapClassLoader { parent: None }
    }

    fn do_load_class(
        self: Rc<Self>,
        class_name: &str,
        class_path_manager: ClassPathManager,
    ) -> Result<KlassRef> {
        if self.is_array_class(class_name) {
            self.do_load_array_class(class_name, class_path_manager)
        } else {
            self.do_load_instance_class(class_name, class_path_manager)
        }
    }

    fn do_load_array_class(
        self: Rc<Self>,
        class_name: &str,
        class_path_manager: ClassPathManager,
    ) -> Result<KlassRef> {
        let dimension = self.calculate_arr_dimension(class_name);
        if dimension == 1 {
            self.do_load_one_dimension_array_class(class_name, class_path_manager)
        } else {
            self.do_load_multi_dimension_array_class(class_name, class_path_manager)
        }
    }

    fn do_load_one_dimension_array_class(
        self: Rc<Self>,
        class_name: &str,
        class_path_manager: ClassPathManager,
    ) -> Result<KlassRef> {
        if class_name.chars().nth(1).unwrap() == '[' {
            self.do_load_object_array_class(1, &class_name[2..], class_path_manager)
        } else {
            Ok(self.do_load_type_array_class(1, class_name.chars().nth(1).unwrap()))
        }
    }

    fn do_load_multi_dimension_array_class(
        self: Rc<Self>,
        class_name: &str,
        class_path_manager: ClassPathManager,
    ) -> Result<KlassRef> {
        let down_type = self
            .clone()
            .load_class(&class_name[1..], class_path_manager)?;
        if let Ok(object_array_klass) = down_type.clone().downcast_rc::<ObjectArrayKlass>() {
            Ok(Rc::new(ObjectArrayKlass::recurese_create(
                self.clone(),
                object_array_klass.clone(),
            )))
        } else if let Ok(type_array_klass) = down_type.clone().downcast_rc::<TypeArrayKlass>() {
            Ok(Rc::new(TypeArrayKlass::recurese_create(
                self.clone(),
                type_array_klass.clone(),
            )))
        } else {
            Err(ClassNotFoundError)
        }
    }

    fn do_load_object_array_class(
        self: Rc<Self>,
        dimension: usize,
        down_type_name: &str,
        class_path_manager: ClassPathManager,
    ) -> Result<KlassRef> {
        let down_type = self
            .clone()
            .load_class(down_type_name, class_path_manager)?;
        if let Ok(instance_klass) = down_type.downcast_rc::<InstanceKlass>() {
            Ok(Rc::new(ObjectArrayKlass::new(
                self.clone(),
                dimension,
                instance_klass.clone(),
            )))
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
        Rc::new(TypeArrayKlass::new(self.clone(), dimension, component_type))
    }

    fn do_load_instance_class(
        self: Rc<Self>,
        class_name: &str,
        mut class_path_manager: ClassPathManager,
    ) -> Result<KlassRef> {
        if let Ok(class_file) = class_path_manager.search_class(class_name) {
            Ok(Rc::new(InstanceKlass::new(class_file, self.clone())))
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
        let mut class_path_manager = ClassPathManager::new();
        class_path_manager.add_class_path(d.to_str().unwrap());
        let class_loader = Rc::new(BootStrapClassLoader::new());
        let class_name = "java/lang/String";
        let klass = class_loader
            .load_class(class_name, class_path_manager)
            .unwrap();
    }

    #[test]
    fn test_bootstrap_class_loader_array() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test");
        let mut class_path_manager = ClassPathManager::new();
        class_path_manager.add_class_path(d.to_str().unwrap());
        let class_loader = Rc::new(BootStrapClassLoader::new());
        let class_name = "[Ljava/lang/String;";
        let klass = class_loader
            .load_class(class_name, class_path_manager)
            .unwrap();
    }

    #[test]
    fn test_bootstrap_class_loader_multi_array() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test");
        let mut class_path_manager = ClassPathManager::new();
        class_path_manager.add_class_path(d.to_str().unwrap());
        let class_loader = Rc::new(BootStrapClassLoader::new());
        let class_name = "[[[Ljava/lang/String;";
        let klass = class_loader
            .load_class(class_name, class_path_manager)
            .unwrap();
    }

    #[test]
    fn test_bootstrap_class_loader_primitive_array() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test");
        let mut class_path_manager = ClassPathManager::new();
        class_path_manager.add_class_path(d.to_str().unwrap());
        let class_loader = Rc::new(BootStrapClassLoader::new());
        let class_name = "[I";
        let klass = class_loader
            .load_class(class_name, class_path_manager)
            .unwrap();
    }
}
