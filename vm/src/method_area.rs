use reader::class_file::ClassFile;
use std::cell::RefCell;
use std::rc::Rc;
use typed_arena::Arena;

use crate::class::{
    ArrayKlassDesc, ArrayKlassRef, ComponentType, InstanceKlassDesc, InstanceKlassRef,
};

pub struct MethodArea {
    instance_allocator: Arena<InstanceKlassDesc>,
    array_allocator: Arena<ArrayKlassDesc>,
    class_file_allocator: Arena<ClassFile>,
}

impl MethodArea {
    pub fn new() -> MethodArea {
        let instance_allocator: Arena<InstanceKlassDesc> = Arena::new();
        let array_allocator: Arena<ArrayKlassDesc> = Arena::new();
        let class_file_allocator: Arena<ClassFile> = Arena::new();
        MethodArea {
            instance_allocator,
            array_allocator,
            class_file_allocator,
        }
    }

    pub fn allocate_instance_klass(&self, class_file: ClassFile) -> InstanceKlassRef {
        let class_file_ref = Box::new(class_file);
        let klass = InstanceKlassDesc::new(class_file_ref);
        Rc::new(RefCell::new(klass))
    }

    pub fn allocate_array_klass(
        &self,
        dimension: usize,
        component_type: ComponentType,
    ) -> ArrayKlassRef {
        let klass = ArrayKlassDesc::new(dimension, component_type);
        Rc::new(RefCell::new(klass))
    }
}
