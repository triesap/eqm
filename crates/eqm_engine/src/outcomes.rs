//! Evidence attempt and count aggregation across exact covered results.

use eqm_domain::{
    AttemptAggregate, AttemptOutcome, EvidenceCounts, EvidencePayload, PositiveCount,
};

/// Aggregate facet outcome before freshness and trust.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceOutcome {
    /// Every relevant terminal result passed and the minimum count was met.
    Satisfied,
    /// A terminal failure occurred without a pass.
    Failed,
    /// No result, zero selection, skips, filters, quarantine, or low count prevented coverage.
    Missing,
    /// Passing and failing immutable history coexist.
    Unstable,
    /// Timeout, cancellation, or execution error prevents a conclusion.
    Unknown,
}

/// Aggregates all covered payloads without erasing retry or failure history.
#[must_use]
pub fn aggregate_evidence_outcomes(
    payloads: &[&EvidencePayload],
    minimum: PositiveCount,
) -> EvidenceOutcome {
    if payloads.is_empty() {
        return EvidenceOutcome::Missing;
    }
    let outcomes: Vec<_> = payloads
        .iter()
        .map(|payload| aggregate_payload(payload, minimum))
        .collect();
    if outcomes.contains(&EvidenceOutcome::Unknown) {
        EvidenceOutcome::Unknown
    } else if outcomes.contains(&EvidenceOutcome::Unstable)
        || (outcomes.contains(&EvidenceOutcome::Satisfied)
            && outcomes.contains(&EvidenceOutcome::Failed))
    {
        EvidenceOutcome::Unstable
    } else if outcomes.contains(&EvidenceOutcome::Failed) {
        EvidenceOutcome::Failed
    } else if outcomes.contains(&EvidenceOutcome::Satisfied) {
        EvidenceOutcome::Satisfied
    } else {
        EvidenceOutcome::Missing
    }
}

fn aggregate_payload(payload: &EvidencePayload, minimum: PositiveCount) -> EvidenceOutcome {
    match payload {
        EvidencePayload::StructuralCheck(execution)
        | EvidencePayload::Test(execution)
        | EvidencePayload::Snapshot(execution) => match execution.aggregate(minimum) {
            AttemptAggregate::Satisfied => EvidenceOutcome::Satisfied,
            AttemptAggregate::Failed => EvidenceOutcome::Failed,
            AttemptAggregate::Missing => EvidenceOutcome::Missing,
            AttemptAggregate::Unstable => EvidenceOutcome::Unstable,
            AttemptAggregate::Unknown => EvidenceOutcome::Unknown,
        },
        EvidencePayload::StaticInventory { counts, .. }
        | EvidencePayload::RuntimeSnapshot { counts, .. } => aggregate_counts(*counts, minimum),
        EvidencePayload::ManualReview { outcome, .. } => match outcome {
            AttemptOutcome::Passed => EvidenceOutcome::Satisfied,
            AttemptOutcome::Failed => EvidenceOutcome::Failed,
            AttemptOutcome::TimedOut | AttemptOutcome::Cancelled | AttemptOutcome::Error => {
                EvidenceOutcome::Unknown
            }
            AttemptOutcome::Skipped | AttemptOutcome::Filtered | AttemptOutcome::Quarantined => {
                EvidenceOutcome::Missing
            }
        },
        EvidencePayload::ReleaseRecord { .. } => EvidenceOutcome::Satisfied,
    }
}

fn aggregate_counts(counts: EvidenceCounts, minimum: PositiveCount) -> EvidenceOutcome {
    if counts.failed > 0 {
        EvidenceOutcome::Failed
    } else if counts.selected == 0
        || counts.passed < minimum.get()
        || counts.skipped > 0
        || counts.filtered > 0
        || counts.quarantined > 0
    {
        EvidenceOutcome::Missing
    } else {
        EvidenceOutcome::Satisfied
    }
}

/// Independent checks still required for a structurally observed logical unit.
/// These are requirements, never verification results or successful count credit.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum LogicalObservationPrerequisite {
    /// Actual evidence trust must be independently established.
    IndependentTrust,
    /// All selected execution outcomes and retry history must be evaluated.
    ExecutionOutcomes,
    /// Structural facts must be independently checked.
    StructuralFacts,
    /// Inventory facts must be independently checked.
    InventoryFacts,
    /// Runtime facts must be independently checked.
    RuntimeFacts,
    /// Explicit review-slot outcomes must be evaluated.
    ReviewOutcomes,
    /// Required independent principals must be authorized and distinct.
    DistinctReviewPrincipals,
    /// The exact referenced record must be independently verified.
    ReferencedRecord,
}

/// Declared structural logical identity. Every unit retains independent
/// prerequisites; construction and membership never establish successful credit.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum LogicalObservationUnit {
    /// One frozen test case, irrespective of copies or retries.
    TestCase(eqm_domain::LogicalCaseId),
    /// One declared structural fact, still requiring independent checking.
    StructuralFact(eqm_domain::LogicalCaseId),
    /// One declared inventory fact, still requiring independent checking.
    InventoryFact(eqm_domain::LogicalCaseId),
    /// One selected snapshot, still requiring outcome verification.
    Snapshot(eqm_domain::LogicalCaseId),
    /// One declared runtime fact, still requiring independent checking.
    RuntimeFact(eqm_domain::LogicalCaseId),
    /// One explicit review slot, still requiring outcome/principal checks.
    ReviewSlot(eqm_domain::LogicalCaseId),
    /// One exact referenced record; aliases cannot multiply one record unit.
    ReferencedRecord(eqm_domain::Sha256Digest),
}

/// Invalid frozen expectations or bounded input yields no partial observation report.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LogicalObservationError {
    /// No pre-result expectation was supplied.
    EmptyExpectations,
    /// Complete coordinate/identity/grouped subjects or campaign differed.
    Context,
    /// An exact binding/local specification was supplied more than once.
    DuplicateSource,
    /// The coordinate mixes incompatible count units.
    CountUnit,
    /// Complete input exceeds cumulative matching work or delivery bounds.
    Matching(crate::CompleteCoverageError),
}
impl std::fmt::Display for LogicalObservationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "logical observation preparation failed: {self:?}")
    }
}
impl std::error::Error for LogicalObservationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Matching(error) => Some(error),
            Self::EmptyExpectations | Self::Context | Self::DuplicateSource | Self::CountUnit => {
                None
            }
        }
    }
}

/// Distinct structural selections and complete audit history. This grants no
/// successful count credit, outcome, independent verification or conformance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LogicalObservationReport<'a> {
    coordinate: crate::ObligationKey,
    identity: eqm_domain::ObligationDigest,
    count_unit: eqm_domain::SelectionCountUnit,
    status: crate::CoverageStatus,
    observed: std::collections::BTreeSet<eqm_domain::LogicalCaseId>,
    units: std::collections::BTreeSet<LogicalObservationUnit>,
    missing: std::collections::BTreeMap<
        eqm_domain::ExpectedSelectionSource,
        std::collections::BTreeSet<eqm_domain::LogicalCaseId>,
    >,
    matching: std::collections::BTreeMap<
        eqm_domain::ExpectedSelectionSource,
        crate::CompleteCoverageReport<'a>,
    >,
    prerequisites: std::collections::BTreeSet<LogicalObservationPrerequisite>,
    conflicts: std::collections::BTreeSet<crate::ImmutableEvidenceAttempt>,
}
impl<'a> LogicalObservationReport<'a> {
    /// Exact complete consuming coordinate.
    #[must_use]
    pub const fn coordinate(&self) -> &crate::ObligationKey {
        &self.coordinate
    }
    /// Claimed pre-result semantic identity; matching tags are not authority.
    #[must_use]
    pub const fn identity(&self) -> eqm_domain::ObligationDigest {
        self.identity
    }
    /// Declared unit, never file/transport/attempt count.
    #[must_use]
    pub const fn count_unit(&self) -> eqm_domain::SelectionCountUnit {
        self.count_unit
    }
    /// Structural expected-set completeness; Covered does not mean satisfied.
    #[must_use]
    pub const fn status(&self) -> crate::CoverageStatus {
        self.status
    }
    /// Distinct structurally observed IDs shared across specifications. Neither
    /// this set nor its length establishes successful independently checked credit.
    #[must_use]
    pub const fn observed(&self) -> &std::collections::BTreeSet<eqm_domain::LogicalCaseId> {
        &self.observed
    }
    /// Typed structural units. Distinct reference aliases yield at most one unit
    /// per exact record digest; independent prerequisites remain mandatory.
    #[must_use]
    pub const fn units(&self) -> &std::collections::BTreeSet<LogicalObservationUnit> {
        &self.units
    }
    /// Every missing source/case AND member, including empty sets for complete sources.
    #[must_use]
    pub const fn missing(
        &self,
    ) -> &std::collections::BTreeMap<
        eqm_domain::ExpectedSelectionSource,
        std::collections::BTreeSet<eqm_domain::LogicalCaseId>,
    > {
        &self.missing
    }
    /// All matching/rejected/conflicting assertions and raw delivery references.
    #[must_use]
    pub const fn matching(
        &self,
    ) -> &std::collections::BTreeMap<
        eqm_domain::ExpectedSelectionSource,
        crate::CompleteCoverageReport<'a>,
    > {
        &self.matching
    }
    /// Relevant immutable conflicts across every selected source; splitting
    /// assertions between local specifications cannot hide the conflict.
    #[must_use]
    pub const fn conflicts(&self) -> &std::collections::BTreeSet<crate::ImmutableEvidenceAttempt> {
        &self.conflicts
    }
    /// Independent prerequisites retained explicitly for each count unit.
    #[must_use]
    pub const fn prerequisites(
        &self,
    ) -> &std::collections::BTreeSet<LogicalObservationPrerequisite> {
        &self.prerequisites
    }
}

/// Derives distinct logical units from complete matching over all frozen AND
/// sources. Split reports, repeated transports and retries never multiply units.
/// Independently verified count eligibility and outcomes remain separate gates.
pub fn collect_logical_observations<'a>(
    expected: &[crate::CompleteCoverageExpectation],
    candidates: &[crate::CompleteEvidenceCandidate<'a>],
) -> Result<LogicalObservationReport<'a>, LogicalObservationError> {
    use crate::{
        CompleteCoverageError, CoverageStatus, MAX_CANDIDATE_DELIVERIES,
        MAX_COMPLETE_COVERAGE_WORK, evaluate_complete_evidence_coverage,
    };
    use eqm_domain::SelectionCountUnit;
    use std::collections::{BTreeMap, BTreeSet};
    let reference = expected
        .first()
        .ok_or(LogicalObservationError::EmptyExpectations)?;
    if expected.len() > MAX_CANDIDATE_DELIVERIES {
        return Err(LogicalObservationError::Matching(
            CompleteCoverageError::WorkLimitExceeded,
        ));
    }
    if candidates.len() > MAX_CANDIDATE_DELIVERIES {
        return Err(LogicalObservationError::Matching(
            CompleteCoverageError::Normalization(crate::CandidateNormalizationError::LimitExceeded),
        ));
    }
    let mut work = 0usize;
    for value in expected {
        work = work.saturating_add(crate::complete_coverage::complete_coverage_work(
            value, candidates,
        ));
        if work > MAX_COMPLETE_COVERAGE_WORK {
            return Err(LogicalObservationError::Matching(
                CompleteCoverageError::WorkLimitExceeded,
            ));
        }
    }
    let mut sources = BTreeSet::new();
    for value in expected {
        if value.coordinate != reference.coordinate
            || value.identity != reference.identity
            || value.structural.contract_digest != reference.structural.contract_digest
            || value.structural.policy_digest != reference.structural.policy_digest
            || value.structural.profiles != reference.structural.profiles
            || value.structural.release_record_digest != reference.structural.release_record_digest
            || (value.subjects.is_none()
                && value.structural.subject != reference.structural.subject)
            || value.subjects != reference.subjects
            || value.selection.source().plan != reference.selection.source().plan
            || value.selection.source().campaign != reference.selection.source().campaign
        {
            return Err(LogicalObservationError::Context);
        }
        if value.selection.count_unit() != reference.selection.count_unit() {
            return Err(LogicalObservationError::CountUnit);
        }
        if !sources.insert((
            &value.selection.source().binding,
            &value.selection.source().specification,
        )) {
            return Err(LogicalObservationError::DuplicateSource);
        }
    }
    let mut observed = BTreeSet::new();
    let mut missing = BTreeMap::new();
    let mut matching = BTreeMap::new();
    let mut prerequisites = BTreeSet::from([LogicalObservationPrerequisite::IndependentTrust]);
    for value in expected {
        let report = evaluate_complete_evidence_coverage(value, candidates)
            .map_err(LogicalObservationError::Matching)?;
        matching.insert(value.selection.source().clone(), report);
        match value.selection.count_unit() {
            SelectionCountUnit::LogicalTestCase | SelectionCountUnit::Snapshot => {
                prerequisites.insert(LogicalObservationPrerequisite::ExecutionOutcomes);
            }
            SelectionCountUnit::StructuralFact => {
                prerequisites.insert(LogicalObservationPrerequisite::ExecutionOutcomes);
                prerequisites.insert(LogicalObservationPrerequisite::StructuralFacts);
            }
            SelectionCountUnit::InventoryFact => {
                prerequisites.insert(LogicalObservationPrerequisite::InventoryFacts);
            }
            SelectionCountUnit::RuntimeFact => {
                prerequisites.insert(LogicalObservationPrerequisite::RuntimeFacts);
            }
            SelectionCountUnit::ReviewSlot => {
                prerequisites.insert(LogicalObservationPrerequisite::ReviewOutcomes);
                if value.selection.independent_review_principals() {
                    prerequisites.insert(LogicalObservationPrerequisite::DistinctReviewPrincipals);
                }
            }
            SelectionCountUnit::VerifiedRecord => {
                prerequisites.insert(LogicalObservationPrerequisite::ReferencedRecord);
            }
        }
    }
    let mut assertions: BTreeMap<
        crate::ImmutableEvidenceAttempt,
        BTreeSet<eqm_domain::SemanticEvidenceDigest>,
    > = BTreeMap::new();
    let mut conflicts = BTreeSet::new();
    for report in matching.values() {
        for (attempt, history) in report.normalized().attempts() {
            if report
                .conflicts()
                .contains(&(attempt.subject().clone(), attempt.lineage().clone()))
            {
                conflicts.insert(attempt.clone());
            }
            for identity in history.assertions().keys() {
                if report
                    .eligible()
                    .contains(&(*identity, attempt.lineage().clone()))
                {
                    assertions
                        .entry(attempt.clone())
                        .or_default()
                        .insert(*identity);
                }
            }
        }
    }
    conflicts.extend(
        assertions
            .into_iter()
            .filter_map(|(attempt, identities)| (identities.len() > 1).then_some(attempt)),
    );
    let mut units = BTreeSet::new();
    for value in expected {
        let report = &matching[value.selection.source()];
        let selected: BTreeSet<_> = report
            .normalized()
            .attempts()
            .iter()
            .filter(|(attempt, history)| {
                !conflicts.contains(*attempt)
                    && history.assertions().keys().any(|identity| {
                        report
                            .eligible()
                            .contains(&(*identity, attempt.lineage().clone()))
                    })
            })
            .map(|(attempt, _)| attempt.lineage().logical_case().clone())
            .collect();
        observed.extend(selected.iter().cloned());
        for (attempt, history) in report.normalized().attempts() {
            if conflicts.contains(attempt) {
                continue;
            }
            for (identity, assertion) in history.assertions() {
                if !report
                    .eligible()
                    .contains(&(*identity, attempt.lineage().clone()))
                {
                    continue;
                }
                let case = attempt.lineage().logical_case().clone();
                let unit = match assertion.result().payload() {
                    EvidencePayload::Test(_) => LogicalObservationUnit::TestCase(case),
                    EvidencePayload::StructuralCheck(_) => {
                        LogicalObservationUnit::StructuralFact(case)
                    }
                    EvidencePayload::StaticInventory { .. } => {
                        LogicalObservationUnit::InventoryFact(case)
                    }
                    EvidencePayload::Snapshot(_) => LogicalObservationUnit::Snapshot(case),
                    EvidencePayload::RuntimeSnapshot { .. } => {
                        LogicalObservationUnit::RuntimeFact(case)
                    }
                    EvidencePayload::ManualReview { .. } => {
                        LogicalObservationUnit::ReviewSlot(case)
                    }
                    EvidencePayload::ReleaseRecord {
                        release_record_digest,
                    } => LogicalObservationUnit::ReferencedRecord(*release_record_digest),
                };
                units.insert(unit);
            }
        }

        missing.insert(
            value.selection.source().clone(),
            value
                .selection
                .selected()
                .difference(&selected)
                .cloned()
                .collect::<BTreeSet<_>>(),
        );
    }
    let status = if !conflicts.is_empty()
        || matching
            .values()
            .any(|report| report.status() == CoverageStatus::Unknown)
    {
        CoverageStatus::Unknown
    } else if missing.values().any(|cases| !cases.is_empty()) {
        CoverageStatus::Missing
    } else {
        CoverageStatus::Covered
    };
    Ok(LogicalObservationReport {
        coordinate: reference.coordinate.clone(),
        identity: reference.identity,
        count_unit: reference.selection.count_unit(),
        status,
        observed,
        units,
        missing,
        matching,
        prerequisites,
        conflicts,
    })
}
