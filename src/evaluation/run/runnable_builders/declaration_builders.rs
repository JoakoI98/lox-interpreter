use crate::common::{Visitable, VisitorWithContext};
use crate::evaluation::evaluator::AssignmentEvaluatorBuilder;
use crate::evaluation::run::runnable::{Runnable, VarDeclarationRunnable};
use crate::evaluation::runtime_value::Result;
use crate::evaluation::BuilderContext;
use crate::evaluation::RuntimeError;
use crate::syntax_analysis::{Declaration, DeclarationType, VarDeclaration};
use crate::tokenizer::TokenValue;

pub struct RunnableBuilder;

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
        context.resolver.borrow_mut().declare(&ident_value)?;
        if let Some(expr) = &node.expr {
            evaluable = Some(expr.accept_with_context(&AssignmentEvaluatorBuilder, context)?);
        }
        context.resolver.borrow_mut().define(&ident_value)?;

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
