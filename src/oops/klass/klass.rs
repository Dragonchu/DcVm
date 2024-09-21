use std::{cell::RefCell, collections::HashMap, rc::{Rc, Weak}};

use crate::{
    classfile::{
        class_file::ClassFile,
        constant_pool::require_constant,
        types::U2,
    },
    classpath::class_loader::ClassLoader,
    common::{ValueType, ACC_FINAL}, oops::oop::{MirrorOop, MirrorOopDesc},
};

use super::{
    instance_klass::{InstanceKlass, InstanceKlassRef}
};

pub type KlassRef = Klass;
#[derive(Debug, Clone)]
pub enum Klass {
    InstanceKlass(InstanceKlass),
    ObjectArrayKlass(ObjectArrayKlass),
    TypeArrayKlass(TypeArrayKlass),
}
#[derive(Debug, Clone)]
pub enum ClassState {
    Allocated,
    Loaded,
    Linked,
    BeginInitialized,
    FullyInitialized,
    InitializationError,
}

#[derive(Debug, Clone)]
pub enum ClassType {
    InstanceKlass,
    ObjectArrayKlass,
    TypeArrayKlass,
}

#[derive(Debug, Clone)]
pub struct KlassMeta {
    pub state: ClassState,
    pub access_flags: U2,
    pub name: String,
    pub ktype: ClassType,
    pub java_mirror: Option<Box<MirrorOopDesc>>,
    pub super_klass: Option<Box<InstanceKlass>>,
}

#[derive(Debug)]
pub struct ArrayKlassMeta {
    klass_meta: KlassMeta,
    class_loader: Rc<dyn ClassLoader>,
    java_loader: Option<MirrorOop>,
    dimension: usize,
}

impl ArrayKlassMeta {
    fn new(
        class_loader: Rc<dyn ClassLoader>,
        java_loader: Option<MirrorOop>,
        dimension: usize,
        class_type: ClassType,
    ) -> ArrayKlassMeta {
        ArrayKlassMeta {
            klass_meta: KlassMeta {
                state: None,
                access_flags: 0,
                name: String::new(),
                ktype: class_type,
                java_mirror: None,
                super_klass: None,
            },
            class_loader: class_loader.clone(),
            java_loader,
            dimension,
        }
    }
}

#[derive(Debug)]
pub struct ObjectArrayKlass {
    array_klass_meta: ArrayKlassMeta,
    component_type: InstanceKlassRef,
    down_dimension_type: Option<Box<ObjectArrayKlass>>,
}

impl ObjectArrayKlass {
    pub fn new(
        class_loader: Rc<dyn ClassLoader>,
        dimension: usize,
        component_type: InstanceKlassRef,
    ) -> Self {
        let array_klass_meta = ArrayKlassMeta::new(
            class_loader.clone(),
            None,
            dimension,
            ClassType::ObjectArrayKlass,
        );
        Self {
            array_klass_meta,
            component_type: component_type.clone(),
            down_dimension_type: None,
        }
    }

    pub fn recurese_create(
        class_loader: Rc<dyn ClassLoader>,
        down_type: Rc<ObjectArrayKlass>,
    ) -> Self {
        let array_klass_meta = ArrayKlassMeta::new(
            class_loader.clone(),
            down_type.array_klass_meta.java_loader.clone(),
            down_type.array_klass_meta.dimension + 1,
            ClassType::ObjectArrayKlass,
        );
        Self {
            array_klass_meta,
            component_type: down_type.component_type.clone(),
            down_dimension_type: None,
        }
    }
}

#[derive(Debug)]
pub struct TypeArrayKlass {
    array_klass_meta: ArrayKlassMeta,
    component_type: ValueType,
    down_dimension_type: Option<Box<TypeArrayKlass>>,
}

impl TypeArrayKlass {
    pub fn new(
        class_loader: Rc<dyn ClassLoader>,
        dimension: usize,
        component_type: ValueType,
    ) -> TypeArrayKlass {
        let array_klass_meta = ArrayKlassMeta::new(
            class_loader.clone(),
            None,
            dimension,
            ClassType::TypeArrayKlass,
        );
        TypeArrayKlass {
            array_klass_meta,
            component_type,
            down_dimension_type: None,
        }
    }

    pub fn recurese_create(
        class_loader: Rc<dyn ClassLoader>,
        down_type: Rc<TypeArrayKlass>,
    ) -> TypeArrayKlass {
        let array_klass_meta = ArrayKlassMeta::new(
            class_loader.clone(),
            down_type.array_klass_meta.java_loader.clone(),
            down_type.array_klass_meta.dimension + 1,
            ClassType::TypeArrayKlass,
        );
        TypeArrayKlass {
            array_klass_meta,
            component_type: down_type.component_type.clone(),
            down_dimension_type: None,
        }
    }
}
