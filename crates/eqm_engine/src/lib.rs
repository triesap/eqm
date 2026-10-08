//! Pure graph resolution and evaluation for EquivalenceMatrix.

#![forbid(unsafe_code)]

// Reuse the integration fixture loader inside private corruption controls.
#[cfg(test)]
extern crate self as eqm_engine;

mod affected;
mod applicability;
mod candidates;
mod comparison;
mod complete_coverage;
mod conformance;
mod coverage;
mod diagnostics;
mod diff;
mod equivalence;
mod expand;
mod exposure;
mod finalized;
mod freshness;
mod invariants;
mod matrix;
mod monotonicity;
mod obligations;
mod outcomes;
mod release;
mod resolve;
mod scope;
mod selection;
mod structure;
mod waivers;

pub use affected::{AffectedIndexes, AffectedSet, ChangedFile, analyze_affected_set};
pub use applicability::{
    ApplicabilityContext, ApplicabilityError, TruthValue, evaluate_applicability,
    evaluate_participation,
};
pub use candidates::{
    CandidateNormalizationError, EvidenceDeliveryReference, EvidenceNormalizationCandidate,
    ImmutableEvidenceAttempt, MAX_CANDIDATE_DELIVERIES, NormalizedCoverageReport,
    NormalizedEvidenceAssertion, NormalizedEvidenceAttempt, NormalizedEvidenceSet,
    evaluate_normalized_evidence_coverage, normalize_evidence_candidates,
};
pub use comparison::{
    ComparisonEligibility, ComparisonParticipant, evaluate_comparison_eligibility,
};
pub use complete_coverage::{
    CompleteCoverageError, CompleteCoverageExpectation, CompleteCoverageMismatch,
    CompleteCoverageReport, CompleteEvidenceCandidate, CoverageSubjectManifest,
    MAX_COMPLETE_COVERAGE_WORK, evaluate_complete_evidence_coverage,
};
pub use conformance::{
    CompletePolicyResolution, ConformanceError, EvaluatedFacets, FacetEvaluationInput, FacetStatus,
    SupportingCheck, TargetConformance, TrustEvaluation, evaluate_complete_target_conformance,
    evaluate_facet_status, evaluate_required_facets, evaluate_target_conformance,
};
pub use coverage::{
    CoverageExpectation, CoverageMismatch, CoverageReport, CoverageStatus, EvidenceCandidate,
    evaluate_evidence_coverage,
};
pub use diagnostics::{diagnostic_registry, explain_diagnostic};
pub use diff::{
    SemanticChange, SemanticChangeKind, SemanticCoordinate, SemanticFieldClass, SemanticProjection,
    SemanticValue, classify_diffs,
};
pub use equivalence::{
    EquivalenceReport, EquivalenceStatus, TargetEvaluation, evaluate_target_set_equivalence,
};
pub use expand::{FragmentDigestMap, expand_fragments};
pub use exposure::{
    ConformanceFact, ExpectedExposure, ExposureComparison, ExposureFacts, ExposureReconciliation,
    ObservedExposure, reconcile_exposure,
};
pub use finalized::FinalizedWorkspaceGraph;
pub use freshness::{
    FreshnessKey, FreshnessMismatch, FreshnessReport, FreshnessStatus, evaluate_evidence_freshness,
};
pub use invariants::validate_graph_invariants;
pub use matrix::{
    Matrix, MatrixAxis, MatrixAxisKey, MatrixCell, MatrixError, MatrixKind, MatrixStatus,
    MatrixValue, generate_matrix,
};
pub use monotonicity::{
    MonotonicChange, MonotonicityError, ProtectedPolicyInput, ProtectedRequirement,
    enforce_monotonic_policy,
};
pub use obligations::{
    Obligation, ObligationDerivation, ObligationError, ObligationKey, ObligationStrength,
    RequiredUniverse, ScopeSubject, UnresolvedCoordinate, UnresolvedReason, derive_obligations,
    derive_obligations_with_scope_bindings, derive_required_universe,
    derive_required_universe_with_scope_bindings, enforce_required_universe_strengthening,
};
pub use outcomes::{
    EvidenceOutcome, LogicalObservationError, LogicalObservationPrerequisite,
    LogicalObservationReport, LogicalObservationUnit, aggregate_evidence_outcomes,
    collect_logical_observations,
};
pub use release::{
    ReleaseCheck, ReleaseContext, ReleaseGateInput, ReleaseGateStatus, ReleaseSubject,
    evaluate_release_gate,
};
pub use resolve::{ResolutionError, resolution_diagnostics, resolve_graph};
pub use selection::{
    AuthorityOrigin, EvaluationMode, PolicyProfileRequest, PolicyRef, ProfileRequest,
    SelectedPolicyProfiles, SelectedProfile, SelectionError, matching_policy_rules,
    select_policy_profiles,
};
pub use structure::{
    RepositoryEntry, RepositoryEntryKind, RepositoryView, StructureFinding, StructureFindingKind,
    StructureReport, evaluate_structure, evaluate_structure_with_roots,
};
pub use waivers::{WaivableStatus, WaiverEvaluation, WaiverInvalidReason, evaluate_waivers};

pub use scope::{ProviderScope, ScenarioScope, ScopeBindingFailure, validate_scope_bindings};

mod expected_selection;
pub use expected_selection::{
    ExpectedSelectionBindingError, ExpectedSelectionMap, SelectionObservation,
    bind_expected_selections, evaluate_required_facets_with_expected_selections,
    expected_selection_coverage,
};
