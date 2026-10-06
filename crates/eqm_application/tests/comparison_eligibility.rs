//! S039-R2: graph-bound checked comparison inputs and exact occurrence coverage.
//!
//! The checked comparison context must be the independently derived graph
//! context; expected coverage is the selected scenario's exact occurrence set;
//! and participants are built only after bytes-first subject matching against
//! independently resolved authority succeeds. Two distinct valid builds share
//! one comparison context, and repeated same-target actors stay distinct.

use eqm_application::{
    CheckedComparisonOutcome, ComparisonContextError, evaluate_checked_comparison, resolve_cohort,
    shared_comparison_context,
};
use eqm_domain::{
    Cohort, CohortAssignment, ComparisonContextInputV2, DimensionId, Extensions, Facet,
    FrameworkId, OccurrenceId, OwnerRef, PlatformId, Policy, PolicyId, PolicyRule,
    PolicySelectionInputV2, PolicySelector, PositiveCount, Profile, ProfileDimension, ProfileId,
    Provider, ProviderId, RepoPath, Revision, RoleId, Scenario, ScenarioDependency, ScenarioId,
    ScenarioParticipant, ScenarioSubject, SelectedDimensionStateV2, SelectedProfileInputV2,
    SelectorText, Sha256Digest, SourceCommit, SymbolicValueId, Target, TargetId, Title, TrustLevel,
    UnitId, WaiverPolicy, WorkspaceGraph, WorkspaceGraphInput,
};
use eqm_engine::{
    ComparisonEligibility, FinalizedWorkspaceGraph, FragmentDigestMap, expand_fragments,
};
use eqm_protocol::schema_graph_v2::CheckedComparisonContextV2;
use eqm_protocol::v2::{CheckedSubjectManifestV2, SubjectMatchError};
use serde_json::json;
use std::collections::BTreeMap;
use std::error::Error;

fn finalize(graph: WorkspaceGraph) -> Result<FinalizedWorkspaceGraph, Box<dyn Error>> {
    Ok(expand_fragments(
        graph,
        &FragmentDigestMap::new(),
        &BTreeMap::new(),
    )?)
}

fn owner(value: &str) -> Result<OwnerRef, Box<dyn Error>> {
    Ok(value.parse()?)
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

fn provider() -> Result<Provider, Box<dyn Error>> {
    Ok(Provider::new(
        ProviderId::new("provider.mailgun")?,
        Sha256Digest::hash_content(b"external configuration"),
        vec![owner("owner://team/providers")?],
        Extensions::default(),
    )?)
}

fn scenario(revision: u64) -> Result<Scenario, Box<dyn Error>> {
    Ok(Scenario::new(
        ScenarioId::new("signup.flow")?,
        Revision::new(revision)?,
        vec![
            ScenarioParticipant::new(
                OccurrenceId::new("phone")?,
                RoleId::new("edge")?,
                ScenarioSubject::Target(TargetId::new("web")?),
            ),
            ScenarioParticipant::new(
                OccurrenceId::new("mailer")?,
                RoleId::new("provider")?,
                ScenarioSubject::Provider(ProviderId::new("provider.mailgun")?),
            ),
        ],
        vec![ScenarioDependency::new(
            OccurrenceId::new("mailer")?,
            OccurrenceId::new("phone")?,
        )?],
        Extensions::default(),
    )?)
}

/// A scenario with two occurrences that reference the *same* target, plus a
/// provider occurrence. Repeated same-target actors must stay distinct.
fn repeated_scenario(revision: u64) -> Result<Scenario, Box<dyn Error>> {
    Ok(Scenario::new(
        ScenarioId::new("signup.flow")?,
        Revision::new(revision)?,
        vec![
            ScenarioParticipant::new(
                OccurrenceId::new("phone_a")?,
                RoleId::new("edge.a")?,
                ScenarioSubject::Target(TargetId::new("web")?),
            ),
            ScenarioParticipant::new(
                OccurrenceId::new("phone_b")?,
                RoleId::new("edge.b")?,
                ScenarioSubject::Target(TargetId::new("web")?),
            ),
            ScenarioParticipant::new(
                OccurrenceId::new("mailer")?,
                RoleId::new("provider")?,
                ScenarioSubject::Provider(ProviderId::new("provider.mailgun")?),
            ),
        ],
        Vec::new(),
        Extensions::default(),
    )?)
}

fn profile(revision: u64) -> Result<Profile, Box<dyn Error>> {
    Ok(Profile::new(
        ProfileId::new("audience.default")?,
        Revision::new(revision)?,
        Title::new("Audience")?,
        vec![owner("owner://team/product")?],
        vec![
            ProfileDimension::new(
                DimensionId::new("locale")?,
                vec![SymbolicValueId::new("en")?, SymbolicValueId::new("fr")?],
                None,
            )?,
            ProfileDimension::new(
                DimensionId::new("region")?,
                vec![SymbolicValueId::new("eu")?, SymbolicValueId::new("us")?],
                None,
            )?,
        ],
        Vec::new(),
        None,
        Extensions::default(),
    )?)
}

fn policy(revision: u64) -> Result<Policy, Box<dyn Error>> {
    let rule = PolicyRule::new(
        PolicySelector::new(
            Some(vec![UnitId::new("account.create")?]),
            None,
            None,
            None,
            None,
        )?,
        eqm_domain::RequirementLevel::Recommended,
        vec![Facet::Behavior],
        TrustLevel::TrustedCi,
        eqm_domain::DurationMillis::new(86_400_000)?,
        Some(PositiveCount::ONE),
    )?;
    Ok(Policy::new(
        PolicyId::new("policy.default")?,
        Revision::new(revision)?,
        Title::new("Default policy")?,
        vec![owner("owner://team/security")?],
        vec![ProfileId::new("audience.default")?],
        vec![TargetId::new("web")?],
        vec![rule],
        WaiverPolicy::deny(),
        None,
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

struct Fixture {
    finalized: FinalizedWorkspaceGraph,
    scenario: Scenario,
    policy: Policy,
    profile: Profile,
}

fn fixture_with(scenario: Scenario) -> Result<Fixture, Box<dyn Error>> {
    let policy = policy(1)?;
    let profile = profile(1)?;
    let finalized = finalize(WorkspaceGraph::new(WorkspaceGraphInput {
        targets: vec![target()?],
        providers: vec![provider()?],
        scenarios: vec![scenario.clone()],
        policies: vec![policy.clone()],
        profiles: vec![profile.clone()],
        ..WorkspaceGraphInput::default()
    })?)?;
    Ok(Fixture {
        finalized,
        scenario,
        policy,
        profile,
    })
}

fn fixture() -> Result<Fixture, Box<dyn Error>> {
    fixture_with(scenario(1)?)
}

/// Rebuilds the fixture graph with the supplied graph-owned cohorts so that
/// `resolve_cohort` can consume the exact retained authority.
fn graph_with_cohorts(
    fixture: &Fixture,
    cohorts: Vec<Cohort>,
) -> Result<FinalizedWorkspaceGraph, Box<dyn Error>> {
    finalize(WorkspaceGraph::new(WorkspaceGraphInput {
        targets: vec![target()?],
        providers: vec![provider()?],
        scenarios: vec![fixture.scenario.clone()],
        policies: vec![fixture.policy.clone()],
        profiles: vec![fixture.profile.clone()],
        cohorts,
        ..WorkspaceGraphInput::default()
    })?)
}

/// Rebuilds the fixture graph with an explicit scenario set and cohorts.
fn graph_with_scenarios(
    fixture: &Fixture,
    scenarios: Vec<Scenario>,
    cohorts: Vec<Cohort>,
) -> Result<FinalizedWorkspaceGraph, Box<dyn Error>> {
    finalize(WorkspaceGraph::new(WorkspaceGraphInput {
        targets: vec![target()?],
        providers: vec![provider()?],
        scenarios,
        policies: vec![fixture.policy.clone()],
        profiles: vec![fixture.profile.clone()],
        cohorts,
        ..WorkspaceGraphInput::default()
    })?)
}

/// Rebuilds the fixture graph with a substituted target configuration.
fn graph_with_target(
    fixture: &Fixture,
    target: Target,
    cohorts: Vec<Cohort>,
) -> Result<FinalizedWorkspaceGraph, Box<dyn Error>> {
    finalize(WorkspaceGraph::new(WorkspaceGraphInput {
        targets: vec![target],
        providers: vec![provider()?],
        scenarios: vec![fixture.scenario.clone()],
        policies: vec![fixture.policy.clone()],
        profiles: vec![fixture.profile.clone()],
        cohorts,
        ..WorkspaceGraphInput::default()
    })?)
}

/// Rebuilds the fixture graph with a substituted provider configuration.
fn graph_with_provider(
    fixture: &Fixture,
    provider: Provider,
    cohorts: Vec<Cohort>,
) -> Result<FinalizedWorkspaceGraph, Box<dyn Error>> {
    finalize(WorkspaceGraph::new(WorkspaceGraphInput {
        targets: vec![target()?],
        providers: vec![provider],
        scenarios: vec![fixture.scenario.clone()],
        policies: vec![fixture.policy.clone()],
        profiles: vec![fixture.profile.clone()],
        cohorts,
        ..WorkspaceGraphInput::default()
    })?)
}

/// A scenario with a different ID but identical occurrence names, roles and
/// subjects to [`scenario`].
fn other_scenario(revision: u64) -> Result<Scenario, Box<dyn Error>> {
    Ok(Scenario::new(
        ScenarioId::new("other.flow")?,
        Revision::new(revision)?,
        vec![
            ScenarioParticipant::new(
                OccurrenceId::new("phone")?,
                RoleId::new("edge")?,
                ScenarioSubject::Target(TargetId::new("web")?),
            ),
            ScenarioParticipant::new(
                OccurrenceId::new("mailer")?,
                RoleId::new("provider")?,
                ScenarioSubject::Provider(ProviderId::new("provider.mailgun")?),
            ),
        ],
        Vec::new(),
        Extensions::default(),
    )?)
}

/// One exact authored combination plus the values a matching candidate needs.
struct Combination {
    cohort: Cohort,
    target_configuration: String,
    provider_configuration: String,
}

fn assignment(occurrence: &str, seed: u8) -> Result<CohortAssignment, Box<dyn Error>> {
    let (target_configuration, provider_configuration) = if occurrence == "mailer" {
        (
            None,
            Some(
                eqm_manifest::canonicalize_provider_configuration_v2(&provider()?)?
                    .digest()
                    .as_digest(),
            ),
        )
    } else {
        (
            Some(
                eqm_manifest::canonicalize_target_configuration_v2(&target()?)?
                    .digest()
                    .as_digest(),
            ),
            None,
        )
    };
    Ok(CohortAssignment::new(
        OccurrenceId::new(occurrence)?,
        "https://example.com/team/project".parse()?,
        "a".repeat(40).parse::<SourceCommit>()?,
        Some(Sha256Digest::hash_content(&[seed, 1])),
        target_configuration,
        provider_configuration,
        Some(SelectorText::new(format!("build.{seed}.{occurrence}"))?),
        Some(Sha256Digest::hash_content(&[seed, 2])),
    )?)
}

fn combination(seed: u8) -> Result<Combination, Box<dyn Error>> {
    let cohort = Cohort::new(
        SelectorText::new(format!("cohort.signup.{seed}"))?,
        Revision::new(1)?,
        ScenarioId::new("signup.flow")?,
        vec![assignment("phone", seed)?, assignment("mailer", seed)?],
        Extensions::default(),
    )?;
    Ok(Combination {
        cohort,
        target_configuration: eqm_manifest::canonicalize_target_configuration_v2(&target()?)?
            .digest()
            .to_string(),
        provider_configuration: eqm_manifest::canonicalize_provider_configuration_v2(&provider()?)?
            .digest()
            .to_string(),
    })
}

fn repeated_combination(seed: u8) -> Result<Combination, Box<dyn Error>> {
    let cohort = Cohort::new(
        SelectorText::new(format!("cohort.signup.{seed}"))?,
        Revision::new(1)?,
        ScenarioId::new("signup.flow")?,
        vec![
            assignment("phone_a", seed)?,
            assignment("phone_b", seed)?,
            assignment("mailer", seed)?,
        ],
        Extensions::default(),
    )?;
    Ok(Combination {
        cohort,
        target_configuration: eqm_manifest::canonicalize_target_configuration_v2(&target()?)?
            .digest()
            .to_string(),
        provider_configuration: eqm_manifest::canonicalize_provider_configuration_v2(&provider()?)?
            .digest()
            .to_string(),
    })
}

/// Builds a checked candidate whose entries match the authored combination and
/// the graph-projected configuration.
fn candidate(
    combination: &Combination,
    seed: u8,
) -> Result<CheckedSubjectManifestV2, Box<dyn Error>> {
    let entries: Vec<serde_json::Value> = combination
        .cohort
        .assignments()
        .iter()
        .map(|assignment| {
            let occurrence = assignment.occurrence().as_str();
            let kind = if occurrence == "mailer" {
                json!({
                    "provider_configuration_digest": combination.provider_configuration,
                })
            } else {
                json!({
                    "target_configuration_digest": combination.target_configuration,
                })
            };
            let mut entry = json!({
                "occurrence": occurrence,
                "repository": assignment.repository().as_str(),
                "source_commit": assignment.source_commit().as_str(),
                "source_content_digest": assignment.source_content().map(|d| d.to_string()),
                "build_id": assignment.build_id().map(|b| b.as_str()),
                "artifact_digest": assignment.artifact_digest().map(|d| d.to_string()),
            });
            let object = entry.as_object_mut().ok_or("entry object")?;
            for (key, value) in kind.as_object().ok_or("kind object")? {
                object.insert(key.clone(), value.clone());
            }
            Ok(entry)
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    let wire = json!({
        "schema": eqm_protocol::SUBJECT_MANIFEST_SCHEMA_V2.to_string(),
        "id": format!("subject.signup.{seed}"),
        "revision": 1,
        "entries": entries,
    });
    Ok(CheckedSubjectManifestV2::from_bytes(&serde_json::to_vec(
        &wire,
    )?)?)
}

/// Reconstructs the wire object for one-axis mutations.
fn candidate_wire(
    combination: &Combination,
    seed: u8,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let entries: Vec<serde_json::Value> = combination
        .cohort
        .assignments()
        .iter()
        .map(|assignment| {
            let occurrence = assignment.occurrence().as_str();
            let kind = if occurrence == "mailer" {
                json!({
                    "provider_configuration_digest": combination.provider_configuration,
                })
            } else {
                json!({
                    "target_configuration_digest": combination.target_configuration,
                })
            };
            let mut entry = json!({
                "occurrence": occurrence,
                "repository": assignment.repository().as_str(),
                "source_commit": assignment.source_commit().as_str(),
                "source_content_digest": assignment.source_content().map(|d| d.to_string()),
                "build_id": assignment.build_id().map(|b| b.as_str()),
                "artifact_digest": assignment.artifact_digest().map(|d| d.to_string()),
            });
            let object = entry.as_object_mut().ok_or("entry object")?;
            for (key, value) in kind.as_object().ok_or("kind object")? {
                object.insert(key.clone(), value.clone());
            }
            Ok(entry)
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    Ok(json!({
        "schema": eqm_protocol::SUBJECT_MANIFEST_SCHEMA_V2.to_string(),
        "id": format!("subject.signup.{seed}"),
        "revision": 1,
        "entries": entries,
    }))
}

fn checked_context(
    fixture: &Fixture,
    region: SelectedDimensionStateV2,
    rule_seed: &[u8],
) -> Result<
    (
        CheckedComparisonContextV2,
        eqm_domain::ComparisonContextDigest,
    ),
    Box<dyn Error>,
> {
    let selection = selection(&fixture.policy, &fixture.profile, region)?;
    let rule = Sha256Digest::hash_content(rule_seed);
    let derived =
        shared_comparison_context(&fixture.finalized, &fixture.scenario, &selection, rule)?;
    let contract = eqm_manifest::canonicalize_contract_v2(fixture.finalized.graph())?.digest();
    let input = ComparisonContextInputV2::new(contract, &fixture.scenario, &selection, rule);
    let mut wire = eqm_manifest::project_comparison_context_v2(&input)?;
    wire["schema"] = json!(eqm_protocol::COMPARISON_CONTEXT_SCHEMA_V2.to_string());
    let checked = CheckedComparisonContextV2::from_bytes(&serde_json::to_vec(&wire)?)?;
    assert_eq!(checked.digest(), derived);
    Ok((checked, derived))
}

fn evaluate(
    fixture: &Fixture,
    checked: &CheckedComparisonContextV2,
    combination: &Combination,
    seed: u8,
) -> Result<CheckedComparisonOutcome, Box<dyn Error>> {
    let selection = selection(
        &fixture.policy,
        &fixture.profile,
        SelectedDimensionStateV2::Unknown,
    )?;
    let graph = graph_with_cohorts(fixture, vec![combination.cohort.clone()])?;
    let resolved = resolve_cohort(&graph, &combination.cohort)?;
    let candidate = candidate(combination, seed)?;
    Ok(evaluate_checked_comparison(
        &graph,
        &fixture.scenario,
        &selection,
        Sha256Digest::hash_content(b"rule"),
        checked,
        &resolved,
        &candidate,
    )?)
}

#[test]
fn protocol_reader_reproduces_derived_graph_context() -> Result<(), Box<dyn Error>> {
    let fixture = fixture()?;
    let (checked, derived) = checked_context(&fixture, SelectedDimensionStateV2::Unknown, b"rule")?;
    assert_eq!(checked.digest(), derived);
    Ok(())
}

#[test]
fn distinct_valid_builds_share_one_context_and_are_eligible() -> Result<(), Box<dyn Error>> {
    let fixture = fixture()?;
    let (checked, _) = checked_context(&fixture, SelectedDimensionStateV2::Unknown, b"rule")?;
    let first = evaluate(&fixture, &checked, &combination(1)?, 1)?;
    let second = evaluate(&fixture, &checked, &combination(2)?, 2)?;
    assert_eq!(first.eligibility(), ComparisonEligibility::Eligible);
    assert_eq!(second.eligibility(), ComparisonEligibility::Eligible);
    // Genuinely different exact subject identities still share one context.
    assert_ne!(
        first.subject_manifest_digest(),
        second.subject_manifest_digest()
    );
    assert_eq!(first.context_digest(), second.context_digest());
    // The matched carrier retains occurrence/kind/configuration/content authority.
    assert_eq!(first.matched().assignments().len(), 2);
    // Assignments are sorted by occurrence: mailer (provider) then phone (target).
    assert!(
        first.matched().assignments()[0]
            .provider_configuration()
            .is_some()
    );
    assert!(
        first.matched().assignments()[1]
            .target_configuration()
            .is_some()
    );
    Ok(())
}

#[test]
fn repeated_same_target_actors_stay_distinct_and_eligible() -> Result<(), Box<dyn Error>> {
    let fixture = fixture_with(repeated_scenario(1)?)?;
    let (checked, _) = checked_context(&fixture, SelectedDimensionStateV2::Unknown, b"rule")?;
    let outcome = evaluate(&fixture, &checked, &repeated_combination(1)?, 1)?;
    assert_eq!(outcome.eligibility(), ComparisonEligibility::Eligible);
    let assignments = outcome.matched().assignments();
    assert_eq!(assignments.len(), 3);
    let occurrences: Vec<&str> = assignments
        .iter()
        .map(|assignment| assignment.occurrence().as_str())
        .collect();
    assert_eq!(occurrences, ["mailer", "phone_a", "phone_b"]);
    // The two phone occurrences project the *same* target configuration but
    // remain separately keyed.
    assert_eq!(
        assignments[1].target_configuration(),
        assignments[2].target_configuration()
    );
    assert_ne!(assignments[1].occurrence(), assignments[2].occurrence());
    Ok(())
}

#[test]
fn context_and_rule_substitution_is_rejected() -> Result<(), Box<dyn Error>> {
    let fixture = fixture()?;
    let selection = selection(
        &fixture.policy,
        &fixture.profile,
        SelectedDimensionStateV2::Unknown,
    )?;
    let combination = combination(1)?;
    let graph = graph_with_cohorts(&fixture, vec![combination.cohort.clone()])?;
    let resolved = resolve_cohort(&graph, &combination.cohort)?;
    let candidate = candidate(&combination, 1)?;

    // A checked context built from a different rule is not the graph context.
    let (other_rule, _) = checked_context(&fixture, SelectedDimensionStateV2::Unknown, b"other")?;
    assert_eq!(
        evaluate_checked_comparison(
            &graph,
            &fixture.scenario,
            &selection,
            Sha256Digest::hash_content(b"rule"),
            &other_rule,
            &resolved,
            &candidate,
        ),
        Err(ComparisonContextError::ContextNotBound)
    );

    // A checked context built from a different selected region is rejected.
    let (other_region, _) = checked_context(
        &fixture,
        SelectedDimensionStateV2::Known(SymbolicValueId::new("eu")?),
        b"rule",
    )?;
    assert_eq!(
        evaluate_checked_comparison(
            &graph,
            &fixture.scenario,
            &selection,
            Sha256Digest::hash_content(b"rule"),
            &other_region,
            &resolved,
            &candidate,
        ),
        Err(ComparisonContextError::ContextNotBound)
    );
    Ok(())
}

#[test]
fn subset_extra_wrong_kind_and_wrong_occurrence_fail_matching() -> Result<(), Box<dyn Error>> {
    let fixture = fixture()?;
    let (checked, _) = checked_context(&fixture, SelectedDimensionStateV2::Unknown, b"rule")?;
    let base_combination = combination(1)?;
    let graph = graph_with_cohorts(&fixture, vec![base_combination.cohort.clone()])?;
    let resolved = resolve_cohort(&graph, &base_combination.cohort)?;
    let selection = selection(
        &fixture.policy,
        &fixture.profile,
        SelectedDimensionStateV2::Unknown,
    )?;
    let rule = Sha256Digest::hash_content(b"rule");
    let evaluate = |candidate: &CheckedSubjectManifestV2| {
        evaluate_checked_comparison(
            &graph,
            &fixture.scenario,
            &selection,
            rule,
            &checked,
            &resolved,
            candidate,
        )
    };

    // A subset that omits an expected occurrence is rejected.
    let mut subset = candidate_wire(&combination(1)?, 1)?;
    subset["entries"].as_array_mut().ok_or("entries")?.pop();
    let subset = CheckedSubjectManifestV2::from_bytes(&serde_json::to_vec(&subset)?)?;
    assert!(matches!(
        evaluate(&subset),
        Err(ComparisonContextError::SubjectMatch(
            SubjectMatchError::MissingOccurrence(_)
        ))
    ));

    // An extra undeclared occurrence is rejected.
    let mut extra = candidate_wire(&combination(1)?, 1)?;
    extra["entries"]
        .as_array_mut()
        .ok_or("entries")?
        .push(json!({
            "occurrence": "absent",
            "repository": "https://example.com/team/project",
            "source_commit": "a".repeat(40),
            "source_content_digest": Sha256Digest::hash_content(&[9]).to_string(),
            "target_configuration_digest": combination(1)?.target_configuration,
            "build_id": "build.9",
            "artifact_digest": Sha256Digest::hash_content(&[9, 9]).to_string(),
        }));
    let extra = CheckedSubjectManifestV2::from_bytes(&serde_json::to_vec(&extra)?)?;
    assert!(matches!(
        evaluate(&extra),
        Err(ComparisonContextError::SubjectMatch(
            SubjectMatchError::ExtraOccurrence(_)
        ))
    ));

    // A phone target entry relabeled as a provider is a kind mismatch.
    let mut relabeled = candidate_wire(&combination(1)?, 1)?;
    let entries = relabeled["entries"].as_array_mut().ok_or("entries")?;
    let phone_index = entries
        .iter()
        .position(|entry| entry["occurrence"] == json!("phone"))
        .ok_or("phone index")?;
    let object = entries[phone_index].as_object_mut().ok_or("phone object")?;
    object.remove("target_configuration_digest");
    object.insert(
        "provider_configuration_digest".to_owned(),
        json!(combination(1)?.provider_configuration),
    );
    let relabeled = CheckedSubjectManifestV2::from_bytes(&serde_json::to_vec(&relabeled)?)?;
    assert!(matches!(
        evaluate(&relabeled),
        Err(ComparisonContextError::SubjectMatch(
            SubjectMatchError::KindMismatch(_)
        ))
    ));

    // A wrong occurrence key is rejected as missing plus extra, never mapped.
    let mut wrong = candidate_wire(&combination(1)?, 1)?;
    let entries = wrong["entries"].as_array_mut().ok_or("entries")?;
    let phone_index = entries
        .iter()
        .position(|entry| entry["occurrence"] == json!("phone"))
        .ok_or("phone index")?;
    entries[phone_index]["occurrence"] = json!("wrongphone");
    let wrong = CheckedSubjectManifestV2::from_bytes(&serde_json::to_vec(&wrong)?)?;
    assert!(matches!(
        evaluate(&wrong),
        Err(ComparisonContextError::SubjectMatch(
            SubjectMatchError::MissingOccurrence(_)
        ))
    ));

    // Duplicate occurrences never reach the matcher: the strict reader rejects.
    let mut duplicate = candidate_wire(&combination(1)?, 1)?;
    let first = duplicate["entries"][0].clone();
    duplicate["entries"]
        .as_array_mut()
        .ok_or("entries")?
        .push(first);
    assert!(CheckedSubjectManifestV2::from_bytes(&serde_json::to_vec(&duplicate)?).is_err());
    Ok(())
}

#[test]
fn graph_membership_is_required_for_every_definition() -> Result<(), Box<dyn Error>> {
    let fixture = fixture()?;
    let base_selection = selection(
        &fixture.policy,
        &fixture.profile,
        SelectedDimensionStateV2::Unknown,
    )?;
    let rule = Sha256Digest::hash_content(b"rule");
    let absent_scenario = scenario(9)?;
    assert_eq!(
        shared_comparison_context(&fixture.finalized, &absent_scenario, &base_selection, rule),
        Err(ComparisonContextError::ScenarioNotInGraph)
    );
    let absent_policy = policy(9)?;
    let absent_selection = selection(
        &absent_policy,
        &fixture.profile,
        SelectedDimensionStateV2::Unknown,
    )?;
    assert_eq!(
        shared_comparison_context(
            &fixture.finalized,
            &fixture.scenario,
            &absent_selection,
            rule
        ),
        Err(ComparisonContextError::PolicyNotInGraph)
    );
    Ok(())
}

#[test]
fn one_axis_scenario_profile_and_rule_changes_move_context_identity() -> Result<(), Box<dyn Error>>
{
    let fixture = fixture()?;
    let (_, base_digest) = checked_context(&fixture, SelectedDimensionStateV2::Unknown, b"rule")?;
    let base_selection = selection(
        &fixture.policy,
        &fixture.profile,
        SelectedDimensionStateV2::Unknown,
    )?;
    let rule = Sha256Digest::hash_content(b"rule");

    // A scenario revision absent from the graph is rejected, never reloaded.
    assert_eq!(
        shared_comparison_context(&fixture.finalized, &scenario(2)?, &base_selection, rule),
        Err(ComparisonContextError::ScenarioNotInGraph)
    );

    // A profile absent from the graph is rejected.
    let other_profile = profile(2)?;
    let other_selection = selection(
        &fixture.policy,
        &other_profile,
        SelectedDimensionStateV2::Unknown,
    )?;
    assert_eq!(
        shared_comparison_context(
            &fixture.finalized,
            &fixture.scenario,
            &other_selection,
            rule
        ),
        Err(ComparisonContextError::ProfileNotInGraph)
    );

    // Only the explicit rule reference changes; the derived identity moves.
    let derived = shared_comparison_context(
        &fixture.finalized,
        &fixture.scenario,
        &base_selection,
        Sha256Digest::hash_content(b"other-rule"),
    )?;
    assert_ne!(derived, base_digest);
    Ok(())
}

#[test]
fn distinct_rule_references_never_collapse() -> Result<(), Box<dyn Error>> {
    let fixture = fixture()?;
    let first = checked_context(&fixture, SelectedDimensionStateV2::Unknown, b"rule-a")?.1;
    let second = checked_context(&fixture, SelectedDimensionStateV2::Unknown, b"rule-b")?.1;
    assert_ne!(first, second);
    Ok(())
}

// ---------------------------------------------------------------------------
// S039-R3: the consumed resolved cohort is bound to exact graph/scenario value
// authority before any candidate matching can produce eligibility.
// ---------------------------------------------------------------------------

#[test]
fn foreign_or_missing_consuming_cohort_is_rejected() -> Result<(), Box<dyn Error>> {
    let fixture = fixture()?;
    let selection = selection(
        &fixture.policy,
        &fixture.profile,
        SelectedDimensionStateV2::Unknown,
    )?;
    let rule = Sha256Digest::hash_content(b"rule");
    let (checked, _) = checked_context(&fixture, SelectedDimensionStateV2::Unknown, b"rule")?;
    let combination = combination(1)?;

    // Resolve against a graph that owns the cohort...
    let owning = graph_with_cohorts(&fixture, vec![combination.cohort.clone()])?;
    let resolved = resolve_cohort(&owning, &combination.cohort)?;
    let candidate = candidate(&combination, 1)?;

    // ...then consume against a graph that has no cohort at all. Foreign
    // resolved authority must never yield Eligible.
    let bare = graph_with_cohorts(&fixture, Vec::new())?;
    assert_eq!(
        evaluate_checked_comparison(
            &bare,
            &fixture.scenario,
            &selection,
            rule,
            &checked,
            &resolved,
            &candidate,
        ),
        Err(ComparisonContextError::CohortNotInGraph)
    );
    Ok(())
}

#[test]
fn another_scenarios_authority_with_matching_names_is_rejected() -> Result<(), Box<dyn Error>> {
    let fixture = fixture()?;
    let selection = selection(
        &fixture.policy,
        &fixture.profile,
        SelectedDimensionStateV2::Unknown,
    )?;
    let rule = Sha256Digest::hash_content(b"rule");
    let (checked, _) = checked_context(&fixture, SelectedDimensionStateV2::Unknown, b"rule")?;

    // A cohort authored for `other.flow` whose occurrence names, roles and
    // subjects happen to match `signup.flow`.
    let other_cohort = Cohort::new(
        SelectorText::new("cohort.other")?,
        Revision::new(1)?,
        ScenarioId::new("other.flow")?,
        vec![assignment("phone", 1)?, assignment("mailer", 2)?],
        Extensions::default(),
    )?;
    let graph = graph_with_scenarios(
        &fixture,
        vec![fixture.scenario.clone(), other_scenario(1)?],
        vec![other_cohort.clone()],
    )?;
    let resolved = resolve_cohort(&graph, &other_cohort)?;
    let candidate = candidate(&combination(1)?, 1)?;
    assert_eq!(
        evaluate_checked_comparison(
            &graph,
            &fixture.scenario,
            &selection,
            rule,
            &checked,
            &resolved,
            &candidate,
        ),
        Err(ComparisonContextError::CohortScenario)
    );
    Ok(())
}

#[test]
fn altered_same_id_revision_cohort_is_rejected() -> Result<(), Box<dyn Error>> {
    let fixture = fixture()?;
    let selection = selection(
        &fixture.policy,
        &fixture.profile,
        SelectedDimensionStateV2::Unknown,
    )?;
    let rule = Sha256Digest::hash_content(b"rule");
    let (checked, _) = checked_context(&fixture, SelectedDimensionStateV2::Unknown, b"rule")?;
    let combination = combination(1)?;
    let owning = graph_with_cohorts(&fixture, vec![combination.cohort.clone()])?;
    let resolved = resolve_cohort(&owning, &combination.cohort)?;
    let candidate = candidate(&combination, 1)?;

    // Same id and revision, different assignment content: not the exact
    // graph-owned value, so the retained authority is stale.
    let altered = Cohort::new(
        SelectorText::new("cohort.signup.1")?,
        Revision::new(1)?,
        ScenarioId::new("signup.flow")?,
        vec![assignment("phone", 8)?, assignment("mailer", 9)?],
        Extensions::default(),
    )?;
    let consuming = graph_with_cohorts(&fixture, vec![altered])?;
    assert_eq!(
        evaluate_checked_comparison(
            &consuming,
            &fixture.scenario,
            &selection,
            rule,
            &checked,
            &resolved,
            &candidate,
        ),
        Err(ComparisonContextError::CohortNotInGraph)
    );
    Ok(())
}

#[test]
fn changed_consuming_configuration_is_rejected() -> Result<(), Box<dyn Error>> {
    let fixture = fixture()?;
    let selection = selection(
        &fixture.policy,
        &fixture.profile,
        SelectedDimensionStateV2::Unknown,
    )?;
    let rule = Sha256Digest::hash_content(b"rule");
    let (checked, _) = checked_context(&fixture, SelectedDimensionStateV2::Unknown, b"rule")?;
    let combination = combination(1)?;
    let owning = graph_with_cohorts(&fixture, vec![combination.cohort.clone()])?;
    let resolved = resolve_cohort(&owning, &combination.cohort)?;
    let candidate = candidate(&combination, 1)?;

    // The authored cohort claim is unchanged, but the consuming graph projects
    // a different target configuration. Re-resolution must reject it.
    let changed_target = Target::new(
        TargetId::new("web")?,
        RepoPath::new("apps/web")?,
        PlatformId::new("web")?,
        FrameworkId::new("other_framework")?,
        vec![owner("owner://team/web")?],
        Extensions::default(),
    )?;
    let changed_target_graph =
        graph_with_target(&fixture, changed_target, vec![combination.cohort.clone()])?;
    assert!(matches!(
        evaluate_checked_comparison(
            &changed_target_graph,
            &fixture.scenario,
            &selection,
            rule,
            &checked,
            &resolved,
            &candidate,
        ),
        Err(ComparisonContextError::Cohort(
            eqm_application::CohortError::ConfigurationMismatch(_)
        ))
    ));

    // The same holds for a changed provider configuration.
    let changed_provider = Provider::new(
        ProviderId::new("provider.mailgun")?,
        Sha256Digest::hash_content(b"other configuration"),
        vec![owner("owner://team/other")?],
        Extensions::default(),
    )?;
    let changed_provider_graph =
        graph_with_provider(&fixture, changed_provider, vec![combination.cohort.clone()])?;
    assert!(matches!(
        evaluate_checked_comparison(
            &changed_provider_graph,
            &fixture.scenario,
            &selection,
            rule,
            &checked,
            &resolved,
            &candidate,
        ),
        Err(ComparisonContextError::Cohort(
            eqm_application::CohortError::ConfigurationMismatch(_)
        ))
    ));
    Ok(())
}

#[test]
fn different_consuming_scenario_revision_is_rejected() -> Result<(), Box<dyn Error>> {
    let fixture = fixture()?;
    let selection = selection(
        &fixture.policy,
        &fixture.profile,
        SelectedDimensionStateV2::Unknown,
    )?;
    let rule = Sha256Digest::hash_content(b"rule");
    let combination = combination(1)?;
    let owning = graph_with_cohorts(&fixture, vec![combination.cohort.clone()])?;
    let resolved = resolve_cohort(&owning, &combination.cohort)?; // revision 1
    let candidate = candidate(&combination, 1)?;

    // Consume with a selected scenario revision 2 that is present in the graph.
    let revised = scenario(2)?;
    let consuming = graph_with_scenarios(
        &fixture,
        vec![revised.clone()],
        vec![combination.cohort.clone()],
    )?;
    let fixture_at_revision_two = Fixture {
        finalized: consuming.clone(),
        scenario: revised.clone(),
        policy: fixture.policy.clone(),
        profile: fixture.profile.clone(),
    };
    let (checked, _) = checked_context(
        &fixture_at_revision_two,
        SelectedDimensionStateV2::Unknown,
        b"rule",
    )?;
    assert_eq!(
        evaluate_checked_comparison(
            &consuming, &revised, &selection, rule, &checked, &resolved, &candidate,
        ),
        Err(ComparisonContextError::CohortScenario)
    );
    Ok(())
}

#[test]
fn substituted_candidate_content_is_rejected_after_binding() -> Result<(), Box<dyn Error>> {
    let fixture = fixture()?;
    let selection = selection(
        &fixture.policy,
        &fixture.profile,
        SelectedDimensionStateV2::Unknown,
    )?;
    let rule = Sha256Digest::hash_content(b"rule");
    let (checked, _) = checked_context(&fixture, SelectedDimensionStateV2::Unknown, b"rule")?;
    let combination = combination(1)?;
    let graph = graph_with_cohorts(&fixture, vec![combination.cohort.clone()])?;
    let resolved = resolve_cohort(&graph, &combination.cohort)?;

    // A build-axis substitution in the candidate is still rejected by the
    // bytes-first matcher after the cohort binding succeeds.
    let mut wire = candidate_wire(&combination, 1)?;
    wire["entries"][0]["build_id"] = json!("build.substituted");
    let substituted = CheckedSubjectManifestV2::from_bytes(&serde_json::to_vec(&wire)?)?;
    assert!(matches!(
        evaluate_checked_comparison(
            &graph,
            &fixture.scenario,
            &selection,
            rule,
            &checked,
            &resolved,
            &substituted,
        ),
        Err(ComparisonContextError::SubjectMatch(
            SubjectMatchError::BuildMismatch(_)
        ))
    ));
    Ok(())
}

#[test]
fn independently_prepared_equivalent_authority_passes() -> Result<(), Box<dyn Error>> {
    let fixture = fixture()?;
    let selection = selection(
        &fixture.policy,
        &fixture.profile,
        SelectedDimensionStateV2::Unknown,
    )?;
    let rule = Sha256Digest::hash_content(b"rule");
    let (checked, _) = checked_context(&fixture, SelectedDimensionStateV2::Unknown, b"rule")?;
    let combination = combination(1)?;

    // Resolution and consumption use two independently constructed graphs whose
    // cohort/scenario/configuration values are equal. Value authority, not
    // pointer identity, is what the consuming boundary requires.
    let owning = graph_with_cohorts(&fixture, vec![combination.cohort.clone()])?;
    let resolved = resolve_cohort(&owning, &combination.cohort)?;
    let consuming = graph_with_cohorts(&fixture, vec![combination.cohort.clone()])?;
    let candidate = candidate(&combination, 1)?;
    let outcome = evaluate_checked_comparison(
        &consuming,
        &fixture.scenario,
        &selection,
        rule,
        &checked,
        &resolved,
        &candidate,
    )?;
    assert_eq!(outcome.eligibility(), ComparisonEligibility::Eligible);
    Ok(())
}
