use std::alloc::Layout;
use std::cell::Cell;
use std::fmt;
use std::fmt::Formatter;
use crate::class::{ArrayKlass, InstanceKlass, Klass, Value};
use bitfield_struct::bitfield;
use std::ptr::NonNull;
use log::debug;

#[bitfield(u64)]
#[derive(PartialEq, Eq)]
struct Header {
    #[bits(10)]
    pub(crate) class_id: usize,

    #[bits(1)]
    pub(crate) state: GcState,

    #[bits(30)]
    identity_hash_code: i32,

    #[bits(23)]
    pub(crate) size: usize,
}

#[derive(Clone, Debug)]
pub struct Oop(*mut u8);

impl Oop {
    pub fn set_element(&mut self, value: Value, index: usize) {
        unsafe {
            let ptr = self.0.add(index);
            match value {
                Value::Int(int) => std::ptr::write(ptr as *mut i32, int),
                Value::Long(long) => std::ptr::write(ptr as *mut i64, long),
                Value::Float(float) => std::ptr::write(ptr as *mut f32, float),
                Value::Double(double) => std::ptr::write(ptr as *mut f64, double),
                Value::Uninitialized | Value::Null => std::ptr::write(ptr as *mut u64, 0),
                Value::Obj(oop) => std::ptr::write(ptr as *mut Oop, oop),
            }
        }
    }
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

/// An allocation error type
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AllocError {
    /// Some attribute of the allocation, most likely the size requested,
    /// could not be fulfilled
    BadRequest,
    /// Out of memory - allocating the space failed
    OOM,
}

/// An allocation on our memory chunk
pub struct AllocEntry {
    pub(crate) ptr: *mut u8,
    pub(crate) alloc_size: usize,
}

/// Models an allocated chunk of memory
struct MemoryChunk {
    memory: *mut u8,
    used: usize,
    capacity: usize,
}

impl fmt::Debug for MemoryChunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{address={:#0x}, used={}, capacity={}}}",
            self.memory as u64, self.used, self.capacity
        )
    }
}

impl MemoryChunk {
    fn new(capacity: usize) -> Self {
        let layout = Layout::from_size_align(capacity, 8).unwrap();
        let ptr = unsafe { std::alloc::alloc_zeroed(layout) };
        debug!(
            "allocated memory chunk of size {} at {:#0x}",
            capacity, ptr as u64
        );

        MemoryChunk {
            memory: ptr,
            capacity,
            used: 0,
        }
    }

    /// Allocates from the chunk, or returns None if there is not enough space
    fn alloc(&mut self, required_size: usize) -> Option<Oop> {
        if self.used + required_size > self.capacity {
            return None;
        }

        // We require all allocations to be aligned to 8 bytes!
        assert_eq!(required_size % 8, 0);

        let ptr = unsafe { self.memory.add(self.used) };
        self.used += required_size;

        Some(Oop(ptr)) 
    }

    unsafe fn contains(&self, ptr: *const u8) -> bool {
        ptr >= self.memory && ptr <= self.memory.add(self.used)
    }

    fn reset(&mut self) {
        self.used = 0;

        // Zero the memory, to attempt and catch bugs
        unsafe {
            std::ptr::write_bytes(self.memory, 0, self.capacity);
        }
    }
}

pub struct Heap{
    cur: MemoryChunk,
    nxt: MemoryChunk,
}

impl Heap {
    pub fn with_maximum_memory(max_size: usize) -> Self {
        let semi_space_capacity = max_size / 2;
        Self {
            cur: MemoryChunk::new(semi_space_capacity),
            nxt: MemoryChunk::new(semi_space_capacity),
        }
    }
}

impl Heap {
    pub(crate) fn alloc(&mut self, klass: &Klass) -> Result<Oop, AllocError> {
        match klass {
            Klass::Instance(instance) => {
                let header_size = size_of::<Header>();
                let required_size = header_size + klass.get_instance_field_cnt();
                self.cur.alloc(Self::align_to_8_bytes(required_size))
                    .map(|oop| {
                        Self::initial_oop(instance, oop)
                    }).ok_or(AllocError::OOM)
            }
            Klass::Array(_) => panic!(),
        }
    }

    pub(crate) fn alloc_array(&mut self, klass: &Klass, length: usize) -> Result<Oop, AllocError> {
        match klass {
            Klass::Instance(_) => {
                panic!()
            }
            Klass::Array(array) => {
                let header_size = size_of::<Header>();
                let required_size = header_size + length *  8;
                self.cur.alloc(Self::align_to_8_bytes(required_size))
                    .map(|oop| {
                        Self::initial_array_oop(array, oop)
                    }).ok_or(AllocError::OOM)
            }
        }
    }
    
    fn initial_oop(klass: &InstanceKlass, oop: Oop) -> Oop {
        oop
    }
    
    fn initial_array_oop(klass: &ArrayKlass, oop: Oop) -> Oop {
        oop
    }

    fn align_to_8_bytes(required_size: usize) -> usize {
        Self::align_up(required_size, 8)
    }

    /// Align downwards. Returns the greatest x with alignment `align`
    /// so that x <= addr. The alignment must be a power of 2.
    fn align_down(addr: usize, align: usize) -> usize {
        if align.is_power_of_two() {
            addr & !(align - 1)
        } else if align == 0 {
            addr
        } else {
            panic!("`align` must be a power of 2");
        }
    }

    /// Align upwards. Returns the smallest x with alignment `align`
    /// so that x >= addr. The alignment must be a power of 2.
    fn align_up(addr: usize, align: usize) -> usize {
        Self::align_down(addr + align - 1, align)
    }

}
