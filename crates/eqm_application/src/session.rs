//! Prepared workspace session ownership.
//!
//! Loading, resolving, expanding, and canonicalizing a workspace happens here,
//! once, for every transport. A [`PreparedSession`] is immutable and borrowable:
//! adapters read the same finalized graph and source map without reloading the
//! workspace or re-deriving conformance.

use eqm_domain::{FinalizedWorkspaceGraph, RepoPath, Sha256Digest};
use eqm_engine::{FragmentDigestMap, expand_fragments, resolve_graph};
use eqm_manifest::{canonicalize_fragment, canonicalize_graph, load_workspace};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::path::{Path, PathBuf};

/// Immutable inputs used to prepare one workspace session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionRequest {
    config: Option<RepoPath>,
}

impl SessionRequest {
    /// Captures loading inputs without reading or mutating the workspace.
    #[must_use]
    pub const fn new(config: Option<RepoPath>) -> Self {
        Self { config }
    }

    /// Returns the explicit configuration path, if any.
    #[must_use]
    pub const fn config(&self) -> Option<&RepoPath> {
        self.config.as_ref()
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

/// Stable prepared-session stage failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionError {
    /// The explicit configuration path was not repository-relative.
    ConfigPath,
    /// The workspace could not be loaded.
    Load,
    /// References could not be resolved.
    Resolve,
    /// A fragment could not be canonicalized.
    FragmentDigest,
    /// Fragments could not be expanded.
    Expand,
    /// The expanded graph could not be canonicalized.
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
    let loaded = load_workspace(start, request.config()).map_err(|_| SessionError::Load)?;
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
