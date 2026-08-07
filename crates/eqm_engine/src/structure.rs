//! Non-executing structural evaluation over an injected repository view.

use eqm_domain::{ArtifactId, ArtifactRole, Binding, RepoPath, Target};
use std::collections::{BTreeMap, BTreeSet};

/// Injected path kind after race-safe repository inspection by an outer layer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RepositoryEntryKind {
    /// Regular file.
    File,
    /// Directory, invalid for a concrete artifact declaration.
    Directory,
    /// Read-only file symlink with an injected fully resolved path.
    Symlink,
}

/// One prepared repository observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepositoryEntry {
    /// Observed path kind.
    pub kind: RepositoryEntryKind,
    /// Fully resolved repository path for a symlink.
    pub resolved: Option<RepoPath>,
    /// Semantically verified roles supplied by structural adapters.
    pub roles: BTreeSet<ArtifactRole>,
}

/// Complete prepared repository path view.
pub type RepositoryView = BTreeMap<RepoPath, RepositoryEntry>;

/// Closed structural failure class.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum StructureFindingKind {
    /// Declared artifact path was absent.
    Missing,
    /// Path was a directory rather than a file artifact.
    PathTypeMismatch,
    /// Declared or resolved path escaped the target root.
    OutsideTargetRoot,
    /// Symlink use was forbidden or lacked a resolved target.
    InvalidSymlink,
    /// Injected semantic roles did not include the declared role.
    RoleMismatch,
    /// Two repository paths collide under portable comparison.
    PortableCollision,
}

/// One stable artifact-linked structural finding.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct StructureFinding {
    /// Artifact ID.
    pub artifact: ArtifactId,
    /// Failure class.
    pub kind: StructureFindingKind,
}

/// Complete deterministic structural result.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StructureReport {
    /// Every finding in artifact/kind order.
    pub findings: BTreeSet<StructureFinding>,
}

impl StructureReport {
    /// Returns true only when every declared artifact check passed.
    #[must_use]
    pub fn satisfied(&self) -> bool {
        self.findings.is_empty()
    }
}

/// Evaluates binding artifacts without filesystem access.
#[must_use]
pub fn evaluate_structure(
    binding: &Binding,
    target: &Target,
    view: &RepositoryView,
    allow_artifact_symlinks: bool,
) -> StructureReport {
    let collisions = portable_collisions(view);
    let mut findings = BTreeSet::new();
    for artifact in binding.artifacts().values().values() {
        let path = artifact.path();
        if !inside(path, target.root()) {
            findings.insert(StructureFinding {
                artifact: artifact.id().clone(),
                kind: StructureFindingKind::OutsideTargetRoot,
            });
        }
        if collisions.contains(path) {
            findings.insert(StructureFinding {
                artifact: artifact.id().clone(),
                kind: StructureFindingKind::PortableCollision,
            });
        }
        let Some(entry) = view.get(path) else {
            findings.insert(StructureFinding {
                artifact: artifact.id().clone(),
                kind: StructureFindingKind::Missing,
            });
            continue;
        };
        match entry.kind {
            RepositoryEntryKind::Directory => {
                findings.insert(StructureFinding {
                    artifact: artifact.id().clone(),
                    kind: StructureFindingKind::PathTypeMismatch,
                });
            }
            RepositoryEntryKind::Symlink => {
                let valid = allow_artifact_symlinks
                    && entry
                        .resolved
                        .as_ref()
                        .is_some_and(|resolved| inside(resolved, target.root()));
                if !valid {
                    findings.insert(StructureFinding {
                        artifact: artifact.id().clone(),
                        kind: StructureFindingKind::InvalidSymlink,
                    });
                }
            }
            RepositoryEntryKind::File => {}
        }
        if !entry.roles.contains(&artifact.role()) {
            findings.insert(StructureFinding {
                artifact: artifact.id().clone(),
                kind: StructureFindingKind::RoleMismatch,
            });
        }
    }
    StructureReport { findings }
}

fn inside(path: &RepoPath, root: &RepoPath) -> bool {
    path.as_str() == root.as_str()
        || path
            .as_str()
            .strip_prefix(root.as_str())
            .is_some_and(|suffix| suffix.starts_with('/'))
}

fn portable_collisions(view: &RepositoryView) -> BTreeSet<RepoPath> {
    let mut grouped: BTreeMap<_, Vec<_>> = BTreeMap::new();
    for path in view.keys() {
        grouped
            .entry(path.portable_collision_key())
            .or_default()
            .push(path.clone());
    }
    grouped
        .into_values()
        .filter(|paths| paths.len() > 1)
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn confinement_uses_complete_path_segments() -> Result<(), Box<dyn Error>> {
        let root = RepoPath::new("apps/web")?;
        assert!(inside(&RepoPath::new("apps/web/src/main.rs")?, &root));
        assert!(inside(&RepoPath::new("apps/web")?, &root));
        assert!(!inside(&RepoPath::new("apps/web-old/main.rs")?, &root));
        assert!(!inside(&RepoPath::new("apps/ios/main.rs")?, &root));
        Ok(())
    }
}
