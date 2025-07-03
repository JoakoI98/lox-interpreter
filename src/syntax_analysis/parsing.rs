use crate::tokenizer::{Token, TokenEnum};

pub struct ParseStream {
    tokens: Vec<Token>,
    current_index: usize,
}

pub struct ParseError {
    pub message: std::string::String,
}

pub type Result<T> = std::result::Result<T, ParseError>;

pub trait Parser {
    fn parse(input: &mut ParseStream) -> Result<Self>
    where
        Self: Sized;

    fn peek(input: &ParseStream) -> bool;
}

impl ParseStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_index: 0,
        }
    }

    pub fn parse<T: Parser>(&mut self) -> Result<T> {
        T::parse(self)
    }

    pub fn peek<T: Parser>(&self) -> bool {
        T::peek(self)
    }

    pub fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.current_index);
        self.current_index += 1;
        token
    }

    pub fn peek1(&self) -> Option<&Token> {
        self.tokens.get(self.current_index)
    }
}

// SingleCharToken
pub struct LeftParen {}

pub struct RightParen {}

pub struct LeftBrace {}

pub struct RightBrace {}
pub struct Comma {}

pub struct Dot {}

pub struct Minus {}

pub struct Plus {}

pub struct Semicolon {}

pub struct Slash {}

pub struct Star {}

pub struct Bang {}

pub struct Equal {}

pub struct Greater {}

pub struct Less {}

pub struct Eof {}

// TwoCharToken
pub struct BangEqual {}

pub struct EqualEqual {}

pub struct GreaterEqual {}

pub struct LessEqual {}

// KeywordToken
pub struct And {}

pub struct Class {}

pub struct Else {}

pub struct False {}

pub struct Fun {}

pub struct For {}

pub struct If {}

pub struct Nil {}

pub struct Or {}

pub struct Print {}

pub struct Return {}

pub struct Super {}

pub struct This {}

pub struct True {}

pub struct Var {}

pub struct While {}

// LiteralToken
pub struct Number {}

pub struct String {}

pub struct Identifier {}

impl Parser for LeftParen {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::LeftParen) => Ok(LeftParen {}),
            _ => Err(ParseError {
                message: "Expected LeftParen".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::LeftParen) => true,
            _ => false,
        }
    }
}

impl Parser for RightParen {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::RightParen) => Ok(RightParen {}),
            _ => Err(ParseError {
                message: "Expected RightParen".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::RightParen) => true,
            _ => false,
        }
    }
}

impl Parser for LeftBrace {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::LeftBrace) => Ok(LeftBrace {}),
            _ => Err(ParseError {
                message: "Expected LeftBrace".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::LeftBrace) => true,
            _ => false,
        }
    }
}

impl Parser for RightBrace {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::RightBrace) => Ok(RightBrace {}),
            _ => Err(ParseError {
                message: "Expected RightBrace".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::RightBrace) => true,
            _ => false,
        }
    }
}

impl Parser for Comma {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Comma) => Ok(Comma {}),
            _ => Err(ParseError {
                message: "Expected Comma".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Comma) => true,
            _ => false,
        }
    }
}

impl Parser for Dot {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Dot) => Ok(Dot {}),
            _ => Err(ParseError {
                message: "Expected Dot".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Dot) => true,
            _ => false,
        }
    }
}

impl Parser for Minus {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Minus) => Ok(Minus {}),
            _ => Err(ParseError {
                message: "Expected Minus".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Minus) => true,
            _ => false,
        }
    }
}

impl Parser for Plus {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Plus) => Ok(Plus {}),
            _ => Err(ParseError {
                message: "Expected Plus".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Plus) => true,
            _ => false,
        }
    }
}

impl Parser for Semicolon {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Semicolon) => Ok(Semicolon {}),
            _ => Err(ParseError {
                message: "Expected Semicolon".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Semicolon) => true,
            _ => false,
        }
    }
}

impl Parser for Slash {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Slash) => Ok(Slash {}),
            _ => Err(ParseError {
                message: "Expected Slash".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Slash) => true,
            _ => false,
        }
    }
}

impl Parser for Star {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Star) => Ok(Star {}),
            _ => Err(ParseError {
                message: "Expected Star".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Star) => true,
            _ => false,
        }
    }
}

impl Parser for Bang {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Bang) => Ok(Bang {}),
            _ => Err(ParseError {
                message: "Expected Bang".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Bang) => true,
            _ => false,
        }
    }
}

impl Parser for Equal {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Equal) => Ok(Equal {}),
            _ => Err(ParseError {
                message: "Expected Equal".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Equal) => true,
            _ => false,
        }
    }
}

impl Parser for Greater {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Greater) => Ok(Greater {}),
            _ => Err(ParseError {
                message: "Expected Greater".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Greater) => true,
            _ => false,
        }
    }
}

impl Parser for Less {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Less) => Ok(Less {}),
            _ => Err(ParseError {
                message: "Expected Less".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Less) => true,
            _ => false,
        }
    }
}

impl Parser for Eof {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Eof) => Ok(Eof {}),
            _ => Err(ParseError {
                message: "Expected Eof".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Eof) => true,
            _ => false,
        }
    }
}

impl Parser for BangEqual {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::BangEqual) => Ok(BangEqual {}),
            _ => Err(ParseError {
                message: "Expected BangEqual".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::BangEqual) => true,
            _ => false,
        }
    }
}

impl Parser for EqualEqual {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::EqualEqual) => Ok(EqualEqual {}),
            _ => Err(ParseError {
                message: "Expected EqualEqual".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::EqualEqual) => true,
            _ => false,
        }
    }
}

impl Parser for GreaterEqual {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::GreaterEqual) => Ok(GreaterEqual {}),
            _ => Err(ParseError {
                message: "Expected GreaterEqual".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::GreaterEqual) => true,
            _ => false,
        }
    }
}

impl Parser for LessEqual {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::LessEqual) => Ok(LessEqual {}),
            _ => Err(ParseError {
                message: "Expected LessEqual".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::LessEqual) => true,
            _ => false,
        }
    }
}

impl Parser for And {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::And) => Ok(And {}),
            _ => Err(ParseError {
                message: "Expected And".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::And) => true,
            _ => false,
        }
    }
}

impl Parser for Class {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Class) => Ok(Class {}),
            _ => Err(ParseError {
                message: "Expected Class".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Class) => true,
            _ => false,
        }
    }
}

impl Parser for Else {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Else) => Ok(Else {}),
            _ => Err(ParseError {
                message: "Expected Else".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Else) => true,
            _ => false,
        }
    }
}

impl Parser for False {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::False) => Ok(False {}),
            _ => Err(ParseError {
                message: "Expected False".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::False) => true,
            _ => false,
        }
    }
}

impl Parser for Fun {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Fun) => Ok(Fun {}),
            _ => Err(ParseError {
                message: "Expected Fun".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Fun) => true,
            _ => false,
        }
    }
}

impl Parser for For {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::For) => Ok(For {}),
            _ => Err(ParseError {
                message: "Expected For".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::For) => true,
            _ => false,
        }
    }
}

impl Parser for If {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::If) => Ok(If {}),
            _ => Err(ParseError {
                message: "Expected If".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::If) => true,
            _ => false,
        }
    }
}

impl Parser for Nil {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Nil) => Ok(Nil {}),
            _ => Err(ParseError {
                message: "Expected Nil".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Nil) => true,
            _ => false,
        }
    }
}

impl Parser for Or {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Or) => Ok(Or {}),
            _ => Err(ParseError {
                message: "Expected Or".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Or) => true,
            _ => false,
        }
    }
}

impl Parser for Print {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Print) => Ok(Print {}),
            _ => Err(ParseError {
                message: "Expected Print".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Print) => true,
            _ => false,
        }
    }
}

impl Parser for Return {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Return) => Ok(Return {}),
            _ => Err(ParseError {
                message: "Expected Return".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Return) => true,
            _ => false,
        }
    }
}

impl Parser for Super {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Super) => Ok(Super {}),
            _ => Err(ParseError {
                message: "Expected Super".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Super) => true,
            _ => false,
        }
    }
}

impl Parser for This {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::This) => Ok(This {}),
            _ => Err(ParseError {
                message: "Expected This".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::This) => true,
            _ => false,
        }
    }
}

impl Parser for True {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::True) => Ok(True {}),
            _ => Err(ParseError {
                message: "Expected True".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::True) => true,
            _ => false,
        }
    }
}

impl Parser for Var {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Var) => Ok(Var {}),
            _ => Err(ParseError {
                message: "Expected Var".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Var) => true,
            _ => false,
        }
    }
}

impl Parser for While {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::While) => Ok(While {}),
            _ => Err(ParseError {
                message: "Expected While".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::While) => true,
            _ => false,
        }
    }
}

impl Parser for Number {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Number) => Ok(Number {}),
            _ => Err(ParseError {
                message: "Expected Number".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Number) => true,
            _ => false,
        }
    }
}

impl Parser for String {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::String) => Ok(String {}),
            _ => Err(ParseError {
                message: "Expected String".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::String) => true,
            _ => false,
        }
    }
}

impl Parser for Identifier {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token_type = input.advance().map(|t: &Token| t.token_type.token_type());
        match token_type {
            Some(TokenEnum::Identifier) => Ok(Identifier {}),
            _ => Err(ParseError {
                message: "Expected Identifier".to_string(),
            }),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        let token = input.peek1().map(|t: &Token| t.token_type.token_type());
        match token {
            Some(TokenEnum::Identifier) => true,
            _ => false,
        }
    }
}
