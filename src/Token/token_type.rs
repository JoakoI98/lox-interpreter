use std::fmt::Display;

use crate::Token::{
    keyword_token::KeywordToken, literal_token::LiteralToken, single_char_token::SingleCharToken,
    two_char_token::TwoCharToken,
};

#[derive(Debug)]
pub enum TokenType {
    SingleCharToken(SingleCharToken),
    TwoCharToken(TwoCharToken),
    LiteralToken(LiteralToken),
    KeywordToken(KeywordToken),
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::SingleCharToken(token) => write!(f, "{}", token),
            TokenType::TwoCharToken(token) => write!(f, "{}", token),
            TokenType::LiteralToken(token) => write!(f, "{}", token),
            TokenType::KeywordToken(token) => write!(f, "{}", token),
        }
    }
}
