use thiserror::Error;

use crate::evaluation::{
    evaluator::Evaluable,
    run::{Callable, Runnable},
    RunState, RuntimeError, RuntimeValue,
};

pub const INIT_FUNCTION_NAME: &str = "init";

#[derive(Debug, Error)]
pub enum FunctionEvaluationError {
    #[error("Can only call functions and classes.\n[line {0}]")]
    UnCallableFunction(usize),
}

#[derive(Debug)]
pub struct FunctionEvaluator {
    pub callable: Box<dyn Evaluable>,
    pub arguments: Vec<Box<dyn Evaluable>>,
    line: usize,
}

impl FunctionEvaluator {
    pub fn new(
        callable: Box<dyn Evaluable>,
        arguments: Vec<Box<dyn Evaluable>>,
        line: usize,
    ) -> Self {
        Self {
            callable,
            arguments,
            line,
        }
    }
}

impl Evaluable for FunctionEvaluator {
    fn eval(&self, state: &RunState) -> Result<RuntimeValue, RuntimeError> {
        let callable = self.callable.eval(state)?;
        let (index, scope, this_pointer) = match callable {
            RuntimeValue::Callable(c) => (c.get_pointer(), c.get_scope(), c.get_this_pointer()),
            _ => return Err(FunctionEvaluationError::UnCallableFunction(self.line).into()),
        };
        let arguments = self
            .arguments
            .iter()
            .map(|arg| arg.eval(state))
            .collect::<Result<Vec<RuntimeValue>, RuntimeError>>()?;
        state.call_function(index, arguments, scope, this_pointer)
    }
}

#[derive(Debug)]
pub struct FunctionCallable {
    function_block: Box<dyn Runnable>,
    parameters: Vec<String>,
    name: String,
}

impl FunctionCallable {
    pub fn new(function_block: Box<dyn Runnable>, parameters: Vec<String>, name: String) -> Self {
        Self {
            function_block,
            parameters,
            name,
        }
    }
}

impl Evaluable for FunctionCallable {
    fn eval(&self, state: &RunState) -> Result<RuntimeValue, RuntimeError> {
        let ret = self.function_block.run(state)?;
        Ok(ret.unwrap_or(RuntimeValue::Nil))
    }
}

impl FunctionCallable {
    fn define_arguments(
        &self,
        arguments: Vec<RuntimeValue>,
        state: &RunState,
    ) -> Result<(), RuntimeError> {
        for (i, argument) in arguments.iter().enumerate() {
            state.declare_variable(self.parameters[i].clone(), Some(argument.clone()), Some(0));
        }
        Ok(())
    }
}

impl Callable for FunctionCallable {
    fn call(
        &self,
        arguments: Vec<RuntimeValue>,
        this_pointer: Option<usize>,
        state: &RunState,
    ) -> Result<RuntimeValue, RuntimeError> {
        if arguments.len() != self.parameters.len() {
            return Err(RuntimeError::ArityMismatch);
        }

        state.enter_scope()?;
        if let Some(this_pointer) = this_pointer {
            state.set_this(this_pointer);
        }
        self.define_arguments(arguments, state)?;
        let result = self.eval(state)?;
        state.exit_scope()?;
        if self.name == INIT_FUNCTION_NAME && this_pointer.is_some() {
            let class = state.get_class_name(this_pointer.unwrap())?;
            return Ok(RuntimeValue::ClassInstance(this_pointer.unwrap(), class));
        }
        Ok(result)
    }
}
