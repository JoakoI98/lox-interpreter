use crate::{
    evaluation::{resolver::Resolver, run::RunState, RuntimeError, RuntimeValue},
    tokenizer::{Token, TokenValue},
};

pub trait Evaluable: std::fmt::Debug {
    fn eval(&self, state: &RunState) -> Result<RuntimeValue, RuntimeError>;
}

#[derive(Debug)]
pub struct EvaluableIdentifier {
    identifier: String,
    line: usize,
    depth: Option<usize>,
}

impl EvaluableIdentifier {
    pub fn from_raw_token(token: &Token, resolver: &Resolver) -> Result<Self, RuntimeError> {
        let identifier_string = match &token.token_value {
            TokenValue::Identifier(_) => token.lexeme.clone(),
            _ => return Err(RuntimeError::ASTInvalidStructure),
        };
        Ok(Self {
            depth: resolver.resolve(&identifier_string)?,
            identifier: identifier_string,
            line: token.line,
        })
    }

    #[inline]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    #[inline]
    pub fn line(&self) -> usize {
        self.line
    }

    #[inline]
    pub fn depth(&self) -> Option<usize> {
        self.depth
    }
}
