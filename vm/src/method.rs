use reader::{constant_pool::{ConstantPool, CpInfo, CpInfoData}, method_info::MethodInfo, types::{U1, U2, U4}};
#[derive(Debug)]
struct ExceptionEntry {
    start_pc: U2,
    end_pc: U2,
    handler_pc: U2,
    catch_type: U2
}
#[derive(Debug)]
struct Code {
    max_stack: U2,
    max_locals: U2,
    code: Vec<U1>,
    exception_table_length: U2,
    exception_table: Vec<ExceptionEntry>
}
#[derive(Debug)]
pub struct Method {
    access_flags: U2,
    name: String,
    descriptor: String,
    //code: Code,
}
impl Method {
    pub fn new(method_info: &MethodInfo, cp_pool: & dyn ConstantPool) -> Method {
        Method {
            access_flags: method_info.access_flags,
            name:  cp_pool.get_utf8_string(method_info.name_index),
            descriptor: cp_pool.get_utf8_string(method_info.descriptor_index)
        }
    }
}