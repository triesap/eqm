//! Exact policy/profile selection over prepared authority.

use eqm_domain::{
    DimensionId, Facet, FullRequirementId, Policy, PolicyId, PolicyRule, ProfileId,
    RequirementScope, Revision, RiskClass, SymbolicValueId, UnitId, WorkspaceGraph,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Closed evaluation mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvaluationMode {
    /// Local development evaluation.
    Development,
    /// Trusted pull-request evaluation.
    PullRequest,
    /// Protected release evaluation.
    Release,
}

/// Authority boundary from which a prepared selection was obtained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthorityOrigin {
    /// Candidate repository authority, allowed only for development defaults.
    CandidateLocal,
    /// Trusted invocation authority, accepted for pull requests.
    TrustedInvocation,
    /// Protected baseline authority, required for releases.
    ProtectedBaseline,
}

/// Exact versioned policy reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyRef {
    id: PolicyId,
    revision: Revision,
}

impl PolicyRef {
    /// Creates an exact policy reference.
    #[must_use]
    pub const fn new(id: PolicyId, revision: Revision) -> Self {
        Self { id, revision }
    }
}

/// Exact versioned profile request and explicit known/unknown dimension values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileRequest {
    id: ProfileId,
    revision: Revision,
    values: BTreeMap<DimensionId, Option<SymbolicValueId>>,
}

impl ProfileRequest {
    /// Creates a profile request. Duplicate dimensions are rejected.
    pub fn new(
        id: ProfileId,
        revision: Revision,
        values: Vec<(DimensionId, Option<SymbolicValueId>)>,
    ) -> Result<Self, SelectionError> {
        let count = values.len();
        let values: BTreeMap<_, _> = values.into_iter().collect();
        if values.len() != count {
            return Err(SelectionError::DuplicateDimension);
        }
        Ok(Self {
            id,
            revision,
            values,
        })
    }
}

/// Prepared policy/profile choice from one explicit authority boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyProfileRequest {
    origin: AuthorityOrigin,
    policy: PolicyRef,
    profiles: Vec<ProfileRequest>,
}

impl PolicyProfileRequest {
    /// Creates a request and rejects duplicate exact profile authorities.
    pub fn new(
        origin: AuthorityOrigin,
        policy: PolicyRef,
        profiles: Vec<ProfileRequest>,
    ) -> Result<Self, SelectionError> {
        let mut keys = BTreeSet::new();
        if profiles
            .iter()
            .any(|profile| !keys.insert((profile.id.clone(), profile.revision)))
        {
            return Err(SelectionError::DuplicateProfile);
        }
        Ok(Self {
            origin,
            policy,
            profiles,
        })
    }
}

/// One validated profile with every declared dimension known or explicitly unknown.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SelectedProfile {
    id: ProfileId,
    revision: Revision,
    values: BTreeMap<DimensionId, Option<SymbolicValueId>>,
}

impl SelectedProfile {
    /// Returns the profile ID.
    #[must_use]
    pub const fn id(&self) -> &ProfileId {
        &self.id
    }
    /// Returns the exact revision.
    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }
    /// Returns every declared dimension in canonical order.
    #[must_use]
    pub const fn values(&self) -> &BTreeMap<DimensionId, Option<SymbolicValueId>> {
        &self.values
    }
}

/// Fully validated exact policy and profiles.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedPolicyProfiles<'a> {
    policy: &'a Policy,
    profiles: BTreeMap<ProfileId, SelectedProfile>,
}

impl<'a> SelectedPolicyProfiles<'a> {
    /// Returns selected policy authority.
    #[must_use]
    pub const fn policy(&self) -> &'a Policy {
        self.policy
    }
    /// Returns selected profiles by ID.
    #[must_use]
    pub const fn profiles(&self) -> &BTreeMap<ProfileId, SelectedProfile> {
        &self.profiles
    }
}

/// Selects and validates exact policy/profile authority for one evaluation mode.
pub fn select_policy_profiles<'a>(
    graph: &'a WorkspaceGraph,
    mode: EvaluationMode,
    explicit: Option<&PolicyProfileRequest>,
    development_default: Option<&PolicyProfileRequest>,
) -> Result<SelectedPolicyProfiles<'a>, SelectionError> {
    let request = match (mode, explicit) {
        (EvaluationMode::Development, Some(request)) => request,
        (EvaluationMode::Development, None) => {
            development_default.ok_or(SelectionError::SelectionRequired)?
        }
        (EvaluationMode::PullRequest | EvaluationMode::Release, Some(request)) => request,
        (EvaluationMode::PullRequest | EvaluationMode::Release, None) => {
            return Err(SelectionError::ExplicitSelectionRequired);
        }
    };
    match (mode, request.origin) {
        (EvaluationMode::Development, _) => {}
        (
            EvaluationMode::PullRequest,
            AuthorityOrigin::TrustedInvocation | AuthorityOrigin::ProtectedBaseline,
        ) => {}
        (EvaluationMode::Release, AuthorityOrigin::ProtectedBaseline) => {}
        (EvaluationMode::PullRequest | EvaluationMode::Release, _) => {
            return Err(SelectionError::UntrustedSelection);
        }
    }
    let policy = graph
        .policies()
        .get(&(request.policy.id.clone(), request.policy.revision))
        .ok_or(SelectionError::PolicyNotFound)?;
    let requested_ids: BTreeSet<_> = request
        .profiles
        .iter()
        .map(|profile| profile.id.clone())
        .collect();
    if requested_ids != *policy.profiles() || request.profiles.len() != requested_ids.len() {
        return Err(SelectionError::ProfileSetMismatch);
    }
    let mut profiles = BTreeMap::new();
    for requested in &request.profiles {
        let profile = graph
            .profiles()
            .get(&(requested.id.clone(), requested.revision))
            .ok_or(SelectionError::ProfileNotFound)?;
        if requested
            .values
            .keys()
            .any(|dimension| !profile.dimensions().contains_key(dimension))
        {
            return Err(SelectionError::UndeclaredDimension);
        }
        let mut values = BTreeMap::new();
        for (dimension, declaration) in profile.dimensions() {
            let selected = requested
                .values
                .get(dimension)
                .cloned()
                .or_else(|| {
                    (mode == EvaluationMode::Development)
                        .then(|| profile.defaults().get(dimension).cloned())
                        .flatten()
                        .map(Some)
                })
                .ok_or(SelectionError::DimensionSelectionRequired)?;
            if selected
                .as_ref()
                .is_some_and(|value| !declaration.values().contains(value))
            {
                return Err(SelectionError::UndeclaredValue);
            }
            values.insert(dimension.clone(), selected);
        }
        profiles.insert(
            requested.id.clone(),
            SelectedProfile {
                id: requested.id.clone(),
                revision: requested.revision,
                values,
            },
        );
    }
    Ok(SelectedPolicyProfiles { policy, profiles })
}

/// Returns policy rules whose every populated closed-selector axis matches.
#[must_use]
pub fn matching_policy_rules<'a>(
    policy: &'a Policy,
    unit: &UnitId,
    requirement: &FullRequirementId,
    risk: RiskClass,
    facet: Facet,
    scope: RequirementScope,
) -> Vec<&'a PolicyRule> {
    policy
        .rules()
        .iter()
        .filter(|rule| {
            let selector = rule.selector();
            selector.units().is_none_or(|values| values.contains(unit))
                && selector
                    .requirements()
                    .is_none_or(|values| values.contains(requirement))
                && selector
                    .risk_classes()
                    .is_none_or(|values| values.contains(&risk))
                && selector
                    .facets()
                    .is_none_or(|values| values.contains(&facet))
                && selector
                    .scopes()
                    .is_none_or(|values| values.contains(&scope))
        })
        .collect()
}

/// Policy/profile selection failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectionError {
    /// Development had neither an explicit selection nor a prepared default.
    SelectionRequired,
    /// A non-local mode omitted its explicit selection.
    ExplicitSelectionRequired,
    /// The selection came from an authority boundary too weak for the mode.
    UntrustedSelection,
    /// The exact policy authority was absent.
    PolicyNotFound,
    /// An exact profile authority was absent.
    ProfileNotFound,
    /// Requested profiles did not exactly equal the policy profile set.
    ProfileSetMismatch,
    /// A profile authority was repeated.
    DuplicateProfile,
    /// A dimension selection was repeated.
    DuplicateDimension,
    /// A selected dimension was not declared.
    UndeclaredDimension,
    /// A selected value was outside its finite declaration.
    UndeclaredValue,
    /// A non-defaulted dimension was omitted instead of known or explicit unknown.
    DimensionSelectionRequired,
}

impl Display for SelectionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for SelectionError {}
