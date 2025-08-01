use super::super::super::runtime_value::{Result, RuntimeError};
use super::super::super::BuilderContext;
use super::super::evaluator::{Evaluable, PrimaryEvaluator};
use super::super::evaluator::{UnaryEvaluator, UnaryOperation};
use crate::common::{Visitable, VisitorWithContext};
use crate::evaluation::evaluator::evaluator_builders::assignment_evaluator_builder::AssignmentEvaluatorBuilder;
use crate::evaluation::evaluator::evaluator_builders::function_call_evaluator_builder::FunctionCallEvaluatorBuilder;
use crate::evaluation::resolver::ResolverError;
use crate::syntax_analysis::{PrimaryExpression, PrimaryExpressionType};
use crate::syntax_analysis::{
    UnaryExpression, UnaryExpressionSelf, UnaryExpressionSelfType, UnaryExpressionType,
};
use crate::tokenizer::TokenValue;

pub struct PrimaryEvaluatorBuilder;

impl VisitorWithContext<&PrimaryExpression, Result<Box<dyn Evaluable>>, BuilderContext>
    for PrimaryEvaluatorBuilder
{
    fn visit_with_context(
        &self,
        node: &PrimaryExpression,
        context: &BuilderContext,
    ) -> Result<Box<dyn Evaluable>> {
        match node {
            PrimaryExpression::PrimaryWithoutSuperExpression(node) => {
                let token = node
                    .token_list
                    .first()
                    .ok_or(RuntimeError::ASTInvalidStructure)?;
                match &node.token_type {
                    PrimaryExpressionType::True => Ok(Box::new(PrimaryEvaluator::Boolean(true))),
                    PrimaryExpressionType::This => {
                        if !context.resolver.borrow().is_in_class() {
                            return Err(ResolverError::ThisOutsideClass(token.line).into());
                        }
                        Ok(Box::new(PrimaryEvaluator::This))
                    }
                    PrimaryExpressionType::False => Ok(Box::new(PrimaryEvaluator::Boolean(false))),
                    PrimaryExpressionType::Nil => Ok(Box::new(PrimaryEvaluator::Nil)),
                    PrimaryExpressionType::Number
                    | PrimaryExpressionType::String
                    | PrimaryExpressionType::Identifier => match &token.token_value {
                        TokenValue::Number(value) => {
                            Ok(Box::new(PrimaryEvaluator::Number(value.clone())))
                        }
                        TokenValue::String(value) => {
                            Ok(Box::new(PrimaryEvaluator::String(value.clone())))
                        }
                        TokenValue::Identifier(_) => Ok(Box::new(
                            PrimaryEvaluator::from_raw_token(token, &context.resolver.borrow())?,
                        )),
                        _ => Err(RuntimeError::ASTInvalidStructure),
                    },
                    PrimaryExpressionType::Expression(expr) => {
                        expr.accept_with_context(&AssignmentEvaluatorBuilder, context)
                    }
                    PrimaryExpressionType::None => Err(RuntimeError::ASTInvalidStructure),
                }
            }
            PrimaryExpression::Super(identifier) => {
                if !context.resolver.borrow().is_in_class() {
                    return Err(ResolverError::ThisOutsideClass(identifier.token.line).into());
                }
                if !context.resolver.borrow().is_super_available() {
                    return Err(ResolverError::SuperNotAvailable(identifier.token.line).into());
                }
                Ok(Box::new(PrimaryEvaluator::Super(identifier.token.clone())))
            }
        }
    }
}

pub struct UnaryEvaluatorBuilder;

impl VisitorWithContext<&UnaryExpression, Result<Box<dyn Evaluable>>, BuilderContext>
    for UnaryEvaluatorBuilder
{
    fn visit_with_context(
        &self,
        node: &UnaryExpression,
        context: &BuilderContext,
    ) -> Result<Box<dyn Evaluable>> {
        match &node.token_type {
            UnaryExpressionType::Call(expr) => {
                expr.accept_with_context(&FunctionCallEvaluatorBuilder, context)
            }
            UnaryExpressionType::UnaryExpressionSelf(expr) => {
                expr.accept_with_context(&UnaryEvaluatorBuilder, context)
            }
            UnaryExpressionType::None => Err(RuntimeError::ASTInvalidStructure),
        }
    }
}

impl VisitorWithContext<&UnaryExpressionSelf, Result<Box<dyn Evaluable>>, BuilderContext>
    for UnaryEvaluatorBuilder
{
    fn visit_with_context(
        &self,
        node: &UnaryExpressionSelf,
        context: &BuilderContext,
    ) -> Result<Box<dyn Evaluable>> {
        let evaluator = node
            .expr
            .accept_with_context(&UnaryEvaluatorBuilder, context)?;
        let operation = match &node.token_type {
            UnaryExpressionSelfType::Bang => UnaryOperation::Not,
            UnaryExpressionSelfType::Minus => UnaryOperation::Negation,
            UnaryExpressionSelfType::None => return Err(RuntimeError::ASTInvalidStructure),
        };
        Ok(Box::new(UnaryEvaluator::new(operation, evaluator)))
    }
}
