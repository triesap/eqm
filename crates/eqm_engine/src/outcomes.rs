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
