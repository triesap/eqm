//! S036: participation applicability per exact requirement/participant/profile.

use eqm_domain::{
    Applicability, ComparisonOperator, DimensionId, Extensions, FullRequirementId, OwnerRef,
    ParticipationRule, ParticipationRuleId, Profile, ProfileDimension, ProfileId, Revision,
    SymbolicValueId, TargetId, Title,
};
use eqm_engine::{ApplicabilityContext, TruthValue, evaluate_participation};
use std::collections::BTreeMap;
use std::error::Error;
use std::str::FromStr;

fn profile() -> Result<Profile, Box<dyn Error>> {
    Ok(Profile::new(
        ProfileId::new("audience.default")?,
        Revision::new(1)?,
        Title::new("Audience")?,
        vec![OwnerRef::from_str("owner://team/product")?],
        vec![ProfileDimension::new(
            DimensionId::new("region")?,
            vec![SymbolicValueId::new("eu")?, SymbolicValueId::new("us")?],
            None,
        )?],
        Vec::new(),
        None,
        Extensions::default(),
    )?)
}

fn context(region: Option<&str>) -> Result<ApplicabilityContext, Box<dyn Error>> {
    Ok(ApplicabilityContext::new(
        &profile()?,
        BTreeMap::from([(
            DimensionId::new("region")?,
            region.map(SymbolicValueId::new).transpose()?,
        )]),
    )?)
}

fn rules() -> Result<BTreeMap<ParticipationRuleId, ParticipationRule>, Box<dyn Error>> {
    let rule = ParticipationRule::new(
        ParticipationRuleId::new("participation.edge")?,
        FullRequirementId::new("account.create#edge_only")?,
        vec![TargetId::new("edge")?],
        Applicability::compare(
            DimensionId::new("region")?,
            ComparisonOperator::Equal,
            SymbolicValueId::new("eu")?,
        ),
        Extensions::default(),
    )?;
    Ok(BTreeMap::from([(rule.id().clone(), rule)]))
}

#[test]
fn participation_is_true_false_and_unknown_per_exact_coordinate() -> Result<(), Box<dyn Error>> {
    let requirement = FullRequirementId::new("account.create#edge_only")?;
    let edge = TargetId::new("edge")?;
    assert_eq!(
        evaluate_participation(&rules()?, &requirement, &edge, &context(Some("eu"))?)?,
        TruthValue::True
    );
    assert_eq!(
        evaluate_participation(&rules()?, &requirement, &edge, &context(Some("us"))?)?,
        TruthValue::False
    );
    // A missing required profile value stays unknown, never a default.
    assert_eq!(
        evaluate_participation(&rules()?, &requirement, &edge, &context(None)?)?,
        TruthValue::Unknown
    );
    Ok(())
}

#[test]
fn unspecified_requirement_target_and_rule_stay_unknown() -> Result<(), Box<dyn Error>> {
    let edge = TargetId::new("edge")?;
    // No rule for this exact requirement.
    assert_eq!(
        evaluate_participation(
            &rules()?,
            &FullRequirementId::new("account.create#other")?,
            &edge,
            &context(Some("eu"))?
        )?,
        TruthValue::Unknown
    );
    // No rule for this exact participant: no pre-fanout conclusion leaks.
    assert_eq!(
        evaluate_participation(
            &rules()?,
            &FullRequirementId::new("account.create#edge_only")?,
            &TargetId::new("tower")?,
            &context(Some("eu"))?
        )?,
        TruthValue::Unknown
    );
    Ok(())
}

#[test]
fn distinct_participants_evaluate_independently() -> Result<(), Box<dyn Error>> {
    let rule = ParticipationRule::new(
        ParticipationRuleId::new("participation.mobile")?,
        FullRequirementId::new("capture.photo#shutter")?,
        vec![TargetId::new("android")?, TargetId::new("ios")?],
        Applicability::always(true),
        Extensions::default(),
    )?;
    let rules = BTreeMap::from([(rule.id().clone(), rule)]);
    let requirement = FullRequirementId::new("capture.photo#shutter")?;
    for target in ["android", "ios"] {
        assert_eq!(
            evaluate_participation(
                &rules,
                &requirement,
                &TargetId::new(target)?,
                &context(None)?
            )?,
            TruthValue::True
        );
    }
    assert_eq!(
        evaluate_participation(
            &rules,
            &requirement,
            &TargetId::new("tower")?,
            &context(None)?
        )?,
        TruthValue::Unknown
    );
    Ok(())
}

#[test]
fn ambiguous_supplied_registry_is_rejected_before_matching() -> Result<(), Box<dyn Error>> {
    let requirement = FullRequirementId::new("account.create#edge_only")?;
    let edge = TargetId::new("edge")?;
    let first = ParticipationRule::new(
        ParticipationRuleId::new("participation.a")?,
        requirement.clone(),
        vec![edge.clone()],
        Applicability::always(true),
        Extensions::default(),
    )?;
    let second = ParticipationRule::new(
        ParticipationRuleId::new("participation.b")?,
        requirement.clone(),
        vec![edge.clone()],
        Applicability::always(false),
        Extensions::default(),
    )?;
    // Distinct authority IDs that constrain the same (requirement, target)
    // coordinate are ambiguous, so the public boundary rejects rather than
    // selecting whichever rule iterates first.
    let ambiguous = BTreeMap::from([(first.id().clone(), first), (second.id().clone(), second)]);
    assert_eq!(
        evaluate_participation(&ambiguous, &requirement, &edge, &context(None)?),
        Err(eqm_engine::ApplicabilityError::AmbiguousParticipation)
    );
    Ok(())
}

#[test]
fn disjoint_coordinates_and_profile_unknown_stay_exact() -> Result<(), Box<dyn Error>> {
    let requirement = FullRequirementId::new("account.create#edge_only")?;
    let other = FullRequirementId::new("account.create#tower_only")?;
    let edge = TargetId::new("edge")?;
    let tower = TargetId::new("tower")?;
    let rules = rules()?;
    // A disjoint requirement/target coordinate has no authority: unknown.
    assert_eq!(
        evaluate_participation(&rules, &other, &edge, &context(Some("eu"))?)?,
        TruthValue::Unknown
    );
    assert_eq!(
        evaluate_participation(&rules, &requirement, &tower, &context(Some("eu"))?)?,
        TruthValue::Unknown
    );
    Ok(())
}
