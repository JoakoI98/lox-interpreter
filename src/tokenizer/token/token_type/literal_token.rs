use super::{single_char_token::SingleCharToken, ArrangedTokens, TokenType};
use regex::Regex;
use std::fmt::Display;

#[derive(Debug)]
pub enum LiteralToken {
    Identifier,
    String(String),
    Number(f64),
}

impl LiteralToken {
    fn parse_number(str: &str) -> Option<f64> {
        let number_regex = Regex::new(r"^[0-9]+(\.[0-9]*)?$").unwrap();

        if number_regex.is_match(str) {
            return str.parse::<f64>().ok();
        }

        None
    }

    fn parse_identifier(str: &str) -> Option<()> {
        let identifier_regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();

        if identifier_regex.is_match(str) {
            return Some(());
        }

        None
    }

    fn parse_string(str: &str) -> Option<String> {
        let mut chars = str.chars();
        let quote_size = 'c'.len_utf8();
        if let Some(c) = chars.next() {
            if c != '"' {
                return None;
            }
        } else {
            return None;
        }

        while let Some(c) = chars.next() {
            if c == '"' {
                if let Some(_next_c) = chars.next() {
                    return None;
                } else {
                    let string = str[quote_size..(str.len() - quote_size)].to_string();
                    return Some(string);
                }
            }
            if c == '\n' {
                return None;
            }
        }
        return Some(str[quote_size..].to_string());
    }

    fn is_valid_string(token: &str) -> bool {
        let last_char = token.chars().last().unwrap();
        return last_char == '"';
    }

    fn arrange_number(&self, lexeme: &str) -> ArrangedTokens {
        let last_char = lexeme.chars().last().unwrap();
        let mut lexeme = lexeme.to_string();
        let mut single_char_token: Option<SingleCharToken> = None;
        if last_char == '.' {
            single_char_token = Some(SingleCharToken::Dot);
            lexeme.pop();
        }

        let number_token = LiteralToken::from_str(lexeme.as_str());
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

    pub fn from_str(str: &str) -> Option<LiteralToken> {
        if let Some(number) = Self::parse_number(str) {
            return Some(LiteralToken::Number(number));
        }

        if let Some(string) = Self::parse_string(str) {
            return Some(LiteralToken::String(string));
        }

        if let Some(_) = Self::parse_identifier(str) {
            return Some(LiteralToken::Identifier);
        }

        None
    }
}

impl TokenType for LiteralToken {
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
