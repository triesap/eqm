//! Obligation derivation over the two-target signup example.

mod support;

use eqm_domain::{
    DimensionId, FullRequirementId, PolicyId, ProfileId, Revision, SymbolicValueId, TargetId,
};
use eqm_engine::{
    ApplicabilityContext, AuthorityOrigin, CompletePolicyResolution, ConformanceError,
    EvaluationMode, FacetStatus, FragmentDigestMap, PolicyProfileRequest, PolicyRef,
    ProfileRequest, RequiredUniverse, ScopeSubject, TargetConformance, UnresolvedReason,
    derive_obligations, derive_required_universe, evaluate_complete_target_conformance,
    expand_fragments, resolve_graph, select_policy_profiles,
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

#[test]
fn required_universe_is_complete_and_deterministic() -> Result<(), Box<dyn Error>> {
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

    let first = derive_required_universe(&finalized, &selection, &applicability, None)?;
    let second = derive_required_universe(&finalized, &selection, &applicability, None)?;
    assert_eq!(first, second, "universe derivation must be deterministic");
    assert_eq!(first.coordinates().len(), 5);
    assert!(
        first
            .coordinates()
            .iter()
            .any(|key| matches!(key.subject, ScopeSubject::Target(_)))
    );
    assert!(
        first
            .coordinates()
            .iter()
            .any(|key| matches!(key.subject, ScopeSubject::TargetSet(_)))
    );
    let expected_revision = Revision::new(1)?;
    assert!(
        first
            .coordinates()
            .iter()
            .all(|key| key.policy_revision == expected_revision)
    );

    // A candidate universe that omits a required coordinate is not a superset.
    let empty = RequiredUniverse::default();
    assert!(!first.is_subset_of(&empty));
    assert!(empty.is_subset_of(&first));
    assert!(
        first.contains(
            first
                .coordinates()
                .iter()
                .next()
                .ok_or("coordinate missing")?
        )
    );

    // The checked completeness boundary accepts a complete resolution and
    // rejects one that omits a participating required coordinate or names an
    // unrelated coordinate.
    let complete: BTreeMap<_, _> = first
        .coordinates()
        .iter()
        .cloned()
        .map(|key| (key, FacetStatus::Satisfied))
        .collect();
    let resolution = CompletePolicyResolution::new(first.clone(), complete.clone())?;
    assert_eq!(
        evaluate_complete_target_conformance(&TargetId::new("android")?, &resolution)?,
        TargetConformance::Conformant
    );
    let (removed_key, removed_target) = first
        .coordinates()
        .iter()
        .find_map(|key| match &key.subject {
            ScopeSubject::Target(target) => Some((key.clone(), target.clone())),
            _ => None,
        })
        .ok_or("target coordinate missing")?;
    let mut missing = complete;
    missing.remove(&removed_key);
    let resolution = CompletePolicyResolution::new(first.clone(), missing)?;
    assert_eq!(
        evaluate_complete_target_conformance(&removed_target, &resolution),
        Err(ConformanceError::IncompletePolicyResolution)
    );
    let extra: BTreeMap<_, _> = first
        .coordinates()
        .iter()
        .take(1)
        .cloned()
        .map(|key| (key, FacetStatus::Satisfied))
        .collect();
    assert_eq!(
        CompletePolicyResolution::new(RequiredUniverse::default(), extra),
        Err(ConformanceError::UnexpectedCoordinate)
    );
    Ok(())
}

#[test]
fn unknown_and_not_applicable_coordinates_remain_visible() -> Result<(), Box<dyn Error>> {
    let (_repository, loaded) = support::loaded_example_with(|root| {
        let path = root.join("eqm/contracts/auth.signup.identifier.toml");
        let mut text = std::fs::read_to_string(&path)?;
        text.push_str(
            "\n[requirements.applicability]\ndimension = \"region\"\noperator = \"eq\"\nvalue = \"eu\"\n",
        );
        std::fs::write(&path, text)?;
        Ok(())
    })?;
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
    let requirement = FullRequirementId::new("account.create.signup.identifier#email_default")?;

    let unknown = ApplicabilityContext::new(
        profile,
        BTreeMap::from([(DimensionId::new("region")?, None)]),
    )?;
    let universe = derive_required_universe(&finalized, &selection, &unknown, None)?;
    assert!(universe.unresolved().iter().any(|coordinate| {
        coordinate.requirement == requirement
            && coordinate.reason == UnresolvedReason::UnknownApplicability
    }));

    let not_applicable = ApplicabilityContext::new(
        profile,
        BTreeMap::from([(
            DimensionId::new("region")?,
            Some(SymbolicValueId::new("us")?),
        )]),
    )?;
    let universe = derive_required_universe(&finalized, &selection, &not_applicable, None)?;
    assert!(universe.unresolved().iter().any(|coordinate| {
        coordinate.requirement == requirement
            && coordinate.reason == UnresolvedReason::NotApplicable
    }));

    let applicable = ApplicabilityContext::new(
        profile,
        BTreeMap::from([(
            DimensionId::new("region")?,
            Some(SymbolicValueId::new("eu")?),
        )]),
    )?;
    let universe = derive_required_universe(&finalized, &selection, &applicable, None)?;
    assert!(
        universe
            .unresolved()
            .iter()
            .all(|coordinate| coordinate.requirement != requirement)
    );
    assert!(
        universe
            .coordinates()
            .iter()
            .any(|coordinate| coordinate.requirement == requirement)
    );
    Ok(())
}
