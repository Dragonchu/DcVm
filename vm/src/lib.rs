#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use crate::heap::RawPtr;

pub mod class;
pub mod class_loader;
pub mod error;
pub mod field;
pub mod heap;
pub mod instructions;
pub mod jvm_thread;
pub mod method;
pub mod native_method;
pub mod pc_register;
pub mod runtime_constant_pool;
pub mod stack;
pub mod vm;
pub mod operand_stack;
pub mod local_vars;

#[derive(Debug, Clone, Copy)]
pub enum JvmValue {
    Boolean(u8),
    Byte(u8),
    Short(u16),
    Int(u32),
    Long(u64),
    Float(u64),
    Double(u64),
    Char(u16),
    ObjRef(RawPtr),
    Null,
}

macro_rules! value_as {
    ($name:ident : $ctor:ident($ty:ty)) => {
        pub fn $name(self) -> Option<$ty> {
            match self {
                Self::$ctor(v) => Some(v),
                _ => None,
            }
        }
    };
}

impl JvmValue {
    pub fn name(&self) -> char {
        match *self {
            JvmValue::Boolean(_) => 'Z',
            JvmValue::Byte(_) => 'B',
            JvmValue::Short(_) => 'S',
            JvmValue::Int(_) => 'I',
            JvmValue::Long(_) => 'L',
            JvmValue::Float(_) => 'F',
            JvmValue::Double(_) => 'D',
            JvmValue::Char(_) => 'C',
            JvmValue::ObjRef(_) | JvmValue::Null => 'A',
        }
    }

    pub fn default_value(letter: char) -> JvmValue {
        match letter {
            'Z' => JvmValue::Boolean(0),
            'B' => JvmValue::Byte(0),
            'S' => JvmValue::Short(0),
            'I' => JvmValue::Int(0),
            'L' => JvmValue::Long(0),
            'F' => JvmValue::Float(0),
            'D' => JvmValue::Double(0),
            'C' => JvmValue::Char(0),
            'A' => JvmValue::Null,
            _ => panic!("Illegal type {} seen when trying to parse", letter),
        }
    }
    
    value_as!(as_bool: Boolean(u8));
    value_as!(as_byte: Byte(u8));
    value_as!(as_short: Short(u16));
    value_as!(as_int: Int(u32));
    value_as!(as_long: Long(u64));
    value_as!(as_float: Float(u64));
    value_as!(as_double: Double(u64));
    value_as!(as_char: Char(u16));
    value_as!(as_obj_ref: ObjRef(RawPtr));
}
