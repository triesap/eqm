//! Seeded real derivation checks against independently enumerated coordinates.

mod support;

use eqm_domain::{
    DimensionId, DurationMillis, Facet, Policy, PolicyId, PolicyRule, PositiveCount, ProfileId,
    RequirementLevel, Revision, Scenario, ScenarioSubject, Sha256Digest, Surface, SymbolicValueId,
    TrustLevel, WorkspaceGraphInput,
};
use eqm_engine::{
    ApplicabilityContext, AuthorityOrigin, EvaluationMode, FacetStatus, FinalizedWorkspaceGraph,
    FragmentDigestMap, MonotonicChange, MonotonicityError, ObligationError, PolicyProfileRequest,
    PolicyRef, ProfileRequest, RequiredUniverse, ScopeSubject, UnresolvedReason,
    enforce_required_universe_strengthening, expand_fragments, resolve_graph,
    select_policy_profiles,
};
use eqm_manifest::{LoadedWorkspace, canonicalize_fragment, canonicalize_graph_v2};
use std::collections::BTreeSet;
use std::error::Error;
use std::fs;

const SEED: u64 = 0x4551_4d31_3233;
const CASES: usize = 512;

fn next(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1);
    *state
}
fn permute<T>(values: &mut [T], state: &mut u64) {
    for i in (1..values.len()).rev() {
        let j = (next(state) % (i as u64 + 1)) as usize;
        values.swap(i, j);
    }
}
fn policy_with_rules(
    policy: &Policy,
    revision: u64,
    rules: Vec<PolicyRule>,
) -> Result<Policy, Box<dyn Error>> {
    Ok(Policy::new(
        policy.id().clone(),
        Revision::new(revision)?,
        policy.title().clone(),
        policy.owners().iter().cloned().collect(),
        policy.profiles().iter().cloned().collect(),
        policy.required_targets().iter().cloned().collect(),
        rules,
        policy.waivers().clone(),
        policy.description().cloned(),
        policy.extensions().clone(),
    )?)
}
fn variant(
    policy: &Policy,
    floor: RequirementLevel,
    trust: TrustLevel,
    age: u64,
    count: u64,
) -> Result<Policy, Box<dyn Error>> {
    let age = DurationMillis::new(age)?;
    let count = PositiveCount::new(count)?;
    let rules = policy
        .rules()
        .iter()
        .map(|r| {
            PolicyRule::new(
                r.selector().clone(),
                floor,
                r.facets().iter().copied().collect(),
                trust,
                age,
                Some(count),
            )
        })
        .collect::<Result<_, eqm_domain::PolicyBuildError>>()?;
    policy_with_rules(policy, 2, rules)
}
fn finalize(
    loaded: &LoadedWorkspace,
    input: WorkspaceGraphInput,
) -> Result<FinalizedWorkspaceGraph, Box<dyn Error>> {
    let graph = resolve_graph(input, loaded.source_map())?;
    let digests: FragmentDigestMap = graph
        .fragments()
        .iter()
        .map(|(k, f)| Ok((k.clone(), canonicalize_fragment(f)?.digest())))
        .collect::<Result<_, Box<dyn Error>>>()?;
    Ok(expand_fragments(graph, &digests, loaded.source_map())?)
}
fn universe(
    graph: &FinalizedWorkspaceGraph,
    revision: u64,
    region: Option<&str>,
    release: Option<Sha256Digest>,
) -> Result<RequiredUniverse, Box<dyn Error>> {
    let request = PolicyProfileRequest::new(
        AuthorityOrigin::TrustedInvocation,
        PolicyRef::new(
            PolicyId::new("consumer.critical_flow")?,
            Revision::new(revision)?,
        ),
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
    let inputs = selection
        .profiles()
        .values()
        .map(|p| {
            graph
                .graph()
                .profiles()
                .get(&(p.id().clone(), p.revision()))
                .map(|d| (d, p.values()))
                .ok_or("profile")
        })
        .collect::<Result<Vec<_>, _>>()?;
    let context = ApplicabilityContext::from_profiles(inputs)?;
    Ok(support::derive_fixture_universe(
        graph, &selection, &context, release,
    )?)
}

// This literal table is independent of production derivation. Scenario details
// are asserted below rather than accepting a generated opaque scenario hash.
fn literal_coordinates(
    extra_facet: bool,
) -> BTreeSet<(String, String, Facet, TrustLevel, u64, u64)> {
    let mut values = BTreeSet::new();
    for target in ["android", "ios"] {
        values.insert((
            "account.create.signup.identifier#email_default".into(),
            target.into(),
            Facet::Behavior,
            TrustLevel::TrustedCi,
            86_400_000,
            1,
        ));
        values.insert((
            "account.create.signup.otp#six_decimal_digits".into(),
            target.into(),
            Facet::Behavior,
            if extra_facet {
                TrustLevel::SignedCi
            } else {
                TrustLevel::TrustedCi
            },
            if extra_facet { 60_000 } else { 86_400_000 },
            if extra_facet { 2 } else { 1 },
        ));
        if extra_facet {
            values.insert((
                "account.create.signup.otp#six_decimal_digits".into(),
                target.into(),
                Facet::Accessibility,
                TrustLevel::TrustedCi,
                300_000,
                3,
            ));
        }
    }
    values.insert((
        "account.create.signup.otp#valid_code_advances".into(),
        "scenario.fixture_signup".into(),
        Facet::Behavior,
        TrustLevel::TrustedCi,
        86_400_000,
        1,
    ));
    values
}
fn assert_literal(universe: &RequiredUniverse, extra_facet: bool) -> Result<(), Box<dyn Error>> {
    assert!(universe.unresolved().is_empty());
    let actual = universe
        .obligations()
        .iter()
        .map(|(key, o)| {
            let subject = match &key.subject {
                ScopeSubject::Target(t) => t.as_str().to_owned(),
                ScopeSubject::Scenario(s) => {
                    assert_eq!(s.scenario().revision(), Revision::new(1)?);
                    let participants = s
                        .participants()
                        .iter()
                        .map(|p| {
                            let ScenarioSubject::Target(t) = p.subject() else {
                                return Err("provider unexpected");
                            };
                            Ok((p.occurrence().as_str(), p.role().as_str(), t.as_str()))
                        })
                        .collect::<Result<BTreeSet<_>, _>>()?;
                    assert_eq!(
                        participants,
                        BTreeSet::from([
                            ("actor_0", "edge", "android"),
                            ("actor_1", "edge", "ios"),
                            ("repeat_actor", "observer", "android")
                        ])
                    );
                    assert_eq!(
                        s.dependencies()
                            .iter()
                            .map(|d| (d.from().as_str(), d.to().as_str()))
                            .collect::<BTreeSet<_>>(),
                        BTreeSet::from([("repeat_actor", "actor_0")])
                    );
                    s.scenario().id().as_str().to_owned()
                }
                _ => return Err("unexpected scope".into()),
            };
            assert_eq!(key.policy.as_str(), "consumer.critical_flow");
            assert_eq!(key.policy_revision, Revision::new(1)?);
            assert_eq!(key.profiles.len(), 1);
            let profile = key
                .profiles
                .get(&ProfileId::new("audience.default")?)
                .ok_or("exact profile missing")?;
            assert_eq!(profile.id().as_str(), "audience.default");
            assert_eq!(profile.revision(), Revision::new(1)?);
            assert_eq!(profile.values().len(), 1);
            assert_eq!(
                profile.values().get(&DimensionId::new("region")?),
                Some(&Some(SymbolicValueId::new("us")?))
            );
            assert!(key.release_context.is_none());
            assert_eq!(key.unit.as_str(), key.requirement.owner());
            Ok((
                key.requirement.to_string(),
                subject,
                key.facet,
                o.strength.minimum_trust,
                o.strength.maximum_age.get(),
                o.strength.minimum_count.get(),
            ))
        })
        .collect::<Result<BTreeSet<_>, Box<dyn Error>>>()?;
    assert_eq!(actual, literal_coordinates(extra_facet));
    Ok(())
}

#[test]
fn seeded_unordered_inputs_preserve_real_derivation_and_semantic_identity()
-> Result<(), Box<dyn Error>> {
    let (_root, loaded) = support::loaded_example_with(|root| {
        let path = root.join("eqm/policies/consumer.critical_flow.toml");
        let mut text = fs::read_to_string(&path)?;
        text.push_str("\n[[rules]]\nminimum_level = \"required\"\nfacets = [\"behavior\"]\nminimum_trust = \"signed_ci\"\nmaximum_age = 60000\nminimum_count = 2\n[rules.selector]\nrequirements = [\"account.create.signup.otp#six_decimal_digits\"]\n\n[[rules]]\nminimum_level = \"required\"\nfacets = [\"accessibility\"]\nminimum_trust = \"trusted_ci\"\nmaximum_age = 300000\nminimum_count = 3\n[rules.selector]\nrequirements = [\"account.create.signup.otp#six_decimal_digits\"]\n");
        fs::write(path, text)?;
        Ok(())
    })?;
    let initial = finalize(&loaded, loaded.graph_input().clone())?;
    assert_literal(&universe(&initial, 1, Some("us"), None)?, true)?;
    let digest = canonicalize_graph_v2(initial.graph())?.digest();
    let mut state = SEED;
    for case in 0..CASES {
        let mut input = loaded.graph_input().clone();
        for p in &mut input.policies {
            let mut rules = p.rules().to_vec();
            permute(&mut rules, &mut state);
            *p = policy_with_rules(p, p.revision().get(), rules)?;
        }
        for s in &mut input.surfaces {
            let mut requirements = s.requirements().values().cloned().collect::<Vec<_>>();
            permute(&mut requirements, &mut state);
            *s = Surface::new(
                s.id().clone(),
                s.revision(),
                s.title().clone(),
                s.journey().clone(),
                s.status(),
                s.owners().iter().cloned().collect(),
                requirements,
                s.fragments().iter().cloned().collect(),
                s.description().cloned(),
                s.extensions().clone(),
            )?;
        }
        for s in &mut input.scenarios {
            let mut participants = s.participants().iter().cloned().collect::<Vec<_>>();
            permute(&mut participants, &mut state);
            let mut dependencies = s.dependencies().iter().cloned().collect::<Vec<_>>();
            permute(&mut dependencies, &mut state);
            *s = Scenario::new(
                s.id().clone(),
                s.revision(),
                participants,
                dependencies,
                s.extensions().clone(),
            )?;
        }
        permute(&mut input.targets, &mut state);
        permute(&mut input.surfaces, &mut state);
        permute(&mut input.bindings, &mut state);
        permute(&mut input.participation, &mut state);
        permute(&mut input.policies, &mut state);
        permute(&mut input.profiles, &mut state);
        let graph = finalize(&loaded, input)?;
        assert_eq!(
            canonicalize_graph_v2(graph.graph())?.digest(),
            digest,
            "seed={SEED} case={case}"
        );
        assert_literal(&universe(&graph, 1, Some("us"), None)?, true)
            .map_err(|e| format!("seed={SEED} case={case}: {e}"))?;
    }
    Ok(())
}

#[test]
fn literal_baseline_contains_five_required_coordinates() -> Result<(), Box<dyn Error>> {
    let (_root, loaded) = support::loaded_example()?;
    let graph = finalize(&loaded, loaded.graph_input().clone())?;
    assert_literal(&universe(&graph, 1, Some("us"), None)?, false)
}

#[test]
fn floor_deletion_fails_and_checked_advisory_promotion_expands_work() -> Result<(), Box<dyn Error>>
{
    let (_root, loaded) = support::loaded_example_with(|root| {
        let p = root.join("eqm/contracts/auth.signup.identifier.toml");
        fs::write(
            &p,
            fs::read_to_string(&p)?.replace("level = \"required\"", "level = \"recommended\""),
        )?;
        Ok(())
    })?;
    let mut input = loaded.graph_input().clone();
    let baseline = input.policies.first().ok_or("policy")?.clone();
    input.policies.push(variant(
        &baseline,
        RequirementLevel::Recommended,
        TrustLevel::TrustedCi,
        86_400_000,
        1,
    )?);
    let graph = finalize(&loaded, input)?;
    let required = universe(&graph, 1, Some("us"), None)?;
    let recommended = universe(&graph, 2, Some("us"), None)?;
    assert_eq!(required.coordinates().len(), 3);
    assert_eq!(required.unresolved().len(), 2);
    assert!(
        required
            .unresolved()
            .iter()
            .all(|c| c.reason == UnresolvedReason::UnmatchedPolicy)
    );
    assert_eq!(recommended.coordinates().len(), 5);
    assert!(recommended.unresolved().is_empty());
    assert_eq!(
        enforce_required_universe_strengthening(&recommended, &required)?,
        MonotonicChange::Strengthened
    );
    assert_eq!(
        enforce_required_universe_strengthening(&required, &recommended),
        Err(MonotonicityError::Weakening)
    );
    assert!(
        !required.is_subset_of(&recommended),
        "revision transition requires the checked API"
    );
    Ok(())
}

#[test]
fn real_derived_facet_constraints_strengthen_only_in_the_correct_direction()
-> Result<(), Box<dyn Error>> {
    let (_root, loaded) = support::loaded_example()?;
    for (trust, age, count) in [
        (TrustLevel::SignedCi, 86_400_000, 1),
        (TrustLevel::TrustedCi, 60_000, 1),
        (TrustLevel::TrustedCi, 86_400_000, 2),
    ] {
        let mut input = loaded.graph_input().clone();
        let p = input.policies.first().ok_or("policy")?.clone();
        input
            .policies
            .push(variant(&p, RequirementLevel::Required, trust, age, count)?);
        let graph = finalize(&loaded, input)?;
        let b = universe(&graph, 1, Some("us"), None)?;
        let c = universe(&graph, 2, Some("us"), None)?;
        assert_eq!(c.coordinates().len(), 5);
        assert!(
            c.obligations()
                .values()
                .all(|o| o.strength.minimum_trust == trust
                    && o.strength.maximum_age.get() == age
                    && o.strength.minimum_count.get() == count)
        );
        assert_eq!(
            enforce_required_universe_strengthening(&c, &b)?,
            MonotonicChange::Strengthened
        );
        assert_eq!(
            enforce_required_universe_strengthening(&b, &c),
            Err(MonotonicityError::Weakening)
        );
    }
    Ok(())
}

#[test]
fn profile_release_and_foreign_graph_contexts_are_incomparable() -> Result<(), Box<dyn Error>> {
    let (_root, loaded) = support::loaded_example()?;
    let graph = finalize(&loaded, loaded.graph_input().clone())?;
    let baseline = universe(&graph, 1, Some("us"), None)?;
    for candidate in [
        universe(&graph, 1, None, None)?,
        universe(&graph, 1, Some("eu"), None)?,
        universe(
            &graph,
            1,
            Some("us"),
            Some(Sha256Digest::hash_content(b"release")),
        )?,
    ] {
        assert_eq!(
            enforce_required_universe_strengthening(&candidate, &baseline),
            Err(MonotonicityError::IncomparableAuthority)
        );
        assert!(!baseline.is_subset_of(&candidate));
    }
    let mut input = loaded.graph_input().clone();
    input.surfaces.clear();
    input.journeys.clear();
    input.bindings.clear();
    input.participation.clear();
    input.waivers.clear();
    let empty_graph = finalize(&loaded, input)?;
    let empty = universe(&empty_graph, 1, Some("us"), None)?;
    assert!(empty.coordinates().is_empty());
    assert!(empty.unresolved().is_empty());
    assert_eq!(
        enforce_required_universe_strengthening(&empty, &baseline),
        Err(MonotonicityError::IncomparableAuthority)
    );
    assert!(!empty.is_subset_of(&baseline));
    Ok(())
}

#[test]
fn unmatched_advisory_warns_but_unmatched_required_rejects() -> Result<(), Box<dyn Error>> {
    for (level, rejects) in [
        ("recommended", false),
        ("optional", false),
        ("required", true),
    ] {
        let (_root, loaded) = support::loaded_example_with(|root| {
            let p = root.join("eqm/contracts/auth.signup.identifier.toml");
            fs::write(
                &p,
                fs::read_to_string(&p)?
                    .replace("level = \"required\"", &format!("level = \"{level}\"")),
            )?;
            let p = root.join("eqm/policies/consumer.critical_flow.toml");
            fs::write(&p,fs::read_to_string(&p)?.replace("risk_classes = [\"high\", \"critical\"]","requirements = [\"account.create.signup.otp#six_decimal_digits\", \"account.create.signup.otp#valid_code_advances\"]"))?;
            Ok(())
        })?;
        let graph = finalize(&loaded, loaded.graph_input().clone())?;
        let result = universe(&graph, 1, Some("us"), None);
        if rejects {
            assert!(matches!(
                result
                    .err()
                    .and_then(|e| e.downcast::<ObligationError>().ok())
                    .as_deref(),
                Some(ObligationError::UnmatchedRequiredRequirement)
            ));
        } else {
            let u = result?;
            assert_eq!(u.coordinates().len(), 3);
            assert_eq!(u.unresolved().len(), 2);
            assert!(
                u.unresolved()
                    .iter()
                    .all(|c| c.reason == UnresolvedReason::UnmatchedPolicy
                        && c.requirement.to_string()
                            == "account.create.signup.identifier#email_default")
            );
        }
    }
    Ok(())
}

#[test]
fn waived_and_unknown_are_never_success() {
    for status in [
        FacetStatus::Waived,
        FacetStatus::Unknown,
        FacetStatus::Missing,
        FacetStatus::Unstable,
    ] {
        assert_ne!(status, FacetStatus::Satisfied);
    }
}

#[test]
fn a_real_empty_derivation_cannot_create_release_authorization() -> Result<(), Box<dyn Error>> {
    use eqm_engine::{
        ReleaseContext, ReleaseGateInput, ReleaseGateStatus, ReleaseSubject, TargetConformance,
        evaluate_release_gate,
    };
    let (_root, loaded) = support::loaded_example()?;
    let mut input = loaded.graph_input().clone();
    input.surfaces.clear();
    input.journeys.clear();
    input.bindings.clear();
    input.participation.clear();
    input.waivers.clear();
    let graph = finalize(&loaded, input)?;
    let empty = universe(&graph, 1, Some("us"), None)?;
    assert!(empty.coordinates().is_empty());
    let digest = Sha256Digest::hash_content(b"explicit synthetic gate fixture");
    let subject = ReleaseSubject {
        target: "android".parse()?,
        app_version: eqm_domain::AppVersion::new("1.2.3")?,
        build_number: eqm_domain::BuildNumber::new("42")?,
        source_commit: "a".repeat(40).parse()?,
        artifact_digest: digest,
        channel: eqm_domain::ReleaseChannel::Production,
    };
    let context = ReleaseContext {
        contract_digest: digest,
        policy_digest: digest,
        profile_values_digest: digest,
        evidence_set_digest: digest,
        runtime_facts_digest: digest,
        trust_config_digest: digest,
        release_record_digest: digest,
        evaluated_at: support::evaluation_time()?,
    };
    let mut gate = ReleaseGateInput {
        expected_subject: subject.clone(),
        observed_subject: Some(subject),
        release_record_verified: true,
        expected_context: context,
        observed_context: Some(context),
        exposure: Vec::new(),
        conformance: Some(TargetConformance::Conformant),
        release_facets: empty
            .coordinates()
            .iter()
            .map(|_| FacetStatus::Satisfied)
            .collect(),
        effective_trust: Some(TrustLevel::SignedCi),
        waivers: BTreeSet::new(),
        empty_scope_authorized: false,
    };
    assert_eq!(evaluate_release_gate(&gate), ReleaseGateStatus::Unknown);
    // Same synthetic ingress, with actual visible work, proves that another
    // missing field did not cause the empty-scope negative.
    gate.release_facets.push(FacetStatus::Satisfied);
    assert_eq!(evaluate_release_gate(&gate), ReleaseGateStatus::Pass);
    Ok(())
}

#[test]
fn policy_metadata_replacement_is_rejected_before_revision_normalization()
-> Result<(), Box<dyn Error>> {
    let (_root, loaded) = support::loaded_example()?;
    let mut input = loaded.graph_input().clone();
    let p = input.policies.first().ok_or("policy")?;
    let replacement = Policy::new(
        p.id().clone(),
        Revision::new(2)?,
        p.title().clone(),
        vec!["owner://team/replacement".parse()?],
        p.profiles().iter().cloned().collect(),
        p.required_targets().iter().cloned().collect(),
        p.rules().to_vec(),
        p.waivers().clone(),
        p.description().cloned(),
        p.extensions().clone(),
    )?;
    input.policies.push(replacement);
    let graph = finalize(&loaded, input)?;
    let baseline = universe(&graph, 1, Some("us"), None)?;
    let candidate = universe(&graph, 2, Some("us"), None)?;
    assert_eq!(baseline.coordinates().len(), 5);
    assert_eq!(candidate.coordinates().len(), 5);
    assert_eq!(
        enforce_required_universe_strengthening(&candidate, &baseline),
        Err(MonotonicityError::IncomparableAuthority)
    );
    Ok(())
}
