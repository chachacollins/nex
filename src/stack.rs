pub type Value = f64;

pub struct Stack {
    items: [Value; 1024],
    stack_top: u16,
}

#[derive(Debug)]
pub enum StackError {
    Underflow,
    Overflow,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            items: [0.0; 1024],
            stack_top: 0,
        }
    }

    pub fn push(&mut self, value: Value) -> Result<(), StackError> {
        if self.stack_top as usize >= self.items.len() {
            return Err(StackError::Overflow);
        }
        self.items[self.stack_top as usize] = value;
        self.stack_top += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Value, StackError> {
        if self.stack_top <= 0 {
            return Err(StackError::Underflow);
        }
        self.stack_top -= 1;
        Ok(self.items[self.stack_top as usize])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
        stack.push(1.).unwrap();
        stack.push(2.).unwrap();
        stack.push(3.).unwrap();
        let _ = stack.pop().unwrap();
        assert_eq!(stack.stack_top, 2);
    }
}
