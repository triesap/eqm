//! S042: synthetic asymmetric CLI/desktop/mobile/provider fixture.
//!
//! Exercises shared/transitive sources, repeated role-bearing actors, distinct
//! independently released build combinations and explicit N/N+1 cohorts through
//! real load, resolve, prepare, projection and checked subject matching. The
//! fixture is intentionally synthetic: it invents no real product build task and
//! its intentional exclusions are not waivers.

use eqm_application::{
    ComparisonContextError, SessionRequest, evaluate_checked_comparison, match_cohort_subjects,
    prepare, resolve_cohort, shared_comparison_context,
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
