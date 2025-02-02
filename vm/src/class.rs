use crate::field::Field;
use crate::method::Method;
use reader::class_file::ClassFile;
use reader::constant_pool::{ConstantPool, CpInfo};
use std::collections::HashMap;
use crate::heap::{ObjectPtr, RawObject};
use reader::types::U2;

#[derive(Clone, Debug)]
pub enum ClassState {
   LOADED
}

#[derive(Debug, Clone)]
pub enum Value {
    Boolean(bool),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Char(char),
    Instance(RawObject),
    Uninitialized
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
    pub(crate) class_name: String,
    pub(crate) class_state: ClassState,
    pub(crate) super_class: String,
    methods: HashMap<String, Method>,
    static_fields: HashMap<String, Field>,
    static_values: HashMap<String, Value>,
    instance_fields: HashMap<String, Field>,
    cp: Vec<CpInfo>,
}

#[derive(Debug, Clone)]
pub struct ArrayKlass {
    dimension: usize,
    component_type: ComponentType,
}

impl Klass {
    pub fn new_instance(class_file: &ClassFile) -> InstanceKlass {
        let cp = &class_file.constant_pool;

        let mut methods = HashMap::new();
        for m_info in &class_file.methods {
            let method = Method::new(m_info, cp);
            methods.insert(method.get_name(), method);
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
    
    pub fn new_array(dimension: usize, component_type: ComponentType) -> ArrayKlass {
        ArrayKlass {
            dimension,
            component_type,
        }
    }

    pub fn get_method(&self, name: &str,args: &str) -> Method {
        todo!()
    }

    pub fn get_field_info(&self, index: U2) -> &Field {
        todo!()
    }
}
