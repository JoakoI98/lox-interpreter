use std::collections::{HashMap, TryReserveError};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResolverError {
    #[error("[line {1}] Error at '{0}': Can't read local variable in its own initializer.")]
    LocalVariableInInitializer(String, usize),

    #[error("[line {1}] Error at '{0}': Already a variable with this name in this scope.")]
    AlreadyDeclaredIdentifier(String, usize),

    #[error("[line {0}] Error at 'return': Can't return from top-level code.")]
    ReturnOutsideFunction(usize),

    #[error("[line {0}] Error at 'this': Can't use 'this' outside of a class.")]
    ThisOutsideClass(usize),

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
    function_depth: usize,
    class_depth: usize,
}

const INITIAL_SCOPE_CAPACITY: usize = 50;
const SCOPE_GROWTH_FACTOR: usize = 2;

impl Resolver {
    pub fn new() -> Result<Self, ResolverError> {
        let mut scopes = Vec::new();
        scopes.try_reserve_exact(INITIAL_SCOPE_CAPACITY)?;
        scopes.push(HashMap::new());
        Ok(Self {
            scopes,
            function_depth: 0,
            class_depth: 0,
        })
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

    pub fn declare(&mut self, identifier: &str, line: usize) -> Result<(), ResolverError> {
        let in_global_scope = self.scopes.len() == 1;

        let last_scope = self
            .scopes
            .last_mut()
            .ok_or(ResolverError::NoAvailableScopes)?;

        if last_scope.contains_key(identifier) && !in_global_scope {
            return Err(ResolverError::AlreadyDeclaredIdentifier(
                identifier.to_string(),
                line,
            ));
        }

        last_scope.insert(identifier.to_string(), false);

        // Functions defines a scope only for its parameters
        // Check if the identifier is already declared in the parameters scope
        if self.function_depth > 0
            && self
                .scopes
                .get(self.scopes.len() - 2)
                .ok_or(ResolverError::NoAvailableScopes)?
                .contains_key(identifier)
        {
            return Err(ResolverError::AlreadyDeclaredIdentifier(
                identifier.to_string(),
                line,
            ));
        }

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

    pub fn resolve(&self, identifier: &str, line: usize) -> Result<Option<usize>, ResolverError> {
        for (index, scope) in self.scopes.iter().rev().enumerate() {
            if let Some(value) = scope.get(identifier) {
                let is_global_scope = index == self.scopes.len() - 1;
                if *value || is_global_scope {
                    return Ok(Some(index));
                }
                return Err(ResolverError::LocalVariableInInitializer(
                    identifier.to_string(),
                    line,
                ));
            }
        }
        Ok(None)
    }

    pub fn enter_function(&mut self) {
        self.function_depth += 1;
    }

    pub fn exit_function(&mut self) {
        self.function_depth -= 1;
    }

    pub fn function_depth(&self) -> usize {
        self.function_depth
    }

    pub fn enter_class(&mut self) {
        self.class_depth += 1;
    }

    pub fn exit_class(&mut self) {
        self.class_depth -= 1;
    }

    pub fn is_in_class(&self) -> bool {
        self.class_depth > 0
    }
}
