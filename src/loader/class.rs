use crate::common::ValueType;

pub enum Class {
    Instance(Instance),
    Array(Array),
}

pub struct Instance {}

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
