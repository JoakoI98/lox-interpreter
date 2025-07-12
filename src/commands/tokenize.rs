use super::{Command, CommandUtils};
use crate::error::Result;

pub struct TokenizeCommand;

impl Command for TokenizeCommand {
    fn run(&self, filename: &str) -> Result<()> {
        CommandUtils::log_debug("Logs from your program will appear here!");

        let file_contents = CommandUtils::read_file(filename)?;
        let (tokens, errors) = CommandUtils::scan_tokens(&file_contents);

        for error in &errors {
            eprintln!("{}", error);
        }
        for token in &tokens {
            println!("{:?}", token);
        }

        if !errors.is_empty() {
            // Return the first scanner error (they all have the same exit code anyway)
            return Err(errors.into_iter().next().unwrap().into());
        }

        Ok(())
    }
}
