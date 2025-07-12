// This file previously contained eval() method implementations on AST types.
// All evaluation logic has been moved to the centralized Evaluator in evaluator.rs
// for better separation of concerns and improved testability.

// The imports below are kept for potential future use or backwards compatibility
use crate::syntax_analysis::{
    Comparison, ComparisonType, Equality, EqualityType, Factor, FactorType, PrimaryExpression,
    PrimaryExpressionType, Term, TermType, UnaryExpression, UnaryExpressionSelf,
    UnaryExpressionSelfType, UnaryExpressionType,
};

pub use super::runtime_value::{Result, RuntimeError, RuntimeValue};

// All evaluation logic has been moved to src/evaluation/evaluator.rs
// Use the Evaluator struct instead of calling .eval() on AST nodes directly.
