use super::evaluator::{Evaluable, PrimaryEvaluator};
use super::runtime_value::{Result, RuntimeError};
use crate::common::Visitable;
use crate::evaluation::evaluator::{
    BinaryEvaluator, BinaryOperation, UnaryEvaluator, UnaryOperation,
};
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
            PrimaryExpressionType::Number | PrimaryExpressionType::String => {
                match &token.token_value {
                    TokenValue::Number(value) => {
                        Ok(Box::new(PrimaryEvaluator::Number(value.clone())))
                    }
                    TokenValue::String(value) => {
                        Ok(Box::new(PrimaryEvaluator::String(value.clone())))
                    }
                    _ => Err(RuntimeError::ASTInvalidStructure),
                }
            }
            PrimaryExpressionType::Expression(expr) => expr.accept(&BinaryEvaluatorBuilder),
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
