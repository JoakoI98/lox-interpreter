use std::fmt::{Debug, Display};

use ast_leaf::ast_leaf;

use super::super::parsing::primitives::{
    And, Bang, BangEqual, EqualEqual, False, Greater, GreaterEqual, Identifier, LeftParen, Less,
    LessEqual, Minus, Nil, Number, Or, Plus, RightParen, Slash, Star, String, True,
};
use super::super::parsing::{
    ExpectedEnum, ParseError, ParseStream, Parser, Result, UnexpectedTokenError,
};
use super::assignments::Expression;

use crate::tokenizer::Token;

#[ast_leaf(( "IDENT" |"NUMBER" | "STRING" | "true" | "false" | "nil" | 1: "(" Expression ")" ))]
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
            PrimaryExpressionType::Expression(expr) => write!(f, "(group {})", expr),
            _ => {
                let token = self.token_list.first().ok_or(std::fmt::Error)?;
                write!(f, "{}", token)
            }
        }
    }
}

pub type UnaryExpressionReference = Box<UnaryExpression>;
impl Parser for UnaryExpressionReference {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let unary_expression = input.parse::<UnaryExpression>()?;
        Ok(Box::new(unary_expression))
    }

    fn peek(input: &ParseStream) -> bool {
        input.peek::<UnaryExpression>()
    }
}

#[ast_leaf((("!" | "-") expr))]
#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpressionSelf {
    #[Type]
    pub token_type: UnaryExpressionSelfType,
    pub expr: UnaryExpressionReference,
    #[TokenList]
    pub token_list: Vec<Token>,
}

impl Display for UnaryExpressionSelf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token = self.token_list.first().ok_or(std::fmt::Error)?;
        write!(f, "({} {})", token, self.expr)
    }
}

#[ast_leaf((UnaryExpressionSelf | PrimaryExpression))]
#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpression {
    #[Type]
    pub token_type: UnaryExpressionType,
}

impl Display for UnaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.token_type {
            UnaryExpressionType::PrimaryExpression(expr) => write!(f, "{}", expr),
            UnaryExpressionType::UnaryExpressionSelf(expr) => write!(f, "{}", expr),
            _ => write!(f, ""),
        }
    }
}

#[ast_leaf(main_unary (("/" | "*") unaries)*)]
#[derive(Debug, PartialEq, Clone)]
pub struct Factor {
    #[Type]
    pub token_type: FactorType,
    pub main_unary: UnaryExpression,
    pub unaries: Vec<(FactorType, UnaryExpression)>,
}

impl Display for Factor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operations = self.unaries.iter().map(|(t, unary)| {
            let token_str: &'static str = match t {
                FactorType::None => "",
                FactorType::Slash => "/",
                FactorType::Star => "*",
            };
            (token_str, unary.to_string())
        });
        let result = operation_display(self.main_unary.to_string().as_str(), operations);
        write!(f, "{}", result)
    }
}

#[ast_leaf(main_factor (("-" | "+") factors)*)]
#[derive(Debug, PartialEq, Clone)]
pub struct Term {
    #[Type]
    pub token_type: TermType,
    pub main_factor: Factor,
    pub factors: Vec<(TermType, Factor)>,
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operations = self.factors.iter().map(|(t, factor)| {
            let token_str: &'static str = match t {
                TermType::None => "",
                TermType::Minus => "-",
                TermType::Plus => "+",
            };
            (token_str, factor.to_string())
        });
        let result = operation_display(self.main_factor.to_string().as_str(), operations);
        write!(f, "{}", result)
    }
}

#[ast_leaf(main_term (("<" | "<=" | ">" | ">=") terms)*)]
#[derive(Debug, PartialEq, Clone)]
pub struct Comparison {
    #[Type]
    pub token_type: ComparisonType,
    pub main_term: Term,
    pub terms: Vec<(ComparisonType, Term)>,
}

impl Display for Comparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operations = self.terms.iter().map(|(t, term)| {
            let token_str: &'static str = match t {
                ComparisonType::None => "",
                ComparisonType::Less => "<",
                ComparisonType::LessEqual => "<=",
                ComparisonType::Greater => ">",
                ComparisonType::GreaterEqual => ">=",
            };
            (token_str, term.to_string())
        });
        let result = operation_display(self.main_term.to_string().as_str(), operations);
        write!(f, "{}", result)
    }
}

#[ast_leaf(main_comparison (("==" | "!=") comparisons)*)]
#[derive(Debug, PartialEq, Clone)]
#[SyncError = "expression"]
pub struct Equality {
    #[Type]
    pub token_type: EqualityType,
    pub main_comparison: Comparison,
    pub comparisons: Vec<(EqualityType, Comparison)>,
}

impl Display for Equality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operations = self.comparisons.iter().map(|(t, comparison)| {
            let token_str: &'static str = match t {
                EqualityType::None => "",
                EqualityType::EqualEqual => "==",
                EqualityType::BangEqual => "!=",
            };
            (token_str, comparison.to_string())
        });
        let result = operation_display(self.main_comparison.to_string().as_str(), operations);
        write!(f, "{}", result)
    }
}

#[ast_leaf(main_equality (("and") equalities)*)]
#[derive(Debug, PartialEq, Clone)]
pub struct LogicalAnd {
    #[Type]
    pub token_type: LogicalAndType,
    pub main_equality: Equality,
    pub equalities: Vec<(LogicalAndType, Equality)>,
}

impl Display for LogicalAnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operations = self.equalities.iter().map(|(t, equality)| {
            let token_str: &'static str = match t {
                LogicalAndType::None => "",
                LogicalAndType::And => "and",
            };
            (token_str, equality.to_string())
        });
        let result = operation_display(self.main_equality.to_string().as_str(), operations);
        write!(f, "{}", result)
    }
}

#[ast_leaf(main_and (("or") ands)*)]
#[derive(Debug, PartialEq, Clone)]
pub struct LogicalOr {
    #[Type]
    pub token_type: LogicalOrType,
    pub main_and: LogicalAnd,
    pub ands: Vec<(LogicalOrType, LogicalAnd)>,
}

impl Display for LogicalOr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operations = self.ands.iter().map(|(t, and)| {
            let token_str: &'static str = match t {
                LogicalOrType::None => "",
                LogicalOrType::Or => "or",
            };
            (token_str, and.to_string())
        });
        let result = operation_display(self.main_and.to_string().as_str(), operations);
        write!(f, "{}", result)
    }
}

fn operation_display<T: Iterator<Item = (&'static str, std::string::String)>>(
    initial: &str,
    operations: T,
) -> std::string::String {
    let mut result = initial.to_string();
    for (op, next) in operations {
        result = format!("({} {} {})", op, result, next);
    }
    result
}
