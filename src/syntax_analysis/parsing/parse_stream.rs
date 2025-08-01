use std::path::Display;

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

    pub fn peek_n(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.current_index + n - 1)
    }

    pub fn run_and_restore<R, F>(&mut self, f: F) -> R
    where
        F: Fn(&mut ParseStream) -> (R, bool),
    {
        let index = self.current_index;
        let (result, success) = f(self);
        if !success {
            self.current_index = index;
        }
        result
    }
}

impl std::fmt::Display for ParseStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in self.tokens.iter().skip(self.current_index) {
            write!(f, "{}", token.lexeme)?;
        }
        Ok(())
    }
}
