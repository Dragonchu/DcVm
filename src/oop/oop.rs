use crate::oop::klass::Klass;

enum OopType {
    InstanceOop,
    PrimitiveOop,
    ObjectArrayOop,
    TypeArrayOop,
}
enum ValueType {
    VOID,
    BYTE,
    BOOLEAN,
    CHAR,
    SHORT,
    INT,
    FLOAT,
    LONG,
    DOUBLE,
    OBJECT,
    ARRAY,
}
enum Oop {
    Array(ArrayOop),
    Double(PrimitiveOop<f32>),
    Float(PrimitiveOop<f64>),
    Instance(InstanceOop),
    Int(PrimitiveOop<i32>),
    Long(PrimitiveOop<i64>),
    Mirror(InstanceOop),
    ObjectArray(ArrayOop),
    Primitive(BaseOop),
    TypeArray(ArrayOop),
}

struct BaseOop {
    _mark: MarkOop,
    _klass: Klass,
}

struct InstanceOop {
    _base_oop: BaseOop, 
    _instance_field_values: Vec<Oop>,
}

pub struct MirrorOop {
    _instance_oop: InstanceOop,
    _mirror_target: Klass,
    _mirroring_primitive_type: ValueType
}

struct PrimitiveOop<T> {
    _base_oop: BaseOop,
    _value: T,
}

struct ArrayOop {
    _base_oop: BaseOop,
    _elements: Vec<Oop>,
}

struct MarkOop {
    oop_type: OopType,
    hash: u32,
}
