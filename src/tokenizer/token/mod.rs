use std::fmt::Display;

mod static_token;
mod token_type;

use thiserror::Error;
use token_type::ArrangedTokens;
use token_type::PARSERS;

pub use static_token::StaticToken;
pub use token_type::{EOFToken, Token as TokenEnum, TokenType, TokenValue};

#[derive(Error, Debug)]
pub enum TokenErrors {
    #[error("Error: Unterminated string.")]
    NotTerminatedString,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: Box<dyn TokenType>,
    pub(super) lexeme: String,
    pub(super) line: usize,
    pub(super) column_start: usize,
    pub(super) column_end: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let literal_value: String = self.token_type.literal_value().unwrap_or("null".into());
        write!(f, "{} {} {}", self.token_type, self.lexeme, literal_value)
    }
}

impl Token {
    pub fn from_str(str: &str, line: usize, column_start: usize) -> Option<Token> {
        let mut token: Option<Box<dyn TokenType>> = None;
        for &parser in PARSERS.iter() {
            token = parser.parse_string(str);
            if token.is_some() {
                break;
            }
        }

        if token.is_none() {
            return None;
        }

        let token = token.unwrap();
        Some(Token {
            token_type: token,
            lexeme: str.to_string(),
            line,
            column_start,
            column_end: column_start + str.len(),
        })
    }

    pub fn arrange_token(
        token: Token,
    ) -> Result<(Self, Option<Self>), super::scanner::ScannerError> {
        let arranged_token_types = token.token_type.arrange_token(&token.lexeme);
        if let Err(error) = arranged_token_types {
            match error {
                TokenErrors::NotTerminatedString => {
                    return Err(super::scanner::ScannerError::NotTerminatedString(
                        token.line,
                    ));
                }
            }
        }

        let arranged_token_types = arranged_token_types.unwrap();

        match arranged_token_types {
            ArrangedTokens::Same => Ok((token, None)),
            ArrangedTokens::Single(token_type) => {
                let new_token = Token {
                    token_type,
                    lexeme: token.lexeme,
                    line: token.line,
                    column_start: token.column_start,
                    column_end: token.column_end,
                };
                Ok((new_token, None))
            }
            ArrangedTokens::Multiple(first_type, second_type) => {
                let first_token = Token {
                    token_type: first_type,
                    lexeme: token.lexeme.clone(),
                    line: token.line,
                    column_start: token.column_start,
                    column_end: token.column_end,
                };
                let second_token = Token {
                    token_type: second_type,
                    lexeme: token.lexeme,
                    line: token.line,
                    column_start: token.column_start,
                    column_end: token.column_end,
                };
                Ok((first_token, Some(second_token)))
            }
        }
    }
}
