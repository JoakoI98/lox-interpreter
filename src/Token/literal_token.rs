use std::fmt::Display;

#[derive(Debug)]
pub enum LiteralToken {
    Identifier,
    String(String),
    Number(f64),
}

impl LiteralToken {
    pub fn from_str(str: &str) -> Option<LiteralToken> {
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
