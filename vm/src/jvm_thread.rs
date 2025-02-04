use crate::{
    class::{Value},
    class_loader::BootstrapClassLoader,
    instructions::Instruction,
    method::Method,
    pc_register::PcRegister,
    stack::Stack,
};
use crate::class::Klass;
use crate::heap::{Oop};
use crate::vm::Vm;

pub struct JvmThread {
    pc_register: PcRegister,
    stack: Stack,
    native: Stack,
}

impl JvmThread {
    pub fn new(
    ) -> JvmThread {
        JvmThread {
            pc_register: PcRegister::new(),
            stack: Stack::new(),
            native: Stack::new(),
        }
    }

    pub fn invoke(
        &mut self,
        receiver: Option<Oop>,
        method: Method,
        class: Klass,
        args: Vec<Oop>,
        vm: &mut Vm,
    ) {
        self.stack.add_frame(receiver, method, class, args);
        self.execute(vm);
        self.stack.pop_frame();
    }

    fn execute(&self, vm: &mut Vm) {
        let cur_frame = self.stack.cur_frame();
        let cur_method = cur_frame.get_cur_method();
        let cur_class = cur_frame.get_cur_class();
        let code = cur_method.get_code();
        for instruction in code.byte_codes.iter() {
            match instruction {
                Instruction::Getstatic(field_index) => {
                    let filed=
                        cur_class.get_field_info(field_index);
                    let field_class = vm.load(&filed.get_name());
                    println!("Getstatic: {:?}", field_class)
                }
                _ => {
                    println!("{:?}", instruction)
                }
            }
        }
    }
}
