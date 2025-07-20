use crate::common::{Visitable, VisitorWithContext};
use crate::evaluation::evaluator::{AssignmentEvaluatorBuilder, FunctionCallable};
use crate::evaluation::run::runnable::{ClassDeclarationRunnable, ClassInitializationCallable};
use crate::evaluation::run::runnable::{
    FunctionDeclarationRunnable, Runnable, VarDeclarationRunnable,
};
use crate::evaluation::runtime_value::Result;
use crate::evaluation::BuilderContext;
use crate::evaluation::RuntimeError;
use crate::syntax_analysis::{
    ClassDeclaration, Declaration, DeclarationType, FunctionDeclaration, VarDeclaration,
};
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
        context
            .resolver
            .borrow_mut()
            .declare(&ident_value, ident_token.line)?;
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
            DeclarationType::FunctionDeclaration(func) => func.accept_with_context(&Self, context),
            DeclarationType::ClassDeclaration(class) => class.accept_with_context(&Self, context),
            DeclarationType::None => Err(RuntimeError::ASTInvalidStructure),
        }
    }
}

impl VisitorWithContext<&FunctionDeclaration, Result<Box<dyn Runnable>>, BuilderContext>
    for RunnableBuilder
{
    fn visit_with_context(
        &self,
        node: &FunctionDeclaration,
        context: &BuilderContext,
    ) -> Result<Box<dyn Runnable>> {
        let function_ast = &node.function;
        let parameters = &function_ast.parameters.parameters;

        let function_ident = function_ast
            .token_list
            .first()
            .ok_or(RuntimeError::ASTInvalidStructure)?;
        let function_ident_string = function_ident.lexeme.clone();

        context
            .resolver
            .borrow_mut()
            .declare(&function_ident_string, function_ident.line)?;
        context
            .resolver
            .borrow_mut()
            .define(&function_ident_string)?;
        context.resolver.borrow_mut().enter_scope()?;
        context.resolver.borrow_mut().enter_function();
        for parameter in parameters {
            context
                .resolver
                .borrow_mut()
                .declare(&parameter.token.lexeme, parameter.token.line)?;
            context
                .resolver
                .borrow_mut()
                .define(&&parameter.token.lexeme)?;
        }

        let block_runnable = function_ast.block.accept_with_context(&Self, context)?;
        context.resolver.borrow_mut().exit_scope()?;
        context.resolver.borrow_mut().exit_function();

        let callable = FunctionCallable::new(
            block_runnable,
            parameters.iter().map(|p| p.token.lexeme.clone()).collect(),
        );

        let pointer = context
            .functions_resolver
            .borrow_mut()
            .add_function(Box::new(callable))?;

        Ok(Box::new(FunctionDeclarationRunnable::new(
            pointer,
            function_ident_string,
        )))
    }
}

impl VisitorWithContext<&ClassDeclaration, Result<Box<dyn Runnable>>, BuilderContext>
    for RunnableBuilder
{
    fn visit_with_context(
        &self,
        node: &ClassDeclaration,
        context: &BuilderContext,
    ) -> Result<Box<dyn Runnable>> {
        let class_ident = node
            .token_list
            .get(1)
            .ok_or(RuntimeError::ASTInvalidStructure)?;
        let class_ident_string = class_ident.lexeme.clone();

        context
            .resolver
            .borrow_mut()
            .declare(&class_ident_string, class_ident.line)?;
        context.resolver.borrow_mut().define(&class_ident_string)?;
        context.resolver.borrow_mut().enter_class();
        context.resolver.borrow_mut().exit_class();

        let callable = ClassInitializationCallable::new(class_ident_string.clone());

        let pointer = context
            .functions_resolver
            .borrow_mut()
            .add_function(Box::new(callable))?;

        Ok(Box::new(ClassDeclarationRunnable::new(
            pointer,
            class_ident_string,
        )))
    }
}
