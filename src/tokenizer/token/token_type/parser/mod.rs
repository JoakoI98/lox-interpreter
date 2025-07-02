use core::str;
use std::sync::LazyLock;

mod keyword_token_parser;
mod literal_token_parser;
mod single_char_token_parser;
mod two_char_token_parser;

use keyword_token_parser::KeywordTokenParser;
use literal_token_parser::LiteralTokenParser;
use single_char_token_parser::SingleCharTokenParser;
use two_char_token_parser::TwoCharTokenParser;

use super::TokenType;

use super::keyword_token::KeywordToken;
use super::literal_token::LiteralToken;
use super::single_char_token::SingleCharToken;
use super::two_char_token::TwoCharToken;

pub trait StringParser: Send + Sync {
    fn parse_string(&self, string: &str) -> Option<Box<dyn TokenType>>;
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
