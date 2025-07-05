use crate::tokenizer::{Token, TokenEnum};

use super::{
    parse_error::{ExpectedEnum, ParseError, Result},
    parse_stream::{ParseStream, Parser},
};

impl Token {
    #[inline]
    pub fn parse(&self, expected: TokenEnum) -> Result<&Self> {
        if self.peek(expected) {
            Ok(self)
        } else {
            Err(ParseError::unexpected_token(self.clone(), expected, None))
        }
    }

    #[inline]
    pub fn peek(&self, expected: TokenEnum) -> bool {
        self.token_type == expected
    }
}

pub struct LeftParen {
    token: Token,
}

pub struct RightParen {
    token: Token,
}

pub struct LeftBrace {
    token: Token,
}

pub struct RightBrace {
    token: Token,
}

pub struct Comma {
    token: Token,
}

pub struct Dot {
    token: Token,
}

pub struct Minus {
    token: Token,
}

pub struct Plus {
    token: Token,
}

pub struct Semicolon {
    token: Token,
}

pub struct Slash {
    token: Token,
}

pub struct Star {
    token: Token,
}

pub struct Bang {
    token: Token,
}

pub struct Equal {
    token: Token,
}

pub struct Greater {
    token: Token,
}

pub struct Less {
    token: Token,
}

pub struct Eof {
    token: Token,
}

// TwoCharToken
pub struct BangEqual {
    token: Token,
}

pub struct EqualEqual {
    token: Token,
}

pub struct GreaterEqual {
    token: Token,
}

pub struct LessEqual {
    token: Token,
}

// KeywordToken
pub struct And {
    token: Token,
}

pub struct Class {
    token: Token,
}

pub struct Else {
    token: Token,
}

pub struct False {
    token: Token,
}

pub struct Fun {
    token: Token,
}

pub struct For {
    token: Token,
}

pub struct If {
    token: Token,
}

pub struct Nil {
    token: Token,
}

pub struct Or {
    token: Token,
}

pub struct Print {
    token: Token,
}

pub struct Return {
    token: Token,
}

pub struct Super {
    token: Token,
}

pub struct This {
    token: Token,
}

pub struct True {
    token: Token,
}

pub struct Var {
    token: Token,
}

pub struct While {
    token: Token,
}

// LiteralToken
pub struct Number {
    token: Token,
}

pub struct String {
    token: Token,
}

pub struct Identifier {
    token: Token,
}

impl Parser for LeftParen {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::LeftParen),
            None,
        ))?;
        token.parse(TokenEnum::LeftParen)?;
        Ok(LeftParen {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::LeftParen))
            .unwrap_or(false)
    }
}

impl Parser for RightParen {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::RightParen),
            None,
        ))?;
        token.parse(TokenEnum::RightParen)?;
        Ok(RightParen {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::RightParen))
            .unwrap_or(false)
    }
}

impl Parser for LeftBrace {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::LeftBrace),
            None,
        ))?;
        token.parse(TokenEnum::LeftBrace)?;
        Ok(LeftBrace {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::LeftBrace))
            .unwrap_or(false)
    }
}

impl Parser for RightBrace {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::RightBrace),
            None,
        ))?;
        token.parse(TokenEnum::RightBrace)?;
        Ok(RightBrace {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::RightBrace))
            .unwrap_or(false)
    }
}

impl Parser for Comma {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Comma),
            None,
        ))?;
        token.parse(TokenEnum::Comma)?;
        Ok(Comma {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Comma))
            .unwrap_or(false)
    }
}

impl Parser for Dot {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Dot),
            None,
        ))?;
        token.parse(TokenEnum::Dot)?;
        Ok(Dot {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Dot))
            .unwrap_or(false)
    }
}

impl Parser for Minus {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Minus),
            None,
        ))?;
        token.parse(TokenEnum::Minus)?;
        Ok(Minus {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Minus))
            .unwrap_or(false)
    }
}

impl Parser for Plus {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Plus),
            None,
        ))?;
        token.parse(TokenEnum::Plus)?;
        Ok(Plus {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Plus))
            .unwrap_or(false)
    }
}

impl Parser for Semicolon {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Semicolon),
            None,
        ))?;
        token.parse(TokenEnum::Semicolon)?;
        Ok(Semicolon {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Semicolon))
            .unwrap_or(false)
    }
}

impl Parser for Slash {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Slash),
            None,
        ))?;
        token.parse(TokenEnum::Slash)?;
        Ok(Slash {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Slash))
            .unwrap_or(false)
    }
}

impl Parser for Star {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Star),
            None,
        ))?;
        token.parse(TokenEnum::Star)?;
        Ok(Star {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Star))
            .unwrap_or(false)
    }
}

impl Parser for Bang {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Bang),
            None,
        ))?;
        token.parse(TokenEnum::Bang)?;
        Ok(Bang {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Bang))
            .unwrap_or(false)
    }
}

impl Parser for Equal {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Equal),
            None,
        ))?;
        token.parse(TokenEnum::Equal)?;
        Ok(Equal {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Equal))
            .unwrap_or(false)
    }
}

impl Parser for Greater {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Greater),
            None,
        ))?;
        token.parse(TokenEnum::Greater)?;
        Ok(Greater {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Greater))
            .unwrap_or(false)
    }
}

impl Parser for Less {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Less),
            None,
        ))?;
        token.parse(TokenEnum::Less)?;
        Ok(Less {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Less))
            .unwrap_or(false)
    }
}

impl Parser for Eof {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Eof),
            None,
        ))?;
        token.parse(TokenEnum::Eof)?;
        Ok(Eof {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Eof))
            .unwrap_or(false)
    }
}

impl Parser for BangEqual {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::BangEqual),
            None,
        ))?;
        token.parse(TokenEnum::BangEqual)?;
        Ok(BangEqual {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::BangEqual))
            .unwrap_or(false)
    }
}

impl Parser for EqualEqual {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::EqualEqual),
            None,
        ))?;
        token.parse(TokenEnum::EqualEqual)?;
        Ok(EqualEqual {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::EqualEqual))
            .unwrap_or(false)
    }
}

impl Parser for GreaterEqual {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::GreaterEqual),
            None,
        ))?;
        token.parse(TokenEnum::GreaterEqual)?;
        Ok(GreaterEqual {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::GreaterEqual))
            .unwrap_or(false)
    }
}

impl Parser for LessEqual {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::LessEqual),
            None,
        ))?;
        token.parse(TokenEnum::LessEqual)?;
        Ok(LessEqual {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::LessEqual))
            .unwrap_or(false)
    }
}

impl Parser for And {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::And),
            None,
        ))?;
        token.parse(TokenEnum::And)?;
        Ok(And {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::And))
            .unwrap_or(false)
    }
}

impl Parser for Class {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Class),
            None,
        ))?;
        token.parse(TokenEnum::Class)?;
        Ok(Class {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Class))
            .unwrap_or(false)
    }
}

impl Parser for Else {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Else),
            None,
        ))?;
        token.parse(TokenEnum::Else)?;
        Ok(Else {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Else))
            .unwrap_or(false)
    }
}

impl Parser for False {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::False),
            None,
        ))?;
        token.parse(TokenEnum::False)?;
        Ok(False {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::False))
            .unwrap_or(false)
    }
}

impl Parser for Fun {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Fun),
            None,
        ))?;
        token.parse(TokenEnum::Fun)?;
        Ok(Fun {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Fun))
            .unwrap_or(false)
    }
}

impl Parser for For {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::For),
            None,
        ))?;
        token.parse(TokenEnum::For)?;
        Ok(For {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::For))
            .unwrap_or(false)
    }
}

impl Parser for If {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::If),
            None,
        ))?;
        token.parse(TokenEnum::If)?;
        Ok(If {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::If))
            .unwrap_or(false)
    }
}

impl Parser for Nil {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Nil),
            None,
        ))?;
        token.parse(TokenEnum::Nil)?;
        Ok(Nil {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Nil))
            .unwrap_or(false)
    }
}

impl Parser for Or {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Or),
            None,
        ))?;
        token.parse(TokenEnum::Or)?;
        Ok(Or {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Or))
            .unwrap_or(false)
    }
}

impl Parser for Print {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Print),
            None,
        ))?;
        token.parse(TokenEnum::Print)?;
        Ok(Print {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Print))
            .unwrap_or(false)
    }
}

impl Parser for Return {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Return),
            None,
        ))?;
        token.parse(TokenEnum::Return)?;
        Ok(Return {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Return))
            .unwrap_or(false)
    }
}

impl Parser for Super {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Super),
            None,
        ))?;
        token.parse(TokenEnum::Super)?;
        Ok(Super {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Super))
            .unwrap_or(false)
    }
}

impl Parser for This {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::This),
            None,
        ))?;
        token.parse(TokenEnum::This)?;
        Ok(This {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::This))
            .unwrap_or(false)
    }
}

impl Parser for True {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::True),
            None,
        ))?;
        token.parse(TokenEnum::True)?;
        Ok(True {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::True))
            .unwrap_or(false)
    }
}

impl Parser for Var {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Var),
            None,
        ))?;
        token.parse(TokenEnum::Var)?;
        Ok(Var {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Var))
            .unwrap_or(false)
    }
}

impl Parser for While {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::While),
            None,
        ))?;
        token.parse(TokenEnum::While)?;
        Ok(While {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::While))
            .unwrap_or(false)
    }
}

impl Parser for Number {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Number),
            None,
        ))?;
        token.parse(TokenEnum::Number)?;
        Ok(Number {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Number))
            .unwrap_or(false)
    }
}

impl Parser for String {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::String),
            None,
        ))?;
        token.parse(TokenEnum::String)?;
        Ok(String {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::String))
            .unwrap_or(false)
    }
}

impl Parser for Identifier {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let token = input.advance().ok_or(ParseError::no_token(
            ExpectedEnum::Token(TokenEnum::Identifier),
            None,
        ))?;
        token.parse(TokenEnum::Identifier)?;
        Ok(Identifier {
            token: token.clone(),
        })
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|t: &Token| t.peek(TokenEnum::Identifier))
            .unwrap_or(false)
    }
}
