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

#[test]
fn graph_construction_rejects_conflicting_participation_coordinates() -> Result<(), Box<dyn Error>>
{
    use eqm_domain::{WorkspaceGraph, WorkspaceGraphBuildError, WorkspaceGraphInput};
    let requirement = FullRequirementId::new("capture.photo#shutter")?;
    let target = TargetId::new("android")?;
    let first = ParticipationRule::new(
        ParticipationRuleId::new("participation.true")?,
        requirement.clone(),
        vec![target.clone()],
        Applicability::always(true),
        Extensions::default(),
    )?;
    let second = ParticipationRule::new(
        ParticipationRuleId::new("participation.false")?,
        requirement.clone(),
        vec![target.clone()],
        Applicability::always(false),
        Extensions::default(),
    )?;
    // Distinct authority IDs must not silently constrain one coordinate with
    // conflicting truth values.
    assert_eq!(
        WorkspaceGraph::new(WorkspaceGraphInput {
            participation: vec![first.clone(), second.clone()],
            ..WorkspaceGraphInput::default()
        }),
        Err(WorkspaceGraphBuildError::DuplicateParticipationCoordinate)
    );
    // Identical-value rules under different IDs are equally ambiguous.
    let identical = ParticipationRule::new(
        ParticipationRuleId::new("participation.true.two")?,
        requirement.clone(),
        vec![target.clone()],
        Applicability::always(true),
        Extensions::default(),
    )?;
    assert_eq!(
        WorkspaceGraph::new(WorkspaceGraphInput {
            participation: vec![first, identical],
            ..WorkspaceGraphInput::default()
        }),
        Err(WorkspaceGraphBuildError::DuplicateParticipationCoordinate)
    );
    Ok(())
}

#[test]
fn graph_construction_accepts_disjoint_coordinates_and_rejects_duplicate_authority()
-> Result<(), Box<dyn Error>> {
    use eqm_domain::{WorkspaceGraph, WorkspaceGraphBuildError, WorkspaceGraphInput};
    let android = ParticipationRule::new(
        ParticipationRuleId::new("participation.android")?,
        FullRequirementId::new("capture.photo#shutter")?,
        vec![TargetId::new("android")?],
        Applicability::always(true),
        Extensions::default(),
    )?;
    let ios = ParticipationRule::new(
        ParticipationRuleId::new("participation.ios")?,
        FullRequirementId::new("capture.photo#shutter")?,
        vec![TargetId::new("ios")?],
        Applicability::always(true),
        Extensions::default(),
    )?;
    let graph = WorkspaceGraph::new(WorkspaceGraphInput {
        participation: vec![android.clone(), ios],
        ..WorkspaceGraphInput::default()
    })?;
    assert_eq!(graph.participation().len(), 2);
    assert_eq!(
        WorkspaceGraph::new(WorkspaceGraphInput {
            participation: vec![android.clone(), android],
            ..WorkspaceGraphInput::default()
        }),
        Err(WorkspaceGraphBuildError::DuplicateParticipation)
    );
    Ok(())
}
