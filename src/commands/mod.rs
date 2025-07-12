mod evaluate;
mod parse;
mod run;
mod tokenize;

use crate::syntax_analysis::ParseStream;
use crate::tokenizer::{scan_tokens, ScannerError, Token};
use std::fs;
use std::io::{self, Write};

pub use evaluate::EvaluateCommand;
pub use parse::ParseCommand;
pub use run::RunCommand;
pub use tokenize::TokenizeCommand;

/// Common result type for all commands
pub type CommandResult = Result<(), i32>;

/// Trait for all interpreter commands
pub trait Command {
    fn run(&self, filename: &str) -> CommandResult;
}

/// Common utilities for file and token operations
pub struct CommandUtils;

impl CommandUtils {
    /// Read file contents with standardized error handling
    pub fn read_file(filename: &str) -> Result<String, i32> {
        fs::read_to_string(filename).map_err(|err| {
            writeln!(io::stderr(), "Failed to read file {}: {}", filename, err).unwrap();
            1 // Generic error code
        })
    }

    /// Scan tokens with standardized error handling
    pub fn scan_tokens_checked(file_contents: &str) -> Result<Vec<Token>, i32> {
        let (tokens, errors) = self::scan_tokens(file_contents);

        // Print all scanner errors
        for error in &errors {
            eprintln!("{}", error);
        }

        if !errors.is_empty() {
            return Err(65); // Scanner error exit code
        }

        Ok(tokens)
    }

    pub fn scan_tokens(file_contents: &str) -> (Vec<Token>, Vec<ScannerError>) {
        if file_contents.is_empty() {
            println!("EOF  null");
            return (vec![], vec![]);
        }

        let (tokens, errors) = scan_tokens(file_contents);

        (tokens, errors)
    }

    /// Create a parse stream from tokens
    pub fn create_parse_stream(tokens: Vec<Token>) -> ParseStream {
        ParseStream::new(tokens)
    }

    /// Log debug information to stderr
    pub fn log_debug(message: &str) {
        writeln!(io::stderr(), "{}", message).unwrap();
    }
}
