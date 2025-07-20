use super::super::Evaluable;
use crate::common::{Visitable, VisitorWithContext};
use crate::evaluation::evaluator::evaluator::AssignmentEvaluator;
use crate::evaluation::evaluator::evaluator_builders::binary_evaluator_builder::BinaryEvaluatorBuilder;
use crate::evaluation::evaluator::evaluator_builders::function_call_evaluator_builder::FunctionCallEvaluatorBuilder;
use crate::evaluation::evaluator::{EvaluableIdentifier, SetExpressionEvaluator};
use crate::evaluation::runtime_value::Result;
use crate::evaluation::BuilderContext;
use crate::syntax_analysis::{Assignment, Call};
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

    fn build_set_expression_evaluator(
        assignment: &Assignment,
        call: &Call,
        ident_token: &Token,
        context: &BuilderContext,
    ) -> Result<Box<dyn Evaluable>> {
        let ident_evaluator = ident_token.lexeme.clone();
        let value_evaluator =
            assignment.accept_with_context(&AssignmentEvaluatorBuilder, context)?;
        let to_access_evaluator =
            call.accept_with_context(&FunctionCallEvaluatorBuilder, context)?;
        Ok(Box::new(SetExpressionEvaluator::new(
            to_access_evaluator,
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
            Assignment::SetExpression(assignment, identifier, call) => {
                Self::build_set_expression_evaluator(assignment, &call, &identifier.token, context)
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
