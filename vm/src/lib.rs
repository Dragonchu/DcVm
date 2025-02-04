use std::cell::LazyCell;
use std::sync::{LazyLock, Mutex};
use heap::Heap;

mod runtime_constant_pool;
mod class_loader;
pub mod class;
pub mod jvm_thread;
mod native_method;
mod pc_register;
mod stack;
pub mod vm;
mod method;
mod instructions;
mod field;
pub mod heap;
