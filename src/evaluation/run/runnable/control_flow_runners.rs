use super::super::run_state::RunState;
use super::basic_runners::Runnable;
use crate::evaluation::evaluator::Evaluable;
use crate::evaluation::run::runnable::basic_runners::RunResult;
use crate::evaluation::runtime_value::Result as RuntimeResult;

#[derive(Debug)]
pub struct IsStatementRunnable {
    if_expr: Box<dyn Evaluable>,
    true_block: Box<dyn Runnable>,
    else_block: Option<Box<dyn Runnable>>,
}

impl IsStatementRunnable {
    pub fn new(
        if_expr: Box<dyn Evaluable>,
        true_block: Box<dyn Runnable>,
        else_block: Option<Box<dyn Runnable>>,
    ) -> Self {
        Self {
            if_expr,
            true_block,
            else_block,
        }
    }
}

impl Runnable for IsStatementRunnable {
    fn run(&self, state: &RunState) -> RunResult {
        let mut ret = None;
        let is_true = self.if_expr.eval(state)?.to_bool()?;
        if is_true {
            ret = self.true_block.run(state)?;
        } else if let Some(else_block) = &self.else_block {
            ret = else_block.run(state)?;
        }
        Ok(ret)
    }
}

#[derive(Debug)]
pub struct WhileStatementRunnable {
    eval_expr: Box<dyn Evaluable>,
    statement: Box<dyn Runnable>,
}

impl WhileStatementRunnable {
    pub fn new(eval_expr: Box<dyn Evaluable>, statement: Box<dyn Runnable>) -> Self {
        Self {
            eval_expr,
            statement,
        }
    }
}

impl Runnable for WhileStatementRunnable {
    fn run(&self, state: &RunState) -> RunResult {
        while self.eval_expr.eval(state)?.to_bool()? {
            let ret = self.statement.run(state)?;
            if ret.is_some() {
                return Ok(ret);
            }
        }
        Ok(None)
    }
}

#[derive(Debug)]
pub struct ForStatementRunnable {
    var_declaration: Option<Box<dyn Runnable>>,
    condition: Option<Box<dyn Evaluable>>,
    increment: Option<Box<dyn Evaluable>>,
    statement: Box<dyn Runnable>,
}

impl ForStatementRunnable {
    pub fn new(
        var_declaration: Option<Box<dyn Runnable>>,
        condition: Option<Box<dyn Evaluable>>,
        increment: Option<Box<dyn Evaluable>>,
        statement: Box<dyn Runnable>,
    ) -> Self {
        Self {
            var_declaration,
            condition,
            increment,
            statement,
        }
    }

    fn eval_condition(&self, state: &RunState) -> RuntimeResult<bool> {
        Ok(self
            .condition
            .as_ref()
            .map(|c| c.eval(state))
            .transpose()?
            .map(|c| c.to_bool())
            .transpose()?
            .unwrap_or(true))
    }
}

impl Runnable for ForStatementRunnable {
    fn run(&self, state: &RunState) -> RunResult {
        state.enter_scope()?;
        if let Some(var_declaration) = &self.var_declaration {
            var_declaration.run(state)?;
        }
        while self.eval_condition(state)? {
            let ret = self.statement.run(state)?;
            if ret.is_some() {
                return Ok(ret);
            }
            if let Some(increment) = &self.increment {
                increment.eval(state)?;
            }
        }
        state.exit_scope()?;
        Ok(None)
    }
}
