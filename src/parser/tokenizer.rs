use crate::error::{Result, FfffError};
use crate::interpreter::exec::{Instruction};

pub struct Tokenizer {
    tokens: Vec<String>
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new()
        }
    }

    pub fn tokenize(&mut self, input: &str) -> Result<Vec<Instruction>> {
        let mut instructions = Vec::new();
        let mut current_token = String::new();

        for c in input.chars() {
            match c {
                'f' |'u' | 'a' => current_token.push(c),
                ' ' | '\n' | '\t' | '\r' => {
                    if !current_token.is_empty() {
                        instructions.push(self.parse_token(&current_token)?);
                        current_token.clear();
                    }
                }
                _ => return Err(FfffError::InvalidToken(c.to_string()))
            }
        }

        if !current_token.is_empty() {
            instructions.push(self.parse_token(&current_token)?);
        }

        Ok(instructions)
    }

    fn parse_token(&self, token: &str) -> Result<Instruction> {
        match token {
            "fufufafa" => Ok(Instruction::Push),
            "fafafufu" => Ok(Instruction::Pop),
            "fufu" => Ok(Instruction::Add),
            "fafa" => Ok(Instruction::Subtract),
            "fu" => Ok(Instruction::Multiply),
            "fa" => Ok(Instruction::Divide),
            "f" => Ok(Instruction::PrintNum),
            "ff" => Ok(Instruction::PrintChar),
            "fff" => Ok(Instruction::Duplicate),
            "ffff" => Ok(Instruction::Swap),
            "fffff" => Ok(Instruction::Input),
            "ffffff" => Ok(Instruction::LoopStart),
            "fffffff" => Ok(Instruction::LoopEnd),
            _ => Err(FfffError::InvalidToken(token.to_string())),
        }
    }
}