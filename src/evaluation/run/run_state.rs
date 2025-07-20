use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::evaluation::{
    evaluator::EvaluableIdentifier, functions_resolver::FunctionsResolver, RuntimeError,
    RuntimeValue,
};

#[derive(Debug)]
pub struct RunScopes {
    values: HashMap<String, RuntimeValue>,
    enclosing: Option<RunScopeRef>,
}

type RunScopeRef = Rc<RefCell<RunScopes>>;

impl RunScopes {
    pub fn new(enclosing: Option<RunScopeRef>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing,
        }
    }

    #[inline]
    pub fn declare_variable(
        &mut self,
        identifier: String,
        value: Option<RuntimeValue>,
        depth: Option<usize>,
    ) {
        let depth = depth.unwrap_or(0);
        if depth == 0 {
            self.values
                .insert(identifier, value.unwrap_or(RuntimeValue::Nil));
        } else {
            self.enclosing
                .as_ref()
                .unwrap()
                .borrow_mut()
                .declare_variable(identifier, value, Some(depth - 1));
        }
    }

    #[inline]
    pub fn set_variable(&mut self, identifier: String, value: RuntimeValue, depth: Option<usize>) {
        let depth = depth.unwrap_or(0);
        if depth > 0 {
            return self.enclosing.as_ref().unwrap().borrow_mut().set_variable(
                identifier,
                value,
                Some(depth - 1),
            );
        }
        if self.values.contains_key(&identifier) {
            self.values.insert(identifier, value);
        } else {
            self.enclosing
                .as_ref()
                .unwrap()
                .borrow_mut()
                .set_variable(identifier, value, None);
        }
    }

    pub fn get_enclosing(&self) -> Option<RunScopeRef> {
        self.enclosing.as_ref().map(|scope| scope.clone())
    }

    pub fn evaluate_variable(
        &self,
        identifier: &EvaluableIdentifier,
        overwrite_depth: Option<usize>,
    ) -> Result<RuntimeValue, RuntimeError> {
        let depth = overwrite_depth.unwrap_or(identifier.depth().unwrap_or(0));
        if depth > 0 {
            return self
                .enclosing
                .as_ref()
                .unwrap()
                .borrow()
                .evaluate_variable(identifier, Some(depth - 1));
        }

        if self.values.contains_key(identifier.identifier()) {
            let value = self.values.get(identifier.identifier()).unwrap();
            return Ok(value.clone());
        }

        self.enclosing
            .as_ref()
            .map(|scope| scope.borrow().evaluate_variable(identifier, Some(0)))
            .transpose()?
            .ok_or(RuntimeError::UndefinedVariable(
                identifier.identifier().to_string(),
                identifier.line(),
            ))
    }
}

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
    ) -> Result<RuntimeValue, RuntimeError> {
        let resolver = self.functions_resolver.borrow();
        let pointer = resolver
            .resolve(index)
            .ok_or(RuntimeError::FunctionNotFound)?;
        pointer.call(arguments, self)
    }
}
