use super::parse_error::{ExpectedEnum, NoTokenError, ParseError, Result, UnexpectedTokenError};

macro_rules! impl_display_debug_for_token {
    ($($struct_name:ident),*) => {
        $(
            #[derive(Debug, PartialEq, Clone)]
            pub struct $struct_name {
                pub token: crate::tokenizer::Token,
            }

            impl crate::syntax_analysis::parsing::parse_stream::Parser for $struct_name {
                fn parse(input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream) -> Result<Self> {
                    let token = input.advance().ok_or(ParseError::NoToken(
                        NoTokenError::no_token(
                            ExpectedEnum::Token(crate::tokenizer::TokenEnum::$struct_name),
                            None,
                        ),
                    ))?;
                    token.parse(crate::tokenizer::TokenEnum::$struct_name)?;
                    Ok($struct_name {
                        token: token.clone(),
                    })
                }

                fn peek(input: &crate::syntax_analysis::parsing::parse_stream::ParseStream) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::$struct_name))
                        .unwrap_or(false)
                }
            }

            impl std::fmt::Display for $struct_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.token)
                }
            }

        )*
    };
}

impl crate::tokenizer::Token {
    #[inline]
    pub fn parse(&self, expected: crate::tokenizer::TokenEnum) -> Result<&Self> {
        if self.peek(expected) {
            Ok(self)
        } else {
            Err(ParseError::UnexpectedToken(
                UnexpectedTokenError::unexpected_token(
                    self.clone(),
                    ExpectedEnum::Token(expected),
                    None,
                ),
            ))
        }
    }

    #[inline]
    pub fn peek(&self, expected: crate::tokenizer::TokenEnum) -> bool {
        self.token_type == expected
    }
}

impl_display_debug_for_token!(
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
    Identifier
);
