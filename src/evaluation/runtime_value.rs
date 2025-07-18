use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Not, Sub},
};

use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
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
    #[error("AST invalid structure")]
    ASTInvalidStructure,
    #[error("Undefined variable '{0}'.\nLine: {1}")]
    UndefinedVariable(String, usize),
    #[error("Resolver error: {0}")]
    ResolverError(#[from] super::resolver::ResolverError),
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

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
