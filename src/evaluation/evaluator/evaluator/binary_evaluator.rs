use super::core::Evaluable;
use crate::evaluation::{run::RunState, RuntimeError, RuntimeValue};

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
    LogicalAnd,
    LogicalOr,
}

#[derive(Debug)]
pub struct BinaryEvaluator {
    operation: BinaryOperation,
    left: Box<dyn Evaluable>,
    right: Box<dyn Evaluable>,
}

impl BinaryEvaluator {
    pub fn new(
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
    fn eval(&self, run_state: &RunState) -> Result<RuntimeValue, RuntimeError> {
        match self.operation {
            // Short circuit evaluation
            BinaryOperation::LogicalAnd => {
                let left = self.left.eval(run_state)?;
                if !left.to_bool()? {
                    return Ok(RuntimeValue::Boolean(false));
                }
                let right = self.right.eval(run_state)?;
                if !right.to_bool()? {
                    return Ok(RuntimeValue::Boolean(false));
                }
                return Ok(right);
            }
            BinaryOperation::LogicalOr => {
                let left = self.left.eval(run_state)?;
                if left.to_bool()? {
                    return Ok(left);
                }
                let right = self.right.eval(run_state)?;
                if right.to_bool()? {
                    return Ok(right);
                }
                return Ok(RuntimeValue::Boolean(false));
            }
            _ => {}
        }
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
            BinaryOperation::LogicalAnd => {
                unreachable!()
            }
            BinaryOperation::LogicalOr => {
                unreachable!()
            }
        }
    }
}
