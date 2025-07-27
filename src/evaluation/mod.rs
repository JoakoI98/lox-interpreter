mod evaluator;
mod functions_resolver;
mod resolver;
mod run;
mod runtime_value;

use std::{cell::RefCell, collections::HashMap};

pub use evaluator::AssignmentEvaluatorBuilder;
pub use resolver::ResolverError;
pub use run::{Program, RunState};
pub use runtime_value::{RuntimeError, RuntimeValue};

// BuilderContext holds shared state for building evaluators and runnables
#[derive(Debug, Default)]
pub struct BuilderContext {
    resolver: RefCell<resolver::Resolver>,
    functions_resolver: RefCell<functions_resolver::FunctionsResolver>,
    class_definitions: RefCell<HashMap<String, usize>>,
}

impl BuilderContext {
    fn new() -> Result<Self, RuntimeError> {
        Ok(Self {
            resolver: RefCell::new(resolver::Resolver::new()?),
            functions_resolver: RefCell::new(functions_resolver::FunctionsResolver::new()?),
            class_definitions: RefCell::new(HashMap::new()),
        })
    }

    fn get_class_definition(&self, identifier: &str) -> Option<usize> {
        self.class_definitions.borrow().get(identifier).cloned()
    }

    fn set_class_definition(&self, identifier: &str, definition: usize) {
        self.class_definitions
            .borrow_mut()
            .insert(identifier.to_string(), definition);
    }
}
