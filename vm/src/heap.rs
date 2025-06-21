use std::alloc::Layout;
use std::cell::Cell;
use std::fmt;
use std::fmt::Formatter;
use crate::class::{ArrayKlass, InstanceKlass, Klass};
use bitfield_struct::bitfield;
use std::ptr::NonNull;
use log::debug;
use reader::types::{U1, U2, U4, U8};
use crate::JvmValue;

#[bitfield(u64)]
#[derive(PartialEq, Eq)]
pub(crate) struct Header {
    #[bits(10)]
    pub(crate) class_id: usize,

    #[bits(1)]
    pub(crate) state: GcState,

    #[bits(30)]
    identity_hash_code: i32,

    #[bits(23)]
    pub(crate) size: usize,
}

#[derive(Clone, Debug, Copy)]
pub struct RawPtr(pub *mut u8);

impl RawPtr {
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn put_field(&mut self, value: JvmValue, index: usize) {
        unsafe {
            let ptr = self.0.add(index);
            match value {
                JvmValue::Boolean(val) => std::ptr::write(ptr as *mut U1, val),
                JvmValue::Byte(val) => std::ptr::write(ptr as *mut U1, val),
                JvmValue::Short(val) => std::ptr::write(ptr as *mut U2, val),
                JvmValue::Int(val) => std::ptr::write(ptr as *mut U4, val),
                JvmValue::Long(val) => std::ptr::write(ptr as *mut U8, val),
                JvmValue::Float(val) => std::ptr::write(ptr as *mut U8, val),
                JvmValue::Double(val) => std::ptr::write(ptr as *mut U8, val),
                JvmValue::Char(val) => std::ptr::write(ptr as *mut U2, val),
                JvmValue::ObjRef(val) => std::ptr::write(ptr as *mut RawPtr, val),
                JvmValue::Null => std::ptr::write(ptr, 0),
            }
        }
    }
    
    pub fn get_field_value(&self, klass: &InstanceKlass, index: usize) -> JvmValue {
        // 获取字段信息
        let fields = klass.get_static_fields();
        if index >= fields.len() {
            return JvmValue::Null; // 字段索引越界
        }
        
        let field = &fields[index];
        let field_values = klass.get_static_field_values();
        
        // 返回字段值，如果索引越界则返回默认值
        if index < field_values.len() {
            field_values[index]
        } else {
            field.get_default()
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
    fn alloc(&mut self, required_size: usize) -> Option<RawPtr> {
        if self.used + required_size > self.capacity {
            return None;
        }

        // We require all allocations to be aligned to 8 bytes!
        assert_eq!(required_size % 8, 0);

        let ptr = unsafe { self.memory.add(self.used) };
        self.used += required_size;

        Some(RawPtr(ptr))
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

    /// 分配一个对象，返回RawPtr
    pub fn alloc_object(&mut self, klass: &InstanceKlass) -> Result<RawPtr, AllocError> {
        let header_size = std::mem::size_of::<Header>();
        let field_size = klass.get_instance_fields().len() * 8; // 8字节对齐，支持int/引用
        let total_size = Self::align_to_8_bytes(header_size + field_size);
        let ptr = self.cur.alloc(total_size).ok_or(AllocError::OOM)?;
        // 初始化对象头部
        unsafe {
            let header_ptr = ptr.0 as *mut Header;
            *header_ptr = Header::new()
                .with_class_id(klass.class_id)
                .with_state(GcState::Unmarked)
                .with_identity_hash_code(0)
                .with_size(total_size);
        }
        Ok(ptr)
    }

    /// 分配一个数组对象，返回RawPtr
    pub fn alloc_array(&mut self, klass: &ArrayKlass, length: usize) -> Result<RawPtr, AllocError> {
        let header_size = std::mem::size_of::<Header>();
        let elem_size = 8; // 8字节对齐，支持int/引用
        let total_size = Self::align_to_8_bytes(header_size + 8 + length * elem_size); // 8字节存储length
        let ptr = self.cur.alloc(total_size).ok_or(AllocError::OOM)?;
        // 初始化头部和length
        unsafe {
            let header_ptr = ptr.0 as *mut Header;
            *header_ptr = Header::new()
                .with_class_id(klass.class_id)
                .with_state(GcState::Unmarked)
                .with_identity_hash_code(0)
                .with_size(total_size);
            let len_ptr = ptr.0.add(header_size) as *mut usize;
            *len_ptr = length;
        }
        Ok(ptr)
    }

    /// 设置对象字段
    pub fn put_field(&mut self, obj: RawPtr, field_offset: usize, value: JvmValue) {
        let header_size = std::mem::size_of::<Header>();
        let addr = unsafe { obj.0.add(header_size + field_offset) };
        match value {
            JvmValue::Int(v) => unsafe { *(addr as *mut i32) = v as i32 },
            JvmValue::ObjRef(ptr) => unsafe { *(addr as *mut RawPtr) = ptr },
            _ => unimplemented!("暂不支持该类型"),
        }
    }

    /// 获取对象字段
    pub fn get_field(&self, obj: RawPtr, field_offset: usize) -> JvmValue {
        let header_size = std::mem::size_of::<Header>();
        let addr = unsafe { obj.0.add(header_size + field_offset) };
        // 这里只举例 int
        let v = unsafe { *(addr as *const i32) };
        JvmValue::Int(v as u32)
    }

    /// 设置数组元素
    pub fn put_array_element(&mut self, arr: RawPtr, index: usize, value: JvmValue) {
        let header_size = std::mem::size_of::<Header>();
        let addr = unsafe { arr.0.add(header_size + 8 + index * 8) };
        match value {
            JvmValue::Int(v) => unsafe { *(addr as *mut i32) = v as i32 },
            JvmValue::ObjRef(ptr) => unsafe { *(addr as *mut RawPtr) = ptr },
            _ => unimplemented!("暂不支持该类型"),
        }
    }

    /// 获取数组元素
    pub fn get_array_element(&self, arr: RawPtr, index: usize) -> JvmValue {
        let header_size = std::mem::size_of::<Header>();
        let addr = unsafe { arr.0.add(header_size + 8 + index * 8) };
        let v = unsafe { *(addr as *const i32) };
        JvmValue::Int(v as u32)
    }

    fn align_to_8_bytes(required_size: usize) -> usize {
        (required_size + 7) & !7
    }
}
