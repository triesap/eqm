//! S052 owning exact-scope controls over real authored workspace ingestion.

mod support;

use eqm_domain::{
    DimensionId, FullRequirementId, PolicyId, ProfileId, RequirementScenarioBindings, Revision,
    ScenarioId, ScenarioRef, ScenarioSubject, SymbolicValueId, TargetId,
};
use eqm_engine::{
    ApplicabilityContext, AuthorityOrigin, CompletePolicyResolution, ConformanceError,
    CoverageStatus, EvaluationMode, EvidenceOutcome, FacetEvaluationInput, FinalizedWorkspaceGraph,
    FragmentDigestMap, FreshnessStatus, ObligationError, PolicyProfileRequest, PolicyRef,
    ProfileRequest, RequiredUniverse, ScopeBindingFailure, ScopeSubject, SupportingCheck,
    TargetConformance, TrustEvaluation, TruthValue, UnresolvedReason, WaiverEvaluation,
    derive_required_universe, derive_required_universe_with_scope_bindings,
    evaluate_complete_target_conformance, evaluate_required_facets, expand_fragments,
    resolve_graph, select_policy_profiles,
};
use eqm_manifest::{LoadedWorkspace, canonicalize_fragment};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;
use std::path::Path;

const REQUIREMENT: &str = "account.create.signup.otp#valid_code_advances";
const SCENARIO: &str = "scenario.fixture_signup";

fn finalize(loaded: &LoadedWorkspace) -> Result<FinalizedWorkspaceGraph, Box<dyn Error>> {
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let digests: FragmentDigestMap = graph
        .fragments()
        .iter()
        .map(|(key, value)| Ok((key.clone(), canonicalize_fragment(value)?.digest())))
        .collect::<Result<_, Box<dyn Error>>>()?;
    Ok(expand_fragments(graph, &digests, loaded.source_map())?)
}

fn bindings(
    requirement: &str,
    scenario: &str,
    revision: u64,
) -> Result<RequirementScenarioBindings, Box<dyn Error>> {
    Ok(RequirementScenarioBindings::new(vec![(
        FullRequirementId::new(requirement)?,
        ScenarioRef::new(ScenarioId::new(scenario)?, Revision::new(revision)?),
    )])?)
}

fn universe(
    graph: &FinalizedWorkspaceGraph,
    map: &RequirementScenarioBindings,
    region: Option<&str>,
) -> Result<RequiredUniverse, Box<dyn Error>> {
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
        graph.graph(),
        EvaluationMode::PullRequest,
        Some(&request),
        None,
    )?;
    let profile = graph
        .graph()
        .profiles()
        .get(&(ProfileId::new("audience.default")?, Revision::new(1)?))
        .ok_or("fixture profile absent")?;
    let context = ApplicabilityContext::new(
        profile,
        BTreeMap::from([(
            DimensionId::new("region")?,
            region.map(SymbolicValueId::new).transpose()?,
        )]),
    )?;
    Ok(derive_required_universe_with_scope_bindings(
        graph, &selection, &context, None, map,
    )?)
}

// Synthetic facet facts establish only scope completeness, never real evidence
// trust, product integration or operational release acceptance.
fn evaluated(universe: &RequiredUniverse) -> Result<eqm_engine::EvaluatedFacets, ConformanceError> {
    evaluate_required_facets(
        universe,
        universe
            .coordinates()
            .iter()
            .cloned()
            .map(|key| {
                (
                    key,
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
                    },
                )
            })
            .collect(),
    )
}

fn scene(universe: &RequiredUniverse) -> Result<&eqm_engine::ScenarioScope, Box<dyn Error>> {
    universe
        .coordinates()
        .iter()
        .find_map(|key| match &key.subject {
            ScopeSubject::Scenario(scope) if key.requirement.as_str() == REQUIREMENT => Some(scope),
            _ => None,
        })
        .ok_or_else(|| "exact scenario coordinate absent".into())
}

fn provider_definition(root: &Path) -> Result<(), Box<dyn Error>> {
    fs::write(
        root.join("eqm/contracts/provider.required.toml"),
        r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/provider.schema.json"
id = "provider.required"
configuration_digest = "sha256:071ca2227754705837aa3ef9748ed59e9f8a015fd765c42f391a4cbc271c6d5e"
owners = ["owner://team/providers"]
"#,
    )?;
    Ok(())
}

fn provider_workspace(
    root: &Path,
    assignment: bool,
    unrelated: bool,
) -> Result<(), Box<dyn Error>> {
    provider_definition(root)?;
    let path = root.join("eqm.toml");
    let text = fs::read_to_string(&path)?.replace(
        "[targets.android]",
        "[targets.android]\ndepends_on = [\"ios\"]",
    );
    let mut text = if assignment {
        text.replace(
            "[targets.ios]",
            "[targets.ios]\nproviders = [\"provider.required\"]",
        )
    } else {
        text
    };
    if unrelated {
        fs::create_dir_all(root.join("targets/web"))?;
        text.push_str("\n[targets.web]\nroot = \"targets/web\"\nplatform = \"web\"\nframework = \"fixture\"\nowners = [\"owner://team/web\"]\n");
        let policy = root.join("eqm/policies/consumer.critical_flow.toml");
        fs::write(
            &policy,
            fs::read_to_string(&policy)?.replace(
                "required_targets = [\"android\", \"ios\"]",
                "required_targets = [\"android\", \"ios\", \"web\"]",
            ),
        )?;
    }
    fs::write(path, text)?;
    Ok(())
}

#[test]
fn exact_scene_preserves_repeated_actors_and_omission_cannot_pass() -> Result<(), Box<dyn Error>> {
    let (_root, loaded) = support::loaded_example()?;
    let graph = finalize(&loaded)?;
    let required = universe(&graph, &bindings(REQUIREMENT, SCENARIO, 1)?, Some("us"))?;
    assert!(required.unresolved().is_empty());
    let scope = scene(&required)?;
    assert_eq!(
        scope.scenario(),
        &ScenarioRef::new(ScenarioId::new(SCENARIO)?, Revision::new(1)?)
    );
    assert_eq!(scope.participants().len(), 3);
    assert_eq!(scope.participants().iter().filter(|actor| matches!(actor.subject(), ScenarioSubject::Target(id) if id.as_str() == "android")).count(), 2);
    assert_eq!(scope.dependencies().len(), 1);
    assert_eq!(
        scope.selected_targets(),
        &BTreeSet::from([TargetId::new("android")?, TargetId::new("ios")?])
    );
    assert!(!required.coordinates().iter().any(|key| matches!(
        key.subject,
        ScopeSubject::TargetSet(_) | ScopeSubject::Provider(_)
    )));
    let complete = CompletePolicyResolution::new(required.clone(), evaluated(&required)?)?;
    for target in ["android", "ios"] {
        assert_eq!(
            evaluate_complete_target_conformance(&TargetId::new(target)?, &complete)?,
            TargetConformance::Conformant
        );
    }
    let mut facts = required
        .coordinates()
        .iter()
        .filter(|key| !matches!(key.subject, ScopeSubject::Scenario(_)))
        .cloned()
        .map(|key| {
            (
                key,
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
                },
            )
        })
        .collect::<BTreeMap<_, _>>();
    let partial = evaluate_required_facets(&required, std::mem::take(&mut facts))?;
    let incomplete = CompletePolicyResolution::new(required, partial)?;
    assert_eq!(
        evaluate_complete_target_conformance(&TargetId::new("android")?, &incomplete),
        Err(ConformanceError::IncompletePolicyResolution)
    );
    Ok(())
}

#[test]
fn absent_map_never_creates_implicit_end_to_end_scope() -> Result<(), Box<dyn Error>> {
    let (_root, loaded) = support::loaded_example()?;
    let graph = finalize(&loaded)?;
    let empty = RequirementScenarioBindings::new(Vec::new())?;
    let required = universe(&graph, &empty, Some("us"))?;
    assert!(!required.coordinates().iter().any(|key| matches!(
        key.subject,
        ScopeSubject::Scenario(_) | ScopeSubject::TargetSet(_)
    )));
    assert!(
        required
            .unresolved()
            .iter()
            .any(|coordinate| coordinate.requirement.as_str() == REQUIREMENT
                && coordinate.subject.is_none()
                && coordinate.reason == UnresolvedReason::MissingScopeAuthority)
    );
    let complete = CompletePolicyResolution::new(required.clone(), evaluated(&required)?)?;
    for target in ["android", "ios"] {
        assert_eq!(
            evaluate_complete_target_conformance(&TargetId::new(target)?, &complete)?,
            TargetConformance::Nonconformant
        );
    }
    // Execute the production no-map wrapper as well, rather than merely a
    // helper-created empty map.
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
        graph.graph(),
        EvaluationMode::PullRequest,
        Some(&request),
        None,
    )?;
    let context = ApplicabilityContext::from_profiles(
        selection
            .profiles()
            .values()
            .map(|selected| {
                let definition = graph
                    .graph()
                    .profiles()
                    .get(&(selected.id().clone(), selected.revision()))
                    .ok_or("profile")?;
                Ok((definition, selected.values()))
            })
            .collect::<Result<Vec<_>, Box<dyn Error>>>()?,
    )?;
    assert_eq!(
        derive_required_universe(&graph, &selection, &context, None)?,
        required
    );
    Ok(())
}

#[test]
fn unknown_incompatible_and_wrong_revision_bindings_reject_exactly() -> Result<(), Box<dyn Error>> {
    let (_root, loaded) = support::loaded_example()?;
    let graph = finalize(&loaded)?;
    universe(&graph, &bindings(REQUIREMENT, SCENARIO, 1)?, Some("us"))?;
    for (requirement, scenario, revision, expected) in [
        (
            "account.create.signup.otp#missing",
            SCENARIO,
            1,
            ScopeBindingFailure::UnknownRequirement,
        ),
        (
            "account.create.signup.identifier#email_default",
            SCENARIO,
            1,
            ScopeBindingFailure::NotEndToEnd,
        ),
        (
            REQUIREMENT,
            "scenario.missing",
            1,
            ScopeBindingFailure::UnknownScenario,
        ),
        (
            REQUIREMENT,
            SCENARIO,
            2,
            ScopeBindingFailure::UnknownScenario,
        ),
    ] {
        let error = universe(
            &graph,
            &bindings(requirement, scenario, revision)?,
            Some("us"),
        )
        .err()
        .ok_or("invalid binding admitted")?;
        assert_eq!(
            error.downcast_ref::<ObligationError>(),
            Some(&ObligationError::InvalidScopeBinding {
                requirement: FullRequirementId::new(requirement)?,
                scenario: ScenarioRef::new(ScenarioId::new(scenario)?, Revision::new(revision)?),
                reason: expected,
            })
        );
    }
    Ok(())
}

#[test]
fn false_unknown_and_mixed_scene_participation_preserves_every_occurrence()
-> Result<(), Box<dyn Error>> {
    for (case, expected) in [
        ("false", UnresolvedReason::NotApplicable),
        ("unknown", UnresolvedReason::UnknownParticipation),
        ("mixed", UnresolvedReason::UnknownParticipation),
    ] {
        let (_root, loaded) = support::loaded_example_with(|root| {
            let path = root.join("eqm/contracts/participation.signup.otp_advance.toml");
            let mut text = fs::read_to_string(&path)?;
            if case == "unknown" {
                text = text.replace(
                    "always = true",
                    "dimension = \"region\"\noperator = \"eq\"\nvalue = \"us\"",
                );
            } else {
                text = text.replace("always = true", "always = false");
                if case == "mixed" {
                    text = text.replace(
                        "targets = [\"android\", \"ios\"]",
                        "targets = [\"android\"]",
                    );
                    fs::write(
                        root.join("eqm/contracts/participation.scene_ios.toml"),
                        format!(
                            "schema = \"https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/participation.schema.json\"\nid = \"participation.scene_ios\"\nrequirement = \"{REQUIREMENT}\"\ntargets = [\"ios\"]\n[applicability]\nalways = true\n"
                        ),
                    )?;
                }
            }
            fs::write(path, text)?;
            Ok(())
        })?;
        let graph = finalize(&loaded)?;
        let required = universe(
            &graph,
            &bindings(REQUIREMENT, SCENARIO, 1)?,
            if case == "unknown" { None } else { Some("us") },
        )?;
        assert!(
            !required
                .coordinates()
                .iter()
                .any(|key| matches!(key.subject, ScopeSubject::Scenario(_)))
        );
        let coordinate = required
            .unresolved()
            .iter()
            .find(|coordinate| coordinate.requirement.as_str() == REQUIREMENT)
            .ok_or("scene outcome missing")?;
        assert_eq!(coordinate.reason, expected, "{case}");
        let Some(ScopeSubject::Scenario(scope)) = &coordinate.subject else {
            return Err("scene authority was lost".into());
        };
        assert_eq!(scope.participants().len(), 3, "{case}");
        assert_eq!(scope.dependencies().len(), 1, "{case}");
        assert_eq!(scope.selected_targets().len(), 2, "{case}");
        assert_eq!(scope.participation().len(), 2, "{case}");
        if case == "mixed" {
            assert_eq!(
                scope.participation().get(&TargetId::new("android")?),
                Some(&TruthValue::False)
            );
            assert_eq!(
                scope.participation().get(&TargetId::new("ios")?),
                Some(&TruthValue::True)
            );
        }
    }
    Ok(())
}

#[test]
fn outside_cohort_actors_stay_visible_without_enlarging_required_targets()
-> Result<(), Box<dyn Error>> {
    for unknown_actor in [false, true] {
        let (_root, loaded) = support::loaded_example_with(|root| {
            let policy = root.join("eqm/policies/consumer.critical_flow.toml");
            fs::write(
                &policy,
                fs::read_to_string(&policy)?.replace(
                    "required_targets = [\"android\", \"ios\"]",
                    "required_targets = [\"android\"]",
                ),
            )?;
            if unknown_actor {
                let participation =
                    root.join("eqm/contracts/participation.signup.otp_advance.toml");
                fs::write(
                    &participation,
                    fs::read_to_string(&participation)?.replace(
                        "targets = [\"android\", \"ios\"]",
                        "targets = [\"android\"]",
                    ),
                )?;
            }
            Ok(())
        })?;
        let graph = finalize(&loaded)?;
        let required = universe(&graph, &bindings(REQUIREMENT, SCENARIO, 1)?, Some("us"))?;
        let scope = if unknown_actor {
            let coordinate = required
                .unresolved()
                .iter()
                .find(|coordinate| coordinate.requirement.as_str() == REQUIREMENT)
                .ok_or("unknown actor absent")?;
            assert_eq!(coordinate.reason, UnresolvedReason::UnknownParticipation);
            let Some(ScopeSubject::Scenario(scope)) = &coordinate.subject else {
                return Err("scene lost".into());
            };
            scope
        } else {
            scene(&required)?
        };
        assert_eq!(scope.participants().len(), 3);
        assert_eq!(
            scope.selected_targets(),
            &BTreeSet::from([TargetId::new("android")?])
        );
        assert!(
            !required.coordinates().iter().any(
                |key| matches!(&key.subject, ScopeSubject::Target(id) if id.as_str() == "ios")
            )
        );
        let complete = CompletePolicyResolution::new(required.clone(), evaluated(&required)?)?;
        assert_eq!(
            evaluate_complete_target_conformance(&TargetId::new("android")?, &complete)?,
            if unknown_actor {
                TargetConformance::Nonconformant
            } else {
                TargetConformance::Conformant
            }
        );
    }
    Ok(())
}

#[test]
fn provider_closure_uses_declared_paths_and_ignores_unrelated_participation()
-> Result<(), Box<dyn Error>> {
    let (_root, loaded) = support::loaded_example_with(|root| {
        provider_workspace(root, true, true)?;
        let contract = root.join("eqm/contracts/auth.signup.otp.toml");
        fs::write(
            &contract,
            fs::read_to_string(&contract)?.replace(
                "scope = \"end_to_end\"",
                "scope = \"shared_provider\"\nprovider = \"provider.required\"",
            ),
        )?;
        Ok(())
    })?;
    let graph = finalize(&loaded)?;
    let required = universe(
        &graph,
        &RequirementScenarioBindings::new(Vec::new())?,
        Some("us"),
    )?;
    let key = required
        .coordinates()
        .iter()
        .find(|key| matches!(key.subject, ScopeSubject::CheckedProvider(_)))
        .ok_or("provider missing")?;
    let ScopeSubject::CheckedProvider(scope) = &key.subject else {
        return Err("provider scope lost".into());
    };
    assert_eq!(
        scope.dependers(),
        &BTreeSet::from([TargetId::new("android")?, TargetId::new("ios")?])
    );
    assert_eq!(
        scope.assignments(),
        &BTreeSet::from([TargetId::new("ios")?])
    );
    assert_eq!(
        scope.dependencies(),
        &BTreeSet::from([(TargetId::new("android")?, TargetId::new("ios")?)])
    );
    assert!(!key.subject.participates(&TargetId::new("web")?));
    assert!(required.unresolved().iter().any(|coordinate| matches!(&coordinate.subject, Some(ScopeSubject::Target(id)) if id.as_str() == "web")));
    assert!(
        !required
            .unresolved()
            .iter()
            .any(|coordinate| coordinate.requirement.as_str() == REQUIREMENT)
    );
    let complete = CompletePolicyResolution::new(required.clone(), evaluated(&required)?)?;
    assert_eq!(
        evaluate_complete_target_conformance(&TargetId::new("android")?, &complete)?,
        TargetConformance::Conformant
    );
    Ok(())
}

#[test]
fn provider_only_scene_requires_actual_assigned_dependers() -> Result<(), Box<dyn Error>> {
    for assignment in [true, false] {
        let (_root, loaded) = support::loaded_example_with(|root| {
            provider_workspace(root, assignment, false)?;
            fs::write(
                root.join("eqm/contracts/scenario.fixture_signup.toml"),
                format!(
                    "schema = \"https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/scenario.schema.json\"\nid = \"{SCENARIO}\"\nrevision = 1\n[[participants]]\nid = \"mailer\"\nrole = \"provider\"\n[participants.subject]\nprovider = \"provider.required\"\n"
                ),
            )?;
            Ok(())
        })?;
        let graph = finalize(&loaded)?;
        let required = universe(&graph, &bindings(REQUIREMENT, SCENARIO, 1)?, Some("us"))?;
        if assignment {
            let scope = scene(&required)?;
            assert_eq!(scope.participants().len(), 1);
            assert_eq!(scope.selected_targets().len(), 2);
            assert_eq!(scope.provider_dependencies().len(), 1);
            assert!(required.unresolved().is_empty());
        } else {
            assert!(required.unresolved().iter().any(|coordinate| coordinate.requirement.as_str() == REQUIREMENT
                && coordinate.reason == UnresolvedReason::MissingScopeAuthority
                && matches!(&coordinate.subject, Some(ScopeSubject::Scenario(scope)) if scope.participants().len() == 1)));
            let complete = CompletePolicyResolution::new(required.clone(), evaluated(&required)?)?;
            assert_eq!(
                evaluate_complete_target_conformance(&TargetId::new("android")?, &complete)?,
                TargetConformance::Nonconformant
            );
        }
    }
    Ok(())
}

#[test]
fn extra_valid_binding_does_not_enlarge_scope_or_erase_private_map_provenance()
-> Result<(), Box<dyn Error>> {
    let (_root, loaded) = support::loaded_example_with(|root| {
        let source = fs::read_to_string(root.join("eqm/contracts/auth.signup.otp.toml"))?;
        let extra = source
            .replace(
                "id = \"account.create.signup.otp\"",
                "id = \"account.create.signup.extra\"",
            )
            .replace(
                "[[fragments]]",
                "[requirements.applicability]\nalways = false\n\n[[fragments]]",
            );
        fs::write(root.join("eqm/contracts/extra.toml"), extra)?;
        let journey = root.join("eqm/contracts/auth.signup.toml");
        let original = fs::read_to_string(&journey)?;
        let expanded = original.replace("surfaces = [\"account.create.signup.identifier\", \"account.create.signup.otp\"]", "surfaces = [\"account.create.signup.identifier\", \"account.create.signup.otp\", \"account.create.signup.extra\"]");
        fs::write(
            journey,
            format!(
                "{expanded}\n[[transitions]]\nfrom = \"account.create.signup.otp\"\nto = \"account.create.signup.extra\"\ntrigger = \"fixture_next\"\n"
            ),
        )?;
        Ok(())
    })?;
    let graph = finalize(&loaded)?;
    let base = bindings(REQUIREMENT, SCENARIO, 1)?;
    let mut entries = base
        .bindings()
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect::<Vec<_>>();
    entries.push((
        FullRequirementId::new("account.create.signup.extra#valid_code_advances")?,
        ScenarioRef::new(ScenarioId::new(SCENARIO)?, Revision::new(1)?),
    ));
    let extra = RequirementScenarioBindings::new(entries)?;
    let baseline = universe(&graph, &base, Some("us"))?;
    let extended = universe(&graph, &extra, Some("us"))?;
    assert_eq!(baseline.coordinates(), extended.coordinates());
    assert_ne!(baseline.scope_bindings(), extended.scope_bindings());
    let facts = evaluated(&baseline)?;
    CompletePolicyResolution::new(baseline.clone(), facts.clone())?;
    assert_eq!(
        CompletePolicyResolution::new(extended, facts),
        Err(ConformanceError::ForeignEvaluationContext)
    );
    Ok(())
}

#[test]
fn provider_missing_assignment_and_actual_depender_unknown_cannot_pass()
-> Result<(), Box<dyn Error>> {
    for case in ["valid", "unassigned", "unknown"] {
        let (_root, loaded) = support::loaded_example_with(|root| {
            provider_workspace(root, case != "unassigned", false)?;
            let contract = root.join("eqm/contracts/auth.signup.otp.toml");
            fs::write(
                &contract,
                fs::read_to_string(&contract)?.replace(
                    "scope = \"end_to_end\"",
                    "scope = \"shared_provider\"\nprovider = \"provider.required\"",
                ),
            )?;
            if case == "unknown" {
                let path = root.join("eqm/contracts/participation.signup.otp_advance.toml");
                fs::write(
                    &path,
                    fs::read_to_string(&path)?
                        .replace("targets = [\"android\", \"ios\"]", "targets = [\"ios\"]"),
                )?;
            }
            Ok(())
        })?;
        let graph = finalize(&loaded)?;
        let required = universe(
            &graph,
            &RequirementScenarioBindings::new(Vec::new())?,
            Some("us"),
        )?;
        if case == "valid" {
            assert!(required.unresolved().is_empty());
            assert!(
                required
                    .coordinates()
                    .iter()
                    .any(|key| matches!(key.subject, ScopeSubject::CheckedProvider(_)))
            );
        } else {
            let unresolved = required
                .unresolved()
                .iter()
                .find(|coordinate| coordinate.requirement.as_str() == REQUIREMENT)
                .ok_or("provider unknown omitted")?;
            assert_eq!(
                unresolved.reason,
                if case == "unassigned" {
                    UnresolvedReason::MissingScopeAuthority
                } else {
                    UnresolvedReason::UnknownParticipation
                }
            );
            if case == "unknown" {
                assert!(
                    matches!(&unresolved.subject, Some(ScopeSubject::CheckedProvider(scope))
                    if scope.dependers().len() == 2 && scope.participation().get(&TargetId::new("android")?) == Some(&TruthValue::Unknown))
                );
            }
        }
        let complete = CompletePolicyResolution::new(required.clone(), evaluated(&required)?)?;
        assert_eq!(
            evaluate_complete_target_conformance(&TargetId::new("android")?, &complete)?,
            if case == "valid" {
                TargetConformance::Conformant
            } else {
                TargetConformance::Nonconformant
            }
        );
    }
    Ok(())
}

#[test]
fn scenario_outside_entire_policy_cohort_retains_scope_but_cannot_pass()
-> Result<(), Box<dyn Error>> {
    for outside_only in [false, true] {
        let (_root, loaded) = support::loaded_example_with(|root| {
            let path = root.join("eqm/policies/consumer.critical_flow.toml");
            fs::write(
                &path,
                fs::read_to_string(&path)?.replace(
                    "required_targets = [\"android\", \"ios\"]",
                    "required_targets = [\"android\"]",
                ),
            )?;
            support::write_scenario(root, if outside_only { &["ios"] } else { &["android"] })?;
            Ok(())
        })?;
        let graph = finalize(&loaded)?;
        let required = universe(&graph, &bindings(REQUIREMENT, SCENARIO, 1)?, Some("us"))?;
        if outside_only {
            let unresolved = required
                .unresolved()
                .iter()
                .find(|coordinate| coordinate.requirement.as_str() == REQUIREMENT)
                .ok_or("outside-only scene disappeared")?;
            assert_eq!(unresolved.reason, UnresolvedReason::MissingScopeAuthority);
            assert!(
                matches!(&unresolved.subject, Some(ScopeSubject::Scenario(scope))
                if scope.selected_targets().is_empty() && scope.participants().len() == 2 && scope.dependencies().len() == 1)
            );
        } else {
            assert_eq!(scene(&required)?.selected_targets().len(), 1);
        }
        let complete = CompletePolicyResolution::new(required.clone(), evaluated(&required)?)?;
        assert_eq!(
            evaluate_complete_target_conformance(&TargetId::new("android")?, &complete)?,
            if outside_only {
                TargetConformance::Nonconformant
            } else {
                TargetConformance::Conformant
            }
        );
    }
    Ok(())
}

#[test]
fn legacy_waiver_cannot_flatten_exact_scenario_subject() -> Result<(), Box<dyn Error>> {
    use eqm_domain::{
        EvidenceScopeSubject, Extensions, IssueRef, Waiver, WaiverId, WaiverProfileScope,
        WaiverReason, WaiverScope,
    };
    use eqm_engine::{WaivableStatus, WaiverInvalidReason, evaluate_waivers};
    let (_root, loaded) = support::loaded_example()?;
    let graph = finalize(&loaded)?;
    let required = universe(&graph, &bindings(REQUIREMENT, SCENARIO, 1)?, Some("us"))?;
    let key = required
        .coordinates()
        .iter()
        .find(|key| matches!(key.subject, ScopeSubject::Scenario(_)))
        .ok_or("scene")?;
    let ScopeSubject::Scenario(scene) = &key.subject else {
        return Err("scene".into());
    };
    let legacy = ScopeSubject::TargetSet(scene.selected_targets().clone());
    let template = graph
        .graph()
        .waivers()
        .values()
        .next()
        .ok_or("waiver template")?;
    let scope = WaiverScope::new(
        EvidenceScopeSubject::TargetSet(scene.selected_targets().clone()),
        key.unit.clone(),
        key.requirement.clone(),
        vec![key.facet],
        key.profiles
            .values()
            .map(|selected| {
                Ok(WaiverProfileScope::new(
                    selected.id().clone(),
                    selected
                        .values()
                        .iter()
                        .map(|(dimension, value)| {
                            Ok((dimension.clone(), value.clone().ok_or("unknown")?))
                        })
                        .collect::<Result<Vec<_>, Box<dyn Error>>>()?,
                )?)
            })
            .collect::<Result<Vec<_>, Box<dyn Error>>>()?,
    )?;
    // Synthetic scope-boundary control only, no production waiver is created.
    let waiver = Waiver::new(
        WaiverId::new("waiver.synthetic_scope")?,
        Revision::new(1)?,
        template.owners().iter().cloned().collect(),
        key.policy.clone(),
        scope,
        WaiverReason::new("Synthetic exact scope regression")?,
        "issue://SCOPE-1".parse::<IssueRef>()?,
        template.approvers().iter().cloned().collect(),
        template.starts_on(),
        template.expires_on(),
        template.controls().iter().copied().collect(),
        Extensions::default(),
    )?;
    let policy = graph
        .graph()
        .policies()
        .get(&(key.policy.clone(), key.policy_revision))
        .ok_or("policy")?;
    let evaluated_on = "2026-08-15".parse()?;
    let evaluate = |key| {
        evaluate_waivers(
            &[&waiver],
            policy,
            key,
            WaivableStatus::Missing,
            evaluated_on,
            template.approvers(),
            template.controls(),
        )
    };
    let mut reference = key.clone();
    reference.subject = legacy;
    assert_eq!(
        evaluate(&reference),
        WaiverEvaluation::Waived(waiver.id().clone())
    );
    assert_eq!(
        evaluate(key),
        WaiverEvaluation::NotApplied(BTreeMap::from([(
            waiver.id().clone(),
            BTreeSet::from([WaiverInvalidReason::Subject])
        )]))
    );
    Ok(())
}

#[test]
fn false_provider_depender_is_not_promoted_by_an_unknown_depender() -> Result<(), Box<dyn Error>> {
    let (_root, loaded) = support::loaded_example_with(|root| {
        provider_workspace(root, true, false)?;
        let path = root.join("eqm/contracts/auth.signup.otp.toml");
        fs::write(
            &path,
            fs::read_to_string(&path)?.replace(
                "scope = \"end_to_end\"",
                "scope = \"shared_provider\"\nprovider = \"provider.required\"",
            ),
        )?;
        let path = root.join("eqm/contracts/participation.signup.otp_advance.toml");
        fs::write(
            &path,
            fs::read_to_string(&path)?
                .replace(
                    "targets = [\"android\", \"ios\"]",
                    "targets = [\"android\"]",
                )
                .replace("always = true", "always = false"),
        )?;
        Ok(())
    })?;
    let graph = finalize(&loaded)?;
    let required = universe(
        &graph,
        &RequirementScenarioBindings::new(Vec::new())?,
        Some("us"),
    )?;
    let unresolved = required
        .unresolved()
        .iter()
        .find(|coordinate| coordinate.requirement.as_str() == REQUIREMENT)
        .ok_or("provider unknown")?;
    assert_eq!(unresolved.reason, UnresolvedReason::UnknownParticipation);
    let subject = unresolved.subject.as_ref().ok_or("subject")?;
    assert!(!subject.participates(&TargetId::new("android")?));
    assert!(subject.participates(&TargetId::new("ios")?));
    let complete = CompletePolicyResolution::new(required.clone(), evaluated(&required)?)?;
    assert_eq!(
        evaluate_complete_target_conformance(&TargetId::new("android")?, &complete)?,
        TargetConformance::Conformant
    );
    assert_eq!(
        evaluate_complete_target_conformance(&TargetId::new("ios")?, &complete)?,
        TargetConformance::Nonconformant
    );
    Ok(())
}

#[test]
fn unknown_requirement_and_false_coordinate_participation_is_visible_not_applicable()
-> Result<(), Box<dyn Error>> {
    for scope in ["each_target", "shared_provider", "end_to_end"] {
        let (_root, loaded) = support::loaded_example_with(|root| {
            if scope == "shared_provider" {
                provider_workspace(root, true, false)?;
            }
            let path = root.join("eqm/contracts/auth.signup.otp.toml");
            let replacement = if scope == "shared_provider" {
                "scope = \"shared_provider\"\nprovider = \"provider.required\"".to_owned()
            } else {
                format!("scope = \"{scope}\"")
            };
            fs::write(&path,fs::read_to_string(&path)?.replace("scope = \"end_to_end\"",&replacement).replace("facets = [\"behavior\"]","facets = [\"behavior\"]\n[requirements.applicability]\ndimension = \"region\"\noperator = \"eq\"\nvalue = \"us\""))?;
            let path = root.join("eqm/contracts/participation.signup.otp_advance.toml");
            fs::write(
                &path,
                fs::read_to_string(&path)?.replace("always = true", "always = false"),
            )?;
            Ok(())
        })?;
        let graph = finalize(&loaded)?;
        let map = if scope == "end_to_end" {
            bindings(REQUIREMENT, SCENARIO, 1)?
        } else {
            RequirementScenarioBindings::new(Vec::new())?
        };
        let required = universe(&graph, &map, None)?;
        let outcomes = required
            .unresolved()
            .iter()
            .filter(|coordinate| coordinate.requirement.as_str() == REQUIREMENT)
            .collect::<Vec<_>>();
        assert!(!outcomes.is_empty(), "{scope}");
        assert!(
            outcomes
                .iter()
                .all(|coordinate| coordinate.reason == UnresolvedReason::NotApplicable),
            "{scope}: {outcomes:?}"
        );
        assert!(
            !required
                .coordinates()
                .iter()
                .any(|key| key.requirement.as_str() == REQUIREMENT)
        );
    }
    Ok(())
}

#[test]
fn unknown_requirement_cannot_hide_outside_scene_or_unassigned_provider()
-> Result<(), Box<dyn Error>> {
    for provider in [false, true] {
        let (_root, loaded) = support::loaded_example_with(|root| {
            let path = root.join("eqm/policies/consumer.critical_flow.toml");
            fs::write(
                &path,
                fs::read_to_string(&path)?.replace(
                    "required_targets = [\"android\", \"ios\"]",
                    "required_targets = [\"android\"]",
                ),
            )?;
            support::write_scenario(root, &["ios"])?;
            if provider {
                provider_workspace(root, false, false)?;
            }
            let path = root.join("eqm/contracts/auth.signup.otp.toml");
            let text=fs::read_to_string(&path)?.replace("facets = [\"behavior\"]","facets = [\"behavior\"]\n[requirements.applicability]\ndimension = \"region\"\noperator = \"eq\"\nvalue = \"us\"");
            fs::write(
                path,
                if provider {
                    text.replace(
                        "scope = \"end_to_end\"",
                        "scope = \"shared_provider\"\nprovider = \"provider.required\"",
                    )
                } else {
                    text
                },
            )?;
            Ok(())
        })?;
        let graph = finalize(&loaded)?;
        let map = if provider {
            RequirementScenarioBindings::new(Vec::new())?
        } else {
            bindings(REQUIREMENT, SCENARIO, 1)?
        };
        let required = universe(&graph, &map, None)?;
        assert!(
            required
                .unresolved()
                .iter()
                .any(|coordinate| coordinate.requirement.as_str() == REQUIREMENT
                    && coordinate.reason == UnresolvedReason::MissingScopeAuthority)
        );
        let complete = CompletePolicyResolution::new(required.clone(), evaluated(&required)?)?;
        assert_eq!(
            evaluate_complete_target_conformance(&TargetId::new("android")?, &complete)?,
            TargetConformance::Nonconformant
        );
    }
    Ok(())
}
