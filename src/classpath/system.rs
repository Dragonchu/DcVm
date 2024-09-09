use crate::oop::klass::Klass;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct SystemDictionary {
    _classes: Arc<RwLock<HashMap<String, Arc<Klass>>>>,
}

impl SystemDictionary {
    fn new() -> Self {
        SystemDictionary {
            _classes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn find(&self, name: &str) -> Option<Arc<Klass>> {
        let classes = self._classes.read().expect("Fail to find class");
        classes.get(name).cloned()
    }

    pub fn put(&self, name: &str, klass: Arc<Klass>) {
        let mut classes = self._classes.write().expect("Fail to insert class");
        classes.insert(name.to_string(), klass);
    }

    pub fn get() -> &'static Self {
        lazy_static! {
            static ref INSTANCE: SystemDictionary = SystemDictionary::new();
        }
        &INSTANCE
    }
}
