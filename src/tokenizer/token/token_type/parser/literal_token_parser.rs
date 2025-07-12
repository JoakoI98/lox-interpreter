use regex::Regex;

use super::{LiteralToken, StringParser, TokenType};

pub struct LiteralTokenParser {}

impl LiteralTokenParser {
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
        }
        return Some(str[quote_size..].to_string());
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

impl StringParser for LiteralTokenParser {
    fn parse_string(&self, string: &str) -> Option<Box<dyn TokenType>> {
        LiteralTokenParser::from_str(string).map(|token| Box::new(token) as Box<dyn TokenType>)
    }
}
