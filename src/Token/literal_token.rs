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

    fn arrange_number(token: super::token::Token) -> Vec<super::token::Token> {
        let last_char = token.lexeme.chars().last().unwrap();
        let mut lexeme = token.lexeme.clone();
        let mut tokens: Vec<super::token::Token> = Vec::new();
        if last_char == '.' {
            let dot_token = super::token::Token::from_str(".", token.line, token.column_start);
            if let Some(dot_token) = dot_token {
                tokens.push(dot_token);
            }
            lexeme.pop();
        }

        let number_token =
            super::token::Token::from_str(lexeme.as_str(), token.line, token.column_start);
        if let Some(number_token) = number_token {
            tokens.insert(0, number_token);
        }

        return tokens;
    }

    pub fn arrange_token(
        token: super::token::Token,
    ) -> Result<Vec<super::token::Token>, super::scanner::ScannerError> {
        match &token.token_type {
            super::token_type::TokenType::LiteralToken(LiteralToken::Number(_)) => {
                Ok(Self::arrange_number(token))
            }
            super::token_type::TokenType::LiteralToken(LiteralToken::String(_)) => {
                if Self::is_valid_string(token.lexeme.clone().as_str()) {
                    Ok(vec![token])
                } else {
                    Err(super::scanner::ScannerError::NotTerminatedString(
                        token.line,
                    ))
                }
            }
            _ => Ok(vec![token]),
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

impl Display for LiteralToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralToken::Identifier => write!(f, "IDENTIFIER"),
            LiteralToken::String(_string) => write!(f, "STRING"),
            LiteralToken::Number(_number) => write!(f, "NUMBER"),
        }
    }
}
