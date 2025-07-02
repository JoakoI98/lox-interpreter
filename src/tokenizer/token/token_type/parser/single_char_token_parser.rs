use super::{SingleCharToken, StringParser, TokenType};

pub struct SingleCharTokenParser {}

impl SingleCharTokenParser {
    pub fn from_str(str: &str) -> Option<SingleCharToken> {
        if str.len() != 1 {
            return None;
        }

        let c = str.chars().next().unwrap();
        match c {
            '(' => Some(SingleCharToken::LeftParen),
            ')' => Some(SingleCharToken::RightParen),
            '{' => Some(SingleCharToken::LeftBrace),
            '}' => Some(SingleCharToken::RightBrace),
            ',' => Some(SingleCharToken::Comma),
            '.' => Some(SingleCharToken::Dot),
            '-' => Some(SingleCharToken::Minus),
            '+' => Some(SingleCharToken::Plus),
            ';' => Some(SingleCharToken::Semicolon),
            '/' => Some(SingleCharToken::Slash),
            '*' => Some(SingleCharToken::Star),
            '!' => Some(SingleCharToken::Bang),
            '=' => Some(SingleCharToken::Equal),
            '>' => Some(SingleCharToken::Greater),
            '<' => Some(SingleCharToken::Less),
            '\0' => Some(SingleCharToken::Eof),
            _ => None,
        }
    }
}

impl StringParser for SingleCharTokenParser {
    fn parse_string(&self, string: &str) -> Option<Box<dyn TokenType>> {
        SingleCharTokenParser::from_str(string).map(|token| Box::new(token) as Box<dyn TokenType>)
    }
}
