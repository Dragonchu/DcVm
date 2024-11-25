use crate::runtime_constant_pool::RunTimeConstantPool;

enum Variable{
    Boolean(bool),
    Byte(i8),
    Char(u16),
    Short(i16),
    Int(i32),
    Float(f32),
    Reference,
    ReturnAddress(*const u8),
    Long(i64),
    Double(f64)
}

struct Frame<'rtcp> {
    local_variables: Vec<Variable>,
    operand_stack: Vec<String>,
    run_time_constant_pool: &'rtcp RunTimeConstantPool 
}

pub struct Stack<'rtcp>{
    frames: Vec<Frame<'rtcp>>
}