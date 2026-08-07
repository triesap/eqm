//! Approved runner-definition resolution and deterministic identity.

use eqm_domain::{
    ArgumentTemplate, DurationMillis, EnvironmentSource, ExtensionValue, PositiveCount, RepoPath,
    RepositoryIdentity, Revision, RunnerBackend, RunnerDefinition, RunnerGuarantee, RunnerId,
    RunnerProgram, Sha256Digest, WorkingDirectoryTemplate,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// External authority constraining one runner definition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunnerResolutionAuthority {
    /// Exact authorized runner ID.
    pub id: RunnerId,
    /// Exact authorized revision.
    pub revision: Revision,
    /// Backends permitted by operator authority.
    pub backends: BTreeSet<RunnerBackend>,
    /// Repository executables and their independently verified digests.
    pub repository_programs: BTreeMap<RepoPath, Sha256Digest>,
    /// Backend guarantees the configured host can enforce.
    pub backend_guarantees: BTreeMap<RunnerBackend, BTreeSet<RunnerGuarantee>>,
    /// Maximum execution time.
    pub maximum_timeout: DurationMillis,
    /// Maximum retained stdout or stderr bytes.
    pub maximum_output_bytes: PositiveCount,
    /// Maximum parallel executions.
    pub maximum_concurrency: PositiveCount,
}

/// Exact executable identity prepared for a runner backend.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResolvedProgram {
    /// Repository-confined executable with independently read content digest.
    Repository {
        /// Normalized repository path.
        path: RepoPath,
        /// Verified executable digest.
        digest: Sha256Digest,
    },
    /// Immutable externally locked executable.
    Locked {
        /// Canonical immutable source identity.
        resolved: RepositoryIdentity,
        /// Locked executable digest.
        digest: Sha256Digest,
    },
}

/// Fully authorized executable runner configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedRunner {
    definition: RunnerDefinition,
    program: ResolvedProgram,
    digest: Sha256Digest,
}

impl ResolvedRunner {
    /// Returns the validated semantic definition.
    #[must_use]
    pub const fn definition(&self) -> &RunnerDefinition {
        &self.definition
    }

    /// Returns the exact executable identity.
    #[must_use]
    pub const fn program(&self) -> &ResolvedProgram {
        &self.program
    }

    /// Returns the canonical semantic definition digest.
    #[must_use]
    pub const fn digest(&self) -> Sha256Digest {
        self.digest
    }
}

/// Resolves a typed definition against exact operator and host authority.
pub fn resolve_runner(
    definition: &RunnerDefinition,
    authority: &RunnerResolutionAuthority,
) -> Result<ResolvedRunner, RunnerResolutionError> {
    if definition.id() != &authority.id || definition.revision() != authority.revision {
        return Err(RunnerResolutionError::AuthorityMismatch);
    }
    if !authority.backends.contains(&definition.backend()) {
        return Err(RunnerResolutionError::BackendNotAuthorized);
    }
    let limits = definition.limits();
    if limits.timeout() > authority.maximum_timeout {
        return Err(RunnerResolutionError::TimeoutExceedsAuthority);
    }
    if limits.max_output_bytes() > authority.maximum_output_bytes {
        return Err(RunnerResolutionError::OutputExceedsAuthority);
    }
    if limits.max_concurrency() > authority.maximum_concurrency {
        return Err(RunnerResolutionError::ConcurrencyExceedsAuthority);
    }
    let enforceable = authority
        .backend_guarantees
        .get(&definition.backend())
        .cloned()
        .unwrap_or_default();
    if !definition.guarantees().is_subset(&enforceable) {
        return Err(RunnerResolutionError::GuaranteeNotEnforceable);
    }
    let program = match definition.program() {
        RunnerProgram::Repository(path) => ResolvedProgram::Repository {
            path: path.clone(),
            digest: authority
                .repository_programs
                .get(path)
                .copied()
                .ok_or(RunnerResolutionError::ProgramNotAuthorized)?,
        },
        RunnerProgram::Locked { resolved, digest } => ResolvedProgram::Locked {
            resolved: resolved.clone(),
            digest: *digest,
        },
    };
    Ok(ResolvedRunner {
        definition: definition.clone(),
        program,
        digest: canonical_runner_digest(definition),
    })
}

fn canonical_runner_digest(definition: &RunnerDefinition) -> Sha256Digest {
    let mut encoder = Encoder::default();
    encoder.text("eqm:v1:runner-definition");
    encoder.text(definition.id().as_str());
    encoder.u64(definition.revision().get());
    encoder.sequence(definition.owners().iter(), |value, encoder| {
        encoder.text(value.as_str());
    });
    encoder.text(definition.backend().as_str());
    match definition.program() {
        RunnerProgram::Repository(path) => {
            encoder.byte(0);
            encoder.text(path.as_str());
        }
        RunnerProgram::Locked { resolved, digest } => {
            encoder.byte(1);
            encoder.text(resolved.as_str());
            encoder.bytes(digest.as_bytes());
        }
    }
    encoder.sequence(
        definition.args().iter(),
        |argument, encoder| match argument {
            ArgumentTemplate::Literal(value) => {
                encoder.byte(0);
                encoder.text(value.as_str());
            }
            ArgumentTemplate::TargetRoot => encoder.byte(1),
            ArgumentTemplate::SelectorJson => encoder.byte(2),
            ArgumentTemplate::ResultPath => encoder.byte(3),
        },
    );
    match definition.cwd() {
        WorkingDirectoryTemplate::TargetRoot => encoder.byte(0),
        WorkingDirectoryTemplate::Repository(path) => {
            encoder.byte(1);
            encoder.text(path.as_str());
        }
        WorkingDirectoryTemplate::ResultPath => encoder.byte(2),
    }
    encoder.sequence(definition.environment().values(), |binding, encoder| {
        encoder.text(binding.name().as_str());
        match binding.source() {
            EnvironmentSource::Literal(value) => {
                encoder.byte(0);
                encoder.text(value.as_str());
            }
            EnvironmentSource::TrustedPath => encoder.byte(1),
            EnvironmentSource::CanonicalLocale => encoder.byte(2),
            EnvironmentSource::UtcTimezone => encoder.byte(3),
        }
    });
    encoder.sequence(definition.secrets().values(), |binding, encoder| {
        encoder.text(binding.name().as_str());
        encoder.text(binding.provider().as_str());
    });
    let limits = definition.limits();
    encoder.u64(limits.timeout().get());
    encoder.u64(limits.max_output_bytes().get());
    encoder.u64(limits.max_concurrency().get());
    encoder.sequence(definition.guarantees().iter(), |guarantee, encoder| {
        encoder.text(guarantee.as_str());
    });
    let semantic_extensions: Vec<_> = definition
        .extensions()
        .values()
        .iter()
        .filter(|(namespace, _)| !namespace.is_display_only())
        .collect();
    encoder.sequence(semantic_extensions, |(namespace, value), encoder| {
        encoder.text(namespace.as_str());
        encode_extension(value, encoder);
    });
    Sha256Digest::hash_content(&encoder.bytes)
}

fn encode_extension(value: &ExtensionValue, encoder: &mut Encoder) {
    match value {
        ExtensionValue::Boolean(value) => {
            encoder.byte(0);
            encoder.byte(u8::from(*value));
        }
        ExtensionValue::Integer(value) => {
            encoder.byte(1);
            encoder.bytes(&value.to_be_bytes());
        }
        ExtensionValue::String(value) => {
            encoder.byte(2);
            encoder.text(value);
        }
        ExtensionValue::Array(values) => {
            encoder.byte(3);
            encoder.sequence(values, encode_extension);
        }
        ExtensionValue::Object(values) => {
            encoder.byte(4);
            encoder.sequence(values, |(key, value), encoder| {
                encoder.text(key.as_str());
                encode_extension(value, encoder);
            });
        }
    }
}

#[derive(Default)]
struct Encoder {
    bytes: Vec<u8>,
}

impl Encoder {
    fn byte(&mut self, value: u8) {
        self.bytes.push(value);
    }

    fn bytes(&mut self, value: &[u8]) {
        self.u64(value.len() as u64);
        self.bytes.extend_from_slice(value);
    }

    fn text(&mut self, value: &str) {
        self.bytes(value.as_bytes());
    }

    fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn sequence<T>(
        &mut self,
        values: impl IntoIterator<Item = T>,
        mut encode: impl FnMut(T, &mut Self),
    ) {
        let values: Vec<_> = values.into_iter().collect();
        self.u64(values.len() as u64);
        for value in values {
            encode(value, self);
        }
    }
}

/// Runner resolution failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RunnerResolutionError {
    /// ID or revision differs from explicit authority.
    AuthorityMismatch,
    /// Backend is outside operator authority.
    BackendNotAuthorized,
    /// Timeout exceeds the host cap.
    TimeoutExceedsAuthority,
    /// Output bound exceeds the host cap.
    OutputExceedsAuthority,
    /// Concurrency exceeds the host cap.
    ConcurrencyExceedsAuthority,
    /// A claimed guarantee is not enforceable by the configured backend.
    GuaranteeNotEnforceable,
    /// Repository executable has no independently verified digest.
    ProgramNotAuthorized,
}

impl Display for RunnerResolutionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for RunnerResolutionError {}

#[cfg(test)]
mod tests {
    use super::*;
    use eqm_domain::{
        EnvironmentBinding, EnvironmentName, Extensions, OwnerRef, RunnerLimits, SelectorText,
    };
    use std::error::Error;

    fn definition(reverse: bool) -> Result<RunnerDefinition, Box<dyn Error>> {
        let mut owners = vec![
            "owner://team/platform".parse::<OwnerRef>()?,
            "owner://role/reviewer".parse::<OwnerRef>()?,
        ];
        let mut environment = vec![
            EnvironmentBinding::new(
                EnvironmentName::new("LANG")?,
                EnvironmentSource::CanonicalLocale,
            )?,
            EnvironmentBinding::new(
                EnvironmentName::new("MODE")?,
                EnvironmentSource::Literal(SelectorText::new("test")?),
            )?,
        ];
        if reverse {
            owners.reverse();
            environment.reverse();
        }
        Ok(RunnerDefinition::new(
            RunnerId::new("runner.tests")?,
            Revision::new(1)?,
            owners,
            RunnerBackend::Local,
            RunnerProgram::Repository(RepoPath::new("tools/test-runner")?),
            vec![ArgumentTemplate::SelectorJson, ArgumentTemplate::ResultPath],
            None,
            environment,
            Vec::new(),
            RunnerLimits::new(
                DurationMillis::new(30_000)?,
                PositiveCount::new(1_024)?,
                None,
            )?,
            Vec::new(),
            Extensions::default(),
        )?)
    }

    fn authority_fixture() -> Result<RunnerResolutionAuthority, Box<dyn Error>> {
        Ok(RunnerResolutionAuthority {
            id: RunnerId::new("runner.tests")?,
            revision: Revision::new(1)?,
            backends: BTreeSet::from([RunnerBackend::Local]),
            repository_programs: BTreeMap::from([(
                RepoPath::new("tools/test-runner")?,
                Sha256Digest::hash_content(b"runner"),
            )]),
            backend_guarantees: BTreeMap::from([(RunnerBackend::Local, BTreeSet::new())]),
            maximum_timeout: DurationMillis::new(60_000)?,
            maximum_output_bytes: PositiveCount::new(2_048)?,
            maximum_concurrency: PositiveCount::new(2)?,
        })
    }

    #[test]
    fn equivalent_definitions_resolve_to_one_digest() -> Result<(), Box<dyn Error>> {
        let authority = authority_fixture()?;
        let first = resolve_runner(&definition(false)?, &authority)?;
        let second = resolve_runner(&definition(true)?, &authority)?;
        assert_eq!(first.digest(), second.digest());
        assert!(matches!(
            first.program(),
            ResolvedProgram::Repository { .. }
        ));
        Ok(())
    }

    #[test]
    fn authority_backend_resource_and_program_mismatches_fail() -> Result<(), Box<dyn Error>> {
        let definition = definition(false)?;
        let mut authority = authority_fixture()?;
        authority.revision = Revision::new(2)?;
        assert_eq!(
            resolve_runner(&definition, &authority),
            Err(RunnerResolutionError::AuthorityMismatch)
        );
        let mut authority = authority_fixture()?;
        authority.backends.clear();
        assert_eq!(
            resolve_runner(&definition, &authority),
            Err(RunnerResolutionError::BackendNotAuthorized)
        );
        let mut authority = authority_fixture()?;
        authority.maximum_timeout = DurationMillis::new(1_000)?;
        assert_eq!(
            resolve_runner(&definition, &authority),
            Err(RunnerResolutionError::TimeoutExceedsAuthority)
        );
        let mut authority = authority_fixture()?;
        authority.repository_programs.clear();
        assert_eq!(
            resolve_runner(&definition, &authority),
            Err(RunnerResolutionError::ProgramNotAuthorized)
        );
        Ok(())
    }
}
