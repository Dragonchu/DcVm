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
    klass: Option<Rc<RefCell<InstanceKlass>>>,
    name: Option<String>,
    descriptor: Option<String>,
    signature: Option<String>,
    access_flag: U2,
    value_type: Option<ValueType>,

    /// Only available when _value_type is OBJECT or ARRAY
    value_class_type: Option<KlassRef>,
    value_class_type_name: Option<String>,

    field_info: Option<Rc<RefCell<FieldInfo>>>,
    constant_attr: Option<Rc<ConstantValueAttribute>>,

    linked: bool,
}

impl Field {
    pub fn new(klass: Rc<RefCell<InstanceKlass>>, field_info: Rc<RefCell<FieldInfo>>) -> Rc<RefCell<Field>> {
        Rc::new(RefCell::new(Self {
            linked: false,
            access_flag: field_info.borrow().access_flags,
            klass: Some(klass),
            field_info: Some(field_info.clone()),
            constant_attr: None,
            value_class_type: None,
            value_class_type_name: None,
            name: None,
            descriptor: None,
            signature: None,
            value_type: None,
        }))
    }

    pub fn link_field(&mut self) {
        if self.linked == true {
            return;
        }
        self.link_name_and_descriptor();
        self.link_attribute();
        self.post_link_value_type();
        self.linked = true;
    }

    fn link_name_and_descriptor(&mut self) {
        let field_info = self.field_info.as_ref().unwrap();
        let pool = self.klass.as_ref().unwrap().borrow().class_file.constant_pool.clone();
        let name = pool.get_utf8(field_info.name_index as usize);
        let desc = pool.get_utf8(field_info.descriptor_index as usize);
        self.name = Some(name);
        self.descriptor = Some(desc);
    }

    fn link_attribute(&mut self) {
        let klass = self.klass.as_ref().unwrap().borrow_mut();
        let pool = klass.class_file.constant_pool.clone();
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
                    let signature = pool.get_utf8(*signature_index as usize);
                    self.signature = Some(signature);
                }
                _ => {}
            }
        }
    }

    fn post_link_value_type(&mut self) {
        let desc = self.descriptor.as_ref().unwrap();
        let value_type = ValueType::try_from(desc.chars().nth(0).unwrap()).unwrap();
        match value_type {
            ValueType::Object => {
                let class_name = &desc[1..desc.len() - 1];
                self.value_class_type_name = Some(class_name.to_string());
            }
            ValueType::Array => {
                let class_name = &desc;
                self.value_class_type_name = Some(class_name.to_string());
            }
            _ => {}
        }
    }

    pub fn is_static(&self) -> bool {
        self.access_flag & ACC_STATIC == ACC_STATIC
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
