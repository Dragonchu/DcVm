use crate::method::Method;
use crate::types::U4;

pub struct Frame {
    _method: Method,
    _native_frame: bool,
    _exception_thrown_here: bool,
    _return_pc: U4,
}
