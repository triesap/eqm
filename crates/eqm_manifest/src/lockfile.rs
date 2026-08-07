//! Strict offline loading of exact import and adapter lock entries.

use crate::WorkspaceConfig;
use crate::dto::LockDto;
use crate::parse_toml;
use eqm_domain::{
    AdapterId, FragmentId, RepoPath, RepositoryIdentity, Revision, SchemaKind, SchemaUri,
    SelectorText, Sha256Digest, SourceCommit, SourceName, TrustLevel,
};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};

/// One exact imported fragment lock entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportLock {
    /// Imported fragment authority.
    pub id: FragmentId,
    /// Exact positive authority revision.
    pub revision: Revision,
    /// Canonical source repository.
    pub source: RepositoryIdentity,
    /// Immutable full source commit.
    pub resolved: SourceCommit,
    /// Exact imported semantic digest.
    pub digest: Sha256Digest,
    /// Optional declared trust floor.
    pub trust: Option<TrustLevel>,
    /// Optional detached signature metadata.
    pub signature: Option<SelectorText>,
}

/// One exact out-of-process adapter lock entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterLock {
    /// Adapter authority.
    pub id: AdapterId,
    /// Exact immutable adapter version.
    pub version: SelectorText,
    /// Canonical source repository.
    pub source: RepositoryIdentity,
    /// Immutable full source commit.
    pub resolved: SourceCommit,
    /// Exact executable digest.
    pub digest: Sha256Digest,
    /// Exact adapter protocol revision.
    pub protocol: Revision,
    /// Optional declared trust floor.
    pub trust: Option<TrustLevel>,
    /// Optional detached signature metadata.
    pub signature: Option<SelectorText>,
}

/// A validated singleton v1 lockfile.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorkspaceLock {
    imports: BTreeMap<FragmentId, ImportLock>,
    adapters: BTreeMap<AdapterId, AdapterLock>,
}

impl WorkspaceLock {
    /// Returns imports in fragment-ID order.
    #[must_use]
    pub const fn imports(&self) -> &BTreeMap<FragmentId, ImportLock> {
        &self.imports
    }
    /// Returns adapters in adapter-ID order.
    #[must_use]
    pub const fn adapters(&self) -> &BTreeMap<AdapterId, AdapterLock> {
        &self.adapters
    }
}

/// Loads the configured lockfile without any network access.
pub fn load_lockfile(config: &WorkspaceConfig) -> Result<WorkspaceLock, LockError> {
    let relative = RepoPath::new(config.dto().lockfile.as_deref().unwrap_or("eqm.lock"))
        .map_err(|_| LockError::InvalidPath)?;
    let path = config.repository_root().join(relative.as_str());
    reject_symlink_components(config.repository_root(), &path)?;
    let bytes = fs::read(&path).map_err(|_| LockError::Unavailable)?;
    let parsed = parse_toml(
        SourceName::new(relative.as_str()).map_err(|_| LockError::InvalidPath)?,
        &bytes,
    )
    .map_err(|_| LockError::InvalidToml)?;
    let dto: LockDto = toml::from_str(parsed.text()).map_err(|_| LockError::InvalidFields)?;
    let schema: SchemaUri = dto.schema.parse().map_err(|_| LockError::WrongSchema)?;
    if schema.kind() != SchemaKind::Lock || dto.version != 1 {
        return Err(LockError::WrongSchema);
    }

    let mut imports = BTreeMap::new();
    for entry in dto.imports {
        let lock = ImportLock {
            id: entry.id.parse().map_err(|_| LockError::InvalidImport)?,
            revision: Revision::new(entry.revision).map_err(|_| LockError::InvalidImport)?,
            source: entry.source.parse().map_err(|_| LockError::InvalidImport)?,
            resolved: entry
                .resolved
                .parse()
                .map_err(|_| LockError::FloatingReference)?,
            digest: entry.digest.parse().map_err(|_| LockError::InvalidDigest)?,
            trust: entry
                .trust
                .as_deref()
                .map(str::parse)
                .transpose()
                .map_err(|_| LockError::InvalidTrust)?,
            signature: optional_text(entry.signature, LockError::InvalidSignature)?,
        };
        if imports.insert(lock.id.clone(), lock).is_some() {
            return Err(LockError::DuplicateImport);
        }
    }

    let mut adapters = BTreeMap::new();
    for entry in dto.adapters {
        if !immutable_version(&entry.version) {
            return Err(LockError::FloatingReference);
        }
        let protocol = Revision::new(entry.protocol).map_err(|_| LockError::InvalidAdapter)?;
        if protocol.get() != 1 {
            return Err(LockError::InvalidAdapter);
        }
        let lock = AdapterLock {
            id: entry.id.parse().map_err(|_| LockError::InvalidAdapter)?,
            version: SelectorText::new(entry.version).map_err(|_| LockError::InvalidAdapter)?,
            source: entry
                .source
                .parse()
                .map_err(|_| LockError::InvalidAdapter)?,
            resolved: entry
                .resolved
                .parse()
                .map_err(|_| LockError::FloatingReference)?,
            digest: entry.digest.parse().map_err(|_| LockError::InvalidDigest)?,
            protocol,
            trust: entry
                .trust
                .as_deref()
                .map(str::parse)
                .transpose()
                .map_err(|_| LockError::InvalidTrust)?,
            signature: optional_text(entry.signature, LockError::InvalidSignature)?,
        };
        if adapters.insert(lock.id.clone(), lock).is_some() {
            return Err(LockError::DuplicateAdapter);
        }
    }
    Ok(WorkspaceLock { imports, adapters })
}

fn optional_text(
    value: Option<String>,
    error: LockError,
) -> Result<Option<SelectorText>, LockError> {
    value
        .map(|value| SelectorText::new(value).map_err(|_| error))
        .transpose()
}

fn immutable_version(value: &str) -> bool {
    !value.is_empty()
        && !value.chars().any(char::is_whitespace)
        && !value.contains(['*', '^', '~', '>', '<'])
        && !matches!(
            value.to_ascii_lowercase().as_str(),
            "main" | "master" | "head" | "latest"
        )
        && !value.starts_with("refs/heads/")
}

fn reject_symlink_components(root: &Path, path: &Path) -> Result<(), LockError> {
    let relative = path
        .strip_prefix(root)
        .map_err(|_| LockError::InvalidPath)?;
    let mut current = PathBuf::from(root);
    for component in relative.components() {
        current.push(component);
        if fs::symlink_metadata(&current)
            .map_err(|_| LockError::Unavailable)?
            .file_type()
            .is_symlink()
        {
            return Err(LockError::Symlink);
        }
    }
    Ok(())
}

/// Strict lockfile loading failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LockError {
    /// Lockfile path was not a valid repository path.
    InvalidPath,
    /// Lockfile was unavailable.
    Unavailable,
    /// Lockfile or one of its path components was a symlink.
    Symlink,
    /// Lockfile TOML was invalid or exceeded its bound.
    InvalidToml,
    /// Strict lockfile fields were malformed.
    InvalidFields,
    /// Schema URI or lock revision was not exact current v1.
    WrongSchema,
    /// Import entry identity was invalid.
    InvalidImport,
    /// Adapter entry identity or protocol was invalid.
    InvalidAdapter,
    /// A resolved identity or adapter version was floating.
    FloatingReference,
    /// Digest did not have exact SHA-256 shape.
    InvalidDigest,
    /// Trust metadata was outside the closed vocabulary.
    InvalidTrust,
    /// Signature metadata was malformed or oversized.
    InvalidSignature,
    /// Import ID appeared more than once.
    DuplicateImport,
    /// Adapter ID appeared more than once.
    DuplicateAdapter,
}

impl Display for LockError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for LockError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::select_workspace_config;

    const CONFIG: &str = r#"schema = "https://schemas.equivalencematrix.dev/v1/workspace"
contract_sources = ["eqm/contracts/**/*.toml"]
binding_sources = ["eqm/bindings/**/*.toml"]
policy_sources = ["eqm/policies/**/*.toml"]
profile_sources = ["eqm/profiles/**/*.toml"]
runner_sources = ["eqm/runners/**/*.toml"]
waiver_sources = ["eqm/waivers/**/*.toml"]
"#;
    const LOCK: &str = r#"schema = "https://schemas.equivalencematrix.dev/v1/lock"
version = 1

[[imports]]
id = "common.form"
revision = 1
source = "https://github.com/example/contracts"
resolved = "0123456789012345678901234567890123456789"
digest = "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
trust = "signed_ci"

[[adapters]]
id = "adapter.web"
version = "1.2.3"
source = "https://github.com/example/adapter"
resolved = "abcdefabcdefabcdefabcdefabcdefabcdefabcd"
digest = "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
protocol = 1
"#;

    fn repository(lock: &str) -> Result<tempfile::TempDir, Box<dyn Error>> {
        let directory = tempfile::tempdir()?;
        fs::create_dir(directory.path().join(".git"))?;
        fs::write(directory.path().join("eqm.toml"), CONFIG)?;
        fs::write(directory.path().join("eqm.lock"), lock)?;
        Ok(directory)
    }

    fn load(lock: &str) -> Result<WorkspaceLock, LockError> {
        let repository = repository(lock).map_err(|_| LockError::Unavailable)?;
        let config =
            select_workspace_config(repository.path(), None).map_err(|_| LockError::Unavailable)?;
        load_lockfile(&config)
    }

    #[test]
    fn exact_imports_and_adapters_load_in_key_order() -> Result<(), Box<dyn Error>> {
        let repository = repository(LOCK)?;
        let config = select_workspace_config(repository.path(), None)?;
        let lock = load_lockfile(&config)?;
        assert_eq!(lock.imports().len(), 1);
        assert_eq!(lock.adapters().len(), 1);
        Ok(())
    }

    #[test]
    fn floating_duplicate_digest_and_old_schema_fail_closed() {
        assert_eq!(
            load(&LOCK.replace("1.2.3", "latest")),
            Err(LockError::FloatingReference)
        );
        let duplicate = LOCK.replace(
            "[[adapters]]",
            r#"[[imports]]
id = "common.form"
revision = 1
source = "https://github.com/example/contracts"
resolved = "0123456789012345678901234567890123456789"
digest = "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"

[[adapters]]"#,
        );
        assert_eq!(load(&duplicate), Err(LockError::DuplicateImport));
        assert_eq!(
            load(&LOCK.replace(
                "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                "sha256:bad"
            )),
            Err(LockError::InvalidDigest)
        );
        assert_eq!(
            load(&LOCK.replace("/v1/lock", "/v0/lock")),
            Err(LockError::WrongSchema)
        );
    }

    #[cfg(unix)]
    #[test]
    fn symlink_lockfile_is_rejected() -> Result<(), Box<dyn Error>> {
        let repository = repository(LOCK)?;
        fs::rename(
            repository.path().join("eqm.lock"),
            repository.path().join("real.lock"),
        )?;
        std::os::unix::fs::symlink(
            repository.path().join("real.lock"),
            repository.path().join("eqm.lock"),
        )?;
        let config = select_workspace_config(repository.path(), None)?;
        assert_eq!(load_lockfile(&config), Err(LockError::Symlink));
        Ok(())
    }
}
