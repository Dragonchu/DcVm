use crate::{attribute_info::AttributeInfo, types::U2};
use crate::types::ACC_STATIC;

#[derive(Debug)]
pub struct FieldInfo {
    pub access_flags: U2,
    pub name_index: U2,
    pub descriptor_index: U2,
    pub attributes_count: U2,
    pub attributes: Vec<AttributeInfo>,
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
    
    pub fn is_static(&self) -> bool {
        self.access_flags & ACC_STATIC == ACC_STATIC
    }
}
