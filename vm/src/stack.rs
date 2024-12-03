use typed_arena::Arena;

use crate::{method::Method, runtime_constant_pool::RunTimeConstantPool};
use crate::class::Oop;
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
    Padding,
    Uninitialized,
}

struct Frame<'memory> {
    local_variables: Vec<Oop<'memory>>,
    operand_stack: Vec<String>,
    method: Method,
}

pub struct Stack<'memory>{
    frames: Vec<&'memory Frame<'memory>>,
    allocator: Arena<Frame<'memory>>,
}
impl<'memory> Stack<'memory> {
    pub fn add_frame(&'memory mut self, receiver: Option<Oop<'memory>>, method: Method, args: Vec<Oop<'memory>>) {
        let code = method.get_code();
        let max_locals = code.max_locals as usize;
        let max_stack = code.max_stack as usize; 
        let mut locals: Vec<Oop<'_>> = receiver.into_iter().chain(args.into_iter()).collect();
        while locals.len() < max_locals{
            locals.push(Oop::Uninitialized)
        }
        let frame = Frame {
            local_variables: locals.clone(),
            operand_stack: Vec::with_capacity(max_stack),
            method
        };
        let frame_ref = self.allocator.alloc(frame);
        self.frames.push(frame_ref)
    }
}
