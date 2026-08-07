//! Pure graph resolution and evaluation for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod applicability;
mod expand;
mod invariants;
mod monotonicity;
mod resolve;
mod selection;

pub use applicability::{
    ApplicabilityContext, ApplicabilityError, TruthValue, evaluate_applicability,
};
pub use expand::{FragmentDigestMap, expand_fragments};
pub use invariants::validate_graph_invariants;
pub use monotonicity::{
    MonotonicChange, MonotonicityError, ProtectedPolicyInput, ProtectedRequirement,
    enforce_monotonic_policy,
};
pub use resolve::{ResolutionError, resolution_diagnostics, resolve_graph};
pub use selection::{
    AuthorityOrigin, EvaluationMode, PolicyProfileRequest, PolicyRef, ProfileRequest,
    SelectedPolicyProfiles, SelectedProfile, SelectionError, matching_policy_rules,
    select_policy_profiles,
};
