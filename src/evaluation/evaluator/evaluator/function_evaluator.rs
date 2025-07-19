use thiserror::Error;

use crate::evaluation::{
    evaluator::Evaluable,
    run::{Callable, Runnable},
    RunState, RuntimeError, RuntimeValue,
};

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
            RuntimeValue::Callable(index, _) => index,
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

#[derive(Debug)]
pub struct FunctionCallable {
    function_block: Box<dyn Runnable>,
    parameters: Vec<String>,
}

impl FunctionCallable {
    pub fn new(function_block: Box<dyn Runnable>, parameters: Vec<String>) -> Self {
        Self {
            function_block,
            parameters,
        }
    }
}

impl Evaluable for FunctionCallable {
    fn eval(&self, state: &RunState) -> Result<RuntimeValue, RuntimeError> {
        self.function_block.run(state)?;
        Ok(RuntimeValue::Nil)
    }
}

impl Callable for FunctionCallable {
    fn arity(&self) -> usize {
        self.parameters.len()
    }

    fn define_arguments(
        &self,
        arguments: Vec<RuntimeValue>,
        state: &RunState,
    ) -> Result<(), RuntimeError> {
        for (i, argument) in arguments.iter().enumerate() {
            state.set_variable(self.parameters[i].clone(), argument.clone(), Some(0));
        }
        Ok(())
    }
}
