use crate::{
    class::{ArrayOopDesc, ArrayOopRef, InstanceOopDesc, InstanceOopRef, Oop},
    class_loader::BootstrapClassLoader,
    heap::Heap,
    jvm_thread::JvmThread,
    method_area::MethodArea,
};

struct Vm {
    heap: Heap,
    method_area: MethodArea,
    class_loader: BootstrapClassLoader,
}
impl Vm {
    fn new(paths: &str) -> Vm {
        Vm {
            heap: Heap::new(),
            method_area: MethodArea::new(),
            class_loader: BootstrapClassLoader::new(paths),
        }
    }

    fn invoke_main(&self, args: Vec<&str>) {
        let args_oop = self.new_string_array(args);
        let main_class = self
            .class_loader
            .load_instance_klass("Main", &self.method_area);
        let main_method = main_class
            .borrow()
            .get_method("main", "([Ljava/lang/String;)V");
        let args_vec = vec![Oop::Array(args_oop)];
        let java_main_thread = JvmThread::new(&self.class_loader, &self.method_area);
        java_main_thread.invoke(None, main_method, main_class, args_vec.clone());
    }

    fn new_string_array(&self, args: Vec<&str>) -> ArrayOopRef {
        let string_array_class = self
            .class_loader
            .load_array_klass("[Ljava/lang/String", &self.method_area);
        let mut string_array_oop = ArrayOopDesc::new(string_array_class, args.len());
        for (index, s) in args.iter().enumerate() {
            let arg_oop = self.new_string(s);
            string_array_oop.set_element_at(index, Oop::Instance(arg_oop));
        }
        self.heap.allocate_array_oop(string_array_oop)
    }

    fn new_string(&self, s: &str) -> InstanceOopRef {
        let char_array_klass = self.class_loader.load_array_klass("[C", &self.method_area);
        let string_klass = self
            .class_loader
            .load_instance_klass("java/lang/String", &self.method_area);
        let mut chars = ArrayOopDesc::new(char_array_klass, s.len());
        let char_array: Vec<Oop> = s.encode_utf16().map(|c| Oop::Int(c as i32)).collect();
        for (index, oop) in char_array.iter().enumerate() {
            chars.set_element_at(index, oop.clone());
        }
        let chars_ref = self.heap.allocate_array_oop(chars);
        let mut java_string = InstanceOopDesc::new(string_klass);
        java_string.set_field_value("java/lang/String", "value", "[C", Oop::Array(chars_ref));
        self.heap.allocate_instance_oop(java_string)
    }
}

#[cfg(test)]
mod tests {
    use crate::class::InstanceOopDesc;
    use crate::vm::Vm;

    #[test]
    fn layout_test() {
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

    #[test]
    fn args_oop_test() {
        let vm = Vm::new("/home/codespace/java/current/jre/lib/rt.jar");
        let args = vec!["hello", "world"];
        let args_oop_array = vm.new_string_array(args);
        println!("{:?}", args_oop_array);
    }

    #[test]
    fn invoke_main_test() {
        let vm = Vm::new("resources/test:/home/codespace/java/current/jre/lib/rt.jar");
        let args = vec!["hello", "world"];
        vm.invoke_main(args);
    }
}
