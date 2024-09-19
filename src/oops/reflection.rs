use std::{cell::RefCell, rc::Rc};

use crate::common::ValueType;

use super::field::Field;

pub(crate) fn primitive_type_to_value_type_no_wrap(c: char) -> ValueType {
    match c {
        'B' => ValueType::Byte,
        'C' => ValueType::Char,
        'D' => ValueType::Double,
        'F' => ValueType::Float,
        'I' => ValueType::Int,
        'J' => ValueType::Long,
        'S' => ValueType::Short,
        'V' => ValueType::Void,
        'Z' => ValueType::Boolean,
        _ => panic!("Invalid primitive type: {}", c),
    }
}

#[derive(Debug, Clone)]
pub struct FieldId{
    pub offset: usize,
    pub field: Rc<RefCell<Field>>,
}