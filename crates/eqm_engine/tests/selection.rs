//! Policy/profile authority-selection fixtures.

mod support;

use eqm_domain::{
    Facet, FullRequirementId, PolicyId, ProfileId, RequirementScope, Revision, RiskClass, UnitId,
};
use eqm_engine::{
    AuthorityOrigin, EvaluationMode, PolicyProfileRequest, PolicyRef, ProfileRequest,
    SelectionError, matching_policy_rules, resolve_graph, select_policy_profiles,
};
use std::error::Error;

fn request(
    origin: AuthorityOrigin,
    values: Vec<(&str, Option<&str>)>,
) -> Result<PolicyProfileRequest, Box<dyn Error>> {
    Ok(PolicyProfileRequest::new(
        origin,
        PolicyRef::new(PolicyId::new("consumer.critical_flow")?, Revision::new(1)?),
        vec![ProfileRequest::new(
            ProfileId::new("audience.default")?,
            Revision::new(1)?,
            values
                .into_iter()
                .map(|(dimension, value)| {
                    Ok((dimension.parse()?, value.map(str::parse).transpose()?))
                })
                .collect::<Result<_, Box<dyn Error>>>()?,
        )?],
    )?)
}

#[test]
fn development_uses_defaults_and_explicit_values_override_them() -> Result<(), Box<dyn Error>> {
    let (_repository, loaded) = support::loaded_example()?;
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let default = request(AuthorityOrigin::CandidateLocal, Vec::new())?;
    let selected =
        select_policy_profiles(&graph, EvaluationMode::Development, None, Some(&default))?;
    assert_eq!(
        selected.profiles()[&ProfileId::new("audience.default")?].values()[&"region".parse()?]
            .as_ref()
            .map(ToString::to_string),
        Some("us".to_owned())
    );
    let explicit = request(
        AuthorityOrigin::CandidateLocal,
        vec![("region", Some("eu"))],
    )?;
    let selected = select_policy_profiles(
        &graph,
        EvaluationMode::Development,
        Some(&explicit),
        Some(&default),
    )?;
    assert_eq!(
        selected.profiles()[&ProfileId::new("audience.default")?].values()[&"region".parse()?]
            .as_ref()
            .map(ToString::to_string),
        Some("eu".to_owned())
    );
    Ok(())
}

#[test]
fn non_local_modes_require_explicit_authoritative_selection() -> Result<(), Box<dyn Error>> {
    let (_repository, loaded) = support::loaded_example()?;
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let local = request(AuthorityOrigin::CandidateLocal, vec![("region", None)])?;
    assert_eq!(
        select_policy_profiles(&graph, EvaluationMode::PullRequest, None, Some(&local)),
        Err(SelectionError::ExplicitSelectionRequired)
    );
    assert_eq!(
        select_policy_profiles(&graph, EvaluationMode::PullRequest, Some(&local), None),
        Err(SelectionError::UntrustedSelection)
    );
    let trusted = request(AuthorityOrigin::TrustedInvocation, vec![("region", None)])?;
    select_policy_profiles(&graph, EvaluationMode::PullRequest, Some(&trusted), None)?;
    assert_eq!(
        select_policy_profiles(&graph, EvaluationMode::Release, Some(&trusted), None),
        Err(SelectionError::UntrustedSelection)
    );
    let protected = request(
        AuthorityOrigin::ProtectedBaseline,
        vec![("region", Some("us"))],
    )?;
    select_policy_profiles(&graph, EvaluationMode::Release, Some(&protected), None)?;
    Ok(())
}

#[test]
fn invalid_authority_and_closed_selector_axes_fail_or_filter_exactly() -> Result<(), Box<dyn Error>>
{
    let (_repository, loaded) = support::loaded_example()?;
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let invalid = request(
        AuthorityOrigin::TrustedInvocation,
        vec![("region", Some("apac"))],
    )?;
    assert_eq!(
        select_policy_profiles(&graph, EvaluationMode::PullRequest, Some(&invalid), None),
        Err(SelectionError::UndeclaredValue)
    );
    let policy = graph.policies().values().next().ok_or("policy missing")?;
    let requirement = FullRequirementId::new("account.create.signup.identifier#email_default")?;
    assert_eq!(
        matching_policy_rules(
            policy,
            &UnitId::new("account.create.signup.identifier")?,
            &requirement,
            RiskClass::High,
            Facet::Behavior,
            RequirementScope::EachTarget,
        )
        .len(),
        1
    );
    assert!(
        matching_policy_rules(
            policy,
            &UnitId::new("account.create.signup.identifier")?,
            &requirement,
            RiskClass::Low,
            Facet::Behavior,
            RequirementScope::EachTarget,
        )
        .is_empty()
    );
    Ok(())
}
