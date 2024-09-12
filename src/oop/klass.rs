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
    inner_class_attr: Option<InnerClassesAttribute>,
    enclosing_method_attr: Option<EnclosingMethodAttribute>,
    boot_strap_methods_attr: BootstrapMethodsAttribute,
    runtime_constant_pool: RuntimeConstantPool,
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
    pub class_loader: ClassLoader,
    pub dimension: usize,
    pub component_type: ValueType,
    pub down_dimension_type: Option<Box<TypeArrayKlass>>,
}

impl TypeArrayKlass {
    pub fn one_dimension(
        class_loader: ClassLoader,
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
        class_loader: ClassLoader,
        dimension: usize,
        component_type: ValueType,
        down_dimension_type: TypeArrayKlass,
    ) -> Self {
        TypeArrayKlass {
            class_loader,
            dimension,
            component_type,
            down_dimension_type: Some(Box::new(down_dimension_type)),
        }
    }
}

pub struct ObjArrayKlass {
    pub class_loader: Box<dyn ClassLoader>,
    pub dimension: usize,
    pub component_type: InstanceKlass,
    pub down_dimension_type: Option<Box<ObjArrayKlass>>,
}

impl ObjArrayKlass {
    pub fn one_dimension(
        class_loader: impl ClassLoader,
        dimension: usize,
        component_type: InstanceKlass,
    ) -> Self {
        ObjArrayKlass {
            class_loader,
            dimension,
            component_type,
            down_dimension_type: None,
        }
    }

    pub fn multi_dimension(
        class_loader: ClassLoader,
        dimension: usize,
        component_type: InstanceKlass,
        down_dimension_type: ObjArrayKlass,
    ) -> Self {
        ObjArrayKlass {
            class_loader,
            dimension,
            component_type,
            down_dimension_type: Some(Box::new(down_dimension_type)),
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
