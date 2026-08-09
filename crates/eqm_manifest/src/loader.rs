//! End-to-end authored workspace loading into unresolved graph input.

use crate::conversion::extensions;
use crate::{
    ContractEntity, DocumentDto, WorkspaceLock, convert_binding, convert_contract, convert_policy,
    convert_profile, convert_runner, convert_waiver, decode_sources, discover_sources,
    load_lockfile, select_workspace_config,
};
use eqm_domain::{
    AdapterLockIdentity, ImportLockIdentity, OwnerRef, RepoPath, Target, TargetId, TrustLevel,
    WorkspaceGraphInput,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::path::{Path, PathBuf};

/// Deterministically loaded workspace authority before engine resolution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoadedWorkspace {
    repository_root: PathBuf,
    graph_input: WorkspaceGraphInput,
    source_map: BTreeMap<Box<str>, RepoPath>,
}

impl LoadedWorkspace {
    /// Returns the canonical repository root used for confined loading.
    #[must_use]
    pub fn repository_root(&self) -> &Path {
        &self.repository_root
    }
    /// Returns unresolved validated graph input.
    #[must_use]
    pub const fn graph_input(&self) -> &WorkspaceGraphInput {
        &self.graph_input
    }
    /// Returns semantic authority keys mapped to stable source paths.
    #[must_use]
    pub const fn source_map(&self) -> &BTreeMap<Box<str>, RepoPath> {
        &self.source_map
    }
    /// Consumes the loader output into graph input for engine resolution.
    #[must_use]
    pub fn into_graph_input(self) -> WorkspaceGraphInput {
        self.graph_input
    }
}

/// Selects, discovers, validates, converts, and lock-binds one workspace.
pub fn load_workspace(
    start: &Path,
    explicit: Option<&RepoPath>,
) -> Result<LoadedWorkspace, LoadError> {
    let config = select_workspace_config(start, explicit).map_err(|_| LoadError::Config)?;
    let sources = discover_sources(&config).map_err(|_| LoadError::Discovery)?;
    let documents = decode_sources(config.repository_root(), &sources)
        .map_err(|error| LoadError::Validation(error.source().cloned()))?;
    let lock = load_lockfile(&config).map_err(|_| LoadError::Lock)?;

    let mut input = WorkspaceGraphInput {
        extensions: extensions(&config.dto().extensions, "eqm.toml")
            .map_err(|_| LoadError::WorkspaceTarget)?,
        ..WorkspaceGraphInput::default()
    };
    let config_source = config
        .config_path()
        .strip_prefix(config.repository_root())
        .ok()
        .and_then(|path| RepoPath::new(path.to_string_lossy().replace('\\', "/")).ok())
        .ok_or(LoadError::Config)?;
    let lock_source = RepoPath::new(config.dto().lockfile.as_deref().unwrap_or("eqm.lock"))
        .map_err(|_| LoadError::Lock)?;
    let mut source_map = BTreeMap::new();
    let mut target_roots = BTreeMap::new();
    let mut portable_roots = BTreeSet::new();
    for (id, target) in &config.dto().targets {
        let id: TargetId = id.parse().map_err(|_| LoadError::WorkspaceTarget)?;
        let root = RepoPath::new(&target.root).map_err(|_| LoadError::WorkspaceTarget)?;
        if !portable_roots.insert(root.portable_collision_key()) {
            return Err(LoadError::DuplicateTargetRoot);
        }
        let authority = Target::new(
            id.clone(),
            root.clone(),
            target
                .platform
                .parse()
                .map_err(|_| LoadError::WorkspaceTarget)?,
            target
                .framework
                .parse()
                .map_err(|_| LoadError::WorkspaceTarget)?,
            target
                .owners
                .iter()
                .map(|owner| {
                    owner
                        .parse::<OwnerRef>()
                        .map_err(|_| LoadError::WorkspaceTarget)
                })
                .collect::<Result<_, _>>()?,
            extensions(&target.extensions, "eqm.toml").map_err(|_| LoadError::WorkspaceTarget)?,
        )
        .map_err(|_| LoadError::WorkspaceTarget)?;
        target_roots.insert(id, root);
        source_map.insert(
            format!("target:{}", authority.id()).into(),
            config_source.clone(),
        );
        input.targets.push(authority);
    }

    for document in &documents {
        match document.document() {
            DocumentDto::Capability(_)
            | DocumentDto::Journey(_)
            | DocumentDto::Surface(_)
            | DocumentDto::Fragment(_) => {
                match convert_contract(document).map_err(conversion_error)? {
                    ContractEntity::Capability(value) => {
                        source_map.insert(
                            format!("capability:{}", value.id()).into(),
                            document.source().clone(),
                        );
                        input.capabilities.push(value);
                    }
                    ContractEntity::Journey(value) => {
                        source_map.insert(
                            format!("journey:{}", value.id()).into(),
                            document.source().clone(),
                        );
                        input.journeys.push(value);
                    }
                    ContractEntity::Surface(value) => {
                        source_map.insert(
                            format!("surface:{}", value.id()).into(),
                            document.source().clone(),
                        );
                        input.surfaces.push(value);
                    }
                    ContractEntity::Fragment(value) => {
                        source_map.insert(
                            format!("fragment:{}@{}", value.id(), value.revision()).into(),
                            document.source().clone(),
                        );
                        input.fragments.push(value);
                    }
                }
            }
            DocumentDto::Binding(_) => {
                let value = convert_binding(document, &target_roots).map_err(conversion_error)?;
                source_map.insert(
                    format!("binding:{}", value.id()).into(),
                    document.source().clone(),
                );
                input.bindings.push(value);
            }
            DocumentDto::Policy(_) => {
                let value = convert_policy(document).map_err(conversion_error)?;
                source_map.insert(
                    format!("policy:{}@{}", value.id(), value.revision()).into(),
                    document.source().clone(),
                );
                input.policies.push(value);
            }
            DocumentDto::Profile(_) => {
                let value = convert_profile(document).map_err(conversion_error)?;
                source_map.insert(
                    format!("profile:{}@{}", value.id(), value.revision()).into(),
                    document.source().clone(),
                );
                input.profiles.push(value);
            }
            DocumentDto::Runner(_) => {
                let value = convert_runner(document).map_err(conversion_error)?;
                source_map.insert(
                    format!("runner:{}@{}", value.id(), value.revision()).into(),
                    document.source().clone(),
                );
                input.runners.push(value);
            }
            DocumentDto::Waiver(_) => {
                let value = convert_waiver(document).map_err(conversion_error)?;
                source_map.insert(
                    format!("waiver:{}@{}", value.id(), value.revision()).into(),
                    document.source().clone(),
                );
                input.waivers.push(value);
            }
        }
    }
    for entry in lock.imports().values() {
        source_map.insert(
            format!("import:{}@{}", entry.id, entry.revision.get()).into(),
            lock_source.clone(),
        );
    }
    for entry in lock.adapters().values() {
        source_map.insert(
            format!("adapter_lock:{}@{}", entry.id, entry.version.as_str()).into(),
            lock_source.clone(),
        );
    }
    apply_lock(&mut input, lock);
    Ok(LoadedWorkspace {
        repository_root: config.repository_root().to_path_buf(),
        graph_input: input,
        source_map,
    })
}

fn apply_lock(input: &mut WorkspaceGraphInput, lock: WorkspaceLock) {
    input.imports = lock
        .imports()
        .values()
        .map(|entry| ImportLockIdentity {
            id: entry.id.clone(),
            revision: entry.revision,
            source: entry.source.clone(),
            resolved: entry.resolved.clone(),
            digest: entry.digest,
            trust: entry.trust.unwrap_or(TrustLevel::UntrustedLocal),
            signature: entry.signature.clone(),
        })
        .collect();
    input.adapter_locks = lock
        .adapters()
        .values()
        .map(|entry| AdapterLockIdentity {
            id: entry.id.clone(),
            version: entry.version.clone(),
            source: entry.source.clone(),
            resolved: entry.resolved.clone(),
            digest: entry.digest,
            protocol: entry.protocol,
            trust: entry.trust.unwrap_or(TrustLevel::UntrustedLocal),
            signature: entry.signature.clone(),
        })
        .collect();
}

fn conversion_error(error: crate::ConversionError) -> LoadError {
    LoadError::Conversion {
        source: error.source().into(),
        field: error.field(),
    }
}

/// Stable workspace-loading stage failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoadError {
    /// Config selection or decoding failed.
    Config,
    /// Source discovery failed.
    Discovery,
    /// Strict source validation failed.
    Validation(Option<RepoPath>),
    /// Workspace target authority was invalid.
    WorkspaceTarget,
    /// Target roots collided portably.
    DuplicateTargetRoot,
    /// Domain conversion failed.
    Conversion {
        /// Stable repository-relative source.
        source: Box<str>,
        /// Nearest invalid field path.
        field: &'static str,
    },
    /// Exact lockfile loading failed.
    Lock,
}

impl Display for LoadError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for LoadError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const CONFIG: &str = r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/workspace.schema.json"
contract_sources = ["eqm/contracts/**/*.toml"]
binding_sources = ["eqm/bindings/**/*.toml"]
policy_sources = ["eqm/policies/**/*.toml"]
profile_sources = ["eqm/profiles/**/*.toml"]
runner_sources = ["eqm/runners/**/*.toml"]
waiver_sources = ["eqm/waivers/**/*.toml"]

[targets.web]
root = "apps/web"
platform = "web"
framework = "sveltekit"
owners = ["owner://team/web"]
"#;
    const LOCK: &str = r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/lock.schema.json"
version = 1
"#;
    const CAPABILITY: &str = r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/capability.schema.json"
id = "account.create"
title = "Account creation"
status = "active"
owners = ["owner://team/accounts"]
"#;

    fn repository(config: &str) -> Result<tempfile::TempDir, Box<dyn Error>> {
        let repository = tempfile::tempdir()?;
        fs::create_dir(repository.path().join(".git"))?;
        fs::create_dir_all(repository.path().join("eqm/contracts"))?;
        fs::write(repository.path().join("eqm.toml"), config)?;
        fs::write(repository.path().join("eqm.lock"), LOCK)?;
        fs::write(
            repository.path().join("eqm/contracts/capability.toml"),
            CAPABILITY,
        )?;
        Ok(repository)
    }

    #[test]
    fn complete_workspace_load_is_repeatable_and_source_mapped() -> Result<(), Box<dyn Error>> {
        let repository = repository(CONFIG)?;
        let first = load_workspace(repository.path(), None)?;
        let second = load_workspace(repository.path(), None)?;
        assert_eq!(first, second);
        assert_eq!(first.graph_input().targets.len(), 1);
        assert_eq!(first.graph_input().capabilities.len(), 1);
        assert_eq!(
            first
                .source_map()
                .get("capability:account.create")
                .map(RepoPath::as_str),
            Some("eqm/contracts/capability.toml")
        );
        Ok(())
    }

    #[test]
    fn portable_target_root_collisions_fail_before_graph_input() -> Result<(), Box<dyn Error>> {
        let config = format!(
            "{CONFIG}\n[targets.admin]\nroot = \"Apps/Web\"\nplatform = \"web\"\nframework = \"none\"\nowners = [\"owner://team/admin\"]\n"
        );
        let repository = repository(&config)?;
        assert_eq!(
            load_workspace(repository.path(), None),
            Err(LoadError::DuplicateTargetRoot)
        );
        Ok(())
    }
}
