use super::super::super::runtime_value::Result;
use super::super::super::BuilderContext;
use super::super::evaluator::Evaluable;
use super::primary_evaluator_builder::PrimaryEvaluatorBuilder;
use crate::common::{Visitable, VisitorWithContext};
use crate::evaluation::evaluator::{ClassAccessorEvaluator, FunctionEvaluator};
use crate::evaluation::AssignmentEvaluatorBuilder;
use crate::syntax_analysis::{AccessorOrArgumentsType, ArgumentsList, Call};

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

        if node.accessor_or_arguments.len() == 0 {
            return Ok(primary_evaluator);
        }

        let (_, first_argument) = node.accessor_or_arguments.first().unwrap();

        let mut current_evaluator = match &first_argument.ty {
            AccessorOrArgumentsType::ArgumentsList(arguments_list) => {
                let function_evaluator = FunctionEvaluator::new(
                    primary_evaluator,
                    arguments_list.accept_with_context(&Self, context)?,
                );
                Box::new(function_evaluator) as Box<dyn Evaluable>
            }
            AccessorOrArgumentsType::Accessor(accessor) => {
                let class_accessor_evaluator = ClassAccessorEvaluator::new(
                    primary_evaluator,
                    accessor.identifier.token.lexeme.clone(),
                );
                Box::new(class_accessor_evaluator) as Box<dyn Evaluable>
            }
            _ => unreachable!(),
        };

        for (_, arguments_list) in node.accessor_or_arguments.iter().skip(1) {
            current_evaluator = match &arguments_list.ty {
                AccessorOrArgumentsType::ArgumentsList(arguments_list) => {
                    let function_evaluator = FunctionEvaluator::new(
                        current_evaluator,
                        arguments_list.accept_with_context(&Self, context)?,
                    );
                    Box::new(function_evaluator) as Box<dyn Evaluable>
                }
                AccessorOrArgumentsType::Accessor(accessor) => {
                    let class_accessor_evaluator = ClassAccessorEvaluator::new(
                        current_evaluator,
                        accessor.identifier.token.lexeme.clone(),
                    );
                    Box::new(class_accessor_evaluator) as Box<dyn Evaluable>
                }
                _ => unreachable!(),
            };
        }

        Ok(current_evaluator)
    }
}
