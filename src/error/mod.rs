use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum LoxError {
    ScannerError { line: usize, message: String },
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxError::ScannerError { line, message } => {
                write!(f, "[line {}] Error: {}", line, message)
            }
        }
    }
}

impl std::error::Error for LoxError {}
