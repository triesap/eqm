//! Borrowed boundary over a workspace finalized by CLI orchestration.

use eqm_domain::{FinalizedWorkspaceGraph, RepoPath, Sha256Digest};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::path::{Path, PathBuf};

/// Immutable finalized workspace authority supplied to the MCP adapter.
#[derive(Clone, Debug)]
pub struct PreparedMcpSession<'a> {
    repository_root: PathBuf,
    finalized: &'a FinalizedWorkspaceGraph,
    source_map: &'a BTreeMap<Box<str>, RepoPath>,
    workspace_digest: Sha256Digest,
}

impl<'a> PreparedMcpSession<'a> {
    /// Creates an adapter view from already finalized orchestration output.
    pub fn new(
        repository_root: &Path,
        finalized: &'a FinalizedWorkspaceGraph,
        source_map: &'a BTreeMap<Box<str>, RepoPath>,
        workspace_digest: Sha256Digest,
    ) -> Result<Self, McpSessionError> {
        let repository_root = repository_root
            .canonicalize()
            .map_err(|_| McpSessionError::RepositoryUnavailable)?;
        Ok(Self {
            repository_root,
            finalized,
            source_map,
            workspace_digest,
        })
    }

    /// Returns the canonical repository root for confined adapter reads.
    #[must_use]
    pub fn repository_root(&self) -> &Path {
        &self.repository_root
    }

    /// Returns the finalized immutable semantic graph.
    #[must_use]
    pub const fn finalized(&self) -> &'a FinalizedWorkspaceGraph {
        self.finalized
    }

    /// Returns authoritative semantic source locations.
    #[must_use]
    pub const fn source_map(&self) -> &'a BTreeMap<Box<str>, RepoPath> {
        self.source_map
    }

    /// Returns the exact canonical workspace digest.
    #[must_use]
    pub const fn workspace_digest(&self) -> Sha256Digest {
        self.workspace_digest
    }
}

/// Prepared adapter boundary construction failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum McpSessionError {
    /// The supplied repository root cannot be canonicalized.
    RepositoryUnavailable,
}

impl Display for McpSessionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("prepared MCP repository root is unavailable")
    }
}

impl Error for McpSessionError {}
