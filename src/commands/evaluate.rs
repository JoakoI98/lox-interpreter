use super::{Command, CommandUtils};
use crate::common::Visitable;
use crate::error::Result;
use crate::evaluation::{AssignmentEvaluatorBuilder, RunState, RuntimeValue};
use crate::syntax_analysis::Expression;

pub struct EvaluateCommand;

impl EvaluateCommand {
    fn evaluate_expression(&self, expression: &Expression) -> Result<RuntimeValue> {
        let evaluator = expression.accept(&AssignmentEvaluatorBuilder)?;
        Ok(evaluator.eval(&mut RunState::default())?)
    }
}

impl Command for EvaluateCommand {
    fn run(&self, filename: &str) -> Result<()> {
        CommandUtils::log_debug("Logs from your program will appear here!");

        let file_contents = CommandUtils::read_file(filename)?;
        let tokens = CommandUtils::scan_tokens_checked(&file_contents)?;

        let mut parse_stream = CommandUtils::create_parse_stream(tokens);

        match parse_stream.parse::<Expression>() {
            Ok(expression) => match self.evaluate_expression(&expression) {
                Ok(result) => {
                    println!("{}", result);
                    Ok(())
                }
                Err(e) => Err(e.into()),
            },
            Err(e) => Err(e.into()),
        }
    }
}
