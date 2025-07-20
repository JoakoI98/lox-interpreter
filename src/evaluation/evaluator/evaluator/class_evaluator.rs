use thiserror::Error;

use crate::evaluation::{evaluator::Evaluable, RunState, RuntimeError, RuntimeValue};

#[derive(Debug, Error)]
pub enum ClassAccessorError {
    #[error("Cannot access {0}")]
    Unaccessible(RuntimeValue),
}

#[derive(Debug)]
pub struct ClassAccessorEvaluator {
    to_access: Box<dyn Evaluable>,
    to_get: String,
}

impl ClassAccessorEvaluator {
    pub fn new(to_access: Box<dyn Evaluable>, to_get: String) -> Self {
        Self { to_access, to_get }
    }
}

impl Evaluable for ClassAccessorEvaluator {
    fn eval(&self, state: &RunState) -> Result<RuntimeValue, RuntimeError> {
        let to_access = self.to_access.eval(state)?;
        let class_instance_pointer = match to_access {
            RuntimeValue::ClassInstance(pointer, _) => pointer,
            _ => return Err(ClassAccessorError::Unaccessible(to_access).into()),
        };
        let class_instance = state
            .get_instance_value(class_instance_pointer, &self.to_get)?
            .unwrap_or(RuntimeValue::Nil);
        Ok(class_instance)
    }
}

#[derive(Debug)]
pub struct SetExpressionEvaluator {
    to_access: Box<dyn Evaluable>,
    to_set: String,
    value: Box<dyn Evaluable>,
}

impl SetExpressionEvaluator {
    pub fn new(to_access: Box<dyn Evaluable>, to_set: String, value: Box<dyn Evaluable>) -> Self {
        Self {
            to_access,
            to_set,
            value,
        }
    }
}

impl Evaluable for SetExpressionEvaluator {
    fn eval(&self, state: &RunState) -> Result<RuntimeValue, RuntimeError> {
        let to_access = self.to_access.eval(state)?;
        let class_instance_pointer = match to_access {
            RuntimeValue::ClassInstance(pointer, _) => pointer,
            _ => return Err(ClassAccessorError::Unaccessible(to_access).into()),
        };
        let to_set_value = self.value.eval(state)?;
        state.set_instance_value(class_instance_pointer, &self.to_set, to_set_value.clone())?;
        Ok(to_set_value)
    }
}
