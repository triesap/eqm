//! Exact protected waiver validation and application.

use crate::{ObligationKey, ScopeSubject};
use eqm_domain::{CalendarDate, EvidenceScopeSubject, Facet, OwnerRef, Policy, Waiver, WaiverId};
use std::collections::{BTreeMap, BTreeSet};

/// Provisional evidence status supplied to waiver evaluation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WaivableStatus {
    /// Terminal failure.
    Failed,
    /// Stale evidence.
    Stale,
    /// Missing evidence.
    Missing,
    /// Unknown is never waivable.
    Unknown,
    /// Unstable is never waivable.
    Unstable,
    /// Satisfaction never needs and cannot be created by a waiver.
    Satisfied,
}

/// Closed reason that a waiver did not validate.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WaiverInvalidReason {
    /// The provisional status is not waivable.
    Status,
    /// Protected policy prohibits waivers.
    PolicyProhibits,
    /// Policy ID differs.
    Policy,
    /// Scope subject differs.
    Subject,
    /// Unit differs.
    Unit,
    /// Requirement differs.
    Requirement,
    /// Facet is outside the exact authorized set.
    Facet,
    /// Profile IDs or values differ.
    Profiles,
    /// Evaluation date is outside the half-open waiver window.
    Date,
    /// Waiver duration exceeds the policy ceiling.
    Duration,
    /// Approvers are insufficient or outside protected authority.
    Approvers,
    /// Required controls are absent from the waiver declaration.
    Controls,
    /// Required controls are not independently satisfied.
    UnsatisfiedControls,
}

/// Complete waiver application result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WaiverEvaluation {
    /// Exactly one valid waiver applies; the result remains visibly waived.
    Waived(WaiverId),
    /// No waiver applies, with deterministic per-waiver reasons.
    NotApplied(BTreeMap<WaiverId, BTreeSet<WaiverInvalidReason>>),
    /// More than one valid waiver matched the same obligation.
    Ambiguous(BTreeSet<WaiverId>),
}

/// Validates candidate waivers against exact obligation and protected authority.
#[must_use]
pub fn evaluate_waivers(
    waivers: &[&Waiver],
    policy: &Policy,
    obligation: &ObligationKey,
    status: WaivableStatus,
    evaluated_on: CalendarDate,
    protected_approvers: &BTreeSet<OwnerRef>,
    satisfied_controls: &BTreeSet<Facet>,
) -> WaiverEvaluation {
    let mut valid = Vec::new();
    let mut invalid = BTreeMap::new();
    for waiver in waivers {
        let reasons = invalid_reasons(
            waiver,
            policy,
            obligation,
            status,
            evaluated_on,
            protected_approvers,
            satisfied_controls,
        );
        if reasons.is_empty() {
            valid.push(waiver.id().clone());
        } else {
            invalid.insert(waiver.id().clone(), reasons);
        }
    }
    match valid.len() {
        0 => WaiverEvaluation::NotApplied(invalid),
        1 => WaiverEvaluation::Waived(valid.remove(0)),
        _ => WaiverEvaluation::Ambiguous(valid.into_iter().collect()),
    }
}

#[allow(clippy::too_many_arguments)]
fn invalid_reasons(
    waiver: &Waiver,
    policy: &Policy,
    obligation: &ObligationKey,
    status: WaivableStatus,
    evaluated_on: CalendarDate,
    protected_approvers: &BTreeSet<OwnerRef>,
    satisfied_controls: &BTreeSet<Facet>,
) -> BTreeSet<WaiverInvalidReason> {
    let mut reasons = BTreeSet::new();
    if !matches!(
        status,
        WaivableStatus::Failed | WaivableStatus::Stale | WaivableStatus::Missing
    ) {
        reasons.insert(WaiverInvalidReason::Status);
    }
    if !policy.waivers().allowed() {
        reasons.insert(WaiverInvalidReason::PolicyProhibits);
    }
    if waiver.policy() != policy.id() || obligation.policy != *policy.id() {
        reasons.insert(WaiverInvalidReason::Policy);
    }
    let scope = waiver.scope();
    if scope.target() != &evidence_subject(&obligation.subject) {
        reasons.insert(WaiverInvalidReason::Subject);
    }
    if scope.unit() != &obligation.unit {
        reasons.insert(WaiverInvalidReason::Unit);
    }
    if scope.requirement() != &obligation.requirement {
        reasons.insert(WaiverInvalidReason::Requirement);
    }
    if !scope.facets().contains(&obligation.facet) {
        reasons.insert(WaiverInvalidReason::Facet);
    }
    let profiles_match = scope.profiles().len() == obligation.profiles.len()
        && scope.profiles().iter().all(|(id, waiver_profile)| {
            obligation.profiles.get(id).is_some_and(|selected| {
                selected.values().len() == waiver_profile.values().len()
                    && selected.values().iter().all(|(dimension, value)| {
                        value.as_ref() == waiver_profile.values().get(dimension)
                    })
            })
        });
    if !profiles_match {
        reasons.insert(WaiverInvalidReason::Profiles);
    }
    if !waiver.is_active_on(evaluated_on) {
        reasons.insert(WaiverInvalidReason::Date);
    }
    if policy.waivers().maximum_days().is_none_or(|maximum| {
        waiver
            .starts_on()
            .days_until(waiver.expires_on())
            .is_none_or(|days| days > u64::from(maximum.get()))
    }) {
        reasons.insert(WaiverInvalidReason::Duration);
    }
    if u64::try_from(waiver.approvers().len()).map_or(true, |count| {
        count < policy.waivers().minimum_approvers().get()
    }) || !waiver.approvers().is_subset(protected_approvers)
    {
        reasons.insert(WaiverInvalidReason::Approvers);
    }
    if !waiver
        .controls()
        .is_superset(policy.waivers().required_controls())
    {
        reasons.insert(WaiverInvalidReason::Controls);
    }
    if !satisfied_controls.is_superset(policy.waivers().required_controls()) {
        reasons.insert(WaiverInvalidReason::UnsatisfiedControls);
    }
    reasons
}

fn evidence_subject(subject: &ScopeSubject) -> EvidenceScopeSubject {
    match subject {
        ScopeSubject::Target(target) => EvidenceScopeSubject::Target(target.clone()),
        ScopeSubject::Provider(provider) => EvidenceScopeSubject::Provider(provider.clone()),
        ScopeSubject::TargetSet(targets) => EvidenceScopeSubject::TargetSet(targets.clone()),
    }
}
