//! Transport adapter over the shared application session boundary.
//!
//! Workspace loading, resolution, expansion, and canonicalization live in
//! `eqm_application::session`. This module only maps CLI options into the
//! application request and adapts a prepared session to the MCP transport; it
//! never re-derives conformance or reloads the workspace.

use crate::cli::{CommandName, GlobalOptions};
use eqm_domain::RepoPath;
use std::path::Path;

pub use eqm_application::session::{PreparedSession, SessionError};

/// CLI command envelope used to prepare one workspace session.
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

    fn application(&self) -> Result<eqm_application::SessionRequest, SessionError> {
        let config = self
            .global
            .config
            .as_ref()
            .map(|path| RepoPath::new(path.to_string_lossy().replace('\\', "/")))
            .transpose()
            .map_err(SessionError::ConfigPath)?;
        Ok(eqm_application::SessionRequest::new(config))
    }
}

/// Prepares one immutable session through the shared application boundary.
pub fn prepare(request: &SessionRequest, start: &Path) -> Result<PreparedSession, SessionError> {
    eqm_application::session::prepare(&request.application()?, start)
}

/// Borrows an already finalized session through the thin MCP transport adapter.
pub fn mcp_session(
    session: &PreparedSession,
) -> Result<eqm_mcp::PreparedMcpSession<'_>, eqm_mcp::McpSessionError> {
    eqm_mcp::PreparedMcpSession::new(
        session.repository_root(),
        session.finalized(),
        session.source_map(),
        session.workspace_digest(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn finalized_cli_session_is_borrowed_without_reloading_for_mcp() -> Result<(), Box<dyn Error>> {
        let repository = crate::test_support::example_repository()?;
        let root = repository.path();
        let request = SessionRequest::new(Default::default(), CommandName::Validate);
        let session = prepare(&request, root)?;
        let mcp = mcp_session(&session)?;
        assert_eq!(mcp.workspace_digest(), session.workspace_digest());
        assert!(std::ptr::eq(mcp.finalized(), session.finalized()));
        assert!(std::ptr::eq(mcp.source_map(), session.source_map()));
        let resources = eqm_mcp::list_resources(&mcp);
        assert!(resources.contains(&eqm_mcp::McpResourceUri::Workspace));
        assert!(resources.contains(&eqm_mcp::McpResourceUri::Findings));
        let evaluated_at = "2026-08-08T00:00:00Z".parse()?;
        for uri in resources {
            let resource = eqm_mcp::read_resource(&mcp, &uri, evaluated_at)?;
            let payload: serde_json::Value = serde_json::from_str(&resource.text)?;
            assert_eq!(
                payload.get("schema").and_then(serde_json::Value::as_str),
                Some(
                    "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/protocol/result.schema.json"
                )
            );
        }
        Ok(())
    }
}
