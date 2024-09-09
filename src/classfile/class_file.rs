use crate::types::{U1, U2, U4};

enum CpInfo {
    Class,
    Double,
    FieldRef,
    Float,
    Integer,
    InterfaceMethodRef,
    InvokeDynamic,
    Long,
    MethodHandle,
    MethodType,
    MethodRef,
    NameAndType,
    String,
    Utf8,
}

struct ConstantClassInfo {
    tag: U1,
    name_index: U2,
}

struct ConstantDoubleInfo {
    tag: U1,
    high_bytes: U4,
    low_bytes: U4,
}

struct ConstantFieldrefInfo {
    tag: U1,
    class_index: U2,
    name_and_type_index: U2,
}

struct ConstantFloatInfo {
    tag: U1,
    bytes: U4,
}

struct ConstantIntegerInfo {
    tag: U1,
    bytes: U4,
}

struct ConstantInterfaceMethodrefInfo {
    tag: U1,
    class_index: U2,
    name_and_type_index: U2,
}

struct ConstantInvokeDynamicInfo {
    tag: U1,
    bootstrap_method_attr_index: U2,
    name_and_type_index: U2,
}

struct ConstantLongInfo {
    tag: U1,
    high_bytes: U4,
    low_bytes: U4,
}

struct ConstantMethodHandleInfo {
    tag: U1,
    reference_kind: U1,
    reference_index: U2,
}

struct ConstantMethodTypeInfo {
    tag: U1,
    descriptor_index: U2,
}

struct ConstantMethodrefInfo {
    tag: U1,
    class_index: U2,
    name_and_type_index: U2,
}

struct ConstantNameAndTypeInfo {
    tag: U1,
    name_index: U2,
    descriptor_index: U2,
}

struct ConstantStringInfo {
    tag: U1,
    string_index: U2,
}

pub struct ClassFile {
    magic: U4,
    minor_version: U2,
    major_version: U2,
    constant_pool_count: U2,
    constant_pool: Vec<CpInfo>,
}
