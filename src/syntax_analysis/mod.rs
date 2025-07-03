use ast_leaf::ast_leaf;

mod parsing;

use parsing::{Minus, ParseStream, Plus, Result};

pub enum PrimaryExpressionType {
    Plus,
    Minus,
}
struct PrimaryExpression {
    pub token: PrimaryExpressionType,
}
impl PrimaryExpression {
    pub fn parse(input: ParseStream) -> Result<Self> {
        if input.peek::<Plus>() {
            input.parse::<Plus>()?;
        } else if input.peek::<Minus>() {
            input.parse::<Minus>()?;
        }
        Self { token }
    }
}
