mod run;
mod run_state;
mod runnable_builders;

use run::Runnable;
pub use run_state::RunState;

use crate::common::Visitable;
use crate::{evaluation::RuntimeError, syntax_analysis::ProgramAst};
use runnable_builders::RunnableBuilder;

pub struct Program {
    program: Box<dyn Runnable>,
    state: RunState,
}

impl Program {
    pub fn new(program_ast: ProgramAst) -> Result<Self, RuntimeError> {
        let runner = program_ast.accept(&RunnableBuilder)?;
        Ok(Self {
            program: runner,
            state: RunState::default(),
        })
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        self.program.run(&mut self.state)
    }
}
