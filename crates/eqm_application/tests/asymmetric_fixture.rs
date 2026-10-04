//! S042: synthetic asymmetric CLI/desktop/mobile/provider fixture.
//!
//! Exercises shared/transitive sources, repeated role-bearing actors, distinct
//! independently released build combinations and explicit N/N+1 cohorts through
//! real load, resolve, prepare, projection and checked subject matching. The
//! fixture is intentionally synthetic: it invents no real product build task and
//! its intentional exclusions are not waivers.

use eqm_application::{
    ComparisonContextError, PreparedSession, SessionRequest, evaluate_checked_comparison,
    match_cohort_subjects, prepare, resolve_cohort, shared_comparison_context,
};
use eqm_domain::{
    ComparisonContextInputV2, DigestDomain, DimensionId, PolicySelectionInputV2, ScenarioId,
    ScenarioSubject, SchemaVersion, SelectedDimensionStateV2, SelectedProfileInputV2, Sha256Digest,
    SymbolicValueId, TargetId,
};
use eqm_engine::ComparisonEligibility;
use eqm_manifest::{canonicalize_provider_configuration_v2, canonicalize_target_configuration_v2};
use eqm_protocol::schema_graph_v2::CheckedComparisonContextV2;
use eqm_protocol::v2::CheckedSubjectManifestV2;
use serde_json::json;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

fn copy_tree(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        if entry.file_name() == "GIT_HEAD.fixture" {
            let git = destination.join(".git");
            fs::create_dir_all(&git)?;
            fs::copy(entry.path(), git.join("HEAD"))?;
            continue;
        }
        let target = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

fn fixture() -> Result<tempfile::TempDir, Box<dyn Error>> {
    let corpus = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/asymmetric");
    let directory = tempfile::tempdir()?;
    copy_tree(&corpus, directory.path())?;
    Ok(directory)
}

fn example_fixture() -> Result<tempfile::TempDir, Box<dyn Error>> {
    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/android-ios");
    let directory = tempfile::tempdir()?;
    copy_tree(&source, directory.path())?;
    fs::create_dir(directory.path().join(".git"))?;
    Ok(directory)
}

/// S043-R5: the P03-R11 scalar and P03-R12 local collection/combination owning
/// repairs survive the genuine authored loader, checked preparation and v2
/// projection, and the repaired members are retained on the finalized graph.
#[test]
fn example_workspace_owning_forms_survive_real_preparation_and_projection()
-> Result<(), Box<dyn Error>> {
    let repository = example_fixture()?;
    let otp = repository.path().join("eqm/contracts/auth.signup.otp.toml");
    let text = fs::read_to_string(&otp)?;
    fs::write(
        &otp,
        text.replace(
            "\"sha256:f4bd7d44c2fc00b13840d5f1d4b04826d6830e698359e4e468ce51d931e36378\"",
            "\"sha256:f4bd7d44c2fc00b13840d5f1d4b04826d6830e698359e4e468ce51d931e36378\"\nprefix = \"card\"",
        ),
    )?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    let graph = session.finalized().graph();

    // RF66: the owning single-segment prefix is retained on the genuine pin.
    let otp_surface = graph
        .surfaces()
        .get(&eqm_domain::SurfaceId::new("account.create.signup.otp")?)
        .ok_or("otp surface")?;
    let fragment_use = otp_surface
        .fragments()
        .iter()
        .next()
        .ok_or("fragment use")?;
    assert_eq!(
        fragment_use
            .prefix()
            .map(eqm_domain::LocalRequirementId::as_str),
        Some("card")
    );

    // RF67: the genuine ordered journey collection is retained.
    let journey = graph
        .journeys()
        .get(&eqm_domain::JourneyId::new("account.create.signup")?)
        .ok_or("journey")?;
    assert_eq!(journey.surfaces().len(), 2);

    // RF67: a local runner retains no guarantees and the fixed PATH binding
    // retains no literal value.
    let runner = graph
        .runners()
        .values()
        .find(|runner| runner.id().as_str() == "runner.ios")
        .ok_or("runner.ios")?;
    assert_eq!(runner.backend(), eqm_domain::RunnerBackend::Local);
    assert!(runner.guarantees().is_empty());
    let environment = runner.environment().values().next().ok_or("environment")?;
    assert_eq!(environment.name().as_str(), "PATH");
    assert_eq!(
        environment.source(),
        &eqm_domain::EnvironmentSource::TrustedPath
    );

    // RF67: the policy selector axis and coherent waiver duration are retained.
    let policy = graph
        .policies()
        .values()
        .find(|policy| policy.id().as_str() == "consumer.critical_flow")
        .ok_or("policy")?;
    assert!(
        policy
            .rules()
            .iter()
            .all(|rule| rule.selector().risk_classes().is_some()),
        "policy selector axis must be retained"
    );
    assert!(policy.waivers().maximum_days().is_some());

    // RF67: the user-visible view artifact retains its surface locator.
    let artifact = graph
        .bindings()
        .values()
        .find(|binding| binding.id().as_str() == "binding.ios.auth_signup")
        .and_then(|binding| binding.artifacts().values().values().next())
        .ok_or("artifact")?;
    assert!(artifact.surface().is_some());

    // The genuine v2 projection remains complete and order-preserving.
    let projected = eqm_manifest::project_graph_v2(session.finalized())?;
    assert_eq!(
        projected["journeys"][0]["surfaces"]
            .as_array()
            .map(Vec::len),
        Some(2)
    );
    Ok(())
}

fn evaluation_time() -> Result<eqm_domain::UtcInstant, Box<dyn Error>> {
    Ok("2026-01-02T03:04:05Z".parse()?)
}

#[test]
fn asymmetric_fixture_loads_resolves_projects_and_matches_cohorts() -> Result<(), Box<dyn Error>> {
    let repository = fixture()?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    assert_eq!(session.semantic_graph_version(), SchemaVersion::V2);
    assert_eq!(
        session.semantic_graph_domain(),
        DigestDomain::SemanticGraphV2
    );
    let graph = session.finalized().graph();

    // Shared/transitive sources and asymmetric roles.
    assert_eq!(graph.targets().len(), 3);
    assert_eq!(graph.source_components().len(), 2);
    assert_eq!(graph.providers().len(), 1);
    assert_eq!(graph.scenarios().len(), 1);
    assert_eq!(graph.cohorts().len(), 2);
    assert_eq!(graph.participation().len(), 1);
    let cli = graph.targets().get(&TargetId::new("cli")?).ok_or("cli")?;
    let mobile = graph
        .targets()
        .get(&TargetId::new("mobile")?)
        .ok_or("mobile")?;
    assert_ne!(cli.role(), mobile.role());
    assert!(
        graph
            .source_components()
            .get(&eqm_domain::SourceComponentId::new("shared.kmp")?)
            .ok_or("shared.kmp")?
            .dependencies()
            .contains(&eqm_domain::SourceComponentId::new("shared.core")?)
    );

    // Repeated mobile actors remain distinct participants.
    let signup = ScenarioId::new("signup.flow")?;
    let scenario = graph
        .scenarios()
        .values()
        .find(|scenario| scenario.id() == &signup)
        .ok_or("scenario")?;
    assert_eq!(scenario.participants().len(), 3);

    // The v2 projection is complete and identity-stable.
    let projected = eqm_manifest::project_graph_v2(session.finalized())?;
    assert_eq!(projected["cohorts"].as_array().map(Vec::len), Some(2));
    let canonical = eqm_manifest::canonicalize_graph_v2(session.finalized())?;
    assert_eq!(session.workspace_digest(), canonical.digest().as_digest());

    // Both approved cohorts resolve against the immutable graph; N and N+1 are
    // distinct independently released combinations, not an inferred window.
    let cohort_n = graph
        .cohorts()
        .values()
        .find(|cohort| cohort.id().as_str() == "cohort.n")
        .ok_or("cohort.n")?;
    let cohort_n1 = graph
        .cohorts()
        .values()
        .find(|cohort| cohort.id().as_str() == "cohort.nplus1")
        .ok_or("cohort.nplus1")?;
    let resolved_n = resolve_cohort(session.finalized(), cohort_n)?;
    let resolved_n1 = resolve_cohort(session.finalized(), cohort_n1)?;
    assert_eq!(resolved_n.occurrences(), 3);
    assert_eq!(resolved_n1.occurrences(), 3);

    let target_config = canonicalize_target_configuration_v2(mobile)?
        .digest()
        .to_string();
    let provider = graph.providers().values().next().ok_or("provider")?;
    let provider_config = canonicalize_provider_configuration_v2(provider)?
        .digest()
        .to_string();

    let candidate_n = candidate(scenario, cohort_n, &target_config, &provider_config)?;
    let candidate_n1 = candidate(scenario, cohort_n1, &target_config, &provider_config)?;
    assert!(match_cohort_subjects(&candidate_n, &resolved_n).is_ok());
    assert!(match_cohort_subjects(&candidate_n1, &resolved_n1).is_ok());
    // Cross-cohort matching is not accepted: the independently released
    // combinations are distinct.
    assert!(match_cohort_subjects(&candidate_n, &resolved_n1).is_err());
    assert!(match_cohort_subjects(&candidate_n1, &resolved_n).is_err());
    Ok(())
}

#[test]
fn asymmetric_fixture_exercises_checked_comparison_consumer() -> Result<(), Box<dyn Error>> {
    let repository = fixture()?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    let graph = session.finalized().graph();
    let scenario = graph
        .scenarios()
        .values()
        .find(|scenario| scenario.id().as_str() == "signup.flow")
        .ok_or("scenario")?;
    let policy = graph
        .policies()
        .values()
        .find(|policy| policy.id().as_str() == "policy.default")
        .ok_or("policy")?;
    let profile = graph
        .profiles()
        .values()
        .find(|profile| profile.id().as_str() == "audience.default")
        .ok_or("profile")?;
    let selection = PolicySelectionInputV2::new(
        policy,
        vec![SelectedProfileInputV2::new(
            profile,
            vec![(
                DimensionId::new("locale")?,
                SelectedDimensionStateV2::Known(SymbolicValueId::new("en")?),
            )],
        )?],
    )?;
    let rule = Sha256Digest::hash_content(b"approved.rule");
    let contract = eqm_manifest::canonicalize_contract_v2(session.finalized())?.digest();
    let input = ComparisonContextInputV2::new(contract, scenario, &selection, rule);
    let mut wire = eqm_manifest::project_comparison_context_v2(&input)?;
    wire["schema"] = json!(eqm_protocol::COMPARISON_CONTEXT_SCHEMA_V2.to_string());
    let checked = CheckedComparisonContextV2::from_bytes(&serde_json::to_vec(&wire)?)?;
    // The protocol reader reproduces the graph-derived comparison identity.
    let derived = shared_comparison_context(session.finalized(), scenario, &selection, rule)?;
    assert_eq!(checked.digest(), derived);

    let mobile = graph
        .targets()
        .get(&TargetId::new("mobile")?)
        .ok_or("mobile")?;
    let provider = graph.providers().values().next().ok_or("provider")?;
    let target_config = canonicalize_target_configuration_v2(mobile)?
        .digest()
        .to_string();
    let provider_config = canonicalize_provider_configuration_v2(provider)?
        .digest()
        .to_string();

    for cohort_id in ["cohort.n", "cohort.nplus1"] {
        let cohort = graph
            .cohorts()
            .values()
            .find(|cohort| cohort.id().as_str() == cohort_id)
            .ok_or("cohort")?;
        let resolved = resolve_cohort(session.finalized(), cohort)?;
        // Retain the independently matched result; never rebuild a public
        // digest struct in its place.
        let candidate_subject = candidate(scenario, cohort, &target_config, &provider_config)?;
        let matched = match_cohort_subjects(&candidate_subject, &resolved)?;
        assert_eq!(matched.assignments().len(), 3);
        let outcome = evaluate_checked_comparison(
            session.finalized(),
            scenario,
            &selection,
            rule,
            &checked,
            &resolved,
            &candidate_subject,
        )?;
        assert_eq!(outcome.eligibility(), ComparisonEligibility::Eligible);
        assert_eq!(outcome.matched().assignments().len(), 3);
        assert_eq!(
            outcome.subject_manifest_digest(),
            candidate_subject.digest()
        );
    }

    // Negative counterpart: a substituted rule context is not the graph context.
    let other_rule = Sha256Digest::hash_content(b"other.rule");
    let other_input = ComparisonContextInputV2::new(contract, scenario, &selection, other_rule);
    let mut other_wire = eqm_manifest::project_comparison_context_v2(&other_input)?;
    other_wire["schema"] = json!(eqm_protocol::COMPARISON_CONTEXT_SCHEMA_V2.to_string());
    let other_checked = CheckedComparisonContextV2::from_bytes(&serde_json::to_vec(&other_wire)?)?;
    let cohort_n = graph
        .cohorts()
        .values()
        .find(|cohort| cohort.id().as_str() == "cohort.n")
        .ok_or("cohort.n")?;
    let resolved_n = resolve_cohort(session.finalized(), cohort_n)?;
    let candidate_n = candidate(scenario, cohort_n, &target_config, &provider_config)?;
    assert_eq!(
        evaluate_checked_comparison(
            session.finalized(),
            scenario,
            &selection,
            rule,
            &other_checked,
            &resolved_n,
            &candidate_n,
        ),
        Err(ComparisonContextError::ContextNotBound)
    );
    Ok(())
}

fn candidate(
    scenario: &eqm_domain::Scenario,
    cohort: &eqm_domain::Cohort,
    target_config: &str,
    provider_config: &str,
) -> Result<CheckedSubjectManifestV2, Box<dyn Error>> {
    let subjects: BTreeMap<&str, &ScenarioSubject> = scenario
        .participants()
        .iter()
        .map(|participant| (participant.occurrence().as_str(), participant.subject()))
        .collect();
    let mut entries = Vec::new();
    for assignment in cohort.assignments() {
        let subject = subjects
            .get(assignment.occurrence().as_str())
            .ok_or("occurrence")?;
        let mut entry = json!({
            "occurrence": assignment.occurrence().as_str(),
            "repository": assignment.repository().as_str(),
            "source_commit": assignment.source_commit().as_str(),
            "source_content_digest": assignment.source_content().map(|d| d.to_string()),
            "build_id": assignment.build_id().map(|b| b.as_str().to_owned()),
            "artifact_digest": assignment.artifact_digest().map(|d| d.to_string()),
        });
        match subject {
            ScenarioSubject::Provider(_) => {
                entry["provider_configuration_digest"] = json!(provider_config);
            }
            ScenarioSubject::Target(_) => {
                entry["target_configuration_digest"] = json!(target_config);
            }
        }
        entries.push(entry);
    }
    let wire = json!({
        "schema": eqm_protocol::SUBJECT_MANIFEST_SCHEMA_V2.to_string(),
        "id": "subject.signup",
        "revision": 1,
        "entries": entries,
    });
    Ok(CheckedSubjectManifestV2::from_bytes(&serde_json::to_vec(
        &wire,
    )?)?)
}

/// Checked comparison inputs derived for one independently prepared session.
struct ComparisonSetup<'a> {
    selection: PolicySelectionInputV2<'a>,
    rule: Sha256Digest,
    checked: CheckedComparisonContextV2,
    target_config: String,
    provider_config: String,
    scenario: &'a eqm_domain::Scenario,
}

/// Derives the checked comparison inputs for one independently prepared session.
fn comparison_setup(session: &PreparedSession) -> Result<ComparisonSetup<'_>, Box<dyn Error>> {
    let graph = session.finalized().graph();
    let scenario = graph
        .scenarios()
        .values()
        .find(|scenario| scenario.id().as_str() == "signup.flow")
        .ok_or("scenario")?;
    let policy = graph
        .policies()
        .values()
        .find(|policy| policy.id().as_str() == "policy.default")
        .ok_or("policy")?;
    let profile = graph
        .profiles()
        .values()
        .find(|profile| profile.id().as_str() == "audience.default")
        .ok_or("profile")?;
    let selection = PolicySelectionInputV2::new(
        policy,
        vec![SelectedProfileInputV2::new(
            profile,
            vec![(
                DimensionId::new("locale")?,
                SelectedDimensionStateV2::Known(SymbolicValueId::new("en")?),
            )],
        )?],
    )?;
    let rule = Sha256Digest::hash_content(b"approved.rule");
    let contract = eqm_manifest::canonicalize_contract_v2(session.finalized())?.digest();
    let input = ComparisonContextInputV2::new(contract, scenario, &selection, rule);
    let mut wire = eqm_manifest::project_comparison_context_v2(&input)?;
    wire["schema"] = json!(eqm_protocol::COMPARISON_CONTEXT_SCHEMA_V2.to_string());
    let checked = CheckedComparisonContextV2::from_bytes(&serde_json::to_vec(&wire)?)?;
    let mobile = graph
        .targets()
        .get(&TargetId::new("mobile")?)
        .ok_or("mobile")?;
    let provider = graph.providers().values().next().ok_or("provider")?;
    let target_config = canonicalize_target_configuration_v2(mobile)?
        .digest()
        .to_string();
    let provider_config = canonicalize_provider_configuration_v2(provider)?
        .digest()
        .to_string();
    Ok(ComparisonSetup {
        selection,
        rule,
        checked,
        target_config,
        provider_config,
        scenario,
    })
}

#[test]
fn prepared_fixture_cross_session_cohort_authority_is_value_bound() -> Result<(), Box<dyn Error>> {
    // Session A owns the exact resolved cohort used as retained authority.
    let repository_a = fixture()?;
    let session_a = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository_a.path(),
    )?;
    let cohort_a = session_a
        .finalized()
        .graph()
        .cohorts()
        .values()
        .find(|cohort| cohort.id().as_str() == "cohort.n")
        .ok_or("cohort.n")?;
    let resolved_a = resolve_cohort(session_a.finalized(), cohort_a)?;

    // An independently prepared but value-equal session may consume it: value
    // authority, not pointer identity.
    let repository_b = fixture()?;
    let session_b = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository_b.path(),
    )?;
    let setup_b = comparison_setup(&session_b)?;
    let cohort_b = session_b
        .finalized()
        .graph()
        .cohorts()
        .values()
        .find(|cohort| cohort.id().as_str() == "cohort.n")
        .ok_or("cohort.n")?;
    let candidate_b = candidate(
        setup_b.scenario,
        cohort_b,
        &setup_b.target_config,
        &setup_b.provider_config,
    )?;
    let outcome = evaluate_checked_comparison(
        session_b.finalized(),
        setup_b.scenario,
        &setup_b.selection,
        setup_b.rule,
        &setup_b.checked,
        &resolved_a,
        &candidate_b,
    )?;
    assert_eq!(outcome.eligibility(), ComparisonEligibility::Eligible);

    // An independently valid session with altered same-ID/revision cohort
    // content (a build-axis change) must reject the retained authority even
    // though every occurrence name still matches.
    let repository_c = fixture()?;
    let cohort_path = repository_c.path().join("eqm/contracts/cohort.n.toml");
    let text = fs::read_to_string(&cohort_path)?.replace("build.n.mailer", "build.altered.mailer");
    fs::write(&cohort_path, text)?;
    let session_c = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository_c.path(),
    )?;
    let setup_c = comparison_setup(&session_c)?;
    let cohort_c = session_c
        .finalized()
        .graph()
        .cohorts()
        .values()
        .find(|cohort| cohort.id().as_str() == "cohort.n")
        .ok_or("cohort.n")?;
    let candidate_c = candidate(
        setup_c.scenario,
        cohort_c,
        &setup_c.target_config,
        &setup_c.provider_config,
    )?;
    assert_eq!(
        evaluate_checked_comparison(
            session_c.finalized(),
            setup_c.scenario,
            &setup_c.selection,
            setup_c.rule,
            &setup_c.checked,
            &resolved_a,
            &candidate_c,
        ),
        Err(ComparisonContextError::CohortNotInGraph)
    );
    Ok(())
}

/// A second scenario in the real fixture whose occurrence names, roles and
/// subjects are identical to `signup.flow`.
const OTHER_SCENARIO: &str = r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/scenario.schema.json"
id = "other.flow"
revision = 1

[[participants]]
id = "mailer"
role = "provider"
[participants.subject]
provider = "provider.mailgun"

[[participants]]
id = "phone_one"
role = "edge"
[participants.subject]
target = "mobile"

[[participants]]
id = "phone_two"
role = "edge"
[participants.subject]
target = "mobile"
"#;

#[test]
fn prepared_fixture_cross_scenario_cohort_authority_is_rejected() -> Result<(), Box<dyn Error>> {
    let repository = fixture()?;
    fs::write(
        repository.path().join("eqm/contracts/scenario.other.toml"),
        OTHER_SCENARIO,
    )?;
    // Retarget `cohort.n` to the other scenario while its occurrence names,
    // roles and subjects still match `signup.flow` exactly.
    let cohort_path = repository.path().join("eqm/contracts/cohort.n.toml");
    let text = fs::read_to_string(&cohort_path)?
        .replace("scenario = \"signup.flow\"", "scenario = \"other.flow\"");
    fs::write(&cohort_path, text)?;

    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    let graph = session.finalized().graph();
    let cohort_n = graph
        .cohorts()
        .values()
        .find(|cohort| cohort.id().as_str() == "cohort.n")
        .ok_or("cohort.n")?;
    let resolved = resolve_cohort(session.finalized(), cohort_n)?;
    let setup = comparison_setup(&session)?;
    let candidate_subject = candidate(
        setup.scenario,
        cohort_n,
        &setup.target_config,
        &setup.provider_config,
    )?;
    assert_eq!(
        evaluate_checked_comparison(
            session.finalized(),
            setup.scenario,
            &setup.selection,
            setup.rule,
            &setup.checked,
            &resolved,
            &candidate_subject,
        ),
        Err(ComparisonContextError::CohortScenario)
    );
    Ok(())
}

#[test]
fn prepared_fixture_admits_repository_and_extension_boundaries() -> Result<(), Box<dyn Error>> {
    // Positive: the existing parser's explicit-port repository and a valid
    // authored extension namespace are admitted by real preparation.
    let repository = fixture()?;
    let cohort_path = repository.path().join("eqm/contracts/cohort.n.toml");
    let text = fs::read_to_string(&cohort_path)?.replace(
        "https://example.com/team/project",
        "https://example.com:443/team/repo",
    );
    fs::write(
        &cohort_path,
        format!("{text}\n[extensions.\"dev.example.ext\"]\nnote = true\n"),
    )?;
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    let cohort_n = session
        .finalized()
        .graph()
        .cohorts()
        .values()
        .find(|cohort| cohort.id().as_str() == "cohort.n")
        .ok_or("cohort.n")?;
    assert!(
        cohort_n
            .assignments()
            .iter()
            .all(|assignment| assignment.repository().as_str().contains(":443")),
        "explicit-port repository must be retained"
    );

    // One-axis negative: an invalid authored extension namespace rejects.
    let repository = fixture()?;
    let provider_path = repository
        .path()
        .join("eqm/contracts/provider.mailgun.toml");
    let text = fs::read_to_string(&provider_path)?;
    fs::write(
        &provider_path,
        format!("{text}\n[extensions.\"dev.example\"]\nnote = true\n"),
    )?;
    assert!(
        prepare(
            &SessionRequest::new(None, evaluation_time()?),
            repository.path(),
        )
        .is_err()
    );

    // One-axis negative: a parser-invalid repository rejects.
    let repository = fixture()?;
    let cohort_path = repository.path().join("eqm/contracts/cohort.n.toml");
    let text = fs::read_to_string(&cohort_path)?.replace(
        "https://example.com/team/project",
        "https://Example.com/team/repo",
    );
    fs::write(&cohort_path, text)?;
    assert!(
        prepare(
            &SessionRequest::new(None, evaluation_time()?),
            repository.path(),
        )
        .is_err()
    );
    Ok(())
}

/// S043-R4: the repaired nested scenario-reference and source-root collection
/// contracts are exercised through the real authored loader and preparation
/// boundary, not the schema alone. Every negative mutates exactly one axis.
#[test]
fn prepared_fixture_admits_nested_subject_and_source_root_boundaries() -> Result<(), Box<dyn Error>>
{
    fn prepare_repository(repository: &tempfile::TempDir) -> Result<bool, Box<dyn Error>> {
        Ok(prepare(
            &SessionRequest::new(None, evaluation_time()?),
            repository.path(),
        )
        .is_ok())
    }

    fn mutate(
        repository: &tempfile::TempDir,
        relative: &str,
        from: &str,
        to: &str,
    ) -> Result<(), Box<dyn Error>> {
        let path = repository.path().join(relative);
        let text = fs::read_to_string(&path)?;
        fs::write(&path, text.replace(from, to))?;
        Ok(())
    }

    // Positive: an additional valid portable source root is retained and both
    // tagged subject alternatives are admitted by real preparation.
    let repository = fixture()?;
    mutate(
        &repository,
        "eqm.toml",
        "roots = [\"shared/kmp\"]",
        "roots = [\"shared/kmp\", \"shared/common\"]",
    )?;
    assert!(
        prepare_repository(&repository)?,
        "multi-root positive denied"
    );
    let session = prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository.path(),
    )?;
    let graph = session.finalized().graph();
    let kmp = graph
        .source_components()
        .values()
        .find(|component| component.id().as_str() == "shared.kmp")
        .ok_or("shared.kmp")?;
    assert_eq!(kmp.roots().len(), 2, "both portable roots must be retained");
    let signup = graph
        .scenarios()
        .values()
        .find(|scenario| scenario.id().as_str() == "signup.flow")
        .ok_or("signup.flow")?;
    assert!(
        signup
            .participants()
            .iter()
            .any(|participant| matches!(participant.subject(), ScenarioSubject::Target(_))),
        "target subject alternative must be admitted"
    );
    assert!(
        signup
            .participants()
            .iter()
            .any(|participant| matches!(participant.subject(), ScenarioSubject::Provider(_))),
        "provider subject alternative must be admitted"
    );

    // Negative: an empty source-root collection rejects.
    let repository = fixture()?;
    mutate(
        &repository,
        "eqm.toml",
        "roots = [\"shared/kmp\"]",
        "roots = []",
    )?;
    assert!(
        !prepare_repository(&repository)?,
        "empty roots were admitted"
    );

    // Negative: an exact duplicate source root rejects.
    let repository = fixture()?;
    mutate(
        &repository,
        "eqm.toml",
        "roots = [\"shared/kmp\"]",
        "roots = [\"shared/kmp\", \"shared/kmp\"]",
    )?;
    assert!(
        !prepare_repository(&repository)?,
        "duplicate roots were admitted"
    );

    // Negative: a non-portable source root rejects.
    let repository = fixture()?;
    mutate(
        &repository,
        "eqm.toml",
        "roots = [\"shared/kmp\"]",
        "roots = [\"../escape\"]",
    )?;
    assert!(
        !prepare_repository(&repository)?,
        "escaping root was admitted"
    );

    // Negative: an invalid target subject reference rejects.
    let repository = fixture()?;
    mutate(
        &repository,
        "eqm/contracts/scenario.signup.toml",
        "target = \"mobile\"",
        "target = \"Web\"",
    )?;
    assert!(
        !prepare_repository(&repository)?,
        "uppercase target subject was admitted"
    );

    // Negative: an invalid provider subject reference rejects.
    let repository = fixture()?;
    mutate(
        &repository,
        "eqm/contracts/scenario.signup.toml",
        "provider = \"provider.mailgun\"",
        "provider = \"provider\"",
    )?;
    assert!(
        !prepare_repository(&repository)?,
        "single-segment provider subject was admitted"
    );
    Ok(())
}
