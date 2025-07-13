use super::{RuntimeError, RuntimeValue};

pub trait Evaluable: std::fmt::Debug {
    fn eval(&self) -> Result<RuntimeValue, RuntimeError>;
}

#[derive(Debug)]
pub enum PrimaryEvaluator {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl Evaluable for PrimaryEvaluator {
    fn eval(&self) -> Result<RuntimeValue, RuntimeError> {
        match self {
            PrimaryEvaluator::Number(value) => Ok(RuntimeValue::Number(value.clone())),
            PrimaryEvaluator::String(value) => Ok(RuntimeValue::String(value.clone())),
            PrimaryEvaluator::Boolean(value) => Ok(RuntimeValue::Boolean(value.clone())),
            PrimaryEvaluator::Nil => Ok(RuntimeValue::Nil),
        }
    }
}

#[derive(Debug)]
pub enum UnaryOperation {
    Negation,
    Not,
}

#[derive(Debug)]
pub struct UnaryEvaluator {
    operation: UnaryOperation,
    operand: Box<dyn Evaluable>,
}

impl UnaryEvaluator {
    pub(super) fn new(operation: UnaryOperation, operand: Box<dyn Evaluable>) -> Self {
        Self { operation, operand }
    }
}

impl Evaluable for UnaryEvaluator {
    fn eval(&self) -> Result<RuntimeValue, RuntimeError> {
        let operand = self.operand.eval()?;
        match self.operation {
            UnaryOperation::Negation => -operand,
            UnaryOperation::Not => !operand,
        }
    }
}

#[derive(Debug)]
pub enum BinaryOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Debug)]
pub struct BinaryEvaluator {
    operation: BinaryOperation,
    left: Box<dyn Evaluable>,
    right: Box<dyn Evaluable>,
}

impl BinaryEvaluator {
    pub(super) fn new(
        operation: BinaryOperation,
        left: Box<dyn Evaluable>,
        right: Box<dyn Evaluable>,
    ) -> Self {
        Self {
            operation,
            left,
            right,
        }
    }
}

impl Evaluable for BinaryEvaluator {
    fn eval(&self) -> Result<RuntimeValue, RuntimeError> {
        let left = self.left.eval()?;
        let right = self.right.eval()?;
        match self.operation {
            BinaryOperation::Addition => left + right,
            BinaryOperation::Subtraction => left - right,
            BinaryOperation::Multiplication => left * right,
            BinaryOperation::Division => left / right,
            BinaryOperation::Equal => left.eq(&right),
            BinaryOperation::NotEqual => left.ne(&right),
            BinaryOperation::GreaterThan => left.gt(&right),
            BinaryOperation::GreaterThanOrEqual => left.ge(&right),
            BinaryOperation::LessThan => left.lt(&right),
            BinaryOperation::LessThanOrEqual => left.le(&right),
        }
    }
}
