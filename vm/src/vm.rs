use crate::heap::ObjPtr;
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
        let mut  args_oop = Vec::with_capacity(args.len());
        for arg in args {
            args_oop.push(self.new_string(arg));
        }
        let main_class = self
            .class_loader
            .load("Main");
        let main_method = main_class
            .get_method("main", "([Ljava/lang/String;)V");
        let mut java_main_thread = JvmThread::new(&self.class_loader);
        java_main_thread.invoke(None, main_method, main_class, args_oop);
    }


    fn new_string(&self, s: &str) -> ObjPtr {
        //load String class
        let string_klass = self
            .class_loader
            .load("java/lang/String");
        let mut string_data = HEAP.alloc(string_klass).unwrap();
        
        //build char array
        let char_array_klass = self.class_loader.load("[C");
        let char_array: Vec<Value> = s.encode_utf16().map(|c| Value::Int(c as i32)).collect();
        let mut char_array_data = HEAP.alloc_array(char_array_klass, char_array.len()).unwrap(); 
        for i in 0..char_array.len() {
            char_array_data.set_element(char_array.get(i).unwrap().clone(), i);
        } 
        
        //construct String
        string_data.set_element(Value::Obj(char_array_data), 0); 
        
        string_data
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
    fn invoke_main_test() {
        let vm = Vm::new(
            "resources/test:/Users/dragonchu/.sdkman/candidates/java/8.0.422-amzn/jre/lib/rt.jar",
        );
        let args = vec!["hello", "world"];
        vm.invoke_main(args);
    }
}
