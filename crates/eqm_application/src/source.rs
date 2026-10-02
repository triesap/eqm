//! Repository identity normalization and exact source-tree provenance.
//!
//! DEV-007 S022 centralized the identity acquisition that was previously copied
//! across CLI commands. DEV-008 S022 defines a conservative, repository-wide
//! source inspection:
//!
//! - the **repository identity** is a canonical configured HTTPS URI derived
//!   from the authored `origin` remote. Supported encodings map deliberately to
//!   one canonical form; missing or unsupported remotes are errors rather than
//!   silently trusted strings.
//! - the **source-tree provenance** binds the inspected bytes: the exact `HEAD`
//!   commit plus whether the repository-wide inspection found any non-authorizing
//!   tracked, index, untracked or ignored content. Only untracked or ignored
//!   content below the reserved generated `.eqm/` output tree is neutral;
//!   tracked changes remain changes even inside that tree, and ignored content
//!   elsewhere is never permission to conceal source. Assume-unchanged and
//!   skip-worktree index flags are rejected because normal status can conceal
//!   modified bytes.
//!
//! The module is effectful (it invokes `git`) and therefore lives in the shared
//! `eqm_application` layer, not in `eqm_domain` or `eqm_engine`.

use eqm_domain::{EvidenceResultBuildError, RepositoryIdentity, SourceCommit};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Maximum nested populated-repository depth inspected below the owning root.
const MAX_NESTED_DEPTH: u32 = 16;
/// Maximum number of repositories inspected in one provenance pass.
const MAX_NESTED_REPOSITORIES: usize = 256;

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

    /// Returns whether the repository-wide inspection found any non-authorizing
    /// tracked, index, untracked or ignored content.
    #[must_use]
    pub const fn dirty(&self) -> bool {
        self.dirty
    }

    /// Returns whether the defined conservative source inspection passed.
    ///
    /// This is not a claim that every byte equals `HEAD`: it means the inspection
    /// observed no tracked/index change and no untracked or ignored content
    /// outside the reserved generated `.eqm/` output tree, and no concealed index
    /// entries. Races and the actual executed subject remain separate concerns
    /// owned by the later immutable-source/acquisition steps.
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
    /// A tracked entry is marked assume-unchanged or skip-worktree, so normal
    /// status cannot authorize a clean provenance claim.
    ConcealedIndex,
    /// A populated nested repository could not be inspected (missing,
    /// uninitialized, not its own root, non-regular or failed Git).
    NestedUnavailable,
    /// A nested repository conceals modified bytes from normal status.
    NestedConcealed,
    /// A nested repository's `HEAD` does not match its declared Gitlink.
    NestedHeadMismatch,
    /// Nested repository traversal exceeded its bound or revisited a path.
    NestedLimit,
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
            Self::ConcealedIndex => {
                write!(
                    formatter,
                    "tracked entries conceal changes from normal status"
                )
            }
            Self::NestedUnavailable => {
                write!(formatter, "a nested repository could not be inspected")
            }
            Self::NestedConcealed => {
                write!(
                    formatter,
                    "a nested repository conceals changes from normal status"
                )
            }
            Self::NestedHeadMismatch => {
                write!(
                    formatter,
                    "a nested repository HEAD differs from its declared Gitlink"
                )
            }
            Self::NestedLimit => {
                write!(
                    formatter,
                    "nested repository traversal exceeded its bounded inspection"
                )
            }
        }
    }
}

impl Error for SourceProvenanceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Identity(error) | Self::Commit(error) => Some(error),
            _ => None,
        }
    }
}

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

fn git_bytes(root: &Path, arguments: &[&str]) -> Result<Vec<u8>, SourceProvenanceError> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(root)
        .output()
        .map_err(|_| SourceProvenanceError::GitUnavailable)?;
    if !output.status.success() {
        return Err(SourceProvenanceError::GitFailed);
    }
    Ok(output.stdout)
}

/// Returns whether any tracked entry is marked assume-unchanged or
/// skip-worktree, which conceals modified bytes from normal status.
fn concealed_entries_present(root: &Path) -> Result<bool, SourceProvenanceError> {
    let listing = git_bytes(root, &["ls-files", "-v", "-z"])?;
    for entry in listing.split(|byte| *byte == 0) {
        let Some(tag) = entry.first() else {
            continue;
        };
        // `H` is a normal cached entry; `S` is skip-worktree and any lowercase
        // tag is assume-unchanged. Both conceal modified bytes from status.
        if *tag == b'S' || tag.is_ascii_lowercase() {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Returns whether one reported path is below the reserved generated output
/// tree `.eqm/`. `.eqm-other` and other prefixes are not the reserved tree.
fn is_reserved_output_path(path: &[u8]) -> bool {
    let path = path.strip_suffix(b"/").unwrap_or(path);
    path == b".eqm" || path.starts_with(b".eqm/")
}

/// Returns whether a NUL-delimited porcelain status has any non-authorizing
/// entry. Tracked/index changes (including renames and copies) are always dirty;
/// untracked and ignored entries are neutral only inside the reserved `.eqm/`
/// output tree of the owning inspected root (set `reserved`). A nested
/// repository has no reserved tree. A rename/copy record's extra original-path
/// field lacks the `XY ` prefix and is ignored by the status-code check.
fn status_has_dirty_content(status: &[u8], reserved: bool) -> bool {
    for token in status
        .split(|byte| *byte == 0)
        .filter(|token| !token.is_empty())
    {
        if token.len() < 3 || token[2] != b' ' {
            continue;
        }
        let (index, worktree) = (token[0], token[1]);
        let path = &token[3..];
        match (index, worktree) {
            (b'R', _) | (b'C', _) => return true,
            (b'?', b'?') | (b'!', b'!') => {
                if !(reserved && is_reserved_output_path(path)) {
                    return true;
                }
            }
            _ => return true,
        }
    }
    false
}

/// Returns the declared Gitlink entries (path plus recorded object id) in the
/// index of one repository root, parsed NUL-delimited so paths with spaces or
/// newlines cannot be misread.
fn gitlink_entries(root: &Path) -> Result<Vec<(PathBuf, String)>, SourceProvenanceError> {
    let listing = git_bytes(root, &["ls-files", "--stage", "-z"])?;
    let mut entries = Vec::new();
    for entry in listing
        .split(|byte| *byte == 0)
        .filter(|entry| !entry.is_empty())
    {
        let Some(tab) = entry.iter().position(|byte| *byte == b'\t') else {
            continue;
        };
        let metadata = &entry[..tab];
        let path = &entry[tab + 1..];
        let mut fields = metadata.split(|byte| *byte == b' ');
        let Some(mode) = fields.next() else {
            continue;
        };
        if mode != b"160000" {
            continue;
        }
        let Some(object) = fields.next() else {
            continue;
        };
        let object = std::str::from_utf8(object)
            .map_err(|_| SourceProvenanceError::NestedUnavailable)?
            .to_owned();
        let path =
            std::str::from_utf8(path).map_err(|_| SourceProvenanceError::NestedUnavailable)?;
        entries.push((PathBuf::from(path), object));
    }
    Ok(entries)
}

/// Tracks canonicalized repository roots and the inspected-repository count so
/// traversal is bounded and cannot revisit or cycle.
#[derive(Default)]
struct InspectionState {
    visited: BTreeSet<PathBuf>,
    repositories: usize,
}

/// Inspects one repository root and every populated nested Gitlink recursively.
///
/// `reserved` is true only for the owning inspected root, whose untracked or
/// ignored generated `.eqm/` output is neutral; nested repositories have no
/// reserved path. Concealment, a child `HEAD` that differs from its declared
/// Gitlink, an unavailable/uninitialized child and any traversal bound are typed
/// unavailable inspections, never clean.
fn inspect_repository(
    root: &Path,
    reserved: bool,
    depth: u32,
    state: &mut InspectionState,
) -> Result<bool, SourceProvenanceError> {
    if depth > MAX_NESTED_DEPTH || state.repositories >= MAX_NESTED_REPOSITORIES {
        return Err(SourceProvenanceError::NestedLimit);
    }
    state.repositories += 1;
    let canonical_root = root
        .canonicalize()
        .map_err(|_| SourceProvenanceError::NestedUnavailable)?;
    if !state.visited.insert(canonical_root) {
        return Err(SourceProvenanceError::NestedLimit);
    }
    if concealed_entries_present(root)? {
        return Err(if depth == 0 {
            SourceProvenanceError::ConcealedIndex
        } else {
            SourceProvenanceError::NestedConcealed
        });
    }
    let status = git_bytes(
        root,
        &[
            "status",
            "--porcelain=v1",
            "-z",
            "--untracked-files=all",
            "--ignored=matching",
            "--ignore-submodules=none",
        ],
    )?;
    let mut dirty = status_has_dirty_content(&status, reserved);
    for (path, declared) in gitlink_entries(root)? {
        let child = root.join(&path);
        let metadata = std::fs::symlink_metadata(&child)
            .map_err(|_| SourceProvenanceError::NestedUnavailable)?;
        if metadata.file_type().is_symlink() || !metadata.is_dir() {
            return Err(SourceProvenanceError::NestedUnavailable);
        }
        // A populated submodule is its own repository root; an empty or
        // uninitialized directory would otherwise resolve to the parent root.
        let top = git_stdout(&child, &["rev-parse", "--show-toplevel"])
            .map_err(|_| SourceProvenanceError::NestedUnavailable)?;
        let top = Path::new(&top)
            .canonicalize()
            .map_err(|_| SourceProvenanceError::NestedUnavailable)?;
        let canonical_child = child
            .canonicalize()
            .map_err(|_| SourceProvenanceError::NestedUnavailable)?;
        if top != canonical_child {
            return Err(SourceProvenanceError::NestedUnavailable);
        }
        let head = git_stdout(&child, &["rev-parse", "HEAD"])
            .map_err(|_| SourceProvenanceError::NestedUnavailable)?;
        if !head.eq_ignore_ascii_case(&declared) {
            return Err(SourceProvenanceError::NestedHeadMismatch);
        }
        if inspect_repository(&child, false, depth + 1, state)? {
            dirty = true;
        }
    }
    Ok(dirty)
}

/// Returns the exact inspected source-tree provenance for one repository root.
///
/// The inspection is repository-wide and conservative: it disables status
/// shortcuts (`--untracked-files=all`, `--ignored=matching`,
/// `--ignore-submodules=none`), parses NUL-delimited paths, recursively inspects
/// every populated nested Gitlink, rejects concealed index entries and treats
/// only untracked or ignored content below the owning root's `.eqm/` as neutral.
/// Missing or failed Git, unsupported identity, concealed entries, nested
/// concealment, Gitlink `HEAD` mismatch, unavailable nested inspection and
/// traversal bounds are typed errors that never default to clean. `dirty` is
/// true, and [`clean`] false, when any non-authorizing content is present.
pub fn inspect_source_provenance(root: &Path) -> Result<SourceProvenance, SourceProvenanceError> {
    let repository = repository_identity(root)?;
    let commit = git_stdout(root, &["rev-parse", "HEAD"])?;
    let source_commit: SourceCommit = commit.parse().map_err(SourceProvenanceError::Commit)?;
    let mut state = InspectionState::default();
    let dirty = inspect_repository(root, true, 0, &mut state)?;
    Ok(SourceProvenance {
        repository,
        source_commit,
        dirty,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn git(root: &Path, arguments: &[&str]) -> Result<(), Box<dyn Error>> {
        let status = Command::new("git")
            .args([
                "-c",
                "user.name=Fixture",
                "-c",
                "user.email=fixture@example.invalid",
            ])
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
        fs::write(root.join(".gitignore"), ".eqm/\ngenerated/\n")?;
        fs::write(root.join("tracked.txt"), "one\n")?;
        git(root, &["add", "."])?;
        git(root, &["commit", "-qm", "fixture"])?;
        Ok(directory)
    }

    /// Adds and commits one local submodule. Setup failure is returned, never
    /// silently treated as a passed regression.
    fn add_submodule(parent: &Path, source: &Path, name: &str) -> Result<(), Box<dyn Error>> {
        let source = source.to_str().ok_or("submodule source path")?;
        let status = Command::new("git")
            .args([
                "-c",
                "user.name=Fixture",
                "-c",
                "user.email=fixture@example.invalid",
                "-c",
                "protocol.file.allow=always",
                "submodule",
                "add",
                source,
                name,
            ])
            .current_dir(parent)
            .status()?;
        if !status.success() {
            return Err(format!("submodule add {name} failed").into());
        }
        git(parent, &["commit", "-qm", &format!("add submodule {name}")])?;
        Ok(())
    }

    fn update_submodules_recursive(root: &Path) -> Result<(), Box<dyn Error>> {
        let status = Command::new("git")
            .args([
                "-c",
                "user.name=Fixture",
                "-c",
                "user.email=fixture@example.invalid",
                "-c",
                "protocol.file.allow=always",
                "submodule",
                "update",
                "--init",
                "--recursive",
            ])
            .current_dir(root)
            .status()?;
        if !status.success() {
            return Err("recursive submodule update failed".into());
        }
        Ok(())
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

    #[test]
    fn reserved_output_tree_is_neutral_but_other_ignored_content_is_not()
    -> Result<(), Box<dyn Error>> {
        let directory = repository()?;
        let root = directory.path();
        // Untracked generated output under `.eqm/` is neutral.
        fs::create_dir_all(root.join(".eqm/results"))?;
        fs::write(root.join(".eqm/results/one.json"), "{}")?;
        assert!(inspect_source_provenance(root)?.clean());
        // A path with a newline under the reserved tree is still neutral, which
        // proves the NUL-delimited parser does not mis-split.
        fs::write(root.join(".eqm/odd\nname.json"), "{}")?;
        assert!(inspect_source_provenance(root)?.clean());
        // A near-miss prefix is not the reserved tree.
        fs::write(root.join(".eqm-other.txt"), "x")?;
        assert!(inspect_source_provenance(root)?.dirty());
        fs::remove_file(root.join(".eqm-other.txt"))?;
        // Ignored content outside the reserved tree is conservative dirty.
        fs::create_dir_all(root.join("generated"))?;
        fs::write(root.join("generated/source.txt"), "x")?;
        assert!(inspect_source_provenance(root)?.dirty());
        Ok(())
    }

    #[test]
    fn tracked_staged_deleted_and_untracked_changes_are_dirty() -> Result<(), Box<dyn Error>> {
        let directory = repository()?;
        let root = directory.path();
        // Tracked worktree modification.
        fs::write(root.join("tracked.txt"), "two\n")?;
        assert!(inspect_source_provenance(root)?.dirty());
        // Staged change.
        git(root, &["add", "tracked.txt"])?;
        assert!(inspect_source_provenance(root)?.dirty());
        git(root, &["reset", "-q", "--hard", "HEAD"])?;
        assert!(inspect_source_provenance(root)?.clean());
        // Deletion.
        fs::remove_file(root.join("tracked.txt"))?;
        assert!(inspect_source_provenance(root)?.dirty());
        git(root, &["checkout", "--", "tracked.txt"])?;
        // Non-output untracked file.
        fs::write(root.join("untracked.txt"), "new\n")?;
        assert!(inspect_source_provenance(root)?.dirty());
        Ok(())
    }

    #[test]
    fn tracked_change_inside_reserved_output_tree_is_dirty() -> Result<(), Box<dyn Error>> {
        let directory = repository()?;
        let root = directory.path();
        fs::create_dir_all(root.join(".eqm"))?;
        fs::write(root.join(".eqm/tracked.json"), "one")?;
        git(root, &["add", "-f", ".eqm/tracked.json"])?;
        git(root, &["commit", "-qm", "tracked output"])?;
        assert!(inspect_source_provenance(root)?.clean());
        fs::write(root.join(".eqm/tracked.json"), "two")?;
        assert!(inspect_source_provenance(root)?.dirty());
        Ok(())
    }

    #[test]
    fn populated_nested_repository_is_inspected_and_clean() -> Result<(), Box<dyn Error>> {
        let directory = repository()?;
        let root = directory.path();
        let child = repository()?;
        add_submodule(root, child.path(), "sub")?;
        assert!(inspect_source_provenance(root)?.clean());
        Ok(())
    }

    #[test]
    fn concealed_nested_modification_is_rejected_for_each_flag() -> Result<(), Box<dyn Error>> {
        for flag in ["--skip-worktree", "--assume-unchanged"] {
            let directory = repository()?;
            let root = directory.path();
            let child = repository()?;
            add_submodule(root, child.path(), "sub")?;
            let child_root = root.join("sub");
            fs::write(child_root.join("tracked.txt"), "two\n")?;
            git(&child_root, &["update-index", flag, "tracked.txt"])?;
            assert_eq!(
                inspect_source_provenance(root),
                Err(SourceProvenanceError::NestedConcealed),
                "{flag} must not conceal nested source"
            );
        }
        Ok(())
    }

    #[test]
    fn nested_untracked_ignored_and_output_content_is_not_neutral() -> Result<(), Box<dyn Error>> {
        let directory = repository()?;
        let root = directory.path();
        let child = repository()?;
        add_submodule(root, child.path(), "sub")?;
        assert!(inspect_source_provenance(root)?.clean());
        // Untracked nested source.
        fs::write(root.join("sub/untracked.txt"), "x\n")?;
        assert!(inspect_source_provenance(root)?.dirty());
        fs::remove_file(root.join("sub/untracked.txt"))?;
        // Ignored nested source is not concealed.
        fs::create_dir_all(root.join("sub/generated"))?;
        fs::write(root.join("sub/generated/source.txt"), "x\n")?;
        assert!(inspect_source_provenance(root)?.dirty());
        // The nested `.eqm/` tree is not the owning root's reserved output.
        fs::create_dir_all(root.join("sub/.eqm"))?;
        fs::write(root.join("sub/.eqm/one.json"), "{}")?;
        assert!(inspect_source_provenance(root)?.dirty());
        Ok(())
    }

    #[test]
    fn nested_head_mismatch_is_rejected() -> Result<(), Box<dyn Error>> {
        let directory = repository()?;
        let root = directory.path();
        let child = repository()?;
        fs::write(child.path().join("tracked.txt"), "two\n")?;
        git(child.path(), &["commit", "-qam", "second"])?;
        add_submodule(root, child.path(), "sub")?;
        // Move the populated child off its declared Gitlink commit.
        git(&root.join("sub"), &["checkout", "-q", "HEAD~1"])?;
        assert_eq!(
            inspect_source_provenance(root),
            Err(SourceProvenanceError::NestedHeadMismatch)
        );
        Ok(())
    }

    #[test]
    fn uninitialized_nested_gitlink_is_unavailable() -> Result<(), Box<dyn Error>> {
        let directory = repository()?;
        let root = directory.path();
        let head = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(root)
            .output()?;
        let head = String::from_utf8(head.stdout)?.trim().to_owned();
        git(
            root,
            &[
                "update-index",
                "--add",
                "--cacheinfo",
                &format!("160000,{head},sub"),
            ],
        )?;
        git(root, &["commit", "-qm", "add uninitialized gitlink"])?;
        assert_eq!(
            inspect_source_provenance(root),
            Err(SourceProvenanceError::NestedUnavailable)
        );
        Ok(())
    }

    #[test]
    fn nested_depth_is_traversed_and_concealed_grandchild_is_rejected() -> Result<(), Box<dyn Error>>
    {
        let directory = repository()?;
        let root = directory.path();
        let child = repository()?;
        let grandchild = repository()?;
        add_submodule(child.path(), grandchild.path(), "inner")?;
        add_submodule(root, child.path(), "sub")?;
        update_submodules_recursive(root)?;
        assert!(inspect_source_provenance(root)?.clean());
        let inner = root.join("sub/inner");
        fs::write(inner.join("tracked.txt"), "two\n")?;
        git(&inner, &["update-index", "--skip-worktree", "tracked.txt"])?;
        assert_eq!(
            inspect_source_provenance(root),
            Err(SourceProvenanceError::NestedConcealed)
        );
        Ok(())
    }

    #[test]
    fn nested_traversal_bounds_are_typed_unavailable() -> Result<(), Box<dyn Error>> {
        let directory = repository()?;
        let root = directory.path();
        let mut counted = InspectionState {
            visited: BTreeSet::new(),
            repositories: MAX_NESTED_REPOSITORIES,
        };
        assert_eq!(
            inspect_repository(root, true, 0, &mut counted),
            Err(SourceProvenanceError::NestedLimit)
        );
        let mut revisited = InspectionState {
            visited: BTreeSet::from([root.canonicalize()?]),
            repositories: 0,
        };
        assert_eq!(
            inspect_repository(root, true, 0, &mut revisited),
            Err(SourceProvenanceError::NestedLimit)
        );
        Ok(())
    }

    #[test]
    fn root_clean_dirty_and_unconfigured_controls_are_preserved() -> Result<(), Box<dyn Error>> {
        let directory = repository()?;
        let root = directory.path();
        assert!(inspect_source_provenance(root)?.clean());
        fs::write(root.join("tracked.txt"), "two\n")?;
        assert!(inspect_source_provenance(root)?.dirty());
        let no_remote = tempfile::tempdir()?;
        git(no_remote.path(), &["init", "-q"])?;
        assert_eq!(
            inspect_source_provenance(no_remote.path()),
            Err(SourceProvenanceError::GitFailed)
        );
        Ok(())
    }
}
