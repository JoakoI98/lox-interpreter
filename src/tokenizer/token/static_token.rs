use std::fmt::{format, Display};

pub use super::token_type::{ArrangedTokens, Token as TokenEnum, TokenType, TokenValue};
use super::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct StaticToken {
    pub token_type: TokenEnum,
    pub lexeme: String,
    pub line: usize,
    pub column_start: usize,
    pub column_end: usize,
    pub token_value: TokenValue,
    literal_display: Option<String>,
    string_display: String,
}

impl From<Token> for StaticToken {
    fn from(token: Token) -> Self {
        let string_display = format!("{}", token);
        StaticToken {
            token_type: token.token_type.token_type(),
            lexeme: token.lexeme,
            line: token.line,
            column_start: token.column_start,
            column_end: token.column_end,
            token_value: token.token_type.get_value(),
            literal_display: token.token_type.literal_value(),
            string_display,
        }
    }
}

impl From<&Token> for StaticToken {
    fn from(token: &Token) -> Self {
        let string_display = format!("{}", token);
        StaticToken {
            token_type: token.token_type.token_type(),
            lexeme: token.lexeme.clone(),
            line: token.line,
            column_start: token.column_start,
            column_end: token.column_end,
            token_value: token.token_type.get_value(),
            literal_display: token.token_type.literal_value(),
            string_display,
        }
    }
}

impl FromIterator<Token> for Vec<StaticToken> {
    fn from_iter<T: IntoIterator<Item = Token>>(iter: T) -> Self {
        iter.into_iter()
            .map(|token| {
                let static_token: StaticToken = token.into();
                static_token
            })
            .collect()
    }
}

impl Display for StaticToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string_display)
    }
}
