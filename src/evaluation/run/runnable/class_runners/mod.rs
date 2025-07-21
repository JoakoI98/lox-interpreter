use crate::evaluation::{
    evaluator::Evaluable,
    run::{Callable, Runnable},
    runtime_value::CallableType,
    RunState, RuntimeError, RuntimeValue,
};

#[derive(Debug)]
pub struct ClassInitializationCallable {
    identifier: String,
    methods: Vec<(usize, String)>,
}

impl ClassInitializationCallable {
    pub fn new(identifier: String, methods: Vec<(usize, String)>) -> Self {
        Self {
            identifier,
            methods,
        }
    }
}

impl Evaluable for ClassInitializationCallable {
    fn eval(&self, state: &RunState) -> Result<RuntimeValue, RuntimeError> {
        let instance_index = state.initialize_instance(self.identifier.clone())?;
        for (pointer, method_name) in &self.methods {
            state.set_instance_value(
                instance_index,
                &method_name,
                RuntimeValue::callable(
                    *pointer,
                    method_name.clone(),
                    None,
                    CallableType::Method(instance_index),
                ),
            )?;
        }

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

    fn call(
        &self,
        _: Vec<RuntimeValue>,
        _: Option<usize>,
        state: &RunState,
    ) -> Result<RuntimeValue, RuntimeError> {
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
                CallableType::ClassConstructor,
            )),
            Some(0),
        );

        Ok(None)
    }
}
