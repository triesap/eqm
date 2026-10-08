//! Read-only structure preparation, independent of authored artifact roles.

use crate::PreparedSession;
use eqm_domain::{ArtifactRole, BindingId, RepoPath};
use eqm_engine::{
    RepositoryEntry, RepositoryEntryKind, RepositoryView, StructureReport,
    evaluate_structure_with_roots,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, Metadata};
use std::path::{Component, Path, PathBuf};
use std::time::SystemTime;

/// Complete preparation rejects rather than truncates above this path bound.
pub const MAX_STRUCTURAL_PATHS: usize = 100_000;

/// Independently inspected input to an explicitly configured role authority.
/// It contains no binding, artifact declaration or requested role.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoleObservationRequest {
    root: PathBuf,
    path: RepoPath,
    canonical_path: PathBuf,
    kind: RepositoryEntryKind,
}

impl RoleObservationRequest {
    /// Canonical owning repository root.
    #[must_use]
    pub fn repository_root(&self) -> &Path {
        &self.root
    }

    /// Actual repository-relative observed path.
    #[must_use]
    pub const fn path(&self) -> &RepoPath {
        &self.path
    }

    /// Canonical confined path observed before the adapter call.
    #[must_use]
    pub fn canonical_path(&self) -> &Path {
        &self.canonical_path
    }

    /// Independently observed file kind.
    #[must_use]
    pub const fn kind(&self) -> RepositoryEntryKind {
        self.kind
    }
}

/// A configured observer's independently supported roles for one exact path.
/// Parsing producer claims is insufficient to implement this authority port.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoleObservation {
    path: RepoPath,
    roles: BTreeSet<ArtifactRole>,
}

impl RoleObservation {
    /// Returns roles independently established by the configured observer.
    #[must_use]
    pub const fn new(path: RepoPath, roles: BTreeSet<ArtifactRole>) -> Self {
        Self { path, roles }
    }
}

/// Independent role authority could not establish an observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RoleObservationError {
    /// Missing, stale, untrusted, unsupported or failed role evidence.
    Unavailable,
}

impl Display for RoleObservationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("independent role observation is unavailable")
    }
}

impl Error for RoleObservationError {}

/// Explicit read-only role authority, without execution or implicit fetching.
/// Implementations must independently validate observations for the inspected
/// file; copied authored roles and imported verified flags are not authority.
/// This port does not establish protected release or cryptographic acceptance.
pub trait StructuralRoleObserver {
    /// Establishes independently supported roles, or reports unavailable input.
    fn observe_roles(
        &self,
        request: &RoleObservationRequest,
    ) -> Result<RoleObservation, RoleObservationError>;
}

/// Non-authorizing failure of complete read-only filesystem preparation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StructurePreparationError {
    /// Root inspection failed or no longer matches the prepared canonical root.
    RootUnavailable,
    /// Inspection failed for a reason other than an actual missing path.
    Inspection(RepoPath),
    /// The inspected path or metadata changed across an observer call.
    Changed(RepoPath),
    /// Complete path preparation exceeded its bound; no partial report is returned.
    LimitExceeded,
    /// An invariant-checked binding unexpectedly lacked its ownership closure.
    MissingOwnership(BindingId),
}

impl Display for StructurePreparationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "structural preparation failed: {self:?}")
    }
}

impl Error for StructurePreparationError {}

/// Complete observations/reports prepared from one immutable session graph.
/// There is no deserialization or public constructor granting this boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedStructuralObservations {
    observations: RepositoryView,
    reports: BTreeMap<BindingId, StructureReport>,
}

impl PreparedStructuralObservations {
    /// Independent file/role observations, retained alongside all findings.
    #[must_use]
    pub const fn observations(&self) -> &RepositoryView {
        &self.observations
    }

    /// Every binding's structural result, before any transport filter.
    #[must_use]
    pub const fn reports(&self) -> &BTreeMap<BindingId, StructureReport> {
        &self.reports
    }
}

/// Prepares all structural observations without loading authority again,
/// executing a runner, fetching content or echoing authored artifact roles.
/// Pathname metadata checks detect changes but do not prove immutable,
/// race-safe acquisition. No artifact payload is opened or executed here.
pub fn prepare_structural_observations(
    session: &PreparedSession,
    observer: Option<&dyn StructuralRoleObserver>,
) -> Result<PreparedStructuralObservations, StructurePreparationError> {
    let root = session.repository_root();
    check_root(root)?;
    let graph = session.finalized().graph();
    let mut paths = BTreeSet::new();
    for binding in graph.bindings().values() {
        for artifact in binding.artifacts().values().values() {
            paths.insert(artifact.path().clone());
            if paths.len() > MAX_STRUCTURAL_PATHS {
                return Err(StructurePreparationError::LimitExceeded);
            }
        }
    }
    let mut observations = RepositoryView::new();
    for path in paths {
        let Some(before) = inspect(root, &path)? else {
            continue;
        };
        let mut roles = None;
        if before.kind == RepositoryEntryKind::File
            && let Some(observer) = observer
        {
            let request = RoleObservationRequest {
                root: root.to_path_buf(),
                path: path.clone(),
                canonical_path: before.canonical_path.clone(),
                kind: before.kind,
            };
            if let Ok(observation) = observer.observe_roles(&request)
                && observation.path == path
            {
                roles = Some(observation.roles);
            }
        }
        if inspect(root, &path)?.as_ref() != Some(&before) {
            return Err(StructurePreparationError::Changed(path));
        }
        observations.insert(
            path,
            RepositoryEntry {
                kind: before.kind,
                resolved: None,
                roles,
            },
        );
    }
    check_root(root)?;
    let mut reports = BTreeMap::new();
    for binding in graph.bindings().values() {
        let roots = graph
            .owned_roots(binding.target())
            .ok_or_else(|| StructurePreparationError::MissingOwnership(binding.id().clone()))?;
        reports.insert(
            binding.id().clone(),
            evaluate_structure_with_roots(binding, &roots, &observations, false),
        );
    }
    Ok(PreparedStructuralObservations {
        observations,
        reports,
    })
}

fn check_root(root: &Path) -> Result<(), StructurePreparationError> {
    let metadata =
        fs::symlink_metadata(root).map_err(|_| StructurePreparationError::RootUnavailable)?;
    if !metadata.is_dir()
        || metadata.file_type().is_symlink()
        || fs::canonicalize(root).ok().as_deref() != Some(root)
    {
        return Err(StructurePreparationError::RootUnavailable);
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Inspection {
    kind: RepositoryEntryKind,
    canonical_path: PathBuf,
    components: Vec<MetadataStamp>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MetadataStamp {
    length: u64,
    modified: Option<SystemTime>,
    created: Option<SystemTime>,
    #[cfg(unix)]
    device_inode: (u64, u64),
}

impl MetadataStamp {
    fn new(metadata: &Metadata) -> Self {
        Self {
            length: metadata.len(),
            modified: metadata.modified().ok(),
            created: metadata.created().ok(),
            #[cfg(unix)]
            device_inode: {
                use std::os::unix::fs::MetadataExt;
                (metadata.dev(), metadata.ino())
            },
        }
    }
}

fn inspect(root: &Path, path: &RepoPath) -> Result<Option<Inspection>, StructurePreparationError> {
    let mut current = root.to_path_buf();
    let mut components = Vec::new();
    let mut iterator = Path::new(path.as_str()).components().peekable();
    while let Some(component) = iterator.next() {
        let Component::Normal(name) = component else {
            return Err(StructurePreparationError::Inspection(path.clone()));
        };
        current.push(name);
        let metadata = match fs::symlink_metadata(&current) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(_) => return Err(StructurePreparationError::Inspection(path.clone())),
        };
        components.push(MetadataStamp::new(&metadata));
        let kind = if metadata.file_type().is_symlink() {
            RepositoryEntryKind::Symlink
        } else if metadata.is_dir() {
            RepositoryEntryKind::Directory
        } else if metadata.is_file() {
            RepositoryEntryKind::File
        } else {
            RepositoryEntryKind::Other
        };
        if iterator.peek().is_none() || kind != RepositoryEntryKind::Directory {
            let canonical_path = if kind == RepositoryEntryKind::File && iterator.peek().is_none() {
                let canonical = fs::canonicalize(&current)
                    .map_err(|_| StructurePreparationError::Inspection(path.clone()))?;
                if !canonical.starts_with(root) || canonical != current {
                    return Err(StructurePreparationError::Inspection(path.clone()));
                }
                canonical
            } else {
                current
            };
            // A regular file used as an ancestor is not a file at the requested path.
            let kind = if iterator.peek().is_some() && kind == RepositoryEntryKind::File {
                RepositoryEntryKind::Other
            } else {
                kind
            };
            return Ok(Some(Inspection {
                kind,
                canonical_path,
                components,
            }));
        }
    }
    Err(StructurePreparationError::Inspection(path.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SessionRequest, prepare};
    use eqm_engine::StructureFindingKind;
    use std::cell::Cell;

    const ANDROID: &str = "apps/android/app/src/main/java/com/example/signup/SignupScreen.kt";
    const IOS: &str = "apps/ios/Signup/Sources/SignupView.swift";

    fn copy_tree(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(destination)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                copy_tree(&entry.path(), &destination.join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), destination.join(entry.file_name()))?;
            }
        }
        Ok(())
    }

    fn fixture(service: bool) -> Result<(tempfile::TempDir, PreparedSession), Box<dyn Error>> {
        let directory = tempfile::tempdir()?;
        copy_tree(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/android-ios"),
            directory.path(),
        )?;
        fs::create_dir(directory.path().join(".git"))?;
        for path in [ANDROID, IOS] {
            let absolute = directory.path().join(path);
            fs::create_dir_all(absolute.parent().ok_or("parent")?)?;
            fs::write(absolute, b"independent fixture file\n")?;
        }
        if service {
            let binding = directory
                .path()
                .join("eqm/bindings/android.auth_signup.toml");
            fs::write(
                &binding,
                fs::read_to_string(&binding)?.replace("role = \"view\"", "role = \"service\""),
            )?;
        }
        let session = prepare(
            &SessionRequest::new(None, "2026-01-02T03:04:05Z".parse()?),
            directory.path(),
        )?;
        Ok((directory, session))
    }

    struct Observer {
        android: ArtifactRole,
        calls: Cell<usize>,
        foreign: bool,
        unavailable: bool,
    }

    impl StructuralRoleObserver for Observer {
        fn observe_roles(
            &self,
            request: &RoleObservationRequest,
        ) -> Result<RoleObservation, RoleObservationError> {
            self.calls.set(self.calls.get() + 1);
            if self.unavailable {
                return Err(RoleObservationError::Unavailable);
            }
            let role = if request.path().as_str() == ANDROID {
                self.android
            } else {
                ArtifactRole::View
            };
            Ok(RoleObservation::new(
                if self.foreign {
                    RepoPath::new(IOS).map_err(|_| RoleObservationError::Unavailable)?
                } else {
                    request.path().clone()
                },
                BTreeSet::from([role]),
            ))
        }
    }

    fn kinds(
        prepared: &PreparedStructuralObservations,
        binding: &str,
    ) -> Result<BTreeSet<StructureFindingKind>, Box<dyn Error>> {
        Ok(prepared
            .reports()
            .get(&binding.parse()?)
            .ok_or("report")?
            .findings
            .iter()
            .map(|finding| finding.kind)
            .collect())
    }

    #[test]
    fn authored_service_needs_independent_roles_and_mismatch_is_distinct()
    -> Result<(), Box<dyn Error>> {
        let (_directory, session) = fixture(true)?;
        let missing = prepare_structural_observations(&session, None)?;
        assert_eq!(
            kinds(&missing, "binding.android.auth_signup")?,
            BTreeSet::from([StructureFindingKind::RoleUnknown])
        );
        assert!(!missing.reports().values().any(StructureReport::satisfied));
        let observer = Observer {
            android: ArtifactRole::Service,
            calls: Cell::new(0),
            foreign: false,
            unavailable: false,
        };
        let matched = prepare_structural_observations(&session, Some(&observer))?;
        assert!(matched.reports().values().all(StructureReport::satisfied));
        assert_eq!(observer.calls.get(), 2);
        let observer = Observer {
            android: ArtifactRole::View,
            ..observer
        };
        let mismatch = prepare_structural_observations(&session, Some(&observer))?;
        assert_eq!(
            kinds(&mismatch, "binding.android.auth_signup")?,
            BTreeSet::from([StructureFindingKind::RoleMismatch])
        );
        assert_eq!(
            matched.observations().keys().collect::<Vec<_>>(),
            mismatch.observations().keys().collect::<Vec<_>>()
        );
        Ok(())
    }

    #[test]
    fn failed_or_foreign_observer_cannot_authorize_roles() -> Result<(), Box<dyn Error>> {
        let (_directory, session) = fixture(true)?;
        for (foreign, unavailable) in [(true, false), (false, true)] {
            let observer = Observer {
                android: ArtifactRole::Service,
                calls: Cell::new(0),
                foreign,
                unavailable,
            };
            let prepared = prepare_structural_observations(&session, Some(&observer))?;
            assert_eq!(
                kinds(&prepared, "binding.android.auth_signup")?,
                BTreeSet::from([StructureFindingKind::RoleUnknown])
            );
        }
        Ok(())
    }

    #[test]
    fn missing_path_is_not_role_success_and_observer_changes_reject() -> Result<(), Box<dyn Error>>
    {
        let (_directory, session) = fixture(false)?;
        fs::remove_file(session.repository_root().join(ANDROID))?;
        let missing = prepare_structural_observations(&session, None)?;
        assert_eq!(
            kinds(&missing, "binding.android.auth_signup")?,
            BTreeSet::from([StructureFindingKind::Missing])
        );
        fs::write(session.repository_root().join(ANDROID), b"short")?;
        struct Mutating;
        impl StructuralRoleObserver for Mutating {
            fn observe_roles(
                &self,
                request: &RoleObservationRequest,
            ) -> Result<RoleObservation, RoleObservationError> {
                fs::write(request.canonical_path(), b"different-length contents")
                    .map_err(|_| RoleObservationError::Unavailable)?;
                Ok(RoleObservation::new(
                    request.path().clone(),
                    BTreeSet::from([ArtifactRole::View]),
                ))
            }
        }
        assert_eq!(
            prepare_structural_observations(&session, Some(&Mutating)),
            Err(StructurePreparationError::Changed(RepoPath::new(ANDROID)?))
        );
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn leaf_symlinks_and_regular_ancestors_are_blocking_without_role_calls()
    -> Result<(), Box<dyn Error>> {
        use std::os::unix::fs::symlink;
        let (_directory, session) = fixture(false)?;
        let external = tempfile::tempdir()?;
        fs::write(external.path().join("external"), b"outside")?;
        let path = session.repository_root().join(ANDROID);
        for destination in [
            session.repository_root().join(IOS),
            external.path().join("external"),
            external.path().join("missing"),
        ] {
            fs::remove_file(&path)?;
            symlink(destination, &path)?;
            let observer = Observer {
                android: ArtifactRole::View,
                calls: Cell::new(0),
                foreign: false,
                unavailable: false,
            };
            let prepared = prepare_structural_observations(&session, Some(&observer))?;
            let findings = kinds(&prepared, "binding.android.auth_signup")?;
            assert!(findings.contains(&StructureFindingKind::InvalidSymlink));
            assert!(!findings.contains(&StructureFindingKind::Missing));
            assert_eq!(observer.calls.get(), 1);
        }
        fs::remove_file(&path)?;
        let parent = path.parent().ok_or("parent")?;
        fs::remove_dir(parent)?;
        fs::write(parent, b"regular ancestor")?;
        let observer = Observer {
            android: ArtifactRole::View,
            calls: Cell::new(0),
            foreign: false,
            unavailable: false,
        };
        let prepared = prepare_structural_observations(&session, Some(&observer))?;
        let findings = kinds(&prepared, "binding.android.auth_signup")?;
        assert!(findings.contains(&StructureFindingKind::PathTypeMismatch));
        assert!(!findings.contains(&StructureFindingKind::Missing));
        assert_eq!(observer.calls.get(), 1);
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn actual_inspection_error_cannot_be_reported_as_absence() -> Result<(), Box<dyn Error>> {
        let (directory, session) = fixture(false)?;
        // A valid semantic path can exceed the host's per-component limit.
        let too_long = format!("apps/android/{}", "a".repeat(512));
        let path = RepoPath::new(too_long.as_str())?;
        let error = fs::symlink_metadata(session.repository_root().join(&too_long))
            .err()
            .ok_or("host unexpectedly admitted the overlong component")?;
        assert_ne!(error.kind(), std::io::ErrorKind::NotFound);
        let binding = directory
            .path()
            .join("eqm/bindings/android.auth_signup.toml");
        fs::write(
            &binding,
            fs::read_to_string(&binding)?.replace(ANDROID, &too_long),
        )?;
        let session = prepare(
            &SessionRequest::new(None, "2026-01-02T03:04:05Z".parse()?),
            directory.path(),
        )?;
        assert_eq!(
            prepare_structural_observations(&session, None),
            Err(StructurePreparationError::Inspection(path))
        );
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn symlink_ancestor_and_special_file_never_reach_role_observer() -> Result<(), Box<dyn Error>> {
        use std::os::unix::{fs::symlink, net::UnixListener};
        let (_directory, session) = fixture(false)?;
        let outside = tempfile::tempdir()?;
        fs::write(outside.path().join("SignupScreen.kt"), b"external bytes")?;
        let parent = session
            .repository_root()
            .join(ANDROID)
            .parent()
            .ok_or("parent")?
            .to_path_buf();
        fs::remove_dir_all(&parent)?;
        symlink(outside.path(), &parent)?;
        let observer = Observer {
            android: ArtifactRole::View,
            calls: Cell::new(0),
            foreign: false,
            unavailable: false,
        };
        let prepared = prepare_structural_observations(&session, Some(&observer))?;
        assert!(
            kinds(&prepared, "binding.android.auth_signup")?
                .contains(&StructureFindingKind::InvalidSymlink)
        );
        assert_eq!(
            observer.calls.get(),
            1,
            "only the independent iOS file may reach the observer"
        );
        fs::remove_file(&parent)?;
        fs::create_dir_all(&parent)?;
        // Bind at a short path within the host socket-address bound, then move
        // the existing socket node to the actual declared artifact path.
        let socket_path = outside.path().join("socket");
        let socket = UnixListener::bind(&socket_path)?;
        fs::rename(socket_path, session.repository_root().join(ANDROID))?;
        let prepared = prepare_structural_observations(&session, None)?;
        assert!(
            kinds(&prepared, "binding.android.auth_signup")?
                .contains(&StructureFindingKind::PathTypeMismatch)
        );
        assert_eq!(
            prepared.observations()[&RepoPath::new(ANDROID)?].kind,
            RepositoryEntryKind::Other
        );
        drop(socket);
        Ok(())
    }
}
