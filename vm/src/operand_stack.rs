use crate::heap::RawPtr;

#[derive(Debug)]
pub struct OperandStack {
    max_size: usize,
    values: Vec<i32>,
    obj_refs: Vec<RawPtr>, // 存储对象引用
}

impl OperandStack {
    pub fn new(max_size: usize) -> Self {
        OperandStack {
            max_size,
            values: Vec::with_capacity(max_size),
            obj_refs: Vec::with_capacity(max_size),
        }
    }

    pub fn push_int(&mut self, value: i32) {
        if self.values.len() >= self.max_size {
            panic!("Stack overflow");
        }
        self.values.push(value);
    }

    pub fn pop_int(&mut self) -> i32 {
        self.values.pop().expect("Stack underflow")
    }

    pub fn push_null(&mut self) {
        self.push_int(0);
    }
    
    pub fn push_obj_ref(&mut self, obj_ref: RawPtr) {
        if self.obj_refs.len() >= self.max_size {
            panic!("Stack overflow");
        }
        self.obj_refs.push(obj_ref);
    }
    
    pub fn pop_obj_ref(&mut self) -> RawPtr {
        self.obj_refs.pop().expect("Stack underflow")
    }
    
    /// 查看栈顶的整数值，但不弹出
    pub fn peek_int(&self) -> i32 {
        *self.values.last().expect("Stack underflow")
    }
    
    /// 查看栈顶的对象引用，但不弹出
    pub fn peek_obj_ref(&self) -> RawPtr {
        *self.obj_refs.last().expect("Stack underflow")
    }
    
    /// 检查整数值栈是否为空
    pub fn is_values_empty(&self) -> bool {
        self.values.is_empty()
    }
    
    /// 检查对象引用栈是否为空
    pub fn is_obj_refs_empty(&self) -> bool {
        self.obj_refs.is_empty()
    }
    
    /// 检查整个操作数栈是否为空
    pub fn is_empty(&self) -> bool {
        self.values.is_empty() && self.obj_refs.is_empty()
    }

    /// 交换栈顶两个整数
    pub fn swap_top_two_ints(&mut self) {
        let len = self.values.len();
        if len >= 2 {
            self.values.swap(len - 1, len - 2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop_int() {
        let mut stack = OperandStack::new(10);
        stack.push_int(42);
        assert_eq!(stack.pop_int(), 42);
    }

    #[test]
    fn test_push_null() {
        let mut stack = OperandStack::new(10);
        stack.push_null();
        assert_eq!(stack.pop_int(), 0);
    }

    #[test]
    #[should_panic(expected = "Stack overflow")]
    fn test_stack_overflow() {
        let mut stack = OperandStack::new(2);
        stack.push_int(1);
        stack.push_int(2);
        stack.push_int(3); // 应该触发栈溢出
    }

    #[test]
    #[should_panic(expected = "Stack underflow")]
    fn test_stack_underflow() {
        let mut stack = OperandStack::new(10);
        stack.pop_int(); // 应该触发栈下溢
    }
} 