use crate::{method::Method, runtime_constant_pool::RunTimeConstantPool};

enum Variable{
    Boolean(bool),
    Byte(i8),
    Char(u16),
    Short(i16),
    Int(i32),
    Float(f32),
    Reference,
    ReturnAddress(usize),
    Long(i64),
    Double(f64),
    Padding
}

struct Frame {
    local_variables: Vec<Variable>,
    operand_stack: Vec<String>,
    method: Method,
}

pub struct Stack{
    frames: Vec<Frame>
}