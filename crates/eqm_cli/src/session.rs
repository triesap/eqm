//! Prepared workspace session request boundary.

use crate::cli::{CommandName, GlobalOptions};
use eqm_domain::{FinalizedWorkspaceGraph, RepoPath, Sha256Digest};
use eqm_engine::{FragmentDigestMap, expand_fragments, resolve_graph};
use eqm_manifest::{canonicalize_fragment, canonicalize_graph, load_workspace};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::path::{Path, PathBuf};

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

/// One fully loaded, resolved, invariant-checked, expanded, canonical session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedSession {
    repository_root: PathBuf,
    finalized: FinalizedWorkspaceGraph,
    source_map: BTreeMap<Box<str>, RepoPath>,
    workspace_digest: Sha256Digest,
}

impl PreparedSession {
    /// Borrows the already finalized session through the thin MCP boundary.
    pub fn mcp_session(&self) -> Result<eqm_mcp::PreparedMcpSession<'_>, eqm_mcp::McpSessionError> {
        eqm_mcp::PreparedMcpSession::new(
            &self.repository_root,
            &self.finalized,
            &self.source_map,
            self.workspace_digest,
        )
    }

    /// Returns the canonical repository root used for confined inspection.
    #[must_use]
    pub fn repository_root(&self) -> &Path {
        &self.repository_root
    }

    /// Returns the finalized graph used by every query in the session.
    #[must_use]
    pub const fn finalized(&self) -> &FinalizedWorkspaceGraph {
        &self.finalized
    }

    /// Returns semantic authority keys mapped to repository-relative sources.
    #[must_use]
    pub const fn source_map(&self) -> &BTreeMap<Box<str>, RepoPath> {
        &self.source_map
    }

    /// Returns the exact canonical semantic workspace digest.
    #[must_use]
    pub const fn workspace_digest(&self) -> Sha256Digest {
        self.workspace_digest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finalized_cli_session_is_borrowed_without_reloading_for_mcp() -> Result<(), Box<dyn Error>> {
        let repository = crate::test_support::example_repository()?;
        let root = repository.path();
        let request = SessionRequest::new(Default::default(), CommandName::Validate);
        let session = prepare(&request, root)?;
        let mcp = session.mcp_session()?;
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

/// Stable prepared-session stage failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionError {
    ConfigPath,
    Load,
    Resolve,
    FragmentDigest,
    Expand,
    Canonicalize,
}

impl Display for SessionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl Error for SessionError {}

/// Prepares one immutable session without execution or workspace writes.
pub fn prepare(request: &SessionRequest, start: &Path) -> Result<PreparedSession, SessionError> {
    let explicit = request
        .global
        .config
        .as_ref()
        .map(|path| RepoPath::new(path.to_string_lossy().replace('\\', "/")))
        .transpose()
        .map_err(|_| SessionError::ConfigPath)?;
    let loaded = load_workspace(start, explicit.as_ref()).map_err(|_| SessionError::Load)?;
    let repository_root = loaded.repository_root().to_path_buf();
    let source_map = loaded.source_map().clone();
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())
        .map_err(|_| SessionError::Resolve)?;
    let digests: FragmentDigestMap = graph
        .fragments()
        .iter()
        .map(|(key, fragment)| {
            canonicalize_fragment(fragment)
                .map(|value| (key.clone(), value.digest()))
                .map_err(|_| SessionError::FragmentDigest)
        })
        .collect::<Result<BTreeMap<_, _>, _>>()?;
    let finalized =
        expand_fragments(graph, &digests, loaded.source_map()).map_err(|_| SessionError::Expand)?;
    let workspace_digest = canonicalize_graph(&finalized)
        .map_err(|_| SessionError::Canonicalize)?
        .digest();
    Ok(PreparedSession {
        repository_root,
        finalized,
        source_map,
        workspace_digest,
    })
}
