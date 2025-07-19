use super::super::parsing::{ParseStream, Parser, Result};
use ast_leaf::ast_leaf;

use crate::syntax_analysis::parsing::primitives::{Comma, LeftParen, RightParen};
use crate::syntax_analysis::{Expression, PrimaryExpression};

#[ast_leaf(first ((",") rest)*)]
#[derive(Debug, PartialEq, Clone)]
pub struct Arguments {
    #[Type]
    pub ty: ArgumentsType,
    pub first: Expression,
    pub rest: Vec<(ArgumentsType, Expression)>,
}

type MaybeArguments = Option<Arguments>;

impl Parser for MaybeArguments {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        if !input.peek::<Arguments>() {
            return Ok(None);
        }
        let arguments = input.parse::<Arguments>()?;
        Ok(Some(arguments))
    }

    fn peek(_: &ParseStream) -> bool {
        true
    }
}

#[ast_leaf("(" maybe_arguments ")")]
#[derive(Debug, PartialEq, Clone)]
pub struct ArgumentsList {
    #[Type]
    pub ty: ArgumentsListType,
    pub maybe_arguments: MaybeArguments,
}

#[ast_leaf(primary (arguments_list)*)]
#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    #[Type]
    pub token_type: CallType,
    pub primary: PrimaryExpression,
    pub arguments_list: Vec<(CallType, ArgumentsList)>,
}
