use std::sync::Arc;

use crate::{
    classfile::attribute_info::{
        BootstrapMethodsAttribute, EnclosingMethodAttribute, InnerClassesAttribute,
    }, classpath::class_loader::ClassLoader, common::types::{ValueType, U2}, runtime::constant_pool::RuntimeConstantPool
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

pub enum Klass {
    InstanceKlass(Arc<InstanceKlass>),
    ArrayKlass(Arc<ArrayKlass>),
    TypeArrayKlass(Arc<TypeArrayKlass>),
    ObjArrayKlass(Arc<ObjArrayKlass>),
    MirrorKlass(Arc<MirrorKlass>),
}

pub struct InstanceKlass {
    state: ClassState,
    access_flags: U2,
    name: String,
    class_type: ClassType,
    supper_klass: &'static Klass,
    class_loader: &'static dyn ClassLoader,
    source_file: String,
    signature: String,
    inner_class_attr: &'static InnerClassesAttribute,
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

pub struct TypeArrayKlass {
    pub class_loader: &'static dyn ClassLoader,
    pub dimension: usize,
    pub component_type: ValueType,
    pub down_dimension_type: Option<&'static TypeArrayKlass>,
}

impl TypeArrayKlass {
    pub fn one_dimension(
        class_loader: Arc<dyn ClassLoader>,
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
        class_loader: &'static dyn ClassLoader,
        down_dimension_type: TypeArrayKlass,
    ) -> Self {
        TypeArrayKlass {
            class_loader,
            dimension: down_dimension_type.dimension + 1,
            component_type: down_dimension_type.component_type.clone(),
            down_dimension_type: Some(down_dimension_type.clone()),
        }
    }
}

pub struct ObjArrayKlass {
    pub class_loader: &'static dyn ClassLoader, 
    pub dimension: usize,
    pub component_type: &'static InstanceKlass<'a>,
    pub down_dimension_type: &'a ObjArrayKlass<'a>,
}

impl ObjArrayKlass {
    pub fn one_dimension(
        class_loader: Arc<dyn ClassLoader>,
        dimension: usize,
        component_type: Arc<InstanceKlass>,
    ) -> Self {
        ObjArrayKlass {
            class_loader,
            dimension,
            component_type,
            down_dimension_type: None,
        }
    }

    pub fn multi_dimension(
        class_loader: Arc<dyn ClassLoader>,
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
