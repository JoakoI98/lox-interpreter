use super::super::Evaluable;
use crate::common::{Visitable, VisitorWithContext};
use crate::evaluation::evaluator::evaluator::AssignmentEvaluator;
use crate::evaluation::evaluator::evaluator_builders::binary_evaluator_builder::BinaryEvaluatorBuilder;
use crate::evaluation::evaluator::EvaluableIdentifier;
use crate::evaluation::runtime_value::Result;
use crate::evaluation::{BuilderContext, RuntimeError};
use crate::syntax_analysis::Assignment;
use crate::syntax_analysis::AssignmentSelf;

pub struct AssignmentEvaluatorBuilder;

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
