use std::fmt::{Debug, Display};

use super::expression::Equality;
use ast_leaf::ast_leaf;

use super::super::parsing::primitives::{Equal, Identifier};
use super::super::parsing::{ParseStream, Parser, Result};

use crate::common::Visitor;
use crate::tokenizer::Token;

pub type AssignmentReference = Box<Assignment>;
impl Parser for AssignmentReference {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let assignment = input.parse::<Assignment>()?;
        Ok(Box::new(assignment))
    }

    fn peek(input: &ParseStream) -> bool {
        input.peek::<Assignment>()
    }
}

#[ast_leaf("IDENT" "=" assignment)]
#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentSelf {
    #[Type]
    pub token_type: AssignmentSelfType,
    pub assignment: AssignmentReference,
    #[TokenList]
    pub token_list: Vec<Token>,
}

impl Display for AssignmentSelf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token = self.token_list.first().ok_or(std::fmt::Error)?;
        write!(f, "({} = {})", token, self.assignment)
    }
}

#[ast_leaf(eq)]
#[derive(Debug, PartialEq, Clone)]
pub struct Assignment {
    #[Type]
    pub token_type: AssignmentType,
    pub eq: Equality,
}

impl Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.eq)
    }
}

pub type Expression = Box<Assignment>;
