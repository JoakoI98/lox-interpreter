use std::collections::{HashMap, TryReserveError};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResolverError {
    #[error("Identifier not found")]
    IdentifierNotFound,

    #[error("Identifier not found")]
    UndeclaredIdentifier,

    #[error("No available scopes")]
    NoAvailableScopes,

    #[error("Not enough space to allocate new scope")]
    NotEnoughSpace(#[from] TryReserveError),
}

#[derive(Debug, Default)]
pub struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
}

const INITIAL_SCOPE_CAPACITY: usize = 50;
const SCOPE_GROWTH_FACTOR: usize = 2;

impl Resolver {
    pub fn new() -> Result<Self, ResolverError> {
        let mut scopes = Vec::new();
        scopes.try_reserve_exact(INITIAL_SCOPE_CAPACITY)?;
        scopes.push(HashMap::new());
        Ok(Self { scopes })
    }

    pub fn enter_scope(&mut self) -> Result<(), ResolverError> {
        if self.scopes.len() == self.scopes.capacity() {
            self.scopes
                .try_reserve_exact(self.scopes.capacity() * SCOPE_GROWTH_FACTOR)?;
        }
        self.scopes.push(HashMap::new());
        Ok(())
    }

    pub fn exit_scope(&mut self) -> Result<(), ResolverError> {
        self.scopes.pop();
        Ok(())
    }

    pub fn declare(&mut self, identifier: &str) -> Result<(), ResolverError> {
        let last_scope = self
            .scopes
            .last_mut()
            .ok_or(ResolverError::NoAvailableScopes)?;

        last_scope.insert(identifier.to_string(), false);

        Ok(())
    }

    pub fn define(&mut self, identifier: &str) -> Result<(), ResolverError> {
        let last_scope = self
            .scopes
            .last_mut()
            .ok_or(ResolverError::NoAvailableScopes)?;

        let key = last_scope
            .get_mut(identifier)
            .ok_or(ResolverError::UndeclaredIdentifier)?;

        *key = true;

        Ok(())
    }

    pub fn resolve(&self, identifier: &str) -> usize {
        for (index, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(identifier) {
                return index;
            }
        }
        self.scopes.len()
    }
}
