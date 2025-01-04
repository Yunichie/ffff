use thiserror::Error;

#[derive(Error, Debug)]
pub enum FfffError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("StackUnderflow")]
    StackUnderflow,
    #[error("Invalid program: {0}")]
    InvalidProgram(String),
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    #[error("Division by zero")]
    DivisionByZero
}

pub type Result<T> = std::result::Result<T, FfffError>;