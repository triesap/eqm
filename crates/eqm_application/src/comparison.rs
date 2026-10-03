//! Session-graph-backed v2 comparison context and eligibility.
//!
//! S039-R1 completes the shared application path: the immutable prepared graph
//! is the only authority for the exact scenario, policy and selected-profile
//! definitions, and the comparison rule is an explicitly supplied approved
//! reference. No reload, "latest" lookup, clock capture, policy-digest
//! substitution or invented rule format occurs. The resulting
//! [`ComparisonContextDigest`] is a shared comparison identity only; exact
//! subject identity and any evaluator/trust conclusion remain separate.

use eqm_domain::{
    ComparisonContextDigest, ComparisonContextInputV2, ContractDigest, FinalizedWorkspaceGraph,
    PolicySelectionInputV2, Scenario, Sha256Digest,
};
use eqm_engine::{ComparisonEligibility, ComparisonParticipant, evaluate_comparison_eligibility};
use eqm_manifest::{CanonicalizationError, canonicalize_comparison_context_v2};
use eqm_protocol::schema_graph_v2::CheckedComparisonContextV2;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Session-graph comparison-context derivation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComparisonContextError {
    /// The supplied scenario is not an exact member of the immutable graph.
    ScenarioNotInGraph,
    /// The supplied policy is not an exact member of the immutable graph.
    PolicyNotInGraph,
    /// A selected profile is not an exact member of the immutable graph.
    ProfileNotInGraph,
    /// The comparison context could not be canonically projected.
    Canonicalize,
}

impl Display for ComparisonContextError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for ComparisonContextError {}

impl From<CanonicalizationError> for ComparisonContextError {
    fn from(_: CanonicalizationError) -> Self {
        Self::Canonicalize
    }
}

/// Derives the shared comparison-context identity from the immutable prepared
/// graph.
///
/// The exact scenario, selected policy and every selected profile must already
/// be members of `graph`; membership is compared by value, so a caller cannot
/// compare against an unloaded or substituted definition. The comparison rule
/// is an explicit approved reference, never a policy digest or invented default.
pub fn shared_comparison_context(
    graph: &FinalizedWorkspaceGraph,
    scenario: &Scenario,
    selection: &PolicySelectionInputV2<'_>,
    contract_digest: ContractDigest,
    rule_digest: Sha256Digest,
) -> Result<ComparisonContextDigest, ComparisonContextError> {
    let key = (scenario.id().clone(), scenario.revision());
    if graph.graph().scenarios().get(&key) != Some(scenario) {
        return Err(ComparisonContextError::ScenarioNotInGraph);
    }
    let policy = selection.policy();
    let policy_key = (policy.id().clone(), policy.revision());
    if graph.graph().policies().get(&policy_key) != Some(policy) {
        return Err(ComparisonContextError::PolicyNotInGraph);
    }
    for selected in selection.profiles() {
        let profile = selected.profile();
        let profile_key = (profile.id().clone(), profile.revision());
        if graph.graph().profiles().get(&profile_key) != Some(profile) {
            return Err(ComparisonContextError::ProfileNotInGraph);
        }
    }
    let input = ComparisonContextInputV2::new(contract_digest, scenario, selection, rule_digest);
    Ok(canonicalize_comparison_context_v2(&input)?.digest())
}

/// Evaluates eligibility from one checked comparison carrier and independently
/// matched participant coverage.
///
/// The typed [`ComparisonContextDigest`] comes from the checked carrier's own
/// semantic bytes; no raw digest string is cast or trusted.
#[must_use]
pub fn checked_comparison_eligibility(
    checked: &CheckedComparisonContextV2,
    participants: &[ComparisonParticipant],
) -> ComparisonEligibility {
    evaluate_comparison_eligibility(checked.digest(), participants)
}
