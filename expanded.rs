mod syntax_analysis {
    use std::fmt::{Debug, Display};
    use ast_leaf::ast_leaf;
    mod parsing {
        mod parse_error {
            use std::fmt::Display;
            use crate::tokenizer::{Token, TokenEnum};
            use thiserror::Error;
            pub enum ExpectedEnum {
                Token(TokenEnum),
                Tokens(Vec<TokenEnum>),
                NonTerminal(String),
                Unknown,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ExpectedEnum {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        ExpectedEnum::Token(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Token",
                                &__self_0,
                            )
                        }
                        ExpectedEnum::Tokens(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Tokens",
                                &__self_0,
                            )
                        }
                        ExpectedEnum::NonTerminal(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "NonTerminal",
                                &__self_0,
                            )
                        }
                        ExpectedEnum::Unknown => {
                            ::core::fmt::Formatter::write_str(f, "Unknown")
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ExpectedEnum {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ExpectedEnum {
                #[inline]
                fn eq(&self, other: &ExpectedEnum) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                        && match (self, other) {
                            (
                                ExpectedEnum::Token(__self_0),
                                ExpectedEnum::Token(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            (
                                ExpectedEnum::Tokens(__self_0),
                                ExpectedEnum::Tokens(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            (
                                ExpectedEnum::NonTerminal(__self_0),
                                ExpectedEnum::NonTerminal(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            _ => true,
                        }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ExpectedEnum {
                #[inline]
                fn clone(&self) -> ExpectedEnum {
                    match self {
                        ExpectedEnum::Token(__self_0) => {
                            ExpectedEnum::Token(::core::clone::Clone::clone(__self_0))
                        }
                        ExpectedEnum::Tokens(__self_0) => {
                            ExpectedEnum::Tokens(::core::clone::Clone::clone(__self_0))
                        }
                        ExpectedEnum::NonTerminal(__self_0) => {
                            ExpectedEnum::NonTerminal(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ExpectedEnum::Unknown => ExpectedEnum::Unknown,
                    }
                }
            }
            impl Display for ExpectedEnum {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        ExpectedEnum::Token(token) => {
                            f.write_fmt(format_args!("Expected: \'{0}\'", token))
                        }
                        ExpectedEnum::NonTerminal(non_terminal) => {
                            f.write_fmt(format_args!("Expected: \'{0}\'", non_terminal))
                        }
                        ExpectedEnum::Tokens(tokens) => {
                            let expected_tokens_string = tokens
                                .iter()
                                .map(|token| ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("\'{0}\'", token),
                                    );
                                    res
                                }))
                                .collect::<Vec<String>>()
                                .join(", ");
                            f.write_fmt(
                                format_args!("Expected one of: {0}", expected_tokens_string),
                            )
                        }
                        ExpectedEnum::Unknown => f.write_fmt(format_args!("")),
                    }
                }
            }
            pub struct UnexpectedTokenError {
                message: Option<String>,
                expected: ExpectedEnum,
                token: Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for UnexpectedTokenError {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "UnexpectedTokenError",
                        "message",
                        &self.message,
                        "expected",
                        &self.expected,
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for UnexpectedTokenError {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for UnexpectedTokenError {
                #[inline]
                fn eq(&self, other: &UnexpectedTokenError) -> bool {
                    self.message == other.message && self.expected == other.expected
                        && self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for UnexpectedTokenError {
                #[inline]
                fn clone(&self) -> UnexpectedTokenError {
                    UnexpectedTokenError {
                        message: ::core::clone::Clone::clone(&self.message),
                        expected: ::core::clone::Clone::clone(&self.expected),
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl Display for UnexpectedTokenError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let message_string = self
                        .message
                        .as_ref()
                        .map(|message| ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(" {0}: ", message),
                            );
                            res
                        }))
                        .unwrap_or("".to_string());
                    let expected_string: String;
                    match self.expected {
                        ExpectedEnum::Unknown => {
                            return f
                                .write_fmt(
                                    format_args!(
                                        "Unexpected token found: \'{0}\'.{1}",
                                        self.token.token_type,
                                        message_string,
                                    ),
                                );
                        }
                        _ => expected_string = self.expected.to_string(),
                    }
                    f.write_fmt(
                        format_args!(
                            "{0}. Found: \'{1}\'.{2}",
                            expected_string,
                            self.token.token_type,
                            message_string,
                        ),
                    )
                }
            }
            impl std::error::Error for UnexpectedTokenError {}
            impl UnexpectedTokenError {
                pub fn unexpected_token(
                    token: Token,
                    expected: ExpectedEnum,
                    message: Option<String>,
                ) -> Self {
                    Self {
                        message: message,
                        expected,
                        token,
                    }
                }
            }
            pub struct NoTokenError {
                expected: ExpectedEnum,
                message: Option<String>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for NoTokenError {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "NoTokenError",
                        "expected",
                        &self.expected,
                        "message",
                        &&self.message,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for NoTokenError {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for NoTokenError {
                #[inline]
                fn eq(&self, other: &NoTokenError) -> bool {
                    self.expected == other.expected && self.message == other.message
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for NoTokenError {
                #[inline]
                fn clone(&self) -> NoTokenError {
                    NoTokenError {
                        expected: ::core::clone::Clone::clone(&self.expected),
                        message: ::core::clone::Clone::clone(&self.message),
                    }
                }
            }
            impl Display for NoTokenError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let message_string = self
                        .message
                        .as_ref()
                        .map(|message| ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(" {0}: ", message),
                            );
                            res
                        }))
                        .unwrap_or("".to_string());
                    let mut expected_string = self.expected.to_string();
                    if !expected_string.is_empty() {
                        expected_string = ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(" {0}.", expected_string),
                            );
                            res
                        });
                    }
                    f.write_fmt(
                        format_args!(
                            "Unexpected end of file.{0}{1}",
                            expected_string,
                            message_string,
                        ),
                    )
                }
            }
            impl NoTokenError {
                pub fn no_token(
                    expected: ExpectedEnum,
                    message: Option<String>,
                ) -> Self {
                    Self { expected, message }
                }
            }
            impl std::error::Error for NoTokenError {}
            pub enum ParseError {
                #[error("{0}")]
                UnexpectedToken(#[from] UnexpectedTokenError),
                #[error("{0}")]
                NoToken(#[from] NoTokenError),
            }
            #[allow(unused_qualifications)]
            impl std::error::Error for ParseError {
                fn source(
                    &self,
                ) -> ::core::option::Option<&(dyn std::error::Error + 'static)> {
                    use thiserror::__private::AsDynError as _;
                    #[allow(deprecated)]
                    match self {
                        ParseError::UnexpectedToken { 0: source, .. } => {
                            ::core::option::Option::Some(source.as_dyn_error())
                        }
                        ParseError::NoToken { 0: source, .. } => {
                            ::core::option::Option::Some(source.as_dyn_error())
                        }
                    }
                }
            }
            #[allow(unused_qualifications)]
            impl ::core::fmt::Display for ParseError {
                fn fmt(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    use thiserror::__private::AsDisplay as _;
                    #[allow(
                        unused_variables,
                        deprecated,
                        clippy::used_underscore_binding
                    )]
                    match self {
                        ParseError::UnexpectedToken(_0) => {
                            __formatter.write_fmt(format_args!("{0}", _0.as_display()))
                        }
                        ParseError::NoToken(_0) => {
                            __formatter.write_fmt(format_args!("{0}", _0.as_display()))
                        }
                    }
                }
            }
            #[allow(unused_qualifications)]
            impl ::core::convert::From<UnexpectedTokenError> for ParseError {
                #[allow(deprecated)]
                fn from(source: UnexpectedTokenError) -> Self {
                    ParseError::UnexpectedToken {
                        0: source,
                    }
                }
            }
            #[allow(unused_qualifications)]
            impl ::core::convert::From<NoTokenError> for ParseError {
                #[allow(deprecated)]
                fn from(source: NoTokenError) -> Self {
                    ParseError::NoToken { 0: source }
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ParseError {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        ParseError::UnexpectedToken(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "UnexpectedToken",
                                &__self_0,
                            )
                        }
                        ParseError::NoToken(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "NoToken",
                                &__self_0,
                            )
                        }
                    }
                }
            }
            pub type Result<T> = std::result::Result<T, ParseError>;
            impl ParseError {
                pub fn found_token(&self) -> Option<Token> {
                    match self {
                        ParseError::UnexpectedToken(error) => Some(error.token.clone()),
                        ParseError::NoToken(_) => None,
                    }
                }
            }
        }
        mod parse_stream {
            use super::parse_error::Result;
            use crate::tokenizer::Token;
            pub struct ParseStream {
                tokens: Vec<Token>,
                current_index: usize,
            }
            pub trait Parser {
                fn parse(input: &mut ParseStream) -> Result<Self>
                where
                    Self: Sized;
                fn peek(input: &ParseStream) -> bool;
            }
            impl ParseStream {
                pub fn new(tokens: Vec<Token>) -> Self {
                    Self { tokens, current_index: 0 }
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
        }
        pub mod primitives {
            use super::parse_error::{
                ExpectedEnum, NoTokenError, ParseError, Result, UnexpectedTokenError,
            };
            impl crate::tokenizer::Token {
                #[inline]
                pub fn parse(
                    &self,
                    expected: crate::tokenizer::TokenEnum,
                ) -> Result<&Self> {
                    if self.peek(expected) {
                        Ok(self)
                    } else {
                        Err(
                            ParseError::UnexpectedToken(
                                UnexpectedTokenError::unexpected_token(
                                    self.clone(),
                                    ExpectedEnum::Token(expected),
                                    None,
                                ),
                            ),
                        )
                    }
                }
                #[inline]
                pub fn peek(&self, expected: crate::tokenizer::TokenEnum) -> bool {
                    self.token_type == expected
                }
            }
            pub struct LeftParen {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for LeftParen {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "LeftParen",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for LeftParen {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for LeftParen {
                #[inline]
                fn eq(&self, other: &LeftParen) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for LeftParen {
                #[inline]
                fn clone(&self) -> LeftParen {
                    LeftParen {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for LeftParen {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::LeftParen),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::LeftParen)?;
                    Ok(LeftParen { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::LeftParen))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for LeftParen {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct RightParen {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for RightParen {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "RightParen",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for RightParen {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for RightParen {
                #[inline]
                fn eq(&self, other: &RightParen) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for RightParen {
                #[inline]
                fn clone(&self) -> RightParen {
                    RightParen {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for RightParen {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(
                                        crate::tokenizer::TokenEnum::RightParen,
                                    ),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::RightParen)?;
                    Ok(RightParen { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::RightParen))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for RightParen {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct LeftBrace {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for LeftBrace {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "LeftBrace",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for LeftBrace {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for LeftBrace {
                #[inline]
                fn eq(&self, other: &LeftBrace) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for LeftBrace {
                #[inline]
                fn clone(&self) -> LeftBrace {
                    LeftBrace {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for LeftBrace {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::LeftBrace),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::LeftBrace)?;
                    Ok(LeftBrace { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::LeftBrace))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for LeftBrace {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct RightBrace {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for RightBrace {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "RightBrace",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for RightBrace {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for RightBrace {
                #[inline]
                fn eq(&self, other: &RightBrace) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for RightBrace {
                #[inline]
                fn clone(&self) -> RightBrace {
                    RightBrace {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for RightBrace {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(
                                        crate::tokenizer::TokenEnum::RightBrace,
                                    ),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::RightBrace)?;
                    Ok(RightBrace { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::RightBrace))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for RightBrace {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Comma {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Comma {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Comma",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Comma {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Comma {
                #[inline]
                fn eq(&self, other: &Comma) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Comma {
                #[inline]
                fn clone(&self) -> Comma {
                    Comma {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Comma {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::Comma),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Comma)?;
                    Ok(Comma { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Comma))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Comma {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Dot {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Dot {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Dot",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Dot {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Dot {
                #[inline]
                fn eq(&self, other: &Dot) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Dot {
                #[inline]
                fn clone(&self) -> Dot {
                    Dot {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Dot {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::Dot),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Dot)?;
                    Ok(Dot { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Dot))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Dot {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Minus {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Minus {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Minus",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Minus {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Minus {
                #[inline]
                fn eq(&self, other: &Minus) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Minus {
                #[inline]
                fn clone(&self) -> Minus {
                    Minus {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Minus {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::Minus),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Minus)?;
                    Ok(Minus { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Minus))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Minus {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Plus {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Plus {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Plus",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Plus {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Plus {
                #[inline]
                fn eq(&self, other: &Plus) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Plus {
                #[inline]
                fn clone(&self) -> Plus {
                    Plus {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Plus {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::Plus),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Plus)?;
                    Ok(Plus { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Plus))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Plus {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Semicolon {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Semicolon {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Semicolon",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Semicolon {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Semicolon {
                #[inline]
                fn eq(&self, other: &Semicolon) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Semicolon {
                #[inline]
                fn clone(&self) -> Semicolon {
                    Semicolon {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Semicolon {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::Semicolon),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Semicolon)?;
                    Ok(Semicolon { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Semicolon))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Semicolon {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Slash {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Slash {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Slash",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Slash {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Slash {
                #[inline]
                fn eq(&self, other: &Slash) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Slash {
                #[inline]
                fn clone(&self) -> Slash {
                    Slash {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Slash {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::Slash),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Slash)?;
                    Ok(Slash { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Slash))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Slash {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Star {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Star {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Star",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Star {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Star {
                #[inline]
                fn eq(&self, other: &Star) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Star {
                #[inline]
                fn clone(&self) -> Star {
                    Star {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Star {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::Star),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Star)?;
                    Ok(Star { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Star))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Star {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Bang {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Bang {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Bang",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Bang {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Bang {
                #[inline]
                fn eq(&self, other: &Bang) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Bang {
                #[inline]
                fn clone(&self) -> Bang {
                    Bang {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Bang {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::Bang),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Bang)?;
                    Ok(Bang { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Bang))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Bang {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Equal {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Equal {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Equal",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Equal {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Equal {
                #[inline]
                fn eq(&self, other: &Equal) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Equal {
                #[inline]
                fn clone(&self) -> Equal {
                    Equal {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Equal {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::Equal),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Equal)?;
                    Ok(Equal { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Equal))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Equal {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Greater {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Greater {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Greater",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Greater {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Greater {
                #[inline]
                fn eq(&self, other: &Greater) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Greater {
                #[inline]
                fn clone(&self) -> Greater {
                    Greater {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Greater {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::Greater),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Greater)?;
                    Ok(Greater { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Greater))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Greater {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Less {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Less {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Less",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Less {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Less {
                #[inline]
                fn eq(&self, other: &Less) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Less {
                #[inline]
                fn clone(&self) -> Less {
                    Less {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Less {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::Less),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Less)?;
                    Ok(Less { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Less))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Less {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Eof {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Eof {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Eof",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Eof {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Eof {
                #[inline]
                fn eq(&self, other: &Eof) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Eof {
                #[inline]
                fn clone(&self) -> Eof {
                    Eof {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Eof {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::Eof),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Eof)?;
                    Ok(Eof { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Eof))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Eof {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct BangEqual {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for BangEqual {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "BangEqual",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for BangEqual {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for BangEqual {
                #[inline]
                fn eq(&self, other: &BangEqual) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for BangEqual {
                #[inline]
                fn clone(&self) -> BangEqual {
                    BangEqual {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for BangEqual {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::BangEqual),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::BangEqual)?;
                    Ok(BangEqual { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::BangEqual))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for BangEqual {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct EqualEqual {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for EqualEqual {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "EqualEqual",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for EqualEqual {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for EqualEqual {
                #[inline]
                fn eq(&self, other: &EqualEqual) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for EqualEqual {
                #[inline]
                fn clone(&self) -> EqualEqual {
                    EqualEqual {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for EqualEqual {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(
                                        crate::tokenizer::TokenEnum::EqualEqual,
                                    ),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::EqualEqual)?;
                    Ok(EqualEqual { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::EqualEqual))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for EqualEqual {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct GreaterEqual {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for GreaterEqual {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "GreaterEqual",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for GreaterEqual {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for GreaterEqual {
                #[inline]
                fn eq(&self, other: &GreaterEqual) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for GreaterEqual {
                #[inline]
                fn clone(&self) -> GreaterEqual {
                    GreaterEqual {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for GreaterEqual {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(
                                        crate::tokenizer::TokenEnum::GreaterEqual,
                                    ),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::GreaterEqual)?;
                    Ok(GreaterEqual {
                        token: token.clone(),
                    })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::GreaterEqual))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for GreaterEqual {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct LessEqual {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for LessEqual {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "LessEqual",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for LessEqual {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for LessEqual {
                #[inline]
                fn eq(&self, other: &LessEqual) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for LessEqual {
                #[inline]
                fn clone(&self) -> LessEqual {
                    LessEqual {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for LessEqual {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::LessEqual),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::LessEqual)?;
                    Ok(LessEqual { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::LessEqual))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for LessEqual {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct And {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for And {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "And",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for And {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for And {
                #[inline]
                fn eq(&self, other: &And) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for And {
                #[inline]
                fn clone(&self) -> And {
                    And {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for And {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::And),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::And)?;
                    Ok(And { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::And))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for And {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Class {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Class {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Class",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Class {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Class {
                #[inline]
                fn eq(&self, other: &Class) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Class {
                #[inline]
                fn clone(&self) -> Class {
                    Class {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Class {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::Class),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Class)?;
                    Ok(Class { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Class))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Class {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Else {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Else {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Else",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Else {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Else {
                #[inline]
                fn eq(&self, other: &Else) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Else {
                #[inline]
                fn clone(&self) -> Else {
                    Else {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Else {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::Else),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Else)?;
                    Ok(Else { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Else))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Else {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct False {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for False {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "False",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for False {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for False {
                #[inline]
                fn eq(&self, other: &False) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for False {
                #[inline]
                fn clone(&self) -> False {
                    False {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for False {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::False),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::False)?;
                    Ok(False { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::False))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for False {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Fun {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Fun {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Fun",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Fun {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Fun {
                #[inline]
                fn eq(&self, other: &Fun) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Fun {
                #[inline]
                fn clone(&self) -> Fun {
                    Fun {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Fun {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::Fun),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Fun)?;
                    Ok(Fun { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Fun))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Fun {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct For {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for For {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "For",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for For {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for For {
                #[inline]
                fn eq(&self, other: &For) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for For {
                #[inline]
                fn clone(&self) -> For {
                    For {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for For {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::For),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::For)?;
                    Ok(For { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::For))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for For {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct If {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for If {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "If",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for If {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for If {
                #[inline]
                fn eq(&self, other: &If) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for If {
                #[inline]
                fn clone(&self) -> If {
                    If {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for If {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::If),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::If)?;
                    Ok(If { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::If))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for If {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Nil {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Nil {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Nil",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Nil {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Nil {
                #[inline]
                fn eq(&self, other: &Nil) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Nil {
                #[inline]
                fn clone(&self) -> Nil {
                    Nil {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Nil {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::Nil),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Nil)?;
                    Ok(Nil { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Nil))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Nil {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Or {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Or {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Or",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Or {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Or {
                #[inline]
                fn eq(&self, other: &Or) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Or {
                #[inline]
                fn clone(&self) -> Or {
                    Or {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Or {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::Or),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Or)?;
                    Ok(Or { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Or))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Or {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Print {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Print {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Print",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Print {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Print {
                #[inline]
                fn eq(&self, other: &Print) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Print {
                #[inline]
                fn clone(&self) -> Print {
                    Print {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Print {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::Print),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Print)?;
                    Ok(Print { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Print))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Print {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Return {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Return {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Return",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Return {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Return {
                #[inline]
                fn eq(&self, other: &Return) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Return {
                #[inline]
                fn clone(&self) -> Return {
                    Return {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Return {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::Return),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Return)?;
                    Ok(Return { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Return))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Return {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Super {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Super {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Super",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Super {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Super {
                #[inline]
                fn eq(&self, other: &Super) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Super {
                #[inline]
                fn clone(&self) -> Super {
                    Super {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Super {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::Super),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Super)?;
                    Ok(Super { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Super))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Super {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct This {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for This {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "This",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for This {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for This {
                #[inline]
                fn eq(&self, other: &This) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for This {
                #[inline]
                fn clone(&self) -> This {
                    This {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for This {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::This),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::This)?;
                    Ok(This { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::This))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for This {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct True {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for True {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "True",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for True {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for True {
                #[inline]
                fn eq(&self, other: &True) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for True {
                #[inline]
                fn clone(&self) -> True {
                    True {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for True {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::True),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::True)?;
                    Ok(True { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::True))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for True {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Var {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Var {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Var",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Var {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Var {
                #[inline]
                fn eq(&self, other: &Var) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Var {
                #[inline]
                fn clone(&self) -> Var {
                    Var {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Var {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::Var),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Var)?;
                    Ok(Var { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Var))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Var {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct While {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for While {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "While",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for While {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for While {
                #[inline]
                fn eq(&self, other: &While) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for While {
                #[inline]
                fn clone(&self) -> While {
                    While {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for While {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::While),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::While)?;
                    Ok(While { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::While))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for While {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Number {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Number {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Number",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Number {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Number {
                #[inline]
                fn eq(&self, other: &Number) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Number {
                #[inline]
                fn clone(&self) -> Number {
                    Number {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Number {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::Number),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Number)?;
                    Ok(Number { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Number))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Number {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct String {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for String {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "String",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for String {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for String {
                #[inline]
                fn eq(&self, other: &String) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for String {
                #[inline]
                fn clone(&self) -> String {
                    String {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for String {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(crate::tokenizer::TokenEnum::String),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::String)?;
                    Ok(String { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::String))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for String {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
            pub struct Identifier {
                pub token: crate::tokenizer::Token,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Identifier {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Identifier",
                        "token",
                        &&self.token,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Identifier {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Identifier {
                #[inline]
                fn eq(&self, other: &Identifier) -> bool {
                    self.token == other.token
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Identifier {
                #[inline]
                fn clone(&self) -> Identifier {
                    Identifier {
                        token: ::core::clone::Clone::clone(&self.token),
                    }
                }
            }
            impl crate::syntax_analysis::parsing::parse_stream::Parser for Identifier {
                fn parse(
                    input: &mut crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> Result<Self> {
                    let token = input
                        .advance()
                        .ok_or(
                            ParseError::NoToken(
                                NoTokenError::no_token(
                                    ExpectedEnum::Token(
                                        crate::tokenizer::TokenEnum::Identifier,
                                    ),
                                    None,
                                ),
                            ),
                        )?;
                    token.parse(crate::tokenizer::TokenEnum::Identifier)?;
                    Ok(Identifier { token: token.clone() })
                }
                fn peek(
                    input: &crate::syntax_analysis::parsing::parse_stream::ParseStream,
                ) -> bool {
                    input
                        .peek1()
                        .map(|t| t.peek(crate::tokenizer::TokenEnum::Identifier))
                        .unwrap_or(false)
                }
            }
            impl std::fmt::Display for Identifier {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.token))
                }
            }
        }
        pub use parse_error::{ExpectedEnum, ParseError, Result, UnexpectedTokenError};
        pub use parse_stream::ParseStream;
        pub use parse_stream::Parser;
    }
    use parsing::primitives::{
        Bang, BangEqual, EqualEqual, False, Greater, GreaterEqual, LeftParen, Less,
        LessEqual, Minus, Nil, Number, Plus, RightParen, Slash, Star, String, True,
    };
    pub use parsing::{
        ExpectedEnum, ParseError, ParseStream, Parser, Result, UnexpectedTokenError,
    };
    use crate::tokenizer::Token;
    pub enum PrimaryExpressionType {
        Number,
        String,
        True,
        False,
        Nil,
        Expression(Expression),
        None,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PrimaryExpressionType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                PrimaryExpressionType::Number => {
                    ::core::fmt::Formatter::write_str(f, "Number")
                }
                PrimaryExpressionType::String => {
                    ::core::fmt::Formatter::write_str(f, "String")
                }
                PrimaryExpressionType::True => {
                    ::core::fmt::Formatter::write_str(f, "True")
                }
                PrimaryExpressionType::False => {
                    ::core::fmt::Formatter::write_str(f, "False")
                }
                PrimaryExpressionType::Nil => ::core::fmt::Formatter::write_str(f, "Nil"),
                PrimaryExpressionType::Expression(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Expression",
                        &__self_0,
                    )
                }
                PrimaryExpressionType::None => {
                    ::core::fmt::Formatter::write_str(f, "None")
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for PrimaryExpressionType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for PrimaryExpressionType {
        #[inline]
        fn eq(&self, other: &PrimaryExpressionType) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (
                        PrimaryExpressionType::Expression(__self_0),
                        PrimaryExpressionType::Expression(__arg1_0),
                    ) => __self_0 == __arg1_0,
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PrimaryExpressionType {
        #[inline]
        fn clone(&self) -> PrimaryExpressionType {
            match self {
                PrimaryExpressionType::Number => PrimaryExpressionType::Number,
                PrimaryExpressionType::String => PrimaryExpressionType::String,
                PrimaryExpressionType::True => PrimaryExpressionType::True,
                PrimaryExpressionType::False => PrimaryExpressionType::False,
                PrimaryExpressionType::Nil => PrimaryExpressionType::Nil,
                PrimaryExpressionType::Expression(__self_0) => {
                    PrimaryExpressionType::Expression(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                PrimaryExpressionType::None => PrimaryExpressionType::None,
            }
        }
    }
    pub struct PrimaryExpression {
        pub token_type: PrimaryExpressionType,
        pub token_list: Vec<Token>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PrimaryExpression {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "PrimaryExpression",
                "token_type",
                &self.token_type,
                "token_list",
                &&self.token_list,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for PrimaryExpression {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for PrimaryExpression {
        #[inline]
        fn eq(&self, other: &PrimaryExpression) -> bool {
            self.token_type == other.token_type && self.token_list == other.token_list
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PrimaryExpression {
        #[inline]
        fn clone(&self) -> PrimaryExpression {
            PrimaryExpression {
                token_type: ::core::clone::Clone::clone(&self.token_type),
                token_list: ::core::clone::Clone::clone(&self.token_list),
            }
        }
    }
    impl Parser for PrimaryExpression {
        fn parse(input: &mut ParseStream) -> Result<PrimaryExpression> {
            fn do_parse(input: &mut ParseStream) -> Result<PrimaryExpression> {
                let mut type_variant = PrimaryExpressionType::None;
                let mut tokens_list: std::collections::LinkedList<
                    crate::tokenizer::Token,
                > = std::collections::LinkedList::new();
                if input.peek::<Number>() {
                    tokens_list.push_back(input.parse::<Number>()?.token.clone());
                    type_variant = PrimaryExpressionType::Number;
                } else if input.peek::<String>() {
                    tokens_list.push_back(input.parse::<String>()?.token.clone());
                    type_variant = PrimaryExpressionType::String;
                } else if input.peek::<True>() {
                    tokens_list.push_back(input.parse::<True>()?.token.clone());
                    type_variant = PrimaryExpressionType::True;
                } else if input.peek::<False>() {
                    tokens_list.push_back(input.parse::<False>()?.token.clone());
                    type_variant = PrimaryExpressionType::False;
                } else if input.peek::<Nil>() {
                    tokens_list.push_back(input.parse::<Nil>()?.token.clone());
                    type_variant = PrimaryExpressionType::Nil;
                } else {
                    tokens_list.push_back(input.parse::<LeftParen>()?.token.clone());
                    let expression = input.parse::<Expression>()?;
                    tokens_list.push_back(input.parse::<RightParen>()?.token.clone());
                    type_variant = PrimaryExpressionType::Expression(expression);
                }
                let token_list: Vec<crate::tokenizer::Token> = tokens_list
                    .into_iter()
                    .collect();
                std::result::Result::Ok(PrimaryExpression {
                    token_type: type_variant,
                    token_list,
                })
            }
            do_parse(input)
        }
        fn peek(input: &ParseStream) -> bool {
            input.peek::<Number>() || input.peek::<String>() || input.peek::<True>()
                || input.peek::<False>() || input.peek::<Nil>()
                || input.peek::<LeftParen>()
        }
    }
    impl Display for PrimaryExpression {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self.token_type {
                PrimaryExpressionType::Expression(expr) => {
                    f.write_fmt(format_args!("(group {0})", expr))
                }
                _ => {
                    let token = self.token_list.first().ok_or(std::fmt::Error)?;
                    f.write_fmt(format_args!("{0}", token))
                }
            }
        }
    }
    type UnaryExpressionReference = Box<UnaryExpression>;
    impl Parser for UnaryExpressionReference {
        fn parse(input: &mut ParseStream) -> Result<Self> {
            let unary_expression = input.parse::<UnaryExpression>()?;
            Ok(Box::new(unary_expression))
        }
        fn peek(input: &ParseStream) -> bool {
            input.peek::<UnaryExpression>()
        }
    }
    pub enum UnaryExpressionOrType {
        UnaryExpressionReference(UnaryExpressionReference),
        PrimaryExpression(PrimaryExpression),
        None,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UnaryExpressionOrType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                UnaryExpressionOrType::UnaryExpressionReference(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "UnaryExpressionReference",
                        &__self_0,
                    )
                }
                UnaryExpressionOrType::PrimaryExpression(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "PrimaryExpression",
                        &__self_0,
                    )
                }
                UnaryExpressionOrType::None => {
                    ::core::fmt::Formatter::write_str(f, "None")
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for UnaryExpressionOrType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for UnaryExpressionOrType {
        #[inline]
        fn eq(&self, other: &UnaryExpressionOrType) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (
                        UnaryExpressionOrType::UnaryExpressionReference(__self_0),
                        UnaryExpressionOrType::UnaryExpressionReference(__arg1_0),
                    ) => __self_0 == __arg1_0,
                    (
                        UnaryExpressionOrType::PrimaryExpression(__self_0),
                        UnaryExpressionOrType::PrimaryExpression(__arg1_0),
                    ) => __self_0 == __arg1_0,
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UnaryExpressionOrType {
        #[inline]
        fn clone(&self) -> UnaryExpressionOrType {
            match self {
                UnaryExpressionOrType::UnaryExpressionReference(__self_0) => {
                    UnaryExpressionOrType::UnaryExpressionReference(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                UnaryExpressionOrType::PrimaryExpression(__self_0) => {
                    UnaryExpressionOrType::PrimaryExpression(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                UnaryExpressionOrType::None => UnaryExpressionOrType::None,
            }
        }
    }
    pub struct UnaryExpressionOr {
        pub token_type: UnaryExpressionOrType,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UnaryExpressionOr {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "UnaryExpressionOr",
                "token_type",
                &&self.token_type,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for UnaryExpressionOr {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for UnaryExpressionOr {
        #[inline]
        fn eq(&self, other: &UnaryExpressionOr) -> bool {
            self.token_type == other.token_type
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UnaryExpressionOr {
        #[inline]
        fn clone(&self) -> UnaryExpressionOr {
            UnaryExpressionOr {
                token_type: ::core::clone::Clone::clone(&self.token_type),
            }
        }
    }
    impl Parser for UnaryExpressionOr {
        fn parse(input: &mut ParseStream) -> Result<UnaryExpressionOr> {
            fn do_parse(input: &mut ParseStream) -> Result<UnaryExpressionOr> {
                let mut type_variant = UnaryExpressionOrType::None;
                let mut tokens_list: std::collections::LinkedList<
                    crate::tokenizer::Token,
                > = std::collections::LinkedList::new();
                if input.peek::<UnaryExpressionReference>() {
                    let unary_expression_reference = input
                        .parse::<UnaryExpressionReference>()?;
                    type_variant = UnaryExpressionOrType::UnaryExpressionReference(
                        unary_expression_reference,
                    );
                } else {
                    let primary_expression = input.parse::<PrimaryExpression>()?;
                    type_variant = UnaryExpressionOrType::PrimaryExpression(
                        primary_expression,
                    );
                }
                std::result::Result::Ok(UnaryExpressionOr {
                    token_type: type_variant,
                })
            }
            do_parse(input)
        }
        fn peek(input: &ParseStream) -> bool {
            input.peek::<UnaryExpressionReference>() || input.peek::<PrimaryExpression>()
        }
    }
    impl Display for UnaryExpressionOr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self.token_type {
                UnaryExpressionOrType::UnaryExpressionReference(expr) => {
                    f.write_fmt(format_args!("{0}", expr))
                }
                UnaryExpressionOrType::PrimaryExpression(expr) => {
                    f.write_fmt(format_args!("{0}", expr))
                }
                _ => f.write_fmt(format_args!("")),
            }
        }
    }
    pub enum UnaryExpressionType {
        Bang,
        Minus,
        None,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UnaryExpressionType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    UnaryExpressionType::Bang => "Bang",
                    UnaryExpressionType::Minus => "Minus",
                    UnaryExpressionType::None => "None",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for UnaryExpressionType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for UnaryExpressionType {
        #[inline]
        fn eq(&self, other: &UnaryExpressionType) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UnaryExpressionType {
        #[inline]
        fn clone(&self) -> UnaryExpressionType {
            match self {
                UnaryExpressionType::Bang => UnaryExpressionType::Bang,
                UnaryExpressionType::Minus => UnaryExpressionType::Minus,
                UnaryExpressionType::None => UnaryExpressionType::None,
            }
        }
    }
    pub struct UnaryExpression {
        pub token_type: UnaryExpressionType,
        pub expr: UnaryExpressionOr,
        pub token_list: Vec<Token>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UnaryExpression {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "UnaryExpression",
                "token_type",
                &self.token_type,
                "expr",
                &self.expr,
                "token_list",
                &&self.token_list,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for UnaryExpression {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for UnaryExpression {
        #[inline]
        fn eq(&self, other: &UnaryExpression) -> bool {
            self.token_type == other.token_type && self.expr == other.expr
                && self.token_list == other.token_list
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UnaryExpression {
        #[inline]
        fn clone(&self) -> UnaryExpression {
            UnaryExpression {
                token_type: ::core::clone::Clone::clone(&self.token_type),
                expr: ::core::clone::Clone::clone(&self.expr),
                token_list: ::core::clone::Clone::clone(&self.token_list),
            }
        }
    }
    impl Parser for UnaryExpression {
        fn parse(input: &mut ParseStream) -> Result<UnaryExpression> {
            fn do_parse(input: &mut ParseStream) -> Result<UnaryExpression> {
                let mut type_variant = UnaryExpressionType::None;
                let mut tokens_list: std::collections::LinkedList<
                    crate::tokenizer::Token,
                > = std::collections::LinkedList::new();
                if input.peek::<Bang>() {
                    tokens_list.push_back(input.parse::<Bang>()?.token.clone());
                    type_variant = UnaryExpressionType::Bang;
                } else {
                    tokens_list.push_back(input.parse::<Minus>()?.token.clone());
                    type_variant = UnaryExpressionType::Minus;
                }
                let expr = input.parse::<UnaryExpressionOr>()?;
                let token_list: Vec<crate::tokenizer::Token> = tokens_list
                    .into_iter()
                    .collect();
                std::result::Result::Ok(UnaryExpression {
                    token_type: type_variant,
                    expr,
                    token_list,
                })
            }
            do_parse(input)
        }
        fn peek(input: &ParseStream) -> bool {
            input.peek::<Bang>() || input.peek::<Minus>()
        }
    }
    impl Display for UnaryExpression {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self.token_type {
                UnaryExpressionType::None => f.write_fmt(format_args!("{0}", self.expr)),
                _ => {
                    let token = self.token_list.first().ok_or(std::fmt::Error)?;
                    f.write_fmt(format_args!("({0} {1})", token, self.expr))
                }
            }
        }
    }
    pub enum FactorType {
        Slash,
        Star,
        None,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FactorType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    FactorType::Slash => "Slash",
                    FactorType::Star => "Star",
                    FactorType::None => "None",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for FactorType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for FactorType {
        #[inline]
        fn eq(&self, other: &FactorType) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for FactorType {
        #[inline]
        fn clone(&self) -> FactorType {
            match self {
                FactorType::Slash => FactorType::Slash,
                FactorType::Star => FactorType::Star,
                FactorType::None => FactorType::None,
            }
        }
    }
    pub struct Factor {
        pub token_type: FactorType,
        pub main_unary: UnaryExpression,
        pub unaries: Vec<(FactorType, UnaryExpression)>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Factor {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Factor",
                "token_type",
                &self.token_type,
                "main_unary",
                &self.main_unary,
                "unaries",
                &&self.unaries,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Factor {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Factor {
        #[inline]
        fn eq(&self, other: &Factor) -> bool {
            self.token_type == other.token_type && self.main_unary == other.main_unary
                && self.unaries == other.unaries
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Factor {
        #[inline]
        fn clone(&self) -> Factor {
            Factor {
                token_type: ::core::clone::Clone::clone(&self.token_type),
                main_unary: ::core::clone::Clone::clone(&self.main_unary),
                unaries: ::core::clone::Clone::clone(&self.unaries),
            }
        }
    }
    impl Parser for Factor {
        fn parse(input: &mut ParseStream) -> Result<Factor> {
            fn do_parse(input: &mut ParseStream) -> Result<Factor> {
                let mut type_variant = FactorType::None;
                let mut tokens_list: std::collections::LinkedList<
                    crate::tokenizer::Token,
                > = std::collections::LinkedList::new();
                let main_unary = input.parse::<UnaryExpression>()?;
                let mut unaries = std::collections::LinkedList::new();
                while input.peek::<Slash>() || input.peek::<Star>() {
                    let mut current_type_variant = FactorType::None;
                    if input.peek::<Slash>() {
                        tokens_list.push_back(input.parse::<Slash>()?.token.clone());
                        current_type_variant = FactorType::Slash;
                    } else {
                        tokens_list.push_back(input.parse::<Star>()?.token.clone());
                        current_type_variant = FactorType::Star;
                    }
                    let nt = input.parse::<UnaryExpression>()?;
                    unaries.push_back((current_type_variant, nt));
                }
                let unaries: Vec<_> = unaries.into_iter().collect();
                std::result::Result::Ok(Factor {
                    token_type: type_variant,
                    main_unary,
                    unaries,
                })
            }
            do_parse(input)
        }
        fn peek(input: &ParseStream) -> bool {
            input.peek::<UnaryExpression>()
        }
    }
    impl Display for Factor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let operations = self
                .unaries
                .iter()
                .map(|(t, unary)| {
                    let token_str: &'static str = match t {
                        FactorType::None => "",
                        FactorType::Slash => "/",
                        FactorType::Star => "*",
                    };
                    (token_str, unary.to_string())
                });
            let result = operation_display(
                self.main_unary.to_string().as_str(),
                operations,
            );
            f.write_fmt(format_args!("{0}", result))
        }
    }
    pub enum TermType {
        Minus,
        Plus,
        None,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TermType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    TermType::Minus => "Minus",
                    TermType::Plus => "Plus",
                    TermType::None => "None",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TermType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TermType {
        #[inline]
        fn eq(&self, other: &TermType) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TermType {
        #[inline]
        fn clone(&self) -> TermType {
            match self {
                TermType::Minus => TermType::Minus,
                TermType::Plus => TermType::Plus,
                TermType::None => TermType::None,
            }
        }
    }
    pub struct Term {
        pub token_type: TermType,
        pub main_factor: Factor,
        pub factors: Vec<(TermType, Factor)>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Term {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Term",
                "token_type",
                &self.token_type,
                "main_factor",
                &self.main_factor,
                "factors",
                &&self.factors,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Term {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Term {
        #[inline]
        fn eq(&self, other: &Term) -> bool {
            self.token_type == other.token_type && self.main_factor == other.main_factor
                && self.factors == other.factors
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Term {
        #[inline]
        fn clone(&self) -> Term {
            Term {
                token_type: ::core::clone::Clone::clone(&self.token_type),
                main_factor: ::core::clone::Clone::clone(&self.main_factor),
                factors: ::core::clone::Clone::clone(&self.factors),
            }
        }
    }
    impl Parser for Term {
        fn parse(input: &mut ParseStream) -> Result<Term> {
            fn do_parse(input: &mut ParseStream) -> Result<Term> {
                let mut type_variant = TermType::None;
                let mut tokens_list: std::collections::LinkedList<
                    crate::tokenizer::Token,
                > = std::collections::LinkedList::new();
                let main_factor = input.parse::<Factor>()?;
                let mut factors = std::collections::LinkedList::new();
                while input.peek::<Minus>() || input.peek::<Plus>() {
                    let mut current_type_variant = TermType::None;
                    if input.peek::<Minus>() {
                        tokens_list.push_back(input.parse::<Minus>()?.token.clone());
                        current_type_variant = TermType::Minus;
                    } else {
                        tokens_list.push_back(input.parse::<Plus>()?.token.clone());
                        current_type_variant = TermType::Plus;
                    }
                    let nt = input.parse::<Factor>()?;
                    factors.push_back((current_type_variant, nt));
                }
                let factors: Vec<_> = factors.into_iter().collect();
                std::result::Result::Ok(Term {
                    token_type: type_variant,
                    main_factor,
                    factors,
                })
            }
            do_parse(input)
        }
        fn peek(input: &ParseStream) -> bool {
            input.peek::<Factor>()
        }
    }
    impl Display for Term {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let operations = self
                .factors
                .iter()
                .map(|(t, factor)| {
                    let token_str: &'static str = match t {
                        TermType::None => "",
                        TermType::Minus => "-",
                        TermType::Plus => "+",
                    };
                    (token_str, factor.to_string())
                });
            let result = operation_display(
                self.main_factor.to_string().as_str(),
                operations,
            );
            f.write_fmt(format_args!("{0}", result))
        }
    }
    pub enum ComparisonType {
        Less,
        LessEqual,
        Greater,
        GreaterEqual,
        None,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ComparisonType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ComparisonType::Less => "Less",
                    ComparisonType::LessEqual => "LessEqual",
                    ComparisonType::Greater => "Greater",
                    ComparisonType::GreaterEqual => "GreaterEqual",
                    ComparisonType::None => "None",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ComparisonType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ComparisonType {
        #[inline]
        fn eq(&self, other: &ComparisonType) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ComparisonType {
        #[inline]
        fn clone(&self) -> ComparisonType {
            match self {
                ComparisonType::Less => ComparisonType::Less,
                ComparisonType::LessEqual => ComparisonType::LessEqual,
                ComparisonType::Greater => ComparisonType::Greater,
                ComparisonType::GreaterEqual => ComparisonType::GreaterEqual,
                ComparisonType::None => ComparisonType::None,
            }
        }
    }
    pub struct Comparison {
        pub token_type: ComparisonType,
        pub main_term: Term,
        pub terms: Vec<(ComparisonType, Term)>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Comparison {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Comparison",
                "token_type",
                &self.token_type,
                "main_term",
                &self.main_term,
                "terms",
                &&self.terms,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Comparison {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Comparison {
        #[inline]
        fn eq(&self, other: &Comparison) -> bool {
            self.token_type == other.token_type && self.main_term == other.main_term
                && self.terms == other.terms
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Comparison {
        #[inline]
        fn clone(&self) -> Comparison {
            Comparison {
                token_type: ::core::clone::Clone::clone(&self.token_type),
                main_term: ::core::clone::Clone::clone(&self.main_term),
                terms: ::core::clone::Clone::clone(&self.terms),
            }
        }
    }
    impl Parser for Comparison {
        fn parse(input: &mut ParseStream) -> Result<Comparison> {
            fn do_parse(input: &mut ParseStream) -> Result<Comparison> {
                let mut type_variant = ComparisonType::None;
                let mut tokens_list: std::collections::LinkedList<
                    crate::tokenizer::Token,
                > = std::collections::LinkedList::new();
                let main_term = input.parse::<Term>()?;
                let mut terms = std::collections::LinkedList::new();
                while input.peek::<Less>() || input.peek::<LessEqual>()
                    || input.peek::<Greater>() || input.peek::<GreaterEqual>()
                {
                    let mut current_type_variant = ComparisonType::None;
                    if input.peek::<Less>() {
                        tokens_list.push_back(input.parse::<Less>()?.token.clone());
                        current_type_variant = ComparisonType::Less;
                    } else if input.peek::<LessEqual>() {
                        tokens_list.push_back(input.parse::<LessEqual>()?.token.clone());
                        current_type_variant = ComparisonType::LessEqual;
                    } else if input.peek::<Greater>() {
                        tokens_list.push_back(input.parse::<Greater>()?.token.clone());
                        current_type_variant = ComparisonType::Greater;
                    } else {
                        tokens_list
                            .push_back(input.parse::<GreaterEqual>()?.token.clone());
                        current_type_variant = ComparisonType::GreaterEqual;
                    }
                    let nt = input.parse::<Term>()?;
                    terms.push_back((current_type_variant, nt));
                }
                let terms: Vec<_> = terms.into_iter().collect();
                std::result::Result::Ok(Comparison {
                    token_type: type_variant,
                    main_term,
                    terms,
                })
            }
            do_parse(input)
        }
        fn peek(input: &ParseStream) -> bool {
            input.peek::<Term>()
        }
    }
    impl Display for Comparison {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let operations = self
                .terms
                .iter()
                .map(|(t, term)| {
                    let token_str: &'static str = match t {
                        ComparisonType::None => "",
                        ComparisonType::Less => "<",
                        ComparisonType::LessEqual => "<=",
                        ComparisonType::Greater => ">",
                        ComparisonType::GreaterEqual => ">=",
                    };
                    (token_str, term.to_string())
                });
            let result = operation_display(
                self.main_term.to_string().as_str(),
                operations,
            );
            f.write_fmt(format_args!("{0}", result))
        }
    }
    pub enum EqualityType {
        EqualEqual,
        BangEqual,
        None,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EqualityType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    EqualityType::EqualEqual => "EqualEqual",
                    EqualityType::BangEqual => "BangEqual",
                    EqualityType::None => "None",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EqualityType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EqualityType {
        #[inline]
        fn eq(&self, other: &EqualityType) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EqualityType {
        #[inline]
        fn clone(&self) -> EqualityType {
            match self {
                EqualityType::EqualEqual => EqualityType::EqualEqual,
                EqualityType::BangEqual => EqualityType::BangEqual,
                EqualityType::None => EqualityType::None,
            }
        }
    }
    pub struct Equality {
        pub token_type: EqualityType,
        pub main_comparison: Comparison,
        pub comparisons: Vec<(EqualityType, Comparison)>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Equality {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Equality",
                "token_type",
                &self.token_type,
                "main_comparison",
                &self.main_comparison,
                "comparisons",
                &&self.comparisons,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Equality {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Equality {
        #[inline]
        fn eq(&self, other: &Equality) -> bool {
            self.token_type == other.token_type
                && self.main_comparison == other.main_comparison
                && self.comparisons == other.comparisons
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Equality {
        #[inline]
        fn clone(&self) -> Equality {
            Equality {
                token_type: ::core::clone::Clone::clone(&self.token_type),
                main_comparison: ::core::clone::Clone::clone(&self.main_comparison),
                comparisons: ::core::clone::Clone::clone(&self.comparisons),
            }
        }
    }
    impl Parser for Equality {
        fn parse(input: &mut ParseStream) -> Result<Equality> {
            fn do_parse(input: &mut ParseStream) -> Result<Equality> {
                let mut type_variant = EqualityType::None;
                let mut tokens_list: std::collections::LinkedList<
                    crate::tokenizer::Token,
                > = std::collections::LinkedList::new();
                let main_comparison = input.parse::<Comparison>()?;
                let mut comparisons = std::collections::LinkedList::new();
                while input.peek::<EqualEqual>() || input.peek::<BangEqual>() {
                    let mut current_type_variant = EqualityType::None;
                    if input.peek::<EqualEqual>() {
                        tokens_list
                            .push_back(input.parse::<EqualEqual>()?.token.clone());
                        current_type_variant = EqualityType::EqualEqual;
                    } else {
                        tokens_list.push_back(input.parse::<BangEqual>()?.token.clone());
                        current_type_variant = EqualityType::BangEqual;
                    }
                    let nt = input.parse::<Comparison>()?;
                    comparisons.push_back((current_type_variant, nt));
                }
                let comparisons: Vec<_> = comparisons.into_iter().collect();
                std::result::Result::Ok(Equality {
                    token_type: type_variant,
                    main_comparison,
                    comparisons,
                })
            }
            do_parse(input)
        }
        fn peek(input: &ParseStream) -> bool {
            input.peek::<Comparison>()
        }
    }
    impl Display for Equality {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let operations = self
                .comparisons
                .iter()
                .map(|(t, comparison)| {
                    let token_str: &'static str = match t {
                        EqualityType::None => "",
                        EqualityType::EqualEqual => "==",
                        EqualityType::BangEqual => "!=",
                    };
                    (token_str, comparison.to_string())
                });
            let result = operation_display(
                self.main_comparison.to_string().as_str(),
                operations,
            );
            f.write_fmt(format_args!("{0}", result))
        }
    }
    pub type Expression = Box<Equality>;
    impl Parser for Expression {
        fn parse(input: &mut ParseStream) -> Result<Self> {
            let equality = input.parse::<Equality>()?;
            Ok(Box::new(equality))
        }
        fn peek(input: &ParseStream) -> bool {
            input.peek::<Equality>()
        }
    }
    fn operation_display<T: Iterator<Item = (&'static str, std::string::String)>>(
        initial: &str,
        operations: T,
    ) -> std::string::String {
        let mut result = initial.to_string();
        for (op, next) in operations {
            result = ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(
                    format_args!("({0} {1} {2})", op, result, next),
                );
                res
            });
        }
        result
    }
}
