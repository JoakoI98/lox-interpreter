use std::fmt::Display;

#[derive(Debug)]
pub enum KeywordToken {
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Return,
    Super,
    Print,
    This,
    True,
    Var,
    While,
}

impl KeywordToken {
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

impl Display for KeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeywordToken::And => write!(f, "AND"),
            KeywordToken::Class => write!(f, "CLASS"),
            KeywordToken::Else => write!(f, "ELSE"),
            KeywordToken::False => write!(f, "FALSE"),
            KeywordToken::Fun => write!(f, "FUN"),
            KeywordToken::For => write!(f, "FOR"),
            KeywordToken::If => write!(f, "IF"),
            KeywordToken::Nil => write!(f, "NIL"),
            KeywordToken::Or => write!(f, "OR"),
            KeywordToken::Return => write!(f, "RETURN"),
            KeywordToken::Super => write!(f, "SUPER"),
            KeywordToken::Print => write!(f, "PRINT"),
            KeywordToken::This => write!(f, "THIS"),
            KeywordToken::True => write!(f, "TRUE"),
            KeywordToken::Var => write!(f, "VAR"),
            KeywordToken::While => write!(f, "WHILE"),
        }
    }
}
