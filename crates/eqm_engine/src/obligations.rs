//! Deterministic policy obligation derivation.

use crate::{
    ApplicabilityContext, ApplicabilityError, SelectedPolicyProfiles, SelectedProfile, TruthValue,
    evaluate_applicability,
};
use eqm_domain::{
    DurationMillis, Facet, FinalizedWorkspaceGraph, FullRequirementId, LifecycleStatus, PolicyId,
    PositiveCount, ProfileId, ProviderId, Requirement, RequirementLevel, RequirementScope,
    Revision, RiskClass, Sha256Digest, TargetId, TrustLevel, UnitId,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Exact subject coordinate induced by requirement scope.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ScopeSubject {
    /// One required implementation target.
    Target(TargetId),
    /// One shared provider executed once for all targets.
    Provider(ProviderId),
    /// The sorted complete required-target set.
    TargetSet(BTreeSet<TargetId>),
}

/// Stable identity of one independently evaluated obligation facet.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ObligationKey {
    /// Selected policy ID.
    pub policy: PolicyId,
    /// Selected policy revision.
    pub policy_revision: Revision,
    /// Every exact profile and selected known/unknown dimension.
    pub profiles: BTreeMap<ProfileId, SelectedProfile>,
    /// Owning surface or fragment-derived unit.
    pub unit: UnitId,
    /// Fully qualified requirement identity.
    pub requirement: FullRequirementId,
    /// Exact scope subject.
    pub subject: ScopeSubject,
    /// Independently evaluated facet.
    pub facet: Facet,
    /// Exact optional release-context digest.
    pub release_context: Option<Sha256Digest>,
}

/// Strongest monotonic composition for one obligation key.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObligationStrength {
    /// Strongest matching minimum requirement level.
    pub minimum_level: RequirementLevel,
    /// Strongest matching minimum trust.
    pub minimum_trust: TrustLevel,
    /// Smallest matching evidence age ceiling.
    pub maximum_age: DurationMillis,
    /// Largest matching minimum evidence count.
    pub minimum_count: PositiveCount,
}

/// One derived obligation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Obligation {
    /// Stable coordinate.
    pub key: ObligationKey,
    /// Composed strength.
    pub strength: ObligationStrength,
}

/// Why one active requirement is visible but not a derived obligation.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UnresolvedReason {
    /// Applicability is explicitly false under approved authority.
    NotApplicable,
    /// Applicability could not be resolved from the selected profile context.
    UnknownApplicability,
    /// A non-required active requirement matched no policy rule.
    UnmatchedPolicy,
}

/// One visible requirement coordinate that yielded no derived obligation.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UnresolvedCoordinate {
    /// Owning surface-derived unit.
    pub unit: UnitId,
    /// Fully qualified requirement identity.
    pub requirement: FullRequirementId,
    /// Why the requirement is not an obligation.
    pub reason: UnresolvedReason,
}

/// Complete derivation, including non-obligation outcomes that remain visible.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ObligationDerivation {
    /// Unique obligations in key order.
    pub obligations: BTreeMap<ObligationKey, Obligation>,
    /// Recommended or optional active requirements with no matching policy rule.
    pub unmatched_warnings: BTreeSet<FullRequirementId>,
    /// Requirements whose applicability was unknown.
    pub unknown_applicability: BTreeSet<FullRequirementId>,
    /// Every visible requirement coordinate that yielded no obligation, with
    /// its exact reason, so completeness cannot silently drop it.
    pub unresolved: BTreeSet<UnresolvedCoordinate>,
}

/// The complete pre-evidence required denominator for one exact selection.
///
/// It is derived from the finalized graph, the selected policy/profiles and the
/// applicability context, before any evidence is read. Evidence filtering,
/// pagination or resource limits must not shrink this set, so later steps
/// compare candidate universes against it rather than regenerating scope.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RequiredUniverse {
    coordinates: BTreeSet<ObligationKey>,
    unresolved: BTreeSet<UnresolvedCoordinate>,
}

impl RequiredUniverse {
    /// Returns every exact required coordinate in canonical order.
    #[must_use]
    pub const fn coordinates(&self) -> &BTreeSet<ObligationKey> {
        &self.coordinates
    }

    /// Returns every visible requirement coordinate that yielded no obligation,
    /// preserving explicit unknown and justified not-applicable reasons.
    #[must_use]
    pub const fn unresolved(&self) -> &BTreeSet<UnresolvedCoordinate> {
        &self.unresolved
    }

    /// Returns whether one exact coordinate is in the required denominator.
    #[must_use]
    pub fn contains(&self, coordinate: &ObligationKey) -> bool {
        self.coordinates.contains(coordinate)
    }

    /// Returns whether `self` is a subset of `other`.
    ///
    /// A candidate universe that omits a coordinate in this protected set has
    /// removed required work and cannot be treated as stronger.
    #[must_use]
    pub fn is_subset_of(&self, other: &Self) -> bool {
        self.coordinates.is_subset(&other.coordinates)
    }
}

/// Derives obligations from a finalized graph and exact selected policy context.
pub fn derive_obligations(
    finalized: &FinalizedWorkspaceGraph,
    selection: &SelectedPolicyProfiles<'_>,
    applicability: &ApplicabilityContext,
    release_context: Option<Sha256Digest>,
) -> Result<ObligationDerivation, ObligationError> {
    derive_derivation(finalized, selection, applicability, release_context)
}

/// Derives the complete pre-evidence required universe for one exact selection.
pub fn derive_required_universe(
    finalized: &FinalizedWorkspaceGraph,
    selection: &SelectedPolicyProfiles<'_>,
    applicability: &ApplicabilityContext,
    release_context: Option<Sha256Digest>,
) -> Result<RequiredUniverse, ObligationError> {
    let derived = derive_derivation(finalized, selection, applicability, release_context)?;
    Ok(RequiredUniverse {
        coordinates: derived.obligations.keys().cloned().collect(),
        unresolved: derived.unresolved,
    })
}

fn derive_derivation(
    finalized: &FinalizedWorkspaceGraph,
    selection: &SelectedPolicyProfiles<'_>,
    applicability: &ApplicabilityContext,
    release_context: Option<Sha256Digest>,
) -> Result<ObligationDerivation, ObligationError> {
    let graph = finalized.graph();
    let policy = selection.policy();
    let mut result = ObligationDerivation::default();
    for surface in graph.surfaces().values() {
        if surface.status() != LifecycleStatus::Active {
            continue;
        }
        let journey = graph
            .journeys()
            .get(surface.journey())
            .ok_or(ObligationError::InvalidFinalizedGraph)?;
        let unit = UnitId::new(surface.id().as_str())
            .map_err(|_| ObligationError::InvalidFinalizedGraph)?;
        for requirement in surface.requirements().values() {
            let full = FullRequirementId::new(format!("{unit}#{}", requirement.id()))
                .map_err(|_| ObligationError::InvalidFinalizedGraph)?;
            match evaluate_applicability(requirement.applicability(), applicability)? {
                TruthValue::False => {
                    result.unresolved.insert(UnresolvedCoordinate {
                        unit: unit.clone(),
                        requirement: full.clone(),
                        reason: UnresolvedReason::NotApplicable,
                    });
                    continue;
                }
                TruthValue::Unknown => {
                    result.unknown_applicability.insert(full.clone());
                    result.unresolved.insert(UnresolvedCoordinate {
                        unit: unit.clone(),
                        requirement: full.clone(),
                        reason: UnresolvedReason::UnknownApplicability,
                    });
                    continue;
                }
                TruthValue::True => {}
            }
            let risk = requirement
                .risk_class()
                .unwrap_or_else(|| journey.risk_class());
            let rules: Vec<_> = policy
                .rules()
                .iter()
                .filter(|rule| {
                    selector_matches(rule.selector(), &unit, &full, risk, requirement)
                        && requirement.level() >= rule.minimum_level()
                })
                .collect();
            if rules.is_empty() {
                if requirement.level() == RequirementLevel::Required {
                    return Err(ObligationError::UnmatchedRequiredRequirement);
                }
                result.unmatched_warnings.insert(full.clone());
                result.unresolved.insert(UnresolvedCoordinate {
                    unit: unit.clone(),
                    requirement: full,
                    reason: UnresolvedReason::UnmatchedPolicy,
                });
                continue;
            }
            let facets: BTreeSet<_> = rules
                .iter()
                .flat_map(|rule| rule.facets().iter().copied())
                .collect();
            let strength = compose_strength(&rules);
            for subject in subjects(requirement, policy.required_targets())? {
                for facet in &facets {
                    let key = ObligationKey {
                        policy: policy.id().clone(),
                        policy_revision: policy.revision(),
                        profiles: selection.profiles().clone(),
                        unit: unit.clone(),
                        requirement: full.clone(),
                        subject: subject.clone(),
                        facet: *facet,
                        release_context,
                    };
                    let obligation = Obligation {
                        key: key.clone(),
                        strength,
                    };
                    if result.obligations.insert(key, obligation).is_some() {
                        return Err(ObligationError::DuplicateObligation);
                    }
                }
            }
        }
    }
    Ok(result)
}

fn selector_matches(
    selector: &eqm_domain::PolicySelector,
    unit: &UnitId,
    requirement: &FullRequirementId,
    risk: RiskClass,
    definition: &Requirement,
) -> bool {
    selector.units().is_none_or(|values| values.contains(unit))
        && selector
            .requirements()
            .is_none_or(|values| values.contains(requirement))
        && selector
            .risk_classes()
            .is_none_or(|values| values.contains(&risk))
        && selector
            .facets()
            .is_none_or(|values| !values.is_disjoint(definition.facets()))
        && selector
            .scopes()
            .is_none_or(|values| values.contains(&definition.scope()))
}

fn compose_strength(rules: &[&eqm_domain::PolicyRule]) -> ObligationStrength {
    let first = rules[0];
    rules.iter().skip(1).fold(
        ObligationStrength {
            minimum_level: first.minimum_level(),
            minimum_trust: first.minimum_trust(),
            maximum_age: first.maximum_age(),
            minimum_count: first.minimum_count(),
        },
        |strength, rule| ObligationStrength {
            minimum_level: strength.minimum_level.max(rule.minimum_level()),
            minimum_trust: strength.minimum_trust.max(rule.minimum_trust()),
            maximum_age: strength.maximum_age.min(rule.maximum_age()),
            minimum_count: strength.minimum_count.max(rule.minimum_count()),
        },
    )
}

fn subjects(
    requirement: &Requirement,
    targets: &BTreeSet<TargetId>,
) -> Result<Vec<ScopeSubject>, ObligationError> {
    match requirement.scope() {
        RequirementScope::EachTarget => {
            Ok(targets.iter().cloned().map(ScopeSubject::Target).collect())
        }
        RequirementScope::SharedProvider => Ok(vec![ScopeSubject::Provider(
            requirement
                .provider()
                .cloned()
                .ok_or(ObligationError::InvalidFinalizedGraph)?,
        )]),
        RequirementScope::EndToEnd => Ok(vec![ScopeSubject::TargetSet(targets.clone())]),
    }
}

/// Obligation derivation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObligationError {
    /// Finalized input violated an assumed typed graph invariant.
    InvalidFinalizedGraph,
    /// Applicability named undeclared profile authority.
    Applicability(ApplicabilityError),
    /// An active required requirement matched no policy rule.
    UnmatchedRequiredRequirement,
    /// Two derivations unexpectedly produced the same exact key.
    DuplicateObligation,
}

impl From<ApplicabilityError> for ObligationError {
    fn from(value: ApplicabilityError) -> Self {
        Self::Applicability(value)
    }
}

impl Display for ObligationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for ObligationError {}

#[cfg(test)]
mod tests {
    use super::*;
    use eqm_domain::{Applicability, Extensions, LocalRequirementId, RequirementStatement};

    fn requirement(scope: RequirementScope) -> Result<Requirement, Box<dyn Error>> {
        Ok(Requirement::new(
            LocalRequirementId::new("sample")?,
            RequirementLevel::Required,
            scope,
            RequirementStatement::new("A bounded normative statement.")?,
            vec![Facet::Behavior],
            Applicability::always(true),
            None,
            (scope == RequirementScope::SharedProvider)
                .then(|| ProviderId::new("identity.primary"))
                .transpose()?,
            Extensions::default(),
        )?)
    }

    #[test]
    fn every_scope_maps_to_its_exact_subject_shape() -> Result<(), Box<dyn Error>> {
        let targets = BTreeSet::from([TargetId::new("ios")?, TargetId::new("web")?]);
        assert_eq!(
            subjects(&requirement(RequirementScope::EachTarget)?, &targets)?,
            vec![
                ScopeSubject::Target(TargetId::new("ios")?),
                ScopeSubject::Target(TargetId::new("web")?),
            ]
        );
        assert_eq!(
            subjects(&requirement(RequirementScope::SharedProvider)?, &targets)?,
            vec![ScopeSubject::Provider(ProviderId::new("identity.primary")?)]
        );
        assert_eq!(
            subjects(&requirement(RequirementScope::EndToEnd)?, &targets)?,
            vec![ScopeSubject::TargetSet(targets)]
        );
        Ok(())
    }
}
