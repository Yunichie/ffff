use crate::error::{FfffError, Result};
use crate::interpreter::stack::Stack;
use std::io::{self, Write};

#[derive(Debug)]
pub enum Instruction {
    Push,             // fufufafa
    Pop,              // fafafufu
    Add,              // fufu
    Subtract,         // fafa
    Multiply,         // fu
    Divide,           // fa
    PrintNum,         // f
    PrintChar,        // ff
    Duplicate,        // fff
    Swap,             // ffff
    Input,            // fffff
    LoopStart,        // ffffff
    LoopEnd,          // fffffff
}

pub struct Exec {
    stack: Stack,
    instruction_pointer: usize,
    loop_stack: Vec<usize>
}

impl Exec {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
            instruction_pointer: 0,
            loop_stack: Vec::new()
        }
    }

    pub fn execute(&mut self, instruction: &Instruction) -> Result<()> {
        match instruction {
            Instruction::Push => Ok(self.stack.push(1)),
            Instruction::Pop => {
                self.stack.pop()?;
                Ok(())
            },
            Instruction::Add => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(a + b);
                Ok(())
            },
            Instruction::Subtract => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(b - a);
                Ok(())
            },
            Instruction::Multiply => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(a * b);
                Ok(())
            },
            Instruction::Divide => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                if a == 0 {
                    return Err(crate::error::FfffError::DivisionByZero);
                }
                self.stack.push(b / a);
                Ok(())
            },
            Instruction::PrintNum => {
                let value = self.stack.peek()?;
                print!("{}", value);
                io::stdout().flush()?;
                Ok(())
            },
            Instruction::PrintChar => {
                let value = self.stack.peek()?;
                print!("{}", value as u8 as char);
                io::stdout().flush()?;
                Ok(())
            },
            Instruction::Duplicate => Ok(self.stack.duplicate()?),
            Instruction::Swap => Ok(self.stack.swap()?),
            Instruction::Input => {
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let value = input.trim().parse().map_err(|_| FfffError::InvalidProgram(input))?;
                self.stack.push(value);
                Ok(())
            },
            _ => Ok(())
        }
    }

    pub fn execute_loop(&mut self, instructions: &[Instruction]) -> Result<()> {
        self.instruction_pointer = 0;
        while self.instruction_pointer < instructions.len() {
            let instruction = &instructions[self.instruction_pointer];
            match instruction {
                Instruction::LoopStart => {
                    let is_zero = self.stack.peek()? == 0;
                    if is_zero {
                        self.instruction_pointer = self.find_matching_loop_end(instructions, self.instruction_pointer)?;
                    } else {
                        self.loop_stack.push(self.instruction_pointer);
                    }
                },
                Instruction::LoopEnd => {
                    if self.stack.peek()? == 0 {
                        self.loop_stack.pop();
                    } else {
                        self.instruction_pointer = *self.loop_stack.last().ok_or(FfffError::InvalidProgram("No matching LoopStart".to_string()))?;
                    }
                },
                _ => {
                    self.execute(instruction)?;
                }
            }
            self.instruction_pointer += 1;
        }
        Ok(())
    }

    fn find_matching_loop_end(&self, instructions: &[Instruction], start_pos: usize) -> Result<usize> {
        let mut nesting_level = 1;
        let mut current_pos = start_pos + 1;

        while current_pos < instructions.len() {
            match &instructions[current_pos] {
                Instruction::LoopStart => nesting_level += 1,
                Instruction::LoopEnd => {
                    nesting_level -= 1;
                    if nesting_level == 0 {
                        return Ok(current_pos);
                    }
                }
                _ => {}
            }
            current_pos += 1;
        }

        Err(FfffError::InvalidProgram("No matching loop end found".to_string()))
    }
}