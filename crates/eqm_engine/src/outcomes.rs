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

/// Structural count capacity before independent outcome/trust verification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LogicalCountConstraintStatus {
    /// Complete observed structural units meet the actual composed minimum.
    /// Independent prerequisites are still required; this is never Satisfied.
    CapacityMet,
    /// A required AND member or distinct unit needed by the minimum is missing.
    Missing,
    /// Relevant immutable conflicts prevent a count conclusion.
    Unknown,
}

/// Count requests that do not belong to the actual frozen coordinate reject.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LogicalCountConstraintError {
    /// No independently declared expectation was supplied.
    EmptyExpectations,
    /// Expectations exceed the declared bound.
    Bound,
    /// The exact derived coordinate is unavailable.
    Coordinate,
    /// The complete frozen selection was omitted, changed or duplicated.
    FrozenSelection,
    /// Collection against these exact validated expectations failed.
    Observations(LogicalObservationError),
}
impl std::fmt::Display for LogicalCountConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "logical count constraint rejected: {self:?}")
    }
}
impl std::error::Error for LogicalCountConstraintError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Observations(error) => Some(error),
            _ => None,
        }
    }
}

/// Pure count assessment retaining every observation and verification prerequisite.
/// No construction or status establishes independently eligible successful credit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LogicalCountConstraintReport<'a> {
    observations: LogicalObservationReport<'a>,
    policy_minimum: PositiveCount,
    specification_minima:
        std::collections::BTreeMap<eqm_domain::ExpectedSelectionSource, PositiveCount>,
    effective_minimum: PositiveCount,
    capacity: usize,
    status: LogicalCountConstraintStatus,
}
impl<'a> LogicalCountConstraintReport<'a> {
    /// Complete structural observation and audit report, including prerequisites.
    #[must_use]
    pub const fn observations(&self) -> &LogicalObservationReport<'a> {
        &self.observations
    }
    /// Minimum composed from rules matching this exact facet/coordinate.
    #[must_use]
    pub const fn policy_minimum(&self) -> PositiveCount {
        self.policy_minimum
    }
    /// Actual source acquisition minima; non-countable kinds contribute one.
    #[must_use]
    pub const fn specification_minima(
        &self,
    ) -> &std::collections::BTreeMap<eqm_domain::ExpectedSelectionSource, PositiveCount> {
        &self.specification_minima
    }
    /// Strongest relevant minimum, never a supplied weaker threshold.
    #[must_use]
    pub const fn effective_minimum(&self) -> PositiveCount {
        self.effective_minimum
    }
    /// Distinct structural units, not independently eligible successful credit.
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }
    /// Explicit capacity/Missing/Unknown conclusion, never a facet outcome.
    #[must_use]
    pub const fn status(&self) -> LogicalCountConstraintStatus {
        self.status
    }
}

/// Validates the complete frozen request, collects observations with those exact
/// expectations, and applies the actual universe's composed minimum.
/// A report from other structural/grouped expectations cannot be supplied.
/// The application additionally binds claimed identity to the opaque S054 map.
/// Outcomes, trust, principal/fact/reference checks and protected consumption
/// remain required. No arbitrary threshold or acquisition total is accepted.
pub fn assess_logical_count_constraints<'a>(
    universe: &crate::RequiredUniverse,
    expected: &[crate::CompleteCoverageExpectation],
    candidates: &[crate::CompleteEvidenceCandidate<'a>],
) -> Result<LogicalCountConstraintReport<'a>, LogicalCountConstraintError> {
    use std::collections::BTreeMap;
    if candidates.len() > crate::MAX_CANDIDATE_DELIVERIES {
        return Err(LogicalCountConstraintError::Observations(
            LogicalObservationError::Matching(crate::CompleteCoverageError::Normalization(
                crate::CandidateNormalizationError::LimitExceeded,
            )),
        ));
    }
    validate_logical_count_context(universe, expected)?;
    let observations = collect_logical_observations(expected, candidates)
        .map_err(LogicalCountConstraintError::Observations)?;
    let first = &expected[0];
    let obligation = universe
        .obligations()
        .get(&first.coordinate)
        .ok_or(LogicalCountConstraintError::Coordinate)?;
    let frozen = universe
        .expected_selections()
        .and_then(|map| map.get(&first.coordinate))
        .ok_or(LogicalCountConstraintError::FrozenSelection)?;
    let policy_minimum = obligation.strength.minimum_count;
    let mut effective_minimum = policy_minimum;
    let mut specification_minima = BTreeMap::new();
    for value in frozen {
        let source = value.source();
        let specification = universe
            .context
            .graph
            .bindings()
            .get(&source.binding)
            .and_then(|binding| binding.evidence().get(&source.specification))
            .ok_or(LogicalCountConstraintError::FrozenSelection)?;
        let minimum = specification.minimum_count().unwrap_or(PositiveCount::ONE);
        effective_minimum = effective_minimum.max(minimum);
        specification_minima.insert(source.clone(), minimum);
    }
    let capacity = if observations.count_unit == eqm_domain::SelectionCountUnit::VerifiedRecord {
        observations.units.len().min(1)
    } else {
        observations.units.len()
    };
    let status = match observations.status {
        crate::CoverageStatus::Unknown => LogicalCountConstraintStatus::Unknown,
        crate::CoverageStatus::Missing => LogicalCountConstraintStatus::Missing,
        crate::CoverageStatus::Covered if (capacity as u64) < effective_minimum.get() => {
            LogicalCountConstraintStatus::Missing
        }
        crate::CoverageStatus::Covered => LogicalCountConstraintStatus::CapacityMet,
    };
    Ok(LogicalCountConstraintReport {
        observations,
        policy_minimum,
        specification_minima,
        effective_minimum,
        capacity,
        status,
    })
}

fn frozen_selection_work(value: &eqm_domain::ExpectedSelectionManifest) -> usize {
    let mut work = value
        .required()
        .len()
        .saturating_add(value.selected().len());
    for group in value.alternatives().values() {
        if work > crate::MAX_COMPLETE_COVERAGE_WORK {
            break;
        }
        work = work.saturating_add(1);
        for members in group.options().values() {
            if work > crate::MAX_COMPLETE_COVERAGE_WORK {
                break;
            }
            work = work.saturating_add(1).saturating_add(members.len());
        }
    }
    work
}

/// Checks the complete frozen count request against actual private derivation.
/// Work includes all alternative definitions before manifest equality/indexing;
/// validation grants no observation, trust, outcome or protected authority.
pub fn validate_logical_count_context(
    universe: &crate::RequiredUniverse,
    expected: &[crate::CompleteCoverageExpectation],
) -> Result<(), LogicalCountConstraintError> {
    use std::collections::{BTreeMap, BTreeSet};
    let first = expected
        .first()
        .ok_or(LogicalCountConstraintError::EmptyExpectations)?;
    if expected.len() > crate::MAX_CANDIDATE_DELIVERIES {
        return Err(LogicalCountConstraintError::Bound);
    }
    if !universe.obligations().contains_key(&first.coordinate) {
        return Err(LogicalCountConstraintError::Coordinate);
    }
    let frozen = universe
        .expected_selections()
        .and_then(|map| map.get(&first.coordinate))
        .ok_or(LogicalCountConstraintError::FrozenSelection)?;
    if frozen.is_empty() || frozen.len() != expected.len() {
        return Err(LogicalCountConstraintError::FrozenSelection);
    }
    let mut work = 0usize;
    for value in expected {
        work = work
            .saturating_add(crate::complete_coverage::complete_coverage_work(value, &[]))
            .saturating_add(frozen_selection_work(&value.selection));
        if work > crate::MAX_COMPLETE_COVERAGE_WORK {
            return Err(LogicalCountConstraintError::Bound);
        }
    }
    for value in frozen {
        work = work.saturating_add(frozen_selection_work(value));
        if work > crate::MAX_COMPLETE_COVERAGE_WORK {
            return Err(LogicalCountConstraintError::Bound);
        }
    }
    let frozen_index = frozen
        .iter()
        .map(|value| (value.source(), value))
        .collect::<BTreeMap<_, _>>();
    let mut sources = BTreeSet::new();
    for value in expected {
        if value.coordinate != first.coordinate
            || value.identity != first.identity
            || !sources.insert(value.selection.source())
            || frozen_index.get(value.selection.source()).copied() != Some(&value.selection)
        {
            return Err(LogicalCountConstraintError::FrozenSelection);
        }
    }
    Ok(())
}

/// Terminal history interpretation only; Passed never means facet satisfaction.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CampaignRetryState {
    /// Every matched executable terminal attempt passed.
    Passed,
    /// A required case failed without a passing retry.
    Failed,
    /// Required cases or terminal selections were absent/non-executed.
    Missing,
    /// One exact subject/case contains failing and passing retries.
    Unstable,
    /// Execution uncertainty or an immutable conflict prevents a conclusion.
    Unknown,
    /// Non-executable kind evaluation remains an independent prerequisite.
    PendingKindEvaluation,
}

/// Every contributing terminal or incomplete-history reason, regardless of precedence.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CampaignRetryReason {
    /// Actual matched terminal outcome, never independently verified trust.
    Terminal(AttemptOutcome),
    /// Contradictory semantic assertions for the same immutable attempt.
    ImmutableConflict,
    /// A selected source/case lacks an unconflicted observation.
    MissingSelectedCase(
        eqm_domain::ExpectedSelectionSource,
        eqm_domain::LogicalCaseId,
    ),
    /// This kind's independent outcome/fact/reference evaluation remains pending.
    PendingKind(eqm_domain::EvidenceKind),
}

/// Exact full-subject logical retry group within the validated selected campaign.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CampaignRetryCase {
    /// Full repository/source/build/artifact/configuration/scope subject.
    pub subject: eqm_domain::EvidenceSubject,
    /// Explicit selected logical case, never an attempt or transport identifier.
    pub logical_case: eqm_domain::LogicalCaseId,
}

/// All matched assertions for one exact subject/case, including contradictions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CampaignCaseRetryHistory<'a> {
    attempts: std::collections::BTreeMap<
        eqm_domain::EvidenceLineage,
        std::collections::BTreeMap<
            eqm_domain::SemanticEvidenceDigest,
            &'a eqm_domain::EvidenceResult,
        >,
    >,
    state: CampaignRetryState,
    reasons: std::collections::BTreeSet<CampaignRetryReason>,
}
impl<'a> CampaignCaseRetryHistory<'a> {
    /// Complete immutable attempt/semantic assertions; raw references are in the audit report.
    #[must_use]
    pub const fn attempts(
        &self,
    ) -> &std::collections::BTreeMap<
        eqm_domain::EvidenceLineage,
        std::collections::BTreeMap<
            eqm_domain::SemanticEvidenceDigest,
            &'a eqm_domain::EvidenceResult,
        >,
    > {
        &self.attempts
    }
    /// Terminal history only, never independently eligible successful count credit.
    #[must_use]
    pub const fn state(&self) -> CampaignRetryState {
        self.state
    }
    /// All terminal/conflict/pending reasons, without summary precedence erasure.
    #[must_use]
    pub const fn reasons(&self) -> &std::collections::BTreeSet<CampaignRetryReason> {
        &self.reasons
    }
}

/// Immutable internally collected retry history and complete structural count audit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CampaignRetryReport<'a> {
    constraints: LogicalCountConstraintReport<'a>,
    histories: std::collections::BTreeMap<CampaignRetryCase, CampaignCaseRetryHistory<'a>>,
    source_cases: std::collections::BTreeMap<
        (
            eqm_domain::ExpectedSelectionSource,
            eqm_domain::LogicalCaseId,
        ),
        std::collections::BTreeSet<CampaignRetryCase>,
    >,
    state: CampaignRetryState,
    reasons: std::collections::BTreeSet<CampaignRetryReason>,
}
impl<'a> CampaignRetryReport<'a> {
    /// Full structural counts, missing AND members, conflicts and independent prerequisites.
    #[must_use]
    pub const fn constraints(&self) -> &LogicalCountConstraintReport<'a> {
        &self.constraints
    }
    /// Every exact matched retry group; rejected assertions remain separately in audit.
    #[must_use]
    pub const fn histories(
        &self,
    ) -> &std::collections::BTreeMap<CampaignRetryCase, CampaignCaseRetryHistory<'a>> {
        &self.histories
    }
    /// Every required source/case and its matched exact subjects; empty sets remain visible.
    #[must_use]
    pub const fn source_cases(
        &self,
    ) -> &std::collections::BTreeMap<
        (
            eqm_domain::ExpectedSelectionSource,
            eqm_domain::LogicalCaseId,
        ),
        std::collections::BTreeSet<CampaignRetryCase>,
    > {
        &self.source_cases
    }
    /// Conjunctive terminal-history summary, not count eligibility or conformance.
    #[must_use]
    pub const fn state(&self) -> CampaignRetryState {
        self.state
    }
    /// Every contributing history and required-selection reason.
    #[must_use]
    pub const fn reasons(&self) -> &std::collections::BTreeSet<CampaignRetryReason> {
        &self.reasons
    }
}

/// Invalid frozen count context or excessive history produces no partial report.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CampaignRetryError {
    /// Same exact internally collected S063 context failed validation.
    Constraints(LogicalCountConstraintError),
    /// Complete retained history/outcome traversal exceeds its cumulative bound.
    WorkLimitExceeded,
}
impl std::fmt::Display for CampaignRetryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "campaign retry history rejected: {self:?}")
    }
}
impl std::error::Error for CampaignRetryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Constraints(error) => Some(error),
            Self::WorkLimitExceeded => None,
        }
    }
}

fn retry_summary(states: impl IntoIterator<Item = CampaignRetryState>) -> CampaignRetryState {
    use CampaignRetryState::{Failed, Missing, Passed, PendingKindEvaluation, Unknown, Unstable};
    let states: std::collections::BTreeSet<_> = states.into_iter().collect();
    for state in [Unknown, Unstable, Failed, Missing, PendingKindEvaluation] {
        if states.contains(&state) {
            return state;
        }
    }
    if states.is_empty() { Missing } else { Passed }
}

/// Preserves selected campaign retries under the same validated frozen expectations.
/// The application also binds actual opaque pre-result identities. Trust, complete
/// successful count eligibility, non-executable kind checks and protected consumption
/// remain independent. No timestamp, supplied history or campaign fallback is accepted.
pub fn assess_campaign_retry_history<'a>(
    universe: &crate::RequiredUniverse,
    expected: &[crate::CompleteCoverageExpectation],
    candidates: &[crate::CompleteEvidenceCandidate<'a>],
) -> Result<CampaignRetryReport<'a>, CampaignRetryError> {
    use std::collections::{BTreeMap, BTreeSet};
    if candidates.len() > crate::MAX_CANDIDATE_DELIVERIES {
        return Err(CampaignRetryError::Constraints(
            LogicalCountConstraintError::Observations(LogicalObservationError::Matching(
                crate::CompleteCoverageError::Normalization(
                    crate::CandidateNormalizationError::LimitExceeded,
                ),
            )),
        ));
    }
    validate_logical_count_context(universe, expected).map_err(CampaignRetryError::Constraints)?;
    // Charge complete incoming payload comparisons before normalization; even
    // rejected/duplicate delivery can require full semantic record comparisons.
    let mut work = 0usize;
    for candidate in candidates {
        let payload_work = match candidate.delivery.result.payload() {
            EvidencePayload::Test(p)
            | EvidencePayload::Snapshot(p)
            | EvidencePayload::StructuralCheck(p) => p.attempts().len(),
            EvidencePayload::StaticInventory { .. }
            | EvidencePayload::RuntimeSnapshot { .. }
            | EvidencePayload::ManualReview { .. }
            | EvidencePayload::ReleaseRecord { .. } => 1,
        };
        work = work.saturating_add(
            payload_work
                .saturating_add(1)
                .saturating_mul(expected.len()),
        );
        if work > crate::MAX_COMPLETE_COVERAGE_WORK {
            return Err(CampaignRetryError::WorkLimitExceeded);
        }
    }
    let constraints = assess_logical_count_constraints(universe, expected, candidates)
        .map_err(CampaignRetryError::Constraints)?;
    let mut histories: BTreeMap<CampaignRetryCase, CampaignCaseRetryHistory<'a>> = BTreeMap::new();
    let mut source_cases = BTreeMap::new();
    for value in expected {
        let source = value.selection.source();
        for case in value.selection.selected() {
            source_cases.insert((source.clone(), case.clone()), BTreeSet::new());
        }
        let report = &constraints.observations.matching[source];
        for (attempt, history) in report.normalized().attempts() {
            let Some(identities) = report
                .matched()
                .get(&(attempt.subject().clone(), attempt.lineage().clone()))
            else {
                continue;
            };
            let key = CampaignRetryCase {
                subject: attempt.subject().clone(),
                logical_case: attempt.lineage().logical_case().clone(),
            };
            source_cases
                .entry((source.clone(), key.logical_case.clone()))
                .or_default()
                .insert(key.clone());
            let group = histories
                .entry(key)
                .or_insert_with(|| CampaignCaseRetryHistory {
                    attempts: BTreeMap::new(),
                    state: CampaignRetryState::Missing,
                    reasons: BTreeSet::new(),
                });
            if constraints.observations.conflicts.contains(attempt) {
                group.reasons.insert(CampaignRetryReason::ImmutableConflict);
            }
            for identity in identities {
                let assertion = &history.assertions()[identity];
                work = work.saturating_add(1);
                let terminal_work = match assertion.result().payload() {
                    EvidencePayload::Test(p)
                    | EvidencePayload::Snapshot(p)
                    | EvidencePayload::StructuralCheck(p) => p.attempts().len(),
                    EvidencePayload::StaticInventory { .. }
                    | EvidencePayload::RuntimeSnapshot { .. }
                    | EvidencePayload::ManualReview { .. }
                    | EvidencePayload::ReleaseRecord { .. } => 1,
                };
                work = work.saturating_add(terminal_work);
                if work > crate::MAX_COMPLETE_COVERAGE_WORK {
                    return Err(CampaignRetryError::WorkLimitExceeded);
                }
                group
                    .attempts
                    .entry(attempt.lineage().clone())
                    .or_default()
                    .insert(*identity, assertion.result());
            }
        }
    }
    for group in histories.values_mut() {
        let mut terminal = Vec::new();
        let mut pending = false;
        for result in group.attempts.values().flat_map(BTreeMap::values) {
            match result.payload() {
                EvidencePayload::Test(p)
                | EvidencePayload::Snapshot(p)
                | EvidencePayload::StructuralCheck(p) => {
                    for attempt in p.attempts() {
                        terminal.push(attempt.outcome());
                        group
                            .reasons
                            .insert(CampaignRetryReason::Terminal(attempt.outcome()));
                    }
                }
                EvidencePayload::StaticInventory { .. }
                | EvidencePayload::RuntimeSnapshot { .. }
                | EvidencePayload::ManualReview { .. }
                | EvidencePayload::ReleaseRecord { .. } => {
                    pending = true;
                    group
                        .reasons
                        .insert(CampaignRetryReason::PendingKind(result.kind()));
                }
            }
        }
        group.state = if group
            .reasons
            .contains(&CampaignRetryReason::ImmutableConflict)
        {
            CampaignRetryState::Unknown
        } else if pending {
            CampaignRetryState::PendingKindEvaluation
        } else {
            match eqm_domain::aggregate_terminal_attempts(terminal) {
                AttemptAggregate::Satisfied => CampaignRetryState::Passed,
                AttemptAggregate::Failed => CampaignRetryState::Failed,
                AttemptAggregate::Missing => CampaignRetryState::Missing,
                AttemptAggregate::Unstable => CampaignRetryState::Unstable,
                AttemptAggregate::Unknown => CampaignRetryState::Unknown,
            }
        };
    }
    let mut reasons: BTreeSet<_> = histories
        .values()
        .flat_map(|h| h.reasons.iter().cloned())
        .collect();
    for (source, cases) in &constraints.observations.missing {
        for case in cases {
            reasons.insert(CampaignRetryReason::MissingSelectedCase(
                source.clone(),
                case.clone(),
            ));
        }
    }
    let mut states: Vec<_> = histories.values().map(|h| h.state).collect();
    if reasons
        .iter()
        .any(|r| matches!(r, CampaignRetryReason::MissingSelectedCase(..)))
    {
        states.push(CampaignRetryState::Missing);
    }
    if !constraints.observations.conflicts.is_empty() {
        states.push(CampaignRetryState::Unknown);
    }
    let state = retry_summary(states);
    Ok(CampaignRetryReport {
        constraints,
        histories,
        source_cases,
        state,
        reasons,
    })
}
