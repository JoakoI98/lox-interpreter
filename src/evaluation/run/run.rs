use std::collections::HashMap;

use crate::evaluation::runtime_value::Result as RuntimeResult;
use crate::evaluation::{evaluator::Evaluable, RuntimeValue};

type RunResult = RuntimeResult<()>;

#[derive(Default, Debug)]
pub struct RunState {
    global_variables: HashMap<String, Option<RuntimeValue>>,
}

impl RunState {
    #[inline]
    pub fn declare_global_variable(&mut self, identifier: String, value: Option<RuntimeValue>) {
        self.global_variables.insert(identifier, value);
    }
}

pub trait Runnable {
    fn run(&self, state: &mut RunState) -> RunResult;
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
    fn run(&self, _: &mut RunState) -> RunResult {
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
    fn run(&self, _: &mut RunState) -> RunResult {
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
    fn run(&self, state: &mut RunState) -> RunResult {
        for statement in &self.statements {
            statement.run(state)?;
        }
        Ok(())
    }
}

pub struct VarDeclarationRunnable {
    identifier: String,
    expr: Option<Box<dyn Evaluable>>,
}

impl VarDeclarationRunnable {
    pub(super) fn new(identifier: String, expr: Option<Box<dyn Evaluable>>) -> Self {
        Self { identifier, expr }
    }
}

impl Runnable for VarDeclarationRunnable {
    fn run(&self, state: &mut RunState) -> RunResult {
        let mut value = None;
        if let Some(expr) = &self.expr {
            value = Some(expr.eval()?);
        }
        state.declare_global_variable(self.identifier.clone(), value);
        Ok(())
    }
}
