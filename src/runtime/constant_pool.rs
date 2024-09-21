use std::{cell::RefCell, rc::Rc};

use crate::{
    classfile::constant_pool::CpInfo, classpath::class_loader::ClassLoader,
    oops::klass::instance_klass::InstanceKlass,
};

pub struct RuntimeConstantPool(Rc<RefCell<RuntimeConstantPoolInner>>);

impl RuntimeConstantPool {
    pub fn new(instance_klass: InstanceKlass) -> Self {
        Rc::new(RefCell::new(RuntimeConstantPoolInner::new(
            instance_klass.get_class_loader(),
            None,
            0,
        )))
    }
}

struct RuntimeConstantPoolInner {
    _classloader: Box<ClassLoader>,
    _raw_pool: Option<Vec<CpInfo>>,
    _entry_count: usize,
}

impl RuntimeConstantPoolInner {
    pub fn new(classloader: Box<ClassLoader>, raw_pool: Option<Vec<CpInfo>>, entry_count: usize) -> Self {
        RuntimeConstantPoolInner {
            _classloader: classloader,
            _raw_pool: raw_pool,
            _entry_count: entry_count
        }
    }
}
