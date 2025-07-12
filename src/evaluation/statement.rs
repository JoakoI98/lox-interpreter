use super::runtime_value::{Result, RuntimeError, RuntimeValue};
use crate::syntax_analysis::{PrintStatement, Program, Statement, StatementType};

impl PrintStatement {
    pub fn run(&self) -> Result<RuntimeValue> {
        let value = self.expr.eval()?;
        println!("{}", value);
        Ok(value)
    }
}

impl Statement {
    pub fn run(&self) -> Result<RuntimeValue> {
        match &self.token_type {
            StatementType::PrintStatement(p) => p.run(),
            StatementType::ExprStatement(e) => e.expr.eval(),
            _ => Err(RuntimeError::UnexpectedRuntimeError),
        }
    }
}

impl Program {
    pub fn run(&self) -> Result<RuntimeValue> {
        for (_, statement) in &self.statements {
            statement.run()?;
        }
        Ok(RuntimeValue::Nil)
    }
}
