use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::evaluation::{
    evaluator::EvaluableIdentifier, runtime_value::ThisInstance, RuntimeError, RuntimeValue,
};

pub struct RunScopes {
    values: HashMap<String, RuntimeValue>,
    this: Option<ThisInstance>,
    enclosing: Option<RunScopeRef>,
}

pub type RunScopeRef = Rc<RefCell<RunScopes>>;

impl RunScopes {
    pub fn new(enclosing: Option<RunScopeRef>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing,
            this: None,
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

    fn print_with_depth(&self, depth: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(enclosing) = &self.enclosing {
            enclosing.borrow().print_with_depth(depth - 1, f)?;
        }
        write!(f, "{} this({:?}): {:?}\n", depth, self.this, self.values)
    }

    fn depth(&self) -> usize {
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().depth() + 1;
        }
        0
    }

    pub fn set_this(&mut self, this: ThisInstance) {
        self.this = Some(this);
    }

    pub fn get_this(&self) -> Option<ThisInstance> {
        let mut this = self.this.clone();
        if this.is_none() {
            if let Some(enclosing) = &self.enclosing {
                this = enclosing.borrow().get_this();
            }
        }
        this
    }

    pub fn unset_this(&mut self) {
        self.this = None;
    }
}

impl std::fmt::Debug for RunScopes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_with_depth(self.depth(), f)?;
        Ok(())
    }
}
