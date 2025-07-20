use crate::evaluation::{
    evaluator::Evaluable,
    run::{Callable, Runnable},
    RunState, RuntimeError, RuntimeValue,
};

#[derive(Debug)]
pub struct ClassInitializationCallable {
    identifier: String,
}

impl ClassInitializationCallable {
    pub fn new(identifier: String) -> Self {
        Self { identifier }
    }
}

impl Evaluable for ClassInitializationCallable {
    fn eval(&self, state: &RunState) -> Result<RuntimeValue, RuntimeError> {
        let instance_index = state.initialize_instance()?;

        Ok(RuntimeValue::ClassInstance(
            instance_index,
            self.identifier.clone(),
        ))
    }
}

impl Callable for ClassInitializationCallable {
    fn arity(&self) -> usize {
        0
    }

    fn define_arguments(&self, _: Vec<RuntimeValue>, _: &RunState) -> Result<(), RuntimeError> {
        Ok(())
    }

    fn call(&self, _: Vec<RuntimeValue>, state: &RunState) -> Result<RuntimeValue, RuntimeError> {
        self.eval(state)
    }
}

#[derive(Debug)]
pub struct ClassDeclarationRunnable {
    class_constructor_pointer: usize,
    identifier: String,
}

impl ClassDeclarationRunnable {
    pub fn new(class_constructor_pointer: usize, identifier: String) -> Self {
        Self {
            class_constructor_pointer,
            identifier,
        }
    }
}

impl Runnable for ClassDeclarationRunnable {
    fn run(&self, state: &RunState) -> Result<Option<RuntimeValue>, RuntimeError> {
        state.declare_variable(
            self.identifier.clone(),
            Some(RuntimeValue::callable(
                self.class_constructor_pointer,
                self.identifier.clone(),
                None,
                true,
            )),
            Some(0),
        );

        Ok(None)
    }
}
