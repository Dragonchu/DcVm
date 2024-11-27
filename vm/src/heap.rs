use typed_arena::Arena;

use crate::{class::{InstanceOopDesc, InstanceOopRef}, method_area::MethodArea};

pub struct Heap<'memory> {
    instance_oops: Arena<InstanceOopDesc<'memory>>,
}
impl<'a> Heap<'a> {
    pub fn new() -> Heap<'a>{
       Heap {
        instance_oops: Arena::new()
       } 
    }
    pub fn allocate_instance_oop(&'a self, instance_oop: InstanceOopDesc<'a>) -> InstanceOopRef<'a> {
        self.instance_oops.alloc(instance_oop)
    }
}