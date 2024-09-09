use crate::classpath::class_loader::ClassLoader;
use crate::oop::oop::MirrorOop;
use crate::types::U2;

enum ClassType {
    InstanceClass,
    ObjectArrayClass,
    TypeArrayClass,
}

enum ClassState {
    Allocated,
    Loaded,
    Linked,
    BeingInitialized,
    FullyInitialized,
    InitializationError,
}

pub struct Klass {
    _state: ClassState,
    _access_flag: U2,
    _name: String,
    _type: ClassType,
    _java_mirror: Box<MirrorOop>,
    _super_class: Box<InstanceKlass>,
}

pub struct InstanceKlass {
    _klass: Klass,
    _class_loader: ClassLoader,
    _java_loader: Box<MirrorOop>,
    
}
