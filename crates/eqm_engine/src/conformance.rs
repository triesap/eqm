//! Normative facet-status precedence and target conformance.

use crate::{
    CoverageStatus, EvidenceOutcome, FreshnessStatus, ObligationKey, ScopeSubject, TruthValue,
    WaiverEvaluation,
};
use eqm_domain::TargetId;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Supporting structural, exposure, or release check state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SupportingCheck {
    /// The check is not required for this facet.
    NotRequired,
    /// The check passed.
    Passed,
    /// The check produced a terminal failure.
    Failed,
    /// Required supporting input was absent.
    Missing,
    /// Supporting input cannot be trusted or interpreted.
    Unknown,
}

/// Independently verified trust conclusion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrustEvaluation {
    /// Effective trust meets the obligation minimum.
    Sufficient,
    /// Effective verified trust is below the minimum.
    Insufficient,
    /// Trust authority or verification is invalid or absent.
    Unknown,
}

/// Closed obligation facet status.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum FacetStatus {
    /// Applicability is false.
    NotApplicable,
    /// Evidence and every required supporting check passed.
    Satisfied,
    /// A valid waiver covers a visible blocker.
    Waived,
    /// Terminal evidence or supporting check failed.
    Failed,
    /// Evidence identity matched but exceeded its age ceiling.
    Stale,
    /// Required evidence or supporting input was absent.
    Missing,
    /// Passing and failing immutable history coexist.
    Unstable,
    /// Context, trust, envelope, time, or internal state is unknowable.
    Unknown,
}

/// Prepared independent facts for one obligation facet.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FacetEvaluationInput {
    /// Applicability result.
    pub applicability: TruthValue,
    /// Exact subject, context, and envelope validation result.
    pub context_valid: bool,
    /// Exact evidence coverage result.
    pub coverage: CoverageStatus,
    /// Attempt and count aggregate.
    pub outcome: EvidenceOutcome,
    /// Exact freshness result.
    pub freshness: FreshnessStatus,
    /// Independently verified trust result.
    pub trust: TrustEvaluation,
    /// Non-executing structure result.
    pub structure: SupportingCheck,
    /// Intended/observed exposure result.
    pub exposure: SupportingCheck,
    /// Exact release-presence result.
    pub release: SupportingCheck,
    /// Exact protected waiver evaluation.
    pub waiver: WaiverEvaluation,
}

/// Applies the normative facet-status precedence table.
#[must_use]
pub fn evaluate_facet_status(input: &FacetEvaluationInput) -> FacetStatus {
    if input.applicability == TruthValue::False {
        return FacetStatus::NotApplicable;
    }
    if input.applicability == TruthValue::Unknown
        || !input.context_valid
        || input.coverage == CoverageStatus::Unknown
        || input.outcome == EvidenceOutcome::Unknown
        || input.freshness == FreshnessStatus::Unknown
        || input.trust == TrustEvaluation::Unknown
        || supporting_contains(input, SupportingCheck::Unknown)
    {
        return FacetStatus::Unknown;
    }
    if input.outcome == EvidenceOutcome::Unstable {
        return FacetStatus::Unstable;
    }
    let provisional = if input.outcome == EvidenceOutcome::Failed
        || input.trust == TrustEvaluation::Insufficient
        || supporting_contains(input, SupportingCheck::Failed)
    {
        Some(FacetStatus::Failed)
    } else if input.freshness == FreshnessStatus::Stale {
        Some(FacetStatus::Stale)
    } else if input.coverage == CoverageStatus::Missing
        || input.outcome == EvidenceOutcome::Missing
        || supporting_contains(input, SupportingCheck::Missing)
    {
        Some(FacetStatus::Missing)
    } else {
        None
    };
    if let Some(status) = provisional {
        if matches!(input.waiver, WaiverEvaluation::Waived(_)) {
            FacetStatus::Waived
        } else {
            status
        }
    } else {
        FacetStatus::Satisfied
    }
}

fn supporting_contains(input: &FacetEvaluationInput, state: SupportingCheck) -> bool {
    [input.structure, input.exposure, input.release].contains(&state)
}

/// Complete target-level conformance classification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TargetConformance {
    /// Every participating facet is satisfied or not applicable.
    Conformant,
    /// At least one facet is waived and every other facet is nonblocking.
    ConditionallyConformant,
    /// At least one participating facet is blocking.
    Nonconformant,
}

/// Derives one target result from a complete policy-derived obligation map.
pub fn evaluate_target_conformance(
    target: &TargetId,
    facets: &BTreeMap<ObligationKey, FacetStatus>,
    policy_resolution_complete: bool,
) -> Result<TargetConformance, ConformanceError> {
    if !policy_resolution_complete {
        return Err(ConformanceError::IncompletePolicyResolution);
    }
    let statuses: Vec<_> = facets
        .iter()
        .filter(|(key, _)| participates(&key.subject, target))
        .map(|(_, status)| *status)
        .collect();
    Ok(classify_statuses(&statuses))
}

fn classify_statuses(statuses: &[FacetStatus]) -> TargetConformance {
    if statuses.iter().any(|status| {
        matches!(
            status,
            FacetStatus::Failed
                | FacetStatus::Stale
                | FacetStatus::Missing
                | FacetStatus::Unstable
                | FacetStatus::Unknown
        )
    }) {
        TargetConformance::Nonconformant
    } else if statuses.contains(&FacetStatus::Waived) {
        TargetConformance::ConditionallyConformant
    } else {
        TargetConformance::Conformant
    }
}

fn participates(subject: &ScopeSubject, target: &TargetId) -> bool {
    match subject {
        ScopeSubject::Target(value) => value == target,
        ScopeSubject::Provider(_) => true,
        ScopeSubject::TargetSet(values) => values.contains(target),
    }
}

/// Target result construction failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConformanceError {
    /// Required policy/obligation construction did not complete.
    IncompletePolicyResolution,
}

impl Display for ConformanceError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for ConformanceError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WaiverEvaluation;
    use std::collections::BTreeMap;

    fn base() -> FacetEvaluationInput {
        FacetEvaluationInput {
            applicability: TruthValue::True,
            context_valid: true,
            coverage: CoverageStatus::Covered,
            outcome: EvidenceOutcome::Satisfied,
            freshness: FreshnessStatus::Fresh,
            trust: TrustEvaluation::Sufficient,
            structure: SupportingCheck::Passed,
            exposure: SupportingCheck::NotRequired,
            release: SupportingCheck::NotRequired,
            waiver: WaiverEvaluation::NotApplied(BTreeMap::new()),
        }
    }

    #[test]
    fn facet_precedence_covers_every_blocking_class() -> Result<(), Box<dyn Error>> {
        let mut cases = Vec::new();
        let mut value = base();
        value.applicability = TruthValue::False;
        cases.push((value, FacetStatus::NotApplicable));
        let mut value = base();
        value.applicability = TruthValue::Unknown;
        cases.push((value, FacetStatus::Unknown));
        let mut value = base();
        value.context_valid = false;
        cases.push((value, FacetStatus::Unknown));
        let mut value = base();
        value.outcome = EvidenceOutcome::Unstable;
        cases.push((value, FacetStatus::Unstable));
        let mut value = base();
        value.outcome = EvidenceOutcome::Failed;
        cases.push((value, FacetStatus::Failed));
        let mut value = base();
        value.freshness = FreshnessStatus::Stale;
        cases.push((value, FacetStatus::Stale));
        let mut value = base();
        value.coverage = CoverageStatus::Missing;
        cases.push((value, FacetStatus::Missing));
        let mut value = base();
        value.trust = TrustEvaluation::Insufficient;
        cases.push((value, FacetStatus::Failed));
        let mut value = base();
        value.structure = SupportingCheck::Unknown;
        cases.push((value, FacetStatus::Unknown));
        let mut value = base();
        value.exposure = SupportingCheck::Missing;
        cases.push((value, FacetStatus::Missing));
        let mut value = base();
        value.release = SupportingCheck::Failed;
        cases.push((value, FacetStatus::Failed));
        cases.push((base(), FacetStatus::Satisfied));
        for (input, expected) in cases {
            assert_eq!(evaluate_facet_status(&input), expected);
        }
        let mut waived = base();
        waived.outcome = EvidenceOutcome::Failed;
        waived.waiver = WaiverEvaluation::Waived(eqm_domain::WaiverId::new("waiver.test")?);
        assert_eq!(evaluate_facet_status(&waived), FacetStatus::Waived);
        waived.outcome = EvidenceOutcome::Unknown;
        assert_eq!(evaluate_facet_status(&waived), FacetStatus::Unknown);
        Ok(())
    }

    #[test]
    fn target_table_handles_empty_conditional_and_blocking_sets() {
        assert_eq!(classify_statuses(&[]), TargetConformance::Conformant);
        assert_eq!(
            classify_statuses(&[FacetStatus::Satisfied, FacetStatus::NotApplicable]),
            TargetConformance::Conformant
        );
        assert_eq!(
            classify_statuses(&[FacetStatus::Satisfied, FacetStatus::Waived]),
            TargetConformance::ConditionallyConformant
        );
        for blocking in [
            FacetStatus::Failed,
            FacetStatus::Stale,
            FacetStatus::Missing,
            FacetStatus::Unstable,
            FacetStatus::Unknown,
        ] {
            assert_eq!(
                classify_statuses(&[FacetStatus::Satisfied, blocking]),
                TargetConformance::Nonconformant
            );
        }
    }
}
