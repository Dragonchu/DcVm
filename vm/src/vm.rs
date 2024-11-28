use crate::{class::{InstanceOopDesc, InstanceOopRef}, class_loader::BootstrapClassLoader, heap::Heap, method_area::MethodArea};

struct Vm<'memory> {
    heap: Heap<'memory>,
    method_area: MethodArea<'memory>,
    class_loader: BootstrapClassLoader<'memory>
}
impl<'memory> Vm<'memory> {
    fn new(paths: &'memory str) -> Vm<'memory> {
        Vm {
            heap: Heap::new(),
            method_area: MethodArea::new(),
            class_loader: BootstrapClassLoader::new(paths)
        }
    }
    fn new_instance(&'memory self, class_name: &str) -> InstanceOopRef<'memory> {
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

    #[test]
    fn get_main_method() {
        let vm = Vm::new("resources/test:/home/codespace/java/current/jre/lib/rt.jar");
        let oop = vm.new_instance("Main");
        let klass = oop.get_klass();
        let method = klass.get_method("<init>", "()V");
        println!("{:?}", method);
    }
}
