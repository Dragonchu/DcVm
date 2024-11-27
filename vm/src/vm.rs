use crate::{class::{InstanceOopDesc, InstanceOopRef}, class_loader::BootstrapClassLoader, heap::Heap, method_area::MethodArea};

struct Vm<'a> {
    heap: Heap<'a>,
    method_area: MethodArea<'a>,
    class_loader: BootstrapClassLoader<'a>
}
impl<'a> Vm<'a> {
    fn new(paths: &'a str) -> Vm<'a> {
        Vm {
            heap: Heap::new(),
            method_area: MethodArea::new(),
            class_loader: BootstrapClassLoader::new(paths)
        }
    }
    fn new_instance(&'a self, class_name: &str) -> InstanceOopRef<'a> {
       let class = self.class_loader.load(class_name, &self.method_area);
       self.heap.allocate_instance_oop(InstanceOopDesc::new(class))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn parse_main_class() {
        let vm = Vm::new("resources/test:/home/codespace/java/current/jre/lib/rt.jar");
        let oop = vm.new_instance("Main");
        println!("{:?}", oop);
    }

    #[test]
    fn layout_test() {
        use std::mem;
        println!("size: {}, align: {}", std::mem::size_of::<InstanceOopDesc>(), std::mem::align_of::<InstanceOopDesc>());
    }
}
