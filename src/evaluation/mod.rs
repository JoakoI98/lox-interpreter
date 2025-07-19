mod evaluator;
mod functions_resolver;
mod resolver;
mod run;
mod runtime_value;

use std::cell::RefCell;

pub use evaluator::AssignmentEvaluatorBuilder;
pub use run::{Program, RunState};
pub use runtime_value::{RuntimeError, RuntimeValue};

// BuilderContext holds shared state for building evaluators and runnables
#[derive(Debug, Default)]
pub struct BuilderContext {
    resolver: RefCell<resolver::Resolver>,
    functions_resolver: RefCell<functions_resolver::FunctionsResolver>,
}

impl BuilderContext {
    fn new() -> Result<Self, RuntimeError> {
        Ok(Self {
            resolver: RefCell::new(resolver::Resolver::new()?),
            functions_resolver: RefCell::new(functions_resolver::FunctionsResolver::new()?),
        })
    }
}
