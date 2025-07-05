mod scanner;
mod token;

pub use scanner::scan_tokens;
pub use token::{StaticToken as Token, TokenEnum};
