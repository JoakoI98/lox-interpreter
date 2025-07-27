use crate::common::{Visitable, VisitorWithContext};
use crate::evaluation::evaluator::{
    AssignmentEvaluatorBuilder, FunctionCallable, PrimaryEvaluator,
};
use crate::evaluation::resolver::ResolverError;
use crate::evaluation::run::runnable::{
    ClassDeclarationRunnable, ClassInitializationCallable, IfNoClassErrorRunnable,
};
use crate::evaluation::run::runnable::{
    FunctionDeclarationRunnable, Runnable, VarDeclarationRunnable,
};
use crate::evaluation::runtime_value::Result;
use crate::evaluation::BuilderContext;
use crate::evaluation::RuntimeError;
use crate::syntax_analysis::{
    ClassDeclaration, Declaration, DeclarationType, Function, FunctionDeclaration, VarDeclaration,
};
use crate::tokenizer::TokenValue;

pub struct RunnableBuilder;

impl RunnableBuilder {
    fn declare_function(
        node: &Function,
        context: &BuilderContext,
        is_method: bool,
        super_available: bool,
    ) -> Result<(usize, String)> {
        let function_ast = node;
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
        if is_method {
            context
                .resolver
                .borrow_mut()
                .enter_method(function_ident_string.clone(), super_available);
        } else {
            context
                .resolver
                .borrow_mut()
                .enter_function(function_ident_string.clone(), super_available);
        }
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
        context.resolver.borrow_mut().exit_function_or_method();

        let callable = FunctionCallable::new(
            block_runnable,
            parameters.iter().map(|p| p.token.lexeme.clone()).collect(),
            function_ident_string.clone(),
        );

        let pointer = context
            .functions_resolver
            .borrow_mut()
            .add_function(Box::new(callable))?;

        Ok((pointer, function_ident_string))
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
        let super_available = context.resolver.borrow().is_super_available();
        let (pointer, function_ident_string) =
            Self::declare_function(&node.function, context, false, super_available)?;
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
        let methods = node
            .functions
            .iter()
            .map(|(_, function)| {
                Self::declare_function(
                    function,
                    context,
                    true,
                    node.super_class.super_class.is_some(),
                )
            })
            .collect::<Result<Vec<(usize, String)>>>()?;
        context.resolver.borrow_mut().exit_class();

        if let Some(super_class) = &node.super_class.super_class {
            if super_class.token.lexeme == class_ident_string {
                return Err(ResolverError::InheritFromItself(
                    super_class.token.lexeme.clone(),
                    super_class.token.line,
                )
                .into());
            }
        }

        let super_class = node
            .super_class
            .super_class
            .as_ref()
            .map(|super_class| context.get_class_definition(&super_class.token.lexeme))
            .flatten();
        if super_class.is_none() && node.super_class.super_class.is_some() {
            let token_ref = &node.super_class.super_class.as_ref().unwrap().token;
            let primary_evaluator =
                PrimaryEvaluator::from_raw_token(token_ref, &context.resolver.borrow())?;
            return Ok(Box::new(IfNoClassErrorRunnable::new(
                primary_evaluator,
                token_ref.line,
            )));
        }

        let callable =
            ClassInitializationCallable::new(class_ident_string.clone(), methods, super_class);

        let pointer = context
            .functions_resolver
            .borrow_mut()
            .add_function(Box::new(callable))?;

        context.set_class_definition(&class_ident_string, pointer);

        let super_class_option = node
            .super_class
            .super_class
            .as_ref()
            .map::<Result<(PrimaryEvaluator, usize)>, _>(|ident| {
                let evaluator =
                    PrimaryEvaluator::from_raw_token(&ident.token, &context.resolver.borrow())?;
                Ok((evaluator, ident.token.line))
            })
            .transpose()?;

        Ok(Box::new(ClassDeclarationRunnable::new(
            pointer,
            class_ident_string,
            super_class_option,
        )))
    }
}
