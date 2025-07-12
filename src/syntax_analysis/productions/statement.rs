use std::fmt::Debug;

use ast_leaf::ast_leaf;

use super::super::parsing::primitives::{Print, Semicolon};
use super::super::parsing::{ParseStream, Parser, Result};

use super::expression::Expression;

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

#[ast_leaf((ExprStatement | PrintStatement))]
#[derive(Debug, PartialEq, Clone)]
pub struct Statement {
    #[Type]
    pub token_type: StatementType,
}

#[ast_leaf((statements)*)]
#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    #[Type]
    pub token_type: ProgramType,
    pub statements: Vec<(ProgramType, Statement)>,
}
