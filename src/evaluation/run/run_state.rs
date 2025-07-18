use std::collections::HashMap;

use crate::evaluation::{evaluator::EvaluableIdentifier, RuntimeError, RuntimeValue};

#[derive(Debug)]
pub struct RunState {
    scopes: Vec<HashMap<String, RuntimeValue>>,
}

impl RunState {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    #[inline]
    pub fn declare_variable(
        &mut self,
        identifier: String,
        value: Option<RuntimeValue>,
        depth: Option<usize>,
    ) {
        let i = self.scopes.len() - depth.unwrap_or(0) - 1;
        self.scopes[i].insert(identifier, value.unwrap_or(RuntimeValue::Nil));
    }

    pub fn set_variable(&mut self, identifier: String, value: RuntimeValue, depth: Option<usize>) {
        let i = self.scopes.len() - depth.unwrap_or(0) - 1;
        self.scopes[i].insert(identifier, value);
    }

    pub fn evaluate_variable(
        &self,
        identifier: &EvaluableIdentifier,
    ) -> Result<RuntimeValue, RuntimeError> {
        let i = self.scopes.len() - identifier.depth().unwrap_or(0) - 1;
        let value =
            self.scopes[i]
                .get(identifier.identifier())
                .ok_or(RuntimeError::UndefinedVariable(
                    identifier.identifier().to_string(),
                    identifier.line(),
                ))?;
        Ok(value.clone())
    }
}
