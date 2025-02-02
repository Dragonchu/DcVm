use crate::class::{Klass, Value};
use bitfield_struct::bitfield;
use std::ptr::NonNull;

#[bitfield(u64)]
#[derive(PartialEq, Eq)]
struct RawObjHeader {
    #[bits(10)]
    pub(crate) class_id: usize,
    
    #[bits(1)]
    pub(crate) state: GcState,

    #[bits(30)]
    identity_hash_code: i32,

    #[bits(23)]
    pub(crate) size: usize,
}


#[derive(Clone, Copy, Debug)]
pub struct ObjectPtr(NonNull<Value>);

#[derive(Debug, Clone)]
pub struct RawObject {
    head: RawObjHeader,
    fields_begin: ObjectPtr,
}

// impl ObjectPtr {
//     pub fn get_field(&self, klass: &Klass, index: U2) -> NonNull<Value> {
//         let field = klass.get_field_info(index);
//         let desc = field.get_descriptor();
//     }
// 
//     fn read(&self, index: U2, desc: &str) -> NonNull<Value> {
//         
//     }
// }

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ObjectKind {
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

pub trait AllocRaw {
    fn alloc<T>(&self, klass: Klass) -> Result<ObjectPtr, AllocError>;

    /// Allocating an array allows the client to put anything in the resulting data
    /// block but the type of the memory block will simply be 'Array'. No other
    /// type information will be stored in the object header.
    /// This is just a special case of alloc<T>() for T=u8 but a count > 1 of u8
    /// instances.  The caller is responsible for the content of the array.
    fn alloc_array(&self, size_bytes: usize) -> Result<ObjectPtr, AllocError>;
}

pub struct Heap;

impl Heap {
    pub fn new() -> Heap {
        Heap
    }
}

impl AllocRaw for Heap {
    fn alloc<T>(&self, klass: Klass) -> Result<ObjectPtr, AllocError> {
        todo!()
    }

    fn alloc_array(&self, size_bytes: usize) -> Result<ObjectPtr, AllocError> {
        todo!()
    }
}
