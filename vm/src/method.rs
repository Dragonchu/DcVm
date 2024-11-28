use std::{cell::{Cell, Ref, RefCell}, fmt::format};

use reader::{attribute_info::AttributeInfo, constant_pool::{ConstantPool, CpInfo, CpInfoData}, method_info::MethodInfo, types::{U1, U2, U4}};
#[derive(Debug, Clone)]
struct ExceptionEntry {
    start_pc: U2,
    end_pc: U2,
    handler_pc: U2,
    catch_type: U2
}
#[derive(Debug, Clone)]
struct Code {
    max_stack: U2,
    max_locals: U2,
    code: Vec<U1>,
    exception_table_length: U2,
    exception_table: Vec<ExceptionEntry>
}
#[derive(Debug, Clone)]
pub struct Method {
    access_flags: U2,
    name: String,
    descriptor: String,
    code: Option<Code>,
}

    pub fn link_code(method_info: &MethodInfo) -> Option<Code>{
        for attribute_info in &method_info.attributes {
            match attribute_info {
                AttributeInfo::Code { attribute_name_index, attribute_length, max_stack, max_locals, code_length, code, exception_table_length, exception_table, attributes_count, attributes } => {
                    let mut rt_exception_table = Vec::new();
                    for exception_entry in exception_table {
                        rt_exception_table.push(
                            ExceptionEntry{
                                start_pc: exception_entry.0,
                                end_pc: exception_entry.1,
                                handler_pc: exception_entry.2,
                                catch_type: exception_entry.3
                            }
                        )
                    }
                    return Some(Code {
                        max_stack: max_stack.clone(),
                        max_locals: max_locals.clone(),
                        code: code.clone(),
                        exception_table_length: exception_table_length.clone(),
                        exception_table: rt_exception_table
                    });
                }
                _ => {
                }
            }
        }
        return None;
    }

impl Method {
    pub fn new(method_info: &MethodInfo, cp_pool: & dyn ConstantPool) -> Method {
        Method {
            access_flags: method_info.access_flags,
            name:  cp_pool.get_utf8_string(method_info.name_index),
            descriptor: cp_pool.get_utf8_string(method_info.descriptor_index),
            code: link_code(method_info)
        }
    }

    pub fn get_unique_key(&self) -> String {
        let name = self.name.clone();
        let descriptor = self.descriptor.clone();
        format!("{name} {descriptor}")
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_descriptor(&self) -> String {
        self.descriptor.clone()
    }

}