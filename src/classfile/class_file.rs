use crate::classfile::types::{U1, U2, U4};

pub enum CpInfo {
    Class(ConstantClassInfo),
    Double(ConstantDoubleInfo),
    FieldRef(ConstantFieldrefInfo),
    Float(ConstantFloatInfo),
    Integer(ConstantIntegerInfo),
    InterfaceMethodRef(ConstantInterfaceMethodrefInfo),
    InvokeDynamic(ConstantInvokeDynamicInfo),
    Long(ConstantLongInfo),
    MethodHandle(ConstantMethodHandleInfo),
    MethodType(ConstantMethodTypeInfo),
    MethodRef(ConstantMethodrefInfo),
    NameAndType(ConstantNameAndTypeInfo),
    String(ConstantStringInfo),
    Utf8(ConstantUtf8Info),
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
            _ => Err(()),
        }
    }
}

//The CONSTANT_Class_info Structure
pub struct ConstantClassInfo {
    tag: U1,
    name_index: U2,
}

impl ConstantClassInfo {
    pub fn new(name_index: U2) -> Self {
        Self {
            tag: ConstantInfoTag::ConstantClass as U1,
            name_index,
        }
    }
}

//The CONSTANT_Fieldref_info, CONSTANT_Methodref_info, and CONSTANT_InterfaceMethodref_info Structures
pub struct ConstantFieldrefInfo {
    tag: U1,
    class_index: U2,
    name_and_type_index: U2,
}

impl ConstantFieldrefInfo {
    pub fn new(class_index: U2, name_and_type_index: U2) -> Self {
        Self {
            tag: ConstantInfoTag::ConstantFieldref as U1,
            class_index,
            name_and_type_index,
        }
    }
}

pub struct ConstantMethodrefInfo {
    tag: U1,
    class_index: U2,
    name_and_type_index: U2,
}

impl ConstantMethodrefInfo {
    pub fn new(class_index: U2, name_and_type_index: U2) -> Self {
        Self {
            tag: ConstantInfoTag::ConstantMethodref as U1,
            class_index,
            name_and_type_index,
        }
    }
}

pub struct ConstantInterfaceMethodrefInfo {
    tag: U1,
    class_index: U2,
    name_and_type_index: U2,
}

impl ConstantInterfaceMethodrefInfo {
    pub fn new(class_index: U2, name_and_type_index: U2) -> Self {
        Self {
            tag: ConstantInfoTag::ConstantInterfaceMethodref as U1,
            class_index,
            name_and_type_index,
        }
    }
}

//The CONSTANT_String_info Structure
pub struct ConstantStringInfo {
    tag: U1,
    string_index: U2,
}

impl ConstantStringInfo {
    pub fn new(string_index: U2) -> Self {
        Self {
            tag: ConstantInfoTag::ConstantString as U1,
            string_index,
        }
    }
}

//The CONSTANT_Integer_info and CONSTANT_Float_info Structures
pub struct ConstantIntegerInfo {
    tag: U1,
    bytes: U4,
}

impl ConstantIntegerInfo {
    pub fn new(bytes: U4) -> Self {
        Self {
            tag: ConstantInfoTag::ConstantInteger as U1,
            bytes,
        }
    }
}

pub struct ConstantFloatInfo {
    tag: U1,
    bytes: U4,
}

impl ConstantFloatInfo {
    pub fn new(bytes: U4) -> Self {
        Self {
            tag: ConstantInfoTag::ConstantFloat as U1,
            bytes,
        }
    }
}


//The CONSTANT_Long_info and CONSTANT_Double_info Structures
pub struct ConstantLongInfo {
    tag: U1,
    high_bytes: U4,
    low_bytes: U4,
}

impl ConstantLongInfo {
    pub fn new(high_bytes: U4, low_bytes: U4) -> Self {
        Self {
            tag: ConstantInfoTag::ConstantLong as U1,
            high_bytes,
            low_bytes,
        }
    }
}

pub struct ConstantDoubleInfo {
    tag: U1,
    high_bytes: U4,
    low_bytes: U4,
}

impl ConstantDoubleInfo {
    pub fn new(high_bytes: U4, low_bytes: U4) -> Self {
        Self {
            tag: ConstantInfoTag::ConstantDouble as U1,
            high_bytes,
            low_bytes,
        }
    }
}

//The CONSTANT_NameAndType_info Structure
pub struct ConstantNameAndTypeInfo {
    tag: U1,
    name_index: U2,
    descriptor_index: U2,
}

impl ConstantNameAndTypeInfo {
    pub fn new(name_index: U2, descriptor_index: U2) -> Self {
        Self {
            tag: ConstantInfoTag::ConstantNameAndType as U1,
            name_index,
            descriptor_index,
        }
    }
}

//The CONSTANT_Utf8_info Structure
pub struct ConstantUtf8Info {
    tag: U1,
    length: U2,
    bytes: Vec<U1>,
}

impl ConstantUtf8Info {
    pub fn new(length: U2, bytes: Vec<U1>) -> Self {
        Self {
            tag: ConstantInfoTag::ConstantUtf8 as U1,
            length,
            bytes,
        }
    }
}

//The CONSTANT_MethodHandle_info Structure
pub struct ConstantMethodHandleInfo {
    tag: U1,
    reference_kind: U1,
    reference_index: U2,
}

impl ConstantMethodHandleInfo {
    pub fn new(reference_kind: U1, reference_index: U2) -> Self {
        Self {
            tag: ConstantInfoTag::ConstantMethodHandle as U1,
            reference_kind,
            reference_index,
        }
    }
}

//The CONSTANT_MethodType_info Structure
pub struct ConstantMethodTypeInfo {
    tag: U1,
    descriptor_index: U2,
}

impl ConstantMethodTypeInfo {
    pub fn new(descriptor_index: U2) -> Self {
        Self {
            tag: ConstantInfoTag::ConstantMethodType as U1,
            descriptor_index,
        }
    }
}

//The CONSTANT_InvokeDynamic_info Structure
pub struct ConstantInvokeDynamicInfo {
    tag: U1,
    bootstrap_method_attr_index: U2,
    name_and_type_index: U2,
}

impl ConstantInvokeDynamicInfo {
    pub fn new(bootstrap_method_attr_index: U2, name_and_type_index: U2) -> Self {
        Self {
            tag: ConstantInfoTag::ConstantInvokeDynamic as U1,
            bootstrap_method_attr_index,
            name_and_type_index,
        }
    }
}

pub struct AttributeInfo {
    attribute_name_index: U2,
    attribute_length: U4,
    info: Vec<U1>,
}

impl AttributeInfo {
    pub fn new(attribute_name_index: U2, attribute_length: U4, info: Vec<U1>) -> Self {
        Self {
            attribute_name_index,
            attribute_length,
            info,
        }
    }
}

pub struct FieldInfo {
    access_flags: U2,
    name_index: U2,
    descriptor_index: U2,
    attributes_count: U2,
    attributes: Vec<AttributeInfo>,
}

impl FieldInfo {
    pub fn new(access_flags: U2, name_index: U2, descriptor_index: U2, attributes: Vec<AttributeInfo>) -> Self {
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
    pub fn new(access_flags: U2, name_index: U2, descriptor_index: U2, attributes: Vec<AttributeInfo>) -> Self {
        Self {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count: attributes.len() as U2,
            attributes,
        }
    }
}

pub struct ClassFile {
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
}

impl ClassFile {
    pub fn new(magic: U4, minor_version: U2, major_version: U2, constant_pool_count: U2, constant_pool: Vec<CpInfo>, access_flags: U2, this_class: U2, super_class: U2, interfaces_count: U2, interfaces: Vec<U2>, fields_count: U2, fields: Vec<FieldInfo>, methods_count: U2, methods: Vec<MethodInfo>, attributes_count: U2, attributes: Vec<AttributeInfo>) -> Self {
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
