use std::fmt::Debug;

use ast_leaf::ast_leaf;

use super::super::parsing::primitives::{LeftBrace, Print, RightBrace, Semicolon};
use super::super::parsing::{ParseStream, Parser, Result};

use super::assignments::Expression;
use crate::syntax_analysis::Declaration;

#[ast_leaf("print" expr ";")]
#[derive(Debug, PartialEq, Clone)]
pub struct PrintStatement {
    #[Type]
    pub token_type: PrintStatementType,
    pub expr: Expression,
}

#[ast_leaf(expr ";")]
#[derive(Debug, PartialEq, Clone)]
pub struct ExprStatement {
    #[Type]
    pub token_type: ExprStatementType,
    pub expr: Expression,
}

#[ast_leaf((ExprStatement | PrintStatement | Block))]
#[derive(Debug, PartialEq, Clone)]
pub struct Statement {
    #[Type]
    pub token_type: StatementType,
}

#[ast_leaf("{" (declarations)* "}")]
#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    #[Type]
    pub token_type: BlockType,
    pub declarations: Vec<(BlockType, Declaration)>,
}
