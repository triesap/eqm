//! Independent S054 literal oracle and every pre-evidence identity axis.
use eqm_domain::*;
use eqm_manifest::{CanonicalizationError, canonicalize_obligation_v2};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

const LITERAL: &str = r#"{"authority_origin":"candidate_local","contract_digest":"sha256:b0f7753090d6379ae6fed3d83f216edcc7631454d16bf5552f38c60e1ecd9dd0","expected_selections":{"state":"missing"},"facet":"behavior","mode":"development","participation_digests":["sha256:0eba2f22d84f1829c627b72e2e57c4c671274c74f3c4ed3850cf5cd9fd33847c"],"policy_selection":{"policy":{"description":"Baseline policy","extensions":{},"id":"policy.default","owners":["owner://team/security"],"profiles":["audience.default"],"required_targets":["web"],"revision":1,"rules":[{"facets":["behavior"],"maximum_age":86400000,"minimum_count":1,"minimum_level":"recommended","minimum_trust":"trusted_ci","selector":{"units":["account.create"]}}],"title":"Default policy","waivers":{"allowed":false,"minimum_approvers":1,"required_controls":[]}},"profiles":[{"definition":{"defaults":{"locale":"en"},"description":"Default audience","dimensions":[{"id":"locale","values":["en","fr"]},{"id":"region","values":["eu","us"]}],"extensions":{},"id":"audience.default","owners":["owner://team/product"],"revision":1,"title":"Audience"},"values":[{"dimension":"locale","state":"known","value":"en"},{"dimension":"region","state":"unknown"}]}]},"requirement":"account.create.signup#visible","scope_bindings_digest":"sha256:9e30ff5852896aabafd17d6de6a8f0c7a420229aac9723136046b62344983516","semantic_graph_digest":"sha256:9be38c59230772d126f3417f0cf924d2c5a2e7be6f9db6bef3b0c8df4fff05ee","strength":{"maximum_age_ms":86400000,"minimum_count":1,"minimum_level":"recommended","minimum_trust":"trusted_ci"},"subject":{"configuration_digest":"sha256:52a4e99e8a7096ba27535fa7c74fa755534298263b77f8fa5e9a68bda4f3c490","kind":"target","target":"web"},"unit":"account.create"}"#;
const DIGEST: &str = "sha256:c8684bd1141e0871b8bd8047838866e99b52b752ae1fff6735ff04b8c1ec0bd9";

fn owner(value: &str) -> Result<OwnerRef, Box<dyn Error>> {
    Ok(value.parse()?)
}

fn profile(revision: u64, title: &str) -> Result<Profile, Box<dyn Error>> {
    Ok(Profile::new(
        ProfileId::new("audience.default")?,
        Revision::new(revision)?,
        Title::new(title)?,
        vec![owner("owner://team/product")?],
        vec![
            ProfileDimension::new(
                eqm_domain::DimensionId::new("locale")?,
                vec![SymbolicValueId::new("en")?, SymbolicValueId::new("fr")?],
                None,
            )?,
            ProfileDimension::new(
                eqm_domain::DimensionId::new("region")?,
                vec![SymbolicValueId::new("eu")?, SymbolicValueId::new("us")?],
                None,
            )?,
        ],
        vec![(
            eqm_domain::DimensionId::new("locale")?,
            SymbolicValueId::new("en")?,
        )],
        Some(Description::new("Default audience")?),
        Extensions::default(),
    )?)
}

fn policy(profiles: Vec<ProfileId>) -> Result<Policy, Box<dyn Error>> {
    let rule = PolicyRule::new(
        PolicySelector::new(
            Some(vec![UnitId::new("account.create")?]),
            None,
            None,
            None,
            None,
        )?,
        RequirementLevel::Recommended,
        vec![Facet::Behavior],
        TrustLevel::TrustedCi,
        DurationMillis::new(86_400_000)?,
        Some(PositiveCount::ONE),
    )?;
    Ok(Policy::new(
        PolicyId::new("policy.default")?,
        Revision::new(1)?,
        Title::new("Default policy")?,
        vec![owner("owner://team/security")?],
        profiles,
        vec![TargetId::new("web")?],
        vec![rule],
        WaiverPolicy::deny(),
        Some(Description::new("Baseline policy")?),
        Extensions::default(),
    )?)
}

fn target() -> Result<Target, Box<dyn Error>> {
    Ok(Target::new(
        TargetId::new("web")?,
        RepoPath::new("apps/web")?,
        PlatformId::new("web")?,
        FrameworkId::new("sveltekit")?,
        vec![owner("owner://team/web")?],
        Extensions::default(),
    )?)
}
fn selection<'a>(
    policy: &'a Policy,
    profile: &'a Profile,
    region: SelectedDimensionStateV2,
) -> Result<PolicySelectionInputV2<'a>, Box<dyn Error>> {
    Ok(PolicySelectionInputV2::new(
        policy,
        vec![SelectedProfileInputV2::new(
            profile,
            vec![
                (
                    DimensionId::new("locale")?,
                    SelectedDimensionStateV2::Known(SymbolicValueId::new("en")?),
                ),
                (DimensionId::new("region")?, region),
            ],
        )?],
    )?)
}
fn input<'a>(
    selection: &'a PolicySelectionInputV2<'a>,
    target: &'a Target,
    unit: &'a UnitId,
    requirement: &'a FullRequirementId,
) -> Result<ObligationIdentityInputV2<'a>, Box<dyn Error>> {
    Ok(ObligationIdentityInputV2 {
        unit,
        requirement,
        facet: Facet::Behavior,
        selection,
        semantic_graph_digest: SemanticGraphV2Digest::hash_content(b"graph"),
        contract_digest: ContractDigest::hash_content(b"contract"),
        scope_bindings_digest: RequirementScenarioBindingsDigest::hash_content(b"scopes"),
        participation_digests: vec![ParticipationDigest::hash_content(b"participation")],
        subject: ObligationScopeInputV2::Target(target),
        expected_selections: ExpectedSelectionIdentityV2::Missing,
        strength: ObligationStrengthInputV2 {
            minimum_level: RequirementLevel::Recommended,
            minimum_trust: TrustLevel::TrustedCi,
            maximum_age: DurationMillis::new(86_400_000)?,
            minimum_count: PositiveCount::ONE,
        },
        release_context: None,
        mode: ObligationModeV2::Development,
        authority_origin: ObligationOriginV2::CandidateLocal,
    })
}
#[test]
fn literal_bytes_and_independent_domain_digest_match() -> Result<(), Box<dyn Error>> {
    let profile = profile(1, "Audience")?;
    let policy = policy(vec![profile.id().clone()])?;
    let selected = selection(&policy, &profile, SelectedDimensionStateV2::Unknown)?;
    let target = target()?;
    let unit = UnitId::new("account.create")?;
    let requirement = FullRequirementId::new("account.create.signup#visible")?;
    let actual = canonicalize_obligation_v2(&input(&selected, &target, &unit, &requirement)?)?;
    assert_eq!(actual.bytes(), LITERAL.as_bytes());
    assert_eq!(actual.digest().to_string(), DIGEST);
    assert_eq!(actual.reference(), format!("eqm:v2:obligation:{DIGEST}"));
    assert_ne!(
        actual.digest().as_digest(),
        Sha256Digest::hash_content(LITERAL.as_bytes())
    );
    Ok(())
}

#[test]
fn every_coordinate_and_context_axis_changes_identity() -> Result<(), Box<dyn Error>> {
    let profile = profile(1, "Audience")?;
    let policy = policy(vec![profile.id().clone()])?;
    let selected = selection(&policy, &profile, SelectedDimensionStateV2::Unknown)?;
    let target = target()?;
    let unit = UnitId::new("account.create")?;
    let requirement = FullRequirementId::new("account.create.signup#visible")?;
    let baseline = input(&selected, &target, &unit, &requirement)?;
    let original = canonicalize_obligation_v2(&baseline)?.digest();
    macro_rules! distinct {
        ($label:literal, $changed:ident, $body:block) => {{
            let mut $changed = baseline.clone();
            $body
            assert_ne!(original, canonicalize_obligation_v2(&$changed)?.digest(), $label);
        }};
    }
    let other_unit = UnitId::new("account.other")?;
    let other_requirement = FullRequirementId::new("account.create.signup#another")?;
    distinct!("unit", changed, {
        changed.unit = &other_unit;
    });
    distinct!("requirement", changed, {
        changed.requirement = &other_requirement;
    });
    distinct!("facet", changed, {
        changed.facet = Facet::Structure;
    });
    distinct!("semantic graph", changed, {
        changed.semantic_graph_digest = SemanticGraphV2Digest::hash_content(b"other graph");
    });
    distinct!("contract", changed, {
        changed.contract_digest = ContractDigest::hash_content(b"other contract");
    });
    distinct!("scope bindings", changed, {
        changed.scope_bindings_digest =
            RequirementScenarioBindingsDigest::hash_content(b"other bindings");
    });
    distinct!("participation definition", changed, {
        changed.participation_digests = vec![ParticipationDigest::hash_content(b"other rule")];
    });
    distinct!("mode", changed, {
        changed.mode = ObligationModeV2::PullRequest;
    });
    distinct!("authority origin", changed, {
        changed.authority_origin = ObligationOriginV2::TrustedInvocation;
    });
    distinct!("release presence", changed, {
        changed.release_context = Some(Sha256Digest::hash_content(b"release.one"));
    });
    let mut first_release = baseline.clone();
    first_release.release_context = Some(Sha256Digest::hash_content(b"release.one"));
    let mut second_release = first_release.clone();
    second_release.release_context = Some(Sha256Digest::hash_content(b"release.two"));
    assert_ne!(
        canonicalize_obligation_v2(&first_release)?.digest(),
        canonicalize_obligation_v2(&second_release)?.digest()
    );
    distinct!("level", changed, {
        changed.strength.minimum_level = RequirementLevel::Required;
    });
    distinct!("trust", changed, {
        changed.strength.minimum_trust = TrustLevel::UntrustedLocal;
    });
    distinct!("age", changed, {
        changed.strength.maximum_age = DurationMillis::new(1)?;
    });
    distinct!("logical count", changed, {
        changed.strength.minimum_count = PositiveCount::new(2)?;
    });
    distinct!("missing versus pure empty present", changed, {
        changed.expected_selections = ExpectedSelectionIdentityV2::Present(vec![]);
    });
    distinct!("expected manifest", changed, {
        changed.expected_selections = ExpectedSelectionIdentityV2::Present(vec![
            ExpectedSelectionManifestDigest::hash_content(b"case.a"),
        ]);
    });
    let other_target = Target::new(
        target.id().clone(),
        RepoPath::new("apps/changed")?,
        target.platform().clone(),
        target.framework().clone(),
        target.owners().iter().cloned().collect(),
        Extensions::default(),
    )?;
    distinct!("actual target configuration", changed, {
        changed.subject = ObligationScopeInputV2::Target(&other_target);
    });
    let known = selection(
        &policy,
        &profile,
        SelectedDimensionStateV2::Known(SymbolicValueId::new("us")?),
    )?;
    distinct!("known versus unknown", changed, {
        changed.selection = &known;
    });
    Ok(())
}

#[test]
fn exact_policy_and_profile_definitions_and_revisions_are_not_just_labels()
-> Result<(), Box<dyn Error>> {
    let base_profile = profile(1, "Audience")?;
    let base_policy = policy(vec![base_profile.id().clone()])?;
    let selected = selection(
        &base_policy,
        &base_profile,
        SelectedDimensionStateV2::Unknown,
    )?;
    let target = target()?;
    let unit = UnitId::new("account.create")?;
    let requirement = FullRequirementId::new("account.create.signup#visible")?;
    let original =
        canonicalize_obligation_v2(&input(&selected, &target, &unit, &requirement)?)?.digest();
    for changed_profile in [
        profile(2, "Audience")?,
        profile(1, "Normative definition changed")?,
    ] {
        let selected = selection(
            &base_policy,
            &changed_profile,
            SelectedDimensionStateV2::Unknown,
        )?;
        assert_ne!(
            original,
            canonicalize_obligation_v2(&input(&selected, &target, &unit, &requirement)?)?.digest()
        );
    }
    for (revision, title) in [(2, "Default policy"), (1, "Normative policy changed")] {
        let changed_policy = Policy::new(
            base_policy.id().clone(),
            Revision::new(revision)?,
            Title::new(title)?,
            base_policy.owners().iter().cloned().collect(),
            base_policy.profiles().iter().cloned().collect(),
            base_policy.required_targets().iter().cloned().collect(),
            base_policy.rules().to_vec(),
            base_policy.waivers().clone(),
            base_policy.description().cloned(),
            base_policy.extensions().clone(),
        )?;
        let selected = selection(
            &changed_policy,
            &base_profile,
            SelectedDimensionStateV2::Unknown,
        )?;
        assert_ne!(
            original,
            canonicalize_obligation_v2(&input(&selected, &target, &unit, &requirement)?)?.digest()
        );
    }
    Ok(())
}

#[test]
fn manifest_and_participation_sets_are_order_invariant_and_reject_duplicates()
-> Result<(), Box<dyn Error>> {
    let profile = profile(1, "Audience")?;
    let policy = policy(vec![profile.id().clone()])?;
    let selected = selection(&policy, &profile, SelectedDimensionStateV2::Unknown)?;
    let target = target()?;
    let unit = UnitId::new("account.create")?;
    let requirement = FullRequirementId::new("account.create.signup#visible")?;
    let mut first = input(&selected, &target, &unit, &requirement)?;
    let a = ExpectedSelectionManifestDigest::hash_content(b"a");
    let b = ExpectedSelectionManifestDigest::hash_content(b"b");
    let p = ParticipationDigest::hash_content(b"p");
    let q = ParticipationDigest::hash_content(b"q");
    first.expected_selections = ExpectedSelectionIdentityV2::Present(vec![a, b]);
    first.participation_digests = vec![p, q];
    let mut reversed = first.clone();
    reversed.expected_selections = ExpectedSelectionIdentityV2::Present(vec![b, a]);
    reversed.participation_digests.reverse();
    assert_eq!(
        canonicalize_obligation_v2(&first)?,
        canonicalize_obligation_v2(&reversed)?
    );
    let mut duplicate = first.clone();
    duplicate.expected_selections = ExpectedSelectionIdentityV2::Present(vec![a, a]);
    assert_eq!(
        canonicalize_obligation_v2(&duplicate),
        Err(CanonicalizationError::DuplicateMember)
    );
    duplicate = first.clone();
    duplicate.participation_digests.push(p);
    assert_eq!(
        canonicalize_obligation_v2(&duplicate),
        Err(CanonicalizationError::DuplicateMember)
    );
    for (number, expected) in [
        (
            9_007_199_254_740_991,
            serde_json::json!(9_007_199_254_740_991_u64),
        ),
        (9_007_199_254_740_992, serde_json::json!("9007199254740992")),
        (u64::MAX, serde_json::json!("18446744073709551615")),
    ] {
        let mut changed = first.clone();
        changed.strength.minimum_count = PositiveCount::new(number)?;
        let wire: serde_json::Value =
            serde_json::from_slice(canonicalize_obligation_v2(&changed)?.bytes())?;
        assert_eq!(wire["strength"]["minimum_count"], expected);
    }
    for (number, expected) in [
        (
            9_007_199_254_740_991,
            serde_json::json!(9_007_199_254_740_991_u64),
        ),
        (9_007_199_254_740_992, serde_json::json!("9007199254740992")),
        (i64::MAX as u64, serde_json::json!(i64::MAX.to_string())),
    ] {
        let mut changed = first.clone();
        changed.strength.maximum_age = DurationMillis::new(number)?;
        let wire: serde_json::Value =
            serde_json::from_slice(canonicalize_obligation_v2(&changed)?.bytes())?;
        assert_eq!(wire["strength"]["maximum_age_ms"], expected);
    }
    Ok(())
}

fn provider_definition(seed: &[u8]) -> Result<Provider, Box<dyn Error>> {
    Ok(Provider::new(
        ProviderId::new("provider.shared")?,
        Sha256Digest::hash_content(seed),
        vec![owner("owner://team/provider")?],
        Extensions::default(),
    )?)
}
fn provider_scope(provider: &Provider) -> Result<ProviderScopeInputV2<'_>, Box<dyn Error>> {
    let web = TargetId::new("web")?;
    let bridge = TargetId::new("bridge")?;
    let config = TargetConfigurationDigest::hash_content(b"config.web");
    Ok(ProviderScopeInputV2 {
        provider,
        dependers: BTreeMap::from([(web.clone(), config)]),
        assignments: BTreeMap::from([(bridge.clone(), config)]),
        dependencies: BTreeSet::from([(web.clone(), bridge.clone())]),
        dependency_configurations: BTreeMap::from([(web.clone(), config), (bridge, config)]),
        participation: BTreeMap::from([(web, ScopeParticipationV2::False)]),
    })
}
fn scenario(
    revision: u64,
    occurrence: &str,
    role: &str,
    provider_tag: bool,
    reverse: bool,
) -> Result<Scenario, Box<dyn Error>> {
    let first = OccurrenceId::new("first")?;
    let second = OccurrenceId::new(occurrence)?;
    let subject = if provider_tag {
        ScenarioSubject::Provider(ProviderId::new("provider.web")?)
    } else {
        ScenarioSubject::Target(TargetId::new("provider.web")?)
    };
    let edge = if reverse {
        ScenarioDependency::new(second.clone(), first.clone())?
    } else {
        ScenarioDependency::new(first.clone(), second.clone())?
    };
    Ok(Scenario::new(
        ScenarioId::new("scenario.flow")?,
        Revision::new(revision)?,
        vec![
            ScenarioParticipant::new(
                first,
                RoleId::new("edge")?,
                ScenarioSubject::Target(TargetId::new("provider.web")?),
            ),
            ScenarioParticipant::new(second, RoleId::new(role)?, subject),
        ],
        vec![edge],
        Extensions::default(),
    )?)
}
#[test]
fn provider_definition_dependency_assignment_configuration_and_false_participation_are_bound()
-> Result<(), Box<dyn Error>> {
    let profile = profile(1, "Audience")?;
    let policy = policy(vec![profile.id().clone()])?;
    let selected = selection(&policy, &profile, SelectedDimensionStateV2::Unknown)?;
    let target = target()?;
    let unit = UnitId::new("account.create")?;
    let requirement = FullRequirementId::new("account.create.signup#visible")?;
    let provider = provider_definition(b"config.one")?;
    let scope = provider_scope(&provider)?;
    let mut baseline = input(&selected, &target, &unit, &requirement)?;
    baseline.subject = ObligationScopeInputV2::Provider(scope.clone());
    let original = canonicalize_obligation_v2(&baseline)?.digest();
    let wire: serde_json::Value =
        serde_json::from_slice(canonicalize_obligation_v2(&baseline)?.bytes())?;
    assert_eq!(wire["subject"]["participation"][0]["state"], "false");
    for axis in 0..6 {
        let mut changed = baseline.clone();
        let mut scope = scope.clone();
        match axis {
            0 => {
                scope.dependers.insert(
                    TargetId::new("extra")?,
                    TargetConfigurationDigest::hash_content(b"extra"),
                );
            }
            1 => {
                scope.assignments.clear();
            }
            2 => {
                scope.dependencies.clear();
            }
            3 => {
                scope
                    .participation
                    .insert(TargetId::new("web")?, ScopeParticipationV2::Unknown);
            }
            4 => {
                scope.dependers.insert(
                    TargetId::new("web")?,
                    TargetConfigurationDigest::hash_content(b"other config"),
                );
            }
            _ => {
                scope.dependency_configurations.insert(
                    TargetId::new("bridge")?,
                    TargetConfigurationDigest::hash_content(b"other bridge"),
                );
            }
        }
        changed.subject = ObligationScopeInputV2::Provider(scope);
        assert_ne!(
            original,
            canonicalize_obligation_v2(&changed)?.digest(),
            "provider axis{axis}"
        );
    }
    let changed_provider = provider_definition(b"config.two")?;
    let mut changed = baseline.clone();
    changed.subject = ObligationScopeInputV2::Provider(provider_scope(&changed_provider)?);
    assert_ne!(original, canonicalize_obligation_v2(&changed)?.digest());
    let as_target = input(&selected, &target, &unit, &requirement)?;
    assert_ne!(original, canonicalize_obligation_v2(&as_target)?.digest());
    Ok(())
}
#[test]
fn repeated_scenario_actors_roles_tags_edges_revision_and_derived_closure_are_bound()
-> Result<(), Box<dyn Error>> {
    let profile = profile(1, "Audience")?;
    let policy = policy(vec![profile.id().clone()])?;
    let selected = selection(&policy, &profile, SelectedDimensionStateV2::Unknown)?;
    let target = target()?;
    let unit = UnitId::new("account.create")?;
    let requirement = FullRequirementId::new("account.create.signup#visible")?;
    let scene = scenario(1, "second", "observer", false, false)?;
    let tagged = scenario(1, "second", "observer", true, false)?;
    let occurrence = OccurrenceId::new("second")?;
    let original_actor = scene
        .participants()
        .iter()
        .find(|actor| actor.occurrence() == &occurrence)
        .ok_or("baseline actor")?;
    let tagged_actor = tagged
        .participants()
        .iter()
        .find(|actor| actor.occurrence() == &occurrence)
        .ok_or("tagged actor")?;
    assert_eq!(original_actor.occurrence(), tagged_actor.occurrence());
    assert_eq!(original_actor.role(), tagged_actor.role());
    assert_eq!(
        original_actor.subject(),
        &ScenarioSubject::Target(TargetId::new("provider.web")?)
    );
    assert_eq!(
        tagged_actor.subject(),
        &ScenarioSubject::Provider(ProviderId::new("provider.web")?)
    );
    assert_eq!(scene.dependencies(), tagged.dependencies());
    let provider = provider_definition(b"config")?;
    let web = TargetId::new("provider.web")?;
    let configs = BTreeMap::from([(
        web.clone(),
        TargetConfigurationDigest::hash_content(b"config.web"),
    )]);
    let states = BTreeMap::from([(web, ScopeParticipationV2::True)]);
    let mut baseline = input(&selected, &target, &unit, &requirement)?;
    baseline.subject = ObligationScopeInputV2::Scenario {
        scenario: &scene,
        selected_targets: configs.clone(),
        providers: vec![provider_scope(&provider)?],
        participation: states.clone(),
    };
    let original = canonicalize_obligation_v2(&baseline)?.digest();
    let wire: serde_json::Value =
        serde_json::from_slice(canonicalize_obligation_v2(&baseline)?.bytes())?;
    assert_eq!(
        wire["subject"]["scenario"]["participants"]
            .as_array()
            .ok_or("participants")?
            .len(),
        2
    );
    assert_eq!(
        wire["subject"]["scenario"]["dependencies"]
            .as_array()
            .ok_or("dependencies")?
            .len(),
        1
    );
    for changed_scene in [
        scenario(2, "second", "observer", false, false)?,
        scenario(1, "other", "observer", false, false)?,
        scenario(1, "second", "edge", false, false)?,
        scenario(1, "second", "observer", true, false)?,
        scenario(1, "second", "observer", false, true)?,
    ] {
        let mut changed = baseline.clone();
        changed.subject = ObligationScopeInputV2::Scenario {
            scenario: &changed_scene,
            selected_targets: configs.clone(),
            providers: vec![provider_scope(&provider)?],
            participation: states.clone(),
        };
        assert_ne!(original, canonicalize_obligation_v2(&changed)?.digest());
    }
    for axis in 0..3 {
        let mut targets = configs.clone();
        let mut providers = vec![provider_scope(&provider)?];
        let mut participation = states.clone();
        match axis {
            0 => {
                targets.clear();
            }
            1 => {
                providers[0].dependencies.clear();
            }
            _ => {
                participation.insert(
                    TargetId::new("provider.web")?,
                    ScopeParticipationV2::Unknown,
                );
            }
        }
        let mut changed = baseline.clone();
        changed.subject = ObligationScopeInputV2::Scenario {
            scenario: &scene,
            selected_targets: targets,
            providers,
            participation,
        };
        assert_ne!(original, canonicalize_obligation_v2(&changed)?.digest());
    }
    Ok(())
}
