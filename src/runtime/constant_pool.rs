use std::sync::Arc;

use crate::{classfile::class_file::CpInfo, classpath::class_loader::ClassLoader};

pub struct RuntimeConstantPool {
    class_loader: Arc<dyn ClassLoader>,
    raw_pool: Vec<CpInfo>,
    entry_count: usize,
}