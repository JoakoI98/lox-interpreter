use std::{cell::RefCell, rc::Rc};

use crate::evaluation::{
    evaluator::EvaluableIdentifier,
    functions_resolver::FunctionsResolver,
    run::{RunScopeRef, RunScopes},
    RuntimeError, RuntimeValue,
};

pub struct RunState {
    scopes: RefCell<RunScopeRef>,
    functions_resolver: RefCell<FunctionsResolver>,
}

impl RunState {
    pub fn new(functions_resolver: FunctionsResolver, scopes: RunScopes) -> Self {
        Self {
            scopes: RefCell::new(Rc::new(RefCell::new(scopes))),
            functions_resolver: RefCell::new(functions_resolver),
        }
    }

    pub fn void() -> Self {
        Self {
            scopes: RefCell::new(Rc::new(RefCell::new(RunScopes::new(None)))),
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
            .borrow()
            .borrow_mut()
            .declare_variable(identifier, value, depth);
    }

    #[inline]
    pub fn set_variable(&self, identifier: String, value: RuntimeValue, depth: Option<usize>) {
        self.scopes
            .borrow()
            .borrow_mut()
            .set_variable(identifier, value, depth);
    }

    #[inline]
    pub fn enter_scope(&self) -> Result<(), RuntimeError> {
        let enclosing = self.scopes.borrow().clone();
        let new_scope = RunScopes::new(Some(enclosing));
        self.scopes.replace(Rc::new(RefCell::new(new_scope)));
        Ok(())
    }

    #[inline]
    pub fn exit_scope(&self) -> Result<(), RuntimeError> {
        let enclosing = self
            .scopes
            .borrow()
            .borrow()
            .get_enclosing()
            .ok_or(RuntimeError::OutOfScope)?;
        self.scopes.replace(enclosing);
        Ok(())
    }

    pub fn evaluate_variable(
        &self,
        identifier: &EvaluableIdentifier,
    ) -> Result<RuntimeValue, RuntimeError> {
        self.scopes
            .borrow()
            .borrow()
            .evaluate_variable(identifier, None)
    }

    pub fn call_function(
        &self,
        index: usize,
        arguments: Vec<RuntimeValue>,
        function_scope: Option<RunScopeRef>,
    ) -> Result<RuntimeValue, RuntimeError> {
        let resolver = self.functions_resolver.borrow();
        let pointer = resolver
            .resolve(index)
            .ok_or(RuntimeError::FunctionNotFound)?;
        // println!("current_scope:\n{:?}", self.get_current_scope());
        // println!("function_scope:\n{:?}", function_scope);

        let restore = self.replace_scopes(function_scope.unwrap_or(self.get_current_scope()));

        let result: Result<RuntimeValue, RuntimeError> = pointer.call(arguments, self);
        restore();
        result
    }

    pub fn replace_scopes(&self, scopes: RunScopeRef) -> impl FnOnce() + use<'_> {
        let current = self.scopes.replace(scopes);
        let restore = || {
            self.scopes.replace(current);
        };
        return restore;
    }

    pub fn get_current_scope(&self) -> RunScopeRef {
        self.scopes.borrow().clone()
    }
}
