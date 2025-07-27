use thiserror::Error;

// Import the module-specific errors from their public exports
use crate::evaluation::{ResolverError, RuntimeError};
use crate::syntax_analysis::ParseError;
use crate::tokenizer::ScannerError;

/// Unified error type for the entire interpreter
#[derive(Error, Debug)]
pub enum InterpreterError {
    /// Errors that occur during tokenization/scanning
    #[error("{0}")]
    Scanner(#[from] ScannerError),

    /// Errors that occur during parsing
    #[error("{0}")]
    Parse(#[from] ParseError),

    /// Errors that occur during runtime evaluation
    #[error("{0}")]
    Runtime(#[from] RuntimeError),

    /// File I/O errors
    #[error("Failed to read file '{filename}': {source}")]
    Io {
        filename: String,
        #[source]
        source: std::io::Error,
    },
}

impl InterpreterError {
    /// Get the appropriate exit code for this error type
    pub fn exit_code(&self) -> i32 {
        match self {
            InterpreterError::Scanner(_) => 65,
            InterpreterError::Parse(_) => 65,
            InterpreterError::Runtime(RuntimeError::ResolverError(_)) => 65,
            InterpreterError::Runtime(_) => 70,
            InterpreterError::Io { .. } => 1,
        }
    }

    /// Create an I/O error with context
    pub fn io_error(filename: String, source: std::io::Error) -> Self {
        Self::Io { filename, source }
    }
}

pub type Result<T> = std::result::Result<T, InterpreterError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exit_codes() {
        // Test that error types map to correct exit codes
        let scanner_err = InterpreterError::Scanner(ScannerError::UnexpectedCharacter('x', 1));
        assert_eq!(scanner_err.exit_code(), 65);

        let io_err = InterpreterError::io_error(
            "test.lox".to_string(),
            std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"),
        );
        assert_eq!(io_err.exit_code(), 1);
    }
}
