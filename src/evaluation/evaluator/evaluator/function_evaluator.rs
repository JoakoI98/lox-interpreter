use thiserror::Error;

use crate::evaluation::{evaluator::Evaluable, RunState, RuntimeError, RuntimeValue};

#[derive(Debug, Error)]
pub enum FunctionEvaluationError {
    #[error("Function is not callable")]
    UnCallableFunction(RuntimeValue),
}

#[derive(Debug)]
pub struct FunctionEvaluator {
    pub callable: Box<dyn Evaluable>,
    pub arguments: Vec<Box<dyn Evaluable>>,
}

impl FunctionEvaluator {
    pub fn new(callable: Box<dyn Evaluable>, arguments: Vec<Box<dyn Evaluable>>) -> Self {
        Self {
            callable,
            arguments,
        }
    }
}

impl Evaluable for FunctionEvaluator {
    fn eval(&self, state: &RunState) -> Result<RuntimeValue, RuntimeError> {
        let callable = self.callable.eval(state)?;
        let index = match callable {
            RuntimeValue::Callable(index) => index,
            _ => return Err(FunctionEvaluationError::UnCallableFunction(callable).into()),
        };
        let arguments = self
            .arguments
            .iter()
            .map(|arg| arg.eval(state))
            .collect::<Result<Vec<RuntimeValue>, RuntimeError>>()?;
        state.call_function(index, arguments)
    }
}
