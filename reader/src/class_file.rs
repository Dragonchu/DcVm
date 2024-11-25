use std::fmt;

use crate::{constant_pool::CpInfo, field_info::FieldInfo, method_info::MethodInfo, types::{U1, U2, U4}};

use super::attribute_info::AttributeInfo;

#[derive(Debug)]
pub struct ClassFile {
    magic: U4,
    minor_version: U2,
    major_version: U2,
    constant_pool_count: U2,
    constant_pool: Vec<CpInfo>,
    pub access_flags: U2,
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
}

impl fmt::Display for ClassFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "magic: {:x}\nminor_version: {}\nmajor_version: {}\nconstant_pool_count: {}\nconstant_pool: {:?}\naccess_flags: {}\nthis_class: {}\nsuper_class: {}\ninterfaces_count: {}\ninterfaces: {:?}\nfields_count: {}\nfields: {:?}\nmethods_count: {}\nmethods: {:?}\nattributes_count: {}\nattributes: {:?}",
               self.magic, self.minor_version, self.major_version, self.constant_pool_count, self.constant_pool, self.access_flags, self.this_class, self.super_class, self.interfaces_count, self.interfaces, self.fields_count, self.fields, self.methods_count, self.methods, self.attributes_count, self.attributes)
    }
}
