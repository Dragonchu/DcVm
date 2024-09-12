use crate::{classfile::class_file::CpInfo, classpath::class_loader::ClassLoader};

pub struct RuntimeConstantPool {
    class_loader: Option<ClassLoader>,
    raw_pool: Vec<CpInfo>,
    entry_count: usize,
}