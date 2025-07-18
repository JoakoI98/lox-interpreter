use super::{Command, CommandUtils};
use crate::{
    error::Result as CommandResult,
    evaluation::{Program, RuntimeError},
    syntax_analysis::ProgramAst,
};

pub struct RunCommand;

impl RunCommand {
    fn run_program(program_ast: ProgramAst) -> Result<(), RuntimeError> {
        let mut program = Program::new_with_context(program_ast)?;
        program.run()?;
        Ok(())
    }
}

impl Command for RunCommand {
    fn run(&self, filename: &str) -> CommandResult<()> {
        CommandUtils::log_debug("Logs from your program will appear here!");

        let file_contents = CommandUtils::read_file(filename)?;
        let tokens = CommandUtils::scan_tokens_checked(&file_contents)?;

        let mut parse_stream = CommandUtils::create_parse_stream(tokens);

        match parse_stream.parse::<ProgramAst>() {
            Ok(program) => Self::run_program(program).map_err(|e| {
                eprintln!("{}", e);
                e.into()
            }),
            Err(e) => Err(e.into()),
        }
    }
}
