//! Shared test support for EquivalenceMatrix workspace crates.
//!
//! This crate provides deterministic, test-only fixture builders: injected
//! evaluation time, exact subject and profile-selection constructors, and
//! synthetic evidence helpers. It is `publish = false` and is never a
//! production dependency, so it cannot introduce a synthetic trust factory or
//! a default global runtime into shipped code. Every builder delegates
//! validation to the same domain constructors used in production and rejects
//! malformed coordinates instead of inventing values.

#![forbid(unsafe_code)]

/// Injected evaluation time helpers.
pub mod clock;
/// Sanitized fixture-construction failures.
pub mod error;
/// Deterministic synthetic evidence helpers.
pub mod evidence;
/// Deterministic exact subject builders.
pub mod subject;
/// Normative representation and rule vectors frozen by ADR-0013.
pub mod vectors;

pub use clock::{
    FIXED_EARLIER_TIME, FIXED_EVALUATION_TIME, FIXED_LATER_TIME, earlier_time, evaluation_time,
    instant, later_time,
};
pub use error::FixtureError;
pub use evidence::{
    attempt, counts, execution_payload, failing_counts, failing_execution, manual_review,
    passing_counts, passing_execution, profile_selection, timed_out_execution, unstable_execution,
};
pub use subject::{
    FIXTURE_COMMIT, FIXTURE_REPOSITORY, SubjectBuilder, provider_subject, target_set_subject,
    target_subject,
};
pub use vectors::{
    REPRESENTATION_VECTORS, REQUIRED_VECTOR_CATEGORIES, RepresentationVector, VectorOutcome,
    vectors_for,
};

#[cfg(test)]
mod tests {
    use super::*;
    use eqm_domain::{AttemptAggregate, AttemptOutcome, PositiveCount};

    #[test]
    fn injected_time_is_fixed_and_ordered() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(evaluation_time()?, evaluation_time()?);
        assert_eq!(evaluation_time()?.to_string(), FIXED_EVALUATION_TIME);
        assert!(earlier_time()? < evaluation_time()?);
        assert!(evaluation_time()? < later_time()?);
        assert!(instant("2026-01-02T03:04:05+00:00").is_err());
        assert!(instant("not-a-time").is_err());
        Ok(())
    }

    #[test]
    fn target_subjects_are_repeatable() -> Result<(), Box<dyn std::error::Error>> {
        let first = target_subject("android")?;
        let second = target_subject("android")?;
        assert_eq!(first, second);
        assert_ne!(first, target_subject("ios")?);
        Ok(())
    }

    #[test]
    fn subject_builder_rejects_malformed_coordinates() {
        assert!(target_subject("Android App").is_err());
        assert!(target_subject("").is_err());
        assert!(provider_subject("Not A Provider").is_err());
        assert!(target_set_subject(&[]).is_err());
        assert!(target_set_subject(&["android", "android"]).is_err());
    }

    #[test]
    fn subject_builder_rejects_bad_repository_and_commit() {
        assert!(
            SubjectBuilder::new()
                .repository("git@github.com:example/eqm.git")
                .target("android")
                .is_err()
        );
        assert!(
            SubjectBuilder::new()
                .commit("deadbeef")
                .target("android")
                .is_err()
        );
    }

    #[test]
    fn target_sets_require_distinct_members() -> Result<(), Box<dyn std::error::Error>> {
        let subject = target_set_subject(&["android", "ios"])?;
        assert_eq!(
            subject.scope(),
            &eqm_domain::EvidenceScopeSubject::target_set(vec![
                "android".parse()?,
                "ios".parse()?,
            ])?
        );
        Ok(())
    }

    #[test]
    fn counts_reject_inconsistent_totals() {
        assert!(counts(2, 1, 0, 0, 0, 0).is_err());
        assert!(counts(1, 1, 1, 0, 0, 0).is_err());
        // An explicit zero-match result remains a valid, non-success observation.
        assert!(counts(0, 0, 0, 0, 0, 0).is_ok());
        assert!(passing_counts(1).is_ok());
        assert!(failing_counts(1).is_ok());
    }

    #[test]
    fn execution_payloads_classify_retry_history() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            passing_execution(1)?.aggregate(PositiveCount::ONE),
            AttemptAggregate::Satisfied
        );
        assert_eq!(
            failing_execution(1)?.aggregate(PositiveCount::ONE),
            AttemptAggregate::Failed
        );
        assert_eq!(
            unstable_execution()?.aggregate(PositiveCount::ONE),
            AttemptAggregate::Unstable
        );
        assert_eq!(
            timed_out_execution(1)?.aggregate(PositiveCount::ONE),
            AttemptAggregate::Unknown
        );
        Ok(())
    }

    #[test]
    fn profile_selections_reject_duplicates_and_empty_values() {
        assert!(profile_selection("release.default", 1, &[]).is_err());
        assert!(profile_selection("release.default", 1, &[("channel", "beta")]).is_ok());
        assert!(
            profile_selection(
                "release.default",
                1,
                &[("channel", "beta"), ("channel", "production")]
            )
            .is_err()
        );
        assert!(profile_selection("release.default", 0, &[("channel", "beta")]).is_err());
        assert!(profile_selection("release", 1, &[("channel", "beta")]).is_err());
    }

    #[test]
    fn manual_review_is_limited_to_pass_or_fail() {
        assert!(manual_review(AttemptOutcome::Passed, "owner://team/reviewer").is_ok());
        assert!(manual_review(AttemptOutcome::Skipped, "owner://team/reviewer").is_err());
        assert!(manual_review(AttemptOutcome::Passed, "reviewer").is_err());
    }
}
