use std::time::{SystemTime, UNIX_EPOCH};

use thiserror::Error;

use crate::evaluation::{evaluator::Evaluable, RunState, RuntimeError, RuntimeValue};

pub trait Callable: Evaluable {
    fn arity(&self) -> usize;
    fn define_arguments(
        &self,
        arguments: Vec<RuntimeValue>,
        state: &RunState,
    ) -> Result<(), RuntimeError>;

    fn call(
        &self,
        arguments: Vec<RuntimeValue>,
        state: &RunState,
    ) -> Result<RuntimeValue, RuntimeError> {
        if arguments.len() != self.arity() {
            return Err(RuntimeError::ArityMismatch);
        }

        self.define_arguments(arguments, state)?;
        self.eval(state)
    }
}
