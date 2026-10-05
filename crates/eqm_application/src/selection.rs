//! Application-owned exact policy/profile selection and obligation preparation.
//!
//! This module is the reusable operation the CLI, MCP, or a future native
//! transport calls: it takes one immutable finalized graph, an explicit
//! evaluation mode, and explicit versus development-default
//! [`PolicyProfileRequest`]s from an explicit authority origin, then runs the
//! engine's exact selection, builds the validated applicability context, and
//! derives the complete obligation universe. It never inspects the calling
//! command, reselects a latest revision, reloads files, reads the clock, or
//! executes a runner. Host verification stays a separate capability port.

use eqm_domain::{FinalizedWorkspaceGraph, ProfileId, Sha256Digest, WorkspaceGraph};
use eqm_engine::{
    ApplicabilityContext, ApplicabilityError, EvaluationMode, ObligationDerivation,
    ObligationError, PolicyProfileRequest, RequiredUniverse, SelectedPolicyProfiles,
    SelectionError, derive_obligations, derive_required_universe, select_policy_profiles,
};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// One prepared selection with its applicability context and derived universe.
///
/// The selection borrows the exact policy/profile definitions from the same
/// immutable finalized graph the operation was given.
pub struct PreparedObligations<'a> {
    selection: SelectedPolicyProfiles<'a>,
    applicability: ApplicabilityContext,
    derivation: ObligationDerivation,
    universe: RequiredUniverse,
}

impl<'a> PreparedObligations<'a> {
    /// Returns the exact selected policy and profiles.
    #[must_use]
    pub const fn selection(&self) -> &SelectedPolicyProfiles<'a> {
        &self.selection
    }

    /// Returns the validated applicability context.
    #[must_use]
    pub const fn applicability(&self) -> &ApplicabilityContext {
        &self.applicability
    }

    /// Returns the derived obligation universe.
    #[must_use]
    pub const fn derivation(&self) -> &ObligationDerivation {
        &self.derivation
    }

    /// Returns the complete pre-evidence required universe.
    #[must_use]
    pub const fn universe(&self) -> &RequiredUniverse {
        &self.universe
    }

    /// Consumes the bundle into its exact parts.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        SelectedPolicyProfiles<'a>,
        ApplicabilityContext,
        ObligationDerivation,
        RequiredUniverse,
    ) {
        (
            self.selection,
            self.applicability,
            self.derivation,
            self.universe,
        )
    }
}

/// Selection and obligation preparation failure retaining its causal error.
#[derive(Debug)]
pub enum ObligationPreparationError {
    /// Policy/profile selection failed under the existing engine rules.
    Selection(SelectionError),
    /// The applicability context could not be constructed.
    Applicability(ApplicabilityError),
    /// Obligation derivation failed.
    Obligations(ObligationError),
    /// A selected exact profile definition was absent from the same graph.
    MissingProfileDefinition {
        /// The missing exact profile authority.
        profile: ProfileId,
    },
}

impl Display for ObligationPreparationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Selection(error) => write!(formatter, "policy selection failed: {error}"),
            Self::Applicability(error) => {
                write!(formatter, "applicability context failed: {error}")
            }
            Self::Obligations(error) => write!(formatter, "obligation derivation failed: {error}"),
            Self::MissingProfileDefinition { profile } => {
                write!(
                    formatter,
                    "selected profile definition is missing: {profile}"
                )
            }
        }
    }
}

impl Error for ObligationPreparationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Selection(error) => Some(error),
            Self::Applicability(error) => Some(error),
            Self::Obligations(error) => Some(error),
            Self::MissingProfileDefinition { .. } => None,
        }
    }
}

/// Prepares the exact selected policy/profiles, applicability context, and
/// derived obligations for one explicit evaluation mode and authority input.
///
/// A protected mode never accepts the development default: the engine rejects a
/// missing explicit selection and any authority origin too weak for the mode,
/// so this operation cannot derive protected authority from the caller's name.
pub fn prepare_obligations<'a>(
    finalized: &'a FinalizedWorkspaceGraph,
    mode: EvaluationMode,
    explicit: Option<&PolicyProfileRequest>,
    development_default: Option<&PolicyProfileRequest>,
    release_context: Option<Sha256Digest>,
) -> Result<PreparedObligations<'a>, ObligationPreparationError> {
    let graph = finalized.graph();
    let selection = select_policy_profiles(graph, mode, explicit, development_default)
        .map_err(ObligationPreparationError::Selection)?;
    let applicability = applicability_context(graph, &selection)?;
    let derivation = derive_obligations(finalized, &selection, &applicability, release_context)
        .map_err(ObligationPreparationError::Obligations)?;
    let universe = derive_required_universe(finalized, &selection, &applicability, release_context)
        .map_err(ObligationPreparationError::Obligations)?;
    Ok(PreparedObligations {
        selection,
        applicability,
        derivation,
        universe,
    })
}

/// Builds the complete applicability context for one exact selection from the
/// same immutable graph, rejecting a missing exact profile definition.
pub fn applicability_context(
    graph: &WorkspaceGraph,
    selection: &SelectedPolicyProfiles<'_>,
) -> Result<ApplicabilityContext, ObligationPreparationError> {
    let mut inputs = Vec::with_capacity(selection.profiles().len());
    for selected in selection.profiles().values() {
        let definition = graph
            .profiles()
            .get(&(selected.id().clone(), selected.revision()))
            .ok_or_else(|| ObligationPreparationError::MissingProfileDefinition {
                profile: selected.id().clone(),
            })?;
        inputs.push((definition, selected.values()));
    }
    ApplicabilityContext::from_profiles(inputs).map_err(ObligationPreparationError::Applicability)
}
