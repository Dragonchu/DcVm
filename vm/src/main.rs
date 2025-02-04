use vm::class::Value;
use vm::heap::{Heap, Oop};
use vm::jvm_thread::JvmThread;
use vm::vm::Vm;

fn invoke_main(args: Vec<&str>, vm: &mut Vm) {
    let main_class = vm.load("Main");
    let main_method = main_class
        .get_method("main", "([Ljava/lang/String;)V");
    let mut java_main_thread = JvmThread::new();
    let args = build_args(args, vm);
    java_main_thread.invoke(None, main_method, main_class, args, vm);
}

fn build_args(args: Vec<&str>, vm: &mut Vm) -> Vec<Oop> {
    let string_array_klass = vm.load("[Ljava/lang/String;");
    let mut string_array = vm.alloc_array(&string_array_klass, args.len()).unwrap();
    for (idx, arg) in args.iter().enumerate() {
        string_array.set_element(Value::Obj(build_string(arg, vm)), idx);
    }
    vec![string_array]
}


fn build_string(s: &str, vm: &mut Vm) -> Oop {
    //load String class
    let string_klass = vm.load("java/lang/String");
    let mut string_data = vm.alloc_object(&string_klass).unwrap();

    //build char array
    let char_array_klass = vm.load("[C");
    let char_array: Vec<Value> = s.encode_utf16().map(|c| Value::Int(c as i32)).collect();
    let mut char_array_data = vm.alloc_array(&char_array_klass, char_array.len()).unwrap();
    for i in 0..char_array.len() {
        char_array_data.set_element(char_array.get(i).unwrap().clone(), i);
    }

    //construct String
    string_data.set_element(Value::Obj(char_array_data), 0);

    string_data
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    use vm::vm::Vm;
    use super::*;


    #[test]
    fn new_string_test() {
        let mut vm = Vm::new("/Users/dragonchu/.sdkman/candidates/java/8.0.422-amzn/jre/lib/rt.jar");
        let java_string_oop = build_string("Hello world", &mut vm);
        println!("{:?}", java_string_oop);
    }


    #[test]
    fn invoke_main_test() {
        let mut vm = Vm::new(
            "resources/test:/Users/dragonchu/.sdkman/candidates/java/8.0.422-amzn/jre/lib/rt.jar",
        );
        let args = vec!["hello", "world"];
        invoke_main(args, &mut vm);
    }

    #[test]
    fn build_arg_test() {
        let mut vm = Vm::new("/Users/dragonchu/.sdkman/candidates/java/8.0.422-amzn/jre/lib/rt.jar");
        let args = build_args(vec!["hello", "world"], &mut vm);
        println!("{:?}", args);
    }
}