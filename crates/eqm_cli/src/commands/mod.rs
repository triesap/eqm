//! Command orchestration over one prepared session.

pub mod validate;

use crate::renderer::OutputPayload;

/// Renderable command result plus stable exit category.
pub struct CommandExecution {
    /// Semantic output representations.
    pub payload: OutputPayload,
    /// Stable process exit code.
    pub exit_code: u8,
}
