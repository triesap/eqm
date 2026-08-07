//! Immutable, replay-bound evidence result values.

use crate::{
    AttemptOutcome, DimensionId, EvidenceKind, Extensions, Facet, FullRequirementId, OwnerRef,
    PositiveCount, ProfileId, ProviderId, Revision, SelectorText, Sha256Digest, SymbolicValueId,
    TargetId, TrustLevel, UnitId, UtcInstant,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// A canonical configured repository identity URI.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RepositoryIdentity(Box<str>);

impl RepositoryIdentity {
    /// Returns the exact canonical HTTPS URI.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for RepositoryIdentity {
    type Err = EvidenceResultBuildError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let Some(rest) = value.strip_prefix("https://") else {
            return Err(EvidenceResultBuildError::InvalidRepositoryIdentity);
        };
        let Some((host, path)) = rest.split_once('/') else {
            return Err(EvidenceResultBuildError::InvalidRepositoryIdentity);
        };
        if value.len() > 1_024
            || host.is_empty()
            || path.is_empty()
            || value.ends_with('/')
            || value.contains(['?', '#', '@'])
            || !host.is_ascii()
            || host
                .bytes()
                .any(|byte| byte.is_ascii_uppercase() || byte.is_ascii_control())
            || path
                .bytes()
                .any(|byte| byte.is_ascii_control() || byte == b' ')
        {
            return Err(EvidenceResultBuildError::InvalidRepositoryIdentity);
        }
        Ok(Self(value.into()))
    }
}

/// An immutable full Git object ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceCommit(Box<str>);

impl SourceCommit {
    /// Returns the lowercase hexadecimal object ID.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for SourceCommit {
    type Err = EvidenceResultBuildError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if !matches!(value.len(), 40 | 64)
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(EvidenceResultBuildError::InvalidSourceCommit);
        }
        Ok(Self(value.into()))
    }
}

/// The exact evaluated scope subject.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceScopeSubject {
    /// One target.
    Target(TargetId),
    /// One shared provider.
    Provider(ProviderId),
    /// A nonempty target set.
    TargetSet(BTreeSet<TargetId>),
}

impl EvidenceScopeSubject {
    /// Creates a nonempty duplicate-free target set.
    pub fn target_set(targets: Vec<TargetId>) -> Result<Self, EvidenceResultBuildError> {
        Ok(Self::TargetSet(unique_nonempty(
            targets,
            EvidenceResultBuildError::TargetsRequired,
            EvidenceResultBuildError::DuplicateTarget,
        )?))
    }
}

/// Replay-bound repository and implementation subject.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceSubject {
    repository: RepositoryIdentity,
    repository_id_digest: Sha256Digest,
    scope: EvidenceScopeSubject,
    source_commit: SourceCommit,
    build_id: Option<SelectorText>,
    artifact_digest: Option<Sha256Digest>,
    target_configuration_digest: Sha256Digest,
}

impl EvidenceSubject {
    /// Creates an exact evidence subject.
    #[must_use]
    pub const fn new(
        repository: RepositoryIdentity,
        repository_id_digest: Sha256Digest,
        scope: EvidenceScopeSubject,
        source_commit: SourceCommit,
        build_id: Option<SelectorText>,
        artifact_digest: Option<Sha256Digest>,
        target_configuration_digest: Sha256Digest,
    ) -> Self {
        Self {
            repository,
            repository_id_digest,
            scope,
            source_commit,
            build_id,
            artifact_digest,
            target_configuration_digest,
        }
    }

    /// Returns repository identity.
    #[must_use]
    pub const fn repository(&self) -> &RepositoryIdentity {
        &self.repository
    }
    /// Returns exact scope identity.
    #[must_use]
    pub const fn scope(&self) -> &EvidenceScopeSubject {
        &self.scope
    }
    /// Returns source commit identity.
    #[must_use]
    pub const fn source_commit(&self) -> &SourceCommit {
        &self.source_commit
    }
    /// Returns the protected repository-ID digest.
    #[must_use]
    pub const fn repository_id_digest(&self) -> Sha256Digest {
        self.repository_id_digest
    }
    /// Returns optional build identity.
    #[must_use]
    pub const fn build_id(&self) -> Option<&SelectorText> {
        self.build_id.as_ref()
    }
    /// Returns optional artifact digest.
    #[must_use]
    pub const fn artifact_digest(&self) -> Option<Sha256Digest> {
        self.artifact_digest
    }
    /// Returns target-configuration digest.
    #[must_use]
    pub const fn target_configuration_digest(&self) -> Sha256Digest {
        self.target_configuration_digest
    }
}

/// An opaque canonical producer identity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProducerRef(Box<str>);

impl ProducerRef {
    /// Returns the exact producer URI.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for ProducerRef {
    type Err = EvidenceResultBuildError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let Some(rest) = value.strip_prefix("producer://") else {
            return Err(EvidenceResultBuildError::InvalidProducer);
        };
        let parts: Vec<_> = rest.split('/').collect();
        if parts.len() != 3
            || !matches!(
                parts[0],
                "local" | "ci" | "human" | "adapter" | "runtime" | "release"
            )
            || !id_segment(parts[1])
            || !token(parts[2])
        {
            return Err(EvidenceResultBuildError::InvalidProducer);
        }
        Ok(Self(value.into()))
    }
}

fn id_segment(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 63
        && value.as_bytes()[0].is_ascii_lowercase()
        && value
            .bytes()
            .skip(1)
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn token(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.as_bytes()[0].is_ascii_alphanumeric()
        && value
            .bytes()
            .skip(1)
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

/// One exact versioned profile selection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileSelection {
    profile: ProfileId,
    revision: Revision,
    values: BTreeMap<DimensionId, SymbolicValueId>,
}

impl ProfileSelection {
    /// Creates a nonempty duplicate-free dimension selection.
    pub fn new(
        profile: ProfileId,
        revision: Revision,
        values: Vec<(DimensionId, SymbolicValueId)>,
    ) -> Result<Self, EvidenceResultBuildError> {
        if values.is_empty() {
            return Err(EvidenceResultBuildError::ProfileValuesRequired);
        }
        let count = values.len();
        let values: BTreeMap<_, _> = values.into_iter().collect();
        if values.len() != count {
            return Err(EvidenceResultBuildError::DuplicateDimension);
        }
        Ok(Self {
            profile,
            revision,
            values,
        })
    }
    /// Returns the profile ID.
    #[must_use]
    pub const fn profile(&self) -> &ProfileId {
        &self.profile
    }
    /// Returns the exact profile revision.
    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }
    /// Returns resolved values by dimension.
    #[must_use]
    pub const fn values(&self) -> &BTreeMap<DimensionId, SymbolicValueId> {
        &self.values
    }
}

/// Exact normalized count totals.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EvidenceCounts {
    /// Selected items.
    pub selected: u64,
    /// Passed items.
    pub passed: u64,
    /// Failed items.
    pub failed: u64,
    /// Skipped items.
    pub skipped: u64,
    /// Filtered items.
    pub filtered: u64,
    /// Quarantined items.
    pub quarantined: u64,
}

impl EvidenceCounts {
    /// Validates exact internal totals and a nonzero selected count.
    pub fn new(
        selected: u64,
        passed: u64,
        failed: u64,
        skipped: u64,
        filtered: u64,
        quarantined: u64,
    ) -> Result<Self, EvidenceResultBuildError> {
        let total = passed
            .checked_add(failed)
            .and_then(|value| value.checked_add(skipped))
            .and_then(|value| value.checked_add(filtered))
            .and_then(|value| value.checked_add(quarantined))
            .ok_or(EvidenceResultBuildError::InconsistentCounts)?;
        if selected == 0 || selected != total {
            return Err(EvidenceResultBuildError::InconsistentCounts);
        }
        Ok(Self {
            selected,
            passed,
            failed,
            skipped,
            filtered,
            quarantined,
        })
    }
}

/// One immutable numbered execution attempt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceAttempt {
    number: PositiveCount,
    outcome: AttemptOutcome,
    started_at: UtcInstant,
    finished_at: UtcInstant,
    message: Option<SelectorText>,
}

impl EvidenceAttempt {
    /// Creates an attempt with an ordered time window.
    pub fn new(
        number: PositiveCount,
        outcome: AttemptOutcome,
        started_at: UtcInstant,
        finished_at: UtcInstant,
        message: Option<SelectorText>,
    ) -> Result<Self, EvidenceResultBuildError> {
        if finished_at < started_at {
            return Err(EvidenceResultBuildError::InvalidTimeWindow);
        }
        Ok(Self {
            number,
            outcome,
            started_at,
            finished_at,
            message,
        })
    }
    /// Returns the consecutive attempt number.
    #[must_use]
    pub const fn number(&self) -> PositiveCount {
        self.number
    }
    /// Returns the terminal outcome.
    #[must_use]
    pub const fn outcome(&self) -> AttemptOutcome {
        self.outcome
    }
    /// Returns attempt start.
    #[must_use]
    pub const fn started_at(&self) -> UtcInstant {
        self.started_at
    }
    /// Returns attempt finish.
    #[must_use]
    pub const fn finished_at(&self) -> UtcInstant {
        self.finished_at
    }
    /// Returns optional bounded message.
    #[must_use]
    pub const fn message(&self) -> Option<&SelectorText> {
        self.message.as_ref()
    }
}

/// Aggregate attempt interpretation before freshness and trust.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AttemptAggregate {
    /// Every attempt passed and count met minimum.
    Satisfied,
    /// A terminal failure occurred without a pass.
    Failed,
    /// Required selected or passing items were absent.
    Missing,
    /// Both passing and failing attempts occurred.
    Unstable,
    /// Timeout, cancellation, or execution error occurred.
    Unknown,
}

/// A validated executable evidence payload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionPayload {
    attempts: Vec<EvidenceAttempt>,
    counts: EvidenceCounts,
    started_at: UtcInstant,
    finished_at: UtcInstant,
}

impl ExecutionPayload {
    /// Creates an execution payload with complete immutable retry history.
    pub fn new(
        attempts: Vec<EvidenceAttempt>,
        counts: EvidenceCounts,
        started_at: UtcInstant,
        finished_at: UtcInstant,
    ) -> Result<Self, EvidenceResultBuildError> {
        if attempts.is_empty() || finished_at < started_at {
            return Err(EvidenceResultBuildError::InvalidTimeWindow);
        }
        for (index, attempt) in attempts.iter().enumerate() {
            if attempt.number().get() != index as u64 + 1
                || attempt.started_at() < started_at
                || attempt.finished_at() > finished_at
            {
                return Err(EvidenceResultBuildError::InvalidAttemptSequence);
            }
        }
        Ok(Self {
            attempts,
            counts,
            started_at,
            finished_at,
        })
    }

    /// Aggregates retries and counts without erasing failure history.
    #[must_use]
    pub fn aggregate(&self, minimum: PositiveCount) -> AttemptAggregate {
        let outcomes: BTreeSet<_> = self.attempts.iter().map(EvidenceAttempt::outcome).collect();
        if outcomes.iter().any(|outcome| {
            matches!(
                outcome,
                AttemptOutcome::TimedOut | AttemptOutcome::Cancelled | AttemptOutcome::Error
            )
        }) {
            AttemptAggregate::Unknown
        } else if outcomes.contains(&AttemptOutcome::Passed)
            && outcomes.contains(&AttemptOutcome::Failed)
        {
            AttemptAggregate::Unstable
        } else if outcomes.contains(&AttemptOutcome::Failed) {
            AttemptAggregate::Failed
        } else if self.counts.passed < minimum.get()
            || self.counts.skipped > 0
            || self.counts.filtered > 0
            || self.counts.quarantined > 0
        {
            AttemptAggregate::Missing
        } else {
            AttemptAggregate::Satisfied
        }
    }

    /// Returns immutable attempts in number order.
    #[must_use]
    pub fn attempts(&self) -> &[EvidenceAttempt] {
        &self.attempts
    }
    /// Returns normalized counts.
    #[must_use]
    pub const fn counts(&self) -> EvidenceCounts {
        self.counts
    }
    /// Returns overall start.
    #[must_use]
    pub const fn started_at(&self) -> UtcInstant {
        self.started_at
    }
    /// Returns overall finish.
    #[must_use]
    pub const fn finished_at(&self) -> UtcInstant {
        self.finished_at
    }
}

/// Closed kind-discriminated result payload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidencePayload {
    /// Structural-check execution.
    StructuralCheck(ExecutionPayload),
    /// Static inventory and selected counts.
    StaticInventory {
        /// Exact immutable inventory digest.
        inventory_digest: Sha256Digest,
        /// Normalized selected-record counts.
        counts: EvidenceCounts,
    },
    /// Test execution.
    Test(ExecutionPayload),
    /// Snapshot execution.
    Snapshot(ExecutionPayload),
    /// Human review, never a waiver.
    ManualReview {
        /// Passed or failed review outcome.
        outcome: AttemptOutcome,
        /// Exact accountable reviewer.
        reviewer: OwnerRef,
        /// Optional bounded review message.
        message: Option<SelectorText>,
    },
    /// Runtime snapshot reference and selected counts.
    RuntimeSnapshot {
        /// Exact immutable runtime-facts digest.
        runtime_facts_digest: Sha256Digest,
        /// Normalized selected-fact counts.
        counts: EvidenceCounts,
    },
    /// Release record reference.
    ReleaseRecord {
        /// Exact immutable release-record digest.
        release_record_digest: Sha256Digest,
    },
}

impl EvidencePayload {
    fn kind(&self) -> EvidenceKind {
        match self {
            Self::StructuralCheck(_) => EvidenceKind::StructuralCheck,
            Self::StaticInventory { .. } => EvidenceKind::StaticInventory,
            Self::Test(_) => EvidenceKind::Test,
            Self::Snapshot(_) => EvidenceKind::Snapshot,
            Self::ManualReview { .. } => EvidenceKind::ManualReview,
            Self::RuntimeSnapshot { .. } => EvidenceKind::RuntimeSnapshot,
            Self::ReleaseRecord { .. } => EvidenceKind::ReleaseRecord,
        }
    }

    /// Creates a manual review limited to passed or failed.
    pub fn manual_review(
        outcome: AttemptOutcome,
        reviewer: OwnerRef,
        message: Option<SelectorText>,
    ) -> Result<Self, EvidenceResultBuildError> {
        if !matches!(outcome, AttemptOutcome::Passed | AttemptOutcome::Failed) {
            return Err(EvidenceResultBuildError::InvalidManualOutcome);
        }
        Ok(Self::ManualReview {
            outcome,
            reviewer,
            message,
        })
    }
}

/// One content-addressed attachment descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceAttachment {
    name: SelectorText,
    media_type: SelectorText,
    digest: Sha256Digest,
    size: u64,
}

impl EvidenceAttachment {
    /// Creates an external content-addressed attachment descriptor.
    #[must_use]
    pub const fn new(
        name: SelectorText,
        media_type: SelectorText,
        digest: Sha256Digest,
        size: u64,
    ) -> Self {
        Self {
            name,
            media_type,
            digest,
            size,
        }
    }
    /// Returns the attachment name.
    #[must_use]
    pub const fn name(&self) -> &SelectorText {
        &self.name
    }
    /// Returns media type.
    #[must_use]
    pub const fn media_type(&self) -> &SelectorText {
        &self.media_type
    }
    /// Returns content digest.
    #[must_use]
    pub const fn digest(&self) -> Sha256Digest {
        self.digest
    }
    /// Returns external byte size.
    #[must_use]
    pub const fn size(&self) -> u64 {
        self.size
    }
}

/// An immutable evidence result envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceResult {
    id: Sha256Digest,
    subject: EvidenceSubject,
    target: TargetId,
    unit: UnitId,
    requirements: BTreeSet<FullRequirementId>,
    facets: BTreeSet<Facet>,
    kind: EvidenceKind,
    evidence_spec_digest: Sha256Digest,
    contract_digest: Sha256Digest,
    binding_digest: Sha256Digest,
    policy_digest: Sha256Digest,
    runner_digest: Option<Sha256Digest>,
    adapter_digest: Option<Sha256Digest>,
    runtime_facts_digest: Option<Sha256Digest>,
    release_record_digest: Option<Sha256Digest>,
    profile_values: BTreeMap<ProfileId, ProfileSelection>,
    producer: ProducerRef,
    claimed_trust: TrustLevel,
    observed_at: UtcInstant,
    payload: EvidencePayload,
    attachments: BTreeMap<SelectorText, EvidenceAttachment>,
    result_digest: Sha256Digest,
    extensions: Extensions,
}

impl EvidenceResult {
    /// Creates an immutable envelope and enforces all local identity invariants.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Sha256Digest,
        subject: EvidenceSubject,
        target: TargetId,
        unit: UnitId,
        requirements: Vec<FullRequirementId>,
        facets: Vec<Facet>,
        kind: EvidenceKind,
        evidence_spec_digest: Sha256Digest,
        contract_digest: Sha256Digest,
        binding_digest: Sha256Digest,
        policy_digest: Sha256Digest,
        runner_digest: Option<Sha256Digest>,
        adapter_digest: Option<Sha256Digest>,
        runtime_facts_digest: Option<Sha256Digest>,
        release_record_digest: Option<Sha256Digest>,
        profile_values: Vec<ProfileSelection>,
        producer: ProducerRef,
        claimed_trust: TrustLevel,
        observed_at: UtcInstant,
        payload: EvidencePayload,
        attachments: Vec<EvidenceAttachment>,
        result_digest: Sha256Digest,
        extensions: Extensions,
    ) -> Result<Self, EvidenceResultBuildError> {
        if id != result_digest {
            return Err(EvidenceResultBuildError::ResultIdentityMismatch);
        }
        if payload.kind() != kind {
            return Err(EvidenceResultBuildError::PayloadKindMismatch);
        }
        let requirements = unique_nonempty(
            requirements,
            EvidenceResultBuildError::RequirementsRequired,
            EvidenceResultBuildError::DuplicateRequirement,
        )?;
        let facets = unique_nonempty(
            facets,
            EvidenceResultBuildError::FacetsRequired,
            EvidenceResultBuildError::DuplicateFacet,
        )?;
        let profile_count = profile_values.len();
        let profile_values: BTreeMap<_, _> = profile_values
            .into_iter()
            .map(|selection| (selection.profile().clone(), selection))
            .collect();
        if profile_values.len() != profile_count {
            return Err(EvidenceResultBuildError::DuplicateProfile);
        }
        let attachment_count = attachments.len();
        let attachments: BTreeMap<_, _> = attachments
            .into_iter()
            .map(|attachment| (attachment.name().clone(), attachment))
            .collect();
        if attachments.len() != attachment_count {
            return Err(EvidenceResultBuildError::DuplicateAttachment);
        }
        Ok(Self {
            id,
            subject,
            target,
            unit,
            requirements,
            facets,
            kind,
            evidence_spec_digest,
            contract_digest,
            binding_digest,
            policy_digest,
            runner_digest,
            adapter_digest,
            runtime_facts_digest,
            release_record_digest,
            profile_values,
            producer,
            claimed_trust,
            observed_at,
            payload,
            attachments,
            result_digest,
            extensions,
        })
    }

    /// Returns the digest-derived result ID.
    #[must_use]
    pub const fn id(&self) -> Sha256Digest {
        self.id
    }
    /// Returns the replay-bound subject.
    #[must_use]
    pub const fn subject(&self) -> &EvidenceSubject {
        &self.subject
    }
    /// Returns the target.
    #[must_use]
    pub const fn target(&self) -> &TargetId {
        &self.target
    }
    /// Returns the unit.
    #[must_use]
    pub const fn unit(&self) -> &UnitId {
        &self.unit
    }
    /// Returns covered requirements.
    #[must_use]
    pub const fn requirements(&self) -> &BTreeSet<FullRequirementId> {
        &self.requirements
    }
    /// Returns covered facets.
    #[must_use]
    pub const fn facets(&self) -> &BTreeSet<Facet> {
        &self.facets
    }
    /// Returns the kind.
    #[must_use]
    pub const fn kind(&self) -> EvidenceKind {
        self.kind
    }
    /// Returns the evidence-specification digest.
    #[must_use]
    pub const fn evidence_spec_digest(&self) -> Sha256Digest {
        self.evidence_spec_digest
    }
    /// Returns contract digest.
    #[must_use]
    pub const fn contract_digest(&self) -> Sha256Digest {
        self.contract_digest
    }
    /// Returns binding digest.
    #[must_use]
    pub const fn binding_digest(&self) -> Sha256Digest {
        self.binding_digest
    }
    /// Returns policy digest.
    #[must_use]
    pub const fn policy_digest(&self) -> Sha256Digest {
        self.policy_digest
    }
    /// Returns runner digest.
    #[must_use]
    pub const fn runner_digest(&self) -> Option<Sha256Digest> {
        self.runner_digest
    }
    /// Returns adapter digest.
    #[must_use]
    pub const fn adapter_digest(&self) -> Option<Sha256Digest> {
        self.adapter_digest
    }
    /// Returns runtime-facts digest.
    #[must_use]
    pub const fn runtime_facts_digest(&self) -> Option<Sha256Digest> {
        self.runtime_facts_digest
    }
    /// Returns release-record digest.
    #[must_use]
    pub const fn release_record_digest(&self) -> Option<Sha256Digest> {
        self.release_record_digest
    }
    /// Returns exact profile selections.
    #[must_use]
    pub const fn profile_values(&self) -> &BTreeMap<ProfileId, ProfileSelection> {
        &self.profile_values
    }
    /// Returns producer identity.
    #[must_use]
    pub const fn producer(&self) -> &ProducerRef {
        &self.producer
    }
    /// Returns claimed trust.
    #[must_use]
    pub const fn claimed_trust(&self) -> TrustLevel {
        self.claimed_trust
    }
    /// Returns observation time.
    #[must_use]
    pub const fn observed_at(&self) -> UtcInstant {
        self.observed_at
    }
    /// Returns kind-discriminated payload.
    #[must_use]
    pub const fn payload(&self) -> &EvidencePayload {
        &self.payload
    }
    /// Returns attachments by name.
    #[must_use]
    pub const fn attachments(&self) -> &BTreeMap<SelectorText, EvidenceAttachment> {
        &self.attachments
    }
    /// Returns the verified result digest.
    #[must_use]
    pub const fn result_digest(&self) -> Sha256Digest {
        self.result_digest
    }
    /// Returns extensions.
    #[must_use]
    pub const fn extensions(&self) -> &Extensions {
        &self.extensions
    }
}

fn unique_nonempty<T: Ord>(
    values: Vec<T>,
    empty: EvidenceResultBuildError,
    duplicate: EvidenceResultBuildError,
) -> Result<BTreeSet<T>, EvidenceResultBuildError> {
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

/// Evidence-result construction failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceResultBuildError {
    /// Repository URI was noncanonical or unsafe.
    InvalidRepositoryIdentity,
    /// Source commit was not a full lowercase object ID.
    InvalidSourceCommit,
    /// Producer URI was malformed.
    InvalidProducer,
    /// Target set was empty.
    TargetsRequired,
    /// Target set contained a duplicate.
    DuplicateTarget,
    /// Profile selection was empty.
    ProfileValuesRequired,
    /// Profile selection repeated a dimension.
    DuplicateDimension,
    /// Count totals were zero, inconsistent, or overflowed.
    InconsistentCounts,
    /// Time window was reversed or attempts were empty.
    InvalidTimeWindow,
    /// Attempt numbers or bounds were inconsistent.
    InvalidAttemptSequence,
    /// Manual review used an unsupported outcome.
    InvalidManualOutcome,
    /// Result ID did not equal its result digest.
    ResultIdentityMismatch,
    /// Payload kind did not equal result kind.
    PayloadKindMismatch,
    /// Coverage omitted requirements.
    RequirementsRequired,
    /// Coverage repeated a requirement.
    DuplicateRequirement,
    /// Coverage omitted facets.
    FacetsRequired,
    /// Coverage repeated a facet.
    DuplicateFacet,
    /// Profile list repeated an ID.
    DuplicateProfile,
    /// Attachment list repeated a name.
    DuplicateAttachment,
}

impl Display for EvidenceResultBuildError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

impl Error for EvidenceResultBuildError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identities_and_counts_fail_closed() -> Result<(), Box<dyn Error>> {
        let repository: RepositoryIdentity = "https://github.com/example/project".parse()?;
        assert_eq!(repository.as_str(), "https://github.com/example/project");
        assert!(
            "ssh://github.com/example/project"
                .parse::<RepositoryIdentity>()
                .is_err()
        );
        let commit: SourceCommit = "a".repeat(40).parse()?;
        assert_eq!(commit.as_str().len(), 40);
        assert!("A".repeat(40).parse::<SourceCommit>().is_err());
        assert!("producer://ci/github/run-1".parse::<ProducerRef>().is_ok());
        assert!(
            "producer://unknown/github/run-1"
                .parse::<ProducerRef>()
                .is_err()
        );
        assert_eq!(
            EvidenceCounts::new(2, 1, 0, 0, 0, 0),
            Err(EvidenceResultBuildError::InconsistentCounts)
        );
        Ok(())
    }

    #[test]
    fn retries_preserve_unstable_and_unknown_history() -> Result<(), Box<dyn Error>> {
        let start: UtcInstant = "2026-08-07T12:00:00Z".parse()?;
        let middle: UtcInstant = "2026-08-07T12:00:01Z".parse()?;
        let end: UtcInstant = "2026-08-07T12:00:02Z".parse()?;
        let failed = EvidenceAttempt::new(
            PositiveCount::new(1)?,
            AttemptOutcome::Failed,
            start,
            middle,
            None,
        )?;
        let passed = EvidenceAttempt::new(
            PositiveCount::new(2)?,
            AttemptOutcome::Passed,
            middle,
            end,
            None,
        )?;
        let payload = ExecutionPayload::new(
            vec![failed, passed],
            EvidenceCounts::new(1, 1, 0, 0, 0, 0)?,
            start,
            end,
        )?;
        assert_eq!(
            payload.aggregate(PositiveCount::ONE),
            AttemptAggregate::Unstable
        );
        let timeout = EvidenceAttempt::new(
            PositiveCount::new(1)?,
            AttemptOutcome::TimedOut,
            start,
            end,
            None,
        )?;
        let timeout_payload = ExecutionPayload::new(
            vec![timeout],
            EvidenceCounts::new(1, 0, 1, 0, 0, 0)?,
            start,
            end,
        )?;
        assert_eq!(
            timeout_payload.aggregate(PositiveCount::ONE),
            AttemptAggregate::Unknown
        );
        Ok(())
    }

    #[test]
    fn skips_filters_and_low_pass_counts_are_missing() -> Result<(), Box<dyn Error>> {
        let start: UtcInstant = "2026-08-07T12:00:00Z".parse()?;
        let end: UtcInstant = "2026-08-07T12:00:01Z".parse()?;
        let passed =
            EvidenceAttempt::new(PositiveCount::ONE, AttemptOutcome::Passed, start, end, None)?;
        for counts in [
            EvidenceCounts::new(2, 1, 0, 1, 0, 0)?,
            EvidenceCounts::new(2, 1, 0, 0, 1, 0)?,
            EvidenceCounts::new(1, 1, 0, 0, 0, 0)?,
        ] {
            let payload = ExecutionPayload::new(vec![passed.clone()], counts, start, end)?;
            assert_eq!(
                payload.aggregate(PositiveCount::new(2)?),
                AttemptAggregate::Missing
            );
        }
        Ok(())
    }

    #[test]
    fn result_enforces_digest_identity_kind_and_unique_coordinates() -> Result<(), Box<dyn Error>> {
        let digest = Sha256Digest::hash_content(b"result");
        let other = Sha256Digest::hash_content(b"other");
        let subject = EvidenceSubject::new(
            "https://github.com/example/project".parse()?,
            Sha256Digest::hash_content(b"repository"),
            EvidenceScopeSubject::Target(TargetId::new("web")?),
            "a".repeat(40).parse()?,
            None,
            None,
            Sha256Digest::hash_content(b"target-config"),
        );
        let payload = EvidencePayload::manual_review(
            AttemptOutcome::Passed,
            "owner://team/reviewers".parse()?,
            Some(SelectorText::new("Approved")?),
        )?;
        let profile = ProfileSelection::new(
            ProfileId::new("audience.default")?,
            Revision::new(1).map_err(|_| EvidenceResultBuildError::ProfileValuesRequired)?,
            vec![(DimensionId::new("region")?, SymbolicValueId::new("eu")?)],
        )?;
        let build = |id, result_digest, kind, requirements| {
            EvidenceResult::new(
                id,
                subject.clone(),
                TargetId::new("web").map_err(|_| EvidenceResultBuildError::TargetsRequired)?,
                UnitId::new("account.create.signup.start")
                    .map_err(|_| EvidenceResultBuildError::RequirementsRequired)?,
                requirements,
                vec![Facet::Behavior],
                kind,
                Sha256Digest::hash_content(b"spec"),
                Sha256Digest::hash_content(b"contract"),
                Sha256Digest::hash_content(b"binding"),
                Sha256Digest::hash_content(b"policy"),
                None,
                None,
                None,
                None,
                vec![profile.clone()],
                "producer://human/review/reviewer-1".parse()?,
                TrustLevel::TrustedCi,
                "2026-08-07T12:00:00Z"
                    .parse()
                    .map_err(|_| EvidenceResultBuildError::InvalidTimeWindow)?,
                payload.clone(),
                Vec::new(),
                result_digest,
                Extensions::default(),
            )
        };
        let requirement = FullRequirementId::new("account.create.signup.start#reachable")?;
        let result = build(
            digest,
            digest,
            EvidenceKind::ManualReview,
            vec![requirement.clone()],
        )?;
        assert_eq!(result.id(), digest);
        assert_eq!(result.result_digest(), digest);
        assert!(matches!(
            build(
                other,
                digest,
                EvidenceKind::ManualReview,
                vec![requirement.clone()]
            ),
            Err(EvidenceResultBuildError::ResultIdentityMismatch)
        ));
        assert!(matches!(
            build(
                digest,
                digest,
                EvidenceKind::Test,
                vec![requirement.clone()]
            ),
            Err(EvidenceResultBuildError::PayloadKindMismatch)
        ));
        assert!(matches!(
            build(
                digest,
                digest,
                EvidenceKind::ManualReview,
                vec![requirement.clone(), requirement]
            ),
            Err(EvidenceResultBuildError::DuplicateRequirement)
        ));
        Ok(())
    }
}
