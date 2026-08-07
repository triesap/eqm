//! Protected-baseline policy monotonicity.

use eqm_domain::{Facet, FullRequirementId, Policy, RequirementLevel, RiskClass, Sha256Digest};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Comparable strength axes for one exact protected contract requirement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtectedRequirement {
    invariant_identity: Sha256Digest,
    level: RequirementLevel,
    risk: RiskClass,
    facets: BTreeSet<Facet>,
}

impl ProtectedRequirement {
    /// Creates a requirement summary. The identity covers all non-comparable semantics.
    pub fn new(
        invariant_identity: Sha256Digest,
        level: RequirementLevel,
        risk: RiskClass,
        facets: Vec<Facet>,
    ) -> Result<Self, MonotonicityError> {
        let count = facets.len();
        let facets: BTreeSet<_> = facets.into_iter().collect();
        if facets.is_empty() || facets.len() != count {
            return Err(MonotonicityError::InvalidPreparedInput);
        }
        Ok(Self {
            invariant_identity,
            level,
            risk,
            facets,
        })
    }

    fn strengthens_or_equals(&self, baseline: &Self) -> Result<bool, MonotonicityError> {
        if self.invariant_identity != baseline.invariant_identity {
            return Err(MonotonicityError::IncomparableAuthority);
        }
        Ok(self.level >= baseline.level
            && self.risk >= baseline.risk
            && self.facets.is_superset(&baseline.facets))
    }
}

/// Exact prepared contract, policy, runner, waiver-authority, and trust input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtectedPolicyInput<'a> {
    /// Selected policy authority.
    pub policy: &'a Policy,
    /// Contract requirements by full immutable identity.
    pub requirements: BTreeMap<FullRequirementId, ProtectedRequirement>,
    /// Immutable runner authority digests that remain available.
    pub runner_authorities: BTreeSet<Sha256Digest>,
    /// Principals permitted to approve waivers, represented by immutable digests.
    pub waiver_authorities: BTreeSet<Sha256Digest>,
    /// Required trust roots and algorithm controls.
    pub trust_controls: BTreeSet<Sha256Digest>,
}

/// Successful monotonic comparison classification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MonotonicChange {
    /// Every protected semantic axis is equal.
    Unchanged,
    /// At least one axis is strictly stronger and none is weaker.
    Strengthened,
}

/// Verifies that a candidate is no weaker than one exact protected baseline.
pub fn enforce_monotonic_policy(
    candidate: &ProtectedPolicyInput<'_>,
    baseline: &ProtectedPolicyInput<'_>,
) -> Result<MonotonicChange, MonotonicityError> {
    if candidate.policy.id() != baseline.policy.id() {
        return Err(MonotonicityError::IncomparableAuthority);
    }
    if !candidate
        .policy
        .profiles()
        .is_superset(baseline.policy.profiles())
        || !candidate
            .policy
            .required_targets()
            .is_superset(baseline.policy.required_targets())
        || !candidate
            .policy
            .waivers()
            .strengthens_or_equals(baseline.policy.waivers())
    {
        return Err(MonotonicityError::Weakening);
    }
    for baseline_rule in baseline.policy.rules() {
        if !candidate.policy.rules().iter().any(|candidate_rule| {
            candidate_rule.selector() == baseline_rule.selector()
                && candidate_rule.strengthens_or_equals(baseline_rule)
        }) {
            return Err(MonotonicityError::Weakening);
        }
    }
    for (id, baseline_requirement) in &baseline.requirements {
        let candidate_requirement = candidate
            .requirements
            .get(id)
            .ok_or(MonotonicityError::Weakening)?;
        if !candidate_requirement.strengthens_or_equals(baseline_requirement)? {
            return Err(MonotonicityError::Weakening);
        }
    }
    if !candidate
        .runner_authorities
        .is_superset(&baseline.runner_authorities)
        || !candidate
            .trust_controls
            .is_superset(&baseline.trust_controls)
        || !candidate
            .waiver_authorities
            .is_subset(&baseline.waiver_authorities)
    {
        return Err(MonotonicityError::Weakening);
    }
    if candidate == baseline {
        Ok(MonotonicChange::Unchanged)
    } else {
        Ok(MonotonicChange::Strengthened)
    }
}

/// Protected-baseline comparison failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MonotonicityError {
    /// Prepared input violated its closed invariants.
    InvalidPreparedInput,
    /// Candidate semantics replaced rather than ordered protected authority.
    IncomparableAuthority,
    /// At least one protected strength axis was weakened.
    Weakening,
}

impl Display for MonotonicityError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for MonotonicityError {}
