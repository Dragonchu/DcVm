// 指令类型定义
pub mod types;
pub use types::Instruction;

// 宏定义
pub mod macros;

// 指令分发表
pub mod dispatch;

// 共享工具模块
pub mod method_utils;

// 指令模块
pub mod aload_0;
pub mod invokespecial;
pub mod invokevirtual;

// 新增的指令模块
pub mod constants;
pub mod load_store;
pub mod arithmetic;
pub mod control;
pub mod stack;
pub mod field_ops;
pub mod ldc_ops;
pub mod object_ops;
pub mod control_extended;
pub mod invokestatic;
pub mod array_ops;
pub mod iinc;
// ... 其他指令模块按需添加 