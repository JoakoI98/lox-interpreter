// This file previously contained run() method implementations on statement AST types.
// All execution logic has been moved to the centralized Evaluator in evaluator.rs
// for better separation of concerns and improved testability.

// The imports below are kept for potential future use or backwards compatibility
use super::runtime_value::{Result, RuntimeError, RuntimeValue};
use crate::syntax_analysis::{PrintStatement, Program, Statement, StatementType};

// All execution logic has been moved to src/evaluation/evaluator.rs
// Use the Evaluator struct instead of calling .run() on AST nodes directly.
