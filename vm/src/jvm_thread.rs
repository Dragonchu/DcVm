use crate::{
    class::{InstanceKlassRef, Oop},
    class_loader::BootstrapClassLoader,
    instructions::Instruction,
    method::Method,
    method_area::MethodArea,
    pc_register::PcRegister,
    stack::Stack,
};

pub struct JvmThread<'thread, 'memory> {
    pc_register: PcRegister,
    stack: Stack<'thread>,
    native: Stack<'thread>,
    class_loader: &'memory BootstrapClassLoader,
    methead_area: &'memory MethodArea,
}

impl<'thread, 'memory> JvmThread<'thread, 'memory> {
    pub fn new(
        class_loader: &'memory BootstrapClassLoader,
        methead_area: &'memory MethodArea,
    ) -> JvmThread<'thread, 'memory> {
        JvmThread {
            pc_register: PcRegister::new(),
            stack: Stack::new(),
            native: Stack::new(),
            class_loader,
            methead_area,
        }
    }

    pub fn invoke(
        &'thread self,
        receiver: Option<Oop>,
        method: Method,
        class: InstanceKlassRef,
        args: Vec<Oop>,
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
                    let (class_name, field_name, descriptor) =
                        cur_class.borrow().get_field_info(field_index);
                    let field_class = self.class_loader.load(&class_name, &self.methead_area);
                    println!("Getstatic: {:?}", field_class)
                }
                _ => {
                    println!("{:?}", instruction)
                }
            }
        }
    }
}
