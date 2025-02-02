use std::ptr::NonNull;
use crate::heap::{AllocRaw, Heap};
use crate::{class::Value, class_loader::BootstrapClassLoader, jvm_thread::JvmThread, HEAP};

struct Vm {
    class_loader: BootstrapClassLoader,
}
impl Vm {
    fn new(paths: &str) -> Vm {
        Vm {
            class_loader: BootstrapClassLoader::new(paths),
        }
    }

    fn invoke_main(&self, args: Vec<&str>) {
        let args_oop = self.new_string_array(args);
        let main_class = self
            .class_loader
            .load("Main");
        let main_method = main_class
            .get_method("main", "([Ljava/lang/String;)V");
        let mut java_main_thread = JvmThread::new(&self.class_loader);
        java_main_thread.invoke(None, main_method, main_class, args_oop.clone());
    }

    fn new_string_array(&self, args: Vec<&str>) -> Vec<Value> {
        let string_arr_class = self.class_loader.load("[Ljava/lang/String");
        for arg in args.iter(){
            //todo allocate arg oop
        }
        todo!()
    }

    fn new_string(&self, s: &str) -> Value {
        let char_array_klass = self.class_loader.load("[C");
        let string_klass = self
            .class_loader
            .load("java/lang/String");
        let char_array: Vec<Value> = s.encode_utf16().map(|c| Value::Int(c as i32)).collect();
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::Vm;


    #[test]
    fn new_string_test() {
        let vm = Vm::new("/Users/dragonchu/.sdkman/candidates/java/8.0.422-amzn/jre/lib/rt.jar");
        let java_string_oop = vm.new_string("Hello world");
        println!("{:?}", java_string_oop);
    }

    #[test]
    fn args_oop_test() {
        let vm = Vm::new("/Users/dragonchu/.sdkman/candidates/java/8.0.422-amzn/jre/lib/rt.jar");
        let args = vec!["hello", "world"];
        let args_oop_array = vm.new_string_array(args);
        println!("{:?}", args_oop_array);
    }

    #[test]
    fn invoke_main_test() {
        let vm = Vm::new(
            "resources/test:/Users/dragonchu/.sdkman/candidates/java/8.0.422-amzn/jre/lib/rt.jar",
        );
        let args = vec!["hello", "world"];
        vm.invoke_main(args);
    }
}
