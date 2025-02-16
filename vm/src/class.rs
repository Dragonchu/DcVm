use crate::field::Field;
use crate::heap::{Heap, RawPtr};
use crate::method::Method;
use crate::JvmValue;
use reader::class_file::ClassFile;
use reader::constant_pool::{ConstantPool, CpInfo};
use reader::types::U2;
use std::collections::HashMap;
use std::process::id;

#[derive(Clone, Debug)]
pub enum ClassState {
    LOADED,
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
    methods: Vec<Method>,
    i_fields: Vec<Field>,
    s_fields: Vec<Field>,
    s_field_val: Vec<JvmValue>,
    m_name_desc_lookup: HashMap<String, usize>,
    f_name_desc_lookup: HashMap<String, usize>,
    cp: Vec<CpInfo>,
}

impl InstanceKlass {
    pub fn of(class_file: &ClassFile, class_id: usize, heap: &mut Heap) -> InstanceKlass {
        let cp = &class_file.constant_pool;

        // process methods
        let mut m_name_desc_lookup = HashMap::new();
        let mut methods = Vec::new();
        for (idx, m_info) in (&class_file.methods).iter().enumerate() {
            let method = Method::new(m_info, cp);
            m_name_desc_lookup.insert(method.get_fq_name_desc().clone(), idx);
            methods.push(method);
        }

        // process fields
        let mut i_fields = Vec::new();
        let mut s_fields = Vec::new();
        let mut s_field_val = Vec::new();
        let mut f_name_desc_lookup = HashMap::new();
        for field_info in &class_file.fields {
            let field = Field::new(field_info, cp);
            if field.is_static() {
                let default_val = field.get_default();
                f_name_desc_lookup.insert(field.get_fq_name_desc(), s_fields.len());
                s_fields.push(field);
                s_field_val.push(default_val);
            } else {
                f_name_desc_lookup.insert(field.get_fq_name_desc(), i_fields.len());
                i_fields.push(field);
            }
        }

        InstanceKlass {
            class_id,
            class_name: class_file.get_class_name(),
            class_state: ClassState::LOADED,
            super_class: class_file.get_super_class_name(),
            methods,
            i_fields,
            s_fields,
            s_field_val,
            m_name_desc_lookup,
            f_name_desc_lookup,
            cp: cp.clone(),
        }
    }

    pub fn get_method(&self, name: &str, desc: &str) -> Option<&Method> {
        let fq_name = format!("{}.{}", name, desc);
        let opt_idx = self.m_name_desc_lookup.get(&fq_name);
        let idx = match opt_idx {
            Some(value) => value.clone(),
            None => return None,
        };
        self.methods.get(idx)
    }

    pub fn get_field_info(&self, cp_index: U2) -> (String, String, String) {
        self.cp.get_field_info(cp_index)
    }

    pub fn get_static_instance(&self, field_name: &str, field_desc: &str) -> JvmValue {
        let fq_name = format!("{}.{}", field_name, field_desc);
        let opt_idx = self.f_name_desc_lookup.get(&fq_name);
        let idx = match opt_idx {
            None => panic!(),
            Some(value) => value.clone(),
        };
        self.s_field_val[idx]
    }
}

#[derive(Debug, Clone)]
pub struct ArrayKlass {
    pub(crate) class_id: usize,
    pub(crate) dimension: usize,
    pub(crate) component_type: ComponentType,
}

impl Klass {
    pub fn new_array(
        dimension: usize,
        component_type: ComponentType,
        class_id: usize,
    ) -> ArrayKlass {
        ArrayKlass {
            class_id,
            dimension,
            component_type,
        }
    }

    pub fn get_method(&self, name: &str, desc: &str) -> Option<&Method> {
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
            Klass::Instance(instance) => instance.class_id,
            Klass::Array(array) => array.class_id,
        }
    }

    pub fn get_instance_field_cnt(&self) -> usize {
        match self {
            Klass::Instance(instance) => instance.i_fields.len(),
            Klass::Array(array) => {
                panic!()
            }
        }
    }

    pub fn get_static_instance(&self, field_name: &str, desc: &str) -> JvmValue {
        match self {
            Klass::Instance(instance) => instance.get_static_instance(field_name, desc),
            Klass::Array(_) => panic!(),
        }
    }
}
