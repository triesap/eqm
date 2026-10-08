//! Claims for complete pre-evidence coordinate projections, never trust factories.
use crate::{
    ContractDigest, DurationMillis, ExpectedSelectionManifestDigest, Facet, FullRequirementId,
    ParticipationDigest, PolicySelectionInputV2, PositiveCount, Provider, RequirementLevel,
    RequirementScenarioBindingsDigest, Scenario, SemanticGraphV2Digest, Sha256Digest, Target,
    TargetConfigurationDigest, TargetId, TrustLevel, UnitId,
};
use std::collections::{BTreeMap, BTreeSet};

/// Closed mode included in a coordinate reference, not proof of authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObligationModeV2 {
    /// Local development.
    Development,
    /// Pull-request evaluation.
    PullRequest,
    /// Release evaluation.
    Release,
}
impl ObligationModeV2 {
    /// Exact projection tag.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Development => "development",
            Self::PullRequest => "pull_request",
            Self::Release => "release",
        }
    }
}
/// Closed origin claim; only host capability preparation proves its provenance.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObligationOriginV2 {
    /// Candidate definitions.
    CandidateLocal,
    /// Trusted invocation definitions.
    TrustedInvocation,
    /// Protected baseline definitions.
    ProtectedBaseline,
}
impl ObligationOriginV2 {
    /// Exact projection tag.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CandidateLocal => "candidate_local",
            Self::TrustedInvocation => "trusted_invocation",
            Self::ProtectedBaseline => "protected_baseline",
        }
    }
}
/// Explicit checked-scope participation state, retaining False and Unknown.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScopeParticipationV2 {
    /// Explicit participation.
    True,
    /// Explicit exclusion.
    False,
    /// Unresolved participation.
    Unknown,
}
impl ScopeParticipationV2 {
    /// Exact projection tag.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::True => "true",
            Self::False => "false",
            Self::Unknown => "unknown",
        }
    }
}
/// All provider dependency axes. Holding this claim grants no checked scope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderScopeInputV2<'a> {
    /// Actual provider definition.
    pub provider: &'a Provider,
    /// Every selected depender and its actual target configuration identity.
    pub dependers: BTreeMap<TargetId, TargetConfigurationDigest>,
    /// Every actual assignment and its target configuration identity.
    pub assignments: BTreeMap<TargetId, TargetConfigurationDigest>,
    /// Full qualified directed dependency closure, including intermediate targets.
    pub dependencies: BTreeSet<(TargetId, TargetId)>,
    /// Every dependency endpoint and its actual target configuration.
    pub dependency_configurations: BTreeMap<TargetId, TargetConfigurationDigest>,
    /// Every selected depender's exact participation.
    pub participation: BTreeMap<TargetId, ScopeParticipationV2>,
}
/// Closed complete scope claim. Lossy provider/target-set labels are absent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObligationScopeInputV2<'a> {
    /// One actual target definition.
    Target(&'a Target),
    /// Complete provider dependency scope.
    Provider(ProviderScopeInputV2<'a>),
    /// Exact authored scenario and full derived selected scope.
    Scenario {
        /// Scenario retains every occurrence, role, tagged subject and edge.
        scenario: &'a Scenario,
        /// Actual selected targets and their configurations.
        selected_targets: BTreeMap<TargetId, TargetConfigurationDigest>,
        /// Every referenced provider's full derived dependencies.
        providers: Vec<ProviderScopeInputV2<'a>>,
        /// Exact participant/depender participation, without filtering False.
        participation: BTreeMap<TargetId, ScopeParticipationV2>,
    },
}
/// Explicit expectation state. Pure projection claims confer no selection trust.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExpectedSelectionIdentityV2 {
    /// No frozen manifest for this coordinate; non-authorizing.
    Missing,
    /// Frozen component identities; canonical set duplicate checking is required.
    /// Application preparation never emits an empty present selection.
    Present(Vec<ExpectedSelectionManifestDigest>),
}
/// Every independent composed strength axis.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObligationStrengthInputV2 {
    /// Matching required level floor.
    pub minimum_level: RequirementLevel,
    /// Matching effective trust floor.
    pub minimum_trust: TrustLevel,
    /// Matching evidence age ceiling.
    pub maximum_age: DurationMillis,
    /// Matching distinct logical count floor.
    pub minimum_count: PositiveCount,
}
/// Complete pre-evidence coordinate claim. No default, deserializer or trust API.
/// Exact checked build/source subjects belong to separately prepared evaluation
/// inputs; graph/configuration digests never substitute for those subjects.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObligationIdentityInputV2<'a> {
    /// Exact owning unit.
    pub unit: &'a UnitId,
    /// Full requirement identity.
    pub requirement: &'a FullRequirementId,
    /// Independently constrained facet.
    pub facet: Facet,
    /// Exact policy/profile definitions and explicit known/unknown dimensions.
    pub selection: &'a PolicySelectionInputV2<'a>,
    /// Complete immutable semantic graph context.
    pub semantic_graph_digest: SemanticGraphV2Digest,
    /// Independent contract component identity.
    pub contract_digest: ContractDigest,
    /// Entire exact requirement/scenario binding map identity.
    pub scope_bindings_digest: RequirementScenarioBindingsDigest,
    /// Declared participation component identities, not just evaluated labels.
    pub participation_digests: Vec<ParticipationDigest>,
    /// Complete checked-scope claims.
    pub subject: ObligationScopeInputV2<'a>,
    /// Explicit Missing/Present independently frozen expected selection.
    pub expected_selections: ExpectedSelectionIdentityV2,
    /// Every composed constraint.
    pub strength: ObligationStrengthInputV2,
    /// Exact optional immutable release context.
    pub release_context: Option<Sha256Digest>,
    /// Prepared evaluation mode.
    pub mode: ObligationModeV2,
    /// Prepared authority origin; not its independent verification proof.
    pub authority_origin: ObligationOriginV2,
}
