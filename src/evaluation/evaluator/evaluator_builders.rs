use super::super::runtime_value::{Result, RuntimeError};
use super::super::BuilderContext;
use super::evaluator::{BinaryEvaluator, BinaryOperation, UnaryEvaluator, UnaryOperation};
use super::evaluator::{Evaluable, PrimaryEvaluator};
use crate::common::{Visitable, VisitorWithContext};
use crate::evaluation::evaluator::evaluator::AssignmentEvaluator;
use crate::evaluation::evaluator::EvaluableIdentifier;
use crate::syntax_analysis::{
    Assignment, AssignmentSelf, Comparison, ComparisonType, Equality, EqualityType, Factor,
    FactorType, Term, TermType, UnaryExpression, UnaryExpressionSelf, UnaryExpressionSelfType,
    UnaryExpressionType,
};
use crate::tokenizer::TokenValue;
use crate::{
    common::Visitor,
    syntax_analysis::{PrimaryExpression, PrimaryExpressionType},
};

pub struct PrimaryEvaluatorBuilder;

impl Visitor<&PrimaryExpression, Result<Box<dyn Evaluable>>> for PrimaryEvaluatorBuilder {
    fn visit(&self, node: &PrimaryExpression) -> Result<Box<dyn Evaluable>> {
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
                TokenValue::Identifier(_) => Ok(Box::new(PrimaryEvaluator::from_raw_token(token)?)),
                _ => Err(RuntimeError::ASTInvalidStructure),
            },
            PrimaryExpressionType::Expression(expr) => expr.accept(&AssignmentEvaluatorBuilder),
            PrimaryExpressionType::None => Err(RuntimeError::ASTInvalidStructure),
        }
    }
}

// Context-aware version for PrimaryEvaluatorBuilder
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

impl Visitor<&UnaryExpression, Result<Box<dyn Evaluable>>> for UnaryEvaluatorBuilder {
    fn visit(&self, node: &UnaryExpression) -> Result<Box<dyn Evaluable>> {
        match &node.token_type {
            UnaryExpressionType::PrimaryExpression(expr) => expr.accept(&PrimaryEvaluatorBuilder),
            UnaryExpressionType::UnaryExpressionSelf(expr) => expr.accept(&UnaryEvaluatorBuilder),
            UnaryExpressionType::None => Err(RuntimeError::ASTInvalidStructure),
        }
    }
}

// Context-aware version for UnaryEvaluatorBuilder
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

impl Visitor<&UnaryExpressionSelf, Result<Box<dyn Evaluable>>> for UnaryEvaluatorBuilder {
    fn visit(&self, node: &UnaryExpressionSelf) -> Result<Box<dyn Evaluable>> {
        let evaluator = node.expr.accept(&UnaryEvaluatorBuilder)?;
        let operation = match &node.token_type {
            UnaryExpressionSelfType::Bang => UnaryOperation::Not,
            UnaryExpressionSelfType::Minus => UnaryOperation::Negation,
            UnaryExpressionSelfType::None => return Err(RuntimeError::ASTInvalidStructure),
        };
        Ok(Box::new(UnaryEvaluator::new(operation, evaluator)))
    }
}

// Context-aware version for UnaryExpressionSelf
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

pub struct BinaryEvaluatorBuilder;

enum BinaryASTOperation<'a> {
    Factor(&'a FactorType),
    Term(&'a TermType),
    Comparison(&'a ComparisonType),
    Equality(&'a EqualityType),
}

impl BinaryEvaluatorBuilder {
    fn from_ast_type_to_evaluator_type(op_type: BinaryASTOperation) -> Result<BinaryOperation> {
        match op_type {
            BinaryASTOperation::Factor(FactorType::Star) => Ok(BinaryOperation::Multiplication),
            BinaryASTOperation::Factor(FactorType::Slash) => Ok(BinaryOperation::Division),
            BinaryASTOperation::Term(TermType::Plus) => Ok(BinaryOperation::Addition),
            BinaryASTOperation::Term(TermType::Minus) => Ok(BinaryOperation::Subtraction),
            BinaryASTOperation::Comparison(ComparisonType::Less) => Ok(BinaryOperation::LessThan),
            BinaryASTOperation::Comparison(ComparisonType::Greater) => {
                Ok(BinaryOperation::GreaterThan)
            }
            BinaryASTOperation::Comparison(ComparisonType::LessEqual) => {
                Ok(BinaryOperation::LessThanOrEqual)
            }
            BinaryASTOperation::Comparison(ComparisonType::GreaterEqual) => {
                Ok(BinaryOperation::GreaterThanOrEqual)
            }
            BinaryASTOperation::Equality(EqualityType::EqualEqual) => Ok(BinaryOperation::Equal),
            BinaryASTOperation::Equality(EqualityType::BangEqual) => Ok(BinaryOperation::NotEqual),
            _ => Err(RuntimeError::ASTInvalidStructure),
        }
    }
}

impl Visitor<&Factor, Result<Box<dyn Evaluable>>> for BinaryEvaluatorBuilder {
    fn visit(&self, node: &Factor) -> Result<Box<dyn Evaluable>> {
        let mut main_evaluator = node.main_unary.accept(&UnaryEvaluatorBuilder)?;
        for (op_type, factor) in &node.unaries {
            let op = Self::from_ast_type_to_evaluator_type(BinaryASTOperation::Factor(op_type))?;
            let right = factor.accept(&UnaryEvaluatorBuilder)?;
            main_evaluator = Box::new(BinaryEvaluator::new(op, main_evaluator, right));
        }
        Ok(main_evaluator)
    }
}

// Context-aware version for Factor
impl VisitorWithContext<&Factor, Result<Box<dyn Evaluable>>, BuilderContext>
    for BinaryEvaluatorBuilder
{
    fn visit_with_context(
        &self,
        node: &Factor,
        context: &BuilderContext,
    ) -> Result<Box<dyn Evaluable>> {
        let mut main_evaluator = node
            .main_unary
            .accept_with_context(&UnaryEvaluatorBuilder, context)?;
        for (op_type, factor) in &node.unaries {
            let op = Self::from_ast_type_to_evaluator_type(BinaryASTOperation::Factor(op_type))?;
            let right = factor.accept_with_context(&UnaryEvaluatorBuilder, context)?;
            main_evaluator = Box::new(BinaryEvaluator::new(op, main_evaluator, right));
        }
        Ok(main_evaluator)
    }
}

impl Visitor<&Term, Result<Box<dyn Evaluable>>> for BinaryEvaluatorBuilder {
    fn visit(&self, node: &Term) -> Result<Box<dyn Evaluable>> {
        let mut main_evaluator = node.main_factor.accept(&BinaryEvaluatorBuilder)?;
        for (op_type, term) in &node.factors {
            let op = Self::from_ast_type_to_evaluator_type(BinaryASTOperation::Term(op_type))?;
            let right = term.accept(&BinaryEvaluatorBuilder)?;
            main_evaluator = Box::new(BinaryEvaluator::new(op, main_evaluator, right));
        }
        Ok(main_evaluator)
    }
}

// Context-aware version for Term
impl VisitorWithContext<&Term, Result<Box<dyn Evaluable>>, BuilderContext>
    for BinaryEvaluatorBuilder
{
    fn visit_with_context(
        &self,
        node: &Term,
        context: &BuilderContext,
    ) -> Result<Box<dyn Evaluable>> {
        let mut main_evaluator = node
            .main_factor
            .accept_with_context(&BinaryEvaluatorBuilder, context)?;
        for (op_type, term) in &node.factors {
            let op = Self::from_ast_type_to_evaluator_type(BinaryASTOperation::Term(op_type))?;
            let right = term.accept_with_context(&BinaryEvaluatorBuilder, context)?;
            main_evaluator = Box::new(BinaryEvaluator::new(op, main_evaluator, right));
        }
        Ok(main_evaluator)
    }
}

impl Visitor<&Comparison, Result<Box<dyn Evaluable>>> for BinaryEvaluatorBuilder {
    fn visit(&self, node: &Comparison) -> Result<Box<dyn Evaluable>> {
        let mut main_evaluator = node.main_term.accept(&BinaryEvaluatorBuilder)?;
        for (op_type, comparison) in &node.terms {
            let op =
                Self::from_ast_type_to_evaluator_type(BinaryASTOperation::Comparison(op_type))?;
            let right = comparison.accept(&BinaryEvaluatorBuilder)?;
            main_evaluator = Box::new(BinaryEvaluator::new(op, main_evaluator, right));
        }
        Ok(main_evaluator)
    }
}

// Context-aware version for Comparison
impl VisitorWithContext<&Comparison, Result<Box<dyn Evaluable>>, BuilderContext>
    for BinaryEvaluatorBuilder
{
    fn visit_with_context(
        &self,
        node: &Comparison,
        context: &BuilderContext,
    ) -> Result<Box<dyn Evaluable>> {
        let mut main_evaluator = node
            .main_term
            .accept_with_context(&BinaryEvaluatorBuilder, context)?;
        for (op_type, comparison) in &node.terms {
            let op =
                Self::from_ast_type_to_evaluator_type(BinaryASTOperation::Comparison(op_type))?;
            let right = comparison.accept_with_context(&BinaryEvaluatorBuilder, context)?;
            main_evaluator = Box::new(BinaryEvaluator::new(op, main_evaluator, right));
        }
        Ok(main_evaluator)
    }
}

impl Visitor<&Equality, Result<Box<dyn Evaluable>>> for BinaryEvaluatorBuilder {
    fn visit(&self, node: &Equality) -> Result<Box<dyn Evaluable>> {
        let mut main_evaluator = node.main_comparison.accept(&BinaryEvaluatorBuilder)?;
        for (op_type, equality) in &node.comparisons {
            let op = Self::from_ast_type_to_evaluator_type(BinaryASTOperation::Equality(op_type))?;
            let right = equality.accept(&BinaryEvaluatorBuilder)?;
            main_evaluator = Box::new(BinaryEvaluator::new(op, main_evaluator, right));
        }
        Ok(main_evaluator)
    }
}

// Context-aware version for Equality
impl VisitorWithContext<&Equality, Result<Box<dyn Evaluable>>, BuilderContext>
    for BinaryEvaluatorBuilder
{
    fn visit_with_context(
        &self,
        node: &Equality,
        context: &BuilderContext,
    ) -> Result<Box<dyn Evaluable>> {
        let mut main_evaluator = node
            .main_comparison
            .accept_with_context(&BinaryEvaluatorBuilder, context)?;
        for (op_type, equality) in &node.comparisons {
            let op = Self::from_ast_type_to_evaluator_type(BinaryASTOperation::Equality(op_type))?;
            let right = equality.accept_with_context(&BinaryEvaluatorBuilder, context)?;
            main_evaluator = Box::new(BinaryEvaluator::new(op, main_evaluator, right));
        }
        Ok(main_evaluator)
    }
}

pub struct AssignmentEvaluatorBuilder;

impl Visitor<&AssignmentSelf, Result<Box<dyn Evaluable>>> for AssignmentEvaluatorBuilder {
    fn visit(&self, node: &AssignmentSelf) -> Result<Box<dyn Evaluable>> {
        let ident_token = node
            .token_list
            .first()
            .ok_or(RuntimeError::ASTInvalidStructure)?;
        let ident_evaluator = EvaluableIdentifier::from_raw_token(ident_token)?;
        let value_evaluator = node.assignment.accept(&AssignmentEvaluatorBuilder)?;
        Ok(Box::new(AssignmentEvaluator::new(
            ident_evaluator,
            value_evaluator,
        )))
    }
}

// Context-aware version for AssignmentSelf
impl VisitorWithContext<&AssignmentSelf, Result<Box<dyn Evaluable>>, BuilderContext>
    for AssignmentEvaluatorBuilder
{
    fn visit_with_context(
        &self,
        node: &AssignmentSelf,
        context: &BuilderContext,
    ) -> Result<Box<dyn Evaluable>> {
        let ident_token = node
            .token_list
            .first()
            .ok_or(RuntimeError::ASTInvalidStructure)?;
        let ident_evaluator = EvaluableIdentifier::from_raw_token(ident_token)?;
        let value_evaluator = node
            .assignment
            .accept_with_context(&AssignmentEvaluatorBuilder, context)?;
        Ok(Box::new(AssignmentEvaluator::new(
            ident_evaluator,
            value_evaluator,
        )))
    }
}

impl Visitor<&Assignment, Result<Box<dyn Evaluable>>> for AssignmentEvaluatorBuilder {
    fn visit(&self, node: &Assignment) -> Result<Box<dyn Evaluable>> {
        // match &node.token_type {
        //     AssignmentType::AssignmentSelf(expr) => expr.accept(&AssignmentEvaluatorBuilder),
        //     AssignmentType::Equality(expr) => expr.accept(&BinaryEvaluatorBuilder),
        //     _ => Err(RuntimeError::ASTInvalidStructure),
        // }
        node.eq.accept(&BinaryEvaluatorBuilder)
    }
}

// Original visitor implementation for Expression
impl Visitor<&crate::syntax_analysis::Expression, Result<Box<dyn Evaluable>>>
    for AssignmentEvaluatorBuilder
{
    fn visit(&self, node: &crate::syntax_analysis::Expression) -> Result<Box<dyn Evaluable>> {
        node.accept(&AssignmentEvaluatorBuilder)
    }
}

// Context-aware version for Assignment
impl VisitorWithContext<&Assignment, Result<Box<dyn Evaluable>>, BuilderContext>
    for AssignmentEvaluatorBuilder
{
    fn visit_with_context(
        &self,
        node: &Assignment,
        context: &BuilderContext,
    ) -> Result<Box<dyn Evaluable>> {
        node.eq
            .accept_with_context(&BinaryEvaluatorBuilder, context)
    }
}

// Context-aware version for Expression
impl
    VisitorWithContext<
        &crate::syntax_analysis::Expression,
        Result<Box<dyn Evaluable>>,
        BuilderContext,
    > for AssignmentEvaluatorBuilder
{
    fn visit_with_context(
        &self,
        node: &crate::syntax_analysis::Expression,
        context: &BuilderContext,
    ) -> Result<Box<dyn Evaluable>> {
        node.accept_with_context(&AssignmentEvaluatorBuilder, context)
    }
}
