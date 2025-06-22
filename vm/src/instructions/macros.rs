// 指令执行函数签名宏
#[macro_export]
macro_rules! instruction_fn {
    ($name:ident, $body:block) => {
        pub fn $name(
            thread: &mut crate::jvm_thread::JvmThread,
            method: &crate::method::Method,
            code: &[u8],
            pc: &mut usize,
            vm: Option<&mut crate::vm::Vm>,
        ) -> Result<(), crate::error::JvmError> $body
    };
}

// 指令执行函数签名宏（带下划线前缀）
#[macro_export]
macro_rules! instruction_fn_ {
    ($name:ident, $body:block) => {
        pub fn $name(
            thread: &mut crate::jvm_thread::JvmThread,
            _method: &crate::method::Method,
            _code: &[u8],
            _pc: &mut usize,
            _vm: Option<&mut crate::vm::Vm>,
        ) -> Result<(), crate::error::JvmError> $body
    };
}

// 指令执行函数签名宏（带code和pc参数）
#[macro_export]
macro_rules! instruction_fn_cp {
    ($name:ident, $body:block) => {
        pub fn $name(
            thread: &mut crate::jvm_thread::JvmThread,
            _method: &crate::method::Method,
            code: &[u8],
            pc: &mut usize,
            _vm: Option<&mut crate::vm::Vm>,
        ) -> Result<(), crate::error::JvmError> $body
    };
} 