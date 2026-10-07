//! P04-R2 scope-admission controls: checked graph token, complete denominator,
//! participation truth and unresolved non-authorization.

mod support;

use eqm_domain::{DimensionId, PolicyId, ProfileId, Revision, SymbolicValueId, TargetId};
use eqm_engine::{
    ApplicabilityContext, AuthorityOrigin, CompletePolicyResolution, ConformanceError,
    EvaluationMode, FacetStatus, FragmentDigestMap, PolicyProfileRequest, PolicyRef,
    ProfileRequest, ScopeSubject, TargetConformance, UnresolvedReason, derive_required_universe,
    evaluate_complete_target_conformance, expand_fragments, resolve_graph, select_policy_profiles,
};
use eqm_manifest::canonicalize_fragment;
use std::collections::BTreeMap;
use std::error::Error;

type Finalized = eqm_engine::FinalizedWorkspaceGraph;

fn request() -> Result<PolicyProfileRequest, Box<dyn Error>> {
    request_value(Some("us"))
}

fn request_value(region: Option<&str>) -> Result<PolicyProfileRequest, Box<dyn Error>> {
    Ok(PolicyProfileRequest::new(
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
    )?)
}

fn finalize(loaded: &eqm_manifest::LoadedWorkspace) -> Result<Finalized, Box<dyn Error>> {
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let digests: FragmentDigestMap = graph
        .fragments()
        .iter()
        .map(|(key, fragment)| Ok((key.clone(), canonicalize_fragment(fragment)?.digest())))
        .collect::<Result<_, Box<dyn Error>>>()?;
    Ok(expand_fragments(graph, &digests, loaded.source_map())?)
}

fn with_participation() -> Result<(tempfile::TempDir, Finalized), Box<dyn Error>> {
    let (repository, loaded) = support::loaded_example_with(|root| {
        // Replace every authored participation document with the asymmetric
        // fixture: android declared true, ios deliberately absent so both the
        // identifier coordinate and every end-to-end target set stay Unknown and
        // non-authorizing.
        for entry in std::fs::read_dir(root.join("eqm/contracts"))? {
            let entry = entry?;
            if entry
                .file_name()
                .to_string_lossy()
                .starts_with("participation.")
            {
                std::fs::remove_file(entry.path())?;
            }
        }
        std::fs::write(
            root.join("eqm/contracts/participation.toml"),
            r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/participation.schema.json"
id = "participation.android_identifier"
requirement = "account.create.signup.identifier#email_default"
targets = ["android"]

[applicability]
always = true
"#,
        )?;
        Ok(())
    })?;
    Ok((repository, finalize(&loaded)?))
}

fn context(finalized: &Finalized) -> Result<ApplicabilityContext, Box<dyn Error>> {
    let selection = select_policy_profiles(
        finalized.graph(),
        EvaluationMode::PullRequest,
        Some(&request()?),
        None,
    )?;
    context_for(finalized, &selection)
}

fn context_for(
    finalized: &Finalized,
    selection: &eqm_engine::SelectedPolicyProfiles<'_>,
) -> Result<ApplicabilityContext, Box<dyn Error>> {
    let mut inputs = Vec::new();
    for selected in selection.profiles().values() {
        let definition = finalized
            .graph()
            .profiles()
            .get(&(selected.id().clone(), selected.revision()))
            .ok_or("profile definition missing")?;
        inputs.push((definition, selected.values()));
    }
    Ok(ApplicabilityContext::from_profiles(inputs)?)
}

#[test]
fn omitted_target_set_coordinate_cannot_yield_conformant() -> Result<(), Box<dyn Error>> {
    let (_repository, loaded) = support::loaded_example()?;
    let finalized = finalize(&loaded)?;
    let selection = select_policy_profiles(
        finalized.graph(),
        EvaluationMode::PullRequest,
        Some(&request()?),
        None,
    )?;
    let applicability = context(&finalized)?;
    let universe = derive_required_universe(&finalized, &selection, &applicability, None)?;
    // The end-to-end requirement produces a target-set coordinate.
    let target_set = universe
        .coordinates()
        .iter()
        .find(|key| matches!(key.subject, ScopeSubject::TargetSet(_)))
        .cloned()
        .ok_or("target-set coordinate missing")?;
    let mut facets: BTreeMap<_, _> = universe
        .coordinates()
        .iter()
        .cloned()
        .map(|key| (key, FacetStatus::Satisfied))
        .collect();
    facets.remove(&target_set);
    let resolution = fixture_resolution(universe, facets)?;
    assert_eq!(
        evaluate_complete_target_conformance(&TargetId::new("android")?, &resolution),
        Err(ConformanceError::IncompletePolicyResolution)
    );
    Ok(())
}

#[test]
fn asymmetric_and_unknown_participation_is_non_authorizing() -> Result<(), Box<dyn Error>> {
    let (_repository, finalized) = with_participation()?;
    let selection = select_policy_profiles(
        finalized.graph(),
        EvaluationMode::PullRequest,
        Some(&request()?),
        None,
    )?;
    let applicability = context(&finalized)?;
    let universe = derive_required_universe(&finalized, &selection, &applicability, None)?;
    let requirement: eqm_domain::FullRequirementId =
        "account.create.signup.identifier#email_default".parse()?;
    let android = TargetId::new("android")?;
    let ios = TargetId::new("ios")?;
    // Declared-true android participation yields an obligation.
    assert!(universe.coordinates().iter().any(|key| {
        key.requirement == requirement
            && matches!(&key.subject, ScopeSubject::Target(t) if t == &android)
    }));
    // A missing rule for ios is Unknown, not implicit global inclusion, so no
    // obligation is derived and the exact unknown coordinate stays visible.
    assert!(!universe.coordinates().iter().any(|key| {
        key.requirement == requirement
            && matches!(&key.subject, ScopeSubject::Target(t) if t == &ios)
    }));
    assert!(universe.unresolved().iter().any(|unresolved| {
        unresolved.requirement == requirement
            && unresolved.subject == Some(ScopeSubject::Target(ios.clone()))
            && unresolved.reason == UnresolvedReason::UnknownParticipation
    }));
    // Satisfied reported statuses for every derived coordinate still cannot
    // make a target conformant while participation is unknown anywhere in its
    // scope; the declared-true android coordinate participates, the missing ios
    // rule stays Unknown and non-authorizing.
    let facets: BTreeMap<_, _> = universe
        .coordinates()
        .iter()
        .cloned()
        .map(|key| (key, FacetStatus::Satisfied))
        .collect();
    let resolution = fixture_resolution(universe, facets)?;
    assert_eq!(
        evaluate_complete_target_conformance(&ios, &resolution)?,
        TargetConformance::Nonconformant
    );
    assert_eq!(
        evaluate_complete_target_conformance(&android, &resolution)?,
        TargetConformance::Nonconformant
    );
    Ok(())
}

#[test]
fn empty_participation_is_non_authorizing_for_v2_and_genuine_v1() -> Result<(), Box<dyn Error>> {
    for legacy in [false, true] {
        let (_repository, loaded) = support::loaded_example_with(|root| {
            for entry in std::fs::read_dir(root.join("eqm/contracts"))? {
                let entry = entry?;
                if entry
                    .file_name()
                    .to_string_lossy()
                    .starts_with("participation.")
                {
                    std::fs::remove_file(entry.path())?;
                }
            }
            if legacy {
                let path = root.join("eqm.toml");
                std::fs::write(
                    &path,
                    std::fs::read_to_string(&path)?.replace(
                        "schemas/v2/manifest/workspace",
                        "schemas/v1/manifest/workspace",
                    ),
                )?;
            }
            Ok(())
        })?;
        let finalized = finalize(&loaded)?;
        assert!(finalized.graph().participation().is_empty());
        let selected = select_policy_profiles(
            finalized.graph(),
            EvaluationMode::PullRequest,
            Some(&request()?),
            None,
        )?;
        let applicability = context_for(&finalized, &selected)?;
        let universe = derive_required_universe(&finalized, &selected, &applicability, None)?;
        assert!(
            universe.coordinates().is_empty(),
            "legacy={legacy}: implicit global participation"
        );
        for target in [TargetId::new("android")?, TargetId::new("ios")?] {
            assert!(
                universe
                    .unresolved()
                    .iter()
                    .any(|coordinate| coordinate.subject
                        == Some(ScopeSubject::Target(target.clone()))
                        && coordinate.reason == UnresolvedReason::UnknownParticipation)
            );
            let resolution = fixture_resolution(universe.clone(), BTreeMap::new())?;
            assert_eq!(
                evaluate_complete_target_conformance(&target, &resolution)?,
                TargetConformance::Nonconformant,
                "legacy={legacy}"
            );
        }
    }
    Ok(())
}

#[test]
fn authored_participation_true_false_unknown_remains_coordinate_exact() -> Result<(), Box<dyn Error>>
{
    let requirement: eqm_domain::FullRequirementId =
        "account.create.signup.identifier#email_default".parse()?;
    for (predicate, region, reason) in [
        ("always = true", Some("us"), None),
        (
            "always = false",
            Some("us"),
            Some(UnresolvedReason::NotApplicable),
        ),
        (
            "dimension = \"region\"\noperator = \"eq\"\nvalue = \"us\"",
            None,
            Some(UnresolvedReason::UnknownParticipation),
        ),
    ] {
        let (_repository, loaded) = support::loaded_example_with(|root| {
            let path = root.join("eqm/contracts/participation.signup.identifier.toml");
            std::fs::write(
                &path,
                std::fs::read_to_string(&path)?.replace("always = true", predicate),
            )?;
            Ok(())
        })?;
        let finalized = finalize(&loaded)?;
        let selected = select_policy_profiles(
            finalized.graph(),
            EvaluationMode::PullRequest,
            Some(&request_value(region)?),
            None,
        )?;
        let applicability = context_for(&finalized, &selected)?;
        let universe = derive_required_universe(&finalized, &selected, &applicability, None)?;
        assert!(
            universe
                .coordinates()
                .iter()
                .any(|key| key.requirement != requirement),
            "one predicate must not globally drop other requirements"
        );
        for target in [TargetId::new("android")?, TargetId::new("ios")?] {
            let subject = ScopeSubject::Target(target.clone());
            assert_eq!(
                universe
                    .coordinates()
                    .iter()
                    .any(|key| key.requirement == requirement && key.subject == subject),
                reason.is_none(),
                "{predicate}"
            );
            if let Some(reason) = reason {
                assert!(
                    universe
                        .unresolved()
                        .iter()
                        .any(|coordinate| coordinate.requirement == requirement
                            && coordinate.subject == Some(subject.clone())
                            && coordinate.reason == reason),
                    "{predicate}: {reason:?}"
                );
            }
            if reason == Some(UnresolvedReason::UnknownParticipation) {
                let facets = universe
                    .coordinates()
                    .iter()
                    .cloned()
                    .map(|key| (key, FacetStatus::Satisfied))
                    .collect();
                let resolution = fixture_resolution(universe.clone(), facets)?;
                assert_eq!(
                    evaluate_complete_target_conformance(&target, &resolution)?,
                    TargetConformance::Nonconformant
                );
            }
        }
    }
    Ok(())
}

#[test]
fn foreign_policy_profile_definitions_and_applicability_reject_at_derivation()
-> Result<(), Box<dyn Error>> {
    let (_repository, loaded) = support::loaded_example()?;
    let finalized = finalize(&loaded)?;
    let selected = select_policy_profiles(
        finalized.graph(),
        EvaluationMode::PullRequest,
        Some(&request()?),
        None,
    )?;
    let applicability = context_for(&finalized, &selected)?;
    let baseline = derive_required_universe(&finalized, &selected, &applicability, None)?;
    for axis in ["policy", "profile"] {
        let (_foreign_repository, foreign_loaded) = support::loaded_example_with(|root| {
            let (relative, before, after) = if axis == "policy" {
                (
                    "eqm/policies/consumer.critical_flow.toml",
                    "minimum_count = 1",
                    "minimum_count = 2",
                )
            } else {
                (
                    "eqm/profiles/audience.default.toml",
                    "region = \"us\"",
                    "region = \"eu\"",
                )
            };
            let path = root.join(relative);
            let original = std::fs::read_to_string(&path)?;
            let changed = original.replace(before, after);
            assert_ne!(original, changed, "{axis} mutation must be real");
            std::fs::write(path, changed)?;
            Ok(())
        })?;
        let foreign = finalize(&foreign_loaded)?;
        assert_eq!(
            derive_required_universe(&foreign, &selected, &applicability, None),
            Err(eqm_engine::ObligationError::ForeignSelection),
            "{axis}"
        );
        let fresh = select_policy_profiles(
            foreign.graph(),
            EvaluationMode::PullRequest,
            Some(&request()?),
            None,
        )?;
        let fresh_context = context_for(&foreign, &fresh)?;
        let fresh_universe = derive_required_universe(&foreign, &fresh, &fresh_context, None)?;
        assert_eq!(
            fresh_universe.coordinates(),
            baseline.coordinates(),
            "visible keys deliberately coincide: {axis}"
        );
    }
    let eu = select_policy_profiles(
        finalized.graph(),
        EvaluationMode::PullRequest,
        Some(&request_value(Some("eu"))?),
        None,
    )?;
    let eu_context = context_for(&finalized, &eu)?;
    assert_eq!(
        derive_required_universe(&finalized, &selected, &eu_context, None),
        Err(eqm_engine::ObligationError::ForeignApplicability)
    );
    assert!(derive_required_universe(&finalized, &eu, &eu_context, None).is_ok());
    Ok(())
}

#[test]
fn distinct_context_status_maps_cannot_be_mixed() -> Result<(), Box<dyn Error>> {
    let (_repository, loaded) = support::loaded_example()?;
    let finalized = finalize(&loaded)?;
    let selection = select_policy_profiles(
        finalized.graph(),
        EvaluationMode::PullRequest,
        Some(&request()?),
        None,
    )?;
    let applicability = context(&finalized)?;
    let universe = derive_required_universe(&finalized, &selection, &applicability, None)?;
    // A status map for a different release context names coordinates outside
    // this universe and is rejected rather than silently ignored.
    let foreign: BTreeMap<_, _> = universe
        .coordinates()
        .iter()
        .take(1)
        .cloned()
        .map(|mut key| {
            key.release_context = Some(eqm_domain::Sha256Digest::hash_content(b"foreign"));
            (key, FacetStatus::Satisfied)
        })
        .collect();
    assert_eq!(
        fixture_resolution(universe, foreign),
        Err(ConformanceError::UnexpectedCoordinate)
    );
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

#[test]
fn equal_keys_do_not_erase_strength_graph_or_mode_provenance() -> Result<(), Box<dyn Error>> {
    let (_repo, loaded) = support::loaded_example()?;
    let original = finalize(&loaded)?;
    let derive = |graph: &Finalized,
                  mode,
                  origin,
                  release|
     -> Result<eqm_engine::RequiredUniverse, Box<dyn Error>> {
        let req = PolicyProfileRequest::new(
            origin,
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
        let selection = select_policy_profiles(graph.graph(), mode, Some(&req), None)?;
        let inputs = selection.profiles().values().map(|selected| {
            (
                &graph.graph().profiles()[&(selected.id().clone(), selected.revision())],
                selected.values(),
            )
        });
        let context = ApplicabilityContext::from_profiles(inputs)?;
        Ok(derive_required_universe(
            graph, &selection, &context, release,
        )?)
    };
    let weak = derive(
        &original,
        EvaluationMode::PullRequest,
        AuthorityOrigin::TrustedInvocation,
        None,
    )?;
    assert!(!weak.coordinates().is_empty());
    let inputs = weak
        .coordinates()
        .iter()
        .cloned()
        .map(|key| (key, fixture_input()))
        .collect();
    let evaluated = eqm_engine::evaluate_required_facets(&weak, inputs)?;
    // Repeated loading/semantic cloning remains reusable, without pointer identity.
    let equivalent = derive(
        &finalize(&loaded)?,
        EvaluationMode::PullRequest,
        AuthorityOrigin::TrustedInvocation,
        None,
    )?;
    CompletePolicyResolution::new(equivalent, evaluated.clone())?;
    for (mode, origin, release) in [
        (
            EvaluationMode::Development,
            AuthorityOrigin::TrustedInvocation,
            None,
        ),
        (
            EvaluationMode::PullRequest,
            AuthorityOrigin::ProtectedBaseline,
            None,
        ),
        (
            EvaluationMode::PullRequest,
            AuthorityOrigin::TrustedInvocation,
            Some(eqm_domain::Sha256Digest::hash_content(b"release")),
        ),
    ] {
        let changed = derive(&original, mode, origin, release)?;
        assert_eq!(
            CompletePolicyResolution::new(changed, evaluated.clone()),
            Err(ConformanceError::ForeignEvaluationContext)
        );
    }
    for mutation in ["strength", "profile", "contract"] {
        let (_repo, changed) = support::loaded_example_with(|root| {
            let path = match mutation {
                "strength" => root.join("eqm/policies/consumer.critical_flow.toml"),
                "profile" => root.join("eqm/profiles/audience.default.toml"),
                _ => root.join("eqm/contracts/signup.toml"),
            };
            // Contract filename is discovered by its exact schema/unit, not assumed.
            let path = if mutation == "contract" {
                std::fs::read_dir(root.join("eqm/contracts"))?
                    .filter_map(Result::ok)
                    .map(|e| e.path())
                    .find(|p| {
                        p.file_name()
                            .is_some_and(|n| n == "auth.signup.identifier.toml")
                    })
                    .ok_or("identifier fixture missing")?
            } else {
                path
            };
            let text = std::fs::read_to_string(&path)?;
            let text = match mutation {
                "strength" => text.replace("minimum_count = 1", "minimum_count = 2"),
                "profile" => text.replace("region = \"us\"", "region = \"eu\""),
                _ => text.replace("Email is selected", "Email must be selected"),
            };
            std::fs::write(path, text)?;
            Ok(())
        })?;
        let changed = derive(
            &finalize(&changed)?,
            EvaluationMode::PullRequest,
            AuthorityOrigin::TrustedInvocation,
            None,
        )?;
        assert_eq!(
            weak.coordinates(),
            changed.coordinates(),
            "{mutation}: test must retain visible keys"
        );
        assert_eq!(
            CompletePolicyResolution::new(changed, evaluated.clone()),
            Err(ConformanceError::ForeignEvaluationContext),
            "{mutation}"
        );
    }
    Ok(())
}

#[test]
fn omitted_provider_coordinate_cannot_yield_conformant() -> Result<(), Box<dyn Error>> {
    let (_repository, loaded) = support::loaded_example_with(|root| {
        let path = root.join("eqm/contracts/auth.signup.otp.toml");
        let text = std::fs::read_to_string(&path)?.replace(
            "scope = \"end_to_end\"",
            "scope = \"shared_provider\"\nprovider = \"provider.required\"",
        );
        std::fs::write(path, text)?;
        std::fs::write(
            root.join("eqm/contracts/provider.required.toml"),
            r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/provider.schema.json"
id = "provider.required"
configuration_digest = "sha256:071ca2227754705837aa3ef9748ed59e9f8a015fd765c42f391a4cbc271c6d5e"
owners = ["owner://team/providers"]
"#,
        )?;
        Ok(())
    })?;
    let finalized = finalize(&loaded)?;
    let selection = select_policy_profiles(
        finalized.graph(),
        EvaluationMode::PullRequest,
        Some(&request()?),
        None,
    )?;
    let universe = derive_required_universe(&finalized, &selection, &context(&finalized)?, None)?;
    assert!(universe.unresolved().is_empty());
    let provider = universe
        .coordinates()
        .iter()
        .find(|key| matches!(key.subject, ScopeSubject::Provider(_)))
        .cloned()
        .ok_or("provider coordinate missing")?;
    let mut facets: BTreeMap<_, _> = universe
        .coordinates()
        .iter()
        .cloned()
        .map(|key| (key, FacetStatus::Satisfied))
        .collect();
    let complete = fixture_resolution(universe.clone(), facets.clone())?;
    assert_eq!(
        evaluate_complete_target_conformance(&TargetId::new("android")?, &complete)?,
        TargetConformance::Conformant
    );
    facets.remove(&provider);
    let incomplete = fixture_resolution(universe, facets)?;
    assert_eq!(
        evaluate_complete_target_conformance(&TargetId::new("android")?, &incomplete),
        Err(ConformanceError::IncompletePolicyResolution)
    );
    Ok(())
}
