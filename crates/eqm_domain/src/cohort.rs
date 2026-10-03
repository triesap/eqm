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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CohortAssignment {
    occurrence: OccurrenceId,
    repository: RepositoryIdentity,
    source_commit: SourceCommit,
    source_content: Option<Sha256Digest>,
    build_id: Option<SelectorText>,
    artifact_digest: Option<Sha256Digest>,
}

impl CohortAssignment {
    /// Creates one assignment from explicit content axes.
    #[must_use]
    pub const fn new(
        occurrence: OccurrenceId,
        repository: RepositoryIdentity,
        source_commit: SourceCommit,
        source_content: Option<Sha256Digest>,
        build_id: Option<SelectorText>,
        artifact_digest: Option<Sha256Digest>,
    ) -> Self {
        Self {
            occurrence,
            repository,
            source_commit,
            source_content,
            build_id,
            artifact_digest,
        }
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

/// Cohort construction failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CohortError {
    /// No occurrence assignment was declared.
    AssignmentsRequired,
    /// Two assignments shared one occurrence ID.
    DuplicateAssignment,
}

impl Display for CohortError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::AssignmentsRequired => "cohort requires at least one assignment",
            Self::DuplicateAssignment => "cohort assignments contain a duplicate occurrence",
        })
    }
}

impl Error for CohortError {}
