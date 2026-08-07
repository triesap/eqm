//! Protected-baseline monotonicity fixtures.

use eqm_domain::{
    DurationMillis, Extensions, Facet, FullRequirementId, OwnerRef, Policy, PolicyId, PolicyRule,
    PolicySelector, PositiveCount, PositiveDays, ProfileId, RequirementLevel, Revision, RiskClass,
    Sha256Digest, TargetId, Title, TrustLevel, WaiverPolicy,
};
use eqm_engine::{
    MonotonicChange, MonotonicityError, ProtectedPolicyInput, ProtectedRequirement,
    enforce_monotonic_policy,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

fn digest(byte: u8) -> Sha256Digest {
    Sha256Digest::from_bytes([byte; 32])
}

fn policy(
    level: RequirementLevel,
    trust: TrustLevel,
    age: u64,
    count: u64,
    targets: Vec<&str>,
    waivers: WaiverPolicy,
) -> Result<Policy, Box<dyn Error>> {
    Ok(Policy::new(
        PolicyId::new("policy.protected")?,
        Revision::new(1)?,
        Title::new("Protected")?,
        vec!["owner://team/security".parse::<OwnerRef>()?],
        vec![ProfileId::new("audience.default")?],
        targets
            .into_iter()
            .map(TargetId::new)
            .collect::<Result<_, _>>()?,
        vec![PolicyRule::new(
            PolicySelector::new(None, None, Some(vec![RiskClass::High]), None, None)?,
            level,
            vec![Facet::Behavior],
            trust,
            DurationMillis::new(age)?,
            Some(PositiveCount::new(count)?),
        )?],
        waivers,
        None,
        Extensions::default(),
    )?)
}

fn requirement(
    identity: u8,
    level: RequirementLevel,
    risk: RiskClass,
    facets: Vec<Facet>,
) -> Result<BTreeMap<FullRequirementId, ProtectedRequirement>, Box<dyn Error>> {
    Ok(BTreeMap::from([(
        FullRequirementId::new("account.create.signup.form#submit")?,
        ProtectedRequirement::new(digest(identity), level, risk, facets)?,
    )]))
}

fn input<'a>(
    policy: &'a Policy,
    requirements: BTreeMap<FullRequirementId, ProtectedRequirement>,
) -> ProtectedPolicyInput<'a> {
    ProtectedPolicyInput {
        policy,
        requirements,
        runner_authorities: BTreeSet::from([digest(2)]),
        waiver_authorities: BTreeSet::from([digest(3)]),
        trust_controls: BTreeSet::from([digest(4)]),
    }
}

#[test]
fn equality_and_every_ordered_strengthening_axis_pass() -> Result<(), Box<dyn Error>> {
    let baseline_policy = policy(
        RequirementLevel::Recommended,
        TrustLevel::TrustedCi,
        86_400_000,
        1,
        vec!["web"],
        WaiverPolicy::new(true, Some(PositiveDays::new(30)?), None, vec![])?,
    )?;
    let candidate_policy = policy(
        RequirementLevel::Required,
        TrustLevel::SignedCi,
        3_600_000,
        2,
        vec!["ios", "web"],
        WaiverPolicy::new(
            true,
            Some(PositiveDays::new(7)?),
            Some(PositiveCount::new(2)?),
            vec![Facet::Behavior],
        )?,
    )?;
    let baseline = input(
        &baseline_policy,
        requirement(
            1,
            RequirementLevel::Recommended,
            RiskClass::Medium,
            vec![Facet::Behavior],
        )?,
    );
    assert_eq!(
        enforce_monotonic_policy(&baseline, &baseline)?,
        MonotonicChange::Unchanged
    );
    let mut candidate = input(
        &candidate_policy,
        requirement(
            1,
            RequirementLevel::Required,
            RiskClass::High,
            vec![Facet::Behavior, Facet::Accessibility],
        )?,
    );
    candidate.runner_authorities.insert(digest(5));
    candidate.trust_controls.insert(digest(6));
    candidate.waiver_authorities.clear();
    assert_eq!(
        enforce_monotonic_policy(&candidate, &baseline)?,
        MonotonicChange::Strengthened
    );
    Ok(())
}

#[test]
fn every_weakening_and_incomparable_replacement_fails_closed() -> Result<(), Box<dyn Error>> {
    let policy = policy(
        RequirementLevel::Required,
        TrustLevel::SignedCi,
        3_600_000,
        2,
        vec!["web"],
        WaiverPolicy::deny(),
    )?;
    let baseline = input(
        &policy,
        requirement(
            1,
            RequirementLevel::Required,
            RiskClass::High,
            vec![Facet::Behavior],
        )?,
    );
    let mut removed_requirement = baseline.clone();
    removed_requirement.requirements.clear();
    assert_eq!(
        enforce_monotonic_policy(&removed_requirement, &baseline),
        Err(MonotonicityError::Weakening)
    );
    let mut lower_risk = baseline.clone();
    lower_risk.requirements = requirement(
        1,
        RequirementLevel::Required,
        RiskClass::Medium,
        vec![Facet::Behavior],
    )?;
    assert_eq!(
        enforce_monotonic_policy(&lower_risk, &baseline),
        Err(MonotonicityError::Weakening)
    );
    let mut replaced = baseline.clone();
    replaced.requirements = requirement(
        9,
        RequirementLevel::Required,
        RiskClass::High,
        vec![Facet::Behavior],
    )?;
    assert_eq!(
        enforce_monotonic_policy(&replaced, &baseline),
        Err(MonotonicityError::IncomparableAuthority)
    );
    let mut runner_removed = baseline.clone();
    runner_removed.runner_authorities.clear();
    assert_eq!(
        enforce_monotonic_policy(&runner_removed, &baseline),
        Err(MonotonicityError::Weakening)
    );
    let mut trust_removed = baseline.clone();
    trust_removed.trust_controls.clear();
    assert_eq!(
        enforce_monotonic_policy(&trust_removed, &baseline),
        Err(MonotonicityError::Weakening)
    );
    let mut waiver_added = baseline.clone();
    waiver_added.waiver_authorities.insert(digest(8));
    assert_eq!(
        enforce_monotonic_policy(&waiver_added, &baseline),
        Err(MonotonicityError::Weakening)
    );
    Ok(())
}

#[test]
fn policy_rule_target_and_waiver_weakening_fail() -> Result<(), Box<dyn Error>> {
    let baseline_policy = policy(
        RequirementLevel::Required,
        TrustLevel::SignedCi,
        3_600_000,
        2,
        vec!["web"],
        WaiverPolicy::deny(),
    )?;
    let baseline = input(
        &baseline_policy,
        requirement(
            1,
            RequirementLevel::Required,
            RiskClass::High,
            vec![Facet::Behavior],
        )?,
    );
    let weaker_rule = policy(
        RequirementLevel::Recommended,
        TrustLevel::TrustedCi,
        86_400_000,
        1,
        vec!["web"],
        WaiverPolicy::deny(),
    )?;
    assert_eq!(
        enforce_monotonic_policy(
            &input(&weaker_rule, baseline.requirements.clone()),
            &baseline,
        ),
        Err(MonotonicityError::Weakening)
    );
    let missing_target = policy(
        RequirementLevel::Required,
        TrustLevel::SignedCi,
        3_600_000,
        2,
        vec!["ios"],
        WaiverPolicy::deny(),
    )?;
    assert_eq!(
        enforce_monotonic_policy(
            &input(&missing_target, baseline.requirements.clone()),
            &baseline,
        ),
        Err(MonotonicityError::Weakening)
    );
    let enabled_waiver = policy(
        RequirementLevel::Required,
        TrustLevel::SignedCi,
        3_600_000,
        2,
        vec!["web"],
        WaiverPolicy::new(true, Some(PositiveDays::new(30)?), None, vec![])?,
    )?;
    assert_eq!(
        enforce_monotonic_policy(
            &input(&enabled_waiver, baseline.requirements.clone()),
            &baseline,
        ),
        Err(MonotonicityError::Weakening)
    );
    Ok(())
}
