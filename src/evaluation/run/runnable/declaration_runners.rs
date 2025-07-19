use super::super::run_state::RunState;
use super::basic_runners::Runnable;
use crate::evaluation::evaluator::Evaluable;
use crate::evaluation::runtime_value::Result as RuntimeResult;

type RunResult = RuntimeResult<()>;

pub struct VarDeclarationRunnable {
    identifier: String,
    expr: Option<Box<dyn Evaluable>>,
}

impl VarDeclarationRunnable {
    pub fn new(identifier: String, expr: Option<Box<dyn Evaluable>>) -> Self {
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
    pub fn new(declarations: Vec<Box<dyn Runnable>>) -> Self {
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
