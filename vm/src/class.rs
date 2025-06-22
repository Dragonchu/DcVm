use crate::field::Field;
use crate::heap::{Heap, RawPtr};
use crate::method::Method;
use crate::JvmValue;
use reader::class_file::ClassFile;
use reader::constant_pool::{ConstantPool, CpInfo};
use reader::types::U2;
use std::collections::HashMap;
use std::process::id;
use crate::logger::Logger;

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
        let class_name = class_file.get_class_name();
        for (idx, m_info) in (&class_file.methods).iter().enumerate() {
            let mut method = Method::from_method_info(m_info, cp);
            // 临时修复：只有java/lang/Object.registerNatives才加ACC_NATIVE
            if class_name == "java/lang/Object" && method.name == "registerNatives" && method.descriptor == "()V" {
                method.access_flags |= 0x0100; // ACC_NATIVE
            }
            // 调试：打印java/lang/Object的所有方法
            if class_name == "java/lang/Object" {
                println!("[Object methods] {}: {}.{}", idx, method.name, method.descriptor);
            }
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

    pub fn get_method(&self, method_name: &str, method_desc: &str) -> Option<&Method> {
        Logger::log_fmt(format_args!("[get_method entry] name: {}, desc: {}", method_name, method_desc));
        let fq_name = format!("{}.{}", method_name, method_desc);
        Logger::log_fmt(format_args!("[get_method] 查找key: {}", fq_name));
        Logger::log_fmt(format_args!("[get_method] 所有key: {:?}", self.m_name_desc_lookup.keys().collect::<Vec<_>>()));
        let opt_idx = self.m_name_desc_lookup.get(&fq_name);
        let idx = match opt_idx {
            Some(value) => value.clone(),
            None => return None,
        };
        self.methods.get(idx)
    }

    /// 在继承链上查找方法，支持方法重写
    pub fn lookup_method(&self, method_name: &str, method_desc: &str, vm: &mut crate::vm::Vm) -> Option<Method> {
        Logger::log_fmt(format_args!("[lookup_method entry] name: {}, desc: {}", method_name, method_desc));
        // 首先在当前类中查找
        if let Some(method) = self.get_method(method_name, method_desc) {
            Logger::log_fmt(format_args!("[lookup_method] class: {}, method: {}, desc: {}, access_flags: 0x{:x}", self.class_name, method_name, method_desc, method.access_flags));
            return Some(method.clone());
        }
        
        // 特殊方法处理：<clinit>和<init>方法不应该在父类中查找
        if method_name == "<clinit>" || method_name == "<init>" {
            Logger::log_fmt(format_args!("[lookup_method] 特殊方法 {} 在当前类 {} 中未找到，不查找父类", method_name, self.class_name));
            return None;
        }
        
        // 如果当前类没有找到，在父类中查找
        if !self.super_class.is_empty() {
            Logger::log_fmt(format_args!("[lookup_method] 递归父类: {} 传递name: {}, desc: {}", self.super_class, method_name, method_desc));
            // 尝试加载父类
            if let Ok(super_klass) = vm.load(&self.super_class) {
                if let crate::class::Klass::Instance(super_instance) = &super_klass {
                    let result = super_instance.lookup_method(method_name, method_desc, vm);
                    Logger::log_fmt(format_args!("[lookup_method] 父类返回: {:?}", result.as_ref().map(|m| m.name.as_str())));
                    return result;
                }
            }
        }
        
        None
    }

    /// 获取类名
    pub fn get_class_name(&self) -> &str {
        &self.class_name
    }

    /// 获取父类名
    pub fn get_super_class_name(&self) -> &str {
        &self.super_class
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

    pub fn get_static_fields(&self) -> &Vec<Field> {
        &self.s_fields
    }

    pub fn get_static_field_values(&self) -> &Vec<JvmValue> {
        &self.s_field_val
    }

    pub fn get_instance_fields(&self) -> &Vec<Field> {
        &self.i_fields
    }

    pub fn get_static_fields_mut(&mut self) -> &mut Vec<Field> {
        &mut self.s_fields
    }

    pub fn get_static_field_values_mut(&mut self) -> &mut Vec<JvmValue> {
        &mut self.s_field_val
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

    pub fn get_method(&self, method_name: &str, method_desc: &str) -> Option<&Method> {
        match self {
            Klass::Instance(instance) => instance.get_method(method_name, method_desc),
            _ => panic!(),
        }
    }

    /// 在继承链上查找方法，支持方法重写
    pub fn lookup_method(&self, method_name: &str, method_desc: &str, vm: &mut crate::vm::Vm) -> Option<Method> {
        match self {
            Klass::Instance(instance) => instance.lookup_method(method_name, method_desc, vm),
            _ => None,
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

    /// 获取类名
    pub fn get_class_name(&self) -> Option<&str> {
        match self {
            Klass::Instance(instance) => Some(&instance.class_name),
            Klass::Array(_) => None,
        }
    }
}
