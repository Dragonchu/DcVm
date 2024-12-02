use typed_arena::Arena;

use crate::{class::{ArrayKlassDesc, ArrayOopDesc, ArrayOopRef, InstanceOopDesc, InstanceOopRef}, method_area::MethodArea};

pub struct Heap<'memory> {
    instance_oops: Arena<InstanceOopDesc<'memory>>,
    array_oops: Arena<ArrayOopDesc<'memory>>
}
impl<'a> Heap<'a> {
    pub fn new() -> Heap<'a>{
       Heap {
        instance_oops: Arena::new(),
        array_oops: Arena::new()
       } 
    }
    pub fn allocate_instance_oop(&'a self, instance_oop: InstanceOopDesc<'a>) -> InstanceOopRef<'a> {
        self.instance_oops.alloc(instance_oop)
    }

    pub fn allocate_array_oop(&'a self, array_oop: ArrayOopDesc<'a>) -> ArrayOopRef<'a> {
        self.array_oops.alloc(array_oop)
    }
}