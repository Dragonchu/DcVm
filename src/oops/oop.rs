use std::rc::Rc;

use crate::{common::ValueType, jni::jni_md::jint};

use super::klass::{InstanceKlass, Klass, KlassRef};

pub type MirrorOop = Rc<MirrorOopDesc>;

#[derive(Debug)]
pub enum Oop {
    InstanceOopDesc(InstanceOopDesc),
}

#[derive(Debug)]
pub enum OopType {
    InstanceOop,
    PrimitiveOop,
    ObjectArrayOop,
    TypeArrayOop,
}

#[derive(Debug)]
pub struct OopMeta<T> {
    oop_type: OopType,
    hash: jint,
    klass: T,
}

impl<T> OopMeta<T> {
    fn new(oop_type: OopType, klass: T) -> OopMeta<T> {
        OopMeta {
            oop_type,
            klass,
            hash: 0,
        }
    }

    fn get_klass(&self) -> &T {
        &self.klass
    }
}

pub type InstanceOop = Rc<InstanceOopDesc>;

#[derive(Debug)]
pub struct InstanceOopDesc {
    oop_desc: OopMeta<InstanceKlass>,
    instance_field_values: Vec<Oop>,
}

#[derive(Debug)]
pub struct MirrorOopDesc {
    instance_oop: InstanceOop,
    mirror_target: Option<KlassRef>,
    mirror_primitive_type: Option<ValueType>,
}

impl MirrorOopDesc {
    pub fn new(instance_oop: InstanceOop) -> MirrorOopDesc {
        MirrorOopDesc {
            instance_oop,
            mirror_target: None,
            mirror_primitive_type: None,
        }
    }
}


impl InstanceOopDesc {
    fn get_instance_class(&self) -> &InstanceKlass {
        self.oop_desc.get_klass()
    }
}
