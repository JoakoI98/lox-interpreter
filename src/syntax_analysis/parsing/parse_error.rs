use std::fmt::Display;

use crate::tokenizer::{Token, TokenEnum};

use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum ExpectedEnum {
    Token(TokenEnum),
    Tokens(Vec<TokenEnum>),
    NonTerminal(String),
    Unknown,
}

impl Display for ExpectedEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpectedEnum::Token(token) => write!(f, "{}", token),
            ExpectedEnum::NonTerminal(non_terminal) => write!(f, "{}", non_terminal.to_lowercase()),
            ExpectedEnum::Tokens(tokens) => {
                let expected_tokens_string = tokens
                    .iter()
                    .map(|token| format!("'{}'", token))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{}", expected_tokens_string)
            }
            ExpectedEnum::Unknown => write!(f, ""),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnexpectedTokenError {
    message: Option<String>,
    expected: ExpectedEnum,
    token: Token,
}

impl Display for UnexpectedTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expected_string = self.expected.to_string();
        let at_string = if self.token.token_type == TokenEnum::Eof {
            "end".to_string()
        } else {
            format!("'{}'", self.token.lexeme)
        };
        write!(
            f,
            "[line {}] Error at {}: Expect '{}'.",
            self.token.line, at_string, expected_string
        )
    }
}

impl std::error::Error for UnexpectedTokenError {}

impl UnexpectedTokenError {
    pub fn unexpected_token(token: Token, expected: ExpectedEnum, message: Option<String>) -> Self {
        Self {
            message: message,
            expected,
            token,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct NoTokenError {
    expected: ExpectedEnum,
    message: Option<String>,
}

impl Display for NoTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message_string = self
            .message
            .as_ref()
            .map(|message| format!(" {}: ", message))
            .unwrap_or("".to_string());

        let mut expected_string = self.expected.to_string();
        if !expected_string.is_empty() {
            expected_string = format!(" {}.", expected_string);
        }

        write!(
            f,
            "Unexpected end of file.{}{}",
            expected_string, message_string
        )
    }
}

impl NoTokenError {
    pub fn no_token(expected: ExpectedEnum, message: Option<String>) -> Self {
        Self { expected, message }
    }
}

impl std::error::Error for NoTokenError {}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("{0}")]
    UnexpectedToken(#[from] UnexpectedTokenError),
    #[error("{0}")]
    NoToken(#[from] NoTokenError),
}

pub type Result<T> = std::result::Result<T, ParseError>;

impl ParseError {
    pub fn found_token(&self) -> Option<Token> {
        match self {
            ParseError::UnexpectedToken(error) => Some(error.token.clone()),
            ParseError::NoToken(_) => None,
        }
    }
}
