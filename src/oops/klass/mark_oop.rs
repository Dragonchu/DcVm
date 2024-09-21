use std::{cell::RefCell, rc::Rc};

use crate::{jni::jni_md::jint, oops::oop::OopType};

pub struct MarkOop(Rc<RefCell<MarkOopDesc>>);
struct MarkOopDesc {
    _type: OopType,
    _hash: jint,
}

impl MarkOop {
    pub fn new(oop_type: OopType) -> MarkOop {
        MarkOop(Rc::new(RefCell::new(MarkOopDesc {
            _type: oop_type,
            _hash: 0,
        })))
    }
    pub fn get_oop_type(&self) -> OopType {
        self.0.borrow()._type
    }

    pub fn set_hash(&self, hash: jint) {
        self.0.borrow_mut()._hash = hash;
    }

    pub fn get_hash(&self) -> jint {
        self.0.borrow()._hash
    }
}
