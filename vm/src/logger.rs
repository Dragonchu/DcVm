use std::sync::atomic::{AtomicBool, Ordering};

/// 全局日志控制标志
static LOG_ENABLED: AtomicBool = AtomicBool::new(true);

/// 日志控制模块
pub struct Logger;

impl Logger {
    /// 启用日志输出
    pub fn enable() {
        LOG_ENABLED.store(true, Ordering::Relaxed);
    }
    
    /// 禁用日志输出
    pub fn disable() {
        LOG_ENABLED.store(false, Ordering::Relaxed);
    }
    
    /// 检查日志是否启用
    pub fn is_enabled() -> bool {
        LOG_ENABLED.load(Ordering::Relaxed)
    }
    
    /// 打印日志（仅在启用时输出）
    pub fn log(message: &str) {
        if Self::is_enabled() {
            println!("{}", message);
        }
    }
    
    /// 打印带格式的日志（仅在启用时输出）
    pub fn log_fmt(args: std::fmt::Arguments) {
        if Self::is_enabled() {
            println!("{}", args);
        }
    }
}

/// 宏：条件打印日志
#[macro_export]
macro_rules! jvm_log {
    ($($arg:tt)*) => {
        if $crate::logger::Logger::is_enabled() {
            println!($($arg)*);
        }
    };
}

/// 宏：条件打印调试日志
#[macro_export]
macro_rules! jvm_debug {
    ($($arg:tt)*) => {
        if $crate::logger::Logger::is_enabled() {
            println!("[DEBUG] {}", format!($($arg)*));
        }
    };
} 