use std::fmt::{Debug, Display};

use super::expression::LogicalOr;

use super::super::parsing::{ParseStream, Parser, Result};

use crate::syntax_analysis::parsing::primitives::{Equal, Identifier};
use crate::tokenizer::{Token, TokenEnum};

#[derive(Debug, PartialEq, Clone)]
pub enum Assignment {
    Assignment(Box<Assignment>, Identifier),
    Evaluable(LogicalOr),
}
impl crate::common::Visitable for Assignment {}
impl Parser for Assignment {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let is_assignment = input
            .peek_n(2)
            .map(|t| t.token_type == TokenEnum::Equal)
            .unwrap_or(false);
        if is_assignment {
            let identifier = input.parse::<Identifier>()?;
            input.parse::<Equal>()?;
            let assignment = input.parse::<Assignment>()?;
            Ok(Assignment::Assignment(Box::new(assignment), identifier))
        } else {
            let evaluable = input.parse::<LogicalOr>()?;
            Ok(Assignment::Evaluable(evaluable))
        }
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|token| token.token_type == TokenEnum::Identifier)
            .unwrap_or(false)
            || input.peek::<LogicalOr>()
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Assignment::Assignment(assignment, identifier) => {
                write!(f, "{} = {}", identifier, assignment)
            }
            Assignment::Evaluable(evaluable) => write!(f, "{}", evaluable),
        }
    }
}

pub type Expression = Box<Assignment>;

impl Parser for Expression {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let assignment = input.parse::<Assignment>()?;
        Ok(Box::new(assignment))
    }

    fn peek(input: &ParseStream) -> bool {
        input.peek::<Assignment>()
    }
}
