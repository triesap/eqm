//! S041: graph-backed cohort resolution and independent candidate matching.

use eqm_application::{CohortError, match_cohort_subjects, resolve_cohort};
use eqm_domain::{
    Cohort, CohortAssignment, Extensions, FrameworkId, OccurrenceId, OwnerRef, PlatformId,
    Provider, ProviderId, RepoPath, Revision, RoleId, Scenario, ScenarioId, ScenarioParticipant,
    ScenarioSubject, SelectorText, Sha256Digest, SourceCommit, Target, TargetConfigurationDigest,
    TargetId, WorkspaceGraph, WorkspaceGraphInput,
};
use eqm_engine::{FinalizedWorkspaceGraph, FragmentDigestMap, expand_fragments};
use eqm_manifest::{canonicalize_provider_configuration_v2, canonicalize_target_configuration_v2};
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
        Vec::new(),
        Extensions::default(),
    )?)
}

fn assignment(occurrence: &str, marker: u8) -> Result<CohortAssignment, Box<dyn Error>> {
    let (target_configuration, provider_configuration) = if occurrence == "mailer" {
        (
            None,
            Some(
                canonicalize_provider_configuration_v2(&provider()?)?
                    .digest()
                    .as_digest(),
            ),
        )
    } else {
        (
            Some(
                canonicalize_target_configuration_v2(&target()?)?
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
        Some(Sha256Digest::hash_content(&[marker])),
        target_configuration,
        provider_configuration,
        Some(SelectorText::new(format!("build.{marker}"))?),
        Some(Sha256Digest::hash_content(&[marker, marker])),
    )?)
}

fn cohort() -> Result<Cohort, Box<dyn Error>> {
    Ok(Cohort::new(
        SelectorText::new("cohort.signup")?,
        Revision::new(1)?,
        ScenarioId::new("signup.flow")?,
        vec![assignment("phone", 1)?, assignment("mailer", 2)?],
        Extensions::default(),
    )?)
}

fn graph(cohort: &Cohort, revisions: &[u64]) -> Result<FinalizedWorkspaceGraph, Box<dyn Error>> {
    finalize(WorkspaceGraph::new(WorkspaceGraphInput {
        targets: vec![target()?],
        providers: vec![provider()?],
        scenarios: revisions
            .iter()
            .map(|revision| scenario(*revision))
            .collect::<Result<Vec<_>, _>>()?,
        cohorts: vec![cohort.clone()],
        ..WorkspaceGraphInput::default()
    })?)
}

fn candidate(
    target_configuration: &str,
    provider_configuration: &str,
) -> Result<CheckedSubjectManifestV2, Box<dyn Error>> {
    let wire = json!({
        "schema": eqm_protocol::SUBJECT_MANIFEST_SCHEMA_V2.to_string(),
        "id": "subject.signup",
        "revision": 1,
        "entries": [
            {
                "occurrence": "phone",
                "repository": "https://example.com/team/project",
                "source_commit": "a".repeat(40),
                "source_content_digest": Sha256Digest::hash_content(&[1]).to_string(),
                "target_configuration_digest": target_configuration,
                "build_id": "build.1",
                "artifact_digest": Sha256Digest::hash_content(&[1, 1]).to_string()
            },
            {
                "occurrence": "mailer",
                "repository": "https://example.com/team/project",
                "source_commit": "a".repeat(40),
                "source_content_digest": Sha256Digest::hash_content(&[2]).to_string(),
                "provider_configuration_digest": provider_configuration,
                "build_id": "build.2",
                "artifact_digest": Sha256Digest::hash_content(&[2, 2]).to_string()
            }
        ]
    });
    Ok(CheckedSubjectManifestV2::from_bytes(&serde_json::to_vec(
        &wire,
    )?)?)
}

#[test]
fn resolved_cohort_matches_only_its_independent_combination() -> Result<(), Box<dyn Error>> {
    let cohort = cohort()?;
    let graph = graph(&cohort, &[1])?;
    let resolved = resolve_cohort(&graph, &cohort)?;
    assert_eq!(resolved.occurrences(), 2);
    assert_eq!(resolved.cohort_id().as_str(), "cohort.signup");
    assert_eq!(resolved.scenario_id().as_str(), "signup.flow");
    assert_eq!(resolved.scenario_revision().get(), 1);

    let target_configuration = canonicalize_target_configuration_v2(&target()?)?
        .digest()
        .to_string();
    let provider_configuration = canonicalize_provider_configuration_v2(&provider()?)?
        .digest()
        .to_string();
    let checked = candidate(&target_configuration, &provider_configuration)?;
    let matched = match_cohort_subjects(&checked, &resolved)?;
    assert_eq!(matched.assignments().len(), 2);

    // An undeclared combination (different build/artifact) is rejected.
    let mut wire = serde_json::to_value(checked.record())?;
    wire["entries"][0]["build_id"] = json!("build.other");
    let altered = CheckedSubjectManifestV2::from_bytes(&serde_json::to_vec(&wire)?)?;
    assert!(matches!(
        match_cohort_subjects(&altered, &resolved),
        Err(SubjectMatchError::BuildMismatch(_))
    ));

    // An absent participant is rejected.
    let mut absent = serde_json::to_value(checked.record())?;
    absent["entries"].as_array_mut().ok_or("entries")?.pop();
    let absent = CheckedSubjectManifestV2::from_bytes(&serde_json::to_vec(&absent)?)?;
    assert!(matches!(
        match_cohort_subjects(&absent, &resolved),
        Err(SubjectMatchError::MissingOccurrence(_))
    ));
    Ok(())
}

#[test]
fn ambiguous_scenario_revision_and_occurrence_coverage_reject() -> Result<(), Box<dyn Error>> {
    // Two scenario revisions sharing one ID cannot be resolved without choosing
    // a "latest"; the cohort is rejected instead.
    let ambiguous = cohort()?;
    assert_eq!(
        resolve_cohort(&graph(&ambiguous, &[1, 2])?, &ambiguous).map(|_| ()),
        Err(CohortError::AmbiguousScenario)
    );

    // A cohort assignment for an occurrence the scenario lacks.
    let extra = Cohort::new(
        SelectorText::new("cohort.signup")?,
        Revision::new(1)?,
        ScenarioId::new("signup.flow")?,
        vec![
            assignment("phone", 1)?,
            assignment("mailer", 2)?,
            assignment("absent", 3)?,
        ],
        Extensions::default(),
    )?;
    assert!(matches!(
        resolve_cohort(&graph(&extra, &[1])?, &extra),
        Err(CohortError::UndeclaredOccurrence(_))
    ));

    // A scenario occurrence with no cohort assignment.
    let missing = Cohort::new(
        SelectorText::new("cohort.signup")?,
        Revision::new(1)?,
        ScenarioId::new("signup.flow")?,
        vec![assignment("phone", 1)?],
        Extensions::default(),
    )?;
    assert!(matches!(
        resolve_cohort(&graph(&missing, &[1])?, &missing),
        Err(CohortError::MissingOccurrence(_))
    ));

    // No matching scenario definition at all.
    let absent = Cohort::new(
        SelectorText::new("cohort.signup")?,
        Revision::new(1)?,
        ScenarioId::new("absent.flow")?,
        vec![assignment("phone", 1)?],
        Extensions::default(),
    )?;
    assert_eq!(
        resolve_cohort(&graph(&absent, &[1])?, &absent).map(|_| ()),
        Err(CohortError::ScenarioNotFound)
    );
    Ok(())
}

fn assignment_claim(
    occurrence: &str,
    target: Option<Sha256Digest>,
    provider: Option<Sha256Digest>,
    marker: u8,
) -> Result<CohortAssignment, Box<dyn Error>> {
    Ok(CohortAssignment::new(
        OccurrenceId::new(occurrence)?,
        "https://example.com/team/project".parse()?,
        "a".repeat(40).parse::<SourceCommit>()?,
        Some(Sha256Digest::hash_content(&[marker])),
        target,
        provider,
        Some(SelectorText::new(format!("build.{marker}"))?),
        Some(Sha256Digest::hash_content(&[marker, marker])),
    )?)
}

#[test]
fn configuration_kind_mismatch_and_altered_authority_reject() -> Result<(), Box<dyn Error>> {
    let provider_digest = canonicalize_provider_configuration_v2(&provider()?)?
        .digest()
        .as_digest();

    // A phone target occurrence that claims provider configuration is wrong-kind.
    let wrong_kind = Cohort::new(
        SelectorText::new("cohort.signup")?,
        Revision::new(1)?,
        ScenarioId::new("signup.flow")?,
        vec![
            assignment_claim("phone", None, Some(provider_digest), 1)?,
            assignment_claim("mailer", None, Some(provider_digest), 2)?,
        ],
        Extensions::default(),
    )?;
    assert!(matches!(
        resolve_cohort(&graph(&wrong_kind, &[1])?, &wrong_kind),
        Err(CohortError::ConfigurationKind(_))
    ));

    // A phone target occurrence whose target claim differs from the graph
    // projection is rejected rather than substituted.
    let wrong_digest = TargetConfigurationDigest::hash_content(b"other").as_digest();
    let mismatch = Cohort::new(
        SelectorText::new("cohort.signup")?,
        Revision::new(1)?,
        ScenarioId::new("signup.flow")?,
        vec![
            assignment_claim("phone", Some(wrong_digest), None, 1)?,
            assignment_claim("mailer", None, Some(provider_digest), 2)?,
        ],
        Extensions::default(),
    )?;
    assert!(matches!(
        resolve_cohort(&graph(&mismatch, &[1])?, &mismatch),
        Err(CohortError::ConfigurationMismatch(_))
    ));

    // An altered caller-created cohort with the identical id/revision is not the
    // graph-owned value and is rejected.
    let original = cohort()?;
    let graph = graph(&original, &[1])?;
    let altered = Cohort::new(
        SelectorText::new("cohort.signup")?,
        Revision::new(1)?,
        ScenarioId::new("signup.flow")?,
        vec![assignment("mailer", 7)?, assignment("phone", 8)?],
        Extensions::default(),
    )?;
    assert_eq!(
        resolve_cohort(&graph, &altered),
        Err(CohortError::NotInGraph)
    );
    // The exact graph-owned cohort still resolves.
    assert!(resolve_cohort(&graph, &original).is_ok());
    Ok(())
}

#[test]
fn incomplete_and_invalid_configuration_assignments_reject() -> Result<(), Box<dyn Error>> {
    let target_digest = canonicalize_target_configuration_v2(&target()?)?
        .digest()
        .as_digest();
    let provider_digest = canonicalize_provider_configuration_v2(&provider()?)?
        .digest()
        .as_digest();
    let occurrence = OccurrenceId::new("phone")?;
    let repository =
        "https://example.com/team/project".parse::<eqm_domain::RepositoryIdentity>()?;
    let commit = "a".repeat(40).parse::<SourceCommit>()?;
    let content = Some(Sha256Digest::hash_content(&[1]));
    let artifact = Some(Sha256Digest::hash_content(&[2]));
    let build = Some(SelectorText::new("build.incomplete")?);

    // Neither configuration claim is present.
    assert!(matches!(
        CohortAssignment::new(
            occurrence.clone(),
            repository.clone(),
            commit.clone(),
            content,
            None,
            None,
            build.clone(),
            artifact
        ),
        Err(eqm_domain::CohortError::ConfigurationAuthority)
    ));

    // Both configuration claims are present.
    assert!(matches!(
        CohortAssignment::new(
            occurrence.clone(),
            repository.clone(),
            commit.clone(),
            content,
            Some(target_digest),
            Some(provider_digest),
            build.clone(),
            artifact
        ),
        Err(eqm_domain::CohortError::ConfigurationAuthority)
    ));

    // A valid claim with an absent build axis is incomplete at resolution.
    let incomplete = Cohort::new(
        SelectorText::new("cohort.signup")?,
        Revision::new(1)?,
        ScenarioId::new("signup.flow")?,
        vec![
            CohortAssignment::new(
                occurrence,
                repository,
                commit,
                content,
                Some(target_digest),
                None,
                None,
                artifact,
            )?,
            assignment("mailer", 2)?,
        ],
        Extensions::default(),
    )?;
    assert!(matches!(
        resolve_cohort(&graph(&incomplete, &[1])?, &incomplete),
        Err(CohortError::IncompleteOccurrence(_))
    ));
    Ok(())
}
