use typed_arena::Arena;

use crate::class::Class;

pub struct MethodArea {
    allocator: Arena<Class>
}