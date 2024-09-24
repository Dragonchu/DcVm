use vm::VM;

mod classfile;
mod common;
mod loader;
mod vm;
mod system_dic;
fn main() {
    let vm = VM::new("/Users/dragonchu/.sdkman/candidates/java/8.0.422-amzn/jre/lib/rt.jar");
    vm.run("Main");
}
