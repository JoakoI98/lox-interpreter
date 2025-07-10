use std::fmt::Debug;

use ast_leaf::ast_leaf;
mod parsing;

use parsing::primitives::{
    Bang, BangEqual, EqualEqual, False, Greater, GreaterEqual, LeftParen, Less, LessEqual, Minus,
    Nil, Number, Plus, RightParen, Slash, Star, String, True,
};
pub use parsing::{ParseStream, Parser, Result};

#[ast_leaf(("nil" | "STRING"))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TestExpression {
    #[Type]
    pub token_type: TestExpressionType,
}

#[ast_leaf(("NUMBER" | "STRING" | "true" | "false" | "nil" | 1: "(" TestExpression ")" ))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrimaryExpression {
    #[Type]
    pub token_type: PrimaryExpressionType,
}

type UnaryExpressionReference = Box<UnaryExpression>;
impl Parser for UnaryExpressionReference {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let unary_expression = input.parse::<UnaryExpression>()?;
        Ok(Box::new(unary_expression))
    }

    fn peek(input: &ParseStream) -> bool {
        input.peek::<UnaryExpression>()
    }
}

#[ast_leaf(((UnaryExpressionReference | PrimaryExpression)))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnaryExpressionOr {
    #[Type]
    pub token_type: UnaryExpressionOrType,
}

#[ast_leaf(("!" | "-") expr)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnaryExpression {
    #[Type]
    pub token_type: UnaryExpressionType,
    pub expr: UnaryExpressionOr,
}

#[ast_leaf(main_unary (("/" | "*") unaries)*)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Factor {
    #[Type]
    pub token_type: FactorType,
    pub main_unary: UnaryExpression,
    pub unaries: Vec<(FactorType, UnaryExpression)>,
}

#[ast_leaf(main_factor (("-" | "+") factors)*)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Term {
    #[Type]
    pub token_type: TermType,
    pub main_factor: Factor,
    pub factors: Vec<(TermType, Factor)>,
}

#[ast_leaf(main_term (("<" | "<=" | ">" | ">=") terms)*)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Comparison {
    #[Type]
    pub token_type: ComparisonType,
    pub main_term: Term,
    pub terms: Vec<(ComparisonType, Term)>,
}

#[ast_leaf(main_comparison (("==" | "!=") comparisons)*)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Equality {
    #[Type]
    pub token_type: EqualityType,
    pub main_comparison: Comparison,
    pub comparisons: Vec<(EqualityType, Comparison)>,
}

#[ast_leaf(comparison)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Expression {
    #[Type]
    pub token_type: ExpressionType,
    pub comparison: Comparison,
}
