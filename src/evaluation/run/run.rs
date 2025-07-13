use crate::evaluation::{evaluator::Evaluable, RuntimeError};

pub struct RunState;

pub trait Runnable {
    fn run(&self, state: &mut RunState) -> Result<(), RuntimeError>;
}

pub struct PrintRunnable {
    value: Box<dyn Evaluable>,
}

impl PrintRunnable {
    pub(super) fn new(value: Box<dyn Evaluable>) -> Self {
        Self { value }
    }
}

impl Runnable for PrintRunnable {
    fn run(&self, _: &mut RunState) -> Result<(), RuntimeError> {
        println!("{}", self.value.eval()?);
        Ok(())
    }
}

pub struct ExpressionRunnable {
    value: Box<dyn Evaluable>,
}

impl ExpressionRunnable {
    pub(super) fn new(value: Box<dyn Evaluable>) -> Self {
        Self { value }
    }
}

impl Runnable for ExpressionRunnable {
    fn run(&self, _: &mut RunState) -> Result<(), RuntimeError> {
        self.value.eval()?;
        Ok(())
    }
}

pub struct ProgramRunnable {
    statements: Vec<Box<dyn Runnable>>,
}

impl ProgramRunnable {
    pub(super) fn new(statements: Vec<Box<dyn Runnable>>) -> Self {
        Self { statements }
    }
}

impl Runnable for ProgramRunnable {
    fn run(&self, state: &mut RunState) -> Result<(), RuntimeError> {
        for statement in &self.statements {
            statement.run(state)?;
        }
        Ok(())
    }
}
