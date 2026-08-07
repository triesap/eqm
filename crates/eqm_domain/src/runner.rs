//! Shell-free runner and immutable out-of-process adapter definitions.

use crate::{
    AdapterId, DurationMillis, Extensions, InventoryCompleteness, OwnerRef, PositiveCount,
    RepoPath, RepositoryIdentity, Revision, RunnerBackend, RunnerGuarantee, RunnerId, SelectorText,
    Sha256Digest,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// One complete typed argv value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArgumentTemplate {
    /// A literal UTF-8 argument.
    Literal(SelectorText),
    /// Exact confined target-root path.
    TargetRoot,
    /// One bounded compact selector JSON argument.
    SelectorJson,
    /// Exact confined result path.
    ResultPath,
}

/// A confined working-directory value.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum WorkingDirectoryTemplate {
    /// Bound target root, the default.
    #[default]
    TargetRoot,
    /// A validated repository-relative directory.
    Repository(RepoPath),
    /// The confined result directory.
    ResultPath,
}

/// A repository-confined or immutable locked executable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RunnerProgram {
    /// A repository-relative executable path.
    Repository(RepoPath),
    /// An immutable externally resolved executable.
    Locked {
        /// Canonical immutable source identity.
        resolved: RepositoryIdentity,
        /// Exact executable digest.
        digest: Sha256Digest,
    },
}

/// A validated portable environment variable name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EnvironmentName(Box<str>);

impl EnvironmentName {
    /// Creates an uppercase portable environment name.
    pub fn new(value: impl Into<Box<str>>) -> Result<Self, RunnerBuildError> {
        let value = value.into();
        if value.is_empty()
            || value.len() > 128
            || !matches!(value.as_bytes().first(), Some(first) if first.is_ascii_uppercase() || *first == b'_')
            || !value
                .bytes()
                .skip(1)
                .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_')
        {
            return Err(RunnerBuildError::InvalidEnvironmentName);
        }
        Ok(Self(value))
    }
    /// Returns the exact name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Explicit non-secret environment value source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EnvironmentSource {
    /// A literal non-secret manifest value.
    Literal(SelectorText),
    /// The trusted runner configuration's fixed PATH.
    TrustedPath,
    /// Fixed `C.UTF-8` locale.
    CanonicalLocale,
    /// Fixed UTC timezone.
    UtcTimezone,
}

/// One explicit non-secret environment binding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnvironmentBinding {
    name: EnvironmentName,
    source: EnvironmentSource,
}

impl EnvironmentBinding {
    /// Creates a typed non-secret binding.
    pub fn new(name: EnvironmentName, source: EnvironmentSource) -> Result<Self, RunnerBuildError> {
        let compatible = matches!(
            (name.as_str(), &source),
            ("PATH", EnvironmentSource::TrustedPath)
                | ("LANG" | "LC_ALL", EnvironmentSource::CanonicalLocale)
                | ("TZ", EnvironmentSource::UtcTimezone)
                | (_, EnvironmentSource::Literal(_))
        );
        if !compatible {
            return Err(RunnerBuildError::IncompatibleEnvironmentSource);
        }
        Ok(Self { name, source })
    }
    /// Returns environment name.
    #[must_use]
    pub const fn name(&self) -> &EnvironmentName {
        &self.name
    }
    /// Returns typed source.
    #[must_use]
    pub const fn source(&self) -> &EnvironmentSource {
        &self.source
    }
}

/// An opaque secret-provider reference, never a secret value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SecretProviderRef(SelectorText);

impl SecretProviderRef {
    /// Creates a provider reference using `secret://<provider>/<name>`.
    pub fn new(value: impl Into<Box<str>>) -> Result<Self, RunnerBuildError> {
        let value: Box<str> = value.into();
        let Some(rest) = value.strip_prefix("secret://") else {
            return Err(RunnerBuildError::InvalidSecretProvider);
        };
        let parts: Vec<_> = rest.split('/').collect();
        if parts.len() != 2 || !lower_segment(parts[0]) || !lower_segment(parts[1]) {
            return Err(RunnerBuildError::InvalidSecretProvider);
        }
        Ok(Self(
            SelectorText::new(value).map_err(|_| RunnerBuildError::InvalidSecretProvider)?,
        ))
    }
    /// Returns the opaque provider reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

fn lower_segment(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 63
        && value.as_bytes()[0].is_ascii_lowercase()
        && value
            .bytes()
            .skip(1)
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

/// One redacted secret binding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SecretBinding {
    name: EnvironmentName,
    provider: SecretProviderRef,
}

impl SecretBinding {
    /// Creates a secret binding without accepting a value.
    #[must_use]
    pub const fn new(name: EnvironmentName, provider: SecretProviderRef) -> Self {
        Self { name, provider }
    }
    /// Returns environment name.
    #[must_use]
    pub const fn name(&self) -> &EnvironmentName {
        &self.name
    }
    /// Returns provider reference.
    #[must_use]
    pub const fn provider(&self) -> &SecretProviderRef {
        &self.provider
    }
}

/// Bounded positive runner resource controls.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RunnerLimits {
    timeout: DurationMillis,
    max_output_bytes: PositiveCount,
    max_concurrency: PositiveCount,
}

impl RunnerLimits {
    /// Creates mandatory positive limits.
    pub fn new(
        timeout: DurationMillis,
        max_output_bytes: PositiveCount,
        max_concurrency: Option<PositiveCount>,
    ) -> Result<Self, RunnerBuildError> {
        if timeout.get() == 0 {
            return Err(RunnerBuildError::PositiveLimitRequired);
        }
        Ok(Self {
            timeout,
            max_output_bytes,
            max_concurrency: max_concurrency.unwrap_or(PositiveCount::ONE),
        })
    }
    /// Returns timeout.
    #[must_use]
    pub const fn timeout(&self) -> DurationMillis {
        self.timeout
    }
    /// Returns output cap.
    #[must_use]
    pub const fn max_output_bytes(&self) -> PositiveCount {
        self.max_output_bytes
    }
    /// Returns concurrency cap.
    #[must_use]
    pub const fn max_concurrency(&self) -> PositiveCount {
        self.max_concurrency
    }
}

/// A versioned shell-free runner definition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunnerDefinition {
    id: RunnerId,
    revision: Revision,
    owners: BTreeSet<OwnerRef>,
    backend: RunnerBackend,
    program: RunnerProgram,
    args: Vec<ArgumentTemplate>,
    cwd: WorkingDirectoryTemplate,
    environment: BTreeMap<EnvironmentName, EnvironmentBinding>,
    secrets: BTreeMap<EnvironmentName, SecretBinding>,
    limits: RunnerLimits,
    guarantees: BTreeSet<RunnerGuarantee>,
    extensions: Extensions,
}

impl RunnerDefinition {
    /// Creates a runner and rejects unsupported local guarantees or name collisions.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: RunnerId,
        revision: Revision,
        owners: Vec<OwnerRef>,
        backend: RunnerBackend,
        program: RunnerProgram,
        args: Vec<ArgumentTemplate>,
        cwd: Option<WorkingDirectoryTemplate>,
        environment: Vec<EnvironmentBinding>,
        secrets: Vec<SecretBinding>,
        limits: RunnerLimits,
        guarantees: Vec<RunnerGuarantee>,
        extensions: Extensions,
    ) -> Result<Self, RunnerBuildError> {
        let owners = unique_nonempty(
            owners,
            RunnerBuildError::OwnersRequired,
            RunnerBuildError::DuplicateOwner,
        )?;
        let environment = keyed_environment(environment)?;
        let secrets = keyed_secrets(secrets)?;
        if environment.keys().any(|name| secrets.contains_key(name)) {
            return Err(RunnerBuildError::EnvironmentSecretCollision);
        }
        let guarantee_count = guarantees.len();
        let guarantees: BTreeSet<_> = guarantees.into_iter().collect();
        if guarantees.len() != guarantee_count {
            return Err(RunnerBuildError::DuplicateGuarantee);
        }
        if backend == RunnerBackend::Local && !guarantees.is_empty() {
            return Err(RunnerBuildError::UnsupportedLocalGuarantee);
        }
        Ok(Self {
            id,
            revision,
            owners,
            backend,
            program,
            args,
            cwd: cwd.unwrap_or_default(),
            environment,
            secrets,
            limits,
            guarantees,
            extensions,
        })
    }
    /// Returns runner ID.
    #[must_use]
    pub const fn id(&self) -> &RunnerId {
        &self.id
    }
    /// Returns revision.
    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }
    /// Returns owners.
    #[must_use]
    pub const fn owners(&self) -> &BTreeSet<OwnerRef> {
        &self.owners
    }
    /// Returns backend.
    #[must_use]
    pub const fn backend(&self) -> RunnerBackend {
        self.backend
    }
    /// Returns program.
    #[must_use]
    pub const fn program(&self) -> &RunnerProgram {
        &self.program
    }
    /// Returns normative argv order.
    #[must_use]
    pub fn args(&self) -> &[ArgumentTemplate] {
        &self.args
    }
    /// Returns cwd template.
    #[must_use]
    pub const fn cwd(&self) -> &WorkingDirectoryTemplate {
        &self.cwd
    }
    /// Returns non-secret bindings.
    #[must_use]
    pub const fn environment(&self) -> &BTreeMap<EnvironmentName, EnvironmentBinding> {
        &self.environment
    }
    /// Returns redacted secret bindings.
    #[must_use]
    pub const fn secrets(&self) -> &BTreeMap<EnvironmentName, SecretBinding> {
        &self.secrets
    }
    /// Returns resource limits.
    #[must_use]
    pub const fn limits(&self) -> RunnerLimits {
        self.limits
    }
    /// Returns claimed backend guarantees.
    #[must_use]
    pub const fn guarantees(&self) -> &BTreeSet<RunnerGuarantee> {
        &self.guarantees
    }
    /// Returns extensions.
    #[must_use]
    pub const fn extensions(&self) -> &Extensions {
        &self.extensions
    }
}

/// The only v1 adapter discovery mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiscoveryMode {
    /// Digest-pinned out-of-process `discover` request/response.
    OutOfProcess,
}

/// Bounded adapter request and response resources.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdapterLimits {
    timeout: DurationMillis,
    max_input_bytes: PositiveCount,
    max_output_bytes: PositiveCount,
    max_entries: PositiveCount,
    max_depth: PositiveCount,
}

impl AdapterLimits {
    /// Creates mandatory finite adapter bounds.
    pub fn new(
        timeout: DurationMillis,
        max_input_bytes: PositiveCount,
        max_output_bytes: PositiveCount,
        max_entries: PositiveCount,
        max_depth: PositiveCount,
    ) -> Result<Self, RunnerBuildError> {
        if timeout.get() == 0 {
            return Err(RunnerBuildError::PositiveLimitRequired);
        }
        Ok(Self {
            timeout,
            max_input_bytes,
            max_output_bytes,
            max_entries,
            max_depth,
        })
    }
    /// Returns timeout.
    #[must_use]
    pub const fn timeout(&self) -> DurationMillis {
        self.timeout
    }
    /// Returns input cap.
    #[must_use]
    pub const fn max_input_bytes(&self) -> PositiveCount {
        self.max_input_bytes
    }
    /// Returns output cap.
    #[must_use]
    pub const fn max_output_bytes(&self) -> PositiveCount {
        self.max_output_bytes
    }
    /// Returns entry cap.
    #[must_use]
    pub const fn max_entries(&self) -> PositiveCount {
        self.max_entries
    }
    /// Returns nesting cap.
    #[must_use]
    pub const fn max_depth(&self) -> PositiveCount {
        self.max_depth
    }
}

/// One exact immutable adapter discovery definition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterDefinition {
    id: AdapterId,
    version: SelectorText,
    resolved: RepositoryIdentity,
    digest: Sha256Digest,
    protocol: Revision,
    discovery_mode: DiscoveryMode,
    completeness: InventoryCompleteness,
    limits: AdapterLimits,
}

impl AdapterDefinition {
    /// Creates a digest-pinned v1 adapter definition.
    pub fn new(
        id: AdapterId,
        version: SelectorText,
        resolved: RepositoryIdentity,
        digest: Sha256Digest,
        protocol: Revision,
        completeness: InventoryCompleteness,
        limits: AdapterLimits,
    ) -> Result<Self, RunnerBuildError> {
        if protocol.get() != 1 {
            return Err(RunnerBuildError::UnsupportedAdapterProtocol);
        }
        Ok(Self {
            id,
            version,
            resolved,
            digest,
            protocol,
            discovery_mode: DiscoveryMode::OutOfProcess,
            completeness,
            limits,
        })
    }
    /// Returns adapter ID.
    #[must_use]
    pub const fn id(&self) -> &AdapterId {
        &self.id
    }
    /// Returns version.
    #[must_use]
    pub const fn version(&self) -> &SelectorText {
        &self.version
    }
    /// Returns immutable resolution.
    #[must_use]
    pub const fn resolved(&self) -> &RepositoryIdentity {
        &self.resolved
    }
    /// Returns executable digest.
    #[must_use]
    pub const fn digest(&self) -> Sha256Digest {
        self.digest
    }
    /// Returns exact protocol revision.
    #[must_use]
    pub const fn protocol(&self) -> Revision {
        self.protocol
    }
    /// Returns discovery mode.
    #[must_use]
    pub const fn discovery_mode(&self) -> DiscoveryMode {
        self.discovery_mode
    }
    /// Returns maximum completeness claim.
    #[must_use]
    pub const fn completeness(&self) -> InventoryCompleteness {
        self.completeness
    }
    /// Returns adapter resource limits.
    #[must_use]
    pub const fn limits(&self) -> AdapterLimits {
        self.limits
    }
}

fn keyed_environment(
    values: Vec<EnvironmentBinding>,
) -> Result<BTreeMap<EnvironmentName, EnvironmentBinding>, RunnerBuildError> {
    let count = values.len();
    let values: BTreeMap<_, _> = values
        .into_iter()
        .map(|value| (value.name().clone(), value))
        .collect();
    if values.len() != count {
        return Err(RunnerBuildError::DuplicateEnvironment);
    }
    Ok(values)
}

fn keyed_secrets(
    values: Vec<SecretBinding>,
) -> Result<BTreeMap<EnvironmentName, SecretBinding>, RunnerBuildError> {
    let count = values.len();
    let values: BTreeMap<_, _> = values
        .into_iter()
        .map(|value| (value.name().clone(), value))
        .collect();
    if values.len() != count {
        return Err(RunnerBuildError::DuplicateSecret);
    }
    Ok(values)
}

fn unique_nonempty<T: Ord>(
    values: Vec<T>,
    empty: RunnerBuildError,
    duplicate: RunnerBuildError,
) -> Result<BTreeSet<T>, RunnerBuildError> {
    if values.is_empty() {
        return Err(empty);
    }
    let count = values.len();
    let values: BTreeSet<_> = values.into_iter().collect();
    if values.len() != count {
        return Err(duplicate);
    }
    Ok(values)
}

/// Runner or adapter construction failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RunnerBuildError {
    /// Owners were empty.
    OwnersRequired,
    /// Owners contained a duplicate.
    DuplicateOwner,
    /// Environment name was invalid.
    InvalidEnvironmentName,
    /// Fixed source did not match its reserved environment name.
    IncompatibleEnvironmentSource,
    /// Secret provider reference was malformed.
    InvalidSecretProvider,
    /// Non-secret environment names contained a duplicate.
    DuplicateEnvironment,
    /// Secret names contained a duplicate.
    DuplicateSecret,
    /// One name was both secret and non-secret.
    EnvironmentSecretCollision,
    /// Guarantee set contained a duplicate.
    DuplicateGuarantee,
    /// Local backend claimed an unenforceable closed guarantee.
    UnsupportedLocalGuarantee,
    /// A mandatory limit was zero.
    PositiveLimitRequired,
    /// Adapter protocol was not exactly v1.
    UnsupportedAdapterProtocol,
}

impl Display for RunnerBuildError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

impl Error for RunnerBuildError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> Result<RunnerLimits, Box<dyn Error>> {
        Ok(RunnerLimits::new(
            DurationMillis::new(60_000)?,
            PositiveCount::new(1_048_576)?,
            None,
        )?)
    }

    #[test]
    fn templates_are_typed_and_local_guarantees_fail() -> Result<(), Box<dyn Error>> {
        let runner = RunnerDefinition::new(
            RunnerId::new("runner.cargo")?,
            Revision::new(1)?,
            vec!["owner://team/tooling".parse()?],
            RunnerBackend::Local,
            RunnerProgram::Repository(RepoPath::new("bin/test-runner")?),
            vec![
                ArgumentTemplate::Literal(SelectorText::new("test")?),
                ArgumentTemplate::SelectorJson,
            ],
            None,
            Vec::new(),
            Vec::new(),
            limits()?,
            Vec::new(),
            Extensions::default(),
        )?;
        assert_eq!(runner.args().len(), 2);
        assert!(matches!(
            RunnerDefinition::new(
                RunnerId::new("runner.cargo")?,
                Revision::new(1)?,
                vec!["owner://team/tooling".parse()?],
                RunnerBackend::Local,
                RunnerProgram::Repository(RepoPath::new("bin/test-runner")?),
                Vec::new(),
                None,
                Vec::new(),
                Vec::new(),
                limits()?,
                vec![RunnerGuarantee::NetworkDenied],
                Extensions::default(),
            ),
            Err(RunnerBuildError::UnsupportedLocalGuarantee)
        ));
        Ok(())
    }

    #[test]
    fn environment_secret_and_adapter_limits_fail_closed() -> Result<(), Box<dyn Error>> {
        let name = EnvironmentName::new("TOKEN")?;
        let environment = EnvironmentBinding::new(
            name.clone(),
            EnvironmentSource::Literal(SelectorText::new("not-secret")?),
        )?;
        let secret = SecretBinding::new(name, SecretProviderRef::new("secret://vault/token")?);
        assert!(matches!(
            RunnerDefinition::new(
                RunnerId::new("runner.test")?,
                Revision::new(1)?,
                vec!["owner://team/tooling".parse()?],
                RunnerBackend::Container,
                RunnerProgram::Repository(RepoPath::new("bin/runner")?),
                Vec::new(),
                None,
                vec![environment],
                vec![secret],
                limits()?,
                Vec::new(),
                Extensions::default(),
            ),
            Err(RunnerBuildError::EnvironmentSecretCollision)
        ));
        assert_eq!(
            AdapterLimits::new(
                DurationMillis::new(0)?,
                PositiveCount::ONE,
                PositiveCount::ONE,
                PositiveCount::ONE,
                PositiveCount::ONE,
            ),
            Err(RunnerBuildError::PositiveLimitRequired)
        );
        Ok(())
    }
}
