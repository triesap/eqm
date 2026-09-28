//! Same-session v2 policy/contract provenance assembly.
//!
//! This is the additive application entry point DEV-006 allocates: it takes one
//! [`PreparedSession`] plus the existing mode and explicit/development-default
//! [`PolicyProfileRequest`] inputs, runs the existing engine selection algorithm
//! against that session's immutable graph, resolves each exact selected profile
//! definition from the *same* graph, builds the validated domain carrier and
//! canonicalizes both the contract and the selected-policy projection.
//!
//! It never reselects a latest revision, reloads files, inspects the clock,
//! executes tools or falls back to the aggregate workspace digest. Missing
//! exact definitions or invalid selections return typed errors that retain their
//! causes. The workspace digest remains a separate aggregate context axis and
//! the v1 CLI/runner records are not switched to these identities.

use crate::session::PreparedSession;
use eqm_domain::{
    PolicySelectionInputError, PolicySelectionInputV2, ProfileId, SelectedDimensionStateV2,
    SelectedProfileInputV2, TargetId,
};
use eqm_engine::{EvaluationMode, PolicyProfileRequest, SelectionError, select_policy_profiles};
use eqm_manifest::{
    CanonicalContractV2, CanonicalPolicySelectionV2, CanonicalTargetConfigurationV2,
    CanonicalizationError, canonicalize_contract_v2, canonicalize_policy_selection_v2,
    canonicalize_target_configuration_v2,
};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Immutable typed provenance bundle for one prepared session and selection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyProvenanceV2 {
    contract: CanonicalContractV2,
    policy_selection: CanonicalPolicySelectionV2,
}

impl PolicyProvenanceV2 {
    /// Returns the exact v2 contract identity and bytes.
    #[must_use]
    pub const fn contract(&self) -> &CanonicalContractV2 {
        &self.contract
    }

    /// Returns the exact v2 selected-policy identity and bytes.
    #[must_use]
    pub const fn policy_selection(&self) -> &CanonicalPolicySelectionV2 {
        &self.policy_selection
    }
}

/// Typed v2 provenance assembly failure retaining its causal error.
#[derive(Debug)]
pub enum PolicyProvenanceError {
    /// Policy/profile selection failed under the existing engine rules.
    Selection(SelectionError),
    /// The exact selected profile definition was absent from the same graph.
    MissingProfileDefinition {
        /// The missing exact profile authority.
        profile: ProfileId,
    },
    /// The validated domain carrier rejected the selection.
    Carrier(PolicySelectionInputError),
    /// The contract projection failed.
    Contract(CanonicalizationError),
    /// The selected-policy projection failed.
    PolicySelection(CanonicalizationError),
}

impl Display for PolicyProvenanceError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Selection(error) => write!(formatter, "policy selection failed: {error}"),
            Self::MissingProfileDefinition { profile } => {
                write!(
                    formatter,
                    "selected profile definition is missing: {profile}"
                )
            }
            Self::Carrier(error) => {
                write!(
                    formatter,
                    "policy selection carrier rejected input: {error}"
                )
            }
            Self::Contract(error) => write!(formatter, "contract projection failed: {error}"),
            Self::PolicySelection(error) => {
                write!(formatter, "policy-selection projection failed: {error}")
            }
        }
    }
}

impl Error for PolicyProvenanceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Selection(error) => Some(error),
            Self::Carrier(error) => Some(error),
            Self::Contract(error) | Self::PolicySelection(error) => Some(error),
            Self::MissingProfileDefinition { .. } => None,
        }
    }
}

/// Assembles the typed v2 contract and selected-policy provenance from one
/// prepared session and the existing selection inputs.
pub fn prepare_policy_provenance(
    session: &PreparedSession,
    mode: EvaluationMode,
    explicit: Option<&PolicyProfileRequest>,
    development_default: Option<&PolicyProfileRequest>,
) -> Result<PolicyProvenanceV2, PolicyProvenanceError> {
    let graph = session.finalized().graph();
    let selected = select_policy_profiles(graph, mode, explicit, development_default)
        .map_err(PolicyProvenanceError::Selection)?;

    let mut profiles = Vec::with_capacity(selected.profiles().len());
    for selected_profile in selected.profiles().values() {
        let definition = graph
            .profiles()
            .get(&(selected_profile.id().clone(), selected_profile.revision()))
            .ok_or_else(|| PolicyProvenanceError::MissingProfileDefinition {
                profile: selected_profile.id().clone(),
            })?;
        let values = selected_profile
            .values()
            .iter()
            .map(|(dimension, value)| {
                (
                    dimension.clone(),
                    match value {
                        Some(value) => SelectedDimensionStateV2::Known(value.clone()),
                        None => SelectedDimensionStateV2::Unknown,
                    },
                )
            })
            .collect();
        profiles.push(
            SelectedProfileInputV2::new(definition, values)
                .map_err(PolicyProvenanceError::Carrier)?,
        );
    }

    let carrier = PolicySelectionInputV2::new(selected.policy(), profiles)
        .map_err(PolicyProvenanceError::Carrier)?;
    let contract =
        canonicalize_contract_v2(session.finalized()).map_err(PolicyProvenanceError::Contract)?;
    let policy_selection = canonicalize_policy_selection_v2(&carrier)
        .map_err(PolicyProvenanceError::PolicySelection)?;
    Ok(PolicyProvenanceV2 {
        contract,
        policy_selection,
    })
}

/// Typed target-configuration assembly failure retaining its causal error.
#[derive(Debug)]
pub enum TargetConfigurationError {
    /// The requested exact target was absent from this prepared graph.
    MissingTarget {
        /// The requested target ID.
        target: TargetId,
    },
    /// The target projection failed.
    Projection(CanonicalizationError),
}

impl Display for TargetConfigurationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingTarget { target } => {
                write!(
                    formatter,
                    "target is missing from the prepared graph: {target}"
                )
            }
            Self::Projection(error) => {
                write!(formatter, "target-configuration projection failed: {error}")
            }
        }
    }
}

impl Error for TargetConfigurationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Projection(error) => Some(error),
            Self::MissingTarget { .. } => None,
        }
    }
}

/// Returns the exact target-configuration identity for one target in the same
/// prepared session.
///
/// This looks up the requested exact [`TargetId`] in the session's immutable
/// finalized graph and invokes the pure manifest projection. It does not reload
/// the graph or files, run selection, read the clock or execute tools, and it
/// never substitutes an empty/default target or the aggregate workspace digest.
/// An unavailable target is an error. The returned identity is a
/// target-definition identity only; it does not establish a selected
/// policy/profile, source revision, resolved source-component content or build
/// artifact identity.
pub fn prepare_target_configuration(
    session: &PreparedSession,
    target: &TargetId,
) -> Result<CanonicalTargetConfigurationV2, TargetConfigurationError> {
    let graph = session.finalized().graph();
    let value =
        graph
            .targets()
            .get(target)
            .ok_or_else(|| TargetConfigurationError::MissingTarget {
                target: target.clone(),
            })?;
    canonicalize_target_configuration_v2(value).map_err(TargetConfigurationError::Projection)
}
