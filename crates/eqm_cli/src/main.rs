//! Command-line entry point for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod cli;
mod commands;
mod renderer;
mod session;

use std::io::{self, IsTerminal};
use std::process::ExitCode;

fn main() -> ExitCode {
    match cli::parse(std::env::args().skip(1)) {
        Ok(cli::ParseOutcome::Help) => {
            print!("{}", cli::help());
            ExitCode::SUCCESS
        }
        Ok(cli::ParseOutcome::Run(parsed)) => {
            let command = parsed.command.name;
            if command == cli::CommandName::McpServe {
                return match std::env::current_dir()
                    .map_err(|error| Box::new(error) as Box<dyn std::error::Error>)
                    .and_then(|start| commands::mcp::serve_stdio(parsed, &start))
                {
                    Ok(()) => ExitCode::SUCCESS,
                    Err(error) => {
                        eprintln!("error: {error}");
                        ExitCode::from(6)
                    }
                };
            }
            let format = parsed.global.format;
            let no_progress = parsed.global.no_progress;
            let color = parsed.global.color;
            let output = renderer::explicit_output_path(parsed.global.output.clone());
            let _decorate = renderer::color_enabled(format, color, io::stdout().is_terminal());
            let mut stderr = io::stderr();
            let mut reporter = renderer::StderrReporter::new(&mut stderr, no_progress);
            if reporter.progress("rendering command result").is_err() {
                return ExitCode::from(6);
            }
            let execution = std::env::current_dir()
                .map_err(|error| Box::new(error) as Box<dyn std::error::Error>)
                .and_then(|start| commands::execute(parsed, &start));
            let execution = match execution {
                Ok(value) => value,
                Err(error) => {
                    let _ = reporter.log(&format!("error: {error}"));
                    return ExitCode::from(match command {
                        cli::CommandName::Verify => 5,
                        cli::CommandName::Attest | cli::CommandName::ReleaseCheck => 7,
                        _ => 6,
                    });
                }
            };
            match renderer::render(&execution.payload, format).and_then(|document| {
                let _machine = document.is_machine();
                renderer::deliver(&document, output.as_deref(), &mut io::stdout())
            }) {
                Ok(()) => ExitCode::from(execution.exit_code),
                Err(error) => {
                    let _ = reporter.log(&format!("error: {error}"));
                    ExitCode::from(6)
                }
            }
        }
        Err(error) => {
            eprintln!("error: {error}\n\n{}", cli::help());
            ExitCode::from(cli::USAGE_EXIT)
        }
    }
}
