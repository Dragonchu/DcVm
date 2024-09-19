use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use crate::classfile::types::U2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Object,
    Array,
}

pub const ACC_PUBLIC: u16 = 0x0001;
pub const ACC_PRIVATE: u16 = 0x0002;
pub const ACC_PROTECTED: u16 = 0x0004;
pub const ACC_STATIC: u16 = 0x0008;
pub const ACC_FINAL: u16 = 0x0010;
pub const ACC_SYNCHRONIZED: u16 = 0x0020;
pub const ACC_SUPER: u16 = 0x0020;
pub const ACC_VOLATILE: u16 = 0x0040;
pub const ACC_BRIDGE: u16 = 0x0040;
pub const ACC_VARARGS: u16 = 0x0080;
pub const ACC_TRANSIENT: u16 = 0x0080;
pub const ACC_NATIVE: u16 = 0x0100;
pub const ACC_INTERFACE: u16 = 0x0200;
pub const ACC_ABSTRACT: u16 = 0x0400;
pub const ACC_STRICT: u16 = 0x0800;
pub const ACC_SYNTHETIC: u16 = 0x1000;
pub const ACC_ANNOTATION: u16 = 0x2000;
pub const ACC_ENUM: u16 = 0x4000;
pub const ACC_MIRANDA: u16 = 0x8000;
pub const ACC_REFLECT_MASK: u16 = 0xFFFF;

pub const ATTRIBUTE_CONSTANT_VALUE: U2 = 0;
pub const ATTRIBUTE_CODE: U2 = 1;
pub const ATTRIBUTE_STACK_MAP_TABLE: U2 = 2;
pub const ATTRIBUTE_EXCEPTIONS: U2 = 3;
pub const ATTRIBUTE_INNER_CLASSES: U2 = 4;
pub const ATTRIBUTE_ENCLOSING_METHOD: U2 = 5;
pub const ATTRIBUTE_SYNTHETIC: U2 = 6;
pub const ATTRIBUTE_SIGNATURE: U2 = 7;
pub const ATTRIBUTE_SOURCE_FILE: U2 = 8;
pub const ATTRIBUTE_SOURCE_DEBUG_EXTENSION: U2 = 9;
pub const ATTRIBUTE_LINE_NUMBER_TABLE: U2 = 10;
pub const ATTRIBUTE_LOCAL_VARIABLE_TABLE: U2 = 11;
pub const ATTRIBUTE_LOCAL_VARIABLE_TYPE_TABLE: U2 = 12;
pub const ATTRIBUTE_DEPRECATED: U2 = 13;
pub const ATTRIBUTE_RUNTIME_VISIBLE_ANNOTATIONS: U2 = 14;
pub const ATTRIBUTE_RUNTIME_INVISIBLE_ANNOTATIONS: U2 = 15;
pub const ATTRIBUTE_RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS: U2 = 16;
pub const ATTRIBUTE_RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS: U2 = 17;
pub const ATTRIBUTE_RUNTIME_VISIBLE_TYPE_ANNOTATIONS: U2 = 18;
pub const ATTRIBUTE_RUNTIME_INVISIBLE_TYPE_ANNOTATIONS: U2 = 19;
pub const ATTRIBUTE_ANNOTATION_DEFAULT: U2 = 20;
pub const ATTRIBUTE_BOOTSTRAP_METHODS: U2 = 21;
pub const ATTRIBUTE_METHOD_PARAMETERS: U2 = 22;

pub static ATTRIBUTE_MAPPING: LazyLock<Mutex<HashMap<&str, U2>>> = LazyLock::new(|| {
    Mutex::new(HashMap::from([
        ("ConstantValue", ATTRIBUTE_CONSTANT_VALUE),
        ("Code", ATTRIBUTE_CODE),
        ("StackMapTable", ATTRIBUTE_STACK_MAP_TABLE),
        ("Exceptions", ATTRIBUTE_EXCEPTIONS),
        ("InnerClasses", ATTRIBUTE_INNER_CLASSES),
        ("EnclosingMethod", ATTRIBUTE_ENCLOSING_METHOD),
        ("Synthetic", ATTRIBUTE_SYNTHETIC),
        ("Signature", ATTRIBUTE_SIGNATURE),
        ("SourceFile", ATTRIBUTE_SOURCE_FILE),
        ("SourceDebugExtension", ATTRIBUTE_SOURCE_DEBUG_EXTENSION),
        ("LineNumberTable", ATTRIBUTE_LINE_NUMBER_TABLE),
        ("LocalVariableTable", ATTRIBUTE_LOCAL_VARIABLE_TABLE),
        (
            "LocalVariableTypeTable",
            ATTRIBUTE_LOCAL_VARIABLE_TYPE_TABLE,
        ),
        ("Deprecated", ATTRIBUTE_DEPRECATED),
        (
            "RuntimeVisibleAnnotations",
            ATTRIBUTE_RUNTIME_VISIBLE_ANNOTATIONS,
        ),
        (
            "RuntimeInvisibleAnnotations",
            ATTRIBUTE_RUNTIME_INVISIBLE_ANNOTATIONS,
        ),
        (
            "RuntimeVisibleParameterAnnotations",
            ATTRIBUTE_RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS,
        ),
        (
            "RuntimeInvisibleParameterAnnotations",
            ATTRIBUTE_RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS,
        ),
        (
            "RuntimeVisibleTypeAnnotations",
            ATTRIBUTE_RUNTIME_VISIBLE_TYPE_ANNOTATIONS,
        ),
        (
            "RuntimeInvisibleTypeAnnotations",
            ATTRIBUTE_RUNTIME_INVISIBLE_TYPE_ANNOTATIONS,
        ),
        ("AnnotationDefault", ATTRIBUTE_ANNOTATION_DEFAULT),
        ("BootstrapMethods", ATTRIBUTE_BOOTSTRAP_METHODS),
        ("MethodParameters", ATTRIBUTE_METHOD_PARAMETERS),
    ]))
});
