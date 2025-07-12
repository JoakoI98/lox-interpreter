use super::{Command, CommandUtils};
use crate::error::Result;
use crate::syntax_analysis::Expression;

pub struct ParseCommand;

impl Command for ParseCommand {
    fn run(&self, filename: &str) -> Result<()> {
        CommandUtils::log_debug("Logs from your program will appear here!");

        let file_contents = CommandUtils::read_file(filename)?;
        let tokens = CommandUtils::scan_tokens_checked(&file_contents)?;

        let mut parse_stream = CommandUtils::create_parse_stream(tokens);

        match parse_stream.parse::<Expression>() {
            Ok(expression) => {
                println!("{}", expression);
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}
