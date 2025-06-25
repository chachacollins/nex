use miette::{Diagnostic, Result};
use thiserror::Error;
pub type Value = (u8, f64);

pub struct Stack {
    items: [Value; 1024],
    stack_top: u16,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Stack Overflow")]
#[diagnostic(
    help("Please report this issue to the maintainer"),
    url("https://chachacollins.com")
)]
struct StackOverflow {}

#[derive(Error, Debug, Diagnostic)]
#[error("Stack Underflow")]
#[diagnostic(
    help("Please report this issue to the maintainer"),
    url("https://chachacollins.com")
)]
struct StackUnderflow {}

impl Stack {
    pub const fn new() -> Self {
        Self {
            items: [(0, 0.0); 1024],
            stack_top: 0,
        }
    }

    pub fn push(&mut self, value: Value) -> Result<()> {
        if self.stack_top as usize >= self.items.len() {
            return Err(StackOverflow {})?;
        }
        self.items[self.stack_top as usize] = value;
        self.stack_top += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Value> {
        if self.stack_top == 0 {
            return Err(StackUnderflow {})?;
        }
        self.stack_top -= 1;
        Ok(self.items[self.stack_top as usize])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stack_operations() {
        let mut stack = Stack::new();
        stack.push((0, 1.)).unwrap();
        stack.push((0, 2.)).unwrap();
        stack.push((0, 3.)).unwrap();
        let _ = stack.pop().unwrap();
        assert_eq!(stack.stack_top, 2);
    }
}
