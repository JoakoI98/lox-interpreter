use std::fmt::Display;

use crate::tokenizer::{Token, TokenEnum};

#[derive(Debug, PartialEq, Clone)]
pub enum ExpectedEnum {
    Token(TokenEnum),
    Tokens(Vec<TokenEnum>),
    Unknown,
}

impl Display for ExpectedEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpectedEnum::Token(token) => write!(f, "Expected: '{}'", token),
            ExpectedEnum::Tokens(tokens) => {
                let expected_tokens_string = tokens
                    .iter()
                    .map(|token| format!("'{}'", token))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "Expected one of: {}", expected_tokens_string)
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
        let message_string = self
            .message
            .as_ref()
            .map(|message| format!(" {}: ", message))
            .unwrap_or("".to_string());

        let expected_string: String;
        match self.expected {
            ExpectedEnum::Unknown => {
                return write!(
                    f,
                    "Unexpected token found: '{}'.{}",
                    self.token.token_type, message_string
                );
            }
            _ => expected_string = self.expected.to_string(),
        }
        write!(
            f,
            "{}. Found: '{}'.{}",
            expected_string, self.token.token_type, message_string
        )
    }
}

impl UnexpectedTokenError {
    pub fn unexpected_token(token: Token, expected: TokenEnum, message: Option<String>) -> Self {
        Self {
            message: message,
            expected: ExpectedEnum::Token(expected),
            token,
        }
    }
}

struct NoTokenError {
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

pub enum ParseError {
    UnexpectedToken(UnexpectedTokenError),
    NoToken(NoTokenError),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken(error) => write!(f, "{}", error),
            ParseError::NoToken(error) => write!(f, "{}", error),
        }
    }
}

impl ParseError {
    pub fn unexpected_token(token: Token, expected: TokenEnum, message: Option<String>) -> Self {
        Self::UnexpectedToken(UnexpectedTokenError::unexpected_token(
            token, expected, message,
        ))
    }

    pub fn no_token(expected: ExpectedEnum, message: Option<String>) -> Self {
        Self::NoToken(NoTokenError::no_token(expected, message))
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;
