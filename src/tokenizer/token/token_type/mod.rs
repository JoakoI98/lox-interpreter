use std::fmt::{Debug, Display};

pub mod keyword_token;
pub mod literal_token;
pub mod single_char_token;
pub mod two_char_token;

pub use super::TokenErrors;

pub enum ArrangedTokens {
    Single(Box<dyn TokenType>),
    Multiple(Box<dyn TokenType>, Box<dyn TokenType>),
    Same,
}

pub trait TokenType: Display + Debug {
    fn literal_value(&self) -> Option<String> {
        None
    }

    fn arrange_token(&self, _lexeme: &str) -> Result<ArrangedTokens, super::TokenErrors> {
        Ok(ArrangedTokens::Same)
    }
}
