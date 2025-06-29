use core::str;
use std::sync::LazyLock;

use crate::Token::{
    keyword_token::KeywordToken, literal_token::LiteralToken, single_char_token::SingleCharToken,
    token_type::TokenType, two_char_token::TwoCharToken,
};

pub trait StringParserReceiver {
    fn parse_tokens<T: StringParser>(&self, parser: &T) -> Option<Box<dyn TokenType>>;
}

impl StringParserReceiver for String {
    fn parse_tokens<T: StringParser>(&self, parser: &T) -> Option<Box<dyn TokenType>> {
        parser.parse_string(self.as_str())
    }
}

impl StringParserReceiver for &str {
    fn parse_tokens<T: StringParser>(&self, parser: &T) -> Option<Box<dyn TokenType>> {
        parser.parse_string(self)
    }
}

pub trait StringParser: Send + Sync {
    fn parse_string(&self, string: &str) -> Option<Box<dyn TokenType>>;
}

struct KeywordTokenParser {}

impl StringParser for KeywordTokenParser {
    fn parse_string(&self, string: &str) -> Option<Box<dyn TokenType>> {
        KeywordToken::from_str(string).map(|token| Box::new(token) as Box<dyn TokenType>)
    }
}

struct LiteralTokenParser {}

impl StringParser for LiteralTokenParser {
    fn parse_string(&self, string: &str) -> Option<Box<dyn TokenType>> {
        LiteralToken::from_str(string).map(|token| Box::new(token) as Box<dyn TokenType>)
    }
}

struct SingleCharTokenParser {}

impl StringParser for SingleCharTokenParser {
    fn parse_string(&self, string: &str) -> Option<Box<dyn TokenType>> {
        SingleCharToken::from_str(string).map(|token| Box::new(token) as Box<dyn TokenType>)
    }
}

struct TwoCharTokenParser {}

impl StringParser for TwoCharTokenParser {
    fn parse_string(&self, string: &str) -> Option<Box<dyn TokenType>> {
        TwoCharToken::from_str(string).map(|token| Box::new(token) as Box<dyn TokenType>)
    }
}

static KEYWORD_TOKEN_PARSER: KeywordTokenParser = KeywordTokenParser {};
static LITERAL_TOKEN_PARSER: LiteralTokenParser = LiteralTokenParser {};
static SINGLE_CHAR_TOKEN_PARSER: SingleCharTokenParser = SingleCharTokenParser {};
static TWO_CHAR_TOKEN_PARSER: TwoCharTokenParser = TwoCharTokenParser {};

pub static PARSERS: LazyLock<Vec<&dyn StringParser>> = LazyLock::new(|| {
    vec![
        &SINGLE_CHAR_TOKEN_PARSER,
        &TWO_CHAR_TOKEN_PARSER,
        &KEYWORD_TOKEN_PARSER,
        &LITERAL_TOKEN_PARSER,
    ]
});
