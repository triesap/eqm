//! Deterministic exact subject builders for fixtures.
//!
//! Every value is derived from fixed fixture inputs, so repeated construction
//! yields identical identities. The builders delegate all coordinate
//! validation to the domain types and reject malformed targets, providers, and
//! empty or duplicate target sets rather than inventing a subject.

use crate::error::FixtureError;
use eqm_domain::{
    EvidenceScopeSubject, EvidenceSubject, ProviderId, RepositoryIdentity, Sha256Digest,
    SourceCommit, TargetId,
};

/// The default fixture repository URI.
pub const FIXTURE_REPOSITORY: &str = "https://github.com/example/eqm-fixture";

/// The default fixture source commit: a full lowercase object ID.
pub const FIXTURE_COMMIT: &str = "0123456789abcdef0123456789abcdef01234567";

/// Builds replay-bound evidence subjects from deterministic fixture inputs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubjectBuilder {
    repository: String,
    commit: String,
}

impl Default for SubjectBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SubjectBuilder {
    /// Creates a builder with the default fixture repository and commit.
    #[must_use]
    pub fn new() -> Self {
        Self {
            repository: FIXTURE_REPOSITORY.to_owned(),
            commit: FIXTURE_COMMIT.to_owned(),
        }
    }

    /// Overrides the repository identity.
    #[must_use]
    pub fn repository(mut self, value: &str) -> Self {
        self.repository = value.to_owned();
        self
    }

    /// Overrides the source commit identity.
    #[must_use]
    pub fn commit(mut self, value: &str) -> Self {
        self.commit = value.to_owned();
        self
    }

    /// Builds a single-target subject.
    pub fn target(self, target: &str) -> Result<EvidenceSubject, FixtureError> {
        let target: TargetId = target.parse().map_err(FixtureError::new)?;
        self.assemble(EvidenceScopeSubject::Target(target))
    }

    /// Builds a single-provider subject.
    pub fn provider(self, provider: &str) -> Result<EvidenceSubject, FixtureError> {
        let provider: ProviderId = provider.parse().map_err(FixtureError::new)?;
        self.assemble(EvidenceScopeSubject::Provider(provider))
    }

    /// Builds a nonempty duplicate-free target-set subject.
    pub fn target_set(self, targets: &[&str]) -> Result<EvidenceSubject, FixtureError> {
        let mut parsed = Vec::with_capacity(targets.len());
        for target in targets {
            parsed.push(target.parse::<TargetId>().map_err(FixtureError::new)?);
        }
        self.assemble(EvidenceScopeSubject::target_set(parsed).map_err(FixtureError::new)?)
    }

    fn assemble(self, scope: EvidenceScopeSubject) -> Result<EvidenceSubject, FixtureError> {
        let repository: RepositoryIdentity = self.repository.parse().map_err(FixtureError::new)?;
        let commit: SourceCommit = self.commit.parse().map_err(FixtureError::new)?;
        let repository_id_digest = Sha256Digest::hash_content(self.repository.as_bytes());
        let target_configuration_digest =
            Sha256Digest::hash_content(b"fixture:target-configuration");
        Ok(EvidenceSubject::new(
            repository,
            repository_id_digest,
            scope,
            commit,
            None,
            None,
            target_configuration_digest,
        ))
    }
}

/// Builds a deterministic single-target subject.
pub fn target_subject(target: &str) -> Result<EvidenceSubject, FixtureError> {
    SubjectBuilder::new().target(target)
}

/// Builds a deterministic single-provider subject.
pub fn provider_subject(provider: &str) -> Result<EvidenceSubject, FixtureError> {
    SubjectBuilder::new().provider(provider)
}

/// Builds a deterministic multi-target subject.
pub fn target_set_subject(targets: &[&str]) -> Result<EvidenceSubject, FixtureError> {
    SubjectBuilder::new().target_set(targets)
}
