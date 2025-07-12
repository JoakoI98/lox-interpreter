mod commands;
mod error;
mod evaluation;
mod syntax_analysis;
mod tokenizer;

use std::env;
use std::io::{self, Write};

use commands::{Command, EvaluateCommand, ParseCommand, RunCommand, TokenizeCommand};

fn main() {
    run();
}

fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} <command> <filename>", args[0]).unwrap();
        return;
    }

    let command_str = &args[1];
    let filename = &args[2];

    let command: Box<dyn Command> = match command_str.as_str() {
        "tokenize" => Box::new(TokenizeCommand),
        "parse" => Box::new(ParseCommand),
        "evaluate" => Box::new(EvaluateCommand),
        "run" => Box::new(RunCommand),
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command_str).unwrap();
            return;
        }
    };

    // Execute the command and handle the result
    match command.run(filename) {
        Ok(()) => {} // Success - exit code 0
        Err(error) => {
            std::process::exit(error.exit_code());
        }
    }
}
