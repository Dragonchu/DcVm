use core::fmt;
use std::cell::RefCell;

use typed_arena::Arena;

use crate::class::{Klass, Oop};
use crate::method::Method;
enum Variable {
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
pub struct Frame {
    local_variables: Vec<Oop>,
    operand_stack: Vec<Oop>,
    method: Method,
    class: Klass,
}
impl Frame {
    pub fn get_cur_method(&self) -> Method {
        self.method.clone()
    }
    pub fn get_cur_class(&self) -> Klass {
        self.class.clone()
    }
}

pub struct Stack<'a> {
    frames: RefCell<Vec<&'a Frame>>,
    allocator: Arena<Frame>,
}
impl fmt::Debug for Stack<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stack")
            .field("frames", &self.frames)
            .finish()
    }
}
impl<'a> Stack<'a> {
    pub fn new() -> Stack<'a> {
        Stack {
            frames: RefCell::new(Vec::new()),
            allocator: Arena::new(),
        }
    }
    pub fn add_frame(
        &'a self,
        receiver: Option<Oop>,
        method: Method,
        class: Klass,
        args: Vec<Oop>,
    ) {
        let code = method.get_code();
        let max_locals = code.max_locals as usize;
        let max_stack = code.max_stack as usize;
        let mut locals: Vec<Oop> = receiver.into_iter().chain(args.into_iter()).collect();
        while locals.len() < max_locals {
            locals.push(Oop::Uninitialized)
        }
        let frame = Frame {
            local_variables: locals.clone(),
            operand_stack: Vec::with_capacity(max_stack),
            method,
            class,
        };
        let frame_ref = self.allocator.alloc(frame);
        self.frames.borrow_mut().push(frame_ref)
    }

    pub fn pop_frame(&self) {
        self.frames.borrow_mut().pop();
    }

    pub fn cur_frame(&self) -> &Frame {
        self.frames
            .borrow()
            .iter()
            .rev()
            .next()
            .expect("No more frame")
    }
}
