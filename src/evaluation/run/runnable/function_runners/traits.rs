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

        state.enter_scope()?;
        self.define_arguments(arguments, state)?;
        let result = self.eval(state)?;
        state.exit_scope()?;
        Ok(result)
    }
}
