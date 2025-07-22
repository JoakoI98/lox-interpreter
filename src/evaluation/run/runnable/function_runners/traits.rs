use crate::evaluation::{evaluator::Evaluable, RunState, RuntimeError, RuntimeValue};

pub trait Callable: Evaluable {
    fn arity(&self, state: &RunState) -> Result<usize, RuntimeError>;

    fn call(
        &self,
        arguments: Vec<RuntimeValue>,
        this_pointer: Option<usize>,
        state: &RunState,
    ) -> Result<RuntimeValue, RuntimeError>;
}
