use std::{cell::RefCell, rc::Rc};

use crate::evaluation::{
    evaluator::EvaluableIdentifier,
    functions_resolver::FunctionsResolver,
    run::{run_state::InstanceManager, RunScopeRef, RunScopes},
    RuntimeError, RuntimeValue,
};

pub struct RunState {
    scopes: RefCell<RunScopeRef>,
    functions_resolver: RefCell<FunctionsResolver>,
    instance_manager: RefCell<InstanceManager>,
}

impl RunState {
    pub fn new(functions_resolver: FunctionsResolver, scopes: RunScopes) -> Self {
        Self {
            scopes: RefCell::new(Rc::new(RefCell::new(scopes))),
            functions_resolver: RefCell::new(functions_resolver),
            instance_manager: RefCell::new(InstanceManager::new().unwrap()),
        }
    }

    pub fn void() -> Self {
        Self {
            scopes: RefCell::new(Rc::new(RefCell::new(RunScopes::new(None)))),
            functions_resolver: RefCell::new(FunctionsResolver::new().unwrap()),
            instance_manager: RefCell::new(InstanceManager::new().unwrap()),
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
        this_pointer: Option<usize>,
    ) -> Result<RuntimeValue, RuntimeError> {
        let resolver = self.functions_resolver.borrow();
        let pointer = resolver
            .resolve(index)
            .ok_or(RuntimeError::FunctionNotFound)?;

        let restore = self.replace_scopes(function_scope.unwrap_or(self.get_current_scope()));
        let result: Result<RuntimeValue, RuntimeError> =
            pointer.call(arguments, this_pointer, self);
        restore();
        result
    }

    pub fn function_arity(&self, index: usize) -> Result<usize, RuntimeError> {
        let resolver = self.functions_resolver.borrow();
        let pointer = resolver
            .resolve(index)
            .ok_or(RuntimeError::FunctionNotFound)?;

        pointer.arity(self)
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

    pub fn initialize_instance(
        &self,
        class_name: String,
        super_class: Option<usize>,
    ) -> Result<usize, RuntimeError> {
        self.instance_manager
            .borrow_mut()
            .initialize_instance(class_name, super_class)
    }

    pub fn get_instance_value(
        &self,
        index: usize,
        key: &str,
        max_depth: Option<usize>,
    ) -> Result<Option<RuntimeValue>, RuntimeError> {
        self.instance_manager
            .borrow()
            .get_instance_value(index, key, max_depth)
    }

    pub fn set_instance_value(
        &self,
        index: usize,
        key: &str,
        value: RuntimeValue,
    ) -> Result<(), RuntimeError> {
        self.instance_manager
            .borrow_mut()
            .set_instance_value(index, key, value)
    }

    pub fn map_this_pointer(&self, index: usize, this_pointer: usize) -> Result<(), RuntimeError> {
        self.instance_manager
            .borrow_mut()
            .map_this_pointer(index, this_pointer)
    }

    pub fn set_this(&self, this: usize) {
        self.scopes.borrow().borrow_mut().set_this(this);
    }

    pub fn get_this(&self) -> Option<usize> {
        self.scopes.borrow().borrow().get_this()
    }

    pub fn unset_this(&self) {
        self.scopes.borrow().borrow_mut().unset_this();
    }

    pub fn get_class_name(&self, index: usize) -> Result<String, RuntimeError> {
        self.instance_manager
            .borrow()
            .get_class_name(index)
            .map(|s| s.to_string())
    }

    pub fn print_scopes(&self) {
        println!("scopes:\n{:?}", self.scopes.borrow().borrow());
    }
}
