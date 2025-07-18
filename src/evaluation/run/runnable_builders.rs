use super::super::runtime_value::Result;
use super::super::BuilderContext;
use super::run::{ExpressionRunnable, PrintRunnable, ProgramRunnable, Runnable};
use crate::evaluation::evaluator::AssignmentEvaluatorBuilder;
use crate::evaluation::run::run::VarDeclarationRunnable;
use crate::syntax_analysis::{Declaration, DeclarationType, VarDeclaration};
use crate::tokenizer::TokenValue;
use crate::{
    common::{Visitable, VisitorWithContext},
    evaluation::RuntimeError,
    syntax_analysis::{ExprStatement, PrintStatement, ProgramAst, Statement, StatementType},
};

pub struct RunnableBuilder;

impl VisitorWithContext<&PrintStatement, Result<Box<dyn Runnable>>, BuilderContext>
    for RunnableBuilder
{
    fn visit_with_context(
        &self,
        node: &PrintStatement,
        context: &BuilderContext,
    ) -> Result<Box<dyn Runnable>> {
        let expr = node
            .expr
            .accept_with_context(&AssignmentEvaluatorBuilder, context)?;
        Ok(Box::new(PrintRunnable::new(expr)))
    }
}

impl VisitorWithContext<&ExprStatement, Result<Box<dyn Runnable>>, BuilderContext>
    for RunnableBuilder
{
    fn visit_with_context(
        &self,
        node: &ExprStatement,
        context: &BuilderContext,
    ) -> Result<Box<dyn Runnable>> {
        let expr = node
            .expr
            .accept_with_context(&AssignmentEvaluatorBuilder, context)?;
        Ok(Box::new(ExpressionRunnable::new(expr)))
    }
}

impl VisitorWithContext<&Statement, Result<Box<dyn Runnable>>, BuilderContext> for RunnableBuilder {
    fn visit_with_context(
        &self,
        node: &Statement,
        context: &BuilderContext,
    ) -> Result<Box<dyn Runnable>> {
        match &node.token_type {
            StatementType::ExprStatement(expr) => expr.accept_with_context(&Self, context),
            StatementType::PrintStatement(print) => print.accept_with_context(&Self, context),
            StatementType::None => Err(RuntimeError::ASTInvalidStructure),
        }
    }
}

impl VisitorWithContext<&VarDeclaration, Result<Box<dyn Runnable>>, BuilderContext>
    for RunnableBuilder
{
    fn visit_with_context(
        &self,
        node: &VarDeclaration,
        context: &BuilderContext,
    ) -> Result<Box<dyn Runnable>> {
        let ident_token = node
            .token_list
            .get(1)
            .ok_or(RuntimeError::ASTInvalidStructure)?;
        let ident_value = match &ident_token.token_value {
            TokenValue::Identifier(_) => ident_token.lexeme.clone(),
            _ => return Err(RuntimeError::ASTInvalidStructure),
        };

        let mut evaluable = None;
        if let Some(expr) = &node.expr {
            evaluable = Some(expr.accept_with_context(&AssignmentEvaluatorBuilder, context)?);
        }

        Ok(Box::new(VarDeclarationRunnable::new(
            ident_value,
            evaluable,
        )))
    }
}

impl VisitorWithContext<&Declaration, Result<Box<dyn Runnable>>, BuilderContext>
    for RunnableBuilder
{
    fn visit_with_context(
        &self,
        node: &Declaration,
        context: &BuilderContext,
    ) -> Result<Box<dyn Runnable>> {
        match &node.token_type {
            DeclarationType::VarDeclaration(var) => var.accept_with_context(&Self, context),
            DeclarationType::Statement(stmt) => stmt.accept_with_context(&Self, context),
            DeclarationType::None => Err(RuntimeError::ASTInvalidStructure),
        }
    }
}

impl VisitorWithContext<&ProgramAst, Result<Box<dyn Runnable>>, BuilderContext>
    for RunnableBuilder
{
    fn visit_with_context(
        &self,
        node: &ProgramAst,
        context: &BuilderContext,
    ) -> Result<Box<dyn Runnable>> {
        let statements = node
            .statements
            .iter()
            .map(|(_, stmt)| stmt.accept_with_context(&Self, context))
            .collect::<Result<Vec<Box<dyn Runnable>>>>()?;
        Ok(Box::new(ProgramRunnable::new(statements)))
    }
}
