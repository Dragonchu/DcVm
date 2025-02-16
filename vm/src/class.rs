use crate::field::Field;
use crate::heap::Oop;
use crate::method::Method;
use reader::class_file::ClassFile;
use reader::constant_pool::{ConstantPool, CpInfo};
use reader::types::U2;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum ClassState {
    LOADED,
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Obj(Oop),
    Uninitialized,
    Null,
}

#[derive(Debug, Clone)]
pub enum ComponentType {
    Void,
    Byte,
    Boolean,
    Char,
    Short,
    Int,
    Float,
    Long,
    Double,
    Object(Box<InstanceKlass>),
    Array(Box<ArrayKlass>),
}

#[derive(Debug, Clone)]
pub enum Klass {
    Instance(InstanceKlass),
    Array(ArrayKlass),
}

#[derive(Debug, Clone)]
pub struct InstanceKlass {
    pub(crate) class_id: usize,
    pub(crate) class_name: String,
    pub(crate) class_state: ClassState,
    pub(crate) super_class: String,
    methods: HashMap<String, Method>,
    static_fields: HashMap<String, Field>,
    static_values: HashMap<String, Value>,
    instance_fields: HashMap<String, Field>,
    cp: Vec<CpInfo>,
}

impl InstanceKlass {
    pub fn get_method(&self, name: &str, desc: &str) -> Method {
        let key = format!("{name}_{desc}");
        self.methods.get(&key).unwrap().clone()
    }

    pub fn get_field_info(&self, index: U2) -> (String, String, String) {
        self.cp.get_field_info(index)
    }
    
    pub fn get_instance_field_cnt(&self) -> usize {
        self.instance_fields.len()
    }
    
}

#[derive(Debug, Clone)]
pub struct ArrayKlass {
    pub(crate) class_id: usize,
    pub(crate) dimension: usize,
    pub(crate) component_type: ComponentType,
}

impl Klass {
    pub fn new_instance(class_file: &ClassFile, class_id: usize) -> InstanceKlass {
        let cp = &class_file.constant_pool;

        let mut methods = HashMap::new();
        for m_info in &class_file.methods {
            let method = Method::new(m_info, cp);
            methods.insert(method.get_unique_key(), method);
        }

        let mut static_fields = HashMap::new();
        let mut static_values = HashMap::new();
        let mut instance_fields = HashMap::new();
        for field_info in &class_file.fields {
            let field_name = cp.get_utf8_string(field_info.name_index);
            let field = Field::new(field_info, cp);
            if field.is_static() {
                static_fields.insert(field_name, field);
                //todo make default static values
            } else {
                instance_fields.insert(field_name, field);
            }
        }

        InstanceKlass {
            class_id,
            class_name: class_file.get_class_name(),
            class_state: ClassState::LOADED,
            super_class: class_file.get_super_class_name(),
            methods,
            static_fields,
            static_values,
            instance_fields,
            cp: cp.clone(),
        }
    }

    pub fn new_array(dimension: usize, component_type: ComponentType, class_id: usize) -> ArrayKlass {
        ArrayKlass {
            class_id,
            dimension,
            component_type,
        }
    }

    pub fn get_method(&self, name: &str, desc: &str) -> Method {
        match self {
            Klass::Instance(instance) => instance.get_method(name, desc),
            _ => panic!(),
        }
    }

    pub fn get_field_info(&self, index: U2) -> (String, String, String) {
        match self {
            Klass::Instance(intance) => intance.get_field_info(index),
            _ => panic!(),
        }
    }
    
    pub fn get_class_id(&self) -> usize {
        match self {
            Klass::Instance(instance) => {instance.class_id}
            Klass::Array(array) => {array.class_id}
        }
    }
    
    pub fn get_instance_field_cnt(&self) -> usize {
        match self {
            Klass::Instance(instance) => {
                instance.instance_fields.len()
            }
            Klass::Array(array) => {
                panic!()
            }
        }
    }
}
