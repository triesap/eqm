//! Deterministic policy obligation derivation.

use crate::{
    ApplicabilityContext, ApplicabilityError, FinalizedWorkspaceGraph, ProviderScope,
    ScenarioScope, ScopeBindingFailure, SelectedPolicyProfiles, SelectedProfile, TruthValue,
    evaluate_applicability, evaluate_participation, validate_scope_bindings,
};
use eqm_domain::{
    DurationMillis, Facet, FullRequirementId, LifecycleStatus, PolicyId, PositiveCount, ProfileId,
    ProviderId, Requirement, RequirementLevel, RequirementScenarioBindings, RequirementScope,
    Revision, RiskClass, ScenarioRef, Sha256Digest, TargetId, TrustLevel, UnitId,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Exact subject coordinate induced by requirement scope.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ScopeSubject {
    /// One required implementation target.
    Target(TargetId),
    /// Legacy provider-ID reference; it supplies no exact dependency authority.
    Provider(ProviderId),
    /// Legacy target-set reference; it supplies no exact scenario authority.
    TargetSet(BTreeSet<TargetId>),
    /// Checked provider assignment and exact selected dependers.
    CheckedProvider(ProviderScope),
    /// Exact checked scenario with all role-bearing occurrences and dependencies.
    Scenario(ScenarioScope),
}

impl ScopeSubject {
    /// Tests exact selected target membership. Legacy provider-ID references
    /// never imply a universal provider attachment.
    #[must_use]
    pub fn participates(&self, target: &TargetId) -> bool {
        match self {
            Self::Target(value) => value == target,
            Self::TargetSet(values) => values.contains(target),
            Self::Provider(_) => false,
            Self::CheckedProvider(scope) => scope.participates(target),
            Self::Scenario(scope) => scope.selected_targets().contains(target),
        }
    }

    /// Returns descriptive selected targets without enlarging the policy cohort.
    #[must_use]
    pub fn targets(&self) -> BTreeSet<TargetId> {
        match self {
            Self::Target(value) => BTreeSet::from([value.clone()]),
            Self::TargetSet(values) => values.clone(),
            Self::Provider(_) => BTreeSet::new(),
            Self::CheckedProvider(scope) => scope
                .dependers()
                .iter()
                .filter(|target| scope.participates(target))
                .cloned()
                .collect(),
            Self::Scenario(scope) => scope.selected_targets().clone(),
        }
    }
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
    /// Coordinate participation is declared but not resolved from the selected
    /// profile context.
    UnknownParticipation,
    /// Exact scenario binding or declared provider assignment is absent.
    MissingScopeAuthority,
    /// A non-required active requirement matched no policy rule.
    UnmatchedPolicy,
}

/// One visible requirement coordinate that yielded no derived obligation.
///
/// The exact subject, selected profiles and immutable release context are
/// retained so an unresolved coordinate cannot be collapsed into a
/// requirement-only marker and cannot be attributed to a different context.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UnresolvedCoordinate {
    /// Owning surface-derived unit.
    pub unit: UnitId,
    /// Fully qualified requirement identity.
    pub requirement: FullRequirementId,
    /// Exact coordinate subject when the requirement fanout is defined.
    pub subject: Option<ScopeSubject>,
    /// Every exact profile and selected known/unknown dimension.
    pub profiles: BTreeMap<ProfileId, SelectedProfile>,
    /// Exact optional release-context digest.
    pub release_context: Option<Sha256Digest>,
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
///
/// Construction is crate-private: there is no public `Default` or unchecked
/// constructor, so an empty or partial set cannot be asserted as complete
/// authority. Mixing a universe with status maps from a distinct selection is
/// rejected using private semantic provenance, including graph definitions,
/// composed strength, mode, origin, applicability and immutable release context.
/// Coordinate labels alone are insufficient. There is no public default or
/// unchecked constructor:
///
/// ```compile_fail
/// let _ = eqm_engine::RequiredUniverse::default();
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequiredUniverse {
    coordinates: BTreeSet<ObligationKey>,
    unresolved: BTreeSet<UnresolvedCoordinate>,
    pub(crate) context: EvaluationContext,
}

/// Private semantic provenance; only checked derivation can construct it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EvaluationContext {
    pub(crate) graph: eqm_domain::WorkspaceGraph,
    scope_bindings: RequirementScenarioBindings,
    policy: eqm_domain::Policy,
    profiles: BTreeMap<ProfileId, SelectedProfile>,
    applicability: ApplicabilityContext,
    mode: crate::EvaluationMode,
    origin: crate::AuthorityOrigin,
    release_context: Option<Sha256Digest>,
    pub(crate) obligations: BTreeMap<ObligationKey, Obligation>,
    pub(crate) expected_selections: Option<crate::expected_selection::ExpectedSelectionMap>,
}

impl RequiredUniverse {
    /// Returns the exact graph-validated map bound into this universe's context.
    #[must_use]
    pub const fn scope_bindings(&self) -> &RequirementScenarioBindings {
        &self.context.scope_bindings
    }

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

/// Derives obligations with no supplied scenario bindings. EndToEnd requirements
/// stay visibly unresolved; there is no implicit target-set selection.
pub fn derive_obligations(
    finalized: &FinalizedWorkspaceGraph,
    selection: &SelectedPolicyProfiles<'_>,
    applicability: &ApplicabilityContext,
    release_context: Option<Sha256Digest>,
) -> Result<ObligationDerivation, ObligationError> {
    let empty = RequirementScenarioBindings::new(Vec::new())
        .map_err(|_| ObligationError::InvalidFinalizedGraph)?;
    derive_obligations_with_scope_bindings(
        finalized,
        selection,
        applicability,
        release_context,
        &empty,
    )
}

/// Derives all scopes from this checked graph and explicit exact scenario claims.
pub fn derive_obligations_with_scope_bindings(
    finalized: &FinalizedWorkspaceGraph,
    selection: &SelectedPolicyProfiles<'_>,
    applicability: &ApplicabilityContext,
    release_context: Option<Sha256Digest>,
    bindings: &RequirementScenarioBindings,
) -> Result<ObligationDerivation, ObligationError> {
    derive_derivation(
        finalized,
        selection,
        applicability,
        release_context,
        bindings,
    )
}

/// Derives the required universe without implicit scenario bindings.
pub fn derive_required_universe(
    finalized: &FinalizedWorkspaceGraph,
    selection: &SelectedPolicyProfiles<'_>,
    applicability: &ApplicabilityContext,
    release_context: Option<Sha256Digest>,
) -> Result<RequiredUniverse, ObligationError> {
    let empty = RequirementScenarioBindings::new(Vec::new())
        .map_err(|_| ObligationError::InvalidFinalizedGraph)?;
    derive_required_universe_with_scope_bindings(
        finalized,
        selection,
        applicability,
        release_context,
        &empty,
    )
}

/// Derives the complete pre-evidence universe and binds the exact map into its
/// private semantic context before accepting any facet facts.
pub fn derive_required_universe_with_scope_bindings(
    finalized: &FinalizedWorkspaceGraph,
    selection: &SelectedPolicyProfiles<'_>,
    applicability: &ApplicabilityContext,
    release_context: Option<Sha256Digest>,
    bindings: &RequirementScenarioBindings,
) -> Result<RequiredUniverse, ObligationError> {
    let derived = derive_derivation(
        finalized,
        selection,
        applicability,
        release_context,
        bindings,
    )?;
    Ok(RequiredUniverse {
        coordinates: derived.obligations.keys().cloned().collect(),
        unresolved: derived.unresolved,
        context: EvaluationContext {
            graph: finalized.graph().clone(),
            scope_bindings: bindings.clone(),
            policy: selection.policy().clone(),
            profiles: selection.profiles().clone(),
            applicability: applicability.clone(),
            mode: selection.mode(),
            origin: selection.origin(),
            release_context,
            obligations: derived.obligations,
            expected_selections: None,
        },
    })
}

fn derive_derivation(
    finalized: &FinalizedWorkspaceGraph,
    selection: &SelectedPolicyProfiles<'_>,
    applicability: &ApplicabilityContext,
    release_context: Option<Sha256Digest>,
    bindings: &RequirementScenarioBindings,
) -> Result<ObligationDerivation, ObligationError> {
    let graph = finalized.graph();
    verify_same_graph_context(finalized, selection, applicability)?;
    validate_scope_bindings(finalized, bindings)?;
    let policy = selection.policy();
    // Coordinate participation always requires exact declared authority. A graph
    // that carries no rule for a coordinate, including v1 data without the
    // axis, is Unknown and non-authorizing; registry emptiness is never treated
    // as version detection or implicit global inclusion.
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
            let base = checked_base_subjects(
                graph,
                requirement,
                policy.required_targets(),
                &full,
                applicability,
                bindings,
            )?;
            match evaluate_applicability(requirement.applicability(), applicability)? {
                TruthValue::False => {
                    for subject in base_subject_options(&base) {
                        result.unresolved.insert(unresolved_coordinate(
                            &unit,
                            &full,
                            subject,
                            selection,
                            release_context,
                            UnresolvedReason::NotApplicable,
                        ));
                    }
                    continue;
                }
                TruthValue::Unknown => {
                    for subject in base_subject_options(&base) {
                        let false_participation = match &subject {
                            Some(ScopeSubject::Target(target)) => {
                                evaluate_participation(
                                    graph.participation(),
                                    &full,
                                    target,
                                    applicability,
                                )? == TruthValue::False
                            }
                            Some(ScopeSubject::CheckedProvider(scope)) => {
                                !scope.participation().is_empty()
                                    && scope
                                        .participation()
                                        .values()
                                        .all(|value| *value == TruthValue::False)
                            }
                            Some(ScopeSubject::Scenario(scope)) => {
                                !scope.missing_provider_assignment()
                                    && !scope.selected_targets().is_empty()
                                    && !scope.participation().is_empty()
                                    && scope
                                        .participation()
                                        .values()
                                        .all(|value| *value == TruthValue::False)
                            }
                            _ => false,
                        };
                        let missing_scope = match &subject {
                            Some(ScopeSubject::CheckedProvider(scope)) => {
                                scope.dependers().is_empty()
                            }
                            Some(ScopeSubject::Scenario(scope)) => {
                                scope.missing_provider_assignment()
                                    || scope.selected_targets().is_empty()
                                    || scope.participation().is_empty()
                            }
                            None => true,
                            _ => false,
                        };
                        let reason = if missing_scope {
                            UnresolvedReason::MissingScopeAuthority
                        } else if false_participation {
                            UnresolvedReason::NotApplicable
                        } else {
                            result.unknown_applicability.insert(full.clone());
                            UnresolvedReason::UnknownApplicability
                        };
                        result.unresolved.insert(unresolved_coordinate(
                            &unit,
                            &full,
                            subject,
                            selection,
                            release_context,
                            reason,
                        ));
                    }
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
                for subject in base_subject_options(&base) {
                    result.unresolved.insert(unresolved_coordinate(
                        &unit,
                        &full,
                        subject,
                        selection,
                        release_context,
                        UnresolvedReason::UnmatchedPolicy,
                    ));
                }
                continue;
            }
            let facets: BTreeSet<_> = rules
                .iter()
                .flat_map(|rule| rule.facets().iter().copied())
                .collect();
            let resolved = resolved_subjects(&base, &full, applicability, graph.participation())?;
            for (subject, reason) in resolved.unresolved {
                result.unresolved.insert(unresolved_coordinate(
                    &unit,
                    &full,
                    subject,
                    selection,
                    release_context,
                    reason,
                ));
            }
            for subject in resolved.subjects {
                for facet in &facets {
                    // Each facet composes only the rules that declare it, so an
                    // unrelated facet's trust/count/age strength cannot leak in.
                    let facet_rules: Vec<_> = rules
                        .iter()
                        .copied()
                        .filter(|rule| rule.facets().contains(facet))
                        .collect();
                    let strength = compose_strength(&facet_rules);
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

fn unresolved_coordinate(
    unit: &UnitId,
    requirement: &FullRequirementId,
    subject: Option<ScopeSubject>,
    selection: &SelectedPolicyProfiles<'_>,
    release_context: Option<Sha256Digest>,
    reason: UnresolvedReason,
) -> UnresolvedCoordinate {
    UnresolvedCoordinate {
        unit: unit.clone(),
        requirement: requirement.clone(),
        subject,
        profiles: selection.profiles().clone(),
        release_context,
        reason,
    }
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

fn checked_base_subjects(
    graph: &eqm_domain::WorkspaceGraph,
    requirement: &Requirement,
    targets: &BTreeSet<TargetId>,
    full: &FullRequirementId,
    applicability: &ApplicabilityContext,
    bindings: &RequirementScenarioBindings,
) -> Result<Vec<ScopeSubject>, ObligationError> {
    match requirement.scope() {
        RequirementScope::EachTarget => {
            Ok(targets.iter().cloned().map(ScopeSubject::Target).collect())
        }
        RequirementScope::SharedProvider => Ok(vec![ScopeSubject::CheckedProvider(
            crate::scope::provider_scope(
                graph,
                requirement
                    .provider()
                    .ok_or(ObligationError::InvalidFinalizedGraph)?,
                targets,
                full,
                applicability,
            )?,
        )]),
        RequirementScope::EndToEnd => bindings
            .bindings()
            .get(full)
            .map(|reference| {
                Ok(vec![ScopeSubject::Scenario(crate::scope::scenario_scope(
                    graph,
                    reference,
                    targets,
                    full,
                    applicability,
                )?)])
            })
            .unwrap_or_else(|| Ok(Vec::new())),
    }
}

fn base_subject_options(base: &[ScopeSubject]) -> Vec<Option<ScopeSubject>> {
    if base.is_empty() {
        vec![None]
    } else {
        base.iter().cloned().map(Some).collect()
    }
}

struct ResolvedSubjects {
    subjects: Vec<ScopeSubject>,
    unresolved: Vec<(Option<ScopeSubject>, UnresolvedReason)>,
}

fn resolved_subjects(
    base: &[ScopeSubject],
    full: &FullRequirementId,
    applicability: &ApplicabilityContext,
    participation: &BTreeMap<eqm_domain::ParticipationRuleId, eqm_domain::ParticipationRule>,
) -> Result<ResolvedSubjects, ObligationError> {
    let mut subjects = Vec::new();
    let mut unresolved = Vec::new();
    if base.is_empty() {
        unresolved.push((None, UnresolvedReason::MissingScopeAuthority));
    }
    for subject in base {
        let reason = match subject {
            ScopeSubject::Target(target) => {
                match evaluate_participation(participation, full, target, applicability)? {
                    TruthValue::True => None,
                    TruthValue::False => Some(UnresolvedReason::NotApplicable),
                    TruthValue::Unknown => Some(UnresolvedReason::UnknownParticipation),
                }
            }
            ScopeSubject::CheckedProvider(scope) => {
                let values = scope.participation();
                if scope.dependers().is_empty() {
                    Some(UnresolvedReason::MissingScopeAuthority)
                } else if values.values().any(|v| *v == TruthValue::Unknown) {
                    Some(UnresolvedReason::UnknownParticipation)
                } else if values.values().any(|v| *v == TruthValue::True) {
                    None
                } else {
                    Some(UnresolvedReason::NotApplicable)
                }
            }
            ScopeSubject::Scenario(scope) => {
                let values = scope.participation();
                if scope.missing_provider_assignment()
                    || scope.selected_targets().is_empty()
                    || values.is_empty()
                {
                    Some(UnresolvedReason::MissingScopeAuthority)
                } else if values.values().all(|v| *v == TruthValue::True) {
                    None
                } else if values.values().all(|v| *v == TruthValue::False) {
                    Some(UnresolvedReason::NotApplicable)
                } else {
                    Some(UnresolvedReason::UnknownParticipation)
                }
            }
            ScopeSubject::Provider(_) | ScopeSubject::TargetSet(_) => {
                return Err(ObligationError::InvalidFinalizedGraph);
            }
        };
        if let Some(reason) = reason {
            unresolved.push((Some(subject.clone()), reason));
        } else {
            subjects.push(subject.clone());
        }
    }
    Ok(ResolvedSubjects {
        subjects,
        unresolved,
    })
}

/// Verifies that one authorizing derivation consumes an exact selection and
/// applicability context resolved from the same checked graph.
///
/// A structurally identical policy/profile/context borrowed from a different
/// graph is indistinguishable and therefore admissible; a weaker foreign policy
/// with the same ID/revision, a foreign applicability context, or a selected
/// profile revision absent from this graph rejects rather than silently
/// shrinking the denominator.
fn verify_same_graph_context(
    finalized: &FinalizedWorkspaceGraph,
    selection: &SelectedPolicyProfiles<'_>,
    applicability: &ApplicabilityContext,
) -> Result<(), ObligationError> {
    let graph = finalized.graph();
    let policy = selection.policy();
    let graph_policy = graph
        .policies()
        .get(&(policy.id().clone(), policy.revision()))
        .ok_or(ObligationError::ForeignSelection)?;
    if graph_policy != policy {
        return Err(ObligationError::ForeignSelection);
    }
    let mut inputs = Vec::with_capacity(selection.profiles().len());
    for selected in selection.profiles().values() {
        let definition = graph
            .profiles()
            .get(&(selected.id().clone(), selected.revision()))
            .ok_or(ObligationError::ForeignSelection)?;
        if selection.definitions().get(selected.id()) != Some(definition) {
            return Err(ObligationError::ForeignSelection);
        }
        inputs.push((definition, selected.values()));
    }
    let expected = ApplicabilityContext::from_profiles(inputs)?;
    if &expected != applicability {
        return Err(ObligationError::ForeignApplicability);
    }
    Ok(())
}

/// Obligation derivation failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObligationError {
    /// A supplied exact binding does not belong to this graph/scope.
    InvalidScopeBinding {
        /// Full supplied requirement identity.
        requirement: FullRequirementId,
        /// Exact supplied scenario identity and revision.
        scenario: ScenarioRef,
        /// Exact validation failure.
        reason: ScopeBindingFailure,
    },
    /// Finalized input violated an assumed typed graph invariant.
    InvalidFinalizedGraph,
    /// Applicability named undeclared profile authority.
    Applicability(ApplicabilityError),
    /// The selection policy, or a selected profile definition, is not the exact
    /// definition of this checked graph.
    ForeignSelection,
    /// The supplied applicability context was not resolved from this graph's
    /// exact selected profile definitions.
    ForeignApplicability,
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

    fn base_subjects(
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

    #[test]
    fn legacy_reference_shapes_do_not_assert_checked_scope_authority() -> Result<(), Box<dyn Error>>
    {
        let targets = BTreeSet::from([TargetId::new("ios")?, TargetId::new("web")?]);
        assert_eq!(
            base_subjects(&requirement(RequirementScope::EachTarget)?, &targets)?,
            vec![
                ScopeSubject::Target(TargetId::new("ios")?),
                ScopeSubject::Target(TargetId::new("web")?),
            ]
        );
        assert_eq!(
            base_subjects(&requirement(RequirementScope::SharedProvider)?, &targets)?,
            vec![ScopeSubject::Provider(ProviderId::new("identity.primary")?)]
        );
        assert_eq!(
            base_subjects(&requirement(RequirementScope::EndToEnd)?, &targets)?,
            vec![ScopeSubject::TargetSet(targets)]
        );
        Ok(())
    }
}
