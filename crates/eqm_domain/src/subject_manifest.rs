//! Exact multi-participant subject manifests (DEV-009 S038).
//!
//! A subject manifest assigns one exact identity per scenario occurrence. Each
//! assignment keeps the repository, source commit, immutable source/component
//! content identity, build ID and artifact digest separate from the target or
//! provider configuration identity, so a target digest alone can never satisfy a
//! missing source or build input. Absence is typed and allowed only for declared
//! development context; trusted completeness requires every relevant axis.

use crate::{
    OccurrenceId, ProviderConfigurationDigest, RepositoryIdentity, Revision, SelectorText,
    Sha256Digest, SourceCommit, TargetConfigurationDigest,
};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// One exact occurrence subject assignment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubjectAssignment {
    occurrence: OccurrenceId,
    repository: RepositoryIdentity,
    source_commit: SourceCommit,
    source_content: Option<Sha256Digest>,
    target_configuration: Option<TargetConfigurationDigest>,
    provider_configuration: Option<ProviderConfigurationDigest>,
    build_id: Option<SelectorText>,
    artifact_digest: Option<Sha256Digest>,
}

impl SubjectAssignment {
    /// Creates one assignment with exactly one configuration authority.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        occurrence: OccurrenceId,
        repository: RepositoryIdentity,
        source_commit: SourceCommit,
        source_content: Option<Sha256Digest>,
        target_configuration: Option<TargetConfigurationDigest>,
        provider_configuration: Option<ProviderConfigurationDigest>,
        build_id: Option<SelectorText>,
        artifact_digest: Option<Sha256Digest>,
    ) -> Result<Self, SubjectManifestError> {
        if target_configuration.is_some() == provider_configuration.is_some() {
            return Err(SubjectManifestError::ConfigurationAuthority);
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

    /// Returns the exact target-configuration identity, if this is a target.
    #[must_use]
    pub const fn target_configuration(&self) -> Option<TargetConfigurationDigest> {
        self.target_configuration
    }

    /// Returns the exact provider-configuration identity, if this is a provider.
    #[must_use]
    pub const fn provider_configuration(&self) -> Option<ProviderConfigurationDigest> {
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

    /// Returns true only when every relevant axis is present.
    ///
    /// A target-configuration identity alone never satisfies a missing source
    /// content, build or artifact axis.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.source_content.is_some() && self.build_id.is_some() && self.artifact_digest.is_some()
    }
}

/// One validated multi-participant subject manifest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubjectManifest {
    id: SelectorText,
    revision: Revision,
    assignments: Vec<SubjectAssignment>,
}

impl SubjectManifest {
    /// Creates a manifest with nonempty, uniquely keyed assignments.
    pub fn new(
        id: SelectorText,
        revision: Revision,
        assignments: Vec<SubjectAssignment>,
    ) -> Result<Self, SubjectManifestError> {
        if assignments.is_empty() {
            return Err(SubjectManifestError::AssignmentsRequired);
        }
        let mut occurrences = BTreeSet::new();
        for assignment in &assignments {
            if !occurrences.insert(assignment.occurrence().clone()) {
                return Err(SubjectManifestError::DuplicateAssignment);
            }
        }
        let mut assignments = assignments;
        assignments.sort_by(|left, right| left.occurrence().cmp(right.occurrence()));
        Ok(Self {
            id,
            revision,
            assignments,
        })
    }

    /// Returns the manifest identity.
    #[must_use]
    pub const fn id(&self) -> &SelectorText {
        &self.id
    }

    /// Returns the exact manifest revision.
    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    /// Returns assignments in canonical occurrence order.
    #[must_use]
    pub fn assignments(&self) -> &[SubjectAssignment] {
        &self.assignments
    }

    /// Requires every assignment to carry its complete trusted identity.
    ///
    /// Declared development context may hold absent axes; trusted completeness
    /// never substitutes zero values or a target digest for them.
    pub fn require_complete(&self) -> Result<(), SubjectManifestError> {
        if self.assignments.iter().all(SubjectAssignment::is_complete) {
            Ok(())
        } else {
            Err(SubjectManifestError::IncompleteSubject)
        }
    }
}

/// Subject-manifest construction or completeness failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubjectManifestError {
    /// No occurrence assignment was declared.
    AssignmentsRequired,
    /// Two assignments shared one occurrence ID.
    DuplicateAssignment,
    /// Neither or both of the target/provider configuration identities were set.
    ConfigurationAuthority,
    /// A trusted-complete assignment omitted a relevant source, build or artifact axis.
    IncompleteSubject,
}

impl Display for SubjectManifestError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::AssignmentsRequired => "subject manifest requires at least one assignment",
            Self::DuplicateAssignment => "subject manifest occurrences contain a duplicate",
            Self::ConfigurationAuthority => {
                "subject assignment requires exactly one configuration authority"
            }
            Self::IncompleteSubject => "subject assignment omits a relevant source/build axis",
        })
    }
}

impl Error for SubjectManifestError {}
