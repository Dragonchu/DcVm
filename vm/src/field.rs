use reader::{
    constant_pool::ConstantPool,
    field_info::FieldInfo,
    types::{ACC_STATIC, U2},
};

pub enum ValueType {
    Void,
    Byte,
    Boolean,
    Char,
    Short,
    Int,
    Float,
    Long,
    Double,
    Object(String),
    Array(String),
}

#[derive(Debug, Clone)]
pub struct Field {
    name: String,
    descriptor: String,
    access_flags: U2,
}
impl Field {
    pub fn new(field_info: &FieldInfo, cp_pool: &dyn ConstantPool) -> Field {
        Field {
            name: cp_pool.get_utf8_string(field_info.name_index),
            descriptor: cp_pool.get_utf8_string(field_info.descriptor_index),
            access_flags: field_info.access_flags,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_descriptor(&self) -> String {
        self.descriptor.clone()
    }

    pub fn is_static(&self) -> bool {
        self.access_flags & ACC_STATIC == ACC_STATIC
    }
}
