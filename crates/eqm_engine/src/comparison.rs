//! Pure v2 comparison eligibility with exact scenario-occurrence coverage.
//!
//! S039-R2 binds the pure engine check to the selected graph scenario's exact
//! occurrence set instead of trusting a submitted participant list. A coverage
//! set that omits, adds, or duplicates an occurrence is ineligible; repeated
//! same-target actors stay distinct because every participant is keyed by its
//! own [`OccurrenceId`]. The shared comparison identity
//! ([`ComparisonContextDigest`]) remains separate from each participant's exact
//! subject identity ([`SubjectManifestDigest`]).
//!
//! Eligibility is not equivalence, conformance or trust: later evaluator steps
//! own those conclusions.

use eqm_domain::{ComparisonContextDigest, OccurrenceId, SubjectManifestDigest};
use std::collections::BTreeSet;

/// Closed comparison-eligibility result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComparisonEligibility {
    /// Every expected occurrence is covered exactly once by a participant that
    /// carries the exact expected shared context and an independently matched
    /// exact subject.
    Eligible,
    /// Coverage differs from the exact expected occurrence set (missing, extra
    /// or duplicate), or a participant carries a different shared context.
    Ineligible,
    /// At least one covered occurrence's shared context or exact subject is
    /// absent, so the expected coverage cannot be proven complete.
    Incomplete,
}

/// One participant's independently derived comparison inputs.
///
/// `context` is the shared comparison identity the participant was checked
/// under; `subject` is that participant's exact, separately matched subject
/// identity. The two identity kinds are deliberately distinct types so a
/// subject digest can never be substituted for a comparison context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComparisonParticipant {
    /// The exact scenario occurrence this participant covers.
    pub occurrence: OccurrenceId,
    /// The participant's shared comparison context, absent when unestablished.
    pub context: Option<ComparisonContextDigest>,
    /// The participant's independently matched exact subject, absent when
    /// unestablished.
    pub subject: Option<SubjectManifestDigest>,
}

/// Evaluates v2 comparison eligibility against the exact expected occurrence
/// set of the selected graph scenario.
///
/// The caller derives `expected_occurrences` from the immutable scenario; it is
/// never inferred from `participants`. Duplicate, missing or extra occurrences
/// are ineligible. A covered occurrence with an absent context or subject is
/// incomplete; a covered occurrence with a different shared context is
/// ineligible. Differing subject digests across participants do not by
/// themselves change eligibility, so distinct valid builds can share one
/// comparison context.
#[must_use]
pub fn evaluate_comparison_eligibility(
    expected: ComparisonContextDigest,
    expected_occurrences: &BTreeSet<OccurrenceId>,
    participants: &[ComparisonParticipant],
) -> ComparisonEligibility {
    if expected_occurrences.is_empty() {
        return ComparisonEligibility::Incomplete;
    }
    let mut seen: BTreeSet<OccurrenceId> = BTreeSet::new();
    for participant in participants {
        if !seen.insert(participant.occurrence.clone()) {
            return ComparisonEligibility::Ineligible;
        }
    }
    if &seen != expected_occurrences {
        return ComparisonEligibility::Ineligible;
    }
    let mut complete = true;
    for participant in participants {
        match (participant.context, participant.subject) {
            (Some(context), Some(_)) => {
                if context != expected {
                    return ComparisonEligibility::Ineligible;
                }
            }
            _ => complete = false,
        }
    }
    if complete {
        ComparisonEligibility::Eligible
    } else {
        ComparisonEligibility::Incomplete
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    fn context(seed: &[u8]) -> ComparisonContextDigest {
        ComparisonContextDigest::hash_content(seed)
    }

    fn subject(seed: &[u8]) -> SubjectManifestDigest {
        SubjectManifestDigest::hash_content(seed)
    }

    fn occurrence_id(value: &str) -> Result<OccurrenceId, Box<dyn Error>> {
        Ok(OccurrenceId::new(value)?)
    }

    fn expected(values: &[&str]) -> Result<BTreeSet<OccurrenceId>, Box<dyn Error>> {
        values
            .iter()
            .map(|value| occurrence_id(value))
            .collect::<Result<BTreeSet<_>, _>>()
    }

    fn participant(
        occurrence: &str,
        context: Option<ComparisonContextDigest>,
        subject: Option<SubjectManifestDigest>,
    ) -> Result<ComparisonParticipant, Box<dyn Error>> {
        Ok(ComparisonParticipant {
            occurrence: occurrence_id(occurrence)?,
            context,
            subject,
        })
    }

    #[test]
    fn exact_coverage_with_repeated_actors_is_eligible() -> Result<(), Box<dyn Error>> {
        let shared = context(b"shared");
        // Two occurrences may reference the same underlying target; because
        // they are keyed by distinct occurrence IDs they stay distinct.
        let participants = vec![
            participant("phone_a", Some(shared), Some(subject(b"build-a")))?,
            participant("phone_b", Some(shared), Some(subject(b"build-b")))?,
        ];
        assert_eq!(
            evaluate_comparison_eligibility(
                shared,
                &expected(&["phone_a", "phone_b"])?,
                &participants
            ),
            ComparisonEligibility::Eligible
        );
        Ok(())
    }

    #[test]
    fn duplicate_missing_and_extra_occurrences_are_ineligible() -> Result<(), Box<dyn Error>> {
        let shared = context(b"shared");
        let covered = expected(&["phone_a", "phone_b"])?;

        let duplicate = vec![
            participant("phone_a", Some(shared), Some(subject(b"a")))?,
            participant("phone_a", Some(shared), Some(subject(b"b")))?,
        ];
        assert_eq!(
            evaluate_comparison_eligibility(shared, &covered, &duplicate),
            ComparisonEligibility::Ineligible
        );

        let missing = vec![participant("phone_a", Some(shared), Some(subject(b"a")))?];
        assert_eq!(
            evaluate_comparison_eligibility(shared, &covered, &missing),
            ComparisonEligibility::Ineligible
        );

        let extra = vec![
            participant("phone_a", Some(shared), Some(subject(b"a")))?,
            participant("phone_b", Some(shared), Some(subject(b"b")))?,
            participant("mailer", Some(shared), Some(subject(b"c")))?,
        ];
        assert_eq!(
            evaluate_comparison_eligibility(shared, &covered, &extra),
            ComparisonEligibility::Ineligible
        );
        Ok(())
    }

    #[test]
    fn mismatched_context_is_ineligible_and_missing_axes_are_incomplete()
    -> Result<(), Box<dyn Error>> {
        let shared = context(b"shared");
        let covered = expected(&["phone"])?;

        let mismatched = vec![participant(
            "phone",
            Some(context(b"other")),
            Some(subject(b"a")),
        )?];
        assert_eq!(
            evaluate_comparison_eligibility(shared, &covered, &mismatched),
            ComparisonEligibility::Ineligible
        );

        let missing_subject = vec![participant("phone", Some(shared), None)?];
        assert_eq!(
            evaluate_comparison_eligibility(shared, &covered, &missing_subject),
            ComparisonEligibility::Incomplete
        );

        let missing_context = vec![participant("phone", None, Some(subject(b"a")))?];
        assert_eq!(
            evaluate_comparison_eligibility(shared, &covered, &missing_context),
            ComparisonEligibility::Incomplete
        );
        Ok(())
    }

    #[test]
    fn empty_expected_coverage_cannot_prove_eligibility() -> Result<(), Box<dyn Error>> {
        let shared = context(b"shared");
        assert_eq!(
            evaluate_comparison_eligibility(shared, &BTreeSet::new(), &[]),
            ComparisonEligibility::Incomplete
        );
        Ok(())
    }
}
