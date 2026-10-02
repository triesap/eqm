//! S027: shared record-history corpus at the real domain boundary.

use eqm_domain::{
    RecordDescriptor, RecordError, RecordId, RecordKind, RecordRef, validate_records,
};
use eqm_test_support::{RecordHistoryOutcome, record_history_vectors};
use std::error::Error;

#[test]
fn shared_record_history_corpus_matches_its_declared_outcomes() -> Result<(), Box<dyn Error>> {
    for vector in record_history_vectors()? {
        let result = validate_records(vector.records.clone());
        match vector.outcome {
            RecordHistoryOutcome::Accept => {
                assert!(result.is_ok(), "{} should validate", vector.id);
            }
            RecordHistoryOutcome::Future => {
                assert_eq!(
                    result.err(),
                    Some(RecordError::FutureReference),
                    "{}",
                    vector.id
                );
            }
            RecordHistoryOutcome::Duplicate => {
                assert_eq!(
                    result.err(),
                    Some(RecordError::DuplicateRecord),
                    "{}",
                    vector.id
                );
            }
            RecordHistoryOutcome::Missing => {
                assert_eq!(
                    result.err(),
                    Some(RecordError::MissingReference),
                    "{}",
                    vector.id
                );
            }
            RecordHistoryOutcome::Cyclic => {
                assert_eq!(
                    result.err(),
                    Some(RecordError::CyclicReference),
                    "{}",
                    vector.id
                );
            }
            RecordHistoryOutcome::SelfReference => {
                assert_eq!(
                    result.err(),
                    Some(RecordError::SelfReference),
                    "{}",
                    vector.id
                );
            }
            RecordHistoryOutcome::DuplicateReference => {
                assert_eq!(
                    result.err(),
                    Some(RecordError::DuplicateReference),
                    "{}",
                    vector.id
                );
            }
        }
    }
    Ok(())
}

#[test]
fn record_history_corpus_is_deterministic() -> Result<(), Box<dyn Error>> {
    assert_eq!(record_history_vectors()?, record_history_vectors()?);
    Ok(())
}

#[test]
fn constructor_controls_reject_self_and_duplicate_references() -> Result<(), Box<dyn Error>> {
    let plan = RecordId::new("plan.release")?;
    assert_eq!(
        RecordDescriptor::new(
            RecordKind::Plan,
            plan.clone(),
            vec![RecordRef::new(RecordKind::Plan, plan.clone())],
        ),
        Err(RecordError::SelfReference)
    );
    let authority = RecordId::new("authority.baseline")?;
    let reference = RecordRef::new(RecordKind::Authority, authority);
    assert_eq!(
        RecordDescriptor::new(
            RecordKind::Campaign,
            RecordId::new("campaign.signup")?,
            vec![reference.clone(), reference],
        ),
        Err(RecordError::DuplicateReference)
    );
    Ok(())
}
