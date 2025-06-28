use std::fmt::Display;

use crate::Token::token_type::TokenType;

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

impl SingleCharToken {
    pub fn from_str(str: &str) -> Option<SingleCharToken> {
        if str.len() != 1 {
            return None;
        }

        let c = str.chars().next().unwrap();
        match c {
            '(' => Some(SingleCharToken::LeftParen),
            ')' => Some(SingleCharToken::RightParen),
            '{' => Some(SingleCharToken::LeftBrace),
            '}' => Some(SingleCharToken::RightBrace),
            ',' => Some(SingleCharToken::Comma),
            '.' => Some(SingleCharToken::Dot),
            '-' => Some(SingleCharToken::Minus),
            '+' => Some(SingleCharToken::Plus),
            ';' => Some(SingleCharToken::Semicolon),
            '/' => Some(SingleCharToken::Slash),
            '*' => Some(SingleCharToken::Star),
            '!' => Some(SingleCharToken::Bang),
            '=' => Some(SingleCharToken::Equal),
            '>' => Some(SingleCharToken::Greater),
            '<' => Some(SingleCharToken::Less),
            '\0' => Some(SingleCharToken::Eof),
            _ => None,
        }
    }
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

impl TokenType for SingleCharToken {}
