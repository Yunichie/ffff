use crate::error::{Result, FfffError};

#[derive(Default)]
pub struct Stack {
    stack: Vec<i64>
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new()
        }
    }

    pub fn push(&mut self, value: i64) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Result<i64> {
        self.stack.pop().ok_or(FfffError::StackUnderflow)
    }

    pub fn peek(&self) -> Result<i64> {
        self.stack.last().copied().ok_or(FfffError::StackUnderflow)
    }

    pub fn duplicate(&mut self) -> Result<()> {
        let value = self.peek()?;
        self.push(value);
        Ok(())
    }

    pub fn swap(&mut self) -> Result<()> {
        let a = self.pop()?;
        let b = self.pop()?;
        self.push(a);
        self.push(b);
        Ok(())
    }
}