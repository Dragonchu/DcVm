#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Void,
    Byte,
    Boolean,
    Char,
    Short,
    Int,
    Float,
    Long,
    Double,
    Object,
    Array,
}

pub type U1 = u8;
pub type U2 = u16;
pub type U4 = u32;
pub type U8 = u64;