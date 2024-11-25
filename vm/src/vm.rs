use crate::{heap::Heap, method_area::MethodArea};

struct Vm<'a> {
    heap: Heap,
    method_area: MethodArea<'a>
}