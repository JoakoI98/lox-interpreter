use std::fmt::Debug;

use ast_leaf::ast_leaf;

use super::super::parsing::primitives::{
    Else, For, If, LeftBrace, LeftParen, Print, Return, RightBrace, RightParen, Semicolon, While,
};
use super::super::parsing::{ParseStream, Parser, Result};

use super::assignments::Expression;
use crate::syntax_analysis::{Declaration, VarDeclaration};

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

#[ast_leaf((ExprStatement | PrintStatement | Block | IfStatement | WhileStatement | ForStatement | ReturnStatement))]
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

type StatementReference = Box<Statement>;

impl Parser for StatementReference {
    fn parse(stream: &mut ParseStream) -> Result<Self> {
        let statement = stream.parse::<Statement>()?;
        Ok(Box::new(statement))
    }

    fn peek(stream: &ParseStream) -> bool {
        stream.peek::<Statement>()
    }
}

#[ast_leaf("if" "(" bool_expr ")" true_statement (("else") false_statement)?)]
#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement {
    #[Type]
    pub token_type: IfStatementType,
    pub bool_expr: Expression,
    pub true_statement: StatementReference,
    pub false_statement: Option<StatementReference>,
}

#[ast_leaf("while" "(" eval_expr ")" statement )]
#[derive(Debug, PartialEq, Clone)]
pub struct WhileStatement {
    #[Type]
    pub token_type: WhileStatementType,
    pub eval_expr: Expression,
    pub statement: StatementReference,
}

#[ast_leaf((expr)?)]
#[derive(Debug, PartialEq, Clone)]
pub struct MaybeExpression {
    #[Type]
    pub token_type: MaybeExpressionType,
    pub expr: Option<Expression>,
}

#[ast_leaf("for" "(" ((VarDeclaration | ExprStatement | ";")) condition ";" increment ")" statement)]
#[derive(Debug, PartialEq, Clone)]
pub struct ForStatement {
    #[Type]
    pub token_type: ForStatementType,
    pub condition: MaybeExpression,
    pub increment: MaybeExpression,
    pub statement: StatementReference,
}

#[ast_leaf("return" (expr)?";")]
#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatement {
    #[Type]
    pub token_type: ReturnStatementType,
    pub expr: Option<Expression>,
}
