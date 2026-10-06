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

use eqm_domain::{
    DimensionId, PolicyId, ProfileId, Revision, Sha256Digest, SymbolicValueId, WorkspaceGraph,
};
use eqm_engine::{
    ApplicabilityContext, ApplicabilityError, AuthorityOrigin, EvaluationMode,
    FinalizedWorkspaceGraph, ObligationDerivation, ObligationError, PolicyProfileRequest,
    PolicyRef, ProfileRequest, RequiredUniverse, SelectedPolicyProfiles, SelectionError,
    derive_obligations, derive_required_universe, select_policy_profiles,
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

/// Transport-parsed exact policy reference with an optional exact revision.
///
/// The adapter supplies only the syntax it received; resolving a unique stored
/// revision, rejecting an ambiguous one and checking membership belongs to
/// [`resolve_selection_request`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyReference {
    id: PolicyId,
    revision: Option<Revision>,
}

impl PolicyReference {
    /// Creates one typed policy reference with an optional exact revision.
    #[must_use]
    pub const fn new(id: PolicyId, revision: Option<Revision>) -> Self {
        Self { id, revision }
    }

    /// Returns the exact policy ID.
    #[must_use]
    pub const fn id(&self) -> &PolicyId {
        &self.id
    }

    /// Returns the explicit exact revision, if any.
    #[must_use]
    pub const fn revision(&self) -> Option<Revision> {
        self.revision
    }
}

/// Transport-parsed profile selection with an optional exact revision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileSelection {
    id: ProfileId,
    revision: Option<Revision>,
    values: Vec<(DimensionId, Option<SymbolicValueId>)>,
}

impl ProfileSelection {
    /// Creates one typed profile selection.
    #[must_use]
    pub const fn new(
        id: ProfileId,
        revision: Option<Revision>,
        values: Vec<(DimensionId, Option<SymbolicValueId>)>,
    ) -> Self {
        Self {
            id,
            revision,
            values,
        }
    }

    /// Returns the exact profile ID.
    #[must_use]
    pub const fn id(&self) -> &ProfileId {
        &self.id
    }

    /// Returns the explicit exact revision, if any.
    #[must_use]
    pub const fn revision(&self) -> Option<Revision> {
        self.revision
    }

    /// Returns the explicit known/unknown dimension assignments.
    #[must_use]
    pub fn values(&self) -> &[(DimensionId, Option<SymbolicValueId>)] {
        &self.values
    }
}

/// One typed, transport-independent request for exact selection authority.
///
/// Adapters parse their own syntax into this shape; the shared application
/// operation resolves exact revisions, rejects ambiguity and enforces policy
/// membership against the one immutable session it is given. It never reloads
/// the workspace, selects a latest revision, reads the clock or runs anything.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicySelectionRequest {
    origin: AuthorityOrigin,
    policy: Option<PolicyReference>,
    profiles: Vec<ProfileSelection>,
}

impl PolicySelectionRequest {
    /// Creates one explicit typed selection request.
    #[must_use]
    pub const fn new(
        origin: AuthorityOrigin,
        policy: Option<PolicyReference>,
        profiles: Vec<ProfileSelection>,
    ) -> Self {
        Self {
            origin,
            policy,
            profiles,
        }
    }

    /// Creates a candidate-local development request.
    #[must_use]
    pub const fn development(
        policy: Option<PolicyReference>,
        profiles: Vec<ProfileSelection>,
    ) -> Self {
        Self::new(AuthorityOrigin::CandidateLocal, policy, profiles)
    }

    /// Returns the explicit authority origin.
    #[must_use]
    pub const fn origin(&self) -> AuthorityOrigin {
        self.origin
    }

    /// Returns the requested policy reference, if any.
    #[must_use]
    pub const fn policy(&self) -> Option<&PolicyReference> {
        self.policy.as_ref()
    }

    /// Returns every requested profile selection in transport order.
    #[must_use]
    pub fn profiles(&self) -> &[ProfileSelection] {
        &self.profiles
    }
}

/// Exact selection preparation failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SelectionPreparationError {
    /// No policy was named and no unique stored policy ID exists.
    PolicyIdRequired,
    /// The exact policy revision was absent.
    PolicyNotFound {
        /// The exact policy ID.
        id: PolicyId,
        /// The explicit revision, if one was supplied.
        revision: Option<Revision>,
    },
    /// A protected mode omitted the exact policy revision it must name.
    PolicyRevisionRequired {
        /// The policy ID whose revision must be explicit.
        id: PolicyId,
    },
    /// A protected mode omitted the exact profile revision it must name.
    ProfileRevisionRequired {
        /// The profile ID whose revision must be explicit.
        id: ProfileId,
    },
    /// A protected mode omitted a profile the resolved policy declares.
    ProfileSelectionRequired {
        /// The omitted profile authority.
        id: ProfileId,
    },
    /// More than one stored revision exists for the policy ID.
    PolicyAmbiguous {
        /// The ambiguous policy ID.
        id: PolicyId,
    },
    /// A selected profile is not declared by the resolved policy.
    ProfileNotInPolicy {
        /// The offending profile authority.
        profile: ProfileId,
    },
    /// A profile was selected more than once, even identically.
    DuplicateProfile {
        /// The repeated profile authority.
        profile: ProfileId,
    },
    /// The exact profile revision was absent.
    ProfileNotFound {
        /// The exact profile ID.
        id: ProfileId,
        /// The explicit revision, if one was supplied.
        revision: Option<Revision>,
    },
    /// More than one stored revision exists for the profile ID.
    ProfileAmbiguous {
        /// The ambiguous profile ID.
        id: ProfileId,
    },
    /// The validated request shape was rejected by the engine.
    Request(SelectionError),
}

impl Display for SelectionPreparationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::PolicyIdRequired => {
                formatter.write_str("exactly one policy ID is required; select a policy explicitly")
            }
            Self::PolicyNotFound { id, revision } => match revision {
                Some(revision) => write!(formatter, "policy `{id}@{revision}` was not found"),
                None => write!(formatter, "policy `{id}` was not found"),
            },
            Self::PolicyRevisionRequired { id } => write!(
                formatter,
                "policy `{id}` must name an explicit exact revision in this mode"
            ),
            Self::ProfileRevisionRequired { id } => write!(
                formatter,
                "profile `{id}` must name an explicit exact revision in this mode"
            ),
            Self::ProfileSelectionRequired { id } => write!(
                formatter,
                "profile `{id}` must be explicitly selected in this mode"
            ),
            Self::PolicyAmbiguous { id } => write!(
                formatter,
                "policy `{id}` has multiple revisions; select one explicitly"
            ),
            Self::ProfileNotInPolicy { profile } => write!(
                formatter,
                "profile `{profile}` is outside the resolved policy profile set"
            ),
            Self::DuplicateProfile { profile } => {
                write!(formatter, "profile `{profile}` was selected more than once")
            }
            Self::ProfileNotFound { id, revision } => match revision {
                Some(revision) => write!(formatter, "profile `{id}@{revision}` was not found"),
                None => write!(formatter, "profile `{id}` was not found"),
            },
            Self::ProfileAmbiguous { id } => write!(
                formatter,
                "profile `{id}` has multiple revisions; select one explicitly"
            ),
            Self::Request(error) => write!(formatter, "selection request failed: {error}"),
        }
    }
}

impl Error for SelectionPreparationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Request(error) => Some(error),
            _ => None,
        }
    }
}

impl From<SelectionError> for SelectionPreparationError {
    fn from(value: SelectionError) -> Self {
        Self::Request(value)
    }
}

/// Resolves one typed selection request against the exact immutable session.
///
/// The exact policy is the supplied reference or, when absent, the unique stored
/// policy ID. An omitted revision resolves only when exactly one stored revision
/// exists; otherwise the request is an explicit missing or ambiguity error.
/// Development may use that unique/default candidate lookup; PullRequest and
/// Release require an explicit exact policy and explicit exact profile
/// revisions instead of filling them, so protected authority never comes from
/// an omitted input. Every profile in the resolved policy set is filled from
/// its unique stored revision unless the request names an exact revision, and a
/// profile outside that set rejects. The same immutable graph is used for every
/// lookup, so distinct sessions or contexts cannot be mixed.
pub fn resolve_selection_request(
    finalized: &FinalizedWorkspaceGraph,
    mode: EvaluationMode,
    request: &PolicySelectionRequest,
) -> Result<PolicyProfileRequest, SelectionPreparationError> {
    let graph = finalized.graph();
    let mut seen = std::collections::BTreeSet::new();
    for profile in &request.profiles {
        if !seen.insert(profile.id.clone()) {
            return Err(SelectionPreparationError::DuplicateProfile {
                profile: profile.id.clone(),
            });
        }
    }
    let (policy_id, policy_revision) = match &request.policy {
        Some(reference) => {
            let revision = match reference.revision {
                Some(revision) => {
                    if !graph
                        .policies()
                        .contains_key(&(reference.id.clone(), revision))
                    {
                        return Err(SelectionPreparationError::PolicyNotFound {
                            id: reference.id.clone(),
                            revision: Some(revision),
                        });
                    }
                    revision
                }
                None => {
                    if mode != EvaluationMode::Development {
                        return Err(SelectionPreparationError::PolicyRevisionRequired {
                            id: reference.id.clone(),
                        });
                    }
                    resolve_omitted_revision(
                        graph
                            .policies()
                            .keys()
                            .filter(|(id, _)| id == &reference.id)
                            .map(|(_, revision)| *revision),
                    )
                    .map_err(|cause| match cause {
                        RevisionCause::Missing => SelectionPreparationError::PolicyNotFound {
                            id: reference.id.clone(),
                            revision: None,
                        },
                        RevisionCause::Ambiguous => SelectionPreparationError::PolicyAmbiguous {
                            id: reference.id.clone(),
                        },
                    })?
                }
            };
            (reference.id.clone(), revision)
        }
        None => {
            if mode != EvaluationMode::Development {
                return Err(SelectionPreparationError::PolicyIdRequired);
            }
            let ids: std::collections::BTreeSet<&PolicyId> =
                graph.policies().keys().map(|(id, _)| id).collect();
            if ids.len() != 1 {
                return Err(SelectionPreparationError::PolicyIdRequired);
            }
            let id = ids
                .into_iter()
                .next()
                .ok_or(SelectionPreparationError::PolicyIdRequired)?
                .clone();
            let revision = resolve_omitted_revision(
                graph
                    .policies()
                    .keys()
                    .filter(|(candidate, _)| candidate == &id)
                    .map(|(_, revision)| *revision),
            )
            .map_err(|cause| match cause {
                RevisionCause::Missing => SelectionPreparationError::PolicyNotFound {
                    id: id.clone(),
                    revision: None,
                },
                RevisionCause::Ambiguous => {
                    SelectionPreparationError::PolicyAmbiguous { id: id.clone() }
                }
            })?;
            (id, revision)
        }
    };
    let policy = graph
        .policies()
        .get(&(policy_id.clone(), policy_revision))
        .ok_or_else(|| SelectionPreparationError::PolicyNotFound {
            id: policy_id.clone(),
            revision: Some(policy_revision),
        })?;
    let selected: std::collections::BTreeMap<&ProfileId, &ProfileSelection> = request
        .profiles
        .iter()
        .map(|profile| (&profile.id, profile))
        .collect();
    for profile in selected.keys() {
        if !policy.profiles().contains(*profile) {
            return Err(SelectionPreparationError::ProfileNotInPolicy {
                profile: (*profile).clone(),
            });
        }
    }
    let mut profiles = Vec::with_capacity(policy.profiles().len());
    for id in policy.profiles() {
        let revision = match selected.get(id) {
            Some(selection) => match selection.revision {
                Some(revision) => revision,
                None => {
                    if mode != EvaluationMode::Development {
                        return Err(SelectionPreparationError::ProfileRevisionRequired {
                            id: id.clone(),
                        });
                    }
                    resolve_omitted_revision(
                        graph
                            .profiles()
                            .keys()
                            .filter(|(candidate, _)| candidate == id)
                            .map(|(_, revision)| *revision),
                    )
                    .map_err(|cause| match cause {
                        RevisionCause::Missing => SelectionPreparationError::ProfileNotFound {
                            id: id.clone(),
                            revision: None,
                        },
                        RevisionCause::Ambiguous => {
                            SelectionPreparationError::ProfileAmbiguous { id: id.clone() }
                        }
                    })?
                }
            },
            None => {
                if mode != EvaluationMode::Development {
                    return Err(SelectionPreparationError::ProfileSelectionRequired {
                        id: id.clone(),
                    });
                }
                resolve_omitted_revision(
                    graph
                        .profiles()
                        .keys()
                        .filter(|(candidate, _)| candidate == id)
                        .map(|(_, revision)| *revision),
                )
                .map_err(|cause| match cause {
                    RevisionCause::Missing => SelectionPreparationError::ProfileNotFound {
                        id: id.clone(),
                        revision: None,
                    },
                    RevisionCause::Ambiguous => {
                        SelectionPreparationError::ProfileAmbiguous { id: id.clone() }
                    }
                })?
            }
        };
        if !graph.profiles().contains_key(&(id.clone(), revision)) {
            return Err(SelectionPreparationError::ProfileNotFound {
                id: id.clone(),
                revision: Some(revision),
            });
        }
        let values = selected
            .get(id)
            .map(|selection| selection.values.clone())
            .unwrap_or_default();
        profiles.push(ProfileRequest::new(id.clone(), revision, values)?);
    }
    Ok(PolicyProfileRequest::new(
        request.origin,
        PolicyRef::new(policy_id, policy_revision),
        profiles,
    )?)
}

/// Why an omitted exact revision could not be resolved.
enum RevisionCause {
    /// No stored revision exists for the authority.
    Missing,
    /// More than one stored revision exists for the authority.
    Ambiguous,
}

fn resolve_omitted_revision(
    revisions: impl Iterator<Item = Revision>,
) -> Result<Revision, RevisionCause> {
    let revisions: Vec<_> = revisions.collect();
    match revisions.as_slice() {
        [revision] => Ok(*revision),
        [] => Err(RevisionCause::Missing),
        _ => Err(RevisionCause::Ambiguous),
    }
}

/// Selects and derives obligations from one typed selection request.
pub fn prepare_obligations_from_request<'a>(
    finalized: &'a FinalizedWorkspaceGraph,
    mode: EvaluationMode,
    request: &PolicySelectionRequest,
    release_context: Option<Sha256Digest>,
) -> Result<PreparedObligations<'a>, ObligationPreparationError> {
    let resolved = resolve_selection_request(finalized, mode, request)
        .map_err(ObligationPreparationError::SelectionPreparation)?;
    prepare_obligations(finalized, mode, Some(&resolved), None, release_context)
}

/// Selection and obligation preparation failure retaining its causal error.
#[derive(Debug)]
pub enum ObligationPreparationError {
    /// Typed selection request resolution failed.
    SelectionPreparation(SelectionPreparationError),
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
            Self::SelectionPreparation(error) => {
                write!(formatter, "selection request preparation failed: {error}")
            }
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
            Self::SelectionPreparation(error) => Some(error),
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
