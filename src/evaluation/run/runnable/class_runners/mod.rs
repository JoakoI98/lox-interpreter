use crate::evaluation::{
    evaluator::{Evaluable, PrimaryEvaluator, INIT_FUNCTION_NAME},
    run::{Callable, Runnable},
    runtime_value::{CallableType, ThisInstance},
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
    fn arity(&self, state: &RunState) -> Result<usize, RuntimeError> {
        self.methods
            .iter()
            .find(|(_, method_name)| method_name == INIT_FUNCTION_NAME)
            .map(|(init_pointer, _)| state.function_arity(*init_pointer))
            .transpose()
            .map(|a| a.unwrap_or(0))
    }

    fn call(
        &self,
        arguments: Vec<RuntimeValue>,
        _: Option<ThisInstance>,
        state: &RunState,
    ) -> Result<RuntimeValue, RuntimeError> {
        let super_class = self
            .super_class
            .as_ref()
            .map(|super_class| {
                let arity = state.function_arity(*super_class)?;
                if arity > arguments.len() {
                    return Err(RuntimeError::ArityMismatch);
                }
                let super_class_arguments = arguments.iter().take(arity).cloned().collect();
                state.call_function(*super_class, super_class_arguments, None, None)
            })
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
                    CallableType::Method(ThisInstance::current(instance_index)),
                ),
            )?;
        }

        let value = RuntimeValue::ClassInstance(instance_index, self.identifier.clone());
        state.map_this_pointer(instance_index, instance_index)?;
        match value {
            RuntimeValue::ClassInstance(this_pointer, _) => {
                let init_callable =
                    state.get_instance_value(this_pointer, INIT_FUNCTION_NAME, None, None)?;
                match init_callable {
                    Some(RuntimeValue::Callable(callable)) => {
                        state.call_function(
                            callable.get_pointer(),
                            arguments,
                            None,
                            Some(ThisInstance::current(this_pointer)),
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
    super_class_evaluable: Option<(PrimaryEvaluator, usize)>,
}

impl ClassDeclarationRunnable {
    pub fn new(
        class_constructor_pointer: usize,
        identifier: String,
        super_class_evaluable: Option<(PrimaryEvaluator, usize)>,
    ) -> Self {
        Self {
            class_constructor_pointer,
            identifier,
            super_class_evaluable,
        }
    }
}

impl Runnable for ClassDeclarationRunnable {
    fn run(&self, state: &RunState) -> Result<Option<RuntimeValue>, RuntimeError> {
        if let Some((evaluator, line)) = &self.super_class_evaluable {
            let super_class = evaluator.eval(state)?;
            match super_class {
                RuntimeValue::Callable(callable) => {
                    if !callable.is_class_constructor() {
                        return Err(RuntimeError::SuperClassMustBeAClass(*line));
                    }
                }
                _ => return Err(RuntimeError::SuperClassMustBeAClass(*line)),
            }
        }

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
