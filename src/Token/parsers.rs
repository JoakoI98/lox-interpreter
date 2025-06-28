use core::str;

use crate::Token::{keyword_token::KeywordToken, token_type::TokenType};

trait StringParserReceiver {
    fn parse_tokens<T: StringParser>(&self, parser: &T) -> Option<Box<dyn TokenType>>;
}

impl StringParserReceiver for String {
    fn parse_tokens<T: StringParser>(&self, parser: &T) -> Option<Box<dyn TokenType>> {
        parser.parse_string(self.as_str())
    }
}

pub trait StringParser {
    fn parse_string(&self, string: &str) -> Option<Box<dyn TokenType>>;
}

struct KeywordTokenParser {}

impl StringParser for KeywordTokenParser {
    fn parse_string(&self, string: &str) -> Option<Box<dyn TokenType>> {
        KeywordToken::from_str(string).map(|token| Box::new(token) as Box<dyn TokenType>)
    }
}

fn test() {
    let p = KeywordTokenParser {};
    let v: Vec<&dyn StringParser> = vec![&p];
}
