use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    bytecode::code_blob::CodeBlob,
    classfile::{
        attribute_info::{CodeAttribute, ExceptionsAttribute},
        class_file::MethodInfo,
        types::U2,
    },
    common::ValueType,
};

use super::klass::{instance_klass::InstanceKlass, mirror_oop::MirrorOop};

pub struct Method(Rc<RefCell<MethodInner>>);

pub struct MethodInner {
    _klass: InstanceKlass,
    _name: String,
    _descriptor: String,
    _signature: String,
    _access_flags: U2,

    /**
     * basic information about a method
     */
    _code_blob: CodeBlob,
    _method_info: MethodInfo,
    _exception_attr: ExceptionsAttribute,
    _code_attr: CodeAttribute,
    /**
     * only available when this method is a native method
     */
    _native_ptr: (),
    /**
     * flags related to descriptor parsing
     */
    _argument_value_types_resolved: bool,
    _argument_value_types_no_wrap_resolved: bool,
    _argument_class_types_resolved: bool,
    _return_type_no_wrap_resolved: bool,
    _return_type_resolved: bool,
    _checked_exceptions_resolved: bool,
    _linked: bool,
    /**
     * result of descriptor parsing
     */
    _return_type: ValueType,
    _return_type_no_wrap: ValueType,
    _argument_value_types: Vec<ValueType>,
    _argument_value_types_no_wrap: Vec<ValueType>,
    _argument_class_types: Vec<MirrorOop>,
    _return_class_type: MirrorOop,

    /** this method is likely to throw these checked exceptions **/
    _checked_exceptions: Vec<InstanceKlass>,
    /** map<start-pc, line-number> **/
    _line_number_table: HashMap<U2, U2>,
    /**
     * annotations
     */
    _runtime_visible_annos: Vec<()>,
    _runtime_visible_param_annos: Vec<()>,
    _runtime_visible_type_annos: Vec<()>,
}
