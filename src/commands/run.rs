use super::{Command, CommandUtils};
use crate::{error::Result, evaluation::Program, syntax_analysis::ProgramAst};

pub struct RunCommand;

impl RunCommand {
    fn run_program(program_ast: ProgramAst) -> Result<()> {
        let mut program = Program::new(program_ast)?;
        let result = program.run();
        if let Err(e) = &result {
            eprintln!("{}", e);
        }
        result.map_err(|e| e.into())
    }
}

impl Command for RunCommand {
    fn run(&self, filename: &str) -> Result<()> {
        CommandUtils::log_debug("Logs from your program will appear here!");

        let file_contents = CommandUtils::read_file(filename)?;
        let tokens = CommandUtils::scan_tokens_checked(&file_contents)?;

        let mut parse_stream = CommandUtils::create_parse_stream(tokens);

        match parse_stream.parse::<ProgramAst>() {
            Ok(program) => Self::run_program(program),
            Err(e) => Err(e.into()),
        }
    }
}
