use std::{any::Any, rc::Rc, sync::Arc};

use crate::{common::ValueType, oops::{klass::{InstanceKlass, Klass, KlassRef, ObjectArrayKlass, TypeArrayKlass}, reflection}};

use super::class_path_manager::ClassPathManager;

struct ClassNotFoundError;

type Result<T> = std::result::Result<T, ClassNotFoundError>;

pub trait ClassLoader {
    fn load_class(self: Rc<Self>, class_name: &str) -> Result<KlassRef>;
}

pub struct BootStrapClassLoader {
    class_path_manager: ClassPathManager,
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
        BootStrapClassLoader {
            class_path_manager: ClassPathManager::new(),
            parent: None,
        }
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
        if class_name.chars().nth(1).unwrap() == '[' {
            self.do_load_object_array_class(1, &class_name[2..])
        } else {
            Ok(self.do_load_type_array_class(1, class_name.chars().nth(1).unwrap()))
        }
    }

    fn do_load_multi_dimension_array_class(self: Rc<Self>, class_name: &str) -> Result<KlassRef> {
        let down_type = self.load_class(&class_name[1..]);
    }

    fn do_load_object_array_class(
        self: Rc<Self>,
        dimension: usize,
        down_type_name: &str,
    ) -> Result<KlassRef> {
        let down_type = self.clone().load_class(down_type_name)?;
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

    fn do_load_type_array_class(self: Rc<Self>, dimension: usize, primitive_type: char) -> KlassRef {
        let component_type = reflection::primitive_type_to_value_type_no_wrap(primitive_type);
        Rc::new(TypeArrayKlass::new(self.clone(), dimension, component_type))
    }

    fn do_load_instance_class(&self, class_name: &str) -> Result<KlassRef> {

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
