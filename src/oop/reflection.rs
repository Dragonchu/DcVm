use crate::common::types::ValueType;

pub fn primitive_type_to_value_type_no_wrap(c: char) -> ValueType {
    match c {
        'B' => ValueType::Byte,
        'C' => ValueType::Char,
        'D' => ValueType::Double,
        'F' => ValueType::Float,
        'I' => ValueType::Int,
        'J' => ValueType::Long,
        'S' => ValueType::Short,
        'Z' => ValueType::Boolean,
        'V' => ValueType::Void,
        _ => panic!("Unknown primitive type: {}", c),
    }
}
