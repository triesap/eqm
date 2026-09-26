//! Deterministic synthetic evidence helpers for fixtures.
//!
//! These helpers build validated domain evidence values for regression tests.
//! They are test-only scaffolding: they never mint a trusted envelope, never
//! raise effective trust, and every malformed coordinate is rejected by the
//! same domain constructors used in production.

use crate::clock;
use crate::error::FixtureError;
use eqm_domain::{
    AttemptOutcome, DimensionId, EvidenceAttempt, EvidenceCounts, EvidencePayload,
    ExecutionPayload, OwnerRef, PositiveCount, ProfileId, ProfileSelection, Revision,
    SymbolicValueId, UtcInstant,
};

/// Builds normalized counts and rejects totals that do not add up.
pub fn counts(
    selected: u64,
    passed: u64,
    failed: u64,
    skipped: u64,
    filtered: u64,
    quarantined: u64,
) -> Result<EvidenceCounts, FixtureError> {
    EvidenceCounts::new(selected, passed, failed, skipped, filtered, quarantined)
        .map_err(FixtureError::new)
}

/// Builds counts where every selected item passed.
pub fn passing_counts(selected: u64) -> Result<EvidenceCounts, FixtureError> {
    counts(selected, selected, 0, 0, 0, 0)
}

/// Builds counts where every selected item failed.
pub fn failing_counts(selected: u64) -> Result<EvidenceCounts, FixtureError> {
    counts(selected, 0, selected, 0, 0, 0)
}

/// Builds one immutable numbered attempt inside the fixed time window.
pub fn attempt(number: u64, outcome: AttemptOutcome) -> Result<EvidenceAttempt, FixtureError> {
    let started = clock::earlier_time().map_err(FixtureError::new)?;
    let finished = clock::evaluation_time().map_err(FixtureError::new)?;
    EvidenceAttempt::new(
        PositiveCount::new(number).map_err(FixtureError::new)?,
        outcome,
        started,
        finished,
        None,
    )
    .map_err(FixtureError::new)
}

/// Builds an execution payload with complete retry history and counts.
///
/// Attempts share the fixed fixture window; their numbers are consecutive from
/// one. The payload deliberately keeps every attempt so retry history cannot be
/// erased by a caller.
pub fn execution_payload(
    outcomes: &[AttemptOutcome],
    counts: EvidenceCounts,
) -> Result<ExecutionPayload, FixtureError> {
    let started: UtcInstant = clock::earlier_time().map_err(FixtureError::new)?;
    let finished: UtcInstant = clock::evaluation_time().map_err(FixtureError::new)?;
    let mut attempts = Vec::with_capacity(outcomes.len());
    for (index, outcome) in outcomes.iter().enumerate() {
        let number = PositiveCount::new(index as u64 + 1).map_err(FixtureError::new)?;
        attempts.push(
            EvidenceAttempt::new(number, *outcome, started, finished, None)
                .map_err(FixtureError::new)?,
        );
    }
    ExecutionPayload::new(attempts, counts, started, finished).map_err(FixtureError::new)
}

/// Builds a satisfied single-attempt execution payload.
pub fn passing_execution(selected: u64) -> Result<ExecutionPayload, FixtureError> {
    execution_payload(&[AttemptOutcome::Passed], passing_counts(selected)?)
}

/// Builds a failed single-attempt execution payload.
pub fn failing_execution(selected: u64) -> Result<ExecutionPayload, FixtureError> {
    execution_payload(&[AttemptOutcome::Failed], failing_counts(selected)?)
}

/// Builds a payload whose history contains both a pass and a failure.
pub fn unstable_execution() -> Result<ExecutionPayload, FixtureError> {
    execution_payload(
        &[AttemptOutcome::Passed, AttemptOutcome::Failed],
        counts(2, 1, 1, 0, 0, 0)?,
    )
}

/// Builds a payload whose terminal outcome is a host timeout.
pub fn timed_out_execution(selected: u64) -> Result<ExecutionPayload, FixtureError> {
    execution_payload(&[AttemptOutcome::TimedOut], passing_counts(selected)?)
}

/// Builds one exact versioned profile selection.
pub fn profile_selection(
    profile: &str,
    revision: u64,
    values: &[(&str, &str)],
) -> Result<ProfileSelection, FixtureError> {
    let profile: ProfileId = profile.parse().map_err(FixtureError::new)?;
    let revision = Revision::new(revision).map_err(FixtureError::new)?;
    let mut selected = Vec::with_capacity(values.len());
    for (dimension, value) in values {
        let dimension: DimensionId = dimension.parse().map_err(FixtureError::new)?;
        let value: SymbolicValueId = value.parse().map_err(FixtureError::new)?;
        selected.push((dimension, value));
    }
    ProfileSelection::new(profile, revision, selected).map_err(FixtureError::new)
}

/// Builds a manual-review payload limited to a passed or failed outcome.
pub fn manual_review(
    outcome: AttemptOutcome,
    reviewer: &str,
) -> Result<EvidencePayload, FixtureError> {
    let reviewer: OwnerRef = reviewer.parse().map_err(FixtureError::new)?;
    EvidencePayload::manual_review(outcome, reviewer, None).map_err(FixtureError::new)
}
