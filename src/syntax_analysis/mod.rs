use ast_leaf::ast_leaf;
mod parsing;

use parsing::primitives::{False, LeftParen, Nil, Number, RightParen, String, True};
pub use parsing::{ParseStream, Parser, Result};

#[ast_leaf(("NUMBER" | "STRING" | "true" | "false" ))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrimaryExpression {
    #[Type]
    pub token_type: PrimaryExpressionType,
}

#[ast_leaf((("nil" | 1: "(" PrimaryExpression ")"))+)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnaryExpression {
    #[Type]
    pub token_type: UnaryExpressionType,
    pub pe: Vec<PrimaryExpression>,
}
