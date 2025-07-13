use std::collections::HashMap;

use crate::evaluation::{evaluator::EvaluableIdentifier, RuntimeError, RuntimeValue};

#[derive(Default, Debug)]
pub struct RunState {
    global_variables: HashMap<String, RuntimeValue>,
}

impl RunState {
    #[inline]
    pub fn declare_global_variable(&mut self, identifier: String, value: Option<RuntimeValue>) {
        self.global_variables
            .insert(identifier, value.unwrap_or(RuntimeValue::Nil));
    }

    pub fn evaluate_global_variable(
        &self,
        identifier: &EvaluableIdentifier,
    ) -> Result<RuntimeValue, RuntimeError> {
        let value = self.global_variables.get(identifier.identifier()).ok_or(
            RuntimeError::UndefinedVariable(identifier.identifier().to_string(), identifier.line()),
        )?;
        Ok(value.clone())
    }
}
