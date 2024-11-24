use crate::{attribute_info::AttributeInfo, types::U2};

#[derive(Debug)]
pub struct FieldInfo {
    access_flags: U2,
    name_index: U2,
    descriptor_index: U2,
    attributes_count: U2,
    attributes: Vec<AttributeInfo>,
}

impl FieldInfo {
    pub fn new(
        access_flags: U2,
        name_index: U2,
        descriptor_index: U2,
        attributes: Vec<AttributeInfo>,
    ) -> Self {
        Self {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count: attributes.len() as U2,
            attributes,
        }
    }
}