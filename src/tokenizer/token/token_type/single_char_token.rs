use std::fmt::Display;

use super::{Token, TokenType};

#[derive(Debug)]
pub enum SingleCharToken {
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
}

impl Display for SingleCharToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SingleCharToken::LeftParen => write!(f, "LEFT_PAREN"),
            SingleCharToken::RightParen => write!(f, "RIGHT_PAREN"),
            SingleCharToken::LeftBrace => write!(f, "LEFT_BRACE"),
            SingleCharToken::RightBrace => write!(f, "RIGHT_BRACE"),
            SingleCharToken::Comma => write!(f, "COMMA"),
            SingleCharToken::Dot => write!(f, "DOT"),
            SingleCharToken::Minus => write!(f, "MINUS"),
            SingleCharToken::Plus => write!(f, "PLUS"),
            SingleCharToken::Semicolon => write!(f, "SEMICOLON"),
            SingleCharToken::Slash => write!(f, "SLASH"),
            SingleCharToken::Star => write!(f, "STAR"),
            SingleCharToken::Bang => write!(f, "BANG"),
            SingleCharToken::Equal => write!(f, "EQUAL"),
            SingleCharToken::Greater => write!(f, "GREATER"),
            SingleCharToken::Less => write!(f, "LESS"),
            SingleCharToken::Eof => write!(f, "EOF"),
        }
    }
}

impl TokenType for SingleCharToken {
    fn token_type(&self) -> Token {
        match self {
            SingleCharToken::LeftParen => Token::LeftParen,
            SingleCharToken::RightParen => Token::RightParen,
            SingleCharToken::LeftBrace => Token::LeftBrace,
            SingleCharToken::RightBrace => Token::RightBrace,
            SingleCharToken::Comma => Token::Comma,
            SingleCharToken::Dot => Token::Dot,
            SingleCharToken::Minus => Token::Minus,
            SingleCharToken::Plus => Token::Plus,
            SingleCharToken::Semicolon => Token::Semicolon,
            SingleCharToken::Slash => Token::Slash,
            SingleCharToken::Star => Token::Star,
            SingleCharToken::Bang => Token::Bang,
            SingleCharToken::Equal => Token::Equal,
            SingleCharToken::Greater => Token::Greater,
            SingleCharToken::Less => Token::Less,
            SingleCharToken::Eof => Token::Eof,
        }
    }
}
