use std::fmt::Display;

#[derive(Debug)]
pub enum TwoCharToken {
    BangEqual,
    EqualEqual,
    GreaterEqual,
    LessEqual,
}

impl TwoCharToken {
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

impl Display for TwoCharToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TwoCharToken::BangEqual => write!(f, "BANG_EQUAL"),
            TwoCharToken::EqualEqual => write!(f, "EQUAL_EQUAL"),
            TwoCharToken::GreaterEqual => write!(f, "GREATER_EQUAL"),
            TwoCharToken::LessEqual => write!(f, "LESS_EQUAL"),
        }
    }
}
