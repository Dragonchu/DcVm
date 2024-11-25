use crate::{pc_register::PcRegister, stack::Stack};

struct JvmThread<'rtcp> {
    pc_register: PcRegister,
    stack: Stack<'rtcp>,
    native: Stack<'rtcp>
}