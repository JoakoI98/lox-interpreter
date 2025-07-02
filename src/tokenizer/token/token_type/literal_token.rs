use crate::tokenizer::token::token_type::Token;

use super::{single_char_token::SingleCharToken, ArrangedTokens, TokenType, TokenValue};
use std::fmt::Display;

#[derive(Debug)]
pub enum LiteralToken {
    Identifier,
    String(String),
    Number(f64),
}

impl LiteralToken {
    fn arrange_number(&self, lexeme: &str) -> ArrangedTokens {
        let last_char = lexeme.chars().last().unwrap();
        let mut lexeme = lexeme.to_string();
        let mut single_char_token: Option<SingleCharToken> = None;
        if last_char == '.' {
            single_char_token = Some(SingleCharToken::Dot);
            lexeme.pop();
        }

        let number_token = lexeme
            .parse::<f64>()
            .ok()
            .map(|number| LiteralToken::Number(number));
        let mut tokens = ArrangedTokens::Same;

        if let Some(number_token) = number_token {
            let number_token = Box::new(number_token);
            if let Some(single_char_token) = single_char_token {
                tokens = ArrangedTokens::Multiple(number_token, Box::new(single_char_token));
            } else {
                tokens = ArrangedTokens::Single(number_token);
            }
        }

        return tokens;
    }

    fn is_valid_string(token: &str) -> bool {
        let last_char = token.chars().last().unwrap();
        return last_char == '"';
    }

    fn arrange_token(&self, lexeme: &str) -> Result<ArrangedTokens, super::TokenErrors> {
        match self {
            LiteralToken::Number(_) => Ok(self.arrange_number(lexeme)),
            LiteralToken::String(_) => {
                if Self::is_valid_string(lexeme.to_string().clone().as_str()) {
                    Ok(ArrangedTokens::Same)
                } else {
                    Err(super::TokenErrors::NotTerminatedString)
                }
            }
            _ => Ok(ArrangedTokens::Same),
        }
    }
}

impl TokenType for LiteralToken {
    fn get_value(&self) -> TokenValue {
        match self {
            LiteralToken::String(s) => TokenValue::String(s.clone()),
            LiteralToken::Number(n) => TokenValue::Number(*n),
            LiteralToken::Identifier => TokenValue::Identifier(String::new()),
        }
    }

    fn literal_value(&self) -> Option<String> {
        match self {
            LiteralToken::String(s) => Some(s.clone()),
            LiteralToken::Number(n) => Some(format!("{:?}", n)),
            _ => None,
        }
    }

    fn arrange_token(&self, lexeme: &str) -> Result<ArrangedTokens, super::TokenErrors> {
        self.arrange_token(lexeme)
    }

    fn token_type(&self) -> Token {
        match self {
            LiteralToken::Identifier => Token::Identifier,
            LiteralToken::String(_) => Token::String,
            LiteralToken::Number(_) => Token::Number,
        }
    }
}

impl Display for LiteralToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralToken::Identifier => write!(f, "IDENTIFIER"),
            LiteralToken::String(_string) => write!(f, "STRING"),
            LiteralToken::Number(_number) => write!(f, "NUMBER"),
        }
    }
}
