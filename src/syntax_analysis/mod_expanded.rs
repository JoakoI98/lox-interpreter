use std::fmt::{Debug, Display};

use ast_leaf::ast_leaf;
mod parsing;

use parsing::primitives::{
    Bang, BangEqual, EqualEqual, False, Greater, GreaterEqual, LeftParen, Less, LessEqual, Minus,
    Nil, Number, Plus, RightParen, Slash, Star, String, True,
};
pub use parsing::{ExpectedEnum, ParseError, ParseStream, Parser, Result, UnexpectedTokenError};

use crate::tokenizer::Token;

pub enum PrimaryExpressionType {
    Number,
    String,
    True,
    False,
    Nil,
    Expression(Expression),
    None,
}
pub struct PrimaryExpression {
    pub token_type: PrimaryExpressionType,
    pub token_list: Vec<Token>,
}

impl Parser for PrimaryExpression {
    fn parse(input: &mut ParseStream) -> Result<PrimaryExpression> {
        fn do_parse(input: &mut ParseStream) -> Result<PrimaryExpression> {
            let mut type_variant = PrimaryExpressionType::None;
            let mut tokens_list: std::collections::LinkedList<crate::tokenizer::Token> =
                std::collections::LinkedList::new();
            if input.peek::<Number>() {
                tokens_list.push_back(input.parse::<Number>()?.token.clone());
                type_variant = PrimaryExpressionType::Number;
            } else if input.peek::<String>() {
                tokens_list.push_back(input.parse::<String>()?.token.clone());
                type_variant = PrimaryExpressionType::String;
            } else if input.peek::<True>() {
                tokens_list.push_back(input.parse::<True>()?.token.clone());
                type_variant = PrimaryExpressionType::True;
            } else if input.peek::<False>() {
                tokens_list.push_back(input.parse::<False>()?.token.clone());
                type_variant = PrimaryExpressionType::False;
            } else if input.peek::<Nil>() {
                tokens_list.push_back(input.parse::<Nil>()?.token.clone());
                type_variant = PrimaryExpressionType::Nil;
            } else {
                tokens_list.push_back(input.parse::<LeftParen>()?.token.clone());
                let expression = input.parse::<Expression>()?;
                tokens_list.push_back(input.parse::<RightParen>()?.token.clone());
                type_variant = PrimaryExpressionType::Expression(expression);
            }
            let token_list: Vec<crate::tokenizer::Token> = tokens_list.into_iter().collect();
            std::result::Result::Ok(PrimaryExpression {
                token_type: type_variant,
                token_list,
            })
        }
        do_parse(input)
    }
    fn peek(input: &ParseStream) -> bool {
        input.peek::<Number>()
            || input.peek::<String>()
            || input.peek::<True>()
            || input.peek::<False>()
            || input.peek::<Nil>()
            || input.peek::<LeftParen>()
    }
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

pub enum UnaryExpressionOrType {
    UnaryExpressionReference(UnaryExpressionReference),
    PrimaryExpression(PrimaryExpression),
    None,
}
pub struct UnaryExpressionOr {
    pub token_type: UnaryExpressionOrType,
}
impl Parser for UnaryExpressionOr {
    fn parse(input: &mut ParseStream) -> Result<UnaryExpressionOr> {
        fn do_parse(input: &mut ParseStream) -> Result<UnaryExpressionOr> {
            let mut type_variant = UnaryExpressionOrType::None;
            let mut tokens_list: std::collections::LinkedList<crate::tokenizer::Token> =
                std::collections::LinkedList::new();
            if input.peek::<UnaryExpressionReference>() {
                let unary_expression_reference = input.parse::<UnaryExpressionReference>()?;
                type_variant =
                    UnaryExpressionOrType::UnaryExpressionReference(unary_expression_reference);
            } else {
                let primary_expression = input.parse::<PrimaryExpression>()?;
                type_variant = UnaryExpressionOrType::PrimaryExpression(primary_expression);
            }
            std::result::Result::Ok(UnaryExpressionOr {
                token_type: type_variant,
            })
        }
        do_parse(input)
    }
    fn peek(input: &ParseStream) -> bool {
        input.peek::<UnaryExpressionReference>() || input.peek::<PrimaryExpression>()
    }
}

impl Display for UnaryExpressionOr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.token_type {
            UnaryExpressionOrType::UnaryExpressionReference(expr) => write!(f, "{}", expr),
            UnaryExpressionOrType::PrimaryExpression(expr) => write!(f, "{}", expr),
            _ => write!(f, ""),
        }
    }
}

pub enum UnaryExpressionType {
    Bang,
    Minus,
    None,
}
pub struct UnaryExpression {
    pub token_type: UnaryExpressionType,
    pub expr: UnaryExpressionOr,
    pub token_list: Vec<Token>,
}
impl Parser for UnaryExpression {
    fn parse(input: &mut ParseStream) -> Result<UnaryExpression> {
        fn do_parse(input: &mut ParseStream) -> Result<UnaryExpression> {
            let mut type_variant = UnaryExpressionType::None;
            let mut tokens_list: std::collections::LinkedList<crate::tokenizer::Token> =
                std::collections::LinkedList::new();
            if input.peek::<Bang>() {
                tokens_list.push_back(input.parse::<Bang>()?.token.clone());
                type_variant = UnaryExpressionType::Bang;
            } else {
                tokens_list.push_back(input.parse::<Minus>()?.token.clone());
                type_variant = UnaryExpressionType::Minus;
            }
            let expr = input.parse::<UnaryExpressionOr>()?;
            let token_list: Vec<crate::tokenizer::Token> = tokens_list.into_iter().collect();
            std::result::Result::Ok(UnaryExpression {
                token_type: type_variant,
                expr,
                token_list,
            })
        }
        do_parse(input)
    }
    fn peek(input: &ParseStream) -> bool {
        input.peek::<Bang>() || input.peek::<Minus>()
    }
}

impl Display for UnaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.token_type {
            UnaryExpressionType::None => write!(f, "{}", self.expr),
            _ => {
                let token = self.token_list.first().ok_or(std::fmt::Error)?;
                write!(f, "({} {})", token, self.expr)
            }
        }
    }
}

pub enum FactorType {
    Slash,
    Star,
    None,
}
pub struct Factor {
    pub token_type: FactorType,
    pub main_unary: UnaryExpression,
    pub unaries: Vec<(FactorType, UnaryExpression)>,
}
impl Parser for Factor {
    fn parse(input: &mut ParseStream) -> Result<Factor> {
        fn do_parse(input: &mut ParseStream) -> Result<Factor> {
            let mut type_variant = FactorType::None;
            let mut tokens_list: std::collections::LinkedList<crate::tokenizer::Token> =
                std::collections::LinkedList::new();
            let main_unary = input.parse::<UnaryExpression>()?;
            let mut unaries = std::collections::LinkedList::new();
            while input.peek::<Slash>() || input.peek::<Star>() {
                let mut current_type_variant = FactorType::None;
                if input.peek::<Slash>() {
                    tokens_list.push_back(input.parse::<Slash>()?.token.clone());
                    current_type_variant = FactorType::Slash;
                } else {
                    tokens_list.push_back(input.parse::<Star>()?.token.clone());
                    current_type_variant = FactorType::Star;
                }
                let nt = input.parse::<UnaryExpression>()?;
                unaries.push_back((current_type_variant, nt));
            }
            let unaries: Vec<_> = unaries.into_iter().collect();
            std::result::Result::Ok(Factor {
                token_type: type_variant,
                main_unary,
                unaries,
            })
        }
        do_parse(input)
    }
    fn peek(input: &ParseStream) -> bool {
        input.peek::<UnaryExpression>()
    }
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

pub enum TermType {
    Minus,
    Plus,
    None,
}
pub struct Term {
    pub token_type: TermType,
    pub main_factor: Factor,
    pub factors: Vec<(TermType, Factor)>,
}
impl Parser for Term {
    fn parse(input: &mut ParseStream) -> Result<Term> {
        fn do_parse(input: &mut ParseStream) -> Result<Term> {
            let mut type_variant = TermType::None;
            let mut tokens_list: std::collections::LinkedList<crate::tokenizer::Token> =
                std::collections::LinkedList::new();
            let main_factor = input.parse::<Factor>()?;
            let mut factors = std::collections::LinkedList::new();
            while input.peek::<Minus>() || input.peek::<Plus>() {
                let mut current_type_variant = TermType::None;
                if input.peek::<Minus>() {
                    tokens_list.push_back(input.parse::<Minus>()?.token.clone());
                    current_type_variant = TermType::Minus;
                } else {
                    tokens_list.push_back(input.parse::<Plus>()?.token.clone());
                    current_type_variant = TermType::Plus;
                }
                let nt = input.parse::<Factor>()?;
                factors.push_back((current_type_variant, nt));
            }
            let factors: Vec<_> = factors.into_iter().collect();
            std::result::Result::Ok(Term {
                token_type: type_variant,
                main_factor,
                factors,
            })
        }
        do_parse(input)
    }
    fn peek(input: &ParseStream) -> bool {
        input.peek::<Factor>()
    }
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

pub enum ComparisonType {
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    None,
}
pub struct Comparison {
    pub token_type: ComparisonType,
    pub main_term: Term,
    pub terms: Vec<(ComparisonType, Term)>,
}
impl Parser for Comparison {
    fn parse(input: &mut ParseStream) -> Result<Comparison> {
        fn do_parse(input: &mut ParseStream) -> Result<Comparison> {
            let mut type_variant = ComparisonType::None;
            let mut tokens_list: std::collections::LinkedList<crate::tokenizer::Token> =
                std::collections::LinkedList::new();
            let main_term = input.parse::<Term>()?;
            let mut terms = std::collections::LinkedList::new();
            while input.peek::<Less>()
                || input.peek::<LessEqual>()
                || input.peek::<Greater>()
                || input.peek::<GreaterEqual>()
            {
                let mut current_type_variant = ComparisonType::None;
                if input.peek::<Less>() {
                    tokens_list.push_back(input.parse::<Less>()?.token.clone());
                    current_type_variant = ComparisonType::Less;
                } else if input.peek::<LessEqual>() {
                    tokens_list.push_back(input.parse::<LessEqual>()?.token.clone());
                    current_type_variant = ComparisonType::LessEqual;
                } else if input.peek::<Greater>() {
                    tokens_list.push_back(input.parse::<Greater>()?.token.clone());
                    current_type_variant = ComparisonType::Greater;
                } else {
                    tokens_list.push_back(input.parse::<GreaterEqual>()?.token.clone());
                    current_type_variant = ComparisonType::GreaterEqual;
                }
                let nt = input.parse::<Term>()?;
                terms.push_back((current_type_variant, nt));
            }
            let terms: Vec<_> = terms.into_iter().collect();
            std::result::Result::Ok(Comparison {
                token_type: type_variant,
                main_term,
                terms,
            })
        }
        do_parse(input)
    }
    fn peek(input: &ParseStream) -> bool {
        input.peek::<Term>()
    }
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

pub enum EqualityType {
    EqualEqual,
    BangEqual,
    None,
}
pub struct Equality {
    pub token_type: EqualityType,
    pub main_comparison: Comparison,
    pub comparisons: Vec<(EqualityType, Comparison)>,
}
impl Parser for Equality {
    fn parse(input: &mut ParseStream) -> Result<Equality> {
        fn do_parse(input: &mut ParseStream) -> Result<Equality> {
            let mut type_variant = EqualityType::None;
            let mut tokens_list: std::collections::LinkedList<crate::tokenizer::Token> =
                std::collections::LinkedList::new();
            let main_comparison = input.parse::<Comparison>()?;
            let mut comparisons = std::collections::LinkedList::new();
            while input.peek::<EqualEqual>() || input.peek::<BangEqual>() {
                let mut current_type_variant = EqualityType::None;
                if input.peek::<EqualEqual>() {
                    tokens_list.push_back(input.parse::<EqualEqual>()?.token.clone());
                    current_type_variant = EqualityType::EqualEqual;
                } else {
                    tokens_list.push_back(input.parse::<BangEqual>()?.token.clone());
                    current_type_variant = EqualityType::BangEqual;
                }
                let nt = input.parse::<Comparison>()?;
                comparisons.push_back((current_type_variant, nt));
            }
            let comparisons: Vec<_> = comparisons.into_iter().collect();
            std::result::Result::Ok(Equality {
                token_type: type_variant,
                main_comparison,
                comparisons,
            })
        }
        do_parse(input)
    }
    fn peek(input: &ParseStream) -> bool {
        input.peek::<Comparison>()
    }
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

pub type Expression = Box<Equality>;
impl Parser for Expression {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let equality = input.parse::<Equality>()?;
        Ok(Box::new(equality))
    }

    fn peek(input: &ParseStream) -> bool {
        input.peek::<Equality>()
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
