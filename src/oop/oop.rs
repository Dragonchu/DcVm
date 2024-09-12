use crate::common::types::{Jdouble, Jfloat, Jint, Jlong};

use super::klass::{InstanceKlass, Klass};

enum OopType {
    InstanceOop,
    PrimitiveOop,
    ObjectArrayOop,
    TypeArrayOop,
}

enum ValueType {
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

type MarkOop = Option<Box<MarkOopDesc>>;
type Oop = Option<Box<OopDesc>>;

struct MarkOopDesc {
    oop_type: OopType,
    hash: Jint,
}

impl MarkOopDesc {
    fn new(oop_type: OopType) -> Self {
        MarkOopDesc {
            oop_type,
            hash: 0,
        }
    }
}

enum OopDesc {
    InstanceOopDesc(InstanceOopDesc),
    ArrayOopDesc(ArrayOopDesc),
    MirrorOopDesc(MirrorOopDesc),
    IntOopDesc(IntOopDesc),
    LongOopDesc(LongOopDesc),
    FloatOopDesc(FloatOopDesc),
    DoubleOopDesc(DoubleOopDesc),
}

struct InstanceOopDesc {
    mark: MarkOopDesc,
    klass: Klass,
    instance_field_values: Vec<Oop>,
}

struct ArrayOopDesc {
    mark: MarkOop,
    klass: Box<Klass>,
    elements: Vec<Oop>,
}

struct MirrorOopDesc {
    mark: MarkOop,
    klass: Box<Klass>,
    mirror_target: Box<Klass>,
    mirroring_primitive_type: ValueType,
}

struct IntOopDesc {
    mark: MarkOop,
    value: Jint,
}

struct LongOopDesc {
    mark: MarkOop,
    value: Jlong,
}

struct FloatOopDesc {
    mark: MarkOop,
    value: Jfloat,
}

struct DoubleOopDesc {
    mark: MarkOop,
    value: Jdouble,
}
