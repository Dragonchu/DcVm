use crate::{taggedptr::{ArrayKlassDesc, ArrayOopDesc, InstanceOopDesc}, rawptr::RawPtr};

/// An unpacked tagged Fat Pointer that carries the type information in the enum structure.
/// This should represent every type native to the runtime.
// ANCHOR: DefFatPtr
#[derive(Copy, Clone)]
pub enum FatPtr {
    Nil,
    InstanceOop(RawPtr<InstanceOopDesc>),
    ArrayOop(RawPtr<ArrayOopDesc>),
    InstanceKlass(RawPtr<InstanceOopDesc>),
    ArrayKlass(RawPtr<ArrayKlassDesc>)
}


