use core::str;
use std::{fmt, rc::Rc};

use crate::classfile::types::{U1, U2, U4};

use super::{attribute_info::AttributeInfo, constant_pool::ConstantPool};



pub enum ConstantInfoTag {
    ConstantUtf8 = 1,
    ConstantInteger = 3,
    ConstantFloat = 4,
    ConstantLong = 5,
    ConstantDouble = 6,
    ConstantClass = 7,
    ConstantString = 8,
    ConstantFieldref = 9,
    ConstantMethodref = 10,
    ConstantInterfaceMethodref = 11,
    ConstantNameAndType = 12,
    ConstantMethodHandle = 15,
    ConstantMethodType = 16,
    ConstantInvokeDynamic = 18,
}

impl TryFrom<u8> for ConstantInfoTag {
    type Error = ();

    fn try_from(value: U1) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ConstantInfoTag::ConstantUtf8),
            3 => Ok(ConstantInfoTag::ConstantInteger),
            4 => Ok(ConstantInfoTag::ConstantFloat),
            5 => Ok(ConstantInfoTag::ConstantLong),
            6 => Ok(ConstantInfoTag::ConstantDouble),
            7 => Ok(ConstantInfoTag::ConstantClass),
            8 => Ok(ConstantInfoTag::ConstantString),
            9 => Ok(ConstantInfoTag::ConstantFieldref),
            10 => Ok(ConstantInfoTag::ConstantMethodref),
            11 => Ok(ConstantInfoTag::ConstantInterfaceMethodref),
            12 => Ok(ConstantInfoTag::ConstantNameAndType),
            15 => Ok(ConstantInfoTag::ConstantMethodHandle),
            16 => Ok(ConstantInfoTag::ConstantMethodType),
            18 => Ok(ConstantInfoTag::ConstantInvokeDynamic),
            _ => {
                println!("Unknown tag: {}", value);
                Err(())
            }
        }
    }
}

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
}

pub struct MethodInfo {
    access_flags: U2,
    name_index: U2,
    descriptor_index: U2,
    attributes_count: U2,
    attributes: Vec<AttributeInfo>,
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

#[derive(Debug)]
pub struct ClassFile {
    magic: U4,
    minor_version: U2,
    major_version: U2,
    constant_pool_count: U2,
    pub constant_pool: ConstantPool,
    pub access_flags: U2,
    pub this_class: U2,
    pub super_class: U2,
    interfaces_count: U2,
    interfaces: Vec<U2>,
    pub fields_count: U2,
    pub fields: Vec<Rc<FieldInfo>>,
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
        constant_pool: ConstantPool,
        access_flags: U2,
        this_class: U2,
        super_class: U2,
        interfaces_count: U2,
        interfaces: Vec<U2>,
        fields_count: U2,
        fields: Vec<Rc<FieldInfo>>,
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
