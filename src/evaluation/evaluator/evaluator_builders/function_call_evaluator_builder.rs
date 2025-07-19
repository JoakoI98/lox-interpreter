use super::super::super::runtime_value::Result;
use super::super::super::BuilderContext;
use super::super::evaluator::Evaluable;
use super::primary_evaluator_builder::PrimaryEvaluatorBuilder;
use crate::common::{Visitable, VisitorWithContext};
use crate::evaluation::evaluator::FunctionEvaluator;
use crate::evaluation::AssignmentEvaluatorBuilder;
use crate::syntax_analysis::{ArgumentsList, Call};

pub struct FunctionCallEvaluatorBuilder;

impl VisitorWithContext<&ArgumentsList, Result<Vec<Box<dyn Evaluable>>>, BuilderContext>
    for FunctionCallEvaluatorBuilder
{
    fn visit_with_context(
        &self,
        node: &ArgumentsList,
        context: &BuilderContext,
    ) -> Result<Vec<Box<dyn Evaluable>>> {
        if node.maybe_arguments.is_none() {
            return Ok(vec![]);
        }

        let arguments = node.maybe_arguments.as_ref().unwrap();
        let mut arguments_evaluator: Vec<_> = Vec::new();
        arguments_evaluator.reserve_exact(arguments.rest.len() + 1);
        arguments_evaluator.push(
            arguments
                .first
                .accept_with_context(&AssignmentEvaluatorBuilder, context)?,
        );
        for (_, expression) in arguments.rest.iter() {
            arguments_evaluator
                .push(expression.accept_with_context(&AssignmentEvaluatorBuilder, context)?);
        }
        Ok(arguments_evaluator)
    }
}

impl VisitorWithContext<&Call, Result<Box<dyn Evaluable>>, BuilderContext>
    for FunctionCallEvaluatorBuilder
{
    fn visit_with_context(
        &self,
        node: &Call,
        context: &BuilderContext,
    ) -> Result<Box<dyn Evaluable>> {
        let primary_evaluator = node
            .primary
            .accept_with_context(&PrimaryEvaluatorBuilder, context)?;

        if node.arguments_list.len() == 0 {
            return Ok(primary_evaluator);
        }

        let (_, first_argument) = node.arguments_list.first().unwrap();

        let mut function_evaluator = FunctionEvaluator::new(
            primary_evaluator,
            first_argument.accept_with_context(&Self, context)?,
        );

        for (_, arguments_list) in node.arguments_list.iter().skip(1) {
            let old_evaluator = Box::new(function_evaluator);
            function_evaluator = FunctionEvaluator::new(
                old_evaluator,
                arguments_list.accept_with_context(&Self, context)?,
            );
        }

        Ok(Box::new(function_evaluator))
    }
}
