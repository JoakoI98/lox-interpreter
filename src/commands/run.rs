use super::{Command, CommandResult, CommandUtils};
use crate::syntax_analysis::Program;

pub struct RunCommand;

impl Command for RunCommand {
    fn run(&self, filename: &str) -> CommandResult {
        CommandUtils::log_debug("Logs from your program will appear here!");

        let file_contents = CommandUtils::read_file(filename)?;
        let tokens = CommandUtils::scan_tokens_checked(&file_contents)?;

        let mut parse_stream = CommandUtils::create_parse_stream(tokens);

        match parse_stream.parse::<Program>() {
            Ok(program) => {
                match program.run() {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        eprintln!("{}", e);
                        Err(70) // Runtime error exit code
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                Err(65) // Parse error exit code
            }
        }
    }
}
