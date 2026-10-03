//! End-to-end authored workspace loading into unresolved graph input.

use crate::conversion::extensions;
use crate::{
    ConfigError, ContractEntity, ConversionError, DiscoveryError, DocumentDto, LockError,
    ValidationError, WorkspaceLock, convert_binding_with_roots, convert_cohort, convert_contract,
    convert_participation, convert_policy, convert_profile, convert_provider, convert_runner,
    convert_scenario, convert_waiver, decode_sources, discover_sources, load_lockfile,
    select_workspace_config,
};
use eqm_domain::{
    AdapterLockIdentity, ImportLockIdentity, OwnerRef, ProviderId, RepoPath, RoleId, SchemaVersion,
    SourceComponent, SourceComponentId, Target, TargetId, TrustLevel, WorkspaceGraphInput,
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
    version: SchemaVersion,
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
    /// Returns the exact authored workspace schema version that selected the
    /// graph representation.
    #[must_use]
    pub const fn authored_version(&self) -> SchemaVersion {
        self.version
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
    let config = select_workspace_config(start, explicit).map_err(LoadError::Config)?;
    let sources = discover_sources(&config).map_err(LoadError::Discovery)?;
    let documents =
        decode_sources(config.repository_root(), &sources).map_err(LoadError::Validation)?;
    let lock = load_lockfile(&config).map_err(LoadError::Lock)?;

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
        .ok_or(LoadError::Config(ConfigError::OutsideRepository))?;
    let lock_source = RepoPath::new(config.dto().lockfile.as_deref().unwrap_or("eqm.lock"))
        .map_err(|_| LoadError::Lock(LockError::InvalidPath))?;
    let mut source_map = BTreeMap::new();
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
        source_map.insert(
            format!("target:{}", authority.id()).into(),
            config_source.clone(),
        );
        input.targets.push(authority);
    }

    // The additive v2 workspace family carries source components and explicit
    // target role/source/dependency references. There is no implicit v1/v2
    // conversion: the v2 document is decoded by its own reader above.
    if let Some(v2) = config.workspace_v2() {
        for (id, target) in &v2.targets {
            let id: TargetId = id.parse().map_err(|_| LoadError::WorkspaceTarget)?;
            let root = RepoPath::new(&target.root).map_err(|_| LoadError::WorkspaceTarget)?;
            if !portable_roots.insert(root.portable_collision_key()) {
                return Err(LoadError::DuplicateTargetRoot);
            }
            let mut authority = Target::new(
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
                extensions(&target.extensions, "eqm.toml")
                    .map_err(|_| LoadError::WorkspaceTarget)?,
            )
            .map_err(|_| LoadError::WorkspaceTarget)?;
            if let Some(role) = &target.role {
                authority = authority.with_role(
                    role.parse::<RoleId>()
                        .map_err(|_| LoadError::WorkspaceTarget)?,
                );
            }
            authority = authority
                .with_source_components(
                    target
                        .source_components
                        .iter()
                        .map(|component| {
                            component
                                .parse::<SourceComponentId>()
                                .map_err(|_| LoadError::WorkspaceTarget)
                        })
                        .collect::<Result<_, _>>()?,
                )
                .map_err(|_| LoadError::WorkspaceTarget)?;
            authority = authority
                .with_depends_on(
                    target
                        .depends_on
                        .iter()
                        .map(|dependency| {
                            dependency
                                .parse::<TargetId>()
                                .map_err(|_| LoadError::WorkspaceTarget)
                        })
                        .collect::<Result<_, _>>()?,
                )
                .map_err(|_| LoadError::WorkspaceTarget)?;
            authority = authority
                .with_providers(
                    target
                        .providers
                        .iter()
                        .map(|provider| {
                            provider
                                .parse::<ProviderId>()
                                .map_err(|_| LoadError::WorkspaceTarget)
                        })
                        .collect::<Result<_, _>>()?,
                )
                .map_err(|_| LoadError::WorkspaceTarget)?;
            source_map.insert(
                format!("target:{}", authority.id()).into(),
                config_source.clone(),
            );
            input.targets.push(authority);
        }
        for (id, component) in &v2.source_components {
            let id: SourceComponentId = id.parse().map_err(|_| LoadError::WorkspaceTarget)?;
            let authority = SourceComponent::new(
                id.clone(),
                component
                    .roots
                    .iter()
                    .map(|root| RepoPath::new(root).map_err(|_| LoadError::WorkspaceTarget))
                    .collect::<Result<_, _>>()?,
                component
                    .owners
                    .iter()
                    .map(|owner| {
                        owner
                            .parse::<OwnerRef>()
                            .map_err(|_| LoadError::WorkspaceTarget)
                    })
                    .collect::<Result<_, _>>()?,
                component
                    .dependencies
                    .iter()
                    .map(|dependency| {
                        dependency
                            .parse::<SourceComponentId>()
                            .map_err(|_| LoadError::WorkspaceTarget)
                    })
                    .collect::<Result<_, _>>()?,
                extensions(&component.extensions, "eqm.toml")
                    .map_err(|_| LoadError::WorkspaceTarget)?,
            )
            .map_err(|_| LoadError::WorkspaceTarget)?;
            source_map.insert(
                format!("source_component:{}", authority.id()).into(),
                config_source.clone(),
            );
            input.source_components.push(authority);
        }
    }

    validate_shared_source_overlaps(&input)?;

    // Each binding validates its artifact paths against the target's complete
    // declared source ownership: the shipped output root plus every declared
    // source component reachable through its dependency closure.
    let components_by_id: BTreeMap<&str, &SourceComponent> = input
        .source_components
        .iter()
        .map(|component| (component.id().as_str(), component))
        .collect();
    let mut binding_roots: BTreeMap<TargetId, Vec<RepoPath>> = BTreeMap::new();
    for target in &input.targets {
        let mut roots = vec![target.root().clone()];
        for id in transitive_component_closure(
            &components_by_id,
            target
                .source_components()
                .iter()
                .map(SourceComponentId::as_str),
        ) {
            if let Some(component) = components_by_id.get(id) {
                roots.extend(component.roots().iter().cloned());
            }
        }
        roots.sort();
        roots.dedup();
        binding_roots.insert(target.id().clone(), roots);
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
                let value = convert_binding_with_roots(document, &binding_roots)
                    .map_err(conversion_error)?;
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
            DocumentDto::Participation(_) => {
                let value = convert_participation(document).map_err(conversion_error)?;
                source_map.insert(
                    format!("participation:{}", value.id()).into(),
                    document.source().clone(),
                );
                input.participation.push(value);
            }
            DocumentDto::Scenario(_) => {
                let value = convert_scenario(document).map_err(conversion_error)?;
                source_map.insert(
                    format!("scenario:{}@{}", value.id(), value.revision().get()).into(),
                    document.source().clone(),
                );
                input.scenarios.push(value);
            }
            DocumentDto::Provider(_) => {
                let value = convert_provider(document).map_err(conversion_error)?;
                source_map.insert(
                    format!("provider:{}", value.id()).into(),
                    document.source().clone(),
                );
                input.providers.push(value);
            }
            DocumentDto::Cohort(_) => {
                let value = convert_cohort(document).map_err(conversion_error)?;
                source_map.insert(
                    format!("cohort:{}@{}", value.id(), value.revision().get()).into(),
                    document.source().clone(),
                );
                input.cohorts.push(value);
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
    let version = if config.workspace_v2().is_some() {
        SchemaVersion::V2
    } else {
        SchemaVersion::V1
    };
    Ok(LoadedWorkspace {
        repository_root: config.repository_root().to_path_buf(),
        graph_input: input,
        source_map,
        version,
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

fn conversion_error(error: ConversionError) -> LoadError {
    LoadError::Conversion(error)
}

/// Stable workspace-loading stage failure retaining its causal error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoadError {
    /// Config selection or decoding failed.
    Config(ConfigError),
    /// Source discovery failed.
    Discovery(DiscoveryError),
    /// Strict source validation failed, retaining source and classification.
    Validation(ValidationError),
    /// Workspace target authority was invalid.
    WorkspaceTarget,
    /// Target roots collided portably.
    DuplicateTargetRoot,
    /// Distinct source components claimed overlapping roots without a declared
    /// dependency relationship, so ownership is ambiguous.
    AmbiguousSourceOverlap,
    /// A target root overlapped a source component it did not declare.
    UndeclaredSourceOverlap,
    /// Domain conversion failed, retaining source and invalid field path.
    Conversion(ConversionError),
    /// Exact lockfile loading failed.
    Lock(LockError),
}

impl Display for LoadError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Config(error) => write!(formatter, "workspace configuration failed: {error}"),
            Self::Discovery(error) => write!(formatter, "source discovery failed: {error}"),
            Self::Validation(error) => write!(formatter, "source validation failed: {error}"),
            Self::Conversion(error) => write!(formatter, "contract conversion failed: {error}"),
            Self::Lock(error) => write!(formatter, "lockfile loading failed: {error}"),
            Self::WorkspaceTarget => formatter.write_str("workspace target authority was invalid"),
            Self::DuplicateTargetRoot => {
                formatter.write_str("workspace target roots collided portably")
            }
            Self::AmbiguousSourceOverlap => {
                formatter.write_str("workspace source-component roots overlap ambiguously")
            }
            Self::UndeclaredSourceOverlap => {
                formatter.write_str("workspace target root overlaps an undeclared source component")
            }
        }
    }
}

impl Error for LoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Config(error) => Some(error),
            Self::Discovery(error) => Some(error),
            Self::Validation(error) => Some(error),
            Self::Conversion(error) => Some(error),
            Self::Lock(error) => Some(error),
            Self::WorkspaceTarget
            | Self::DuplicateTargetRoot
            | Self::AmbiguousSourceOverlap
            | Self::UndeclaredSourceOverlap => None,
        }
    }
}

/// Returns whether two repository-relative roots overlap (one contains the
/// other). Distinct output roots never overlap; shared component roots may.
fn roots_overlap(left: &RepoPath, right: &RepoPath) -> bool {
    let left = left.as_str();
    let right = right.as_str();
    left == right
        || left
            .strip_prefix(right)
            .is_some_and(|rest| rest.starts_with('/'))
        || right
            .strip_prefix(left)
            .is_some_and(|rest| rest.starts_with('/'))
}

/// Returns one deterministic transitive component closure over the declared
/// component dependency graph. `depends_on` target edges never enter this
/// closure: only a target's own declared `source_components` (and their
/// transitive component dependencies) grant shared roots.
fn transitive_component_closure<'a>(
    components: &BTreeMap<&'a str, &'a SourceComponent>,
    seeds: impl IntoIterator<Item = &'a str>,
) -> BTreeSet<&'a str> {
    let mut seen = BTreeSet::new();
    let mut stack: Vec<&'a str> = seeds.into_iter().collect();
    while let Some(id) = stack.pop() {
        if !seen.insert(id) {
            continue;
        }
        if let Some(component) = components.get(id) {
            stack.extend(
                component
                    .dependencies()
                    .iter()
                    .map(SourceComponentId::as_str),
            );
        }
    }
    seen
}

/// Rejects ambiguous source-component root overlaps and undeclared target/root
/// overlaps using one checked transitive component closure. Output identity (the
/// target root) stays distinct from shared source roots, so a declared shared
/// component is not a target-root collision.
fn validate_shared_source_overlaps(input: &WorkspaceGraphInput) -> Result<(), LoadError> {
    let components: BTreeMap<&str, &SourceComponent> = input
        .source_components
        .iter()
        .map(|component| (component.id().as_str(), component))
        .collect();
    let ids: Vec<&str> = components.keys().copied().collect();
    for (index, left) in ids.iter().enumerate() {
        let left_closure = transitive_component_closure(&components, [*left]);
        let left_component = components[left];
        for right in &ids[index + 1..] {
            let right_component = components[right];
            let overlaps = left_component.roots().iter().any(|left_root| {
                right_component
                    .roots()
                    .iter()
                    .any(|right_root| roots_overlap(left_root, right_root))
            });
            // The complete overlap must be explained by the same declared
            // source-component dependency closure, in either direction.
            let explained = left_closure.contains(right)
                || transitive_component_closure(&components, [*right]).contains(left);
            if overlaps && !explained {
                return Err(LoadError::AmbiguousSourceOverlap);
            }
        }
    }
    for target in &input.targets {
        let closure = transitive_component_closure(
            &components,
            target
                .source_components()
                .iter()
                .map(SourceComponentId::as_str),
        );
        for (id, component) in &components {
            let overlaps = component
                .roots()
                .iter()
                .any(|root| roots_overlap(root, target.root()));
            if overlaps && !closure.contains(id) {
                return Err(LoadError::UndeclaredSourceOverlap);
            }
        }
    }
    Ok(())
}

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
