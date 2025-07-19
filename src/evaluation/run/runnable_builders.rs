use super::super::runtime_value::Result;
use super::super::BuilderContext;
use super::run::{
    ExpressionRunnable, IsStatementRunnable, PrintRunnable, ProgramRunnable, Runnable,
};
use crate::evaluation::evaluator::AssignmentEvaluatorBuilder;
use crate::evaluation::run::run::{
    BlockRunnable, ForStatementRunnable, VarDeclarationRunnable, WhileStatementRunnable,
};
use crate::syntax_analysis::{
    Declaration, DeclarationType, ForStatement, ForStatementType, IfStatement, VarDeclaration,
    WhileStatement,
};
use crate::tokenizer::TokenValue;
use crate::{
    common::{Visitable, VisitorWithContext},
    evaluation::RuntimeError,
    syntax_analysis::{Block, ExprStatement, PrintStatement, ProgramAst, Statement, StatementType},
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
            StatementType::Block(block) => block.accept_with_context(&Self, context),
            StatementType::IfStatement(if_stmt) => if_stmt.accept_with_context(&Self, context),
            StatementType::WhileStatement(while_stmt) => {
                while_stmt.accept_with_context(&Self, context)
            }
            StatementType::ForStatement(for_stmt) => for_stmt.accept_with_context(&Self, context),
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

impl VisitorWithContext<&Block, Result<Box<dyn Runnable>>, BuilderContext> for RunnableBuilder {
    fn visit_with_context(
        &self,
        node: &Block,
        context: &BuilderContext,
    ) -> Result<Box<dyn Runnable>> {
        context.resolver.borrow_mut().enter_scope()?;
        let declarations = node
            .declarations
            .iter()
            .map(|(_, stmt)| stmt.accept_with_context(&Self, context))
            .collect::<Result<Vec<Box<dyn Runnable>>>>()?;
        context.resolver.borrow_mut().exit_scope()?;
        Ok(Box::new(BlockRunnable::new(declarations)))
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
