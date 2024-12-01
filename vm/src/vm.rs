use crate::{
    class::{ArrayOopDesc, InstanceOopDesc, InstanceOopRef, Klass, Oop},
    class_loader::BootstrapClassLoader,
    heap::Heap,
    method_area::MethodArea,
};

struct Vm<'memory> {
    heap: Heap<'memory>,
    method_area: MethodArea<'memory>,
    class_loader: BootstrapClassLoader<'memory>,
}
impl<'memory> Vm<'memory> {
    fn new(paths: &'memory str) -> Vm<'memory> {
        Vm {
            heap: Heap::new(),
            method_area: MethodArea::new(),
            class_loader: BootstrapClassLoader::new(paths),
        }
    }

    fn new_string(&'memory self, s: &str) -> InstanceOopRef<'memory> {
        let char_array_klass = self.class_loader.load_array_klass("[C", &self.method_area);
        let string_klass = self.class_loader.load_instance_klass("java/lang/String", &self.method_area);
        let mut chars = char_array_klass.new_instance(s.len());
        let char_array: Vec<Oop> = s.encode_utf16().map(|c| Oop::Int(c as i32)).collect();
        for (index, oop) in char_array.iter().enumerate() {
            chars.set_element_at(index, oop.clone());
        }
        let chars_ref = self.heap.allocate_array_oop(chars);
        let mut java_string = string_klass.new_instance();
        java_string.set_field_value("java/lang/String", "value", "[C", Oop::Array(chars_ref));
        self.heap.allocate_instance_oop(java_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn layout_test() {
        use std::mem;
        println!(
            "size: {}, align: {}",
            std::mem::size_of::<InstanceOopDesc>(),
            std::mem::align_of::<InstanceOopDesc>()
        );
    }

    #[test]
    fn new_string_test() {
        let vm = Vm::new("/home/codespace/java/current/jre/lib/rt.jar");
        let java_string_oop = vm.new_string("Hello world");
        println!("{:?}", java_string_oop);
    }

}
