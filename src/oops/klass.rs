use std::{collections::HashMap, hash::Hash};

use crate::classfile::types::U2;

use super::oop::{MirrorOop, MirrorOopDesc, Oop};

pub enum Klass {
    InstanceKlass(InstanceKlass),
}

pub enum ClassState {
    Allocated,
    Loaded,
    Linked,
    BeginInitialized,
    FullyInitialized,
    InitializationError,
}

pub enum ClassType {
    InstanceKlass,
    ObjectArrayKlass,
    TypeArrayKlass,
}

struct KlassMeta {
    state: ClassState,
    access_flags: U2,
    name: String,
    ktype: ClassType,
    java_mirror: Option<MirrorOop>,
    super_klass: Option<Box<InstanceKlass>>,
}
pub struct InstanceKlass {
    klass_meta: KlassMeta,
    class_loader: String,
    java_loader: String,
    class_file: String,
    source_file: String,
    signature: String,
    inner_class_attr: String,
    enclosing_method_attr: String,
    boot_strap_methods_attr: String,
    runtime_constant_pool: String,
    static_field_nums: usize,
    instance_field_nums: usize,
    all_methods: HashMap<String, String>,
    vtable: HashMap<String, String>,
    static_fields: HashMap<String, String>,
    static_field_values: Vec<Oop>,
    interfaces: HashMap<String, Box<InstanceKlass>>,
}
