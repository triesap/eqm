//! Complete structural eligibility over explicit frozen data, never verification.

use crate::{
    CandidateNormalizationError, CoverageExpectation, CoverageMismatch, CoverageStatus,
    EvidenceCandidate, EvidenceNormalizationCandidate, NormalizedEvidenceSet, ObligationKey,
    ScopeSubject, normalize_evidence_candidates,
};
use eqm_domain::{
    EvidenceLineage, EvidenceScopeSubject, EvidenceSubject, ExpectedSelectionManifest,
    ObligationDigest, ScenarioParticipant, ScenarioSubject, SemanticEvidenceDigest, Sha256Digest,
    SubjectManifest,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Maximum structural comparison work; overflow fails without partial output.
pub const MAX_COMPLETE_COVERAGE_WORK: usize = 1_000_000;

/// Complete explicit participant-to-subject association. This is input data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoverageSubjectManifest {
    /// All exact occurrence/role/tag assignments, without target-name collapse.
    pub participants: BTreeSet<ScenarioParticipant>,
    /// Complete source/configuration/build/artifact assignments per occurrence.
    pub subjects: SubjectManifest,
}

/// Independent pre-result expectations. Constructing this grants no authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompleteCoverageExpectation {
    /// Exact component, profile and primary execution subject expectations.
    pub structural: CoverageExpectation,
    /// Complete immutable obligation context, including roles and dependencies.
    pub coordinate: ObligationKey,
    /// Pre-result identity prepared independently by the application/manifest layer.
    pub identity: ObligationDigest,
    /// Frozen binding/specification/plan/campaign and selected logical cases.
    pub selection: ExpectedSelectionManifest,
    /// Required complete grouped subjects; typed absence for a target scope.
    pub subjects: Option<CoverageSubjectManifest>,
}

/// One actual admission's explicit context claims. Claims are not verification.
#[derive(Clone, Copy, Debug)]
pub struct CompleteEvidenceCandidate<'a> {
    /// Checked structural delivery, retaining original mapping/provenance references.
    pub delivery: EvidenceNormalizationCandidate<'a>,
    /// Claimed consuming coordinate; missing context is never copied from expected.
    pub coordinate: Option<&'a ObligationKey>,
    /// Claimed pre-result identity; a matching tag alone supplies no authority.
    pub identity: Option<ObligationDigest>,
    /// Independently matched grouped subjects are required by protected consumers.
    pub subjects: Option<&'a CoverageSubjectManifest>,
}

/// Every rejected structural axis, retained deterministically.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CompleteCoverageMismatch {
    /// Existing exact component/subject/profile mismatch.
    Structural(CoverageMismatch),
    /// Complete coordinate, including scope/roles/dependencies/context, differed.
    Coordinate,
    /// Pre-result complete semantic context differed or was missing.
    ObligationIdentity,
    /// Grouped subject/occurrence mapping differed or was absent.
    SubjectManifest,
    /// Explicit selected plan differed.
    Plan,
    /// Explicit selected campaign differed.
    Campaign,
    /// The logical case was not a frozen selected member.
    LogicalCase,
}

/// Malformed expectations or bounded input fail without returning partial success.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompleteCoverageError {
    /// Expected components, coordinate, selection or required subjects contradict.
    InvalidExpectation,
    /// Complete context comparison work exceeds its declared bound.
    WorkLimitExceeded,
    /// Complete input could not be normalized safely.
    Normalization(CandidateNormalizationError),
}
impl Display for CompleteCoverageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "complete evidence matching failed: {self:?}")
    }
}
impl Error for CompleteCoverageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Normalization(error) => Some(error),
            Self::InvalidExpectation | Self::WorkLimitExceeded => None,
        }
    }
}

/// Structural eligibility only; no count, authentication or conformance conclusion.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompleteCoverageReport<'a> {
    status: CoverageStatus,
    eligible: BTreeSet<(SemanticEvidenceDigest, EvidenceLineage)>,
    rejected: BTreeMap<Sha256Digest, BTreeSet<CompleteCoverageMismatch>>,
    conflicts: BTreeSet<(EvidenceSubject, EvidenceLineage)>,
    normalized: NormalizedEvidenceSet<'a>,
}
impl<'a> CompleteCoverageReport<'a> {
    /// Returns structural matching status, never effective trust or satisfaction.
    #[must_use]
    pub const fn status(&self) -> CoverageStatus {
        self.status
    }
    /// Returns unique matching assertions with lineage; length is not count credit.
    #[must_use]
    pub const fn eligible(&self) -> &BTreeSet<(SemanticEvidenceDigest, EvidenceLineage)> {
        &self.eligible
    }
    /// Returns all rejected axes, including unrelated candidates.
    #[must_use]
    pub const fn rejected(&self) -> &BTreeMap<Sha256Digest, BTreeSet<CompleteCoverageMismatch>> {
        &self.rejected
    }
    /// Returns only same-attempt contradictions that passed every required axis.
    #[must_use]
    pub const fn conflicts(&self) -> &BTreeSet<(EvidenceSubject, EvidenceLineage)> {
        &self.conflicts
    }
    /// Returns every normalized assertion and original raw/mapping/provenance reference.
    #[must_use]
    pub const fn normalized(&self) -> &NormalizedEvidenceSet<'a> {
        &self.normalized
    }
}

fn subjects_valid(value: &CoverageSubjectManifest) -> bool {
    if value.subjects.require_complete().is_err()
        || value.participants.len() != value.subjects.assignments().len()
    {
        return false;
    }
    let assignments: BTreeMap<_, _> = value
        .subjects
        .assignments()
        .iter()
        .map(|assignment| (assignment.occurrence(), assignment))
        .collect();
    let mut occurrences = BTreeSet::new();
    value.participants.iter().all(|participant| {
        occurrences.insert(participant.occurrence())
            && assignments
                .get(participant.occurrence())
                .is_some_and(|assignment| match participant.subject() {
                    ScenarioSubject::Target(_) => {
                        assignment.target_configuration().is_some()
                            && assignment.provider_configuration().is_none()
                    }
                    ScenarioSubject::Provider(_) => {
                        assignment.provider_configuration().is_some()
                            && assignment.target_configuration().is_none()
                    }
                })
    })
}

fn expected_valid(expected: &CompleteCoverageExpectation) -> bool {
    let base = &expected.structural;
    let key = &expected.coordinate;
    let source = expected.selection.source();
    if base.unit != key.unit
        || base.requirement != key.requirement
        || base.facet != key.facet
        || base.release_record_digest != key.release_context
        || base.evidence_spec_id != source.specification
        || base.evidence_spec_digest != source.specification_digest.as_digest()
        || base.binding_digest != source.binding_digest.as_digest()
        || base.kind != expected.selection.kind()
        || !key.subject.participates(&base.target)
        || base.profiles.len() != key.profiles.len()
        || key.profiles.iter().any(|(id, selected)| {
            base.profiles.get(id).is_none_or(|profile| {
                profile.profile() != id
                    || profile.revision() != selected.revision()
                    || profile.values().len() != selected.values().len()
                    || selected.values().iter().any(|(dimension, value)| {
                        value
                            .as_ref()
                            .is_none_or(|value| profile.values().get(dimension) != Some(value))
                    })
            })
        })
    {
        return false;
    }
    match (&key.subject, base.subject.scope(), &expected.subjects) {
        (ScopeSubject::Target(target), EvidenceScopeSubject::Target(actual), None) => {
            target == actual
        }
        (
            ScopeSubject::Scenario(scope),
            EvidenceScopeSubject::TargetSet(targets),
            Some(subjects),
        ) => {
            targets == scope.selected_targets()
                && subjects.participants == *scope.participants()
                && subjects_valid(subjects)
                && primary_subject_matches(base, subjects)
        }
        (
            ScopeSubject::CheckedProvider(scope),
            EvidenceScopeSubject::Provider(provider),
            Some(subjects),
        ) => {
            let required: BTreeSet<_> = scope
                .dependers()
                .iter()
                .filter(|target| scope.participates(target))
                .cloned()
                .map(ScenarioSubject::Target)
                .chain(std::iter::once(ScenarioSubject::Provider(
                    scope.provider().clone(),
                )))
                .collect();
            let actual: BTreeSet<_> = subjects
                .participants
                .iter()
                .map(|p| p.subject().clone())
                .collect();
            provider == scope.provider()
                && actual == required
                && actual.len() == subjects.participants.len()
                && subjects_valid(subjects)
                && primary_subject_matches(base, subjects)
        }
        _ => false,
    }
}

fn primary_subject_matches(base: &CoverageExpectation, subjects: &CoverageSubjectManifest) -> bool {
    let assignments: BTreeMap<_, _> = subjects
        .subjects
        .assignments()
        .iter()
        .map(|assignment| (assignment.occurrence(), assignment))
        .collect();
    subjects.participants.iter().any(|participant| {
        participant.subject() == &ScenarioSubject::Target(base.target.clone())
            && assignments
                .get(participant.occurrence())
                .is_some_and(|assignment| {
                    assignment.repository() == base.subject.repository()
                        && assignment.source_commit() == base.subject.source_commit()
                        && assignment.build_id() == base.subject.build_id()
                        && assignment.artifact_digest() == base.subject.artifact_digest()
                        && assignment
                            .target_configuration()
                            .map(|value| value.as_digest())
                            == Some(base.subject.target_configuration_digest())
                })
    })
}

fn context_work(key: &ObligationKey, subjects: Option<&CoverageSubjectManifest>) -> usize {
    let mut work = 16usize;
    for profile in key.profiles.values() {
        work = work.saturating_add(profile.values().len());
    }
    work = work.saturating_add(match &key.subject {
        ScopeSubject::Target(_) | ScopeSubject::Provider(_) => 1,
        ScopeSubject::TargetSet(targets) => targets.len(),
        ScopeSubject::CheckedProvider(scope) => scope
            .dependers()
            .len()
            .saturating_add(scope.assignments().len())
            .saturating_add(scope.dependencies().len())
            .saturating_add(scope.participation().len()),
        ScopeSubject::Scenario(scope) => scope
            .participants()
            .len()
            .saturating_add(scope.dependencies().len())
            .saturating_add(scope.selected_targets().len())
            .saturating_add(scope.participation().len())
            .saturating_add(scope.provider_dependencies().values().fold(
                0usize,
                |sum, provider| {
                    sum.saturating_add(provider.dependers().len())
                        .saturating_add(provider.assignments().len())
                        .saturating_add(provider.dependencies().len())
                        .saturating_add(provider.participation().len())
                },
            )),
    });
    if let Some(subjects) = subjects {
        work = work
            .saturating_add(subjects.participants.len())
            .saturating_add(subjects.subjects.assignments().len());
    }
    work
}

/// Matches complete frozen coordinates before considering attempt conflicts.
/// Input structures and matching identities remain claims; protected consumers
/// separately validate graph provenance, signature/trust and the frozen input chain.
pub fn evaluate_complete_evidence_coverage<'a>(
    expected: &CompleteCoverageExpectation,
    candidates: &[CompleteEvidenceCandidate<'a>],
) -> Result<CompleteCoverageReport<'a>, CompleteCoverageError> {
    if candidates.len() > crate::MAX_CANDIDATE_DELIVERIES {
        return Err(CompleteCoverageError::Normalization(
            CandidateNormalizationError::LimitExceeded,
        ));
    }
    let expected_work = context_work(&expected.coordinate, expected.subjects.as_ref());
    let mut work = expected_work.saturating_add(expected.selection.selected().len());
    for candidate in candidates {
        work = work.saturating_add(expected_work).saturating_add(
            candidate
                .coordinate
                .map_or(1, |key| context_work(key, candidate.subjects)),
        );
        if work > MAX_COMPLETE_COVERAGE_WORK {
            return Err(CompleteCoverageError::WorkLimitExceeded);
        }
    }
    if work > MAX_COMPLETE_COVERAGE_WORK {
        return Err(CompleteCoverageError::WorkLimitExceeded);
    }
    if !expected_valid(expected) {
        return Err(CompleteCoverageError::InvalidExpectation);
    }
    let deliveries = candidates
        .iter()
        .map(|candidate| candidate.delivery)
        .collect::<Vec<_>>();
    let normalized =
        normalize_evidence_candidates(&deliveries).map_err(CompleteCoverageError::Normalization)?;
    let mut rejected: BTreeMap<Sha256Digest, BTreeSet<CompleteCoverageMismatch>> = BTreeMap::new();
    let mut matching: BTreeMap<
        (EvidenceSubject, EvidenceLineage),
        BTreeSet<SemanticEvidenceDigest>,
    > = BTreeMap::new();
    for candidate in candidates {
        let delivery = &candidate.delivery;
        let mut axes: BTreeSet<_> = super::coverage::mismatches(
            &expected.structural,
            &EvidenceCandidate {
                evidence_spec_id: delivery.evidence_spec_id,
                result: delivery.result,
            },
        )
        .into_iter()
        .map(CompleteCoverageMismatch::Structural)
        .collect();
        let source = expected.selection.source();
        for (matched, axis) in [
            (
                candidate.coordinate == Some(&expected.coordinate),
                CompleteCoverageMismatch::Coordinate,
            ),
            (
                candidate.identity == Some(expected.identity),
                CompleteCoverageMismatch::ObligationIdentity,
            ),
            (
                candidate.subjects == expected.subjects.as_ref(),
                CompleteCoverageMismatch::SubjectManifest,
            ),
            (
                delivery.lineage.plan() == &source.plan,
                CompleteCoverageMismatch::Plan,
            ),
            (
                delivery.lineage.campaign() == &source.campaign,
                CompleteCoverageMismatch::Campaign,
            ),
            (
                expected
                    .selection
                    .selected()
                    .contains(delivery.lineage.logical_case()),
                CompleteCoverageMismatch::LogicalCase,
            ),
        ] {
            if !matched {
                axes.insert(axis);
            }
        }
        if axes.is_empty() {
            matching
                .entry((delivery.result.subject().clone(), delivery.lineage.clone()))
                .or_default()
                .insert(delivery.semantic_identity);
        } else {
            rejected
                .entry(delivery.result.id())
                .or_default()
                .extend(axes);
        }
    }
    let mut eligible = BTreeSet::new();
    let mut conflicts = BTreeSet::new();
    for (key, identities) in matching {
        if identities.len() > 1 {
            conflicts.insert(key);
        } else {
            eligible.extend(
                identities
                    .into_iter()
                    .map(|identity| (identity, key.1.clone())),
            );
        }
    }
    let status = if !conflicts.is_empty() {
        CoverageStatus::Unknown
    } else if eligible.is_empty() {
        CoverageStatus::Missing
    } else {
        CoverageStatus::Covered
    };
    Ok(CompleteCoverageReport {
        status,
        eligible,
        rejected,
        conflicts,
        normalized,
    })
}
