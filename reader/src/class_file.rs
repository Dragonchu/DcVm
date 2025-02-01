use std::fmt;

use crate::{
    constant_pool::CpInfo,
    field_info::FieldInfo,
    method_info::MethodInfo,
    types::{U2, U4},
};

use super::attribute_info::AttributeInfo;

use crate::constant_pool::ConstantPool;

#[derive(Debug)]
pub struct ClassFile {
    pub magic: U4,
    pub minor_version: U2,
    pub major_version: U2,
    pub constant_pool_count: U2,
    pub constant_pool: Vec<CpInfo>,
    pub access_flags: U2,
    pub this_class: U2,
    pub super_class: U2,
    pub interfaces_count: U2,
    pub interfaces: Vec<U2>,
    pub fields_count: U2,
    pub fields: Vec<FieldInfo>,
    pub methods_count: U2,
    pub methods: Vec<MethodInfo>,
    pub attributes_count: U2,
    pub attributes: Vec<AttributeInfo>,
}

impl ClassFile {
    pub fn new(
        magic: U4,
        minor_version: U2,
        major_version: U2,
        constant_pool_count: U2,
        constant_pool: Vec<CpInfo>,
        access_flags: U2,
        this_class: U2,
        super_class: U2,
        interfaces_count: U2,
        interfaces: Vec<U2>,
        fields_count: U2,
        fields: Vec<FieldInfo>,
        methods_count: U2,
        methods: Vec<MethodInfo>,
        attributes_count: U2,
        attributes: Vec<AttributeInfo>,
    ) -> Self {
        Self {
            magic,
            minor_version,
            major_version,
            constant_pool_count,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces_count,
            interfaces,
            fields_count,
            fields,
            methods_count,
            methods,
            attributes_count,
            attributes,
        }
    }

    pub fn get_class_name(&self) -> String {
        match self.constant_pool.get((self.this_class -1) as usize) {
            Some(CpInfo::Class { tag: _, name_index }) => {
                self.constant_pool.get_utf8_string(*name_index)
            }
            other => panic!("Wrong type {other:?}"),
        }
    }

    pub fn get_super_class_name(&self) -> String {
        match self.constant_pool.get((self.super_class -1 ) as usize) {
            Some(CpInfo::Class { tag: _, name_index }) => {
                self.constant_pool.get_utf8_string(*name_index)
            }
            other => panic!("Wrong type {other:?}"),
        }
    }
}

impl fmt::Display for ClassFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "magic: {:x}\nminor_version: {}\nmajor_version: {}\nconstant_pool_count: {}\nconstant_pool: {:?}\naccess_flags: {}\nthis_class: {}\nsuper_class: {}\ninterfaces_count: {}\ninterfaces: {:?}\nfields_count: {}\nfields: {:?}\nmethods_count: {}\nmethods: {:?}\nattributes_count: {}\nattributes: {:?}",
               self.magic, self.minor_version, self.major_version, self.constant_pool_count, self.constant_pool, self.access_flags, self.this_class, self.super_class, self.interfaces_count, self.interfaces, self.fields_count, self.fields, self.methods_count, self.methods, self.attributes_count, self.attributes)
    }
}
