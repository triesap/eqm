//! Command-line entry point for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod cli;
mod session;

use std::process::ExitCode;

fn main() -> ExitCode {
    match cli::parse(std::env::args().skip(1)) {
        Ok(cli::ParseOutcome::Help) => {
            print!("{}", cli::help());
            ExitCode::SUCCESS
        }
        Ok(cli::ParseOutcome::Run(parsed)) => {
            let _request = session::SessionRequest::new(parsed.global, parsed.command.name);
            println!("{} is not implemented yet", parsed.command.name.as_str());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}\n\n{}", cli::help());
            ExitCode::from(cli::USAGE_EXIT)
        }
    }
}
