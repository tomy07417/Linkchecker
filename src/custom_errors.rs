use std::error::Error;

#[derive(Debug)]
pub enum CustomError {
    UnexpectedError,
    FileNotExist(String),
    FileReadError(String),
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::FileNotExist(msg) => write!(f, "File does not exist: {}", msg),
            CustomError::FileReadError(msg) => write!(f, "Failed to read file: {}", msg),
            CustomError::UnexpectedError => write!(f, "Unexpected error occurred"),
        }
    }
}

impl Error for CustomError {}
