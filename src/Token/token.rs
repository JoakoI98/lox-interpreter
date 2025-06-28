use std::fmt::Display;

use crate::Token::token_type::ArrangedTokens;

use super::{
    keyword_token::KeywordToken, literal_token::LiteralToken, single_char_token::SingleCharToken,
    token_type::TokenType, two_char_token::TwoCharToken,
};

#[derive(Debug)]
pub struct Token<T: TokenType> {
    pub(super) token_type: T,
    pub(super) lexeme: String,
    pub(super) line: usize,
    pub(super) column_start: usize,
    pub(super) column_end: usize,
}

impl<T: TokenType> Display for Token<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let literal_value: String = self.token_type.literal_value().unwrap_or("null".into());
        write!(f, "{} {} {}", self.token_type, self.lexeme, literal_value)
    }
}

impl<T: TokenType> Token<T> {
    pub fn from_str(str: &str, line: usize, column_start: usize) -> Option<Token<T>> {
        let single_char_token = SingleCharToken::from_str(str);
        if let Some(single_char_token) = single_char_token {
            return Some(Token {
                token_type: TokenType::SingleCharToken(single_char_token),
                lexeme: str.to_string(),
                line,
                column_start,
                column_end: column_start + str.len(),
            });
        }

        if let Some(two_char_token) = TwoCharToken::from_str(str) {
            return Some(Token {
                token_type: TokenType::TwoCharToken(two_char_token),
                lexeme: str.to_string(),
                line,
                column_start,
                column_end: column_start + str.len(),
            });
        }

        if let Some(keyword_token) = KeywordToken::from_str(str) {
            return Some(Token {
                token_type: TokenType::KeywordToken(keyword_token),
                lexeme: str.to_string(),
                line,
                column_start,
                column_end: column_start + str.len(),
            });
        }

        if let Some(literal_token) = LiteralToken::from_str(str) {
            return Some(Token {
                token_type: TokenType::LiteralToken(literal_token),
                lexeme: str.to_string(),
                line,
                column_start,
                column_end: column_start + str.len(),
            });
        }

        return None;
    }

    pub fn arrange_token(
        token: Token<T>,
    ) -> Result<ArrangedTokens<T, SingleCharToken>, super::scanner::ScannerError> {
        T::arrange_token(token)
    }
}
