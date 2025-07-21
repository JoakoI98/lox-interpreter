use super::core::{Evaluable, EvaluableIdentifier};
use crate::{
    evaluation::{resolver::Resolver, run::RunState, RuntimeError, RuntimeValue},
    tokenizer::Token,
};

#[derive(Debug)]
pub enum PrimaryEvaluator {
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(EvaluableIdentifier),
    Nil,
    This,
}

impl PrimaryEvaluator {
    pub fn from_raw_token(token: &Token, resolver: &Resolver) -> Result<Self, RuntimeError> {
        Ok(PrimaryEvaluator::Identifier(
            EvaluableIdentifier::from_raw_token(token, resolver)?,
        ))
    }
}

impl Evaluable for PrimaryEvaluator {
    fn eval(&self, run_state: &RunState) -> Result<RuntimeValue, RuntimeError> {
        match self {
            PrimaryEvaluator::Number(value) => Ok(RuntimeValue::Number(value.clone())),
            PrimaryEvaluator::String(value) => Ok(RuntimeValue::String(value.clone())),
            PrimaryEvaluator::Boolean(value) => Ok(RuntimeValue::Boolean(value.clone())),
            PrimaryEvaluator::Nil => Ok(RuntimeValue::Nil),
            PrimaryEvaluator::Identifier(identifier) => run_state.evaluate_variable(identifier),
            PrimaryEvaluator::This => {
                let pointer = run_state.get_this().ok_or(RuntimeError::ThisNotInScope)?;
                let class_name = run_state.get_class_name(pointer)?;
                Ok(RuntimeValue::ClassInstance(pointer, class_name))
            }
        }
    }
}
