use reader::{
    constant_pool::ConstantPool,
    field_info::FieldInfo,
    types::{ACC_STATIC, U2},
};
use crate::JvmValue;

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
    offset: usize,
}
impl Field {
    pub fn new(field_info: &FieldInfo, cp_pool: &dyn ConstantPool) -> Field {
        Field {
            name: cp_pool.get_utf8_string(field_info.name_index),
            descriptor: cp_pool.get_utf8_string(field_info.descriptor_index),
            access_flags: field_info.access_flags,
            offset: 0,
        }
    }
    
    pub fn get_fq_name_desc(&self) -> String {
        format!("{}.{}", self.name, self.descriptor)
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
    
    pub fn get_default(&self) -> JvmValue {
        match self.descriptor.as_str() {
            "Z" => JvmValue::Boolean(0),
            "B" => JvmValue::Byte(0),
            "S" => JvmValue::Short(0),
            "C" => JvmValue::Char(0),
            "I" => JvmValue::Int(0),
            "J" => JvmValue::Long(0),
            "F" => JvmValue::Float(0),
            "D" => JvmValue::Double(0),
            _ => JvmValue::Null,
        }
    }

    pub fn get_offset(&self) -> usize {
        self.offset
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
}
