use crate::heap::{AllocError, Heap, RawPtr};
use crate::{class_loader::BootstrapClassLoader, jvm_thread::JvmThread, };
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
    
    pub fn load(&mut self, class_name: &str) -> Klass {
        self.class_loader.load(class_name, &mut self.heap)
    }

    pub fn alloc_array(&mut self, klass: &Klass, length: usize) -> Result<RawPtr, AllocError> {
        self.heap.alloc_array(klass, length)
    }
    
    pub fn alloc_object(&mut self, klass: &Klass) -> Result<RawPtr, AllocError> {
        self.heap.alloc(klass)
    }
    
}




