use std::fmt::Debug;

use ast_leaf::ast_leaf;

use super::super::parsing::primitives::{Equal, Identifier, Semicolon, Var};
use super::super::parsing::{ParseStream, Parser, Result};
use super::assignments::Expression;

use super::statement::Statement;
use crate::common::Visitor;
use crate::tokenizer::Token;

#[ast_leaf("var" "IDENT" (("=") expr)? ";")]
#[derive(Debug, PartialEq, Clone)]
pub struct VarDeclaration {
    #[Type]
    pub token_type: VarDeclarationType,
    pub expr: Option<Expression>,
    #[TokenList]
    pub token_list: Vec<Token>,
}

#[ast_leaf((VarDeclaration | Statement))]
#[derive(Debug, PartialEq, Clone)]
pub struct Declaration {
    #[Type]
    pub token_type: DeclarationType,
}

#[ast_leaf((statements)*)]
#[derive(Debug, PartialEq, Clone)]
pub struct ProgramAst {
    #[Type]
    pub token_type: ProgramType,
    pub statements: Vec<(ProgramType, Declaration)>,
}
