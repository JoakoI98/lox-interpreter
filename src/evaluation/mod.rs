pub mod evaluator;
pub mod resolver;
pub mod run;
pub mod runtime_value;

use std::cell::RefCell;

pub use evaluator::AssignmentEvaluatorBuilder;
pub use run::{Program, RunState};
pub use runtime_value::{RuntimeError, RuntimeValue};

// BuilderContext holds shared state for building evaluators and runnables
#[derive(Debug, Default)]
pub struct BuilderContext {
    pub resolver: RefCell<resolver::Resolver>,
}

impl BuilderContext {
    pub fn new() -> Result<Self, RuntimeError> {
        Ok(Self {
            resolver: RefCell::new(resolver::Resolver::new()?),
        })
    }
}
