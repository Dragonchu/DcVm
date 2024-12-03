use crate::{pc_register::PcRegister, stack::Stack};

struct JvmThread<'memory>{
    pc_register: PcRegister,
    stack: Stack<'memory>,
    native: Stack<'memory>
}