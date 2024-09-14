use std::rc::Rc;

use super::klass::{InstanceKlass, Klass};

pub enum Oop {
    InstanceOopDesc(InstanceOopDesc)
}

pub struct OopMeta<T> {
    pub klass: T,
}



pub struct InstanceOopDesc {
    oop_desc: OopMeta<InstanceKlass>,
    instance_field_values: Vec<Oop>,
}

impl InstanceOopDesc {
    fn get_instance_class() -> InstanceKlass {

    }
}
