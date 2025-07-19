use super::super::run_state::RunState;
use crate::evaluation::evaluator::Evaluable;
use crate::evaluation::runtime_value::Result as RuntimeResult;
use crate::evaluation::RuntimeValue;

pub type RunResult = RuntimeResult<Option<RuntimeValue>>;

pub trait Runnable: std::fmt::Debug {
    fn run(&self, state: &RunState) -> RunResult;
}

#[derive(Debug)]
pub struct PrintRunnable {
    value: Box<dyn Evaluable>,
}

impl PrintRunnable {
    pub fn new(value: Box<dyn Evaluable>) -> Self {
        Self { value }
    }
}

impl Runnable for PrintRunnable {
    fn run(&self, run_state: &RunState) -> RunResult {
        println!("{}", self.value.eval(run_state)?);
        Ok(None)
    }
}

#[derive(Debug)]
pub struct ExpressionRunnable {
    value: Box<dyn Evaluable>,
}

impl ExpressionRunnable {
    pub fn new(value: Box<dyn Evaluable>) -> Self {
        Self { value }
    }
}

impl Runnable for ExpressionRunnable {
    fn run(&self, run_state: &RunState) -> RunResult {
        self.value.eval(run_state)?;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct ProgramRunnable {
    statements: Vec<Box<dyn Runnable>>,
}

impl ProgramRunnable {
    pub fn new(statements: Vec<Box<dyn Runnable>>) -> Self {
        Self { statements }
    }
}

impl Runnable for ProgramRunnable {
    fn run(&self, state: &RunState) -> RunResult {
        for statement in &self.statements {
            let ret = statement.run(state)?;
            if ret.is_some() {
                return Ok(ret);
            }
        }
        Ok(None)
    }
}

#[derive(Debug)]
pub struct FunctionDeclarationRunnable {
    function_pointer: usize,
    identifier: String,
}

impl FunctionDeclarationRunnable {
    pub fn new(function_pointer: usize, identifier: String) -> Self {
        Self {
            function_pointer,
            identifier,
        }
    }
}

impl Runnable for FunctionDeclarationRunnable {
    fn run(&self, state: &RunState) -> RunResult {
        state.set_variable(
            self.identifier.clone(),
            RuntimeValue::Callable(self.function_pointer, self.identifier.clone()),
            Some(0),
        );
        Ok(None)
    }
}
