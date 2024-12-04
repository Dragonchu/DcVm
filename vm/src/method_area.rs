use reader::class_file::ClassFile;
use typed_arena::Arena;

use crate::class::{ArrayKlassDesc, ArrayKlassRef, ComponentType, InstanceKlassDesc, InstanceKlassRef};

pub struct MethodArea<'klass, 'oop> {
    instance_allocator: Arena<InstanceKlassDesc<'klass, 'oop>>,
    array_allocator: Arena<ArrayKlassDesc<'klass>>,
    class_file_allocator: Arena<ClassFile>
}

impl<'klass, 'oop> MethodArea<'klass, 'oop> {
    pub fn new() -> MethodArea<'klass, 'oop> {
        let instance_allocator: Arena<InstanceKlassDesc> = Arena::new();
        let array_allocator: Arena<ArrayKlassDesc> = Arena::new();
        let class_file_allocator: Arena<ClassFile> = Arena::new();
        MethodArea {
            instance_allocator,
            array_allocator,
            class_file_allocator
        }
    }
    pub fn allocate_instance_klass(&'klass self, class_file: ClassFile) -> InstanceKlassRef<'klass>{
        let class_file_ref = self.class_file_allocator.alloc(class_file);
        self.instance_allocator
            .alloc(InstanceKlassDesc::new(class_file_ref))
    }
    pub fn allocate_array_klass(&'klass self, dimension: usize, component_type: ComponentType<'klass>) -> ArrayKlassRef<'klass> {
        self.array_allocator
            .alloc(ArrayKlassDesc::new(dimension, component_type))
    }
}