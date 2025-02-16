use crate::heap::{AllocError, Heap, Oop};
use crate::{class::Value, class_loader::BootstrapClassLoader, jvm_thread::JvmThread, };
use crate::class::Klass;

pub struct Vm {
    class_loader: BootstrapClassLoader,
    heap: Heap,
}
impl Vm {
    pub fn new(paths: &str) -> Vm {
        Vm {
            class_loader: BootstrapClassLoader::new(paths),
            heap: Heap::with_maximum_memory(1024 * 1024),
        }
    }
    
    pub fn load(&self, class_name: &str) -> Klass {
        self.class_loader.load(class_name)
    }

    pub fn alloc_array(&mut self, klass: &Klass, length: usize) -> Result<Oop, AllocError> {
        self.heap.alloc_array(klass, length)
    }
    
    pub fn alloc_object(&mut self, klass: &Klass) -> Result<Oop, AllocError> {
        self.heap.alloc(klass)
    }
    
}




