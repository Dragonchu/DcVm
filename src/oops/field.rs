use std::rc::Rc;

use crate::{
    classfile::{
        attribute_info::{self, ConstantValueAttribute},
        class_file::{CpInfo, FieldInfo},
        constant_pool,
        types::U2,
    },
    common::ValueType,
};

use super::klass::{InstanceKlass, Klass, KlassRef};

pub struct Field {
    klass: Option<Rc<InstanceKlass>>,
    name: Option<String>,
    descriptor: Option<String>,
    signature: Option<String>,
    access_flag: U2,
    value_type: Option<ValueType>,

    /// Only available when _value_type is OBJECT or ARRAY
    value_class_type: Option<KlassRef>,
    value_class_type_name: Option<String>,

    field_info: Option<Rc<FieldInfo>>,
    constant_attr: Option<Rc<ConstantValueAttribute>>,

    linked: bool,
}

impl Field {
    pub fn new(klass: Rc<InstanceKlass>, field_info: Rc<FieldInfo>) -> Field {
        Self {
            linked: false,
            access_flag: field_info.access_flags,
            klass: Some(klass),
            field_info: Some(field_info.clone()),
            constant_attr: None,
            value_class_type: None,
            value_class_type_name: None,
            name: None,
            descriptor: None,
            signature: None,
            value_type: None,
        }
    }

    pub fn link_field(&mut self, pool: &Vec<CpInfo>) {
        if self.linked == true {
            return;
        }
        let field_info = self.field_info.as_ref().unwrap();
        let name = constant_pool::require_constant_utf8(pool, field_info.name_index as usize);
        let desc = constant_pool::require_constant_utf8(pool, field_info.descriptor_index as usize);
        self.name = Some(name);
        self.descriptor = Some(desc);
        self.link_attribute(pool);
    }

    fn link_attribute(&mut self, pool: &Vec<CpInfo>) {
        let field_info = self.field_info.as_ref().unwrap();
        for (_, attr) in field_info.attributes.iter().enumerate() {
            match attr {
                attribute_info::AttributeInfo::ConstantValue(attr) => {
                    self.constant_attr = Some(attr.clone());
                }
                attribute_info::AttributeInfo::Signature {
                    attribute_length: _,
                    attribute_name_index: _,
                    signature_index,
                } => {
                    let signature =
                        constant_pool::require_constant_utf8(pool, *signature_index as usize);
                    self.signature = Some(signature);
                }
                _ => {}
            }
        }
    }

    fn post_link_value_type(&mut self) {
        let desc = self.descriptor.as_ref().unwrap();
        let value_type = ValueType::from(desc);
    }
}
