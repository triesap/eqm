//! Command orchestration over one prepared session.

pub mod affected;
pub mod attest;
pub mod check;
pub mod context;
pub mod diff;
pub mod discover;
pub mod doctor;
pub mod evaluation;
pub mod explain;
pub mod fmt;
pub mod init_new;
pub mod locate;
pub mod lock;
pub mod matrix;
pub mod mcp;
pub mod obligations;
pub mod reconcile;
pub mod release_check;
pub mod show;
pub mod validate;
pub mod verify;

use crate::cli::{CommandName, ParsedCli};
use crate::renderer::OutputPayload;
use eqm_domain::{DiagnosticBuildError, RepoPath, SourceLocation, SourceName, SourcePosition};
use eqm_protocol::SourceLocationDto;
use std::error::Error;
use std::path::Path;

/// Executes one parsed non-MCP command through the production dispatcher.
pub fn execute(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    match parsed.command.name {
        CommandName::Init => init_new::init(parsed, start),
        CommandName::New => init_new::new(parsed, start),
        CommandName::Fmt => fmt::execute(parsed, start),
        CommandName::Validate => validate::execute(parsed, start),
        CommandName::Check => check::execute(parsed, start),
        CommandName::Show => show::execute(parsed, start),
        CommandName::Locate => locate::execute(parsed, start),
        CommandName::Context => context::execute(parsed, start),
        CommandName::Matrix => matrix::execute(parsed, start),
        CommandName::Obligations => obligations::execute(parsed, start),
        CommandName::Diff => diff::execute(parsed, start),
        CommandName::Affected => affected::execute(parsed, start),
        CommandName::Discover => discover::execute(parsed, start),
        CommandName::Reconcile => reconcile::execute(parsed, start),
        CommandName::Verify => verify::execute(parsed, start),
        CommandName::Attest => attest::execute(parsed, start),
        CommandName::ReleaseCheck => release_check::execute(parsed, start),
        CommandName::Explain => explain::execute(parsed),
        CommandName::Doctor => doctor::execute(parsed, start),
        CommandName::LockUpdate => lock::execute(parsed, start),
        CommandName::McpServe => Err("MCP serve requires the stdio dispatcher".into()),
    }
}

fn source_location(path: &RepoPath) -> Result<SourceLocationDto, DiagnosticBuildError> {
    let position = SourcePosition::new(1, 1)?;
    let location = SourceLocation::new(SourceName::new(path.as_str())?, position, position)?;
    Ok(SourceLocationDto::from_domain(&location))
}

/// Renderable command result plus stable exit category.
pub struct CommandExecution {
    /// Semantic output representations.
    pub payload: OutputPayload,
    /// Stable process exit code.
    pub exit_code: u8,
}
