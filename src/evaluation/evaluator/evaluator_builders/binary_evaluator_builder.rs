use super::super::Evaluable;
use crate::common::{Visitable, VisitorWithContext};
use crate::evaluation::evaluator::evaluator::{BinaryEvaluator, BinaryOperation};
use crate::evaluation::evaluator::evaluator_builders::primary_evaluator_builder::UnaryEvaluatorBuilder;
use crate::evaluation::runtime_value::Result;
use crate::evaluation::{BuilderContext, RuntimeError};
use crate::syntax_analysis::{
    Comparison, ComparisonType, Equality, EqualityType, Factor, FactorType, LogicalAnd, LogicalOr,
    Term, TermType,
};

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

impl VisitorWithContext<&LogicalAnd, Result<Box<dyn Evaluable>>, BuilderContext>
    for BinaryEvaluatorBuilder
{
    fn visit_with_context(
        &self,
        node: &LogicalAnd,
        context: &BuilderContext,
    ) -> Result<Box<dyn Evaluable>> {
        let mut main_evaluator = node
            .main_equality
            .accept_with_context(&BinaryEvaluatorBuilder, context)?;
        for (_, equality) in &node.equalities {
            let op = BinaryOperation::LogicalAnd;
            let right = equality.accept_with_context(&BinaryEvaluatorBuilder, context)?;
            main_evaluator = Box::new(BinaryEvaluator::new(op, main_evaluator, right));
        }
        Ok(main_evaluator)
    }
}

impl VisitorWithContext<&LogicalOr, Result<Box<dyn Evaluable>>, BuilderContext>
    for BinaryEvaluatorBuilder
{
    fn visit_with_context(
        &self,
        node: &LogicalOr,
        context: &BuilderContext,
    ) -> Result<Box<dyn Evaluable>> {
        let mut main_evaluator = node
            .main_and
            .accept_with_context(&BinaryEvaluatorBuilder, context)?;
        for (_, and) in &node.ands {
            let op = BinaryOperation::LogicalOr;
            let right = and.accept_with_context(&BinaryEvaluatorBuilder, context)?;
            main_evaluator = Box::new(BinaryEvaluator::new(op, main_evaluator, right));
        }
        Ok(main_evaluator)
    }
}
