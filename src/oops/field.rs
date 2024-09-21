use std::{cell::RefCell, rc::{Rc, Weak}};

use crate::{
    classfile::{
        attribute_info::{self, ConstantValueAttribute},
        class_file::FieldInfo,
        constant_pool::{self, ConstantPool, CpInfo},
        types::U2,
    }, classpath::{class_loader::ClassLoader, class_path_manager::CLASS_PATH_MANGER}, common::{ValueType, ACC_STATIC}
};

use super::klass::{instance_klass::InstanceKlass, klass::KlassRef};

#[derive(Debug)]
pub struct Field {
    klass: InstanceKlass,
    name: String,
    descriptor: String,
    signature: Option<String>,
    access_flag: U2,
    value_type: ValueType,
    /// Only available when _value_type is OBJECT or ARRAY
    value_class_type: Option<KlassRef>,
    value_class_type_name: Option<String>,
    constant_attr: Option<ConstantValueAttribute>,
    linked: bool,
}

impl Field {
    pub fn new(klass: Weak<InstanceKlass>,field_info: &FieldInfo) -> Self {
        let constant_pool = &klass.upgrade().unwrap().class_file.constant_pool;
        let class_loader =  &klass.upgrade().unwrap().class_loader;
        let name = constant_pool.get_utf8(field_info.name_index as usize);
        let descriptor = constant_pool.get_utf8(field_info.descriptor_index as usize);
        let mut constant_attr = None;
        let mut signature = None;
        for (_, attr) in field_info.attributes.iter().enumerate() {
            match attr {
                attribute_info::AttributeInfo::ConstantValue(attr) => {
                    constant_attr = Some(attr.clone());
                }
                attribute_info::AttributeInfo::Signature {
                    attribute_length: _,
                    attribute_name_index: _,
                    signature_index,
                } => {
                    let signature_str = constant_pool.get_utf8(*signature_index as usize);
                    signature = Some(signature_str);
                }
                _ => {}
            }
        }
        let value_type = ValueType::try_from(descriptor.chars().nth(0).unwrap()).unwrap();
        let (value_class_type_name, value_class_type) = match value_type {
            ValueType::Object => {
                let class_type_name = &descriptor[1..descriptor.len() - 1];
                let class_type = class_loader.load_class(class_type_name).expect("Class not found"); 
                (Some(class_type_name.to_string()), Some(class_type))
            }
            ValueType::Array => {
                let class_type_name = &descriptor;
                let class_type = class_loader.load_class(class_type_name).expect("Class not found");
                (Some(class_type_name.to_string()), Some(class_type))
            }
            _ => (None, None)
        };
        Self {
            klass,
            linked: true,
            access_flag: field_info.access_flags,
            constant_attr,
            value_class_type,
            value_class_type_name,
            name,
            descriptor,
            signature,
            value_type,
        }
    }
}

