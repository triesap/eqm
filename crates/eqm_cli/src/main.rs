//! Command-line entry point for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod cli;
mod commands;
mod renderer;
mod session;

use serde_json::json;
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
            let execution = if matches!(
                command,
                cli::CommandName::Validate
                    | cli::CommandName::Check
                    | cli::CommandName::Show
                    | cli::CommandName::Locate
                    | cli::CommandName::Context
                    | cli::CommandName::Matrix
                    | cli::CommandName::Obligations
                    | cli::CommandName::Diff
                    | cli::CommandName::Affected
                    | cli::CommandName::Discover
                    | cli::CommandName::Reconcile
            ) {
                match std::env::current_dir() {
                    Ok(start) if command == cli::CommandName::Validate => {
                        commands::validate::execute(parsed, &start)
                    }
                    Ok(start) if command == cli::CommandName::Check => {
                        commands::check::execute(parsed, &start)
                    }
                    Ok(start) if command == cli::CommandName::Show => {
                        commands::show::execute(parsed, &start)
                    }
                    Ok(start) if command == cli::CommandName::Locate => {
                        commands::locate::execute(parsed, &start)
                    }
                    Ok(start) if command == cli::CommandName::Context => {
                        commands::context::execute(parsed, &start)
                    }
                    Ok(start) if command == cli::CommandName::Matrix => {
                        commands::matrix::execute(parsed, &start)
                    }
                    Ok(start) if command == cli::CommandName::Obligations => {
                        commands::obligations::execute(parsed, &start)
                    }
                    Ok(start) if command == cli::CommandName::Diff => {
                        commands::diff::execute(parsed, &start)
                    }
                    Ok(start) if command == cli::CommandName::Affected => {
                        commands::affected::execute(parsed, &start)
                    }
                    Ok(start) if command == cli::CommandName::Discover => {
                        commands::discover::execute(parsed, &start)
                    }
                    Ok(start) => commands::reconcile::execute(parsed, &start),
                    Err(error) => Err(Box::new(error) as Box<dyn std::error::Error>),
                }
            } else {
                let _request = session::SessionRequest::new(parsed.global, command);
                Ok(commands::CommandExecution {
                    payload: renderer::OutputPayload {
                        human: format!("{} is not implemented yet", command.as_str()),
                        json: json!({"command": command.as_str(), "status": "not_implemented"}),
                        sarif: Some(json!({"version":"2.1.0", "runs":[]})),
                        markdown: Some(format!("# {}\n\nNot implemented yet.", command.as_str())),
                    },
                    exit_code: 0,
                })
            };
            let execution = match execution {
                Ok(value) => value,
                Err(error) => {
                    let _ = reporter.log(&format!("error: {error}"));
                    return ExitCode::from(6);
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
