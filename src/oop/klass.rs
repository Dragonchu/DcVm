use crate::{classpath::class_loader::ClassLoader, common::types::U2};

pub enum ClassType {
  InstanceClass,
  ObjectArrayClass,
  TypeArrayClass
}
pub enum ClassState {
    Allocated,
    Loaded,
    Linked,
    BeingInitialized,
    FullyInitialized,
    InitializationError,
}

pub enum Klass {
    InstanceKlass(InstanceKlass),
    ArrayKlass(ArrayKlass),
    TypeArrayKlass(TypeArrayKlass),
    ObjArrayKlass(ObjArrayKlass),
    MirrorKlass(MirrorKlass),
}

pub struct InstanceKlass {
    state: ClassState,
    access_flags: U2,
    name: String,
    class_type: ClassType,
    supper_klass: Option<Box<Klass>>,
    class_loader: ClassLoader,
    source_file: String,
    signature: String,
    inner_class_attr: 
}

pub struct ArrayKlass {
    state: ClassState,
    access_flags: U2,
    name: String,
}

impl ArrayKlass {
    pub fn new(access_flags: U2, name: String) -> Self {
        ArrayKlass {
            state: ClassState::Allocated,
            access_flags,
            name,
        }
    }
}

pub struct TypeArrayKlass {
    state: ClassState,
    access_flags: U2,
    name: String,
}

impl TypeArrayKlass {
    pub fn new(access_flags: U2, name: String) -> Self {
        TypeArrayKlass {
            state: ClassState::Allocated,
            access_flags,
            name,
        }
    }
}

pub struct ObjArrayKlass {
    state: ClassState,
    access_flags: U2,
    name: String,
}

impl ObjArrayKlass {
    pub fn new(access_flags: U2, name: String) -> Self {
        ObjArrayKlass {
            state: ClassState::Allocated,
            access_flags,
            name,
        }
    }
}

pub struct MirrorKlass {
    state: ClassState,
    access_flags: U2,
    name: String,
}

impl MirrorKlass {
    pub fn new(access_flags: U2, name: String) -> Self {
        MirrorKlass {
            state: ClassState::Allocated,
            access_flags,
            name,
        }
    }
}
