use core::fmt;
use std::cell::RefCell;

use typed_arena::Arena;

use crate::{method::Method, runtime_constant_pool::RunTimeConstantPool};
use crate::class::{InstanceKlassRef, Oop};
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

#[derive(Debug)]
pub struct Frame<'memory> {
    local_variables: Vec<Oop<'memory>>,
    operand_stack: Vec<Oop<'memory>>,
    method: Method,
    class: InstanceKlassRef<'memory>
}
impl<'memory> Frame<'memory> {
    pub fn get_cur_method(&self) -> Method {
        self.method.clone()
    }
    pub fn get_cur_class(&self) -> InstanceKlassRef<'_>{
        self.class
    }
}

pub struct Stack<'memory>{
    frames: RefCell<Vec<&'memory Frame<'memory>>>,
    allocator: Arena<Frame<'memory>>,
}
impl<'memory> fmt::Debug for Stack<'memory> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stack")
        .field("frames", &self.frames)
        .finish()
    }
}
impl<'memory> Stack<'memory> {
    pub fn new() -> Stack<'memory>{
        Stack {
            frames: RefCell::new(Vec::new()),
            allocator: Arena::new()
        }
    }
    pub fn add_frame(&'memory self, receiver: Option<Oop<'memory>>, method: Method, class: InstanceKlassRef<'memory>, args: Vec<Oop<'memory>>) {
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
            method,
            class
        };
        let frame_ref = self.allocator.alloc(frame);
        self.frames.borrow_mut().push(frame_ref)
    }

    pub fn pop_frame(&self) {
        self.frames.borrow_mut().pop();
    }

    pub fn cur_frame(&self) -> &Frame{
        self.frames.borrow().iter().rev().next().expect("No more frame")
    }
}
