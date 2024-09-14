use std::sync::Arc;

use crate::{
    classfile::attribute_info::{
        BootstrapMethodsAttribute, EnclosingMethodAttribute, InnerClassesAttribute,
    },
    classpath::class_loader::ClassLoaderRef,
    common::types::{ValueType, U2},
    runtime::constant_pool::RuntimeConstantPool,
};

pub enum ClassType {
    InstanceClass,
    ObjectArrayClass,
    TypeArrayClass,
}
pub enum ClassState {
    Allocated,
    Loaded,
    Linked,
    BeingInitialized,
    FullyInitialized,
    InitializationError,
}

type KlassRef = Option<Arc<Klass>>;

pub enum Klass {
    InstanceKlass(InstanceKlass),
    ArrayKlass(ArrayKlass),
    TypeArrayKlass(TypeArrayKlass),
    ObjArrayKlass(ObjArrayKlass),
    MirrorKlass(MirrorKlass),
}

pub type InstanceKlassRef = Option<Arc<InstanceKlass>>;

pub struct InstanceKlass {
    state: ClassState,
    access_flags: U2,
    name: String,
    class_type: ClassType,
    supper_klass: KlassRef,
    class_loader: ClassLoaderRef,
    source_file: String,
    signature: String,
    inner_class_attr: InnerClassesAttribute,
    enclosing_method_attr: &'static EnclosingMethodAttribute,
    boot_strap_methods_attr: &'static BootstrapMethodsAttribute,
    runtime_constant_pool: &'static RuntimeConstantPool,
    static_field_nums: usize,
    instance_field_nums: usize,
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

pub type TypeArrayKlassRef = Option<Arc<TypeArrayKlass>>;

pub struct TypeArrayKlass {
    pub class_loader: ClassLoaderRef,
    pub dimension: usize,
    pub component_type: ValueType,
    pub down_dimension_type: TypeArrayKlassRef,
}

impl TypeArrayKlass {
    pub fn one_dimension(
        class_loader: ClassLoaderRef,
        dimension: usize,
        component_type: ValueType,
    ) -> Self {
        TypeArrayKlass {
            class_loader,
            dimension,
            component_type,
            down_dimension_type: None,
        }
    }

    pub fn multi_dimension(
        class_loader: ClassLoaderRef,
        down_dimension_type: TypeArrayKlass,
    ) -> Self {
        TypeArrayKlass {
            class_loader,
            dimension: down_dimension_type.dimension + 1,
            component_type: down_dimension_type.component_type.clone(),
            down_dimension_type: Some(down_dimension_type),
        }
    }
}

pub type ObjArrayKlassRef = Option<Arc<ObjArrayKlass>>;

pub struct ObjArrayKlass {
    pub class_loader: ClassLoaderRef,
    pub dimension: usize,
    pub component_type: &'static InstanceKlass,
    pub down_dimension_type: ObjArrayKlassRef,
}

impl ObjArrayKlass {
    pub fn one_dimension(
        class_loader: ClassLoaderRef,
        dimension: usize,
        component_type: &'static InstanceKlass,
    ) -> Self {
        ObjArrayKlass {
            class_loader,
            dimension,
            component_type,
            down_dimension_type: None,
        }
    }

    pub fn multi_dimension(
        class_loader: ClassLoaderRef,
        down_dimension_type: Arc<ObjArrayKlass>,
    ) -> Self {
        ObjArrayKlass {
            class_loader,
            dimension: down_dimension_type.dimension + 1,
            component_type: down_dimension_type.component_type.clone(),
            down_dimension_type: Some(down_dimension_type),
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
