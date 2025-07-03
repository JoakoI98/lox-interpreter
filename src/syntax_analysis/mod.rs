use ast_leaf::ast_leaf;
mod parsing;

pub use parsing::ParseStream;
use parsing::{Minus, Parser, Plus, Result};

#[ast_leaf(("+" | "-"))]
pub struct PrimaryExpression {
    #[Type]
    pub token: PrimaryExpressionType,
}
