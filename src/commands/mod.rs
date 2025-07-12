mod evaluate;
mod parse;
mod run;
mod tokenize;

use crate::error::{InterpreterError, Result};
use crate::syntax_analysis::ParseStream;
use crate::tokenizer::{scan_tokens, ScannerError, Token};
use std::fs;
use std::io::{self, Write};

pub use evaluate::EvaluateCommand;
pub use parse::ParseCommand;
pub use run::RunCommand;
pub use tokenize::TokenizeCommand;

/// Trait for all interpreter commands
pub trait Command {
    fn run(&self, filename: &str) -> Result<()>;
}

/// Common utilities for file and token operations
pub struct CommandUtils;

impl CommandUtils {
    /// Read file contents with standardized error handling
    pub fn read_file(filename: &str) -> Result<String> {
        fs::read_to_string(filename)
            .map_err(|err| InterpreterError::io_error(filename.to_string(), err))
    }

    /// Scan tokens and return both tokens and any errors found
    pub fn scan_tokens(file_contents: &str) -> (Vec<Token>, Vec<ScannerError>) {
        if file_contents.is_empty() {
            println!("EOF  null");
            return (vec![], vec![]);
        }

        scan_tokens(file_contents)
    }

    /// Scan tokens with error checking - returns error if any scanner errors found
    pub fn scan_tokens_checked(file_contents: &str) -> Result<Vec<Token>> {
        let (tokens, errors) = Self::scan_tokens(file_contents);

        // Print all scanner errors
        for error in &errors {
            eprintln!("{}", error);
        }

        if !errors.is_empty() {
            // Return the first scanner error (they'll all have the same exit code anyway)
            return Err(errors.into_iter().next().unwrap().into());
        }

        Ok(tokens)
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
