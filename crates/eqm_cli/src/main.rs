//! Command-line entry point for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod cli;
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
            let _request = session::SessionRequest::new(parsed.global, command);
            let _decorate = renderer::color_enabled(format, color, io::stdout().is_terminal());
            let mut stderr = io::stderr();
            let mut reporter = renderer::StderrReporter::new(&mut stderr, no_progress);
            if reporter.progress("rendering command result").is_err() {
                return ExitCode::from(6);
            }
            let payload = renderer::OutputPayload {
                human: format!("{} is not implemented yet", command.as_str()),
                json: json!({"command": command.as_str(), "status": "not_implemented"}),
                sarif: Some(json!({"version":"2.1.0", "runs":[]})),
                markdown: Some(format!("# {}\n\nNot implemented yet.", command.as_str())),
            };
            match renderer::render(&payload, format).and_then(|document| {
                let _machine = document.is_machine();
                renderer::deliver(&document, output.as_deref(), &mut io::stdout())
            }) {
                Ok(()) => ExitCode::SUCCESS,
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
