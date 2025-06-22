use std::fmt;

#[derive(Debug)]
pub enum JvmError {
    ArithmeticError(String),
    NullPointerError(String),
    ClassNotFoundError(String),
    NoSuchMethodError(String),
    IllegalStateError(String),
    StackOverflowError(String),
    OutOfMemoryError(String),
    Unimplemented(String),
}

impl fmt::Display for JvmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JvmError::ArithmeticError(msg) => write!(f, "ArithmeticError: {}", msg),
            JvmError::NullPointerError(msg) => write!(f, "NullPointerError: {}", msg),
            JvmError::ClassNotFoundError(msg) => write!(f, "ClassNotFoundError: {}", msg),
            JvmError::NoSuchMethodError(msg) => write!(f, "NoSuchMethodError: {}", msg),
            JvmError::IllegalStateError(msg) => write!(f, "IllegalStateError: {}", msg),
            JvmError::StackOverflowError(msg) => write!(f, "StackOverflowError: {}", msg),
            JvmError::OutOfMemoryError(msg) => write!(f, "OutOfMemoryError: {}", msg),
            JvmError::Unimplemented(msg) => write!(f, "Unimplemented: {}", msg),
        }
    }
}

impl std::error::Error for JvmError {} 