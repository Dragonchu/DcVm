#[derive(Debug)]
pub struct OperandStack {
    max_size: usize,
    values: Vec<i32>,
}

impl OperandStack {
    pub fn new(max_size: usize) -> Self {
        OperandStack {
            max_size,
            values: Vec::with_capacity(max_size),
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