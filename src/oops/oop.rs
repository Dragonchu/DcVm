use crate::jni::jni_md::jint;

use super::klass::{InstanceKlass, Klass};

pub type MirrorOop = Box<MirrorOopDesc>;

pub enum Oop {
    InstanceOopDesc(InstanceOopDesc),
}

pub enum OopType {
    InstanceOop,
    PrimitiveOop,
    ObjectArrayOop,
    TypeArrayOop,
}

pub enum ValueType {
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Boolean,
    Void,
}

pub struct OopMeta<T> {
    oop_type: OopType,
    hash: jint,
    klass: T,
}

pub struct InstanceOopDesc {
    oop_desc: OopMeta<InstanceKlass>,
    instance_field_values: Vec<Oop>,
}

pub struct MirrorOopDesc {
    instance_oop: InstanceOopDesc,
    mirror_target: Option<Klass>,
    mirror_primitive_type: Option<ValueType>,
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

impl InstanceOopDesc {
    fn get_instance_class(&self) -> InstanceKlass {
        self.get_instance_class()
    }
}
