use crate::evaluation::{
    evaluator::Evaluable, runtime_value::ThisInstance, RunState, RuntimeError, RuntimeValue,
};

pub trait Callable: Evaluable {
    fn arity(&self, state: &RunState) -> Result<usize, RuntimeError>;

    fn call(
        &self,
        arguments: Vec<RuntimeValue>,
        this_pointer: Option<ThisInstance>,
        state: &RunState,
    ) -> Result<RuntimeValue, RuntimeError>;
}
