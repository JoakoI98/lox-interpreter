use std::{cell::RefCell, collections::HashMap};

use crate::evaluation::{
    evaluator::EvaluableIdentifier, functions_resolver::FunctionsResolver, run::Callable,
    RuntimeError, RuntimeValue,
};

#[derive(Debug)]
pub struct RunScopes {
    scopes: Vec<HashMap<String, RuntimeValue>>,
}

impl RunScopes {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    #[inline]
    pub fn declare_variable(
        &mut self,
        identifier: String,
        value: Option<RuntimeValue>,
        depth: Option<usize>,
    ) {
        let i = self.scopes.len() - depth.unwrap_or(0) - 1;
        self.scopes[i].insert(identifier, value.unwrap_or(RuntimeValue::Nil));
    }

    #[inline]
    pub fn set_variable(&mut self, identifier: String, value: RuntimeValue, depth: Option<usize>) {
        for scope in self.scopes.iter_mut().rev().skip(depth.unwrap_or(0)) {
            if scope.contains_key(&identifier) {
                scope.insert(identifier, value);
                return;
            }
        }
    }

    #[inline]
    pub fn enter_scope(&mut self) -> Result<(), RuntimeError> {
        self.scopes.push(HashMap::new());
        Ok(())
    }

    #[inline]
    pub fn exit_scope(&mut self) -> Result<(), RuntimeError> {
        self.scopes.pop();
        Ok(())
    }

    pub fn evaluate_variable(
        &self,
        identifier: &EvaluableIdentifier,
    ) -> Result<RuntimeValue, RuntimeError> {
        for scope in self
            .scopes
            .iter()
            .rev()
            .skip(identifier.depth().unwrap_or(0))
        {
            if scope.contains_key(identifier.identifier()) {
                let value = scope.get(identifier.identifier()).unwrap();

                return Ok(value.clone());
            }
        }
        Err(RuntimeError::UndefinedVariable(
            identifier.identifier().to_string(),
            identifier.line(),
        ))
    }
}

pub struct RunState {
    scopes: RefCell<RunScopes>,
    functions_resolver: RefCell<FunctionsResolver>,
}

impl RunState {
    pub fn new(functions_resolver: FunctionsResolver, scopes: RunScopes) -> Self {
        Self {
            scopes: RefCell::new(scopes),
            functions_resolver: RefCell::new(functions_resolver),
        }
    }

    pub fn void() -> Self {
        Self {
            scopes: RefCell::new(RunScopes::new()),
            functions_resolver: RefCell::new(FunctionsResolver::new().unwrap()),
        }
    }

    #[inline]
    pub fn declare_variable(
        &self,
        identifier: String,
        value: Option<RuntimeValue>,
        depth: Option<usize>,
    ) {
        self.scopes
            .borrow_mut()
            .declare_variable(identifier, value, depth);
    }

    #[inline]
    pub fn set_variable(&self, identifier: String, value: RuntimeValue, depth: Option<usize>) {
        self.scopes
            .borrow_mut()
            .set_variable(identifier, value, depth);
    }

    #[inline]
    pub fn enter_scope(&self) -> Result<(), RuntimeError> {
        self.scopes.borrow_mut().enter_scope()
    }

    #[inline]
    pub fn exit_scope(&self) -> Result<(), RuntimeError> {
        self.scopes.borrow_mut().exit_scope()
    }

    pub fn evaluate_variable(
        &self,
        identifier: &EvaluableIdentifier,
    ) -> Result<RuntimeValue, RuntimeError> {
        self.scopes.borrow().evaluate_variable(identifier)
    }

    pub fn call_function(
        &self,
        index: usize,
        arguments: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RuntimeError> {
        let resolver = self.functions_resolver.borrow();
        let pointer = resolver
            .resolve(index)
            .ok_or(RuntimeError::FunctionNotFound)?;
        pointer.call(arguments, self)
    }
}
