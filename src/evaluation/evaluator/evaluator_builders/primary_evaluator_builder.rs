use super::super::super::runtime_value::{Result, RuntimeError};
use super::super::super::BuilderContext;
use super::super::evaluator::{BinaryEvaluator, BinaryOperation, UnaryEvaluator, UnaryOperation};
use super::super::evaluator::{Evaluable, PrimaryEvaluator};
use crate::common::{Visitable, VisitorWithContext};
use crate::evaluation::evaluator::evaluator_builders::assignment_evaluator_builder::AssignmentEvaluatorBuilder;
use crate::syntax_analysis::{
    Comparison, ComparisonType, Equality, EqualityType, Factor, FactorType, Term, TermType,
    UnaryExpression, UnaryExpressionSelf, UnaryExpressionSelfType, UnaryExpressionType,
};
use crate::tokenizer::TokenValue;
use crate::{
    common::Visitor,
    syntax_analysis::{PrimaryExpression, PrimaryExpressionType},
};

pub struct PrimaryEvaluatorBuilder;

impl VisitorWithContext<&PrimaryExpression, Result<Box<dyn Evaluable>>, BuilderContext>
    for PrimaryEvaluatorBuilder
{
    fn visit_with_context(
        &self,
        node: &PrimaryExpression,
        context: &BuilderContext,
    ) -> Result<Box<dyn Evaluable>> {
        let token = node
            .token_list
            .first()
            .ok_or(RuntimeError::ASTInvalidStructure)?;
        match &node.token_type {
            PrimaryExpressionType::True => Ok(Box::new(PrimaryEvaluator::Boolean(true))),
            PrimaryExpressionType::False => Ok(Box::new(PrimaryEvaluator::Boolean(false))),
            PrimaryExpressionType::Nil => Ok(Box::new(PrimaryEvaluator::Nil)),
            PrimaryExpressionType::Number
            | PrimaryExpressionType::String
            | PrimaryExpressionType::Identifier => match &token.token_value {
                TokenValue::Number(value) => Ok(Box::new(PrimaryEvaluator::Number(value.clone()))),
                TokenValue::String(value) => Ok(Box::new(PrimaryEvaluator::String(value.clone()))),
                TokenValue::Identifier(_) => {
                    // Here we could use the resolver to resolve the identifier
                    // For now, just create the evaluator
                    Ok(Box::new(PrimaryEvaluator::from_raw_token(token)?))
                }
                _ => Err(RuntimeError::ASTInvalidStructure),
            },
            PrimaryExpressionType::Expression(expr) => {
                expr.accept_with_context(&AssignmentEvaluatorBuilder, context)
            }
            PrimaryExpressionType::None => Err(RuntimeError::ASTInvalidStructure),
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
            UnaryExpressionType::PrimaryExpression(expr) => {
                expr.accept_with_context(&PrimaryEvaluatorBuilder, context)
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
