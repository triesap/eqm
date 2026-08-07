//! Pure graph resolution and evaluation for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod applicability;
mod conformance;
mod coverage;
mod equivalence;
mod expand;
mod exposure;
mod freshness;
mod invariants;
mod monotonicity;
mod obligations;
mod outcomes;
mod resolve;
mod selection;
mod structure;
mod waivers;

pub use applicability::{
    ApplicabilityContext, ApplicabilityError, TruthValue, evaluate_applicability,
};
pub use conformance::{
    ConformanceError, FacetEvaluationInput, FacetStatus, SupportingCheck, TargetConformance,
    TrustEvaluation, evaluate_facet_status, evaluate_target_conformance,
};
pub use coverage::{
    CoverageExpectation, CoverageMismatch, CoverageReport, CoverageStatus, EvidenceCandidate,
    evaluate_evidence_coverage,
};
pub use equivalence::{
    EquivalenceReport, EquivalenceStatus, TargetEvaluation, evaluate_target_set_equivalence,
};
pub use expand::{FragmentDigestMap, expand_fragments};
pub use exposure::{
    ConformanceFact, ExpectedExposure, ExposureComparison, ExposureFacts, ExposureReconciliation,
    ObservedExposure, reconcile_exposure,
};
pub use freshness::{
    FreshnessKey, FreshnessMismatch, FreshnessReport, FreshnessStatus, evaluate_evidence_freshness,
};
pub use invariants::validate_graph_invariants;
pub use monotonicity::{
    MonotonicChange, MonotonicityError, ProtectedPolicyInput, ProtectedRequirement,
    enforce_monotonic_policy,
};
pub use obligations::{
    Obligation, ObligationDerivation, ObligationError, ObligationKey, ObligationStrength,
    ScopeSubject, derive_obligations,
};
pub use outcomes::{EvidenceOutcome, aggregate_evidence_outcomes};
pub use resolve::{ResolutionError, resolution_diagnostics, resolve_graph};
pub use selection::{
    AuthorityOrigin, EvaluationMode, PolicyProfileRequest, PolicyRef, ProfileRequest,
    SelectedPolicyProfiles, SelectedProfile, SelectionError, matching_policy_rules,
    select_policy_profiles,
};
pub use structure::{
    RepositoryEntry, RepositoryEntryKind, RepositoryView, StructureFinding, StructureFindingKind,
    StructureReport, evaluate_structure,
};
pub use waivers::{WaivableStatus, WaiverEvaluation, WaiverInvalidReason, evaluate_waivers};
