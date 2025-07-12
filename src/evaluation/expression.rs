use crate::syntax_analysis::{
    Comparison, ComparisonType, Equality, EqualityType, Factor, FactorType, PrimaryExpression,
    PrimaryExpressionType, Term, TermType, UnaryExpression, UnaryExpressionSelf,
    UnaryExpressionSelfType, UnaryExpressionType,
};

pub use super::runtime_value::{Result, RuntimeError, RuntimeValue};

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
            PrimaryExpressionType::Expression(e) => e.eval(),
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
