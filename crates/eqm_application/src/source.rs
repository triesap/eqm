//! Repository identity normalization and exact source-tree provenance.
//!
//! DEV-007 S022 centralizes the identity acquisition that was previously copied
//! across CLI commands. Two things are deliberately separate:
//!
//! - the **repository identity** is a canonical configured HTTPS URI derived
//!   from the authored `origin` remote. Supported encodings map deliberately to
//!   one canonical form; missing or unsupported remotes are errors rather than
//!   silently trusted strings.
//! - the **source-tree provenance** binds the inspected bytes: the exact `HEAD`
//!   commit plus whether the working tree is dirty (modified or untracked
//!   content). A dirty tree never masquerades as clean `HEAD` provenance.
//!
//! The module is effectful (it invokes `git`) and therefore lives in the shared
//! `eqm_application` layer, not in `eqm_domain` or `eqm_engine`.

use eqm_domain::{EvidenceResultBuildError, RepositoryIdentity, SourceCommit};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::path::Path;
use std::process::Command;

/// Exact inspected source-tree provenance for one repository root.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceProvenance {
    repository: RepositoryIdentity,
    source_commit: SourceCommit,
    dirty: bool,
}

impl SourceProvenance {
    /// Returns the normalized repository identity.
    #[must_use]
    pub const fn repository(&self) -> &RepositoryIdentity {
        &self.repository
    }

    /// Returns the exact inspected `HEAD` commit.
    #[must_use]
    pub const fn source_commit(&self) -> &SourceCommit {
        &self.source_commit
    }

    /// Returns whether tracked or untracked content differs from `HEAD`.
    #[must_use]
    pub const fn dirty(&self) -> bool {
        self.dirty
    }

    /// Returns whether the working tree exactly matches `HEAD`.
    #[must_use]
    pub const fn clean(&self) -> bool {
        !self.dirty
    }
}

/// Source provenance acquisition or normalization failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceProvenanceError {
    /// The repository has no configured `origin` remote.
    MissingRemote,
    /// The remote encoding is not one of the deliberately supported forms.
    UnsupportedRemote,
    /// The normalized repository identity was invalid.
    Identity(EvidenceResultBuildError),
    /// The inspected `HEAD` was not a full object ID.
    Commit(EvidenceResultBuildError),
    /// The `git` executable could not be invoked.
    GitUnavailable,
    /// A `git` invocation did not succeed.
    GitFailed,
}

impl Display for SourceProvenanceError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRemote => write!(formatter, "origin remote is not configured"),
            Self::UnsupportedRemote => write!(formatter, "origin remote encoding is unsupported"),
            Self::Identity(error) => write!(formatter, "repository identity is invalid: {error:?}"),
            Self::Commit(error) => write!(formatter, "source commit is invalid: {error:?}"),
            Self::GitUnavailable => write!(formatter, "git is unavailable"),
            Self::GitFailed => write!(formatter, "git inspection failed"),
        }
    }
}

impl Error for SourceProvenanceError {}

/// Normalizes one supported `origin` remote encoding into a canonical HTTPS
/// repository identity.
///
/// Supported forms (optionally with a trailing `.git` and/or `/`):
/// `https://host/path`, `ssh://git@host/path` and `git@host:path`. Missing or
/// other encodings (for example `git://`) are errors and are never trusted as a
/// repository identity without deliberate normalization.
pub fn normalize_repository_identity(
    remote: &str,
) -> Result<RepositoryIdentity, SourceProvenanceError> {
    let remote = remote.trim();
    if remote.is_empty() {
        return Err(SourceProvenanceError::MissingRemote);
    }
    let https = if let Some(rest) = remote.strip_prefix("https://") {
        format!("https://{rest}")
    } else if let Some(rest) = remote.strip_prefix("ssh://") {
        let rest = rest.strip_prefix("git@").unwrap_or(rest);
        format!("https://{rest}")
    } else if let Some(rest) = remote.strip_prefix("git@") {
        let (host, path) = rest
            .split_once(':')
            .ok_or(SourceProvenanceError::UnsupportedRemote)?;
        format!("https://{host}/{path}")
    } else {
        return Err(SourceProvenanceError::UnsupportedRemote);
    };
    let normalized = https
        .trim_end_matches('/')
        .trim_end_matches(".git")
        .to_owned();
    normalized.parse().map_err(SourceProvenanceError::Identity)
}

fn git_stdout(root: &Path, arguments: &[&str]) -> Result<String, SourceProvenanceError> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(root)
        .output()
        .map_err(|_| SourceProvenanceError::GitUnavailable)?;
    if !output.status.success() {
        return Err(SourceProvenanceError::GitFailed);
    }
    String::from_utf8(output.stdout)
        .map(|value| value.trim().to_owned())
        .map_err(|_| SourceProvenanceError::GitFailed)
}

/// Returns the normalized repository identity for one repository root.
pub fn repository_identity(root: &Path) -> Result<RepositoryIdentity, SourceProvenanceError> {
    let remote = git_stdout(root, &["remote", "get-url", "origin"])?;
    normalize_repository_identity(&remote)
}

/// Returns the exact inspected source-tree provenance for one repository root.
///
/// The `dirty` flag is true when `git status --porcelain` reports any tracked or
/// untracked change; ignored generated state (for example `.eqm/`) is not a
/// change.
pub fn inspect_source_provenance(root: &Path) -> Result<SourceProvenance, SourceProvenanceError> {
    let repository = repository_identity(root)?;
    let commit = git_stdout(root, &["rev-parse", "HEAD"])?;
    let source_commit: SourceCommit = commit.parse().map_err(SourceProvenanceError::Commit)?;
    let status = git_stdout(root, &["status", "--porcelain", "--untracked-files=normal"])?;
    Ok(SourceProvenance {
        repository,
        source_commit,
        dirty: !status.is_empty(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn git(root: &Path, arguments: &[&str]) -> Result<(), Box<dyn Error>> {
        let status = Command::new("git")
            .args(arguments)
            .current_dir(root)
            .status()?;
        if !status.success() {
            return Err("git fixture command failed".into());
        }
        Ok(())
    }

    fn repository() -> Result<tempfile::TempDir, Box<dyn Error>> {
        let directory = tempfile::tempdir()?;
        let root = directory.path();
        git(root, &["init", "-q"])?;
        git(
            root,
            &[
                "remote",
                "add",
                "origin",
                "git@github.com:example/project.git",
            ],
        )?;
        fs::write(root.join("tracked.txt"), "one\n")?;
        git(root, &["add", "."])?;
        git(
            root,
            &[
                "-c",
                "user.name=Fixture",
                "-c",
                "user.email=fixture@example.invalid",
                "commit",
                "-qm",
                "fixture",
            ],
        )?;
        Ok(directory)
    }

    #[test]
    fn supported_remote_encodings_map_to_one_identity() -> Result<(), Box<dyn Error>> {
        let expected = "https://github.com/example/project";
        for remote in [
            "git@github.com:example/project.git",
            "git@github.com:example/project",
            "ssh://git@github.com/example/project.git",
            "ssh://github.com/example/project",
            "https://github.com/example/project",
            "https://github.com/example/project.git",
            "https://github.com/example/project/",
        ] {
            assert_eq!(
                normalize_repository_identity(remote)?.as_str(),
                expected,
                "remote {remote} must normalize deliberately"
            );
        }
        Ok(())
    }

    #[test]
    fn unsupported_or_missing_remotes_are_errors() {
        assert_eq!(
            normalize_repository_identity(""),
            Err(SourceProvenanceError::MissingRemote)
        );
        assert_eq!(
            normalize_repository_identity("git://github.com/example/project.git"),
            Err(SourceProvenanceError::UnsupportedRemote)
        );
        assert_eq!(
            normalize_repository_identity("git@github.com"),
            Err(SourceProvenanceError::UnsupportedRemote)
        );
        assert!(matches!(
            normalize_repository_identity("https://github.com/example/project name"),
            Err(SourceProvenanceError::Identity(_))
        ));
    }

    #[test]
    fn clean_tree_is_clean_and_modified_or_untracked_content_is_dirty() -> Result<(), Box<dyn Error>>
    {
        let directory = repository()?;
        let root = directory.path();
        let clean = inspect_source_provenance(root)?;
        assert!(clean.clean());
        assert!(!clean.dirty());
        assert_eq!(
            clean.repository().as_str(),
            "https://github.com/example/project"
        );

        fs::write(root.join("tracked.txt"), "two\n")?;
        assert!(inspect_source_provenance(root)?.dirty());

        git(root, &["checkout", "--", "tracked.txt"])?;
        assert!(inspect_source_provenance(root)?.clean());

        fs::write(root.join("untracked.txt"), "new\n")?;
        assert!(inspect_source_provenance(root)?.dirty());
        Ok(())
    }

    #[test]
    fn missing_repository_is_reported() {
        let root = PathBuf::from("/definitely-not-a-git-repository");
        assert!(matches!(
            inspect_source_provenance(&root),
            Err(SourceProvenanceError::GitUnavailable | SourceProvenanceError::GitFailed)
        ));
    }
}
