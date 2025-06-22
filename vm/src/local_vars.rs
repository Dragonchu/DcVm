use crate::JvmValue;

#[derive(Debug)]
pub struct LocalVars {
    max_locals: usize,
    values: Vec<JvmValue>,
}

impl LocalVars {
    pub fn new(max_locals: usize) -> Self {
        LocalVars {
            max_locals,
            values: vec![JvmValue::Null; max_locals],
        }
    }

    pub fn get_int(&self, index: usize) -> i32 {
        if index >= self.max_locals {
            panic!("Local variable index out of bounds");
        }
        match self.values[index] {
            JvmValue::Int(v) => v as i32,
            _ => panic!("Not an int at index {}", index),
        }
    }

    pub fn set_int(&mut self, index: usize, value: i32) {
        if index >= self.max_locals {
            panic!("Local variable index out of bounds");
        }
        self.values[index] = JvmValue::Int(value as u32);
    }

    pub fn set_obj_ref(&mut self, index: usize, obj_ref: crate::heap::RawPtr) {
        if index >= self.max_locals {
            panic!("Local variable index out of bounds");
        }
        self.values[index] = JvmValue::ObjRef(obj_ref);
    }

    pub fn get_obj_ref(&self, index: usize) -> crate::heap::RawPtr {
        if index >= self.max_locals {
            panic!("Local variable index out of bounds");
        }
        match self.values[index] {
            JvmValue::ObjRef(ptr) => ptr,
            JvmValue::Null => crate::heap::RawPtr(std::ptr::null_mut()),
            _ => panic!("Not an object reference at index {}", index),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get_int() {
        let mut locals = LocalVars::new(10);
        locals.set_int(0, 42);
        assert_eq!(locals.get_int(0), 42);
    }

    #[test]
    fn test_multiple_variables() {
        let mut locals = LocalVars::new(10);
        locals.set_int(0, 1);
        locals.set_int(1, 2);
        locals.set_int(2, 3);
        assert_eq!(locals.get_int(0), 1);
        assert_eq!(locals.get_int(1), 2);
        assert_eq!(locals.get_int(2), 3);
    }

    #[test]
    #[should_panic(expected = "Local variable index out of bounds")]
    fn test_index_out_of_bounds_get() {
        let locals = LocalVars::new(5);
        locals.get_int(5); // 应该触发越界错误
    }

    #[test]
    #[should_panic(expected = "Local variable index out of bounds")]
    fn test_index_out_of_bounds_set() {
        let mut locals = LocalVars::new(5);
        locals.set_int(5, 42); // 应该触发越界错误
    }
} 