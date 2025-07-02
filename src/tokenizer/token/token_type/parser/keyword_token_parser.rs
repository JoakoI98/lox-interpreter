use super::{KeywordToken, StringParser, TokenType};

pub struct KeywordTokenParser {}

impl KeywordTokenParser {
    pub fn from_str(str: &str) -> Option<KeywordToken> {
        match str {
            "and" => Some(KeywordToken::And),
            "class" => Some(KeywordToken::Class),
            "else" => Some(KeywordToken::Else),
            "false" => Some(KeywordToken::False),
            "fun" => Some(KeywordToken::Fun),
            "for" => Some(KeywordToken::For),
            "if" => Some(KeywordToken::If),
            "nil" => Some(KeywordToken::Nil),
            "or" => Some(KeywordToken::Or),
            "return" => Some(KeywordToken::Return),
            "super" => Some(KeywordToken::Super),
            "print" => Some(KeywordToken::Print),
            "this" => Some(KeywordToken::This),
            "true" => Some(KeywordToken::True),
            "var" => Some(KeywordToken::Var),
            "while" => Some(KeywordToken::While),
            _ => None,
        }
    }
}

impl StringParser for KeywordTokenParser {
    fn parse_string(&self, string: &str) -> Option<Box<dyn TokenType>> {
        KeywordTokenParser::from_str(string).map(|token| Box::new(token) as Box<dyn TokenType>)
    }
}
