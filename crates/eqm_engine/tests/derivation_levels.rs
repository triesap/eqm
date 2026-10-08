//! Real authored severity changes preserve visible work without graph substitution.

mod support;

use eqm_domain::{DimensionId, PolicyId, ProfileId, Revision, SymbolicValueId};
use eqm_engine::{
    ApplicabilityContext, AuthorityOrigin, EvaluationMode, FragmentDigestMap, MonotonicityError,
    PolicyProfileRequest, PolicyRef, ProfileRequest, ScopeSubject, UnresolvedReason,
    enforce_required_universe_strengthening, expand_fragments, resolve_graph,
    select_policy_profiles,
};
use eqm_manifest::canonicalize_fragment;
use std::collections::BTreeSet;
use std::error::Error;
use std::fs;

#[test]
fn increasing_real_requirement_severity_preserves_and_promotes_visible_work()
-> Result<(), Box<dyn Error>> {
    let mut previous = None;
    for level in ["optional", "recommended", "required"] {
        let (_root, loaded) = support::loaded_example_with(|root| {
            let path = root.join("eqm/contracts/auth.signup.identifier.toml");
            fs::write(
                &path,
                fs::read_to_string(&path)?
                    .replace("level = \"required\"", &format!("level = \"{level}\"")),
            )?;
            let path = root.join("eqm/policies/consumer.critical_flow.toml");
            fs::write(
                &path,
                fs::read_to_string(&path)?.replace(
                    "minimum_level = \"required\"",
                    "minimum_level = \"recommended\"",
                ),
            )?;
            Ok(())
        })?;
        let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
        let digests: FragmentDigestMap = graph
            .fragments()
            .iter()
            .map(|(k, f)| Ok((k.clone(), canonicalize_fragment(f)?.digest())))
            .collect::<Result<_, Box<dyn Error>>>()?;
        let finalized = expand_fragments(graph, &digests, loaded.source_map())?;
        let request = PolicyProfileRequest::new(
            AuthorityOrigin::TrustedInvocation,
            PolicyRef::new(PolicyId::new("consumer.critical_flow")?, Revision::new(1)?),
            vec![ProfileRequest::new(
                ProfileId::new("audience.default")?,
                Revision::new(1)?,
                vec![(
                    DimensionId::new("region")?,
                    Some(SymbolicValueId::new("us")?),
                )],
            )?],
        )?;
        let selection = select_policy_profiles(
            finalized.graph(),
            EvaluationMode::PullRequest,
            Some(&request),
            None,
        )?;
        let profile = finalized
            .graph()
            .profiles()
            .values()
            .next()
            .ok_or("profile")?;
        let applicability = ApplicabilityContext::from_profiles(vec![(
            profile,
            selection
                .profiles()
                .values()
                .next()
                .ok_or("selected")?
                .values(),
        )])?;
        let universe =
            support::derive_fixture_universe(&finalized, &selection, &applicability, None)?;
        let coordinates = universe
            .coordinates()
            .iter()
            .map(|key| {
                let subject = match &key.subject {
                    ScopeSubject::Target(t) => t.as_str().to_owned(),
                    ScopeSubject::Scenario(s) => s.scenario().id().as_str().to_owned(),
                    _ => return Err("scope"),
                };
                Ok((key.requirement.to_string(), subject))
            })
            .collect::<Result<BTreeSet<_>, _>>()?;
        let mut literal = BTreeSet::from([
            (
                "account.create.signup.otp#six_decimal_digits".to_owned(),
                "android".to_owned(),
            ),
            (
                "account.create.signup.otp#six_decimal_digits".to_owned(),
                "ios".to_owned(),
            ),
            (
                "account.create.signup.otp#valid_code_advances".to_owned(),
                "scenario.fixture_signup".to_owned(),
            ),
        ]);
        if level == "optional" {
            assert_eq!(universe.unresolved().len(), 2);
            assert_eq!(
                universe
                    .unresolved()
                    .iter()
                    .map(|c| {
                        assert_eq!(c.reason, UnresolvedReason::UnmatchedPolicy);
                        assert_eq!(
                            c.requirement.to_string(),
                            "account.create.signup.identifier#email_default"
                        );
                        let Some(ScopeSubject::Target(t)) = &c.subject else {
                            return Err("subject");
                        };
                        Ok(t.as_str())
                    })
                    .collect::<Result<BTreeSet<_>, _>>()?,
                BTreeSet::from(["android", "ios"])
            );
        } else {
            assert!(universe.unresolved().is_empty());
            for target in ["android", "ios"] {
                literal.insert((
                    "account.create.signup.identifier#email_default".to_owned(),
                    target.to_owned(),
                ));
            }
        }
        assert_eq!(coordinates, literal, "authored level={level}");
        if let Some((prior, prior_coordinates)) = &previous {
            assert!(coordinates.is_superset(prior_coordinates));
            assert_eq!(
                enforce_required_universe_strengthening(&universe, prior),
                Err(MonotonicityError::IncomparableAuthority),
                "authored graph replacements need their separate protected integration"
            );
        }
        previous = Some((universe, coordinates));
    }
    Ok(())
}
