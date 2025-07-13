mod scanner;
mod token;

pub use scanner::{scan_tokens, ScannerError};
pub use token::{StaticToken as Token, TokenEnum, TokenValue};
