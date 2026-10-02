//! S034: capability participation rules bind requirements to explicit targets.

use eqm_domain::{
    Applicability, Extensions, FullRequirementId, ParticipationError, ParticipationRule,
    ParticipationRuleId, PolicyBuildError, TargetId, validate_participation_rules,
};
use std::error::Error;

fn rule(
    id: &str,
    requirement: &str,
    targets: &[&str],
) -> Result<ParticipationRule, Box<dyn Error>> {
    Ok(ParticipationRule::new(
        ParticipationRuleId::new(id)?,
        FullRequirementId::new(requirement)?,
        targets
            .iter()
            .map(|target| TargetId::new(*target))
            .collect::<Result<Vec<_>, _>>()?,
        Applicability::always(true),
        Extensions::default(),
    )?)
}

#[test]
fn rule_binds_a_requirement_to_explicit_targets() -> Result<(), Box<dyn Error>> {
    let rule = rule(
        "participation.mobile",
        "capture.photo#shutter",
        &["android", "ios"],
    )?;
    assert_eq!(rule.id().as_str(), "participation.mobile");
    assert_eq!(rule.requirement().as_str(), "capture.photo#shutter");
    assert_eq!(rule.targets().len(), 2);
    assert!(rule.targets().contains(&TargetId::new("android")?));
    assert_eq!(rule.applicability(), &Applicability::always(true));
    Ok(())
}

#[test]
fn empty_or_duplicate_targets_reject() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        ParticipationRule::new(
            ParticipationRuleId::new("participation.mobile")?,
            FullRequirementId::new("capture.photo#shutter")?,
            vec![],
            Applicability::always(true),
            Extensions::default(),
        ),
        Err(PolicyBuildError::TargetsRequired)
    );
    assert_eq!(
        ParticipationRule::new(
            ParticipationRuleId::new("participation.mobile")?,
            FullRequirementId::new("capture.photo#shutter")?,
            vec![TargetId::new("android")?, TargetId::new("android")?],
            Applicability::always(true),
            Extensions::default(),
        ),
        Err(PolicyBuildError::DuplicateTarget)
    );
    Ok(())
}

#[test]
fn duplicate_rule_and_coordinate_authority_reject() -> Result<(), Box<dyn Error>> {
    let first = rule(
        "participation.mobile",
        "capture.photo#shutter",
        &["android"],
    )?;
    let duplicate_id = rule("participation.mobile", "capture.photo#other", &["ios"])?;
    assert_eq!(
        validate_participation_rules(vec![first.clone(), duplicate_id]),
        Err(ParticipationError::DuplicateRule)
    );

    let duplicate_coordinate = rule(
        "participation.other",
        "capture.photo#shutter",
        &["android", "ios"],
    )?;
    assert_eq!(
        validate_participation_rules(vec![first, duplicate_coordinate]),
        Err(ParticipationError::DuplicateCoordinate)
    );
    Ok(())
}

#[test]
fn distinct_coordinates_index_by_rule_authority() -> Result<(), Box<dyn Error>> {
    let mobile = rule(
        "participation.mobile",
        "capture.photo#shutter",
        &["android", "ios"],
    )?;
    let desktop = rule("participation.desktop", "review.queue#approve", &["tower"])?;
    let indexed = validate_participation_rules(vec![desktop, mobile])?;
    assert_eq!(indexed.len(), 2);
    assert!(indexed.contains_key(&ParticipationRuleId::new("participation.mobile")?));
    Ok(())
}
