use crate::{
    class::{Value},
    class_loader::BootstrapClassLoader,
    instructions::Instruction,
    method::Method,
    pc_register::PcRegister,
    stack::Stack,
};
use crate::class::Klass;
use crate::heap::{ObjPtr, Object};

pub struct JvmThread<'a> {
    pc_register: PcRegister,
    stack: Stack,
    native: Stack,
    class_loader: &'a BootstrapClassLoader,
}

impl<'a> JvmThread<'a> {
    pub fn new(
        class_loader: &'a BootstrapClassLoader,
    ) -> JvmThread<'a> {
        JvmThread {
            pc_register: PcRegister::new(),
            stack: Stack::new(),
            native: Stack::new(),
            class_loader,
        }
    }

    pub fn invoke(
        &mut self,
        receiver: Option<ObjPtr>,
        method: Method,
        class: Klass,
        args: Vec<ObjPtr>,
    ) {
        self.stack.add_frame(receiver, method, class, args);
        self.execute();
        self.stack.pop_frame();
    }

    fn execute(&self) {
        let cur_frame = self.stack.cur_frame();
        let cur_method = cur_frame.get_cur_method();
        let cur_class = cur_frame.get_cur_class();
        let code = cur_method.get_code();
        for instruction in code.byte_codes.iter() {
            match instruction {
                Instruction::Getstatic(field_index) => {
                    let filed=
                        cur_class.get_field_info(field_index);
                    let field_class = self.class_loader.load(&filed.get_name());
                    println!("Getstatic: {:?}", field_class)
                }
                _ => {
                    println!("{:?}", instruction)
                }
            }
        }
    }
}
