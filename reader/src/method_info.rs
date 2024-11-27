use std::fmt;

use crate::{attribute_info::AttributeInfo, types::U2};

pub struct MethodInfo {
    pub access_flags: U2,
    pub name_index: U2,
    pub descriptor_index: U2,
    pub attributes_count: U2,
    pub attributes: Vec<AttributeInfo>,
}

impl MethodInfo {
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

impl fmt::Debug for MethodInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n  MethodInfo{{\n\taccess_flags: {},\n\tname_index: {},\n\tdescriptor_index: {},\n\tattributes_count: {},\n\tattributes: {:?}\n}}",
               self.access_flags, self.name_index, self.descriptor_index, self.attributes_count, self.attributes)
    }
}