//! Obligation derivation over the two-target signup example.

mod support;

use eqm_domain::{DimensionId, PolicyId, ProfileId, Revision, SymbolicValueId};
use eqm_engine::{
    ApplicabilityContext, AuthorityOrigin, EvaluationMode, FragmentDigestMap, PolicyProfileRequest,
    PolicyRef, ProfileRequest, ScopeSubject, derive_obligations, expand_fragments, resolve_graph,
    select_policy_profiles,
};
use eqm_manifest::canonicalize_fragment;
use std::collections::BTreeMap;
use std::error::Error;

#[test]
fn signup_obligations_cover_each_target_and_end_to_end_without_duplicates()
-> Result<(), Box<dyn Error>> {
    let (_repository, loaded) = support::loaded_example()?;
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let selection_graph = graph.clone();
    let digests: FragmentDigestMap = graph
        .fragments()
        .iter()
        .map(|(key, fragment)| Ok((key.clone(), canonicalize_fragment(fragment)?.digest())))
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
        &selection_graph,
        EvaluationMode::PullRequest,
        Some(&request),
        None,
    )?;
    let profile = selection_graph
        .profiles()
        .values()
        .next()
        .ok_or("profile missing")?;
    let applicability = ApplicabilityContext::new(
        profile,
        BTreeMap::from([(
            DimensionId::new("region")?,
            Some(SymbolicValueId::new("us")?),
        )]),
    )?;
    let derived = derive_obligations(&finalized, &selection, &applicability, None)?;
    assert_eq!(derived.obligations.len(), 5);
    assert!(derived.unmatched_warnings.is_empty());
    assert!(derived.unknown_applicability.is_empty());
    assert_eq!(
        derived
            .obligations
            .keys()
            .filter(|key| matches!(key.subject, ScopeSubject::Target(_)))
            .count(),
        4
    );
    assert_eq!(
        derived
            .obligations
            .keys()
            .filter(|key| matches!(key.subject, ScopeSubject::TargetSet(_)))
            .count(),
        1
    );
    Ok(())
}
