use std::collections::TryReserveError;

use thiserror::Error;

use crate::evaluation::run::Callable;

#[derive(Error, Debug)]
pub enum FunctionsResolverError {
    #[error("Not enough space to allocate new scope")]
    NotEnoughSpace(#[from] TryReserveError),
}

#[derive(Debug, Default)]
pub struct FunctionsResolver {
    functions: Vec<Box<dyn Callable>>,
}

const INITIAL_SCOPE_CAPACITY: usize = 50;
const SCOPE_GROWTH_FACTOR: usize = 2;

impl FunctionsResolver {
    pub fn new() -> Result<Self, FunctionsResolverError> {
        let mut functions = Vec::new();
        functions.try_reserve_exact(INITIAL_SCOPE_CAPACITY)?;
        Ok(Self { functions })
    }

    pub fn add_function(
        &mut self,
        function: Box<dyn Callable>,
    ) -> Result<usize, FunctionsResolverError> {
        if self.functions.len() == self.functions.capacity() {
            self.functions
                .try_reserve_exact(self.functions.capacity() * SCOPE_GROWTH_FACTOR)?;
        }

        let index = self.functions.len();
        self.functions.push(function);
        Ok(index)
    }

    pub fn resolve(&self, index: usize) -> Option<&Box<dyn Callable>> {
        self.functions.get(index)
    }
}
