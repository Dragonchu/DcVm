use reader::types::U4;
use vm::heap::RawPtr;
use vm::jvm_thread::JvmThread;
use vm::vm::Vm;
use vm::JvmValue;

fn invoke_main(args: Vec<&str>, vm: &mut Vm) {
    let main_class = vm.load("Main");
    let main_method = main_class
        .get_method("main", "([Ljava/lang/String;)V")
        .expect("failed to load main method")
        .clone();
    let mut java_main_thread = JvmThread::new();
    let args = build_args(args, vm);
    java_main_thread.invoke(None, main_method, main_class, args, vm);
}

fn build_args(args: Vec<&str>, vm: &mut Vm) -> Vec<RawPtr> {
    let string_array_klass = vm.load("[Ljava/lang/String;");
    let mut string_array = vm.alloc_array(&string_array_klass, args.len()).unwrap();
    for (idx, arg) in args.iter().enumerate() {
        string_array.put_field(JvmValue::ObjRef(build_string(arg, vm)), idx);
    }
    vec![string_array]
}


fn build_string(s: &str, vm: &mut Vm) -> RawPtr {
    //load String class
    let string_klass = vm.load("java/lang/String");
    let mut string_data = vm.alloc_object(&string_klass).unwrap();

    //build char array
    let char_array_klass = vm.load("[C");
    let char_array: Vec<JvmValue> = s.encode_utf16().map(|c| JvmValue::Int(c as U4)).collect();
    let mut char_array_data = vm.alloc_array(&char_array_klass, char_array.len()).unwrap();
    for i in 0..char_array.len() {
        char_array_data.put_field(char_array.get(i).unwrap().clone(), i);
    }

    //construct String
    string_data.put_field(JvmValue::ObjRef(char_array_data), 0);

    string_data
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm::vm::Vm;


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