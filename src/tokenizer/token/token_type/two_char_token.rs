use std::fmt::Display;

use super::{Token, TokenType};

#[derive(Debug)]
pub enum TwoCharToken {
    BangEqual,
    EqualEqual,
    GreaterEqual,
    LessEqual,
}

impl Display for TwoCharToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TwoCharToken::BangEqual => write!(f, "BANG_EQUAL"),
            TwoCharToken::EqualEqual => write!(f, "EQUAL_EQUAL"),
            TwoCharToken::GreaterEqual => write!(f, "GREATER_EQUAL"),
            TwoCharToken::LessEqual => write!(f, "LESS_EQUAL"),
        }
    }
}

impl TokenType for TwoCharToken {
    fn token_type(&self) -> Token {
        match self {
            TwoCharToken::BangEqual => Token::BangEqual,
            TwoCharToken::EqualEqual => Token::EqualEqual,
            TwoCharToken::GreaterEqual => Token::GreaterEqual,
            TwoCharToken::LessEqual => Token::LessEqual,
        }
    }
}
