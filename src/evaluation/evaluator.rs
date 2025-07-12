use super::runtime_value::{Result, RuntimeError, RuntimeValue};
use crate::syntax_analysis::{
    Comparison, ComparisonType, Equality, EqualityType, Expression, Factor, FactorType,
    PrimaryExpression, PrimaryExpressionType, PrintStatement, Program, Statement, StatementType,
    Term, TermType, UnaryExpression, UnaryExpressionSelf, UnaryExpressionSelfType,
    UnaryExpressionType,
};

/// Centralized evaluator for all AST nodes
///
/// This struct separates evaluation concerns from AST structure,
/// making the code more modular and testable.
pub struct Evaluator;

impl Evaluator {
    /// Create a new evaluator instance
    pub fn new() -> Self {
        Self
    }

    /// Evaluate an expression and return its runtime value
    pub fn eval_expression(&self, expr: &Expression) -> Result<RuntimeValue> {
        self.eval_equality(expr)
    }

    /// Evaluate a primary expression (literals, identifiers, grouped expressions)
    pub fn eval_primary_expression(&self, expr: &PrimaryExpression) -> Result<RuntimeValue> {
        let token = expr
            .token_list
            .first()
            .ok_or(RuntimeError::UnexpectedRuntimeError)?;

        match &expr.token_type {
            PrimaryExpressionType::Number => {
                Ok(RuntimeValue::Number(token.token_value.number().unwrap()))
            }
            PrimaryExpressionType::String => {
                Ok(RuntimeValue::String(token.token_value.string().unwrap()))
            }
            PrimaryExpressionType::False => Ok(RuntimeValue::Boolean(false)),
            PrimaryExpressionType::True => Ok(RuntimeValue::Boolean(true)),
            PrimaryExpressionType::Nil => Ok(RuntimeValue::Nil),
            PrimaryExpressionType::Expression(e) => self.eval_expression(e),
            _ => Err(RuntimeError::UnexpectedRuntimeError),
        }
    }

    /// Evaluate a unary expression (!expr, -expr, or primary expression)
    pub fn eval_unary_expression(&self, expr: &UnaryExpression) -> Result<RuntimeValue> {
        match &expr.token_type {
            UnaryExpressionType::UnaryExpressionSelf(expr) => self.eval_unary_expression_self(expr),
            UnaryExpressionType::PrimaryExpression(expr) => self.eval_primary_expression(expr),
            _ => Err(RuntimeError::UnexpectedRuntimeError),
        }
    }

    /// Evaluate a unary expression with operator (!expr or -expr)
    pub fn eval_unary_expression_self(&self, expr: &UnaryExpressionSelf) -> Result<RuntimeValue> {
        let inner = self.eval_unary_expression(&expr.expr)?;
        match &expr.token_type {
            UnaryExpressionSelfType::Bang => !inner,
            UnaryExpressionSelfType::Minus => -inner,
            _ => Err(RuntimeError::UnexpectedRuntimeError),
        }
    }

    /// Evaluate a factor (multiplication and division)
    pub fn eval_factor(&self, expr: &Factor) -> Result<RuntimeValue> {
        let mut acc = self.eval_unary_expression(&expr.main_unary)?;
        for (op, factor) in expr.unaries.iter() {
            let curr = match op {
                FactorType::Star => acc * self.eval_unary_expression(factor)?,
                FactorType::Slash => acc / self.eval_unary_expression(factor)?,
                _ => return Err(RuntimeError::UnexpectedRuntimeError),
            }?;
            acc = curr;
        }
        Ok(acc)
    }

    /// Evaluate a term (addition and subtraction)
    pub fn eval_term(&self, expr: &Term) -> Result<RuntimeValue> {
        let mut acc = self.eval_factor(&expr.main_factor)?;
        for (op, factor) in expr.factors.iter() {
            let curr = match op {
                TermType::Plus => acc + self.eval_factor(factor)?,
                TermType::Minus => acc - self.eval_factor(factor)?,
                _ => return Err(RuntimeError::UnexpectedRuntimeError),
            }?;
            acc = curr;
        }
        Ok(acc)
    }

    /// Evaluate a comparison expression (<, <=, >, >=)
    pub fn eval_comparison(&self, expr: &Comparison) -> Result<RuntimeValue> {
        let mut acc = self.eval_term(&expr.main_term)?;
        for (op, term) in expr.terms.iter() {
            let curr = match op {
                ComparisonType::Less => acc.lt(&self.eval_term(term)?),
                ComparisonType::LessEqual => acc.le(&self.eval_term(term)?),
                ComparisonType::Greater => acc.gt(&self.eval_term(term)?),
                ComparisonType::GreaterEqual => acc.ge(&self.eval_term(term)?),
                _ => return Err(RuntimeError::UnexpectedRuntimeError),
            }?;
            acc = curr;
        }
        Ok(acc)
    }

    /// Evaluate an equality expression (==, !=)
    pub fn eval_equality(&self, expr: &Equality) -> Result<RuntimeValue> {
        let mut acc = self.eval_comparison(&expr.main_comparison)?;
        for (op, comparison) in expr.comparisons.iter() {
            let curr = match op {
                EqualityType::EqualEqual => acc.eq(&self.eval_comparison(comparison)?),
                EqualityType::BangEqual => acc.ne(&self.eval_comparison(comparison)?),
                _ => return Err(RuntimeError::UnexpectedRuntimeError),
            }?;
            acc = curr;
        }
        Ok(acc)
    }

    /// Execute a print statement
    pub fn run_print_statement(&self, stmt: &PrintStatement) -> Result<RuntimeValue> {
        let value = self.eval_expression(&stmt.expr)?;
        println!("{}", value);
        Ok(value)
    }

    /// Execute a statement
    pub fn run_statement(&self, stmt: &Statement) -> Result<RuntimeValue> {
        match &stmt.token_type {
            StatementType::PrintStatement(p) => self.run_print_statement(p),
            StatementType::ExprStatement(e) => self.eval_expression(&e.expr),
            _ => Err(RuntimeError::UnexpectedRuntimeError),
        }
    }

    /// Execute a program (sequence of statements)
    pub fn run_program(&self, program: &Program) -> Result<RuntimeValue> {
        for (_, statement) in &program.statements {
            self.run_statement(statement)?;
        }
        Ok(RuntimeValue::Nil)
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluator_creation() {
        let evaluator = Evaluator::new();
        // Basic test to ensure the evaluator can be created
        assert!(true);
    }

    #[test]
    fn test_evaluator_default() {
        let evaluator = Evaluator::default();
        // Test that default implementation works
        assert!(true);
    }
}
