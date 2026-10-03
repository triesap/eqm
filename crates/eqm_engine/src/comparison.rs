//! Pure v2 comparison eligibility.
//!
//! S039-R1 separates the shared comparison identity
//! ([`ComparisonContextDigest`]) from each participant's exact subject identity
//! ([`SubjectManifestDigest`]). Two independently matched, genuinely different
//! source/build/artifact assignments remain eligible for comparison when every
//! participant carries the exact expected shared context. Missing context or
//! subject coverage stays incomplete; a different shared context is
//! ineligible. Eligibility is not equivalence, conformance or trust: later
//! evaluator steps own those conclusions.

use eqm_domain::{ComparisonContextDigest, OccurrenceId, SubjectManifestDigest};

/// Closed comparison-eligibility result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComparisonEligibility {
    /// Every participant carries the exact expected shared context and an
    /// independently matched exact subject.
    Eligible,
    /// At least one participant carries a different shared context.
    Ineligible,
    /// Coverage or shared context is absent for at least one participant.
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

/// Evaluates v2 comparison eligibility for one exact shared context.
///
/// A participant with a different context makes the set ineligible; an absent
/// context or subject makes it incomplete. Differing subject digests across
/// participants do not by themselves change eligibility.
#[must_use]
pub fn evaluate_comparison_eligibility(
    expected: ComparisonContextDigest,
    participants: &[ComparisonParticipant],
) -> ComparisonEligibility {
    if participants.is_empty() {
        return ComparisonEligibility::Incomplete;
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

    fn participant(
        occurrence: &str,
        context: Option<ComparisonContextDigest>,
        subject: Option<SubjectManifestDigest>,
    ) -> Result<ComparisonParticipant, Box<dyn Error>> {
        Ok(ComparisonParticipant {
            occurrence: OccurrenceId::new(occurrence)?,
            context,
            subject,
        })
    }

    #[test]
    fn distinct_subjects_share_one_context_but_missing_or_mismatch_is_not_eligible()
    -> Result<(), Box<dyn Error>> {
        let shared = context(b"shared");
        let two = vec![
            participant("phone", Some(shared), Some(subject(b"build-a")))?,
            participant("mailer", Some(shared), Some(subject(b"build-b")))?,
        ];
        // Genuinely different exact subject identities still share one context.
        assert_eq!(
            evaluate_comparison_eligibility(shared, &two),
            ComparisonEligibility::Eligible
        );

        // A mismatched shared context is ineligible, never an unchecked cast.
        let mismatched = vec![
            participant("phone", Some(shared), Some(subject(b"build-a")))?,
            participant("mailer", Some(context(b"other")), Some(subject(b"build-b")))?,
        ];
        assert_eq!(
            evaluate_comparison_eligibility(shared, &mismatched),
            ComparisonEligibility::Ineligible
        );

        // Missing exact subject coverage is incomplete.
        let missing_subject = vec![
            participant("phone", Some(shared), Some(subject(b"build-a")))?,
            participant("mailer", Some(shared), None)?,
        ];
        assert_eq!(
            evaluate_comparison_eligibility(shared, &missing_subject),
            ComparisonEligibility::Incomplete
        );

        // Missing shared context is incomplete.
        let missing_context = vec![participant("phone", None, Some(subject(b"build-a")))?];
        assert_eq!(
            evaluate_comparison_eligibility(shared, &missing_context),
            ComparisonEligibility::Incomplete
        );

        // No participants cannot prove coverage.
        assert_eq!(
            evaluate_comparison_eligibility(shared, &[]),
            ComparisonEligibility::Incomplete
        );
        Ok(())
    }
}
