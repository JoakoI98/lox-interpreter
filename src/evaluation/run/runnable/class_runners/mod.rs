use crate::evaluation::{
    evaluator::{Evaluable, INIT_FUNCTION_NAME},
    run::{Callable, Runnable},
    runtime_value::CallableType,
    RunState, RuntimeError, RuntimeValue,
};

#[derive(Debug)]
pub struct ClassInitializationCallable {
    identifier: String,
    methods: Vec<(usize, String)>,
    super_class: Option<usize>,
}

impl ClassInitializationCallable {
    pub fn new(
        identifier: String,
        methods: Vec<(usize, String)>,
        super_class: Option<usize>,
    ) -> Self {
        Self {
            identifier,
            methods,
            super_class,
        }
    }
}

impl Evaluable for ClassInitializationCallable {
    fn eval(&self, _: &RunState) -> Result<RuntimeValue, RuntimeError> {
        Ok(RuntimeValue::Nil)
    }
}

impl Callable for ClassInitializationCallable {
    fn call(
        &self,
        arguments: Vec<RuntimeValue>,
        _: Option<usize>,
        state: &RunState,
    ) -> Result<RuntimeValue, RuntimeError> {
        let super_class = self
            .super_class
            .as_ref()
            .map(|super_class| state.call_function(*super_class, arguments.clone(), None, None))
            .transpose()?
            .map(|super_class| super_class.get_class_instance())
            .flatten();

        if self.super_class.is_some() && super_class.is_none() {
            return Err(RuntimeError::SuperClassNotFound);
        }

        let instance_index = state.initialize_instance(self.identifier.clone(), super_class)?;
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

        let value = RuntimeValue::ClassInstance(instance_index, self.identifier.clone());
        match value {
            RuntimeValue::ClassInstance(this_pointer, _) => {
                let init_callable =
                    state.get_instance_value(this_pointer, INIT_FUNCTION_NAME, Some(1))?;
                match init_callable {
                    Some(RuntimeValue::Callable(callable)) => {
                        state.call_function(
                            callable.get_pointer(),
                            arguments,
                            None,
                            Some(this_pointer),
                        )?;
                    }
                    _ => {}
                }
            }
            _ => unreachable!(),
        }

        Ok(value)
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
