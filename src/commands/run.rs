use super::{Command, CommandUtils};
use crate::error::Result;
use crate::evaluation::Evaluator;
use crate::syntax_analysis::Program;

pub struct RunCommand;

impl Command for RunCommand {
    fn run(&self, filename: &str) -> Result<()> {
        CommandUtils::log_debug("Logs from your program will appear here!");

        let file_contents = CommandUtils::read_file(filename)?;
        let tokens = CommandUtils::scan_tokens_checked(&file_contents)?;

        let mut parse_stream = CommandUtils::create_parse_stream(tokens);

        match parse_stream.parse::<Program>() {
            Ok(program) => {
                let evaluator = Evaluator::new();
                match evaluator.run_program(&program) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.into()),
                }
            }
            Err(e) => Err(e.into()),
        }
    }
}
