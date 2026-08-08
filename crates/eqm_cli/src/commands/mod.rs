//! Command orchestration over one prepared session.

pub mod affected;
pub mod attest;
pub mod check;
pub mod context;
pub mod diff;
pub mod discover;
pub mod evaluation;
pub mod locate;
pub mod matrix;
pub mod obligations;
pub mod reconcile;
pub mod release_check;
pub mod show;
pub mod validate;
pub mod verify;

use crate::renderer::OutputPayload;
use eqm_domain::{DiagnosticBuildError, RepoPath, SourceLocation, SourceName, SourcePosition};
use eqm_protocol::SourceLocationDto;

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
