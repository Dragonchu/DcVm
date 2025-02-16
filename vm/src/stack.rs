use core::fmt;

use crate::class::Klass;
use crate::heap::RawPtr;
use crate::method::Method;
use crate::JvmValue;
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

#[derive(Debug, Clone)]
pub struct Frame {
    local_variables: Vec<JvmValue>,
    operand_stack: Vec<JvmValue>,
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
    pub fn push_value(&mut self, value: JvmValue) {
        self.operand_stack.push(value);
    }
}

pub struct Stack {
    frames: Vec<Frame>,
}
impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stack")
            .field("frames", &self.frames)
            .finish()
    }
}
impl Stack {
    pub fn new() -> Stack {
        Stack { frames: Vec::new() }
    }
    pub fn add_frame(
        &mut self,
        receiver: Option<RawPtr>,
        method: Method,
        class: Klass,
        args: Vec<RawPtr>,
    ) {
        let code = method.get_code();
        let max_locals = code.max_locals as usize;
        let max_stack = code.max_stack as usize;
        let mut locals: Vec<JvmValue> = receiver
            .into_iter()
            .chain(args.into_iter())
            .map(|obj| JvmValue::ObjRef(obj))
            .collect();
        while locals.len() < max_locals {
            locals.push(JvmValue::Null)
        }
        let frame = Frame {
            local_variables: locals.clone(),
            operand_stack: Vec::with_capacity(max_stack),
            method,
            class,
        };
        self.frames.push(frame);
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop();
    }

    pub fn cur_frame(&mut self) -> &mut Frame {
        self.frames.iter_mut().rev().next().expect("No more frame")
    }
}
