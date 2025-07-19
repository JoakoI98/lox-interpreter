use super::core::{Evaluable, EvaluableIdentifier};
use crate::evaluation::{run::RunState, RuntimeError, RuntimeValue};

#[derive(Debug)]
pub struct AssignmentEvaluator {
    identifier: EvaluableIdentifier,
    value: Box<dyn Evaluable>,
}

impl AssignmentEvaluator {
    pub fn new(identifier: EvaluableIdentifier, value: Box<dyn Evaluable>) -> Self {
        Self { identifier, value }
    }
}

impl Evaluable for AssignmentEvaluator {
    fn eval(&self, run_state: &mut RunState) -> Result<RuntimeValue, RuntimeError> {
        let value = self.value.eval(run_state)?;
        run_state.set_variable(
            self.identifier.identifier().to_string(),
            value.clone(),
            self.identifier.depth(),
        );
        Ok(value)
    }
}
