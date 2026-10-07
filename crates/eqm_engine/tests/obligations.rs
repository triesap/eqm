//! Obligation derivation over the two-target signup example.

mod support;

use eqm_domain::{
    DimensionId, Facet, FullRequirementId, PolicyId, ProfileId, Revision, SymbolicValueId,
    TargetId, TrustLevel, WorkspaceGraph, WorkspaceGraphInput,
};
use eqm_engine::{
    ApplicabilityContext, AuthorityOrigin, CompletePolicyResolution, ConformanceError,
    EvaluationMode, FacetStatus, FragmentDigestMap, PolicyProfileRequest, PolicyRef,
    ProfileRequest, ScopeSubject, SelectedPolicyProfiles, TargetConformance, UnresolvedReason,
    evaluate_complete_target_conformance, expand_fragments, resolve_graph, select_policy_profiles,
};
use eqm_manifest::canonicalize_fragment;
use std::collections::BTreeMap;
use std::error::Error;

type Finalized = eqm_engine::FinalizedWorkspaceGraph;

/// Selects exact authority and builds the applicability context from the same
/// immutable finalized graph, as an authorizing derivation now requires.
fn select_and_context<'a>(
    finalized: &'a Finalized,
    region: Option<&str>,
) -> Result<(SelectedPolicyProfiles<'a>, ApplicabilityContext), Box<dyn Error>> {
    let request = PolicyProfileRequest::new(
        AuthorityOrigin::TrustedInvocation,
        PolicyRef::new(PolicyId::new("consumer.critical_flow")?, Revision::new(1)?),
        vec![ProfileRequest::new(
            ProfileId::new("audience.default")?,
            Revision::new(1)?,
            vec![(
                DimensionId::new("region")?,
                region.map(SymbolicValueId::new).transpose()?,
            )],
        )?],
    )?;
    let selection = select_policy_profiles(
        finalized.graph(),
        EvaluationMode::PullRequest,
        Some(&request),
        None,
    )?;
    let applicability = {
        let mut inputs = Vec::new();
        for selected in selection.profiles().values() {
            let definition = finalized
                .graph()
                .profiles()
                .get(&(selected.id().clone(), selected.revision()))
                .ok_or("profile definition missing")?;
            inputs.push((definition, selected.values()));
        }
        ApplicabilityContext::from_profiles(inputs)?
    };
    Ok((selection, applicability))
}

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
    let derived =
        support::derive_fixture_obligations(&finalized, &selection, &applicability, None)?;
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
            .filter(|key| matches!(key.subject, ScopeSubject::Scenario(_)))
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

    let first = support::derive_fixture_universe(&finalized, &selection, &applicability, None)?;
    let second = support::derive_fixture_universe(&finalized, &selection, &applicability, None)?;
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
            .any(|key| matches!(key.subject, ScopeSubject::Scenario(_)))
    );
    let expected_revision = Revision::new(1)?;
    assert!(
        first
            .coordinates()
            .iter()
            .all(|key| key.policy_revision == expected_revision)
    );

    // A candidate universe that omits a required coordinate is not a superset.
    // An empty universe is derived from a genuine graph with no surface
    // requirements; it is never publicly asserted.
    let minimal = expand_fragments(
        WorkspaceGraph::new(WorkspaceGraphInput {
            policies: selection_graph.policies().values().cloned().collect(),
            profiles: selection_graph.profiles().values().cloned().collect(),
            ..WorkspaceGraphInput::default()
        })?,
        &FragmentDigestMap::new(),
        &BTreeMap::new(),
    )?;
    let empty_selection = select_policy_profiles(
        minimal.graph(),
        EvaluationMode::PullRequest,
        Some(&request),
        None,
    )?;
    let empty = support::derive_fixture_universe(&minimal, &empty_selection, &applicability, None)?;
    assert!(empty.coordinates().is_empty());
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
    let resolution = fixture_resolution(first.clone(), complete.clone())?;
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
    let resolution = fixture_resolution(first.clone(), missing)?;
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
        fixture_resolution(empty, extra),
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
    let digests: FragmentDigestMap = graph
        .fragments()
        .iter()
        .map(|(key, fragment)| Ok((key.clone(), canonicalize_fragment(fragment)?.digest())))
        .collect::<Result<_, Box<dyn Error>>>()?;
    let finalized = expand_fragments(graph, &digests, loaded.source_map())?;
    let requirement = FullRequirementId::new("account.create.signup.identifier#email_default")?;

    let (selection, unknown) = select_and_context(&finalized, None)?;
    let universe = support::derive_fixture_universe(&finalized, &selection, &unknown, None)?;
    assert!(universe.unresolved().iter().any(|coordinate| {
        coordinate.requirement == requirement
            && coordinate.reason == UnresolvedReason::UnknownApplicability
    }));

    assert!(
        universe
            .unresolved()
            .iter()
            .all(|coordinate| { coordinate.reason == UnresolvedReason::UnknownApplicability }),
        "the unknown-applicability control must not be masked by another unresolved cause"
    );
    let facets = universe
        .coordinates()
        .iter()
        .cloned()
        .map(|key| (key, FacetStatus::Satisfied))
        .collect();
    let resolution = fixture_resolution(universe, facets)?;
    for target in ["android", "ios"] {
        assert_eq!(
            evaluate_complete_target_conformance(&TargetId::new(target)?, &resolution)?,
            TargetConformance::Nonconformant
        );
    }

    let (selection, not_applicable) = select_and_context(&finalized, Some("us"))?;
    let universe = support::derive_fixture_universe(&finalized, &selection, &not_applicable, None)?;
    assert!(universe.unresolved().iter().any(|coordinate| {
        coordinate.requirement == requirement
            && coordinate.reason == UnresolvedReason::NotApplicable
    }));

    let (selection, applicable) = select_and_context(&finalized, Some("eu"))?;
    let universe = support::derive_fixture_universe(&finalized, &selection, &applicable, None)?;
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

#[test]
fn facet_strength_does_not_leak_across_facets() -> Result<(), Box<dyn Error>> {
    let (_repository, loaded) = support::loaded_example_with(|root| {
        let path = root.join("eqm/policies/consumer.critical_flow.toml");
        let text = std::fs::read_to_string(&path)?;
        let text = text.replace(
            "minimum_trust = \"trusted_ci\"",
            "minimum_trust = \"signed_ci\"",
        );
        let extra = "\n[[rules]]\nminimum_level = \"required\"\nfacets = [\"accessibility\"]\nminimum_trust = \"trusted_ci\"\nmaximum_age = 86400000\nminimum_count = 1\n\n[rules.selector]\nrisk_classes = [\"high\", \"critical\"]\n";
        std::fs::write(&path, format!("{text}{extra}"))?;
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
    let applicability = ApplicabilityContext::new(
        profile,
        BTreeMap::from([(
            DimensionId::new("region")?,
            Some(SymbolicValueId::new("us")?),
        )]),
    )?;
    let derivation =
        support::derive_fixture_obligations(&finalized, &selection, &applicability, None)?;
    let requirement = FullRequirementId::new("account.create.signup.otp#six_decimal_digits")?;
    let strength = |facet: Facet| -> Result<TrustLevel, Box<dyn Error>> {
        derivation
            .obligations
            .iter()
            .find(|(key, _)| key.requirement == requirement && key.facet == facet)
            .map(|(_, obligation)| obligation.strength.minimum_trust)
            .ok_or_else(|| format!("{facet} obligation missing").into())
    };
    assert_eq!(strength(Facet::Behavior)?, TrustLevel::SignedCi);
    assert_eq!(strength(Facet::Accessibility)?, TrustLevel::TrustedCi);
    Ok(())
}

// Asserted fixture facts test pure engine semantics, not protected trust.
fn fixture_input() -> eqm_engine::FacetEvaluationInput {
    use eqm_engine::*;
    FacetEvaluationInput {
        applicability: TruthValue::True,
        context_valid: true,
        coverage: CoverageStatus::Covered,
        outcome: EvidenceOutcome::Satisfied,
        freshness: FreshnessStatus::Fresh,
        trust: TrustEvaluation::Sufficient,
        structure: SupportingCheck::NotRequired,
        exposure: SupportingCheck::NotRequired,
        release: SupportingCheck::NotRequired,
        waiver: WaiverEvaluation::NotApplied(BTreeMap::new()),
    }
}
fn fixture_resolution(
    universe: eqm_engine::RequiredUniverse,
    facets: BTreeMap<eqm_engine::ObligationKey, FacetStatus>,
) -> Result<CompletePolicyResolution, ConformanceError> {
    let inputs = facets
        .into_iter()
        .map(|(key, status)| {
            assert_eq!(
                status,
                FacetStatus::Satisfied,
                "fixture facts require an explicit passing control"
            );
            (key, fixture_input())
        })
        .collect();
    let evaluated = eqm_engine::evaluate_required_facets(&universe, inputs)?;
    CompletePolicyResolution::new(universe, evaluated)
}
