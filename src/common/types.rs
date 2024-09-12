pub type U1 = u8;
pub type U2 = u16;
pub type U4 = u32;
pub type U8 = u64;

pub type Jboolean = u8;
pub type Jchar = u16;
pub type Jshort = i16;
pub type Jint = i32;
pub type Jlong = i64;
pub type Jbyte = i8;
pub type Jfloat = f32;
pub type Jdouble = f64;
pub type Jsize = isize;

#[derive(Debug, Clone, Copy)]
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
