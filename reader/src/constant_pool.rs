use core::str;
use std::fmt;

use crate::types::{U1, U2, U4};

impl CpInfo {
    pub fn to_utf8_string(&self) -> String {
        if let CpInfo::Utf8 { tag:_, length:_, bytes } = self {
           str::from_utf8(bytes).unwrap().to_string()
        } else {
            panic!("wrong type: {:?}", self)
        }
    }
}
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

pub trait ConstantPool {
    fn get_utf8_string(&self, index: U2) -> String;
    fn get_field_info(&self, field_index: U2) -> (String,String, String);
}

impl ConstantPool for Vec<CpInfo> {
    fn get_utf8_string(&self, index: U2) -> String {
        let cp_info = self.get((index - 1) as usize).expect("Unknow string");
        cp_info.to_utf8_string()
    }
    fn get_field_info(&self, field_index: U2) -> (String,String, String){
        if let CpInfo::FieldRef { tag, class_index, name_and_type_index } = self.get((field_index-1) as usize).expect("Unknow field") {
            if let CpInfo::Class { tag, name_index: class_name_index } = self.get((class_index -1) as usize).expect("Unknown class") {
                if let CpInfo::NameAndType { tag, name_index, descriptor_index } = self.get((name_and_type_index-1) as usize).expect("Unknow name and type") {
                    (self.get_utf8_string(class_name_index.clone()),self.get_utf8_string(name_index.clone()), self.get_utf8_string(descriptor_index.clone()))
                }else {
                    panic!("Wrong type")
                }
            }else {
                panic!("wrong type")
            }
        }else {
            panic!("Wrong type")
        }
    }
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


