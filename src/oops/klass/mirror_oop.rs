use crate::{common::ValueType, oops::oop::Oop};

use super::{instance_klass::InstanceKlass, klass::Klass, mark_oop::MarkOop};

pub struct MirrorOop(MirrorOopDesc);
struct MirrorOopDesc{
    _mark: MarkOop,
    _klass: InstanceKlass,
    _instance_field_values: Vec<Oop>,
    _mirror_target: Klass,
    _mirror_primitive_type: ValueType,
}

impl MirrorOop {

    fn get_klass(&self) -> &InstanceKlass {
        &self.0._klass
    }

    fn get_mark(&self) -> &MarkOop {
        &self.0._mark
    }

    fn get_instance_class(&self) -> &InstanceKlass {
        &self.0._klass
    }
}