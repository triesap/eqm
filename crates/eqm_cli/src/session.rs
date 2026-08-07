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
    workspace_digest: Sha256Digest,
}

impl PreparedSession {
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

    /// Returns the exact canonical semantic workspace digest.
    #[must_use]
    pub const fn workspace_digest(&self) -> Sha256Digest {
        self.workspace_digest
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
        workspace_digest,
    })
}
