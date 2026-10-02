//! Checked protocol-to-domain evidence admission.
//!
//! DEV-009 S025 freezes one checked conversion from an already-parsed
//! [`CheckedEvidenceResultV2`] plus explicit typed lineage, a declared execution
//! and an independently supplied [`EvidenceAdmissionContext`] into an
//! [`AdmittedEvidenceV2`]. The conversion cross-checks identity, subject, scope,
//! target, unit, requirement, facet, digest, profile, provider-association and
//! payload-reference axes, validates lineage relationships and count/outcome
//! consistency, and only then constructs the private-field result.
//!
//! This never verifies producer trust, storage or signatures, never routes a v1
//! record through an implicit conversion and never invents a missing reference.
//! The structural v2 reader (`CheckedEvidenceResultV2::from_bytes`) remains a
//! distinct, non-admitting API.

use crate::CheckedEvidenceResultV2;
use crate::v2::{
    AttachmentV2Dto, AttemptV2Dto, CountsV2Dto, EvidencePayloadV2Dto, EvidenceSubjectV2Dto,
    ExecutionPayloadV2Dto, ProfileValueV2Dto, ScopeSubjectV2Dto,
};
use eqm_domain::{
    AdapterDigest, AttemptOutcome, BindingDigest, ContractDigest, DimensionId, EvidenceAttachment,
    EvidenceAttempt, EvidenceCounts, EvidenceExecution, EvidenceKind, EvidenceLineage,
    EvidencePayload, EvidenceResult, EvidenceResultBuildError, EvidenceScopeSubject,
    EvidenceSpecificationDigest, EvidenceSubject, ExecutionPayload, ExpectedLineageAuthority,
    Facet, FullRequirementId, OwnerRef, PolicySelectionDigest, PositiveCount, ProfileId,
    ProfileSelection, ProviderId, RepositoryIdentity, Revision, RunnerDigest, SelectorText,
    SemanticEvidenceDigest, Sha256Digest, SourceCommit, SymbolicValueId, TargetConfigurationDigest,
    TargetId, TrustLevel, UnitId, UtcInstant,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Independently supplied approved context for admitting one v2 evidence record.
///
/// Every axis is explicit input. The record's references must match this
/// context exactly; nothing is defaulted, zero-filled or derived from a
/// workspace digest. Approved component identities use their existing typed
/// digest newtypes, not interchangeable generic SHA values.
///
/// The typed fields cannot be substituted with another identity domain:
///
/// ```compile_fail
/// use eqm_domain::BindingDigest;
/// use eqm_protocol::EvidenceAdmissionContext;
/// fn replace(context: &mut EvidenceAdmissionContext) {
///     // `contract_digest` is a `ContractDigest`, not a `BindingDigest`.
///     context.contract_digest = BindingDigest::hash_content(b"binding");
/// }
/// ```
///
/// ```compile_fail
/// use eqm_domain::RunnerDigest;
/// use eqm_protocol::EvidenceAdmissionContext;
/// fn replace(context: &mut EvidenceAdmissionContext) {
///     // `adapter_digest` is `Option<AdapterDigest>`, not `Option<RunnerDigest>`.
///     context.adapter_digest = Some(RunnerDigest::hash_content(b"runner"));
/// }
/// ```
///
/// The positive counterpart assigns the matching typed domain, and each
/// identity's exact SHA is its wire encoding:
///
/// ```
/// use eqm_domain::{ContractDigest, Sha256Digest};
/// use eqm_protocol::EvidenceAdmissionContext;
/// fn replace(context: &mut EvidenceAdmissionContext, value: ContractDigest) {
///     context.contract_digest = value;
/// }
/// let contract = ContractDigest::hash_content(b"contract");
/// let plain: Sha256Digest = contract.as_digest();
/// assert_eq!(plain.to_string(), contract.to_string());
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceAdmissionContext {
    /// Expected replay-bound subject (repository/commit/build/artifact/configuration/scope).
    pub subject: EvidenceSubject,
    /// Expected execution target.
    pub target: TargetId,
    /// Expected owning unit.
    pub unit: UnitId,
    /// Expected requirement covered by this evidence.
    pub requirement: FullRequirementId,
    /// Expected complete facet set.
    pub facets: BTreeSet<Facet>,
    /// Expected envelope kind.
    pub kind: EvidenceKind,
    /// Expected evidence-specification identity.
    pub evidence_spec_digest: EvidenceSpecificationDigest,
    /// Expected contract identity.
    pub contract_digest: ContractDigest,
    /// Expected binding identity.
    pub binding_digest: BindingDigest,
    /// Expected selected-policy identity.
    pub policy_digest: PolicySelectionDigest,
    /// Expected target-configuration identity, whose exact SHA must equal the
    /// subject configuration field.
    pub target_configuration_digest: TargetConfigurationDigest,
    /// Expected present or absent runner identity.
    pub runner_digest: Option<RunnerDigest>,
    /// Expected present or absent adapter identity.
    pub adapter_digest: Option<AdapterDigest>,
    /// Expected present or absent runtime-facts reference (exact structural SHA;
    /// no distinct wrapper exists).
    pub runtime_facts_digest: Option<Sha256Digest>,
    /// Expected present or absent release-record reference (exact structural SHA;
    /// no distinct wrapper exists).
    pub release_record_digest: Option<Sha256Digest>,
    /// Expected exact selected profiles and their revisions/values.
    pub profiles: BTreeMap<ProfileId, ProfileSelection>,
    /// Explicitly declared provider-to-target association for provider scopes.
    pub provider_targets: BTreeMap<ProviderId, BTreeSet<TargetId>>,
    /// Independently supplied expected plan/campaign authority.
    pub expected_lineage: ExpectedLineageAuthority,
}

/// A checked, admitted v2 evidence record bound to its explicit lineage.
///
/// Fields are private; only [`admit_evidence`] constructs one, so an arbitrary
/// parsed record cannot be promoted to an admitted result without the full
/// cross-checks:
///
/// ```compile_fail
/// use eqm_protocol::AdmittedEvidenceV2;
/// fn forge() -> AdmittedEvidenceV2 {
///     // `result` is a private field, so unchecked construction fails.
///     AdmittedEvidenceV2 { result: todo!() }
/// }
/// ```
///
/// The accessors return their own typed option, not a bare identity:
///
/// ```compile_fail
/// use eqm_domain::AdapterDigest;
/// use eqm_protocol::AdmittedEvidenceV2;
/// fn takes(_: AdapterDigest) {}
/// fn read(value: &AdmittedEvidenceV2) {
///     // `adapter_digest()` returns `Option<AdapterDigest>`.
///     takes(value.adapter_digest());
/// }
/// ```
///
/// The positive accessor form unwraps the typed option:
///
/// ```
/// use eqm_domain::AdapterDigest;
/// use eqm_protocol::AdmittedEvidenceV2;
/// fn takes(_: Option<AdapterDigest>) {}
/// fn read(value: &AdmittedEvidenceV2) {
///     takes(value.adapter_digest());
/// }
/// ```
///
/// Holding one confers no producer trust.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedEvidenceV2 {
    result: EvidenceResult,
    lineage: EvidenceLineage,
    execution: EvidenceExecution,
    semantic_identity: SemanticEvidenceDigest,
    raw_bytes: Vec<u8>,
    evidence_spec_digest: EvidenceSpecificationDigest,
    contract_digest: ContractDigest,
    binding_digest: BindingDigest,
    policy_digest: PolicySelectionDigest,
    target_configuration_digest: TargetConfigurationDigest,
    runner_digest: Option<RunnerDigest>,
    adapter_digest: Option<AdapterDigest>,
}

impl AdmittedEvidenceV2 {
    /// Returns the admitted domain result envelope.
    #[must_use]
    pub const fn result(&self) -> &EvidenceResult {
        &self.result
    }

    /// Returns the validated execution lineage.
    #[must_use]
    pub const fn lineage(&self) -> &EvidenceLineage {
        &self.lineage
    }

    /// Returns the declared execution.
    #[must_use]
    pub const fn execution(&self) -> &EvidenceExecution {
        &self.execution
    }

    /// Returns the recomputed semantic evidence identity.
    #[must_use]
    pub const fn semantic_identity(&self) -> SemanticEvidenceDigest {
        self.semantic_identity
    }

    /// Returns the exact original bytes.
    #[must_use]
    pub fn raw_bytes(&self) -> &[u8] {
        &self.raw_bytes
    }

    /// Returns the retained typed evidence-specification expectation.
    #[must_use]
    pub const fn evidence_spec_digest(&self) -> EvidenceSpecificationDigest {
        self.evidence_spec_digest
    }

    /// Returns the retained typed contract expectation.
    #[must_use]
    pub const fn contract_digest(&self) -> ContractDigest {
        self.contract_digest
    }

    /// Returns the retained typed binding expectation.
    #[must_use]
    pub const fn binding_digest(&self) -> BindingDigest {
        self.binding_digest
    }

    /// Returns the retained typed selected-policy expectation.
    #[must_use]
    pub const fn policy_digest(&self) -> PolicySelectionDigest {
        self.policy_digest
    }

    /// Returns the retained typed target-configuration expectation.
    #[must_use]
    pub const fn target_configuration_digest(&self) -> TargetConfigurationDigest {
        self.target_configuration_digest
    }

    /// Returns the retained typed runner expectation, if any.
    #[must_use]
    pub const fn runner_digest(&self) -> Option<RunnerDigest> {
        self.runner_digest
    }

    /// Returns the retained typed adapter expectation, if any.
    #[must_use]
    pub const fn adapter_digest(&self) -> Option<AdapterDigest> {
        self.adapter_digest
    }
}

/// Evidence-admission failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceAdmissionError {
    /// `id`, `result_digest` and the recomputed semantic identity disagreed.
    IdentityMismatch,
    /// The exact schema marker was not the v2 evidence family.
    SchemaMismatch,
    /// The envelope kind did not match the expected kind.
    KindMismatch,
    /// The subject repository/commit/build/artifact/configuration differed.
    SubjectMismatch,
    /// The scope shape or value differed from the expected scope.
    ScopeMismatch,
    /// The target differed from the expected target or scope membership.
    TargetMismatch,
    /// The unit differed from the expected unit.
    UnitMismatch,
    /// A requirement did not belong to the unit or was not the expected one.
    RequirementMismatch,
    /// The facet set differed from the expected set.
    FacetMismatch,
    /// A digest reference differed from the expected context.
    DigestMismatch,
    /// The selected profiles differed from the expected selection.
    ProfileMismatch,
    /// A provider scope had no declared association to the target.
    ProviderUnassociated,
    /// The payload runtime reference did not match the envelope.
    RuntimeReferenceMismatch,
    /// The payload release reference did not match the envelope.
    ReleaseReferenceMismatch,
    /// Asserted counts contradicted the recorded attempt outcomes.
    CountOutcomeContradiction,
    /// Numbered retry windows were not chronological (a retry began before the
    /// preceding retry finished).
    RetryChronology,
    /// A lineage relationship did not match its declared execution or authority.
    Lineage(EvidenceResultBuildError),
    /// A typed domain value could not be built from the parsed record.
    Domain(EvidenceResultBuildError),
    /// The parsed record could not be converted into its typed domain value.
    Malformed(&'static str),
}

impl Display for EvidenceAdmissionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lineage(error) | Self::Domain(error) => write!(formatter, "{self:?}: {error}"),
            Self::Malformed(reason) => write!(formatter, "malformed evidence: {reason}"),
            _ => write!(formatter, "{self:?}"),
        }
    }
}

impl Error for EvidenceAdmissionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Lineage(error) | Self::Domain(error) => Some(error),
            _ => None,
        }
    }
}

fn malformed(reason: &'static str) -> EvidenceAdmissionError {
    EvidenceAdmissionError::Malformed(reason)
}

fn digest(value: &str) -> Result<Sha256Digest, EvidenceAdmissionError> {
    value.parse().map_err(|_| malformed("digest"))
}

fn instant(value: crate::CanonicalInstant) -> Result<UtcInstant, EvidenceAdmissionError> {
    value
        .get()
        .to_string()
        .parse()
        .map_err(|_| malformed("instant"))
}

fn selector(value: &str) -> Result<SelectorText, EvidenceAdmissionError> {
    SelectorText::new(value).map_err(|_| malformed("selector"))
}

fn counts(dto: &CountsV2Dto) -> Result<EvidenceCounts, EvidenceAdmissionError> {
    EvidenceCounts::new(
        dto.selected.get(),
        dto.passed.get(),
        dto.failed.get(),
        dto.skipped.get(),
        dto.filtered.get(),
        dto.quarantined.get(),
    )
    .map_err(EvidenceAdmissionError::Domain)
}

fn attempt(dto: &AttemptV2Dto) -> Result<EvidenceAttempt, EvidenceAdmissionError> {
    let number = PositiveCount::new(dto.number.get()).map_err(|_| malformed("positive count"))?;
    let outcome = dto
        .outcome
        .parse::<AttemptOutcome>()
        .map_err(|_| malformed("attempt outcome"))?;
    let message = dto.message.as_deref().map(selector).transpose()?;
    EvidenceAttempt::new(
        number,
        outcome,
        instant(dto.started_at)?,
        instant(dto.finished_at)?,
        message,
    )
    .map_err(EvidenceAdmissionError::Domain)
}

fn execution(dto: &ExecutionPayloadV2Dto) -> Result<ExecutionPayload, EvidenceAdmissionError> {
    let attempts = dto
        .attempts
        .iter()
        .map(attempt)
        .collect::<Result<Vec<_>, _>>()?;
    ExecutionPayload::new(
        attempts,
        counts(&dto.counts)?,
        instant(dto.started_at)?,
        instant(dto.finished_at)?,
    )
    .map_err(EvidenceAdmissionError::Domain)
}

fn payload(dto: &EvidencePayloadV2Dto) -> Result<EvidencePayload, EvidenceAdmissionError> {
    Ok(match dto {
        EvidencePayloadV2Dto::StructuralCheck { execution: value } => {
            EvidencePayload::StructuralCheck(execution(value)?)
        }
        EvidencePayloadV2Dto::Test { execution: value } => EvidencePayload::Test(execution(value)?),
        EvidencePayloadV2Dto::Snapshot { execution: value } => {
            EvidencePayload::Snapshot(execution(value)?)
        }
        EvidencePayloadV2Dto::StaticInventory {
            inventory_digest,
            counts: value,
        } => EvidencePayload::StaticInventory {
            inventory_digest: digest(inventory_digest)?,
            counts: counts(value)?,
        },
        EvidencePayloadV2Dto::RuntimeSnapshot {
            runtime_facts_digest,
            counts: value,
        } => EvidencePayload::RuntimeSnapshot {
            runtime_facts_digest: digest(runtime_facts_digest)?,
            counts: counts(value)?,
        },
        EvidencePayloadV2Dto::ReleaseRecord {
            release_record_digest,
        } => EvidencePayload::ReleaseRecord {
            release_record_digest: digest(release_record_digest)?,
        },
        EvidencePayloadV2Dto::ManualReview {
            outcome,
            reviewer,
            message,
        } => EvidencePayload::manual_review(
            outcome
                .parse::<AttemptOutcome>()
                .map_err(|_| malformed("manual outcome"))?,
            reviewer
                .parse::<OwnerRef>()
                .map_err(|_| malformed("reviewer"))?,
            message.as_deref().map(selector).transpose()?,
        )
        .map_err(EvidenceAdmissionError::Domain)?,
    })
}

fn scope(dto: &ScopeSubjectV2Dto) -> Result<EvidenceScopeSubject, EvidenceAdmissionError> {
    match dto {
        ScopeSubjectV2Dto::Target { target } => Ok(EvidenceScopeSubject::Target(
            TargetId::new(target.as_str()).map_err(|_| malformed("identifier"))?,
        )),
        ScopeSubjectV2Dto::Provider { provider } => Ok(EvidenceScopeSubject::Provider(
            ProviderId::new(provider.as_str()).map_err(|_| malformed("identifier"))?,
        )),
        ScopeSubjectV2Dto::TargetSet { targets } => {
            let targets = targets
                .iter()
                .map(|target| TargetId::new(target.as_str()).map_err(|_| malformed("identifier")))
                .collect::<Result<Vec<_>, _>>()?;
            EvidenceScopeSubject::target_set(targets).map_err(EvidenceAdmissionError::Domain)
        }
    }
}

fn subject(dto: &EvidenceSubjectV2Dto) -> Result<EvidenceSubject, EvidenceAdmissionError> {
    let repository: RepositoryIdentity = dto
        .repository
        .parse()
        .map_err(|_| malformed("repository"))?;
    let source_commit: SourceCommit = dto
        .source_commit
        .parse()
        .map_err(|_| malformed("source commit"))?;
    let build_id = dto.build_id.as_deref().map(selector).transpose()?;
    let artifact_digest = dto.artifact_digest.as_deref().map(digest).transpose()?;
    Ok(EvidenceSubject::new(
        repository,
        digest(&dto.repository_id_digest)?,
        scope(&dto.scope)?,
        source_commit,
        build_id,
        artifact_digest,
        digest(&dto.target_configuration_digest)?,
    ))
}

fn profiles(
    values: &[ProfileValueV2Dto],
) -> Result<BTreeMap<ProfileId, ProfileSelection>, EvidenceAdmissionError> {
    let mut grouped: BTreeMap<ProfileId, (Revision, Vec<(DimensionId, SymbolicValueId)>)> =
        BTreeMap::new();
    for value in values {
        let profile =
            ProfileId::new(value.profile.as_str()).map_err(|_| malformed("identifier"))?;
        let revision = Revision::new(value.revision.get()).map_err(|_| malformed("revision"))?;
        let dimension =
            DimensionId::new(value.dimension.as_str()).map_err(|_| malformed("identifier"))?;
        let selected =
            SymbolicValueId::new(value.value.as_str()).map_err(|_| malformed("identifier"))?;
        let entry = grouped.entry(profile).or_insert((revision, Vec::new()));
        if entry.0 != revision {
            return Err(EvidenceAdmissionError::ProfileMismatch);
        }
        entry.1.push((dimension, selected));
    }
    grouped
        .into_iter()
        .map(|(profile, (revision, values))| {
            Ok((
                profile.clone(),
                ProfileSelection::new(profile, revision, values)
                    .map_err(EvidenceAdmissionError::Domain)?,
            ))
        })
        .collect()
}

fn attachments(
    values: &[AttachmentV2Dto],
) -> Result<Vec<EvidenceAttachment>, EvidenceAdmissionError> {
    values
        .iter()
        .map(|value| {
            Ok(EvidenceAttachment::new(
                selector(&value.name)?,
                selector(&value.media_type)?,
                digest(&value.digest)?,
                value.size.get(),
            ))
        })
        .collect()
}

fn scope_admits(scope: &EvidenceScopeSubject, target: &TargetId) -> bool {
    match scope {
        EvidenceScopeSubject::Target(value) => value == target,
        EvidenceScopeSubject::TargetSet(values) => values.contains(target),
        EvidenceScopeSubject::Provider(_) => true,
    }
}

fn requirements_belong_to_unit(requirements: &BTreeSet<FullRequirementId>, unit: &UnitId) -> bool {
    requirements.iter().all(|requirement| {
        requirement
            .as_str()
            .split_once('#')
            .is_some_and(|(owner, _)| owner == unit.as_str())
    })
}

fn counts_match_outcomes(counts: EvidenceCounts, attempts: &[EvidenceAttempt]) -> bool {
    let present = |outcome: AttemptOutcome| attempts.iter().any(|item| item.outcome() == outcome);
    (counts.passed > 0) == present(AttemptOutcome::Passed)
        && (counts.failed > 0) == present(AttemptOutcome::Failed)
        && (counts.skipped > 0) == present(AttemptOutcome::Skipped)
        && (counts.filtered > 0) == present(AttemptOutcome::Filtered)
        && (counts.quarantined > 0) == present(AttemptOutcome::Quarantined)
}

/// Requires each numbered retry to begin at or after the preceding retry's
/// finish. Equal boundaries are allowed; the comparison preserves nanoseconds.
fn retries_are_ordered(attempts: &[EvidenceAttempt]) -> bool {
    attempts
        .windows(2)
        .all(|pair| pair[1].started_at() >= pair[0].finished_at())
}

/// Admits one parsed v2 evidence record against explicit lineage, a declared
/// execution and an independently supplied expected context.
#[allow(clippy::too_many_lines)]
pub fn admit_evidence(
    checked: &CheckedEvidenceResultV2,
    lineage: &EvidenceLineage,
    execution: &EvidenceExecution,
    context: &EvidenceAdmissionContext,
) -> Result<AdmittedEvidenceV2, EvidenceAdmissionError> {
    let record = checked.record();
    lineage
        .validate_against(execution, &context.expected_lineage)
        .map_err(EvidenceAdmissionError::Lineage)?;

    // `id` must equal `result_digest` and both must equal the recomputed
    // semantic identity, which excludes those two members from its projection.
    let id = digest(&record.id)?;
    let result_digest = digest(&record.result_digest)?;
    let semantic = checked.semantic_identity().as_digest();
    if id != result_digest || id != semantic {
        return Err(EvidenceAdmissionError::IdentityMismatch);
    }
    if record.schema != crate::EVIDENCE_RESULT_SCHEMA_V2.to_string() {
        return Err(EvidenceAdmissionError::SchemaMismatch);
    }

    let kind = record
        .kind
        .parse::<EvidenceKind>()
        .map_err(|_| malformed("kind"))?;
    if kind != context.kind {
        return Err(EvidenceAdmissionError::KindMismatch);
    }

    let admitted_subject = subject(&record.subject)?;
    if admitted_subject != context.subject {
        return Err(EvidenceAdmissionError::SubjectMismatch);
    }
    let admitted_target =
        TargetId::new(record.target.as_str()).map_err(|_| malformed("identifier"))?;
    if admitted_target != context.target {
        return Err(EvidenceAdmissionError::TargetMismatch);
    }
    if !scope_admits(admitted_subject.scope(), &admitted_target) {
        return Err(EvidenceAdmissionError::TargetMismatch);
    }
    if let EvidenceScopeSubject::Provider(provider) = admitted_subject.scope() {
        let associated = context
            .provider_targets
            .get(provider)
            .is_some_and(|targets| targets.contains(&admitted_target));
        if !associated {
            return Err(EvidenceAdmissionError::ProviderUnassociated);
        }
    }
    let admitted_unit = UnitId::new(record.unit.as_str()).map_err(|_| malformed("identifier"))?;
    if admitted_unit != context.unit {
        return Err(EvidenceAdmissionError::UnitMismatch);
    }

    let admitted_requirements = record
        .requirements
        .iter()
        .map(|requirement| {
            FullRequirementId::new(requirement.as_str()).map_err(|_| malformed("identifier"))
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    if !requirements_belong_to_unit(&admitted_requirements, &admitted_unit)
        || !admitted_requirements.contains(&context.requirement)
    {
        return Err(EvidenceAdmissionError::RequirementMismatch);
    }

    let admitted_facets = record
        .facets
        .iter()
        .map(|facet| facet.parse::<Facet>().map_err(|_| malformed("facet")))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if admitted_facets != context.facets {
        return Err(EvidenceAdmissionError::FacetMismatch);
    }

    if digest(&record.evidence_spec_digest)? != context.evidence_spec_digest.as_digest()
        || digest(&record.contract_digest)? != context.contract_digest.as_digest()
        || digest(&record.binding_digest)? != context.binding_digest.as_digest()
        || digest(&record.policy_digest)? != context.policy_digest.as_digest()
    {
        return Err(EvidenceAdmissionError::DigestMismatch);
    }
    // The explicit target-configuration identity must equal the subject
    // configuration field; it is never inferred from the workspace digest.
    if context.subject.target_configuration_digest()
        != context.target_configuration_digest.as_digest()
    {
        return Err(EvidenceAdmissionError::SubjectMismatch);
    }

    let optional =
        |value: &Option<String>| -> Result<Option<Sha256Digest>, EvidenceAdmissionError> {
            value.as_deref().map(digest).transpose()
        };
    if optional(&record.runner_digest)? != context.runner_digest.map(RunnerDigest::as_digest)
        || optional(&record.adapter_digest)? != context.adapter_digest.map(AdapterDigest::as_digest)
        || optional(&record.runtime_facts_digest)? != context.runtime_facts_digest
        || optional(&record.release_record_digest)? != context.release_record_digest
    {
        return Err(EvidenceAdmissionError::DigestMismatch);
    }

    let admitted_profiles = profiles(&record.profile_values)?;
    if admitted_profiles != context.profiles {
        return Err(EvidenceAdmissionError::ProfileMismatch);
    }

    let admitted_payload = payload(&record.payload)?;
    if let EvidencePayload::RuntimeSnapshot {
        runtime_facts_digest,
        ..
    } = &admitted_payload
        && Some(*runtime_facts_digest) != context.runtime_facts_digest
    {
        return Err(EvidenceAdmissionError::RuntimeReferenceMismatch);
    }
    if let EvidencePayload::ReleaseRecord {
        release_record_digest,
    } = &admitted_payload
        && Some(*release_record_digest) != context.release_record_digest
    {
        return Err(EvidenceAdmissionError::ReleaseReferenceMismatch);
    }
    if let Some(attempted) = execution_of(&admitted_payload) {
        if !counts_match_outcomes(attempted.counts(), attempted.attempts()) {
            return Err(EvidenceAdmissionError::CountOutcomeContradiction);
        }
        if !retries_are_ordered(attempted.attempts()) {
            return Err(EvidenceAdmissionError::RetryChronology);
        }
    }

    let admitted_attachments = attachments(&record.attachments)?;
    let claimed_trust = record
        .claimed_trust
        .parse::<TrustLevel>()
        .map_err(|_| malformed("claimed trust"))?;
    let producer = record
        .producer
        .parse::<eqm_domain::ProducerRef>()
        .map_err(|_| malformed("producer"))?;
    let observed_at: UtcInstant = record
        .observed_at
        .get()
        .to_string()
        .parse()
        .map_err(|_| malformed("observed_at"))?;

    let result = EvidenceResult::new(
        id,
        admitted_subject,
        admitted_target,
        admitted_unit,
        admitted_requirements.into_iter().collect(),
        admitted_facets.into_iter().collect(),
        kind,
        context.evidence_spec_digest.as_digest(),
        context.contract_digest.as_digest(),
        context.binding_digest.as_digest(),
        context.policy_digest.as_digest(),
        context.runner_digest.map(RunnerDigest::as_digest),
        context.adapter_digest.map(AdapterDigest::as_digest),
        context.runtime_facts_digest,
        context.release_record_digest,
        admitted_profiles.into_values().collect(),
        producer,
        claimed_trust,
        observed_at,
        admitted_payload,
        admitted_attachments,
        result_digest,
        eqm_domain::Extensions::default(),
    )
    .map_err(EvidenceAdmissionError::Domain)?;

    Ok(AdmittedEvidenceV2 {
        result,
        lineage: lineage.clone(),
        execution: execution.clone(),
        semantic_identity: checked.semantic_identity(),
        raw_bytes: checked.raw_bytes().to_vec(),
        evidence_spec_digest: context.evidence_spec_digest,
        contract_digest: context.contract_digest,
        binding_digest: context.binding_digest,
        policy_digest: context.policy_digest,
        target_configuration_digest: context.target_configuration_digest,
        runner_digest: context.runner_digest,
        adapter_digest: context.adapter_digest,
    })
}

fn execution_of(payload: &EvidencePayload) -> Option<&ExecutionPayload> {
    match payload {
        EvidencePayload::StructuralCheck(value)
        | EvidencePayload::Test(value)
        | EvidencePayload::Snapshot(value) => Some(value),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2::{
        AttemptV2Dto, CanonicalInstant, CountsV2Dto, EvidenceResultV2Dto, EvidenceSubjectV2Dto,
        ExecutionPayloadV2Dto, ProfileValueV2Dto,
    };
    use crate::{CanonicalSet, CheckedEvidenceResultV2, WireUint};
    use eqm_domain::{AttemptId, CampaignId, LogicalCaseId, PlanId, RunId};
    use std::error::Error;

    fn digest(seed: &str) -> String {
        Sha256Digest::hash_content(seed.as_bytes()).to_string()
    }

    fn instant(value: &str) -> Result<CanonicalInstant, Box<dyn Error>> {
        Ok(CanonicalInstant::new(value.parse()?))
    }

    fn execution() -> Result<ExecutionPayloadV2Dto, Box<dyn Error>> {
        Ok(ExecutionPayloadV2Dto {
            attempts: vec![AttemptV2Dto {
                number: WireUint::new(1),
                outcome: "passed".to_owned(),
                started_at: instant("2026-01-02T03:04:05Z")?,
                finished_at: instant("2026-01-02T03:04:06Z")?,
                message: None,
            }],
            counts: CountsV2Dto {
                selected: WireUint::new(1),
                passed: WireUint::new(1),
                failed: WireUint::new(0),
                skipped: WireUint::new(0),
                filtered: WireUint::new(0),
                quarantined: WireUint::new(0),
            },
            started_at: instant("2026-01-02T03:04:05Z")?,
            finished_at: instant("2026-01-02T03:04:06Z")?,
        })
    }

    fn record() -> Result<EvidenceResultV2Dto, Box<dyn Error>> {
        let id = digest("placeholder");
        Ok(EvidenceResultV2Dto {
            schema: crate::EVIDENCE_RESULT_SCHEMA_V2.to_string(),
            id: id.clone(),
            subject: EvidenceSubjectV2Dto {
                repository: "https://example.com/team/project".to_owned(),
                repository_id_digest: digest("repo"),
                scope: ScopeSubjectV2Dto::Target {
                    target: "web".to_owned(),
                },
                source_commit: "a".repeat(40),
                build_id: None,
                artifact_digest: None,
                target_configuration_digest: TargetConfigurationDigest::hash_content(b"config")
                    .to_string(),
            },
            target: "web".to_owned(),
            unit: "unit.test".to_owned(),
            requirements: CanonicalSet::from_members(vec!["unit.test#r18".to_owned()])?,
            facets: CanonicalSet::from_members(vec!["behavior".to_owned()])?,
            kind: "test".to_owned(),
            evidence_spec_digest: EvidenceSpecificationDigest::hash_content(b"spec").to_string(),
            contract_digest: ContractDigest::hash_content(b"contract").to_string(),
            binding_digest: BindingDigest::hash_content(b"binding").to_string(),
            policy_digest: PolicySelectionDigest::hash_content(b"policy").to_string(),
            runner_digest: None,
            adapter_digest: None,
            runtime_facts_digest: None,
            release_record_digest: None,
            profile_values: CanonicalSet::empty(),
            producer: "producer://ci/runner/1".to_owned(),
            claimed_trust: "signed_ci".to_owned(),
            observed_at: instant("2026-01-02T03:04:05Z")?,
            payload: EvidencePayloadV2Dto::Test {
                execution: execution()?,
            },
            attachments: CanonicalSet::empty(),
            result_digest: id,
        })
    }

    /// Sets `id`/`result_digest` to the recomputed semantic identity and parses.
    fn seal(record: EvidenceResultV2Dto) -> Result<CheckedEvidenceResultV2, Box<dyn Error>> {
        let provisional = CheckedEvidenceResultV2::from_bytes(&serde_json::to_vec(&record)?)?;
        let semantic = provisional.semantic_identity().to_string();
        let mut sealed = record;
        sealed.id = semantic.clone();
        sealed.result_digest = semantic;
        Ok(CheckedEvidenceResultV2::from_bytes(&serde_json::to_vec(
            &sealed,
        )?)?)
    }

    fn lineage() -> Result<EvidenceLineage, Box<dyn Error>> {
        Ok(EvidenceLineage::from_parts(
            eqm_domain::EvidenceLineageParts {
                plan: Some(PlanId::new("plan.release")?),
                campaign: Some(CampaignId::new("campaign.signup")?),
                run: Some(RunId::new("run.one")?),
                logical_case: Some(LogicalCaseId::new("case.identifier")?),
                attempt: Some(AttemptId::new("attempt.one")?),
            },
        )?)
    }

    fn execution_record() -> Result<EvidenceExecution, Box<dyn Error>> {
        Ok(EvidenceExecution::new(
            RunId::new("run.one")?,
            LogicalCaseId::new("case.identifier")?,
            vec![AttemptId::new("attempt.one")?],
        )?)
    }

    fn context() -> Result<EvidenceAdmissionContext, Box<dyn Error>> {
        Ok(EvidenceAdmissionContext {
            subject: EvidenceSubject::new(
                "https://example.com/team/project".parse()?,
                Sha256Digest::hash_content(b"repo"),
                EvidenceScopeSubject::Target(TargetId::new("web")?),
                "a".repeat(40).parse()?,
                None,
                None,
                TargetConfigurationDigest::hash_content(b"config").as_digest(),
            ),
            target: TargetId::new("web")?,
            unit: UnitId::new("unit.test")?,
            requirement: FullRequirementId::new("unit.test#r18")?,
            facets: BTreeSet::from([Facet::Behavior]),
            kind: EvidenceKind::Test,
            evidence_spec_digest: EvidenceSpecificationDigest::hash_content(b"spec"),
            contract_digest: ContractDigest::hash_content(b"contract"),
            binding_digest: BindingDigest::hash_content(b"binding"),
            policy_digest: PolicySelectionDigest::hash_content(b"policy"),
            target_configuration_digest: TargetConfigurationDigest::hash_content(b"config"),
            runner_digest: None,
            adapter_digest: None,
            runtime_facts_digest: None,
            release_record_digest: None,
            profiles: BTreeMap::new(),
            provider_targets: BTreeMap::new(),
            expected_lineage: ExpectedLineageAuthority::new(
                PlanId::new("plan.release")?,
                CampaignId::new("campaign.signup")?,
            ),
        })
    }

    #[test]
    fn positive_fixture_admits_with_identity_and_relationships() -> Result<(), Box<dyn Error>> {
        let checked = seal(record()?)?;
        let admitted = admit_evidence(&checked, &lineage()?, &execution_record()?, &context()?)?;
        assert_eq!(admitted.result().target().as_str(), "web");
        assert_eq!(admitted.result().unit().as_str(), "unit.test");
        assert_eq!(admitted.result().kind(), EvidenceKind::Test);
        assert_eq!(admitted.semantic_identity(), checked.semantic_identity());
        assert_eq!(admitted.raw_bytes(), checked.raw_bytes());
        assert_eq!(admitted.lineage().attempt().as_str(), "attempt.one");
        Ok(())
    }

    #[test]
    fn identity_must_equal_the_recomputed_semantic_digest() -> Result<(), Box<dyn Error>> {
        let mut value = record()?;
        let other = digest("other");
        value.id = other.clone();
        value.result_digest = other;
        let checked = CheckedEvidenceResultV2::from_bytes(&serde_json::to_vec(&value)?)?;
        assert_eq!(
            admit_evidence(&checked, &lineage()?, &execution_record()?, &context()?),
            Err(EvidenceAdmissionError::IdentityMismatch)
        );
        Ok(())
    }

    #[test]
    fn one_axis_mismatches_reject_before_admission() -> Result<(), Box<dyn Error>> {
        let mut cases: Vec<(&str, EvidenceResultV2Dto, EvidenceAdmissionError)> = Vec::new();

        let mut value = record()?;
        value.kind = "snapshot".to_owned();
        cases.push(("kind", value, EvidenceAdmissionError::KindMismatch));

        let mut value = record()?;
        value.subject.repository = "https://example.com/team/other".to_owned();
        cases.push(("subject", value, EvidenceAdmissionError::SubjectMismatch));

        let mut value = record()?;
        value.target = "ios".to_owned();
        cases.push(("target", value, EvidenceAdmissionError::TargetMismatch));

        let mut value = record()?;
        value.unit = "unit.other".to_owned();
        cases.push(("unit", value, EvidenceAdmissionError::UnitMismatch));

        let mut value = record()?;
        value.requirements = CanonicalSet::from_members(vec!["unit.test#r19".to_owned()])?;
        cases.push((
            "requirement",
            value,
            EvidenceAdmissionError::RequirementMismatch,
        ));

        let mut value = record()?;
        value.requirements = CanonicalSet::from_members(vec!["other.unit#r18".to_owned()])?;
        cases.push((
            "requirement-unit",
            value,
            EvidenceAdmissionError::RequirementMismatch,
        ));

        let mut value = record()?;
        value.facets = CanonicalSet::from_members(vec!["structure".to_owned()])?;
        cases.push(("facet", value, EvidenceAdmissionError::FacetMismatch));

        let mut value = record()?;
        value.evidence_spec_digest = digest("spec-2");
        cases.push(("digest", value, EvidenceAdmissionError::DigestMismatch));

        let mut value = record()?;
        value.profile_values = CanonicalSet::from_members(vec![ProfileValueV2Dto {
            profile: "audience.default".to_owned(),
            revision: WireUint::new(1),
            dimension: "locale".to_owned(),
            value: "en".to_owned(),
        }])?;
        cases.push(("profile", value, EvidenceAdmissionError::ProfileMismatch));

        for (name, value, expected) in cases {
            let checked = seal(value)?;
            assert_eq!(
                admit_evidence(&checked, &lineage()?, &execution_record()?, &context()?),
                Err(expected),
                "{name}"
            );
        }
        Ok(())
    }

    #[test]
    fn provider_scope_requires_a_declared_association() -> Result<(), Box<dyn Error>> {
        let mut value = record()?;
        value.subject.scope = ScopeSubjectV2Dto::Provider {
            provider: "provider.one".to_owned(),
        };
        let checked = seal(value)?;
        let mut expected = context()?;
        expected.subject = EvidenceSubject::new(
            "https://example.com/team/project".parse()?,
            Sha256Digest::hash_content(b"repo"),
            EvidenceScopeSubject::Provider(ProviderId::new("provider.one")?),
            "a".repeat(40).parse()?,
            None,
            None,
            TargetConfigurationDigest::hash_content(b"config").as_digest(),
        );
        assert_eq!(
            admit_evidence(&checked, &lineage()?, &execution_record()?, &expected),
            Err(EvidenceAdmissionError::ProviderUnassociated)
        );
        expected.provider_targets.insert(
            ProviderId::new("provider.one")?,
            BTreeSet::from([TargetId::new("web")?]),
        );
        assert!(admit_evidence(&checked, &lineage()?, &execution_record()?, &expected).is_ok());
        Ok(())
    }

    #[test]
    fn counts_must_not_contradict_attempt_outcomes() -> Result<(), Box<dyn Error>> {
        let mut value = record()?;
        let EvidencePayloadV2Dto::Test { execution } = &mut value.payload else {
            return Err("test payload".into());
        };
        execution.attempts[0].outcome = "failed".to_owned();
        // Arithmetic is valid (selected = passed + failed = 1) but no attempt
        // passed while `passed` claims a passing case.
        execution.counts.passed = WireUint::new(1);
        execution.counts.failed = WireUint::new(0);
        let checked = seal(value)?;
        assert_eq!(
            admit_evidence(&checked, &lineage()?, &execution_record()?, &context()?),
            Err(EvidenceAdmissionError::CountOutcomeContradiction)
        );
        Ok(())
    }

    #[test]
    fn runtime_and_release_payload_references_must_match_the_envelope() -> Result<(), Box<dyn Error>>
    {
        let mut value = record()?;
        value.kind = "runtime_snapshot".to_owned();
        value.runtime_facts_digest = Some(digest("runtime"));
        value.payload = EvidencePayloadV2Dto::RuntimeSnapshot {
            runtime_facts_digest: digest("runtime-other"),
            counts: CountsV2Dto {
                selected: WireUint::new(1),
                passed: WireUint::new(1),
                failed: WireUint::new(0),
                skipped: WireUint::new(0),
                filtered: WireUint::new(0),
                quarantined: WireUint::new(0),
            },
        };
        let checked = seal(value)?;
        let mut expected = context()?;
        expected.kind = EvidenceKind::RuntimeSnapshot;
        expected.runtime_facts_digest = Some(Sha256Digest::hash_content(b"runtime"));
        assert_eq!(
            admit_evidence(&checked, &lineage()?, &execution_record()?, &expected),
            Err(EvidenceAdmissionError::RuntimeReferenceMismatch)
        );

        let mut value = record()?;
        value.kind = "release_record".to_owned();
        value.release_record_digest = Some(digest("release"));
        value.payload = EvidencePayloadV2Dto::ReleaseRecord {
            release_record_digest: digest("release-other"),
        };
        let checked = seal(value)?;
        let mut expected = context()?;
        expected.kind = EvidenceKind::ReleaseRecord;
        expected.release_record_digest = Some(Sha256Digest::hash_content(b"release"));
        assert_eq!(
            admit_evidence(&checked, &lineage()?, &execution_record()?, &expected),
            Err(EvidenceAdmissionError::ReleaseReferenceMismatch)
        );
        Ok(())
    }

    #[test]
    fn lineage_mismatch_rejects_at_the_admission_boundary() -> Result<(), Box<dyn Error>> {
        let checked = seal(record()?)?;
        let mut expected = context()?;
        expected.expected_lineage = ExpectedLineageAuthority::new(
            PlanId::new("plan.other")?,
            CampaignId::new("campaign.signup")?,
        );
        assert_eq!(
            admit_evidence(&checked, &lineage()?, &execution_record()?, &expected),
            Err(EvidenceAdmissionError::Lineage(
                EvidenceResultBuildError::PlanMismatch
            ))
        );
        Ok(())
    }

    fn attempt_dto(
        number: u64,
        started: &str,
        finished: &str,
    ) -> Result<AttemptV2Dto, Box<dyn Error>> {
        Ok(AttemptV2Dto {
            number: WireUint::new(number),
            outcome: "passed".to_owned(),
            started_at: instant(started)?,
            finished_at: instant(finished)?,
            message: None,
        })
    }

    fn execution_payload(
        kind: &str,
        attempts: Vec<AttemptV2Dto>,
    ) -> Result<EvidencePayloadV2Dto, Box<dyn Error>> {
        let execution = ExecutionPayloadV2Dto {
            attempts,
            counts: CountsV2Dto {
                selected: WireUint::new(1),
                passed: WireUint::new(1),
                failed: WireUint::new(0),
                skipped: WireUint::new(0),
                filtered: WireUint::new(0),
                quarantined: WireUint::new(0),
            },
            started_at: instant("2026-01-02T03:00:00Z")?,
            finished_at: instant("2026-01-02T04:00:00Z")?,
        };
        Ok(match kind {
            "structural_check" => EvidencePayloadV2Dto::StructuralCheck { execution },
            "snapshot" => EvidencePayloadV2Dto::Snapshot { execution },
            _ => EvidencePayloadV2Dto::Test { execution },
        })
    }

    fn record_with_attempts(
        kind: &str,
        attempts: Vec<AttemptV2Dto>,
    ) -> Result<EvidenceResultV2Dto, Box<dyn Error>> {
        let mut value = record()?;
        value.kind = kind.to_owned();
        value.payload = execution_payload(kind, attempts)?;
        Ok(value)
    }

    fn context_for_kind(kind: EvidenceKind) -> Result<EvidenceAdmissionContext, Box<dyn Error>> {
        let mut expected = context()?;
        expected.kind = kind;
        Ok(expected)
    }

    #[test]
    fn retained_typed_expectations_are_exposed() -> Result<(), Box<dyn Error>> {
        let checked = seal(record()?)?;
        let expected = context()?;
        let admitted = admit_evidence(&checked, &lineage()?, &execution_record()?, &expected)?;
        assert_eq!(admitted.contract_digest(), expected.contract_digest);
        assert_eq!(admitted.binding_digest(), expected.binding_digest);
        assert_eq!(
            admitted.evidence_spec_digest(),
            expected.evidence_spec_digest
        );
        assert_eq!(admitted.policy_digest(), expected.policy_digest);
        assert_eq!(
            admitted.target_configuration_digest(),
            expected.target_configuration_digest
        );
        assert_eq!(admitted.runner_digest(), None);
        assert_eq!(admitted.adapter_digest(), None);
        Ok(())
    }

    #[test]
    fn subject_core_reference_and_scope_mutants_reject() -> Result<(), Box<dyn Error>> {
        let mut cases: Vec<(&str, EvidenceResultV2Dto, EvidenceAdmissionError)> = Vec::new();

        let mut value = record()?;
        value.subject.repository = "https://example.com/team/other".to_owned();
        cases.push(("repository", value, EvidenceAdmissionError::SubjectMismatch));

        let mut value = record()?;
        value.subject.repository_id_digest = digest("repo-2");
        cases.push((
            "repository-id",
            value,
            EvidenceAdmissionError::SubjectMismatch,
        ));

        let mut value = record()?;
        value.subject.source_commit = "b".repeat(40);
        cases.push((
            "source-commit",
            value,
            EvidenceAdmissionError::SubjectMismatch,
        ));

        let mut value = record()?;
        value.subject.build_id = Some("build-2".to_owned());
        cases.push(("build", value, EvidenceAdmissionError::SubjectMismatch));

        let mut value = record()?;
        value.subject.artifact_digest = Some(digest("artifact"));
        cases.push(("artifact", value, EvidenceAdmissionError::SubjectMismatch));

        let mut value = record()?;
        value.subject.target_configuration_digest =
            TargetConfigurationDigest::hash_content(b"config-2").to_string();
        cases.push((
            "target-configuration",
            value,
            EvidenceAdmissionError::SubjectMismatch,
        ));

        let mut value = record()?;
        value.subject.scope = ScopeSubjectV2Dto::TargetSet {
            targets: CanonicalSet::from_members(vec!["web".to_owned(), "ios".to_owned()])?,
        };
        cases.push(("scope", value, EvidenceAdmissionError::SubjectMismatch));

        let mut value = record()?;
        value.contract_digest = ContractDigest::hash_content(b"contract-2").to_string();
        cases.push(("contract", value, EvidenceAdmissionError::DigestMismatch));

        let mut value = record()?;
        value.binding_digest = BindingDigest::hash_content(b"binding-2").to_string();
        cases.push(("binding", value, EvidenceAdmissionError::DigestMismatch));

        let mut value = record()?;
        value.policy_digest = PolicySelectionDigest::hash_content(b"policy-2").to_string();
        cases.push(("policy", value, EvidenceAdmissionError::DigestMismatch));

        let mut value = record()?;
        value.runner_digest = Some(RunnerDigest::hash_content(b"runner").to_string());
        cases.push((
            "runner-present",
            value,
            EvidenceAdmissionError::DigestMismatch,
        ));

        let mut value = record()?;
        value.adapter_digest = Some(AdapterDigest::hash_content(b"adapter").to_string());
        cases.push((
            "adapter-present",
            value,
            EvidenceAdmissionError::DigestMismatch,
        ));

        for (name, value, expected) in cases {
            let checked = seal(value)?;
            assert_eq!(
                admit_evidence(&checked, &lineage()?, &execution_record()?, &context()?),
                Err(expected),
                "{name}"
            );
        }

        // A present typed context expectation against an absent record reference.
        for context_with in ["runner", "adapter"] {
            let mut expected = context()?;
            if context_with == "runner" {
                expected.runner_digest = Some(RunnerDigest::hash_content(b"runner"));
            } else {
                expected.adapter_digest = Some(AdapterDigest::hash_content(b"adapter"));
            }
            assert_eq!(
                admit_evidence(
                    &seal(record()?)?,
                    &lineage()?,
                    &execution_record()?,
                    &expected
                ),
                Err(EvidenceAdmissionError::DigestMismatch),
                "absent record {context_with}"
            );
        }

        // Positive counterparts: an exact present typed runner/adapter identity admits.
        let mut value = record()?;
        value.runner_digest = Some(RunnerDigest::hash_content(b"runner").to_string());
        value.adapter_digest = Some(AdapterDigest::hash_content(b"adapter").to_string());
        let checked = seal(value)?;
        let mut expected = context()?;
        expected.runner_digest = Some(RunnerDigest::hash_content(b"runner"));
        expected.adapter_digest = Some(AdapterDigest::hash_content(b"adapter"));
        let admitted = admit_evidence(&checked, &lineage()?, &execution_record()?, &expected)?;
        assert_eq!(admitted.runner_digest(), expected.runner_digest);
        assert_eq!(admitted.adapter_digest(), expected.adapter_digest);
        Ok(())
    }

    #[test]
    fn profile_revision_and_value_are_exact() -> Result<(), Box<dyn Error>> {
        let profile_id = ProfileId::new("audience.default")?;
        let mut expected = context()?;
        expected.profiles.insert(
            profile_id.clone(),
            ProfileSelection::new(
                profile_id,
                Revision::new(1)?,
                vec![(DimensionId::new("locale")?, SymbolicValueId::new("en")?)],
            )?,
        );
        // Exact match admits.
        let mut value = record()?;
        value.profile_values = CanonicalSet::from_members(vec![ProfileValueV2Dto {
            profile: "audience.default".to_owned(),
            revision: WireUint::new(1),
            dimension: "locale".to_owned(),
            value: "en".to_owned(),
        }])?;
        assert!(
            admit_evidence(&seal(value)?, &lineage()?, &execution_record()?, &expected).is_ok()
        );
        // A different value rejects.
        let mut value = record()?;
        value.profile_values = CanonicalSet::from_members(vec![ProfileValueV2Dto {
            profile: "audience.default".to_owned(),
            revision: WireUint::new(1),
            dimension: "locale".to_owned(),
            value: "fr".to_owned(),
        }])?;
        assert_eq!(
            admit_evidence(&seal(value)?, &lineage()?, &execution_record()?, &expected),
            Err(EvidenceAdmissionError::ProfileMismatch)
        );
        // A different revision rejects.
        let mut value = record()?;
        value.profile_values = CanonicalSet::from_members(vec![ProfileValueV2Dto {
            profile: "audience.default".to_owned(),
            revision: WireUint::new(2),
            dimension: "locale".to_owned(),
            value: "en".to_owned(),
        }])?;
        assert_eq!(
            admit_evidence(&seal(value)?, &lineage()?, &execution_record()?, &expected),
            Err(EvidenceAdmissionError::ProfileMismatch)
        );
        Ok(())
    }

    #[test]
    fn payload_kind_must_agree_with_envelope_kind() -> Result<(), Box<dyn Error>> {
        let mut value = record()?;
        value.payload = EvidencePayloadV2Dto::Snapshot {
            execution: execution()?,
        };
        assert!(matches!(
            admit_evidence(
                &seal(value)?,
                &lineage()?,
                &execution_record()?,
                &context()?
            ),
            Err(EvidenceAdmissionError::Domain(
                EvidenceResultBuildError::PayloadKindMismatch
            ))
        ));
        Ok(())
    }

    #[test]
    fn retry_chronology_control_accepts_ordered_and_rejects_reversed() -> Result<(), Box<dyn Error>>
    {
        // The same sealed temporal matrix runs for every execution-bearing kind
        // with its context kind kept consistent.
        for (kind_str, kind) in [
            ("structural_check", EvidenceKind::StructuralCheck),
            ("test", EvidenceKind::Test),
            ("snapshot", EvidenceKind::Snapshot),
        ] {
            let expected = context_for_kind(kind)?;

            // Strictly sequential.
            let sequential = vec![
                attempt_dto(1, "2026-01-02T03:04:05Z", "2026-01-02T03:04:06Z")?,
                attempt_dto(2, "2026-01-02T03:04:07Z", "2026-01-02T03:04:08Z")?,
            ];
            assert!(
                admit_evidence(
                    &seal(record_with_attempts(kind_str, sequential)?)?,
                    &lineage()?,
                    &execution_record()?,
                    &expected
                )
                .is_ok(),
                "{kind_str} sequential"
            );

            // Equal boundaries are allowed and nanoseconds are retained.
            let equal = vec![
                attempt_dto(
                    1,
                    "2026-01-02T03:04:05.000000001Z",
                    "2026-01-02T03:04:05.000000002Z",
                )?,
                attempt_dto(
                    2,
                    "2026-01-02T03:04:05.000000002Z",
                    "2026-01-02T03:04:05.000000003Z",
                )?,
            ];
            assert!(
                admit_evidence(
                    &seal(record_with_attempts(kind_str, equal)?)?,
                    &lineage()?,
                    &execution_record()?,
                    &expected
                )
                .is_ok(),
                "{kind_str} equal boundary"
            );

            // Reversed: retry 2 begins before retry 1 finishes.
            let reversed = vec![
                attempt_dto(1, "2026-01-02T03:04:09Z", "2026-01-02T03:04:10Z")?,
                attempt_dto(2, "2026-01-02T03:04:05Z", "2026-01-02T03:04:06Z")?,
            ];
            assert_eq!(
                admit_evidence(
                    &seal(record_with_attempts(kind_str, reversed)?)?,
                    &lineage()?,
                    &execution_record()?,
                    &expected
                ),
                Err(EvidenceAdmissionError::RetryChronology),
                "{kind_str} reversed"
            );

            // Overlapping: retry 2 begins before retry 1 finishes.
            let overlapping = vec![
                attempt_dto(1, "2026-01-02T03:04:05Z", "2026-01-02T03:04:08Z")?,
                attempt_dto(2, "2026-01-02T03:04:07Z", "2026-01-02T03:04:09Z")?,
            ];
            assert_eq!(
                admit_evidence(
                    &seal(record_with_attempts(kind_str, overlapping)?)?,
                    &lineage()?,
                    &execution_record()?,
                    &expected
                ),
                Err(EvidenceAdmissionError::RetryChronology),
                "{kind_str} overlapping"
            );

            // Out of the aggregate window: domain attempt-window containment rejects.
            let out_of_window = vec![attempt_dto(
                1,
                "2026-01-02T02:59:00Z",
                "2026-01-02T02:59:30Z",
            )?];
            assert!(
                matches!(
                    admit_evidence(
                        &seal(record_with_attempts(kind_str, out_of_window)?)?,
                        &lineage()?,
                        &execution_record()?,
                        &expected
                    ),
                    Err(EvidenceAdmissionError::Domain(
                        EvidenceResultBuildError::InvalidAttemptSequence
                    ))
                ),
                "{kind_str} out of window"
            );
        }
        Ok(())
    }
}
