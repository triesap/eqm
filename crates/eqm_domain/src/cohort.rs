//! Authored compatibility-cohort authority (DEV-009 S041).
//!
//! A cohort declares one approved combination of exact participant subjects for
//! one scenario. Its assignments reuse the S038 content axes (repository,
//! source commit, source/component content, build and artifact) keyed by unique
//! occurrence; the target/provider *configuration* authority comes from the
//! immutable graph projection, not the authored document. Loading or holding a
//! cohort proves neither that the declared source/build material was acquired
//! nor that it is trusted.

use crate::{
    Extensions, OccurrenceId, RepositoryIdentity, Revision, ScenarioId, SelectorText, Sha256Digest,
    SourceCommit,
};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// One authored cohort assignment for an exact scenario occurrence.
///
/// The assignment retains the authored target/provider configuration claim
/// exactly one of which must be present. That claim is validated SHA-256 text
/// compared against the independently projected typed graph configuration at
/// resolution time; it is never substituted, and the graph projection is not
/// authorized to delete it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CohortAssignment {
    occurrence: OccurrenceId,
    repository: RepositoryIdentity,
    source_commit: SourceCommit,
    source_content: Option<Sha256Digest>,
    target_configuration: Option<Sha256Digest>,
    provider_configuration: Option<Sha256Digest>,
    build_id: Option<SelectorText>,
    artifact_digest: Option<Sha256Digest>,
}

impl CohortAssignment {
    /// Creates one assignment with exactly one authored configuration claim.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        occurrence: OccurrenceId,
        repository: RepositoryIdentity,
        source_commit: SourceCommit,
        source_content: Option<Sha256Digest>,
        target_configuration: Option<Sha256Digest>,
        provider_configuration: Option<Sha256Digest>,
        build_id: Option<SelectorText>,
        artifact_digest: Option<Sha256Digest>,
    ) -> Result<Self, CohortError> {
        if target_configuration.is_some() == provider_configuration.is_some() {
            return Err(CohortError::ConfigurationAuthority);
        }
        Ok(Self {
            occurrence,
            repository,
            source_commit,
            source_content,
            target_configuration,
            provider_configuration,
            build_id,
            artifact_digest,
        })
    }

    /// Returns the occurrence this assignment keys.
    #[must_use]
    pub const fn occurrence(&self) -> &OccurrenceId {
        &self.occurrence
    }

    /// Returns the exact source repository.
    #[must_use]
    pub const fn repository(&self) -> &RepositoryIdentity {
        &self.repository
    }

    /// Returns the exact source commit.
    #[must_use]
    pub const fn source_commit(&self) -> &SourceCommit {
        &self.source_commit
    }

    /// Returns the immutable source/component content identity, if present.
    #[must_use]
    pub const fn source_content(&self) -> Option<Sha256Digest> {
        self.source_content
    }

    /// Returns the authored target-configuration claim, if this is a target.
    #[must_use]
    pub const fn target_configuration(&self) -> Option<Sha256Digest> {
        self.target_configuration
    }

    /// Returns the authored provider-configuration claim, if this is a provider.
    #[must_use]
    pub const fn provider_configuration(&self) -> Option<Sha256Digest> {
        self.provider_configuration
    }

    /// Returns the exact build identifier, if present.
    #[must_use]
    pub const fn build_id(&self) -> Option<&SelectorText> {
        self.build_id.as_ref()
    }

    /// Returns the exact build artifact digest, if present.
    #[must_use]
    pub const fn artifact_digest(&self) -> Option<Sha256Digest> {
        self.artifact_digest
    }

    /// Returns true only when every content axis is present.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.source_content.is_some() && self.build_id.is_some() && self.artifact_digest.is_some()
    }
}

/// One authored compatibility cohort.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cohort {
    id: SelectorText,
    revision: Revision,
    scenario: ScenarioId,
    assignments: Vec<CohortAssignment>,
    extensions: Extensions,
}

impl Cohort {
    /// Creates a cohort with nonempty, uniquely keyed assignments.
    pub fn new(
        id: SelectorText,
        revision: Revision,
        scenario: ScenarioId,
        assignments: Vec<CohortAssignment>,
        extensions: Extensions,
    ) -> Result<Self, CohortError> {
        if assignments.is_empty() {
            return Err(CohortError::AssignmentsRequired);
        }
        let mut occurrences = BTreeSet::new();
        for assignment in &assignments {
            if !occurrences.insert(assignment.occurrence().clone()) {
                return Err(CohortError::DuplicateAssignment);
            }
        }
        let mut assignments = assignments;
        assignments.sort_by(|left, right| left.occurrence().cmp(right.occurrence()));
        Ok(Self {
            id,
            revision,
            scenario,
            assignments,
            extensions,
        })
    }

    /// Returns the cohort identity.
    #[must_use]
    pub const fn id(&self) -> &SelectorText {
        &self.id
    }

    /// Returns the exact cohort revision.
    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    /// Returns the exact scenario this cohort covers.
    #[must_use]
    pub const fn scenario(&self) -> &ScenarioId {
        &self.scenario
    }

    /// Returns assignments in canonical occurrence order.
    #[must_use]
    pub fn assignments(&self) -> &[CohortAssignment] {
        &self.assignments
    }

    /// Returns semantic extensions.
    #[must_use]
    pub const fn extensions(&self) -> &Extensions {
        &self.extensions
    }
}

/// Cohort construction or assignment failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CohortError {
    /// No occurrence assignment was declared.
    AssignmentsRequired,
    /// Two assignments shared one occurrence ID.
    DuplicateAssignment,
    /// Neither or both of the target/provider configuration claims were set.
    ConfigurationAuthority,
}

impl Display for CohortError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::AssignmentsRequired => "cohort requires at least one assignment",
            Self::DuplicateAssignment => "cohort assignments contain a duplicate occurrence",
            Self::ConfigurationAuthority => {
                "cohort assignment requires exactly one configuration authority"
            }
        })
    }
}

impl Error for CohortError {}
