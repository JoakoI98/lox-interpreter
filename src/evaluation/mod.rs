use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Not, Sub},
};

use thiserror::Error;

use crate::syntax_analysis::{
    Comparison, ComparisonType, Equality, EqualityType, Expression, Factor, FactorType,
    PrimaryExpression, PrimaryExpressionType, Term, TermType, UnaryExpression, UnaryExpressionSelf,
    UnaryExpressionSelfType, UnaryExpressionType,
};

#[derive(Debug, PartialEq)]
pub enum RuntimeValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl Display for RuntimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeValue::Number(f64) => write!(f, "{}", f64),
            RuntimeValue::String(s) => write!(f, "{}", s),
            RuntimeValue::Boolean(b) => write!(f, "{}", b),
            RuntimeValue::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Operand must be a {0}")]
    UnaryOperandError(String),
    #[error("Operand must be {0}")]
    BinaryOperandError(String),
    #[error("Unexpected runtime error")]
    UnexpectedRuntimeError,
}

type Result<T> = std::result::Result<T, RuntimeError>;

impl Not for RuntimeValue {
    type Output = Result<RuntimeValue>;
    fn not(self) -> Self::Output {
        match self {
            RuntimeValue::Boolean(b) => Ok(RuntimeValue::Boolean(!b)),
            RuntimeValue::Number(f) => Ok(RuntimeValue::Boolean(f == 0.0)),
            RuntimeValue::String(str) => Ok(RuntimeValue::Boolean(str.is_empty())),
            RuntimeValue::Nil => Ok(RuntimeValue::Boolean(true)),
        }
    }
}

impl Neg for RuntimeValue {
    type Output = Result<RuntimeValue>;
    fn neg(self) -> Self::Output {
        match self {
            RuntimeValue::Number(f) => Ok(RuntimeValue::Number(-f)),
            _ => Err(RuntimeError::UnaryOperandError("number".to_string())),
        }
    }
}

impl Mul for RuntimeValue {
    type Output = Result<RuntimeValue>;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RuntimeValue::Number(f1), RuntimeValue::Number(f2)) => {
                Ok(RuntimeValue::Number(f1 * f2))
            }
            _ => Err(RuntimeError::BinaryOperandError("numbers".to_string())),
        }
    }
}

impl Div for RuntimeValue {
    type Output = Result<RuntimeValue>;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RuntimeValue::Number(f1), RuntimeValue::Number(f2)) => {
                Ok(RuntimeValue::Number(f1 / f2))
            }
            _ => Err(RuntimeError::BinaryOperandError("numbers".to_string())),
        }
    }
}

impl Add for RuntimeValue {
    type Output = Result<RuntimeValue>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RuntimeValue::Number(f1), RuntimeValue::Number(f2)) => {
                Ok(RuntimeValue::Number(f1 + f2))
            }
            (RuntimeValue::String(s1), RuntimeValue::String(s2)) => {
                Ok(RuntimeValue::String(s1 + s2.as_str()))
            }
            _ => Err(RuntimeError::BinaryOperandError(
                "numbers or strings".to_string(),
            )),
        }
    }
}

impl Sub for RuntimeValue {
    type Output = Result<RuntimeValue>;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RuntimeValue::Number(f1), RuntimeValue::Number(f2)) => {
                Ok(RuntimeValue::Number(f1 - f2))
            }
            _ => Err(RuntimeError::BinaryOperandError(
                "numbers or strings".to_string(),
            )),
        }
    }
}

impl RuntimeValue {
    pub fn lt(&self, rhs: &RuntimeValue) -> Result<RuntimeValue> {
        match (&self, &rhs) {
            (RuntimeValue::Number(f1), RuntimeValue::Number(f2)) => {
                Ok(RuntimeValue::Boolean(f1 < f2))
            }
            _ => Err(RuntimeError::BinaryOperandError("numbers".to_string())),
        }
    }

    pub fn le(&self, rhs: &RuntimeValue) -> Result<RuntimeValue> {
        match (&self, &rhs) {
            (RuntimeValue::Number(f1), RuntimeValue::Number(f2)) => {
                Ok(RuntimeValue::Boolean(f1 <= f2))
            }
            _ => Err(RuntimeError::BinaryOperandError("numbers".to_string())),
        }
    }

    pub fn gt(&self, rhs: &RuntimeValue) -> Result<RuntimeValue> {
        match (&self, &rhs) {
            (RuntimeValue::Number(f1), RuntimeValue::Number(f2)) => {
                Ok(RuntimeValue::Boolean(f1 > f2))
            }
            _ => Err(RuntimeError::BinaryOperandError("numbers".to_string())),
        }
    }

    pub fn ge(&self, rhs: &RuntimeValue) -> Result<RuntimeValue> {
        match (&self, &rhs) {
            (RuntimeValue::Number(f1), RuntimeValue::Number(f2)) => {
                Ok(RuntimeValue::Boolean(f1 >= f2))
            }
            _ => Err(RuntimeError::BinaryOperandError("numbers".to_string())),
        }
    }

    pub fn eq(&self, rhs: &RuntimeValue) -> Result<RuntimeValue> {
        Ok(RuntimeValue::Boolean(*self == *rhs))
    }

    pub fn ne(&self, rhs: &RuntimeValue) -> Result<RuntimeValue> {
        Ok(RuntimeValue::Boolean(*self != *rhs))
    }
}

impl PrimaryExpression {
    pub fn eval(&self) -> Result<RuntimeValue> {
        let token = self
            .token_list
            .first()
            .ok_or(RuntimeError::UnexpectedRuntimeError)?;
        match &self.token_type {
            PrimaryExpressionType::Number => {
                Ok(RuntimeValue::Number(token.token_value.number().unwrap()))
            }
            PrimaryExpressionType::String => {
                Ok(RuntimeValue::String(token.token_value.string().unwrap()))
            }
            PrimaryExpressionType::False => Ok(RuntimeValue::Boolean(false)),
            PrimaryExpressionType::True => Ok(RuntimeValue::Boolean(true)),
            PrimaryExpressionType::Nil => Ok(RuntimeValue::Nil),
            PrimaryExpressionType::Expression(_) => Ok(RuntimeValue::Nil),
            _ => Err(RuntimeError::UnexpectedRuntimeError),
        }
    }
}

impl UnaryExpression {
    pub fn eval(&self) -> Result<RuntimeValue> {
        match &self.token_type {
            UnaryExpressionType::UnaryExpressionSelf(expr) => expr.eval(),
            UnaryExpressionType::PrimaryExpression(expr) => expr.eval(),
            _ => Err(RuntimeError::UnexpectedRuntimeError),
        }
    }
}

impl UnaryExpressionSelf {
    pub fn eval(&self) -> Result<RuntimeValue> {
        let inner = self.expr.eval()?;
        match &self.token_type {
            UnaryExpressionSelfType::Bang => !inner,
            UnaryExpressionSelfType::Minus => -inner,
            _ => Err(RuntimeError::UnexpectedRuntimeError),
        }
    }
}

impl Factor {
    pub fn eval(&self) -> Result<RuntimeValue> {
        let mut acc = self.main_unary.eval()?;
        for (op, factor) in self.unaries.iter() {
            let curr = match op {
                FactorType::Star => acc * factor.eval()?,
                FactorType::Slash => acc / factor.eval()?,
                _ => Err(RuntimeError::UnexpectedRuntimeError),
            }?;
            acc = curr;
        }
        return Ok(acc);
    }
}

impl Term {
    pub fn eval(&self) -> Result<RuntimeValue> {
        let mut acc = self.main_factor.eval()?;
        for (op, factor) in self.factors.iter() {
            let curr = match op {
                TermType::Plus => acc + factor.eval()?,
                TermType::Minus => acc - factor.eval()?,
                _ => Err(RuntimeError::UnexpectedRuntimeError),
            }?;
            acc = curr;
        }
        return Ok(acc);
    }
}

impl Comparison {
    pub fn eval(&self) -> Result<RuntimeValue> {
        let mut acc = self.main_term.eval()?;
        for (op, term) in self.terms.iter() {
            let curr = match op {
                ComparisonType::Less => acc.lt(&term.eval()?),
                ComparisonType::LessEqual => acc.le(&term.eval()?),
                ComparisonType::Greater => acc.gt(&term.eval()?),
                ComparisonType::GreaterEqual => acc.ge(&term.eval()?),
                _ => Err(RuntimeError::UnexpectedRuntimeError),
            }?;
            acc = curr;
        }
        return Ok(acc);
    }
}

impl Equality {
    pub fn eval(&self) -> Result<RuntimeValue> {
        let mut acc = self.main_comparison.eval()?;
        for (op, term) in self.comparisons.iter() {
            let curr = match op {
                EqualityType::EqualEqual => acc.eq(&term.eval()?),
                EqualityType::BangEqual => acc.ne(&term.eval()?),
                _ => Err(RuntimeError::UnexpectedRuntimeError),
            }?;
            acc = curr;
        }
        return Ok(acc);
    }
}
