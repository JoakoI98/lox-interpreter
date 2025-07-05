use std::fmt::{Debug, Display};

mod parser;

mod keyword_token;
mod literal_token;
mod single_char_token;
mod two_char_token;

pub use super::TokenErrors;

pub use parser::PARSERS;
pub use single_char_token::SingleCharToken::Eof as EOFToken;

pub enum ArrangedTokens {
    Single(Box<dyn TokenType>),
    Multiple(Box<dyn TokenType>, Box<dyn TokenType>),
    Same,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    Number(f64),
    String(String),
    Identifier(String),
    None,
}

pub trait TokenType: Display + Debug {
    fn token_type(&self) -> Token;

    fn is_token(&self, token_type: Token) -> bool {
        self.token_type() == token_type
    }

    fn get_value(&self) -> TokenValue {
        TokenValue::None
    }

    fn literal_value(&self) -> Option<String> {
        None
    }

    fn arrange_token(&self, _lexeme: &str) -> Result<ArrangedTokens, super::TokenErrors> {
        Ok(ArrangedTokens::Same)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Token {
    // SingleCharToken
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    Equal,
    Greater,
    Less,
    Eof,

    // TwoCharToken
    BangEqual,
    EqualEqual,
    GreaterEqual,
    LessEqual,

    // KeywordToken
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // LiteralToken
    Number,
    String,
    Identifier,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_representation = match self {
            Token::LeftParen => "(",
            Token::RightParen => ")",
            Token::LeftBrace => "{",
            Token::RightBrace => "}",
            Token::Comma => ",",
            Token::Dot => ".",
            Token::Minus => "-",
            Token::Plus => "+",
            Token::Semicolon => ";",
            Token::Slash => "/",
            Token::Star => "*",
            Token::Bang => "!",
            Token::Equal => "=",
            Token::Greater => ">",
            Token::Less => "<",
            Token::Eof => "EOF",
            Token::BangEqual => "!=",
            Token::EqualEqual => "==",
            Token::GreaterEqual => ">=",
            Token::LessEqual => "<=",
            Token::And => "and",
            Token::Class => "class",
            Token::Else => "else",
            Token::False => "false",
            Token::Fun => "fun",
            Token::For => "for",
            Token::If => "if",
            Token::Nil => "nil",
            Token::Or => "or",
            Token::Print => "print",
            Token::Return => "return",
            Token::Super => "super",
            Token::This => "this",
            Token::True => "true",
            Token::Var => "var",
            Token::While => "while",
            Token::Number => "number",
            Token::String => "string",
            Token::Identifier => "identifier",
        };
        write!(f, "{}", string_representation)
    }
}
