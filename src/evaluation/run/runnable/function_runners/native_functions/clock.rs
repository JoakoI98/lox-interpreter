use super::NativeFunctionError;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::evaluation::{
    evaluator::Evaluable, run::Callable, RunState, RuntimeError, RuntimeValue,
};

#[derive(Debug)]
pub struct ClockNativeFunction;

impl Callable for ClockNativeFunction {
    fn arity(&self) -> usize {
        0
    }

    fn define_arguments(
        &self,
        _arguments: Vec<RuntimeValue>,
        _state: &RunState,
    ) -> Result<(), RuntimeError> {
        Ok(())
    }
}

impl Evaluable for ClockNativeFunction {
    fn eval(&self, _: &RunState) -> Result<RuntimeValue, RuntimeError> {
        let now = SystemTime::now();
        let since_epoch = now
            .duration_since(UNIX_EPOCH)
            .map_err(|_| NativeFunctionError::SystemClockBeforeUnixEpoch)?
            .as_secs();
        Ok(RuntimeValue::Number(since_epoch as f64))
    }
}
