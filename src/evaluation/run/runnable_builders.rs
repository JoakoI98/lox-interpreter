use super::super::runtime_value::Result;
use super::run::{ExpressionRunnable, PrintRunnable, ProgramRunnable, Runnable};
use crate::{
    common::{Visitable, Visitor},
    evaluation::{BinaryEvaluatorBuilder, RuntimeError},
    syntax_analysis::{ExprStatement, PrintStatement, ProgramAst, Statement, StatementType},
};

pub struct RunnableBuilder;

impl Visitor<&PrintStatement, Result<Box<dyn Runnable>>> for RunnableBuilder {
    fn visit(&self, node: &PrintStatement) -> Result<Box<dyn Runnable>> {
        let expr = node.expr.accept(&BinaryEvaluatorBuilder)?;
        Ok(Box::new(PrintRunnable::new(expr)))
    }
}

impl Visitor<&ExprStatement, Result<Box<dyn Runnable>>> for RunnableBuilder {
    fn visit(&self, node: &ExprStatement) -> Result<Box<dyn Runnable>> {
        let expr = node.expr.accept(&BinaryEvaluatorBuilder)?;
        Ok(Box::new(ExpressionRunnable::new(expr)))
    }
}

impl Visitor<&Statement, Result<Box<dyn Runnable>>> for RunnableBuilder {
    fn visit(&self, node: &Statement) -> Result<Box<dyn Runnable>> {
        match &node.token_type {
            StatementType::ExprStatement(expr) => expr.accept(&Self),
            StatementType::PrintStatement(print) => print.accept(&Self),
            StatementType::None => Err(RuntimeError::ASTInvalidStructure),
        }
    }
}

impl Visitor<&ProgramAst, Result<Box<dyn Runnable>>> for RunnableBuilder {
    fn visit(&self, node: &ProgramAst) -> Result<Box<dyn Runnable>> {
        let statements = node
            .statements
            .iter()
            .map(|(_, stmt)| stmt.accept(&Self))
            .collect::<Result<Vec<Box<dyn Runnable>>>>()?;
        Ok(Box::new(ProgramRunnable::new(statements)))
    }
}
