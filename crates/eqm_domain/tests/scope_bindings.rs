//! S052-A exact scope selection claims; graph authority is validated later.

use eqm_domain::{
    FullRequirementId, RequirementScenarioBindings, Revision, ScenarioId, ScenarioRef,
    ScopeBindingsError,
};
use std::error::Error;

fn entry(revision: u64) -> Result<(FullRequirementId, ScenarioRef), Box<dyn Error>> {
    Ok((
        FullRequirementId::new("account.create.signup.otp#valid_code_advances")?,
        ScenarioRef::new(ScenarioId::new("signup.flow")?, Revision::new(revision)?),
    ))
}

#[test]
fn identical_and_conflicting_duplicate_requirement_claims_reject() -> Result<(), Box<dyn Error>> {
    let first = entry(1)?;
    for second in [entry(1)?, entry(2)?] {
        assert_eq!(
            RequirementScenarioBindings::new(vec![first.clone(), second]),
            Err(ScopeBindingsError::DuplicateRequirement(first.0.clone()))
        );
    }
    Ok(())
}

#[test]
fn distinct_requirements_and_exact_revisions_are_preserved() -> Result<(), Box<dyn Error>> {
    let first = entry(1)?;
    let second_requirement = FullRequirementId::new("account.create.signup.otp#another_rule")?;
    let bindings = RequirementScenarioBindings::new(vec![
        first.clone(),
        (second_requirement.clone(), first.1.clone()),
    ])?;
    assert_eq!(bindings.bindings().len(), 2);
    assert_eq!(bindings.bindings().get(&second_requirement), Some(&first.1));
    assert_eq!(first.1.id().as_str(), "signup.flow");
    assert_eq!(first.1.revision().get(), 1);
    assert_ne!(first.1, entry(2)?.1);
    // Explicit absence is data, never an implicit scenario authorization.
    assert!(
        RequirementScenarioBindings::new(Vec::new())?
            .bindings()
            .is_empty()
    );
    Ok(())
}
