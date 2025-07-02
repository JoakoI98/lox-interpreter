use std::fmt::{Debug, Display};

mod parser;

mod keyword_token;
mod literal_token;
mod single_char_token;
mod two_char_token;

pub use super::TokenErrors;

pub use parser::PARSERS;
pub use single_char_token::SingleCharToken::Eof as EOFToken;

pub enum ArrangedTokens {
    Single(Box<dyn TokenType>),
    Multiple(Box<dyn TokenType>, Box<dyn TokenType>),
    Same,
}

pub enum TokenValue {
    Number(f64),
    String(String),
    Identifier(String),
    None,
}

pub trait TokenType: Display + Debug {
    fn token_type(&self) -> Token;

    fn is_token(&self, token_type: Token) -> bool {
        self.token_type() == token_type
    }

    fn get_value(&self) -> TokenValue {
        TokenValue::None
    }

    fn literal_value(&self) -> Option<String> {
        None
    }

    fn arrange_token(&self, _lexeme: &str) -> Result<ArrangedTokens, super::TokenErrors> {
        Ok(ArrangedTokens::Same)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Token {
    // SingleCharToken
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    Equal,
    Greater,
    Less,
    Eof,

    // TwoCharToken
    BangEqual,
    EqualEqual,
    GreaterEqual,
    LessEqual,

    // KeywordToken
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // LiteralToken
    Number,
    String,
    Identifier,
}
