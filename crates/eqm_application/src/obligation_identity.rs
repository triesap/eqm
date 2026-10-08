//! Single-universe complete pre-evidence identity preparation for all adapters.
use crate::PreparedExpectedSelections;
use eqm_domain::{
    ExpectedSelectionIdentityV2, ObligationIdentityInputV2, ObligationModeV2, ObligationOriginV2,
    ObligationScopeInputV2, ObligationStrengthInputV2, PolicySelectionInputError,
    PolicySelectionInputV2, ProviderScopeInputV2, ScopeParticipationV2, SelectedDimensionStateV2,
    SelectedProfileInputV2, TargetConfigurationDigest, TargetId,
};
use eqm_engine::{
    AuthorityOrigin, EvaluationMode, FinalizedWorkspaceGraph, ObligationKey, ProviderScope,
    RequiredUniverse, ScopeSubject, TruthValue,
};
use eqm_manifest::{
    CanonicalObligationV2, CanonicalizationError, canonicalize_contract_v2, canonicalize_graph_v2,
    canonicalize_obligation_v2, canonicalize_participation_v2, canonicalize_scope_bindings_v2,
    canonicalize_target_configuration_v2,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Opaque complete pre-evidence identity map, retaining exact private context.
/// No identity grants evidence, checked source/build or release authorization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedObligationIdentities {
    universe: RequiredUniverse,
    identities: BTreeMap<ObligationKey, CanonicalObligationV2>,
}
impl PreparedObligationIdentities {
    /// Exact universe used for every reference, before presentation filtering.
    #[must_use]
    pub const fn universe(&self) -> &RequiredUniverse {
        &self.universe
    }
    /// Complete identities in canonical coordinate order.
    #[must_use]
    pub const fn identities(&self) -> &BTreeMap<ObligationKey, CanonicalObligationV2> {
        &self.identities
    }
}
/// Fail-closed preparation errors, without graph/component or label fallbacks.
#[derive(Debug)]
pub enum ObligationIdentityError {
    /// The supplied graph differs from the immutable expected-bound universe.
    Graph,
    /// An exact graph definition/configuration is unavailable.
    Definition,
    /// A lossy legacy provider/target-set reference cannot identify checked scope.
    LossyScope,
    /// Exact policy/profile carrier validation failed.
    Selection(PolicySelectionInputError),
    /// A pure canonical component/coordinate projection failed.
    Projection(CanonicalizationError),
}
impl Display for ObligationIdentityError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "obligation identity preparation failed: {self:?}")
    }
}
impl Error for ObligationIdentityError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Selection(error) => Some(error),
            Self::Projection(error) => Some(error),
            Self::Graph | Self::Definition | Self::LossyScope => None,
        }
    }
}
fn state(value: TruthValue) -> ScopeParticipationV2 {
    match value {
        TruthValue::True => ScopeParticipationV2::True,
        TruthValue::False => ScopeParticipationV2::False,
        TruthValue::Unknown => ScopeParticipationV2::Unknown,
    }
}
fn configurations(
    ids: impl IntoIterator<Item = TargetId>,
    known: &BTreeMap<TargetId, TargetConfigurationDigest>,
) -> Result<BTreeMap<TargetId, TargetConfigurationDigest>, ObligationIdentityError> {
    ids.into_iter()
        .map(|id| {
            let digest = *known.get(&id).ok_or(ObligationIdentityError::Definition)?;
            Ok((id, digest))
        })
        .collect()
}
fn provider<'a>(
    graph: &'a eqm_domain::WorkspaceGraph,
    scope: &ProviderScope,
    known: &BTreeMap<TargetId, TargetConfigurationDigest>,
) -> Result<ProviderScopeInputV2<'a>, ObligationIdentityError> {
    let endpoints: BTreeSet<_> = scope
        .dependencies()
        .iter()
        .flat_map(|(from, to)| [from.clone(), to.clone()])
        .collect();
    Ok(ProviderScopeInputV2 {
        provider: graph
            .providers()
            .get(scope.provider())
            .ok_or(ObligationIdentityError::Definition)?,
        dependers: configurations(scope.dependers().iter().cloned(), known)?,
        assignments: configurations(scope.assignments().iter().cloned(), known)?,
        dependencies: scope.dependencies().clone(),
        dependency_configurations: configurations(endpoints, known)?,
        participation: scope
            .participation()
            .iter()
            .map(|(id, value)| (id.clone(), state(*value)))
            .collect(),
    })
}
pub(crate) fn subject<'a>(
    graph: &'a eqm_domain::WorkspaceGraph,
    subject: &ScopeSubject,
    known: &BTreeMap<TargetId, TargetConfigurationDigest>,
) -> Result<ObligationScopeInputV2<'a>, ObligationIdentityError> {
    match subject {
        ScopeSubject::Target(id) => Ok(ObligationScopeInputV2::Target(
            graph
                .targets()
                .get(id)
                .ok_or(ObligationIdentityError::Definition)?,
        )),
        ScopeSubject::CheckedProvider(scope) => Ok(ObligationScopeInputV2::Provider(provider(
            graph, scope, known,
        )?)),
        ScopeSubject::Scenario(scope) => Ok(ObligationScopeInputV2::Scenario {
            scenario: graph
                .scenarios()
                .get(&(scope.scenario().id().clone(), scope.scenario().revision()))
                .ok_or(ObligationIdentityError::Definition)?,
            selected_targets: configurations(scope.selected_targets().iter().cloned(), known)?,
            providers: scope
                .provider_dependencies()
                .values()
                .map(|scope| provider(graph, scope, known))
                .collect::<Result<Vec<_>, _>>()?,
            participation: scope
                .participation()
                .iter()
                .map(|(id, value)| (id.clone(), state(*value)))
                .collect(),
        }),
        ScopeSubject::Provider(_) | ScopeSubject::TargetSet(_) => {
            Err(ObligationIdentityError::LossyScope)
        }
    }
}
/// Prepares every identity once from the checked expected-bound universe.
/// Selection, graph, clocks, results and filters are never fetched or recomputed.
/// Pure projections retain claims; this operation verifies their same-graph source.
pub fn prepare_obligation_identities(
    finalized: &FinalizedWorkspaceGraph,
    expected: &PreparedExpectedSelections,
) -> Result<PreparedObligationIdentities, ObligationIdentityError> {
    let universe = expected.universe();
    if !universe.matches_graph(finalized) {
        return Err(ObligationIdentityError::Graph);
    }
    let graph = finalized.graph();
    let mut profiles = Vec::new();
    for selected in universe.profiles().values() {
        let definition = graph
            .profiles()
            .get(&(selected.id().clone(), selected.revision()))
            .ok_or(ObligationIdentityError::Definition)?;
        let values = selected
            .values()
            .iter()
            .map(|(dimension, value)| {
                (
                    dimension.clone(),
                    value
                        .as_ref()
                        .map_or(SelectedDimensionStateV2::Unknown, |value| {
                            SelectedDimensionStateV2::Known(value.clone())
                        }),
                )
            })
            .collect();
        profiles.push(
            SelectedProfileInputV2::new(definition, values)
                .map_err(ObligationIdentityError::Selection)?,
        );
    }
    let selection = PolicySelectionInputV2::new(universe.policy(), profiles)
        .map_err(ObligationIdentityError::Selection)?;
    let semantic_graph_digest = canonicalize_graph_v2(graph)
        .map_err(ObligationIdentityError::Projection)?
        .digest();
    let contract_digest = canonicalize_contract_v2(graph)
        .map_err(ObligationIdentityError::Projection)?
        .digest();
    let scope_bindings_digest = canonicalize_scope_bindings_v2(universe.scope_bindings())
        .map_err(ObligationIdentityError::Projection)?
        .digest();
    let participation_digests = graph
        .participation()
        .values()
        .map(|value| canonicalize_participation_v2(value).map(|value| value.digest()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(ObligationIdentityError::Projection)?;
    let known = graph
        .targets()
        .values()
        .map(|target| {
            Ok((
                target.id().clone(),
                canonicalize_target_configuration_v2(target)?.digest(),
            ))
        })
        .collect::<Result<BTreeMap<_, _>, CanonicalizationError>>()
        .map_err(ObligationIdentityError::Projection)?;
    let mode = match universe.mode() {
        EvaluationMode::Development => ObligationModeV2::Development,
        EvaluationMode::PullRequest => ObligationModeV2::PullRequest,
        EvaluationMode::Release => ObligationModeV2::Release,
    };
    let authority_origin = match universe.authority_origin() {
        AuthorityOrigin::CandidateLocal => ObligationOriginV2::CandidateLocal,
        AuthorityOrigin::TrustedInvocation => ObligationOriginV2::TrustedInvocation,
        AuthorityOrigin::ProtectedBaseline => ObligationOriginV2::ProtectedBaseline,
    };
    let mut identities = BTreeMap::new();
    for (key, obligation) in universe.obligations() {
        let strength = obligation.strength;
        let value = ObligationIdentityInputV2 {
            unit: &key.unit,
            requirement: &key.requirement,
            facet: key.facet,
            selection: &selection,
            semantic_graph_digest,
            contract_digest,
            scope_bindings_digest,
            participation_digests: participation_digests.clone(),
            subject: subject(graph, &key.subject, &known)?,
            expected_selections: expected
                .digests()
                .get(key)
                .map_or(ExpectedSelectionIdentityV2::Missing, |digests| {
                    ExpectedSelectionIdentityV2::Present(digests.clone())
                }),
            strength: ObligationStrengthInputV2 {
                minimum_level: strength.minimum_level,
                minimum_trust: strength.minimum_trust,
                maximum_age: strength.maximum_age,
                minimum_count: strength.minimum_count,
            },
            release_context: universe.release_context(),
            mode,
            authority_origin,
        };
        identities.insert(
            key.clone(),
            canonicalize_obligation_v2(&value).map_err(ObligationIdentityError::Projection)?,
        );
    }
    Ok(PreparedObligationIdentities {
        universe: universe.clone(),
        identities,
    })
}
