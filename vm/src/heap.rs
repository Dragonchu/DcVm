use crate::class::{ArrayKlass, InstanceKlass, Klass, Value};
use bitfield_struct::bitfield;
use std::ptr::NonNull;

#[bitfield(u64)]
#[derive(PartialEq, Eq)]
struct Header {
    #[bits(2)]
    pub(crate) kind: ObjectKind,
    #[bits(8)]
    pub(crate) class_id: usize,

    #[bits(1)]
    pub(crate) state: GcState,

    #[bits(30)]
    identity_hash_code: i32,

    #[bits(23)]
    pub(crate) size: usize,
}

#[derive(Debug, Clone)]
pub struct Data {
    begin: NonNull<Value>,
    length: usize,
}

impl Data {
    pub fn write(&mut self, value: Value, index: usize) {
        unsafe { self.begin.offset(index as isize).write(value) }
    }

    pub fn read(&self, index: usize) -> Value {
        unsafe { self.begin.offset(index as isize).read() }
    }
}

#[derive(Clone, Debug)]
pub struct ObjPtr(NonNull<Object>);

impl ObjPtr {
    pub fn set_element(&mut self, value: Value, index: usize) {
        unsafe {
            self.0.as_mut().data.write(value, index);
        }
    }
}

impl Into<Value> for ObjPtr {
    fn into(self) -> Value {
        unsafe {
            match self.0.as_ref().head.kind() {
                ObjectKind::Base => self.0.as_ref().data.begin.as_ptr().read().into(),
                ObjectKind::Array | ObjectKind::Object => Value::Obj(self),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    head: Header,
    data: Data,
}

impl Object {
    pub fn set_element(&mut self, value: Value, index: usize) {
        self.data.write(value, index);
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ObjectKind {
    Base,
    Object,
    Array,
}
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) enum GcState {
    Unmarked,
    Marked,
}

// Needed for usage with bitfield
impl From<u64> for GcState {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Unmarked,
            1 => Self::Marked,
            _ => panic!("invalid value for GcState: {}", value),
        }
    }
}

impl From<GcState> for u64 {
    fn from(value: GcState) -> Self {
        value as u64
    }
}

// Needed for usage with bitfield
impl From<u64> for ObjectKind {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Object,
            1 => Self::Array,
            _ => panic!("invalid value for GcState: {}", value),
        }
    }
}

impl From<ObjectKind> for u64 {
    fn from(value: ObjectKind) -> Self {
        value as u64
    }
}

/// An allocation error type
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AllocError {
    /// Some attribute of the allocation, most likely the size requested,
    /// could not be fulfilled
    BadRequest,
    /// Out of memory - allocating the space failed
    OOM,
}

pub struct Heap;

impl Heap {
    pub fn new() -> Heap {
        Heap
    }
}

impl Heap {
    pub(crate) fn alloc(&self, klass: Klass) -> Result<ObjPtr, AllocError> {
        match klass {
            Klass::Instance(instance) => {
                todo!()
            }
            Klass::Array(_) => panic!(),
        }
    }

    pub(crate) fn alloc_array(&self, klass: Klass, length: usize) -> Result<ObjPtr, AllocError> {
        match klass {
            Klass::Instance(_) => {
                panic!()
            }
            Klass::Array(array) => {
                let field_cnt = array.dimension * length;
                todo!()
            }
        }
    }
}
