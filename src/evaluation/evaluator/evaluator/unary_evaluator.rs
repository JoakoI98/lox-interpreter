use super::core::Evaluable;
use crate::evaluation::{run::RunState, RuntimeError, RuntimeValue};

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
    pub fn new(operation: UnaryOperation, operand: Box<dyn Evaluable>) -> Self {
        Self { operation, operand }
    }
}

impl Evaluable for UnaryEvaluator {
    fn eval(&self, run_state: &RunState) -> Result<RuntimeValue, RuntimeError> {
        let operand = self.operand.eval(run_state)?;
        match self.operation {
            UnaryOperation::Negation => -operand,
            UnaryOperation::Not => !operand,
        }
    }
}
