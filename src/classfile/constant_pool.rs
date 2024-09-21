use core::str;
use std::fmt;

use super::{types::{U1, U2, U4}};

#[derive(Debug, Clone)]
pub struct ConstantPool {
    pool: Vec<CpInfo>,
}

impl ConstantPool {
    pub fn new(pool: Vec<CpInfo>) -> Self {
        Self { pool }
    }

    pub fn get(&self, index: usize) -> &CpInfo {
        let cp_info = self.pool.get(index).expect("Invalid constant pool index");
        cp_info
    }

    pub fn get_utf8(&self, name_index: usize) -> String {
        let cp_info = self.get(name_index);
        if let CpInfo::Utf8 { tag: _, length: _, bytes } = cp_info {
            String::from_utf8_lossy(bytes).to_string()
        } else {
            panic!("Expected CpInfo::Utf8, found {:?}", cp_info);
        }
    }

    pub fn get_class_name(&self, class_info_index: usize) -> String {
        let cp_info = self.get(class_info_index);
        if let CpInfo::Class { tag: _, name_index } = cp_info {
            self.get_utf8(*name_index as usize)
        } else {
            panic!("Expected CpInfo::Class, found {:?}", cp_info);
        }
    }
}

#[derive(Clone)]
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

pub fn require_constant(pool: &Vec<CpInfo>, index: usize) -> &CpInfo {
    let cp_info = pool.get(index).expect("Invalid constant pool index");
    cp_info
}

pub fn require_constant_utf8(pool: &Vec<CpInfo>, index: usize) -> String {
    let cp_info = require_constant(pool, index);
    if let CpInfo::Utf8 { tag: _, length: _, bytes } = cp_info {
        String::from_utf8_lossy(bytes).to_string()
    } else {
        panic!("Expected CpInfo::Utf8, found {:?}", cp_info);
    }
}
