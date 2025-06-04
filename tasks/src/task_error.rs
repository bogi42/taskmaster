use rustyline::error::ReadlineError;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Task with id {0} not found")]
    TaskNotFound(usize),
    #[error("File I/O error: {0}")]
    Io(#[from] io::Error), //automatically convert from io::Error to TaskError::Io
    #[error("Field '{0}' needs a value, please provide one")]
    Empty(String),
    #[error("Error parsing/serializing JSON data: {0}")]
    Json(#[from] serde_json::Error),
    #[error("An unknown error occured: {0}")]
    Unknown(String),
    #[error("Interactive input error: {0}")]
    Readline(#[from] ReadlineError),
    #[error("User input was cancelled")]
    InputCancelled,
    #[error("Argument mismatch: {0}")]
    ArgumentMismatch(String),
}
