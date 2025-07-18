use crate::{
    evaluation::{resolver::Resolver, run::RunState},
    tokenizer::{Token, TokenValue},
};

use super::super::{RuntimeError, RuntimeValue};

pub trait Evaluable: std::fmt::Debug {
    fn eval(&self, state: &mut RunState) -> Result<RuntimeValue, RuntimeError>;
}

#[derive(Debug)]
pub struct EvaluableIdentifier {
    identifier: String,
    line: usize,
    depth: Option<usize>,
}

impl EvaluableIdentifier {
    pub(super) fn from_raw_token(token: &Token, resolver: &Resolver) -> Result<Self, RuntimeError> {
        let identifier_string = match &token.token_value {
            TokenValue::Identifier(_) => token.lexeme.clone(),
            _ => return Err(RuntimeError::ASTInvalidStructure),
        };
        Ok(Self {
            depth: resolver.resolve(&identifier_string)?,
            identifier: identifier_string,
            line: token.line,
        })
    }

    #[inline]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    #[inline]
    pub fn line(&self) -> usize {
        self.line
    }

    #[inline]
    pub fn depth(&self) -> Option<usize> {
        self.depth
    }
}

#[derive(Debug)]
pub enum PrimaryEvaluator {
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(EvaluableIdentifier),
    Nil,
}

impl PrimaryEvaluator {
    pub(super) fn from_raw_token(token: &Token, resolver: &Resolver) -> Result<Self, RuntimeError> {
        Ok(PrimaryEvaluator::Identifier(
            EvaluableIdentifier::from_raw_token(token, resolver)?,
        ))
    }
}

impl Evaluable for PrimaryEvaluator {
    fn eval(&self, run_state: &mut RunState) -> Result<RuntimeValue, RuntimeError> {
        match self {
            PrimaryEvaluator::Number(value) => Ok(RuntimeValue::Number(value.clone())),
            PrimaryEvaluator::String(value) => Ok(RuntimeValue::String(value.clone())),
            PrimaryEvaluator::Boolean(value) => Ok(RuntimeValue::Boolean(value.clone())),
            PrimaryEvaluator::Nil => Ok(RuntimeValue::Nil),
            PrimaryEvaluator::Identifier(identifier) => run_state.evaluate_variable(identifier),
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
    fn eval(&self, run_state: &mut RunState) -> Result<RuntimeValue, RuntimeError> {
        let operand = self.operand.eval(run_state)?;
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
    fn eval(&self, run_state: &mut RunState) -> Result<RuntimeValue, RuntimeError> {
        let left = self.left.eval(run_state)?;
        let right = self.right.eval(run_state)?;
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

#[derive(Debug)]
pub struct AssignmentEvaluator {
    identifier: EvaluableIdentifier,
    value: Box<dyn Evaluable>,
}

impl AssignmentEvaluator {
    pub(super) fn new(identifier: EvaluableIdentifier, value: Box<dyn Evaluable>) -> Self {
        Self { identifier, value }
    }
}

impl Evaluable for AssignmentEvaluator {
    fn eval(&self, run_state: &mut RunState) -> Result<RuntimeValue, RuntimeError> {
        let value = self.value.eval(run_state)?;
        run_state.set_variable(
            self.identifier.identifier.clone(),
            value.clone(),
            self.identifier.depth,
        );
        Ok(value)
    }
}
