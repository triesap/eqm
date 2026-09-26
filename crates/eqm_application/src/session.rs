//! Prepared workspace session ownership.
//!
//! Loading, resolving, expanding, and canonicalizing a workspace happens here,
//! once, for every transport. A [`PreparedSession`] is immutable and borrowable:
//! adapters read the same finalized graph and source map without reloading the
//! workspace or re-deriving conformance.

use eqm_domain::{FinalizedWorkspaceGraph, RepoPath, RepoPathError, Sha256Digest};
use eqm_engine::{FragmentDigestMap, ResolutionError, expand_fragments, resolve_graph};
use eqm_manifest::{
    CanonicalizationError, LoadError, canonicalize_fragment, canonicalize_graph, load_workspace,
};
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

/// Stable prepared-session stage failure retaining its causal error.
#[derive(Debug)]
pub enum SessionError {
    /// The explicit configuration path was not repository-relative.
    ConfigPath(RepoPathError),
    /// The workspace could not be loaded.
    Load(LoadError),
    /// References could not be resolved.
    Resolve(ResolutionError),
    /// A fragment could not be canonicalized.
    FragmentDigest(CanonicalizationError),
    /// Fragments could not be expanded.
    Expand(ResolutionError),
    /// The expanded graph could not be canonicalized.
    Canonicalize(CanonicalizationError),
}

impl Display for SessionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConfigPath(error) => {
                write!(formatter, "session configuration path is invalid: {error}")
            }
            Self::Load(error) => write!(formatter, "session workspace load failed: {error}"),
            Self::Resolve(error) => write!(formatter, "session graph resolution failed: {error}"),
            Self::FragmentDigest(error) => {
                write!(formatter, "session fragment digest failed: {error}")
            }
            Self::Expand(error) => write!(formatter, "session fragment expansion failed: {error}"),
            Self::Canonicalize(error) => {
                write!(formatter, "session canonicalization failed: {error}")
            }
        }
    }
}

impl Error for SessionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ConfigPath(error) => Some(error),
            Self::Load(error) => Some(error),
            Self::Resolve(error) | Self::Expand(error) => Some(error),
            Self::FragmentDigest(error) | Self::Canonicalize(error) => Some(error),
        }
    }
}

/// Prepares one immutable session without execution or workspace writes.
pub fn prepare(request: &SessionRequest, start: &Path) -> Result<PreparedSession, SessionError> {
    let loaded = load_workspace(start, request.config()).map_err(SessionError::Load)?;
    let repository_root = loaded.repository_root().to_path_buf();
    let source_map = loaded.source_map().clone();
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())
        .map_err(SessionError::Resolve)?;
    let digests: FragmentDigestMap = graph
        .fragments()
        .iter()
        .map(|(key, fragment)| {
            canonicalize_fragment(fragment)
                .map(|value| (key.clone(), value.digest()))
                .map_err(SessionError::FragmentDigest)
        })
        .collect::<Result<BTreeMap<_, _>, _>>()?;
    let finalized =
        expand_fragments(graph, &digests, loaded.source_map()).map_err(SessionError::Expand)?;
    let workspace_digest = canonicalize_graph(&finalized)
        .map_err(SessionError::Canonicalize)?
        .digest();
    Ok(PreparedSession {
        repository_root,
        finalized,
        source_map,
        workspace_digest,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn example() -> Result<tempfile::TempDir, Box<dyn Error>> {
        let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/android-ios");
        let repository = tempfile::tempdir()?;
        copy_directory(&source, repository.path())?;
        fs::create_dir(repository.path().join(".git"))?;
        Ok(repository)
    }

    fn copy_directory(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(destination)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let target = destination.join(entry.file_name());
            if entry.file_type()?.is_dir() {
                copy_directory(&entry.path(), &target)?;
            } else {
                fs::copy(entry.path(), target)?;
            }
        }
        Ok(())
    }

    #[test]
    fn unavailable_start_retains_the_config_cause() -> Result<(), Box<dyn Error>> {
        let missing = Path::new("/definitely-not-an-eqm-workspace");
        match prepare(&SessionRequest::new(None), missing) {
            Err(SessionError::Load(LoadError::Config(error))) => {
                assert_eq!(error, eqm_manifest::ConfigError::StartUnavailable);
            }
            other => return Err(format!("expected a config cause, got {other:?}").into()),
        }
        Ok(())
    }

    #[test]
    fn malformed_graph_retains_the_resolution_cause() -> Result<(), Box<dyn Error>> {
        let repository = example()?;
        let journey = repository.path().join("eqm/contracts/auth.signup.toml");
        let text = fs::read_to_string(&journey)?.replace(
            "capability = \"account.create\"",
            "capability = \"account.missing\"",
        );
        fs::write(&journey, text)?;
        match prepare(&SessionRequest::new(None), repository.path()) {
            Err(SessionError::Resolve(ResolutionError::Findings(findings))) => {
                assert!(!findings.is_empty());
            }
            other => return Err(format!("expected a resolution cause, got {other:?}").into()),
        }
        Ok(())
    }
}
