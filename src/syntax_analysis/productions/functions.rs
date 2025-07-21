use std::collections::LinkedList;
use std::fmt::Display;

use super::super::parsing::{ParseStream, Parser, Result};
use ast_leaf::ast_leaf;

use crate::syntax_analysis::parsing::primitives::{Comma, Dot, Identifier, LeftParen, RightParen};
use crate::syntax_analysis::{Block, Expression, PrimaryExpression};
use crate::tokenizer::Token;

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
    #[TokenList]
    pub token_list: Vec<Token>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Accessor {
    pub identifier: Identifier,
}

impl Parser for Accessor {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        input.parse::<Dot>()?;
        let identifier = input.parse::<Identifier>()?;
        Ok(Accessor { identifier })
    }

    fn peek(input: &ParseStream) -> bool {
        input.peek::<Dot>()
    }
}

#[ast_leaf((Accessor | ArgumentsList))]
#[derive(Debug, PartialEq, Clone)]
pub struct AccessorOrArguments {
    #[Type]
    pub ty: AccessorOrArgumentsType,
}

#[ast_leaf(primary (accessor_or_arguments)*)]
#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    #[Type]
    pub token_type: CallType,
    pub primary: PrimaryExpression,
    pub accessor_or_arguments: Vec<(CallType, AccessorOrArguments)>,
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.primary)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parameters {
    pub parameters: Vec<Identifier>,
}

impl Parser for Parameters {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let mut parameters = LinkedList::new();
        if input.peek::<Identifier>() {
            parameters.push_back(input.parse::<Identifier>()?);
        }
        while input.peek::<Comma>() {
            input.parse::<Comma>()?;
            parameters.push_back(input.parse::<Identifier>()?);
        }
        Ok(Parameters {
            parameters: parameters.into_iter().collect(),
        })
    }

    fn peek(_input: &ParseStream) -> bool {
        true
    }
}

impl Display for Parameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.parameters
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[ast_leaf("IDENT" "(" parameters ")" block)]
#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    #[Type]
    pub ty: FunctionType,
    pub block: Block,
    pub parameters: Parameters,
    #[TokenList]
    pub token_list: Vec<Token>,
}
