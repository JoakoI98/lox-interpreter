mod run_state;
mod runnable;
mod runnable_builders;

pub use run_state::{RunScopes, RunState};
use runnable::Runnable;
pub use runnable::{Callable, NativeFunctionError};

use crate::common::Visitable;
use crate::evaluation::run::runnable::get_native_functions;
use crate::evaluation::{BuilderContext, RuntimeValue};
use crate::{evaluation::RuntimeError, syntax_analysis::ProgramAst};
use runnable_builders::RunnableBuilder;

pub struct Program {
    program: Box<dyn Runnable>,
    state: RunState,
}

impl Program {
    pub fn new_with_context(program_ast: ProgramAst) -> Result<Self, RuntimeError> {
        let context = BuilderContext::new()?;
        let mut scopes = RunScopes::new();
        Self::initialize_context(&context, &mut scopes)?;
        let state = RunState::new(context.functions_resolver.take(), scopes);
        let runner = program_ast.accept_with_context(&RunnableBuilder, &context)?;
        Ok(Self {
            program: runner,
            state,
        })
    }

    fn initialize_context(
        context: &BuilderContext,
        scopes: &mut RunScopes,
    ) -> Result<(), RuntimeError> {
        let functions = get_native_functions();
        for (name, function) in functions.into_iter() {
            let pointer = context
                .functions_resolver
                .borrow_mut()
                .add_function(function)?;
            context.resolver.borrow_mut().declare(name)?;
            context.resolver.borrow_mut().define(name)?;
            let callable_value = RuntimeValue::Callable(pointer);
            scopes.set_variable(name.to_string(), callable_value, None);
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        self.program.run(&mut self.state)
    }
}
