//! Approved runner-definition resolution and deterministic identity.
//!
//! The resolved identity binds the complete semantic [`RunnerDefinition`] to
//! the independently verified executable content and the approved execution
//! context (the enforceable backend guarantees). A definition digest alone is
//! insufficient: replacing the bytes at the same repository path must change
//! the identity, and so must a semantic definition change. A repository
//! executable without an independently verified digest stays unresolved and
//! non-executable (`ProgramNotAuthorized`).

use crate::identity::{CanonicalRunnerIdentity, RunnerIdentityError, canonical_runner_identity_v2};
use eqm_domain::{
    DurationMillis, PositiveCount, RepoPath, RepositoryIdentity, Revision, RunnerBackend,
    RunnerDefinition, RunnerGuarantee, RunnerId, RunnerProgram, Sha256Digest,
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
    approved_guarantees: BTreeSet<RunnerGuarantee>,
    identity: CanonicalRunnerIdentity,
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

    /// Returns the approved enforceable backend guarantees bound into the
    /// runner identity.
    #[must_use]
    pub const fn approved_guarantees(&self) -> &BTreeSet<RunnerGuarantee> {
        &self.approved_guarantees
    }

    /// Returns the immutable canonical v2 runner identity bytes.
    ///
    /// These are the exact JCS bytes hashed under `eqm:v2:runner` by
    /// [`Self::identity_v2`]. They include the complete definition, the
    /// resolved program and the approved enforceable guarantees.
    #[must_use]
    pub fn canonical_bytes(&self) -> &[u8] {
        self.identity.bytes()
    }

    /// Returns the typed domain-separated v2 runner identity.
    ///
    /// The returned [`RunnerDigest`](eqm_domain::RunnerDigest) cannot be
    /// substituted where another domain identity is required:
    ///
    /// ```compile_fail
    /// use eqm_domain::{RunnerDigest, TargetConfigurationDigest};
    /// fn requires_target(_: TargetConfigurationDigest) {}
    /// requires_target(RunnerDigest::hash_content(b"definition"));
    /// ```
    ///
    /// The positive typed form constructs and compares under its own domain:
    ///
    /// ```
    /// use eqm_domain::{RunnerDigest, Sha256Digest};
    /// let first = RunnerDigest::hash_content(b"definition");
    /// let second = RunnerDigest::hash_content(b"definition");
    /// let other = RunnerDigest::hash_content(b"other");
    /// assert_eq!(first, second);
    /// assert_ne!(first, other);
    /// let plain: Sha256Digest = first.as_digest();
    /// assert_eq!(plain.to_string(), first.to_string());
    /// ```
    #[must_use]
    pub const fn identity_v2(&self) -> eqm_domain::RunnerDigest {
        self.identity.digest()
    }

    /// Legacy/wire adapter returning the underlying digest for the existing
    /// evidence DTO field.
    ///
    /// This is exactly [`Self::identity_v2`]'s digest. New v2 consumers must use
    /// [`Self::identity_v2`]; this adapter exists only to feed the established
    /// `runner_digest` string without adding a v1 wire field.
    #[must_use]
    pub const fn digest(&self) -> Sha256Digest {
        self.identity_v2().as_digest()
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
    let identity = canonical_runner_identity_v2(definition, &program, &enforceable)
        .map_err(map_identity_error)?;
    Ok(ResolvedRunner {
        identity,
        definition: definition.clone(),
        program,
        approved_guarantees: enforceable,
    })
}

fn map_identity_error(error: RunnerIdentityError) -> RunnerResolutionError {
    match error {
        RunnerIdentityError::TooLarge => RunnerResolutionError::IdentityTooLarge,
        RunnerIdentityError::Serialization => RunnerResolutionError::IdentitySerialization,
        RunnerIdentityError::DuplicateMember => RunnerResolutionError::IdentityDuplicateMember,
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
    /// Canonical runner-identity projection exceeded the component byte bound.
    IdentityTooLarge,
    /// Canonical runner-identity projection could not be serialized.
    IdentitySerialization,
    /// Canonical runner-identity projection rejected a duplicate set member.
    IdentityDuplicateMember,
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
        ArgumentTemplate, EnvironmentBinding, EnvironmentName, EnvironmentSource, Extensions,
        OwnerRef, RunnerLimits, SelectorText,
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

    fn definition_with_timeout(timeout: u64) -> Result<RunnerDefinition, Box<dyn Error>> {
        Ok(RunnerDefinition::new(
            RunnerId::new("runner.tests")?,
            Revision::new(1)?,
            vec!["owner://team/platform".parse::<OwnerRef>()?],
            RunnerBackend::Local,
            RunnerProgram::Repository(RepoPath::new("tools/test-runner")?),
            vec![ArgumentTemplate::SelectorJson, ArgumentTemplate::ResultPath],
            None,
            Vec::new(),
            Vec::new(),
            RunnerLimits::new(
                DurationMillis::new(timeout)?,
                PositiveCount::new(1_024)?,
                None,
            )?,
            Vec::new(),
            Extensions::default(),
        )?)
    }

    #[test]
    fn executable_replacement_changes_runner_identity() -> Result<(), Box<dyn Error>> {
        let definition = definition(false)?;
        let mut authority = authority_fixture()?;
        let first = resolve_runner(&definition, &authority)?;
        // Replace the bytes at the same repository path.
        authority.repository_programs.insert(
            RepoPath::new("tools/test-runner")?,
            Sha256Digest::hash_content(b"runner-v2"),
        );
        let second = resolve_runner(&definition, &authority)?;
        assert_ne!(
            first.digest(),
            second.digest(),
            "replacing executable bytes at the same path must change the identity"
        );
        Ok(())
    }

    #[test]
    fn definition_only_change_invalidates_runner_identity() -> Result<(), Box<dyn Error>> {
        let authority = authority_fixture()?;
        let first = resolve_runner(&definition_with_timeout(30_000)?, &authority)?;
        let second = resolve_runner(&definition_with_timeout(31_000)?, &authority)?;
        assert_ne!(
            first.digest(),
            second.digest(),
            "a definition-only change must change the identity"
        );
        Ok(())
    }

    #[test]
    fn approved_execution_context_changes_runner_identity() -> Result<(), Box<dyn Error>> {
        let definition = definition(false)?;
        let first = resolve_runner(&definition, &authority_fixture()?)?;
        let mut widened = authority_fixture()?;
        widened.backend_guarantees.insert(
            RunnerBackend::Local,
            BTreeSet::from([RunnerGuarantee::IsolatedProcess]),
        );
        let second = resolve_runner(&definition, &widened)?;
        assert_ne!(
            first.digest(),
            second.digest(),
            "a changed approved execution context must change the identity"
        );
        Ok(())
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
    fn literal_v2_runner_identity_bytes_and_digest_match_independent_oracle()
    -> Result<(), Box<dyn Error>> {
        // Reviewed literal bytes, independently recomputed as
        // SHA256("eqm:v2:runner" || 0x00 || bytes).
        const BYTES: &str = r#"{"approved_guarantees":[],"definition":{"args":[{"kind":"selector_json"},{"kind":"result_path"}],"backend":"local","cwd":{"kind":"target_root"},"environment":[{"name":"LANG","source":"canonical_locale"},{"name":"MODE","source":"literal","value":"test"}],"extensions":{},"guarantees":[],"id":"runner.tests","max_concurrency":1,"max_output_bytes":1024,"owners":["owner://role/reviewer","owner://team/platform"],"program":{"kind":"repository","path":"tools/test-runner"},"revision":1,"secrets":[],"timeout_ms":30000},"program":{"digest":"sha256:527aa9f431539da8e151d5434d1d5e611d973f601d8e970790882624554146b0","kind":"repository","path":"tools/test-runner"}}"#;
        const DIGEST: &str =
            "sha256:fe1510783141a3e0978460e70c06506cc1b7580404c1b4468e3bd2860885d428";
        let runner = resolve_runner(&definition(false)?, &authority_fixture()?)?;
        assert_eq!(std::str::from_utf8(runner.canonical_bytes())?, BYTES);
        assert_eq!(runner.identity_v2().to_string(), DIGEST);
        // The wire adapter must remain exactly the typed v2 digest.
        assert_eq!(runner.digest().to_string(), DIGEST);
        Ok(())
    }

    #[test]
    fn actual_executable_replacement_changes_identity_through_acquisition()
    -> Result<(), Box<dyn Error>> {
        // Mirrors the CLI repository-authority acquisition: a regular repository
        // file is read with `fs::read` and hashed with `Sha256Digest::hash_content`.
        let root = tempfile::tempdir()?;
        let program = RepoPath::new("bin/runner")?;
        let absolute = root.path().join(program.as_str());
        std::fs::create_dir_all(absolute.parent().ok_or("program parent")?)?;
        std::fs::write(&absolute, b"#!/bin/sh\nexit 0\n")?;
        let definition = RunnerDefinition::new(
            RunnerId::new("runner.tests")?,
            Revision::new(1)?,
            vec!["owner://team/platform".parse::<OwnerRef>()?],
            RunnerBackend::Local,
            RunnerProgram::Repository(program.clone()),
            vec![ArgumentTemplate::SelectorJson],
            None,
            Vec::new(),
            Vec::new(),
            RunnerLimits::new(
                DurationMillis::new(30_000)?,
                PositiveCount::new(1_024)?,
                None,
            )?,
            Vec::new(),
            Extensions::default(),
        )?;
        let authority = |path: &std::path::Path| -> Result<_, Box<dyn Error>> {
            Ok(RunnerResolutionAuthority {
                id: RunnerId::new("runner.tests")?,
                revision: Revision::new(1)?,
                backends: BTreeSet::from([RunnerBackend::Local]),
                repository_programs: BTreeMap::from([(
                    program.clone(),
                    Sha256Digest::hash_content(&std::fs::read(path)?),
                )]),
                backend_guarantees: BTreeMap::from([(RunnerBackend::Local, BTreeSet::new())]),
                maximum_timeout: DurationMillis::new(30_000)?,
                maximum_output_bytes: PositiveCount::new(1_024)?,
                maximum_concurrency: PositiveCount::ONE,
            })
        };
        let first = resolve_runner(&definition, &authority(&absolute)?)?;
        // Replace the actual bytes on disk and re-observe the digest.
        std::fs::write(&absolute, b"#!/bin/sh\nexit 1\n")?;
        let second = resolve_runner(&definition, &authority(&absolute)?)?;
        assert_ne!(first.identity_v2(), second.identity_v2());
        assert_ne!(first.canonical_bytes(), second.canonical_bytes());
        Ok(())
    }

    #[test]
    fn locked_program_identity_binds_locked_resolution_and_digest() -> Result<(), Box<dyn Error>> {
        let image: RepositoryIdentity = "https://registry.example/images/test".parse()?;
        let digest = Sha256Digest::hash_content(b"image");
        let definition = RunnerDefinition::new(
            RunnerId::new("runner.container")?,
            Revision::new(1)?,
            vec!["owner://team/platform".parse::<OwnerRef>()?],
            RunnerBackend::Container,
            RunnerProgram::Locked {
                resolved: image.clone(),
                digest,
            },
            vec![ArgumentTemplate::SelectorJson],
            None,
            Vec::new(),
            Vec::new(),
            RunnerLimits::new(
                DurationMillis::new(30_000)?,
                PositiveCount::new(1_024)?,
                None,
            )?,
            vec![RunnerGuarantee::NetworkDenied],
            Extensions::default(),
        )?;
        let authority = RunnerResolutionAuthority {
            id: RunnerId::new("runner.container")?,
            revision: Revision::new(1)?,
            backends: BTreeSet::from([RunnerBackend::Container]),
            repository_programs: BTreeMap::new(),
            backend_guarantees: BTreeMap::from([(
                RunnerBackend::Container,
                BTreeSet::from([RunnerGuarantee::NetworkDenied]),
            )]),
            maximum_timeout: DurationMillis::new(30_000)?,
            maximum_output_bytes: PositiveCount::new(1_024)?,
            maximum_concurrency: PositiveCount::ONE,
        };
        let resolved = resolve_runner(&definition, &authority)?;
        assert!(matches!(resolved.program(), ResolvedProgram::Locked { .. }));
        // The same locked definition resolves to a stable identity.
        assert_eq!(
            resolved.identity_v2(),
            resolve_runner(&definition, &authority)?.identity_v2()
        );
        Ok(())
    }
}
