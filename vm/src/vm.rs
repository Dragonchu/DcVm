use crate::{heap::Heap, method_area::MethodArea};

struct Vm {
    heap: Heap,
    method_area: MethodArea
}