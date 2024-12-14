use crate::class::{ArrayOopDesc, ArrayOopRef, InstanceOopDesc, InstanceOopRef};
use gc::Gc;

pub struct Heap;
impl Heap {
    pub fn new() -> Heap {
        Heap {}
    }
    pub fn allocate_instance_oop(&self, instance_oop: InstanceOopDesc) -> InstanceOopRef {
        Gc::new(instance_oop)
    }

    pub fn allocate_array_oop(&self, array_oop: ArrayOopDesc) -> ArrayOopRef {
        Gc::new(array_oop)
    }
}
