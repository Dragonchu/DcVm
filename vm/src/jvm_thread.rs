use crate::{pc_register::PcRegister, stack::Stack};

struct JvmThread{
    pc_register: PcRegister,
    stack: Stack,
    native: Stack
}