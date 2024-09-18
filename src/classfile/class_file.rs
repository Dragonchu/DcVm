use core::str;
use std::fmt;

use crate::classfile::types::{U1, U2, U4};

use super::attribute_info::AttributeInfo;

pub enum CpInfo {
    Class {
        tag: U1,
        name_index: U2,
    },
    Double {
        tag: U1,
        high_bytes: U4,
        low_bytes: U4,
    },
    FieldRef {
        tag: U1,
        class_index: U2,
        name_and_type_index: U2,
    },
    Float {
        tag: U1,
        bytes: U4,
    },
    Integer {
        tag: U1,
        bytes: U4,
    },
    InterfaceMethodRef {
        tag: U1,
        class_index: U2,
        name_and_type_index: U2,
    },
    InvokeDynamic {
        tag: U1,
        bootstrap_method_attr_index: U2,
        name_and_type_index: U2,
    },
    Long {
        tag: U1,
        high_bytes: U4,
        low_bytes: U4,
    },
    MethodHandle {
        tag: U1,
        reference_kind: U1,
        reference_index: U2,
    },
    MethodType {
        tag: U1,
        descriptor_index: U2,
    },
    MethodRef {
        tag: U1,
        class_index: U2,
        name_and_type_index: U2,
    },
    NameAndType {
        tag: U1,
        name_index: U2,
        descriptor_index: U2,
    },
    String {
        tag: U1,
        string_index: U2,
    },
    Utf8 {
        tag: U1,
        length: U2,
        bytes: Vec<U1>,
    },
    Padding,
}

impl fmt::Debug for CpInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CpInfo::Class { tag, name_index } => write!(f, "\n  Class{{tag {}, name_index: {}}}", tag, name_index),
            CpInfo::Double { tag, high_bytes, low_bytes } => write!(f, "\n  Double{{tag: {}, high_bytes: {}, low_bytes: {}}}", tag, high_bytes, low_bytes),
            CpInfo::FieldRef { tag, class_index, name_and_type_index } => write!(f, "\n  FieldRef{{ tag: {}, class_index: {}, name_and_type_index: {}}}", tag, class_index, name_and_type_index),
            CpInfo::Float { tag, bytes } => write!(f, "\n  Float{{tag: {}, bytes: {}}}", tag, bytes),
            CpInfo::Integer { tag, bytes } => write!(f, "\n  Integer{{tag: {}, bytes: {}}}", tag, bytes),
            CpInfo::InterfaceMethodRef { tag, class_index, name_and_type_index } => write!(f, "\n  InterfaceMethodRef{{tag: {}, class_index: {}, name_and_type_index: {}}}", tag, class_index, name_and_type_index),
            CpInfo::InvokeDynamic { tag, bootstrap_method_attr_index, name_and_type_index } => write!(f, "\n  InvokeDynamic{{tag: {}, bootstrap_method_attr_index: {}, name_and_type_index: {}}}", tag, bootstrap_method_attr_index, name_and_type_index),
            CpInfo::Long { tag, high_bytes, low_bytes } => write!(f, "\n  Long{{tag: {}, high_bytes: {}, low_bytes: {}}}", tag, high_bytes, low_bytes),
            CpInfo::MethodHandle { tag, reference_kind, reference_index } => write!(f, "\n  MethodHandle{{tag: {}, reference_kind: {}, reference_index: {}}}", tag, reference_kind, reference_index),
            CpInfo::MethodType { tag, descriptor_index } => write!(f, "\n  MethodType{{tag: {}, descriptor_index: {}}}", tag, descriptor_index),
            CpInfo::MethodRef { tag, class_index, name_and_type_index } => write!(f, "\n  MethodRef{{tag: {}, class_index: {}, name_and_type_index: {}}}", tag, class_index, name_and_type_index),
            CpInfo::NameAndType { tag, name_index, descriptor_index } => write!(f, "\n  NameAndType{{tag: {}, name_index: {}, descriptor_index: {}}}", tag, name_index, descriptor_index),
            CpInfo::String { tag, string_index } => write!(f, "\n  String{{tag: {}, string_index: {}}}", tag, string_index),
            CpInfo::Utf8 { tag, length, bytes } => write!(f, "\n  Utf8{{tag: {}, length: {}, bytes: {:?}}}", tag, length, str::from_utf8(bytes).unwrap()),
            CpInfo::Padding => write!(f, "\n  Padding"),
        }
    }
}

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
    pub constant_pool: Vec<CpInfo>,
    pub access_flags: U2,
    pub this_class: U2,
    pub super_class: U2,
    interfaces_count: U2,
    interfaces: Vec<U2>,
    pub fields_count: U2,
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
