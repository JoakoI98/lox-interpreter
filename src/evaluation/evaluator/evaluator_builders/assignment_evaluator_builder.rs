use super::super::Evaluable;
use crate::common::{Visitable, VisitorWithContext};
use crate::evaluation::evaluator::evaluator::AssignmentEvaluator;
use crate::evaluation::evaluator::evaluator_builders::binary_evaluator_builder::BinaryEvaluatorBuilder;
use crate::evaluation::evaluator::EvaluableIdentifier;
use crate::evaluation::runtime_value::Result;
use crate::evaluation::BuilderContext;
use crate::syntax_analysis::Assignment;
use crate::tokenizer::Token;

pub struct AssignmentEvaluatorBuilder;

impl AssignmentEvaluatorBuilder {
    fn build_assignment_evaluator(
        assignment: &Assignment,
        ident_token: &Token,
        context: &BuilderContext,
    ) -> Result<Box<dyn Evaluable>> {
        let ident_evaluator =
            EvaluableIdentifier::from_raw_token(ident_token, &context.resolver.borrow())?;
        let value_evaluator =
            assignment.accept_with_context(&AssignmentEvaluatorBuilder, context)?;
        Ok(Box::new(AssignmentEvaluator::new(
            ident_evaluator,
            value_evaluator,
        )))
    }
}

impl VisitorWithContext<&Assignment, Result<Box<dyn Evaluable>>, BuilderContext>
    for AssignmentEvaluatorBuilder
{
    fn visit_with_context(
        &self,
        node: &Assignment,
        context: &BuilderContext,
    ) -> Result<Box<dyn Evaluable>> {
        match node {
            Assignment::Assignment(assignment, identifier) => {
                Self::build_assignment_evaluator(assignment, &identifier.token, context)
            }
            Assignment::Evaluable(evaluable) => {
                evaluable.accept_with_context(&BinaryEvaluatorBuilder, context)
            }
        }
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
