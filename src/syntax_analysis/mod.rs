use std::fmt::{Debug, Display};

use ast_leaf::ast_leaf;
mod parsing;

use parsing::primitives::{
    Bang, BangEqual, EqualEqual, False, Greater, GreaterEqual, LeftParen, Less, LessEqual, Minus,
    Nil, Number, Plus, RightParen, Slash, Star, String, True,
};
pub use parsing::{ParseStream, Parser, Result};

use crate::tokenizer::Token;

#[ast_leaf(("NUMBER" | "STRING" | "true" | "false" | "nil" | 1: "(" Expression ")" ))]
#[derive(Debug, PartialEq, Clone)]
pub struct PrimaryExpression {
    #[Type]
    pub token_type: PrimaryExpressionType,
    #[TokenList]
    pub token_list: Vec<Token>,
}

impl Display for PrimaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.token_type {
            PrimaryExpressionType::Expression(expr) => write!(f, "{}", expr),
            _ => {
                let token = self.token_list.first().ok_or(std::fmt::Error)?;
                write!(f, "{}", token)
            }
        }
    }
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
#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpressionOr {
    #[Type]
    pub token_type: UnaryExpressionOrType,
}

#[ast_leaf(("!" | "-") expr)]
#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpression {
    #[Type]
    pub token_type: UnaryExpressionType,
    pub expr: UnaryExpressionOr,
}

#[ast_leaf(main_unary (("/" | "*") unaries)*)]
#[derive(Debug, PartialEq, Clone)]
pub struct Factor {
    #[Type]
    pub token_type: FactorType,
    pub main_unary: UnaryExpression,
    pub unaries: Vec<(FactorType, UnaryExpression)>,
}

#[ast_leaf(main_factor (("-" | "+") factors)*)]
#[derive(Debug, PartialEq, Clone)]
pub struct Term {
    #[Type]
    pub token_type: TermType,
    pub main_factor: Factor,
    pub factors: Vec<(TermType, Factor)>,
}

#[ast_leaf(main_term (("<" | "<=" | ">" | ">=") terms)*)]
#[derive(Debug, PartialEq, Clone)]
pub struct Comparison {
    #[Type]
    pub token_type: ComparisonType,
    pub main_term: Term,
    pub terms: Vec<(ComparisonType, Term)>,
}

#[ast_leaf(main_comparison (("==" | "!=") comparisons)*)]
#[derive(Debug, PartialEq, Clone)]
pub struct Equality {
    #[Type]
    pub token_type: EqualityType,
    pub main_comparison: Comparison,
    pub comparisons: Vec<(EqualityType, Comparison)>,
}

type Expression = Box<Equality>;
impl Parser for Expression {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let equality = input.parse::<Equality>()?;
        Ok(Box::new(equality))
    }

    fn peek(input: &ParseStream) -> bool {
        input.peek::<Equality>()
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EXPR")
    }
}
