use std::fmt::Display;

use super::{Token, TokenType};

#[derive(Debug)]
pub enum KeywordToken {
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Return,
    Super,
    Print,
    This,
    True,
    Var,
    While,
}

impl Display for KeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeywordToken::And => write!(f, "AND"),
            KeywordToken::Class => write!(f, "CLASS"),
            KeywordToken::Else => write!(f, "ELSE"),
            KeywordToken::False => write!(f, "FALSE"),
            KeywordToken::Fun => write!(f, "FUN"),
            KeywordToken::For => write!(f, "FOR"),
            KeywordToken::If => write!(f, "IF"),
            KeywordToken::Nil => write!(f, "NIL"),
            KeywordToken::Or => write!(f, "OR"),
            KeywordToken::Return => write!(f, "RETURN"),
            KeywordToken::Super => write!(f, "SUPER"),
            KeywordToken::Print => write!(f, "PRINT"),
            KeywordToken::This => write!(f, "THIS"),
            KeywordToken::True => write!(f, "TRUE"),
            KeywordToken::Var => write!(f, "VAR"),
            KeywordToken::While => write!(f, "WHILE"),
        }
    }
}

impl TokenType for KeywordToken {
    fn token_type(&self) -> Token {
        match self {
            KeywordToken::And => Token::And,
            KeywordToken::Class => Token::Class,
            KeywordToken::Else => Token::Else,
            KeywordToken::False => Token::False,
            KeywordToken::Fun => Token::Fun,
            KeywordToken::For => Token::For,
            KeywordToken::If => Token::If,
            KeywordToken::Nil => Token::Nil,
            KeywordToken::Or => Token::Or,
            KeywordToken::Return => Token::Return,
            KeywordToken::Super => Token::Super,
            KeywordToken::Print => Token::Print,
            KeywordToken::This => Token::This,
            KeywordToken::True => Token::True,
            KeywordToken::Var => Token::Var,
            KeywordToken::While => Token::While,
        }
    }
}
