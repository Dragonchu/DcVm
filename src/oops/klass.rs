use std::{collections::HashMap, rc::Rc};


use crate::{classfile::{class_file::ClassFile, types::U2}, classpath::class_loader::ClassLoader, common::ValueType};

use super::oop::{MirrorOop, Oop};

pub type KlassRef = Klass;
#[derive(Debug)]
pub enum Klass {
    InstanceKlass(Rc<InstanceKlass>),
    ObjectArrayKlass(Rc<ObjectArrayKlass>),
    TypeArrayKlass(Rc<TypeArrayKlass>),
}
#[derive(Debug)]
pub enum ClassState {
    Allocated,
    Loaded,
    Linked,
    BeginInitialized,
    FullyInitialized,
    InitializationError,
}

#[derive(Debug)]
pub enum ClassType {
    InstanceKlass,
    ObjectArrayKlass,
    TypeArrayKlass,
}

#[derive(Debug)]
pub struct KlassMeta {
    state: Option<ClassState>,
    access_flags: U2,
    name: String,
    ktype: ClassType,
    java_mirror: Option<MirrorOop>,
    super_klass: Option<Box<InstanceKlass>>,
}

#[derive(Debug)]
pub struct InstanceKlass {
    klass_meta: KlassMeta,
    class_loader: Rc<dyn ClassLoader>,
    java_loader: Option<MirrorOop>,
    class_file: ClassFile,
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
    interfaces: HashMap<String, Rc<InstanceKlass>>,
}

impl InstanceKlass {
    pub fn new(class_file: ClassFile, class_loader: Rc<dyn ClassLoader>) -> Self {
         
        Self {
            klass_meta: KlassMeta {
                state: None,
                access_flags: class_file.access_flags,
                name: String::new(),
                ktype: ClassType::InstanceKlass,
                java_mirror: None,
                super_klass: None,
            },
            class_loader: class_loader.clone(),
            java_loader: None,
            class_file: class_file,
            source_file: String::new(),
            signature: String::new(),
            inner_class_attr: String::new(),
            enclosing_method_attr: String::new(),
            boot_strap_methods_attr: String::new(),
            runtime_constant_pool: String::new(),
            static_field_nums: 0,
            instance_field_nums: 0,
            all_methods: HashMap::new(),
            vtable: HashMap::new(),
            static_fields: HashMap::new(),
            static_field_values: Vec::new(),
            interfaces: HashMap::new(),
        }
    }
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
    component_type: Rc<InstanceKlass>,
    down_dimension_type: Option<Box<ObjectArrayKlass>>,
}

impl ObjectArrayKlass {
    pub fn new(
        class_loader: Rc<dyn ClassLoader>,
        dimension: usize,
        component_type: Rc<InstanceKlass>,
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
    down_dimension_type: Option<Rc<TypeArrayKlass>>,
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
