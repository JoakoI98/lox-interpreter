use super::super::run_state::RunState;
use crate::evaluation::evaluator::Evaluable;
use crate::evaluation::runtime_value::Result as RuntimeResult;

type RunResult = RuntimeResult<()>;

pub trait Runnable {
    fn run(&self, state: &mut RunState) -> RunResult;
}

pub struct PrintRunnable {
    value: Box<dyn Evaluable>,
}

impl PrintRunnable {
    pub fn new(value: Box<dyn Evaluable>) -> Self {
        Self { value }
    }
}

impl Runnable for PrintRunnable {
    fn run(&self, run_state: &mut RunState) -> RunResult {
        println!("{}", self.value.eval(run_state)?);
        Ok(())
    }
}

pub struct ExpressionRunnable {
    value: Box<dyn Evaluable>,
}

impl ExpressionRunnable {
    pub fn new(value: Box<dyn Evaluable>) -> Self {
        Self { value }
    }
}

impl Runnable for ExpressionRunnable {
    fn run(&self, run_state: &mut RunState) -> RunResult {
        self.value.eval(run_state)?;
        Ok(())
    }
}

pub struct ProgramRunnable {
    statements: Vec<Box<dyn Runnable>>,
}

impl ProgramRunnable {
    pub fn new(statements: Vec<Box<dyn Runnable>>) -> Self {
        Self { statements }
    }
}

impl Runnable for ProgramRunnable {
    fn run(&self, state: &mut RunState) -> RunResult {
        for statement in &self.statements {
            statement.run(state)?;
        }
        Ok(())
    }
}
