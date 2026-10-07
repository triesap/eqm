//! Normative facet-status precedence and target conformance.

use crate::{
    CoverageStatus, EvidenceOutcome, FreshnessStatus, ObligationKey, ScopeSubject, TruthValue,
    UnresolvedCoordinate, UnresolvedReason, WaiverEvaluation,
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

/// Non-authorizing reference classification of an asserted status map.
/// Production admission uses the checked evaluated carrier below.
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
    subject.participates(target)
}

/// Engine-evaluated facet statuses bound to their immutable derivation context.
/// Raw status maps and serialized claims cannot construct this carrier.
///
/// ```compile_fail
/// use eqm_engine::{CompletePolicyResolution, RequiredUniverse};
/// fn forge(universe: RequiredUniverse) {
///     let statuses = std::collections::BTreeMap::new();
///     let _ = CompletePolicyResolution::new(universe, statuses);
/// }
/// ```
///
/// ```compile_fail
/// use eqm_engine::EvaluatedFacets;
/// let forged = EvaluatedFacets { facets: Default::default() };
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvaluatedFacets {
    context: crate::obligations::EvaluationContext,
    facets: BTreeMap<ObligationKey, FacetStatus>,
}

/// Evaluates prepared facet facts in one checked derivation context.
/// Facts are inputs, not trust authority: application callers must validate
/// evidence, subject, counts, freshness and effective trust before supplying them.
/// Reference fixtures using asserted facts prove only local engine semantics.
pub fn evaluate_required_facets(
    universe: &crate::RequiredUniverse,
    inputs: BTreeMap<ObligationKey, FacetEvaluationInput>,
) -> Result<EvaluatedFacets, ConformanceError> {
    if inputs.keys().any(|key| !universe.contains(key)) {
        return Err(ConformanceError::UnexpectedCoordinate);
    }
    Ok(EvaluatedFacets {
        context: universe.context.clone(),
        facets: inputs
            .into_iter()
            .map(|(key, input)| (key, evaluate_facet_status(&input)))
            .collect(),
    })
}

/// Checked policy resolution whose completeness is validated against the
/// pre-evidence required universe instead of a freely asserted flag.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompletePolicyResolution {
    universe: crate::RequiredUniverse,
    facets: BTreeMap<ObligationKey, FacetStatus>,
}

impl CompletePolicyResolution {
    /// Pairs a validated universe with engine-evaluated same-context facet statuses.
    ///
    /// Context is checked even when coordinate keys coincide. Evaluation rejects
    /// facts outside the required universe before producing its carrier. Per-target
    /// participation completeness is checked when the target result is derived.
    pub fn new(
        universe: crate::RequiredUniverse,
        evaluated: EvaluatedFacets,
    ) -> Result<Self, ConformanceError> {
        if universe.context != evaluated.context {
            return Err(ConformanceError::ForeignEvaluationContext);
        }
        Ok(Self {
            universe,
            facets: evaluated.facets,
        })
    }

    /// Returns the validated required universe.
    #[must_use]
    pub const fn universe(&self) -> &crate::RequiredUniverse {
        &self.universe
    }

    /// Returns the per-coordinate facet statuses.
    #[must_use]
    pub const fn facets(&self) -> &BTreeMap<ObligationKey, FacetStatus> {
        &self.facets
    }
}

/// Derives one target result from a checked complete resolution, rejecting a
/// resolution that omits any participating required coordinate. Every target,
/// provider and target-set coordinate that participates in this target must
/// carry an explicit facet status, and any unresolved applicability or
/// participation coordinate is non-authorizing, so a set of reported passes
/// cannot be promoted to Conformant while work is still unknown. Unmatched
/// optional warnings and explicit not-applicable outcomes remain nonblocking.
pub fn evaluate_complete_target_conformance(
    target: &TargetId,
    resolution: &CompletePolicyResolution,
) -> Result<TargetConformance, ConformanceError> {
    if resolution
        .universe
        .coordinates()
        .iter()
        .filter(|key| participates(&key.subject, target))
        .any(|key| !resolution.facets.contains_key(key))
    {
        return Err(ConformanceError::IncompletePolicyResolution);
    }
    if resolution
        .universe
        .unresolved()
        .iter()
        .any(|unresolved| unresolved_blocks(unresolved, target))
    {
        return Ok(TargetConformance::Nonconformant);
    }
    let statuses: Vec<_> = resolution
        .facets
        .iter()
        .filter(|(key, _)| participates(&key.subject, target))
        .map(|(_, status)| *status)
        .collect();
    Ok(classify_statuses(&statuses))
}

fn unresolved_blocks(unresolved: &UnresolvedCoordinate, target: &TargetId) -> bool {
    if unresolved.reason == UnresolvedReason::MissingScopeAuthority {
        return true;
    }
    let affects = match &unresolved.subject {
        Some(subject) => participates(subject, target),
        None => true,
    };
    affects
        && matches!(
            unresolved.reason,
            UnresolvedReason::UnknownApplicability | UnresolvedReason::UnknownParticipation
        )
}

/// Target result construction failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConformanceError {
    /// Required policy/obligation construction did not complete.
    IncompletePolicyResolution,
    /// A facet status named a coordinate outside the required universe.
    UnexpectedCoordinate,
    /// Evaluated facts belong to a different semantic graph or selection context.
    ForeignEvaluationContext,
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

    #[test]
    fn every_unknown_axis_and_scope_participation_is_explicit() -> Result<(), Box<dyn Error>> {
        for configure in [
            |value: &mut FacetEvaluationInput| value.coverage = CoverageStatus::Unknown,
            |value: &mut FacetEvaluationInput| value.outcome = EvidenceOutcome::Unknown,
            |value: &mut FacetEvaluationInput| value.freshness = FreshnessStatus::Unknown,
            |value: &mut FacetEvaluationInput| value.trust = TrustEvaluation::Unknown,
            |value: &mut FacetEvaluationInput| value.exposure = SupportingCheck::Unknown,
            |value: &mut FacetEvaluationInput| value.release = SupportingCheck::Unknown,
        ] {
            let mut value = base();
            configure(&mut value);
            assert_eq!(evaluate_facet_status(&value), FacetStatus::Unknown);
        }

        let web = TargetId::new("web")?;
        let ios = TargetId::new("ios")?;
        let requirement: eqm_domain::FullRequirementId =
            "account.create.signup.identifier#email_default".parse()?;
        let provider = crate::ObligationKey {
            policy: eqm_domain::PolicyId::new("policy.release")?,
            policy_revision: eqm_domain::Revision::new(1)?,
            profiles: BTreeMap::new(),
            unit: eqm_domain::UnitId::new("account.create.signup.identifier")?,
            requirement: requirement.clone(),
            subject: ScopeSubject::Provider(eqm_domain::ProviderId::new("identity.primary")?),
            facet: eqm_domain::Facet::Behavior,
            release_context: None,
        };
        let target = crate::ObligationKey {
            subject: ScopeSubject::Target(web.clone()),
            ..provider.clone()
        };
        let target_set = crate::ObligationKey {
            subject: ScopeSubject::TargetSet(std::collections::BTreeSet::from([web.clone(), ios])),
            ..provider.clone()
        };
        let facets = BTreeMap::from([
            (provider, FacetStatus::Satisfied),
            (target, FacetStatus::Satisfied),
            (target_set, FacetStatus::Waived),
        ]);
        assert_eq!(
            evaluate_target_conformance(&web, &facets, true)?,
            TargetConformance::ConditionallyConformant
        );
        assert_eq!(
            evaluate_target_conformance(&web, &facets, false),
            Err(ConformanceError::IncompletePolicyResolution)
        );
        assert!(!participates(
            &ScopeSubject::Provider(eqm_domain::ProviderId::new("identity.primary")?),
            &web
        ));
        assert!(participates(&ScopeSubject::Target(web.clone()), &web));
        assert!(!participates(
            &ScopeSubject::Target(TargetId::new("android")?),
            &web
        ));
        assert_eq!(
            ConformanceError::IncompletePolicyResolution.to_string(),
            "IncompletePolicyResolution"
        );
        Ok(())
    }
}
