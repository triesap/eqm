//! Prepared workspace session request boundary.

use crate::cli::{CommandName, GlobalOptions};

/// Immutable inputs used to prepare one workspace session for a command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionRequest {
    /// Parsed global options.
    pub global: GlobalOptions,
    /// Exact command identity requesting the session.
    pub command: CommandName,
}

impl SessionRequest {
    /// Captures loading inputs without reading or mutating the workspace.
    #[must_use]
    pub const fn new(global: GlobalOptions, command: CommandName) -> Self {
        Self { global, command }
    }
}
