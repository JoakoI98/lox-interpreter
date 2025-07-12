use ast_leaf::ast_leaf;

// #[ast_leaf("+" ("+" | "*" | 0: non_terminal) nt )]
// pub struct MacroIntegrationTest {
//     #[Type]
//     pub testomg: TestType,
//     pub testomg2: TestType,
//     pub nt: String,
// }

mod evaluation;
mod syntax_analysis;
mod tokenizer;

use std::env;
use std::fs;
use std::io::{self, Write};

use tokenizer::scan_tokens;

fn main() {
    run();
}

fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|err| {
                writeln!(io::stderr(), "Failed to read file {}: {}", filename, err).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let (tokens, errors) = scan_tokens(file_contents.as_str());
                for error in &errors {
                    eprintln!("{}", error);
                }
                for token in &tokens {
                    println!("{:?}", token);
                }
                if !errors.is_empty() {
                    std::process::exit(65);
                }
            } else {
                println!("EOF  null"); // Placeholder, replace this line when implementing the scanner
            }
        }
        "parse" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|err| {
                writeln!(io::stderr(), "Failed to read file {}: {}", filename, err).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let (tokens, errors) = scan_tokens(file_contents.as_str());
                for error in &errors {
                    eprintln!("{}", error);
                }
                if !errors.is_empty() {
                    std::process::exit(65);
                }
                let mut parse_stream = syntax_analysis::ParseStream::new(tokens);
                let r = parse_stream.parse::<syntax_analysis::Expression>();
                match r {
                    Ok(r) => println!("{}", r),
                    Err(e) => {
                        eprintln!("{}", e);
                        std::process::exit(65);
                    }
                }
            } else {
                println!("EOF  null"); // Placeholder, replace this line when implementing the scanner
            }
        }
        "evaluate" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|err| {
                writeln!(io::stderr(), "Failed to read file {}: {}", filename, err).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let (tokens, errors) = scan_tokens(file_contents.as_str());
                for error in &errors {
                    eprintln!("{}", error);
                }
                if !errors.is_empty() {
                    std::process::exit(65);
                }
                let mut parse_stream = syntax_analysis::ParseStream::new(tokens);
                let r = parse_stream.parse::<syntax_analysis::Expression>();
                match r {
                    Ok(r) => {
                        let r = r.eval();
                        match r {
                            Ok(r) => println!("{}", r),
                            Err(e) => {
                                eprintln!("{}", e);
                                std::process::exit(70);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        std::process::exit(65);
                    }
                }
            } else {
                println!("EOF  null"); // Placeholder, replace this line when implementing the scanner
            }
        }
        "run" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|err| {
                writeln!(io::stderr(), "Failed to read file {}: {}", filename, err).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let (tokens, errors) = scan_tokens(file_contents.as_str());
                for error in &errors {
                    eprintln!("{}", error);
                }
                if !errors.is_empty() {
                    std::process::exit(65);
                }
                let mut parse_stream = syntax_analysis::ParseStream::new(tokens);
                let r = parse_stream.parse::<syntax_analysis::Program>();
                match r {
                    Ok(r) => {
                        let r = r.run();
                        match r {
                            Err(e) => {
                                eprintln!("{}", e);
                                std::process::exit(70);
                            }
                            _ => {}
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        std::process::exit(65);
                    }
                }
            } else {
                println!("EOF  null"); // Placeholder, replace this line when implementing the scanner
            }
        }

        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
        }
    }
    return;
}
