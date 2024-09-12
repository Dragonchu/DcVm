use crate::{bytecode::code_blob::CodeBlob, classfile::{attribute_info::{CodeAtrribute, ExceptionsAttribute}, class_file::MethodInfo}, common::types::U2, jni::native_method::JavaNativeMethod};

struct Method {
    name: String,
    descriptor: String,
    signature: String,
    access_flags: U2,
    code_blob: CodeBlob, 
    method_info: MethodInfo,
    exception_attr: ExceptionsAttribute,
    code_attr: CodeAtrribute,
    native_method: Option<JavaNativeMethod>,
    argument_value_types_resolved: bool,
    argument_value_types_no_wrap_resolved: bool,
    argument_class_type_resolved: bool,
    return_type_no_wrap_resolved: bool,
    return_type_resolved: bool,
    checked_exceptions_resolved: bool,
    linked: bool,
}