//! Attempt, count, retry, and terminal-outcome aggregation fixtures.

use eqm_domain::{
    AttemptOutcome, EvidenceAttempt, EvidenceCounts, EvidencePayload, ExecutionPayload,
    PositiveCount, Sha256Digest,
};
use eqm_engine::{EvidenceOutcome, aggregate_evidence_outcomes};
use std::error::Error;

fn execution(
    outcomes: &[AttemptOutcome],
    counts: EvidenceCounts,
) -> Result<EvidencePayload, Box<dyn Error>> {
    let start = "2026-08-07T12:00:00Z".parse()?;
    let end = "2026-08-07T12:00:01Z".parse()?;
    let attempts = outcomes
        .iter()
        .enumerate()
        .map(|(index, outcome)| {
            Ok(EvidenceAttempt::new(
                PositiveCount::new(index as u64 + 1)?,
                *outcome,
                start,
                end,
                None,
            )?)
        })
        .collect::<Result<_, Box<dyn Error>>>()?;
    Ok(EvidencePayload::Test(ExecutionPayload::new(
        attempts, counts, start, end,
    )?))
}

#[test]
fn terminal_retry_and_count_table_is_complete() -> Result<(), Box<dyn Error>> {
    let minimum = PositiveCount::new(2)?;
    let cases = [
        (
            execution(
                &[AttemptOutcome::Passed],
                EvidenceCounts::new(2, 2, 0, 0, 0, 0)?,
            )?,
            EvidenceOutcome::Satisfied,
        ),
        (
            execution(
                &[AttemptOutcome::Failed],
                EvidenceCounts::new(1, 0, 1, 0, 0, 0)?,
            )?,
            EvidenceOutcome::Failed,
        ),
        (
            execution(
                &[AttemptOutcome::Failed, AttemptOutcome::Passed],
                EvidenceCounts::new(2, 2, 0, 0, 0, 0)?,
            )?,
            EvidenceOutcome::Unstable,
        ),
        (
            execution(
                &[AttemptOutcome::TimedOut],
                EvidenceCounts::new(1, 0, 1, 0, 0, 0)?,
            )?,
            EvidenceOutcome::Unknown,
        ),
        (
            execution(
                &[AttemptOutcome::Cancelled],
                EvidenceCounts::new(1, 0, 1, 0, 0, 0)?,
            )?,
            EvidenceOutcome::Unknown,
        ),
        (
            execution(
                &[AttemptOutcome::Error],
                EvidenceCounts::new(1, 0, 1, 0, 0, 0)?,
            )?,
            EvidenceOutcome::Unknown,
        ),
        (
            execution(
                &[AttemptOutcome::Passed],
                EvidenceCounts::new(1, 1, 0, 0, 0, 0)?,
            )?,
            EvidenceOutcome::Missing,
        ),
    ];
    for (payload, expected) in cases {
        assert_eq!(aggregate_evidence_outcomes(&[&payload], minimum), expected);
    }
    Ok(())
}

#[test]
fn zero_skip_filter_quarantine_and_cross_result_conflict_are_visible() -> Result<(), Box<dyn Error>>
{
    let minimum = PositiveCount::ONE;
    assert_eq!(
        aggregate_evidence_outcomes(&[], minimum),
        EvidenceOutcome::Missing
    );
    for counts in [
        EvidenceCounts::new(0, 0, 0, 0, 0, 0)?,
        EvidenceCounts::new(1, 0, 0, 1, 0, 0)?,
        EvidenceCounts::new(1, 0, 0, 0, 1, 0)?,
        EvidenceCounts::new(1, 0, 0, 0, 0, 1)?,
    ] {
        let payload = EvidencePayload::StaticInventory {
            inventory_digest: Sha256Digest::from_bytes([1; 32]),
            counts,
        };
        assert_eq!(
            aggregate_evidence_outcomes(&[&payload], minimum),
            EvidenceOutcome::Missing
        );
    }
    let passed = execution(
        &[AttemptOutcome::Passed],
        EvidenceCounts::new(1, 1, 0, 0, 0, 0)?,
    )?;
    let failed = execution(
        &[AttemptOutcome::Failed],
        EvidenceCounts::new(1, 0, 1, 0, 0, 0)?,
    )?;
    assert_eq!(
        aggregate_evidence_outcomes(&[&passed, &failed], minimum),
        EvidenceOutcome::Unstable
    );
    Ok(())
}
