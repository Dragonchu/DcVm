use std::alloc::System;
use crate::class::Klass;
use crate::heap::RawPtr;
use crate::vm::Vm;
use crate::{
    instructions::Instruction,
    method::Method,
    pc_register::PcRegister,
    stack::Stack,
};

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
        receiver: Option<RawPtr>,
        method: Method,
        class: Klass,
        args: Vec<RawPtr>,
        vm: &mut Vm,
    ) {
        self.stack.add_frame(receiver, method, class, args);
        self.execute(vm);
        self.stack.pop_frame();
    }

    fn execute(&mut self, vm: &mut Vm) {
        let cur_frame = self.stack.cur_frame();
        let cur_method = cur_frame.get_cur_method();
        let cur_class = cur_frame.get_cur_class();
        let code = cur_method.get_code();
        for instruction in code.byte_codes.iter() {
            match instruction {
                Instruction::Getstatic(field_index) => {
                    let (klass_name, field_name, desc)=
                        cur_class.get_field_info(field_index);
                    let klass = vm.load(&klass_name);
                    let value = klass.get_static_instance(&field_name, &desc);
                    self.stack.cur_frame().push_value(value);
                }
                _ => {
                    println!("{:?}", instruction)
                }
            }
        }
    }
}
