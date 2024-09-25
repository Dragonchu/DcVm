use crate::common::{ValueType, U2};

pub enum Class {
    Instance(Instance),
    Array(Array),
}

pub struct Field {
    access_flags: U2,
    name: String,
    descriptor: String,
}

pub struct Instance {
    access_flags: U2,
    class_name: String,
    super_class_name: String,
    interfaces: Vec<String>,

}

pub enum ElementType {
    Object(Instance),
    Primitive(ValueType),
}

pub struct Array {
    element_type: ElementType,
    dimensions: usize,
}

impl Instance {
    fn new() -> Self {
        Self {}
    }

    fn get_static_method(&self, method_name: &str, descriptor: &str) {
        todo!()
    }
}
