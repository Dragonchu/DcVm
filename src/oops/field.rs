use std::{cell::RefCell, rc::Rc};

use crate::{
    classfile::{
        attribute_info::{self, ConstantValueAttribute},
        class_file::FieldInfo,
        constant_pool::{self, ConstantPool, CpInfo},
        types::U2,
    },
    common::{ValueType, ACC_STATIC},
};

use super::klass::{instance_klass::InstanceKlass, klass::KlassRef};

#[derive(Debug)]
pub struct Field {
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
    pub fn new(field_info: &FieldInfo, constant_pool: &ConstantPool) -> Self {
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
        let value_class_type_name = match value_type {
            ValueType::Object => {
                let class_name = &descriptor[1..descriptor.len() - 1];
                Some(class_name.to_string())
            }
            ValueType::Array => {
                let class_name = &descriptor;
                Some(class_name.to_string())
            }
            _ => None
        };
        Self {
            linked: true,
            access_flag: field_info.access_flags,
            constant_attr,
            value_class_type: None,
            value_class_type_name: None,
            name,
            descriptor,
            signature,
            value_type,
        }
    }

    pub fn make_identity(&mut self) -> String {
        let belong_to = self.klass.as_mut().unwrap().borrow();
        let mut identity = String::new();
        let class_name: &String = &belong_to.klass_meta.name;
        identity.push_str(class_name);
        identity.push_str(":");
        identity.push_str(self.name.as_ref().unwrap());
        identity.push_str(":");
        identity.push_str(self.descriptor.as_ref().unwrap());
        identity
    }
}
