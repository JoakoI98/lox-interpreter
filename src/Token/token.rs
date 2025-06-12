use std::fmt::Display;

use super::{
    literal_token::LiteralToken, single_char_token::SingleCharToken, token_type::TokenType,
};

#[derive(Debug)]
pub struct Token {
    pub(super) token_type: TokenType,
    pub(super) lexeme: String,
    pub(super) line: usize,
    pub(super) column_start: usize,
    pub(super) column_end: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let literal_value = match &self.token_type {
            TokenType::LiteralToken(LiteralToken::String(s)) => s.clone(),
            TokenType::LiteralToken(LiteralToken::Number(n)) => n.to_string(),
            _ => "null".to_string(),
        };

        write!(f, "{} {} {}", self.token_type, self.lexeme, literal_value)
    }
}

impl Token {
    pub fn from_str(str: &str, line: usize, column_start: usize) -> Option<Token> {
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

        return None;
    }
}
