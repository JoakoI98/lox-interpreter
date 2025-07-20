use super::declaration_builders::RunnableBuilder;
use crate::common::{Visitable, VisitorWithContext};
use crate::evaluation::evaluator::AssignmentEvaluatorBuilder;
use crate::evaluation::resolver::ResolverError;
use crate::evaluation::run::runnable::{
    ExpressionRunnable, ForStatementRunnable, IsStatementRunnable, PrintRunnable, ReturnRunnable,
    Runnable, WhileStatementRunnable,
};
use crate::evaluation::runtime_value::Result;
use crate::evaluation::BuilderContext;
use crate::evaluation::RuntimeError;
use crate::syntax_analysis::{
    ExprStatement, ForStatement, ForStatementType, IfStatement, PrintStatement, ReturnStatement,
    Statement, StatementType, WhileStatement,
};

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
            StatementType::Block(block) => block.accept_with_context(&Self, context),
            StatementType::IfStatement(if_stmt) => if_stmt.accept_with_context(&Self, context),
            StatementType::WhileStatement(while_stmt) => {
                while_stmt.accept_with_context(&Self, context)
            }
            StatementType::ForStatement(for_stmt) => for_stmt.accept_with_context(&Self, context),
            StatementType::ReturnStatement(return_stmt) => {
                return_stmt.accept_with_context(&Self, context)
            }
            StatementType::None => Err(RuntimeError::ASTInvalidStructure),
        }
    }
}

impl VisitorWithContext<&IfStatement, Result<Box<dyn Runnable>>, BuilderContext>
    for RunnableBuilder
{
    fn visit_with_context(
        &self,
        node: &IfStatement,
        context: &BuilderContext,
    ) -> Result<Box<dyn Runnable>> {
        let if_expr = node
            .bool_expr
            .accept_with_context(&AssignmentEvaluatorBuilder, context)?;
        let true_block = node.true_statement.accept_with_context(&Self, context)?;
        let else_block = node
            .false_statement
            .as_ref()
            .map(|block| block.accept_with_context(&Self, context))
            .transpose()?;
        Ok(Box::new(IsStatementRunnable::new(
            if_expr, true_block, else_block,
        )))
    }
}

impl VisitorWithContext<&WhileStatement, Result<Box<dyn Runnable>>, BuilderContext>
    for RunnableBuilder
{
    fn visit_with_context(
        &self,
        node: &WhileStatement,
        context: &BuilderContext,
    ) -> Result<Box<dyn Runnable>> {
        let eval_expr = node
            .eval_expr
            .accept_with_context(&AssignmentEvaluatorBuilder, context)?;
        let statement = node.statement.accept_with_context(&Self, context)?;
        Ok(Box::new(WhileStatementRunnable::new(eval_expr, statement)))
    }
}

impl VisitorWithContext<&ForStatement, Result<Box<dyn Runnable>>, BuilderContext>
    for RunnableBuilder
{
    fn visit_with_context(
        &self,
        node: &ForStatement,
        context: &BuilderContext,
    ) -> Result<Box<dyn Runnable>> {
        context.resolver.borrow_mut().enter_scope()?;
        let var_declaration = match &node.token_type {
            ForStatementType::VarDeclaration(var) => Some(var.accept_with_context(&Self, context)?),
            ForStatementType::ExprStatement(expr) => {
                Some(expr.accept_with_context(&Self, context)?)
            }
            ForStatementType::Semicolon => None,
            ForStatementType::None => return Err(RuntimeError::ASTInvalidStructure),
        };

        let condition = node
            .condition
            .expr
            .as_ref()
            .map(|expr| expr.accept_with_context(&AssignmentEvaluatorBuilder, context))
            .transpose()?;
        let increment = node
            .increment
            .expr
            .as_ref()
            .map(|expr| expr.accept_with_context(&AssignmentEvaluatorBuilder, context))
            .transpose()?;
        let statement = node.statement.accept_with_context(&Self, context)?;
        context.resolver.borrow_mut().exit_scope()?;
        Ok(Box::new(ForStatementRunnable::new(
            var_declaration,
            condition,
            increment,
            statement,
        )))
    }
}

impl VisitorWithContext<&ReturnStatement, Result<Box<dyn Runnable>>, BuilderContext>
    for RunnableBuilder
{
    fn visit_with_context(
        &self,
        node: &ReturnStatement,
        context: &BuilderContext,
    ) -> Result<Box<dyn Runnable>> {
        let function_depth = context.resolver.borrow().function_depth();
        if function_depth == 0 {
            return Err(ResolverError::ReturnOutsideFunction(
                node.token_list.first().unwrap().line,
            )
            .into());
        }
        let expr = node
            .expr
            .as_ref()
            .map(|e| e.accept_with_context(&AssignmentEvaluatorBuilder, context))
            .transpose()?;
        Ok(Box::new(ReturnRunnable::new(expr)))
    }
}
