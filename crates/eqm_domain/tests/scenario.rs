//! S037: role-bearing scenario participant occurrences remain distinct.

use eqm_domain::{
    Extensions, OccurrenceId, ProviderId, Revision, RoleId, Scenario, ScenarioDependency,
    ScenarioError, ScenarioId, ScenarioParticipant, ScenarioSubject, TargetId,
};
use std::error::Error;

fn occurrence(value: &str) -> Result<OccurrenceId, Box<dyn Error>> {
    Ok(OccurrenceId::new(value)?)
}

fn participant(
    occurrence_id: &str,
    role: &str,
    target: &str,
) -> Result<ScenarioParticipant, Box<dyn Error>> {
    Ok(ScenarioParticipant::new(
        occurrence(occurrence_id)?,
        RoleId::new(role)?,
        ScenarioSubject::Target(TargetId::new(target)?),
    ))
}

#[test]
fn repeated_targets_with_distinct_occurrences_stay_distinct() -> Result<(), Box<dyn Error>> {
    let first = participant("phone_one", "edge", "mobile")?;
    let second = participant("phone_two", "edge", "mobile")?;
    let scenario = Scenario::new(
        ScenarioId::new("signup.flow")?,
        Revision::new(1)?,
        vec![first.clone(), second.clone()],
        vec![ScenarioDependency::new(
            occurrence("phone_two")?,
            occurrence("phone_one")?,
        )?],
        Extensions::default(),
    )?;
    assert_eq!(scenario.participants().len(), 2);
    assert_ne!(first, second);
    assert_eq!(scenario.participants().iter().next(), Some(&first));
    Ok(())
}

#[test]
fn provider_subject_is_tagged_and_distinct_from_target() -> Result<(), Box<dyn Error>> {
    let provider = ScenarioParticipant::new(
        occurrence("mailer")?,
        RoleId::new("provider")?,
        ScenarioSubject::Provider(ProviderId::new("provider.mailgun")?),
    );
    assert_eq!(
        provider.subject(),
        &ScenarioSubject::Provider(ProviderId::new("provider.mailgun")?)
    );
    assert_ne!(
        provider.subject(),
        &ScenarioSubject::Target(TargetId::new("mailgun")?)
    );
    Ok(())
}

#[test]
fn duplicate_occurrence_and_unknown_dependency_reject() -> Result<(), Box<dyn Error>> {
    let first = participant("phone", "edge", "mobile")?;
    assert_eq!(
        Scenario::new(
            ScenarioId::new("signup.flow")?,
            Revision::new(1)?,
            vec![first.clone(), first],
            Vec::new(),
            Extensions::default(),
        ),
        Err(ScenarioError::DuplicateOccurrence)
    );

    let only = participant("phone", "edge", "mobile")?;
    assert_eq!(
        Scenario::new(
            ScenarioId::new("signup.flow")?,
            Revision::new(1)?,
            vec![only],
            vec![ScenarioDependency::new(
                occurrence("phone")?,
                occurrence("absent")?,
            )?],
            Extensions::default(),
        ),
        Err(ScenarioError::UnknownOccurrence)
    );
    Ok(())
}

#[test]
fn self_and_duplicate_dependencies_reject() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        ScenarioDependency::new(occurrence("phone")?, occurrence("phone")?),
        Err(ScenarioError::SelfDependency)
    );
    let dependency = ScenarioDependency::new(occurrence("a")?, occurrence("b")?)?;
    assert_eq!(
        Scenario::new(
            ScenarioId::new("signup.flow")?,
            Revision::new(1)?,
            vec![
                participant("a", "edge", "mobile")?,
                participant("b", "tower", "desktop")?,
            ],
            vec![dependency.clone(), dependency],
            Extensions::default(),
        ),
        Err(ScenarioError::DuplicateDependency)
    );
    Ok(())
}

#[test]
fn empty_participants_reject() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        Scenario::new(
            ScenarioId::new("signup.flow")?,
            Revision::new(1)?,
            Vec::new(),
            Vec::new(),
            Extensions::default(),
        ),
        Err(ScenarioError::ParticipantsRequired)
    );
    Ok(())
}
