#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use crate::heap::RawPtr;
use reader::types::{U1, U2, U4, U8};

pub mod class;
mod class_loader;
mod field;
pub mod heap;
mod instructions;
pub mod jvm_thread;
mod method;
mod native_method;
mod pc_register;
mod runtime_constant_pool;
mod stack;
pub mod vm;

#[derive(Debug, Clone, Copy)]
pub enum JvmValue {
    Boolean(U1),
    Byte(U1),
    Short(U2),
    Int(U4),
    Long(U8),
    Float(U8),
    Double(U8),
    Char(U2),
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
    
    value_as!(as_bool: Boolean(U1));
    value_as!(as_byte: Byte(U1));
    value_as!(as_short: Short(U2));
    value_as!(as_int: Int(U4));
    value_as!(as_long: Long(U8));
    value_as!(as_float: Float(U8));
    value_as!(as_double: Double(U8));
    value_as!(as_char: Char(U2));
    value_as!(as_obj_ref: ObjRef(RawPtr));
}
