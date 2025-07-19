use super::run_state::RunState;
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
    pub(super) fn new(value: Box<dyn Evaluable>) -> Self {
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
    pub(super) fn new(value: Box<dyn Evaluable>) -> Self {
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
            value = Some(expr.eval(state)?);
        }
        state.declare_variable(self.identifier.clone(), value, Some(0));
        Ok(())
    }
}

pub struct BlockRunnable {
    declarations: Vec<Box<dyn Runnable>>,
}

impl BlockRunnable {
    pub(super) fn new(declarations: Vec<Box<dyn Runnable>>) -> Self {
        Self { declarations }
    }
}

impl Runnable for BlockRunnable {
    fn run(&self, state: &mut RunState) -> RunResult {
        state.enter_scope()?;
        for declaration in &self.declarations {
            declaration.run(state)?;
        }
        state.exit_scope()?;
        Ok(())
    }
}
