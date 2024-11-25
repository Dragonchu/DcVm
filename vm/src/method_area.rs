use reader::class_file::ClassFile;
use typed_arena::Arena;

use crate::class::{InstanceKlassDesc, InstanceKlassRef};

pub struct MethodArea<'a> {
    instance_allocator: Arena<InstanceKlassDesc<'a>>,
    class_file_allocator: Arena<ClassFile>
}

impl<'a> MethodArea<'a> {
    pub fn new() -> MethodArea<'a> {
        let instance_allocator: Arena<InstanceKlassDesc> = Arena::new();
        let class_file_allocator: Arena<ClassFile> = Arena::new();
        MethodArea {
            instance_allocator,
            class_file_allocator
        }
    }
    pub fn allocate_instance_klass(&'a self, class_file: ClassFile) -> InstanceKlassRef<'a>{
        let class_file_ref = self.class_file_allocator.alloc(class_file);
        self.instance_allocator
            .alloc(InstanceKlassDesc::new(class_file_ref))
    }
}