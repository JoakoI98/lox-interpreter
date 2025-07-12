use super::{Command, CommandResult, CommandUtils};

pub struct TokenizeCommand;

impl Command for TokenizeCommand {
    fn run(&self, filename: &str) -> CommandResult {
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
            return Err(65); // Scanner error exit code
        }

        Ok(())
    }
}
