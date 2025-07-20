use std::fmt::{Debug, Display};

use super::expression::LogicalOr;

use super::super::parsing::{ParseStream, Parser, Result};

use crate::syntax_analysis::parsing::primitives::{Equal, Identifier, Semicolon};
use crate::syntax_analysis::{AccessorOrArgumentsType, Call, ParseError};
use crate::tokenizer::TokenEnum;

#[derive(Debug)]
enum AssignmentPreParse {
    NoAssignment,
    Identifier,
    SetExpression(Call, Identifier),
    Error(ParseError),
}
#[derive(Debug, PartialEq, Clone)]
pub enum Assignment {
    Assignment(Box<Assignment>, Identifier),
    SetExpression(Box<Assignment>, Identifier, Call),
    Evaluable(LogicalOr),
}
impl crate::common::Visitable for Assignment {}
impl Parser for Assignment {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let pre_parse = input.run_and_restore(|input| {
            let call = input.parse::<Call>();
            if call.is_err() || !input.peek::<Equal>() {
                return (AssignmentPreParse::NoAssignment, false);
            }

            let equal = input
                .parse::<Equal>()
                .map_err(|e| AssignmentPreParse::Error(e));

            if let Err(e) = equal {
                return (e, false);
            }

            let call = call.unwrap();
            if call.accessor_or_arguments.len() == 0 {
                return (AssignmentPreParse::Identifier, false);
            }

            let (_, last) = call.accessor_or_arguments.last().unwrap();
            let identifier = match &last.ty {
                AccessorOrArgumentsType::Accessor(ac) => ac.identifier.clone(),
                _ => {
                    let err = input.parse::<Semicolon>().err().unwrap();
                    return (AssignmentPreParse::Error(err), false);
                }
            };

            let mut without_last: Vec<_> = Vec::new();
            without_last.reserve_exact(call.accessor_or_arguments.len() - 1);
            for i in 0..call.accessor_or_arguments.len() - 1 {
                without_last.push(call.accessor_or_arguments[i].clone());
            }

            let new_call = Call {
                token_type: call.token_type,
                primary: call.primary,
                accessor_or_arguments: without_last,
            };

            return (
                AssignmentPreParse::SetExpression(new_call, identifier),
                true,
            );
        });

        match pre_parse {
            AssignmentPreParse::NoAssignment => {
                let evaluable = input.parse::<LogicalOr>()?;
                Ok(Assignment::Evaluable(evaluable))
            }
            AssignmentPreParse::Identifier => {
                let identifier = input.parse::<Identifier>()?;
                input.parse::<Equal>()?;
                let assignment = input.parse::<Assignment>()?;
                Ok(Assignment::Assignment(Box::new(assignment), identifier))
            }
            AssignmentPreParse::SetExpression(call, identifier) => {
                let assignment = input.parse::<Assignment>()?;
                Ok(Assignment::SetExpression(
                    Box::new(assignment),
                    identifier,
                    call,
                ))
            }
            AssignmentPreParse::Error(err) => Err(err),
        }
    }

    fn peek(input: &ParseStream) -> bool {
        input
            .peek1()
            .map(|token| token.token_type == TokenEnum::Identifier)
            .unwrap_or(false)
            || input.peek::<LogicalOr>()
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Assignment::Assignment(assignment, identifier) => {
                write!(f, "{} = {}", identifier, assignment)
            }
            Assignment::SetExpression(assignment, identifier, call) => {
                write!(f, "{}.{} = {}", call, identifier, assignment)
            }
            Assignment::Evaluable(evaluable) => write!(f, "{}", evaluable),
        }
    }
}

pub type Expression = Box<Assignment>;

impl Parser for Expression {
    fn parse(input: &mut ParseStream) -> Result<Self> {
        let assignment = input.parse::<Assignment>()?;
        Ok(Box::new(assignment))
    }

    fn peek(input: &ParseStream) -> bool {
        input.peek::<Assignment>()
    }
}
