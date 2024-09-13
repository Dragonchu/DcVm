use crate::{classfile::class_file::CpInfo, classpath::class_loader::ClassLoaderRef};

pub struct RuntimeConstantPool {
    class_loader: ClassLoaderRef,
    raw_pool: Vec<CpInfo>,
    entry_count: usize,
}
