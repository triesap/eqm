//! Exact scopes derived only from the checked graph and explicit bindings.

use crate::{
    ApplicabilityContext, FinalizedWorkspaceGraph, ObligationError, TruthValue,
    evaluate_participation,
};
use eqm_domain::{
    FullRequirementId, ProviderId, RequirementScenarioBindings, RequirementScope,
    ScenarioDependency, ScenarioParticipant, ScenarioRef, ScenarioSubject, TargetId,
    WorkspaceGraph,
};
use std::collections::{BTreeMap, BTreeSet};

/// Checked provider dependency scope, never an implicit attachment to all targets.
///
/// Construction is internal to checked graph derivation; there is no unchecked
/// public constructor or serde/default authority.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ProviderScope {
    provider: ProviderId,
    dependers: BTreeSet<TargetId>,
    assignments: BTreeSet<TargetId>,
    dependencies: BTreeSet<(TargetId, TargetId)>,
    participation: BTreeMap<TargetId, TruthValue>,
}

impl ProviderScope {
    /// Returns the exact provider identity.
    #[must_use]
    pub const fn provider(&self) -> &ProviderId {
        &self.provider
    }

    /// Returns selected targets with a direct or transitive declared assignment.
    #[must_use]
    pub const fn dependers(&self) -> &BTreeSet<TargetId> {
        &self.dependers
    }

    /// Returns actual assignment targets reached from selected dependers.
    #[must_use]
    pub const fn assignments(&self) -> &BTreeSet<TargetId> {
        &self.assignments
    }

    /// Returns exact dependency links on paths to those assignments.
    #[must_use]
    pub const fn dependencies(&self) -> &BTreeSet<(TargetId, TargetId)> {
        &self.dependencies
    }

    /// Returns participation for every actual selected depender.
    #[must_use]
    pub const fn participation(&self) -> &BTreeMap<TargetId, TruthValue> {
        &self.participation
    }

    pub(crate) fn participates(&self, target: &TargetId) -> bool {
        self.dependers.contains(target)
            && matches!(
                self.participation.get(target),
                Some(TruthValue::True | TruthValue::Unknown)
            )
    }
}

/// Exact checked scenario, retaining every occurrence and directed edge.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ScenarioScope {
    scenario: ScenarioRef,
    participants: BTreeSet<ScenarioParticipant>,
    dependencies: BTreeSet<ScenarioDependency>,
    selected_targets: BTreeSet<TargetId>,
    provider_dependencies: BTreeMap<ProviderId, ProviderScope>,
    participation: BTreeMap<TargetId, TruthValue>,
    missing_provider_assignment: bool,
}

impl ScenarioScope {
    /// Returns the explicit exact scenario reference.
    #[must_use]
    pub const fn scenario(&self) -> &ScenarioRef {
        &self.scenario
    }

    /// Returns all role-bearing occurrences, including repeated actors.
    #[must_use]
    pub const fn participants(&self) -> &BTreeSet<ScenarioParticipant> {
        &self.participants
    }

    /// Returns every authored occurrence dependency.
    #[must_use]
    pub const fn dependencies(&self) -> &BTreeSet<ScenarioDependency> {
        &self.dependencies
    }

    /// Returns affected targets within the selected policy cohort.
    #[must_use]
    pub const fn selected_targets(&self) -> &BTreeSet<TargetId> {
        &self.selected_targets
    }

    /// Returns exact assigned provider paths used by this scenario.
    #[must_use]
    pub const fn provider_dependencies(&self) -> &BTreeMap<ProviderId, ProviderScope> {
        &self.provider_dependencies
    }

    /// Returns relevant actor/depender participation without pruning occurrences.
    #[must_use]
    pub const fn participation(&self) -> &BTreeMap<TargetId, TruthValue> {
        &self.participation
    }

    pub(crate) const fn missing_provider_assignment(&self) -> bool {
        self.missing_provider_assignment
    }
}

/// Exact binding validation failure, retaining the supplied identities.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScopeBindingFailure {
    /// The full requirement is absent from the finalized graph.
    UnknownRequirement,
    /// The requirement does not have EndToEnd scope.
    NotEndToEnd,
    /// No scenario has the exact requested ID and revision.
    UnknownScenario,
}

/// Validates every supplied map entry against this same finalized graph.
///
/// Valid inactive/unselected bindings remain claims and cannot enlarge the
/// selected universe. Missing active entries are visible unresolved derivation,
/// rather than an invented all-target or all-scenarios selection.
pub fn validate_scope_bindings(
    finalized: &FinalizedWorkspaceGraph,
    bindings: &RequirementScenarioBindings,
) -> Result<(), ObligationError> {
    let graph = finalized.graph();
    for (full, scenario) in bindings.bindings() {
        let requirement = graph
            .surfaces()
            .values()
            .filter(|surface| surface.id().as_str() == full.owner())
            .flat_map(|surface| surface.requirements().values())
            .find(|requirement| requirement.id().as_str() == full.local())
            .or_else(|| {
                graph
                    .fragments()
                    .values()
                    .filter(|fragment| fragment.id().as_str() == full.owner())
                    .flat_map(|fragment| fragment.requirements().values())
                    .find(|requirement| requirement.id().as_str() == full.local())
            });
        let failure = match requirement {
            None => Some(ScopeBindingFailure::UnknownRequirement),
            Some(value) if value.scope() != RequirementScope::EndToEnd => {
                Some(ScopeBindingFailure::NotEndToEnd)
            }
            Some(_)
                if !graph
                    .scenarios()
                    .contains_key(&(scenario.id().clone(), scenario.revision())) =>
            {
                Some(ScopeBindingFailure::UnknownScenario)
            }
            Some(_) => None,
        };
        if let Some(reason) = failure {
            return Err(ObligationError::InvalidScopeBinding {
                requirement: full.clone(),
                scenario: scenario.clone(),
                reason,
            });
        }
    }
    Ok(())
}

pub(crate) fn provider_scope(
    graph: &WorkspaceGraph,
    provider: &ProviderId,
    selected: &BTreeSet<TargetId>,
    full: &FullRequirementId,
    applicability: &ApplicabilityContext,
) -> Result<ProviderScope, ObligationError> {
    if !graph.providers().contains_key(provider) {
        return Err(ObligationError::InvalidFinalizedGraph);
    }
    let mut scope = ProviderScope {
        provider: provider.clone(),
        dependers: BTreeSet::new(),
        assignments: BTreeSet::new(),
        dependencies: BTreeSet::new(),
        participation: BTreeMap::new(),
    };
    // Reverse reachability finds declared dependers in linear graph work. The
    // subsequent forward walk keeps only assignment paths reached from the
    // selected cohort, without recursive depth or repeated diamond expansion.
    let mut reverse: BTreeMap<TargetId, BTreeSet<TargetId>> = BTreeMap::new();
    let mut reachable = BTreeSet::new();
    for definition in graph.targets().values() {
        if definition.providers().contains(provider) {
            reachable.insert(definition.id().clone());
        }
        for dependency in definition.depends_on() {
            reverse
                .entry(dependency.clone())
                .or_default()
                .insert(definition.id().clone());
        }
    }
    let mut pending: Vec<_> = reachable.iter().cloned().collect();
    while let Some(target) = pending.pop() {
        if let Some(parents) = reverse.get(&target) {
            for parent in parents {
                if reachable.insert(parent.clone()) {
                    pending.push(parent.clone());
                }
            }
        }
    }
    scope.dependers = selected.intersection(&reachable).cloned().collect();
    let mut pending: Vec<_> = scope.dependers.iter().cloned().collect();
    let mut visited = BTreeSet::new();
    while let Some(target) = pending.pop() {
        if !visited.insert(target.clone()) {
            continue;
        }
        let definition = graph
            .targets()
            .get(&target)
            .ok_or(ObligationError::InvalidFinalizedGraph)?;
        if definition.providers().contains(provider) {
            scope.assignments.insert(target.clone());
        }
        for dependency in definition.depends_on() {
            if reachable.contains(dependency) {
                scope
                    .dependencies
                    .insert((target.clone(), dependency.clone()));
                pending.push(dependency.clone());
            }
        }
    }
    for target in &scope.dependers {
        scope.participation.insert(
            target.clone(),
            evaluate_participation(graph.participation(), full, target, applicability)?,
        );
    }
    Ok(scope)
}

pub(crate) fn scenario_scope(
    graph: &WorkspaceGraph,
    reference: &ScenarioRef,
    selected: &BTreeSet<TargetId>,
    full: &FullRequirementId,
    applicability: &ApplicabilityContext,
) -> Result<ScenarioScope, ObligationError> {
    let definition = graph
        .scenarios()
        .get(&(reference.id().clone(), reference.revision()))
        .ok_or(ObligationError::InvalidFinalizedGraph)?;
    let mut scope = ScenarioScope {
        scenario: reference.clone(),
        participants: definition.participants().clone(),
        dependencies: definition.dependencies().clone(),
        selected_targets: BTreeSet::new(),
        provider_dependencies: BTreeMap::new(),
        participation: BTreeMap::new(),
        missing_provider_assignment: false,
    };
    let mut relevant = BTreeSet::new();
    for participant in definition.participants() {
        match participant.subject() {
            ScenarioSubject::Target(target) => {
                relevant.insert(target.clone());
                if selected.contains(target) {
                    scope.selected_targets.insert(target.clone());
                }
            }
            ScenarioSubject::Provider(provider) => {
                let dependency = provider_scope(graph, provider, selected, full, applicability)?;
                scope.missing_provider_assignment |= dependency.dependers().is_empty();
                relevant.extend(dependency.dependers().iter().cloned());
                scope
                    .selected_targets
                    .extend(dependency.dependers().iter().cloned());
                scope
                    .provider_dependencies
                    .insert(provider.clone(), dependency);
            }
        }
    }
    for target in relevant {
        scope.participation.insert(
            target.clone(),
            evaluate_participation(graph.participation(), full, &target, applicability)?,
        );
    }
    Ok(scope)
}
