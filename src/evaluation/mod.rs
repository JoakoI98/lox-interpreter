mod evaluator;
mod evaluator_builders;
mod run;
mod runnable_builders;
mod runtime_value;

pub use evaluator_builders::BinaryEvaluatorBuilder;
pub use runtime_value::{RuntimeError, RuntimeValue};

use crate::common::Visitable;

use crate::{
    evaluation::{
        run::{RunState, Runnable},
        runnable_builders::RunnableBuilder,
    },
    syntax_analysis::ProgramAst,
};

pub struct Program {
    program: Box<dyn Runnable>,
    state: RunState,
}

impl Program {
    pub(super) fn new(program_ast: ProgramAst) -> Result<Self, RuntimeError> {
        let runner = program_ast.accept(&RunnableBuilder)?;
        Ok(Self {
            program: runner,
            state: RunState,
        })
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        self.program.run(&mut self.state)
    }
}
