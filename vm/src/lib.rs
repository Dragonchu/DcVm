use std::sync::LazyLock;
use heap::Heap;

mod runtime_constant_pool;
mod class_loader;
mod class;
mod jvm_thread;
mod native_method;
mod pc_register;
mod stack;
mod vm;
mod method;
mod instructions;
mod field;
pub mod heap;

static HEAP: LazyLock<Heap> = LazyLock::new(|| {
    let heap = Heap::new();
    heap
});
