use thiserror::Error;

mod clock;

#[derive(Debug, Error)]
pub enum NativeFunctionError {
    #[error("System clock before UNIX EPOCH")]
    SystemClockBeforeUnixEpoch,
}

use clock::ClockNativeFunction;

use crate::evaluation::run::Callable;

pub fn get_native_functions() -> Vec<(&'static str, Box<dyn Callable>)> {
    vec![("clock", Box::new(ClockNativeFunction))]
}
