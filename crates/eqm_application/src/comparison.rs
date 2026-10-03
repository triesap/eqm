//! Session-graph-backed v2 comparison context and checked eligibility.
//!
//! S039-R2 completes the shared application path: the immutable prepared graph
//! is the only authority for the exact scenario, policy, selected-profile and
//! contract definitions, and the comparison rule is an explicitly supplied
//! approved reference. The checked wire comparison context must equal the
//! independently derived graph context; a well-formed, self-selected context is
//! never accepted as expected authority. Expected occurrence coverage is the
//! selected graph scenario's exact occurrence set, and participants are built
//! only after the existing bytes-first subject matching against independently
//! resolved authority succeeds.
//!
//! S039-R3 binds the consumed resolved cohort to that same authority: the
//! carrier's exact retained value must equal the consuming graph's exact
//! ID/revision entry, name the selected scenario, and independently re-resolve
//! against the consuming graph to the same expected axes. Value equality, not
//! pointer identity, is authoritative, so independently prepared equivalent
//! copies may pass while foreign, wrong-scenario, altered or re-configured
//! authority is rejected with a typed cause before eligibility.
//!
//! The resulting [`ComparisonContextDigest`] is a shared comparison identity
//! only; exact subject identity and any evaluator/trust conclusion remain
//! separate.

use crate::cohort::{CohortError, ResolvedCohort, resolve_cohort};
use eqm_domain::{
    ComparisonContextDigest, ComparisonContextInputV2, FinalizedWorkspaceGraph,
    PolicySelectionInputV2, Scenario, Sha256Digest, SubjectManifest, SubjectManifestDigest,
};
use eqm_engine::{ComparisonEligibility, ComparisonParticipant, evaluate_comparison_eligibility};
use eqm_manifest::{
    CanonicalizationError, canonicalize_comparison_context_v2, canonicalize_contract_v2,
};
use eqm_protocol::schema_graph_v2::CheckedComparisonContextV2;
use eqm_protocol::v2::{
    CheckedSubjectManifestV2, SubjectMatchError, SubjectMatchMode, match_subjects,
};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Session-graph comparison-context derivation and checked-eligibility failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComparisonContextError {
    /// The supplied scenario is not an exact member of the immutable graph.
    ScenarioNotInGraph,
    /// The supplied policy is not an exact member of the immutable graph.
    PolicyNotInGraph,
    /// A selected profile is not an exact member of the immutable graph.
    ProfileNotInGraph,
    /// The comparison context could not be canonically projected.
    Canonicalize,
    /// The checked wire context is not the independently derived graph context.
    ContextNotBound,
    /// The matched candidate coverage differs from the exact scenario coverage.
    ScenarioCoverage,
    /// The retained resolved cohort belongs to a different scenario than the
    /// selected exact scenario.
    CohortScenario,
    /// The retained cohort value is absent from, or altered relative to, the
    /// consuming immutable graph's exact ID/revision entry.
    CohortNotInGraph,
    /// Re-resolving the retained cohort against the consuming graph failed.
    Cohort(CohortError),
    /// The independently re-derived expected axes differ from the carrier's
    /// axes, so the retained authority is not current for this graph.
    CohortRevalidation,
    /// The checked candidate subject manifest failed independent matching.
    SubjectMatch(SubjectMatchError),
}

impl Display for ComparisonContextError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for ComparisonContextError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::SubjectMatch(error) => Some(error),
            Self::Cohort(error) => Some(error),
            _ => None,
        }
    }
}

impl From<CanonicalizationError> for ComparisonContextError {
    fn from(_: CanonicalizationError) -> Self {
        Self::Canonicalize
    }
}

/// Derives the shared comparison-context identity from the immutable prepared
/// graph.
///
/// The contract identity is the graph's own frozen contract projection: a
/// caller-selected contract digest is never accepted. The exact scenario,
/// selected policy and every selected profile must already be members of
/// `graph`; membership is compared by value, so a caller cannot compare against
/// an unloaded or substituted definition. The comparison rule is an explicit
/// approved reference, never a policy digest or invented default.
pub fn shared_comparison_context(
    graph: &FinalizedWorkspaceGraph,
    scenario: &Scenario,
    selection: &PolicySelectionInputV2<'_>,
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
    let contract = canonicalize_contract_v2(graph)?.digest();
    let input = ComparisonContextInputV2::new(contract, scenario, selection, rule_digest);
    Ok(canonicalize_comparison_context_v2(&input)?.digest())
}

/// Immutable result of one checked comparison evaluation.
///
/// The matched [`SubjectManifest`] retains each occurrence's exact
/// kind/configuration/content authority; the candidate's separate
/// [`SubjectManifestDigest`] is retained alongside it. `eligibility` is only a
/// comparison-coverage conclusion, never conformance or trust.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedComparisonOutcome {
    matched: SubjectManifest,
    subject_manifest_digest: SubjectManifestDigest,
    context: ComparisonContextDigest,
    eligibility: ComparisonEligibility,
}

impl CheckedComparisonOutcome {
    /// Returns the independently matched subject manifest.
    #[must_use]
    pub const fn matched(&self) -> &SubjectManifest {
        &self.matched
    }

    /// Returns the checked candidate's own subject-manifest identity.
    #[must_use]
    pub const fn subject_manifest_digest(&self) -> SubjectManifestDigest {
        self.subject_manifest_digest
    }

    /// Returns the independently derived shared comparison identity.
    #[must_use]
    pub const fn context_digest(&self) -> ComparisonContextDigest {
        self.context
    }

    /// Returns the comparison-coverage eligibility conclusion.
    #[must_use]
    pub const fn eligibility(&self) -> ComparisonEligibility {
        self.eligibility
    }
}

/// Evaluates checked comparison eligibility at the real application boundary.
///
/// The checked wire context must equal the graph-derived expected context. The
/// retained resolved cohort is value authority: it must equal the consuming
/// graph's exact ID/revision entry and name the selected scenario, and
/// re-resolving it against that graph with the owning resolver must reproduce
/// the carrier's expected axes. Only then are the candidate bytes matched
/// against the independently resolved expected subjects; a successful match
/// must cover exactly the selected graph scenario's occurrences, and only after
/// that match succeeds are the immutable participants constructed, so a public
/// digest struct can never stand in for the matching proof.
pub fn evaluate_checked_comparison(
    graph: &FinalizedWorkspaceGraph,
    scenario: &Scenario,
    selection: &PolicySelectionInputV2<'_>,
    rule_digest: Sha256Digest,
    checked_context: &CheckedComparisonContextV2,
    resolved: &ResolvedCohort,
    candidate: &CheckedSubjectManifestV2,
) -> Result<CheckedComparisonOutcome, ComparisonContextError> {
    let expected = shared_comparison_context(graph, scenario, selection, rule_digest)?;
    if checked_context.digest() != expected {
        return Err(ComparisonContextError::ContextNotBound);
    }
    // The retained resolved cohort is value authority, not address identity:
    // it must name the selected exact scenario, equal the consuming graph's
    // exact ID/revision entry, and independently re-resolve against that graph
    // to the same expected axes before any candidate matching occurs. This
    // rejects foreign, wrong-scenario, altered same-ID/revision and changed
    // configuration authority instead of trusting occurrence-name equality.
    if resolved.scenario_id() != scenario.id()
        || resolved.scenario_revision() != scenario.revision()
    {
        return Err(ComparisonContextError::CohortScenario);
    }
    let retained_key = (resolved.cohort_id().clone(), resolved.cohort_revision());
    if graph.graph().cohorts().get(&retained_key) != Some(resolved.cohort()) {
        return Err(ComparisonContextError::CohortNotInGraph);
    }
    if resolve_cohort(graph, resolved.cohort())
        .map_err(ComparisonContextError::Cohort)?
        .expected()
        != resolved.expected()
    {
        return Err(ComparisonContextError::CohortRevalidation);
    }
    let scenario_occurrences: BTreeSet<eqm_domain::OccurrenceId> = scenario
        .participants()
        .iter()
        .map(|participant| participant.occurrence().clone())
        .collect();
    let matched = match_subjects(
        candidate,
        resolved.expected(),
        SubjectMatchMode::Completeness,
    )
    .map_err(ComparisonContextError::SubjectMatch)?;
    let matched_occurrences: BTreeSet<eqm_domain::OccurrenceId> = matched
        .assignments()
        .iter()
        .map(|assignment| assignment.occurrence().clone())
        .collect();
    if matched_occurrences != scenario_occurrences {
        return Err(ComparisonContextError::ScenarioCoverage);
    }
    let participants: Vec<ComparisonParticipant> = matched
        .assignments()
        .iter()
        .map(|assignment| ComparisonParticipant {
            occurrence: assignment.occurrence().clone(),
            context: Some(checked_context.digest()),
            subject: Some(candidate.digest()),
        })
        .collect();
    let eligibility =
        evaluate_comparison_eligibility(expected, &scenario_occurrences, &participants);
    Ok(CheckedComparisonOutcome {
        matched,
        subject_manifest_digest: candidate.digest(),
        context: expected,
        eligibility,
    })
}
