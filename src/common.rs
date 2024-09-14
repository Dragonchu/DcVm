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
