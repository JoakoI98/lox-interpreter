use super::{StringParser, TokenType, TwoCharToken};

pub struct TwoCharTokenParser {}

impl TwoCharTokenParser {
    pub fn from_str(str: &str) -> Option<TwoCharToken> {
        if str.len() != 2 {
            return None;
        }

        match str {
            "!=" => Some(TwoCharToken::BangEqual),
            "==" => Some(TwoCharToken::EqualEqual),
            ">=" => Some(TwoCharToken::GreaterEqual),
            "<=" => Some(TwoCharToken::LessEqual),
            _ => None,
        }
    }
}

impl StringParser for TwoCharTokenParser {
    fn parse_string(&self, string: &str) -> Option<Box<dyn TokenType>> {
        TwoCharTokenParser::from_str(string).map(|token| Box::new(token) as Box<dyn TokenType>)
    }
}
