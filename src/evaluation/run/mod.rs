mod run;
mod run_state;
mod runnable_builders;

use run::Runnable;
pub use run_state::RunState;

use crate::common::Visitable;
use crate::evaluation::BuilderContext;
use crate::{evaluation::RuntimeError, syntax_analysis::ProgramAst};
use runnable_builders::RunnableBuilder;

pub struct Program {
    program: Box<dyn Runnable>,
    state: RunState,
}

impl Program {
    pub fn new_with_context(program_ast: ProgramAst) -> Result<Self, RuntimeError> {
        let context = BuilderContext::new()?;
        let runner = program_ast.accept_with_context(&RunnableBuilder, &context)?;
        Ok(Self {
            program: runner,
            state: RunState::default(),
        })
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        self.program.run(&mut self.state)
    }
}
