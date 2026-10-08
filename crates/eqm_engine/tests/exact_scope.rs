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

// S061 pure grouped matching controls. Scope/configuration/binding/specification
// come from the real checked graph. Synthetic assertion/context identities are
// explicit engine input data, not wire canonicalization or verified provenance;
// the application owning controls exercise actual bytes and S054 identities.
struct CoverageFixture {
    expected: eqm_engine::CompleteCoverageExpectation,
    result: eqm_domain::EvidenceResult,
    identity: eqm_domain::SemanticEvidenceDigest,
    lineage: eqm_domain::EvidenceLineage,
    provenance: eqm_domain::SelectorText,
}

impl CoverageFixture {
    fn candidate(&self) -> eqm_engine::CompleteEvidenceCandidate<'_> {
        eqm_engine::CompleteEvidenceCandidate {
            delivery: eqm_engine::EvidenceNormalizationCandidate {
                result: &self.result,
                lineage: &self.lineage,
                semantic_identity: self.identity,
                raw_identity: eqm_domain::RawObjectDigest::hash_content(
                    b"synthetic exact original",
                ),
                evidence_spec_id: &self.expected.structural.evidence_spec_id,
                provenance: &self.provenance,
            },
            coordinate: Some(&self.expected.coordinate),
            identity: Some(self.expected.identity),
            subjects: self.expected.subjects.as_ref(),
        }
    }
}

fn coverage_fixture(
    graph: &FinalizedWorkspaceGraph,
    key: eqm_engine::ObligationKey,
) -> Result<CoverageFixture, Box<dyn Error>> {
    use eqm_domain::*;
    let target = TargetId::new("android")?;
    let binding = graph
        .graph()
        .bindings()
        .values()
        .find(|binding| binding.target() == &target && binding.unit() == &key.unit)
        .ok_or("owning binding missing")?;
    let specification = binding
        .evidence()
        .values()
        .find(|spec| {
            spec.kind() == EvidenceKind::Test
                && spec.requirements().contains(&key.requirement)
                && spec.facets().contains(&key.facet)
        })
        .ok_or("owning test specification missing")?;
    let participants = match &key.subject {
        ScopeSubject::Scenario(scope) => scope.participants().clone(),
        ScopeSubject::CheckedProvider(scope) => {
            let mut participants = BTreeSet::new();
            for target in scope
                .dependers()
                .iter()
                .filter(|target| key.subject.participates(target))
            {
                participants.insert(ScenarioParticipant::new(
                    OccurrenceId::new(format!("fixture_{}", target.as_str()))?,
                    RoleId::new("edge")?,
                    ScenarioSubject::Target(target.clone()),
                ));
            }
            participants.insert(ScenarioParticipant::new(
                OccurrenceId::new("fixture_provider")?,
                RoleId::new("service")?,
                ScenarioSubject::Provider(scope.provider().clone()),
            ));
            participants
        }
        _ => return Err("checked grouped scope required".into()),
    };
    let repository: RepositoryIdentity = "https://example.com/fixture/group".parse()?;
    let commit: SourceCommit = "a".repeat(40).parse()?;
    let build = SelectorText::new("fixture-build")?;
    let artifact = Sha256Digest::hash_content(b"synthetic built artifact");
    let configuration = eqm_manifest::canonicalize_target_configuration_v2(
        graph
            .graph()
            .targets()
            .get(&target)
            .ok_or("target missing")?,
    )?
    .digest();
    let mut assignments = Vec::new();
    for participant in &participants {
        let (target_configuration, provider_configuration) = match participant.subject() {
            ScenarioSubject::Target(target) => (
                Some(
                    eqm_manifest::canonicalize_target_configuration_v2(
                        graph
                            .graph()
                            .targets()
                            .get(target)
                            .ok_or("participant target missing")?,
                    )?
                    .digest(),
                ),
                None,
            ),
            ScenarioSubject::Provider(provider) => (
                None,
                Some(
                    eqm_manifest::canonicalize_provider_configuration_v2(
                        graph
                            .graph()
                            .providers()
                            .get(provider)
                            .ok_or("participant provider missing")?,
                    )?
                    .digest(),
                ),
            ),
        };
        assignments.push(SubjectAssignment::new(
            participant.occurrence().clone(),
            repository.clone(),
            commit.clone(),
            Some(Sha256Digest::hash_content(b"synthetic source content")),
            target_configuration,
            provider_configuration,
            Some(build.clone()),
            Some(artifact),
        )?);
    }
    let subjects = eqm_engine::CoverageSubjectManifest {
        participants,
        subjects: SubjectManifest::new(
            SelectorText::new("fixture_subjects")?,
            Revision::new(1)?,
            assignments,
        )?,
    };
    let scope = match &key.subject {
        ScopeSubject::Scenario(scope) => {
            EvidenceScopeSubject::TargetSet(scope.selected_targets().clone())
        }
        ScopeSubject::CheckedProvider(scope) => {
            EvidenceScopeSubject::Provider(scope.provider().clone())
        }
        _ => return Err("checked group missing".into()),
    };
    let subject = EvidenceSubject::new(
        repository,
        Sha256Digest::hash_content(b"synthetic repository identity"),
        scope,
        commit,
        Some(build),
        Some(artifact),
        configuration.as_digest(),
    );
    let lineage = EvidenceLineage::from_parts(EvidenceLineageParts {
        plan: Some(PlanId::new("fixture.plan")?),
        campaign: Some(CampaignId::new("fixture.campaign")?),
        run: Some(RunId::new("fixture.run")?),
        logical_case: Some(LogicalCaseId::new("fixture.case")?),
        attempt: Some(AttemptId::new("fixture.attempt")?),
    })?;
    let selection = ExpectedSelectionManifest::new(
        ExpectedSelectionSource {
            binding: binding.id().clone(),
            binding_revision: binding.revision(),
            binding_digest: eqm_manifest::canonicalize_binding_v2(binding)?.digest(),
            specification: specification.id().clone(),
            specification_digest: eqm_manifest::canonicalize_evidence_spec_v2(specification)?
                .digest(),
            plan: lineage.plan().clone(),
            campaign: lineage.campaign().clone(),
        },
        EvidenceKind::Test,
        SelectionCountUnit::LogicalTestCase,
        vec![lineage.logical_case().clone()],
        vec![],
        false,
    )?;
    let profiles = key
        .profiles
        .iter()
        .map(|(id, selected)| {
            let values = selected
                .values()
                .iter()
                .map(|(dimension, value)| {
                    Ok((
                        dimension.clone(),
                        value.clone().ok_or("unknown fixture profile")?,
                    ))
                })
                .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
            Ok((
                id.clone(),
                ProfileSelection::new(id.clone(), selected.revision(), values)?,
            ))
        })
        .collect::<Result<BTreeMap<_, _>, Box<dyn Error>>>()?;
    let structural = eqm_engine::CoverageExpectation {
        evidence_spec_id: specification.id().clone(),
        evidence_spec_digest: selection.source().specification_digest.as_digest(),
        requirement: key.requirement.clone(),
        facet: key.facet,
        subject,
        target,
        unit: key.unit.clone(),
        kind: EvidenceKind::Test,
        contract_digest: eqm_manifest::canonicalize_contract_v2(graph.graph())?
            .digest()
            .as_digest(),
        binding_digest: selection.source().binding_digest.as_digest(),
        policy_digest: Sha256Digest::hash_content(b"synthetic selected policy"),
        runner_digest: None,
        adapter_digest: None,
        runtime_facts_digest: None,
        profiles,
        release_record_digest: key.release_context,
    };
    let expected = eqm_engine::CompleteCoverageExpectation {
        structural,
        coordinate: key,
        identity: ObligationDigest::hash_content(b"synthetic pure context identity"),
        selection,
        subjects: Some(subjects),
    };
    let identity = SemanticEvidenceDigest::hash_content(b"synthetic pure passed assertion");
    let result = coverage_assertion(&expected, identity, AttemptOutcome::Passed)?;
    Ok(CoverageFixture {
        expected,
        result,
        identity,
        lineage,
        provenance: SelectorText::new("fixture.receipt")?,
    })
}

fn coverage_assertion(
    expected: &eqm_engine::CompleteCoverageExpectation,
    identity: eqm_domain::SemanticEvidenceDigest,
    outcome: eqm_domain::AttemptOutcome,
) -> Result<eqm_domain::EvidenceResult, Box<dyn Error>> {
    use eqm_domain::*;
    let base = &expected.structural;
    let start: UtcInstant = "2026-01-02T03:00:00Z".parse()?;
    let finish: UtcInstant = "2026-01-02T03:00:01Z".parse()?;
    let counts = EvidenceCounts::new(
        1,
        u64::from(outcome == AttemptOutcome::Passed),
        u64::from(outcome == AttemptOutcome::Failed),
        0,
        0,
        0,
    )?;
    let execution = ExecutionPayload::new(
        vec![EvidenceAttempt::new(
            PositiveCount::new(1)?,
            outcome,
            start,
            finish,
            None,
        )?],
        counts,
        start,
        finish,
    )?;
    Ok(EvidenceResult::new(
        identity.as_digest(),
        base.subject.clone(),
        base.target.clone(),
        base.unit.clone(),
        vec![base.requirement.clone()],
        vec![base.facet],
        base.kind,
        base.evidence_spec_digest,
        base.contract_digest,
        base.binding_digest,
        base.policy_digest,
        base.runner_digest,
        base.adapter_digest,
        base.runtime_facts_digest,
        base.release_record_digest,
        base.profiles.values().cloned().collect(),
        "producer://local/fixture/grouped".parse()?,
        TrustLevel::UntrustedLocal,
        finish,
        EvidencePayload::Test(execution),
        vec![],
        identity.as_digest(),
        Extensions::default(),
    )?)
}

fn checked_coverage_fixture(
    provider: bool,
) -> Result<(tempfile::TempDir, CoverageFixture), Box<dyn Error>> {
    let (root, fixture, _) = checked_count_coverage_fixture(provider)?;
    Ok((root, fixture))
}

fn checked_count_coverage_fixture(
    provider: bool,
) -> Result<(tempfile::TempDir, CoverageFixture, RequiredUniverse), Box<dyn Error>> {
    let (root, loaded) = support::loaded_example_with(|root| {
        // The example intentionally has no OTP binding. Declare an explicit
        // synthetic mapping before ingestion; never reuse the identifier mapping
        // implicitly for a different requirement.
        let binding = fs::read_to_string(root.join("eqm/bindings/android.auth_signup.toml"))?
            .replace("binding.android.auth_signup", "binding.android.fixture_otp")
            .replace(
                "account.create.signup.identifier#email_default",
                REQUIREMENT,
            )
            .replace(
                "account.create.signup.identifier",
                "account.create.signup.otp",
            );
        fs::write(root.join("eqm/bindings/android.fixture_otp.toml"), binding)?;
        if provider {
            provider_workspace(root, true, false)?;
            let path = root.join("eqm/contracts/auth.signup.otp.toml");
            fs::write(
                &path,
                fs::read_to_string(&path)?.replace(
                    "scope = \"end_to_end\"",
                    "scope = \"shared_provider\"\nprovider = \"provider.required\"",
                ),
            )?;
        }
        Ok(())
    })?;
    let graph = finalize(&loaded)?;
    let map = if provider {
        RequirementScenarioBindings::new(vec![])?
    } else {
        bindings(REQUIREMENT, SCENARIO, 1)?
    };
    let required = universe(&graph, &map, Some("us"))?;
    let key = required
        .coordinates()
        .iter()
        .find(|key| {
            key.requirement.as_str() == REQUIREMENT
                && key.facet == eqm_domain::Facet::Behavior
                && matches!(
                    &key.subject,
                    ScopeSubject::Scenario(_) | ScopeSubject::CheckedProvider(_)
                )
        })
        .ok_or("checked grouped behavior missing")?
        .clone();
    Ok((root, coverage_fixture(&graph, key)?, required))
}

#[test]
fn complete_grouped_matching_preserves_roles_subjects_and_rejected_relevance()
-> Result<(), Box<dyn Error>> {
    use eqm_domain::{RoleId, ScenarioParticipant};
    use eqm_engine::{
        CompleteCoverageError, CompleteCoverageMismatch, evaluate_complete_evidence_coverage,
    };
    for provider in [false, true] {
        let (_root, fixture) = checked_coverage_fixture(provider)?;
        let candidate = fixture.candidate();
        let accepted =
            evaluate_complete_evidence_coverage(&fixture.expected, &[candidate, candidate])?;
        assert_eq!(
            accepted.status(),
            CoverageStatus::Covered,
            "provider={provider}"
        );
        assert_eq!(
            accepted.eligible(),
            &BTreeSet::from([(fixture.identity, fixture.lineage.clone())])
        );
        assert!(accepted.rejected().is_empty());
        assert!(accepted.conflicts().is_empty());
        let mut wrong_roles = fixture
            .expected
            .subjects
            .clone()
            .ok_or("group subjects absent")?;
        let actor = wrong_roles
            .participants
            .iter()
            .next()
            .ok_or("participant absent")?
            .clone();
        wrong_roles.participants.remove(&actor);
        wrong_roles.participants.insert(ScenarioParticipant::new(
            actor.occurrence().clone(),
            RoleId::new("substituted_role")?,
            actor.subject().clone(),
        ));
        let mut missing_actor = fixture
            .expected
            .subjects
            .clone()
            .ok_or("group subjects absent")?;
        missing_actor.participants.remove(&actor);
        for claim in [
            eqm_engine::CompleteEvidenceCandidate {
                subjects: None,
                ..candidate
            },
            eqm_engine::CompleteEvidenceCandidate {
                subjects: Some(&wrong_roles),
                ..candidate
            },
            eqm_engine::CompleteEvidenceCandidate {
                subjects: Some(&missing_actor),
                ..candidate
            },
        ] {
            let rejected = evaluate_complete_evidence_coverage(&fixture.expected, &[claim, claim])?;
            assert_eq!(rejected.status(), CoverageStatus::Missing);
            assert!(rejected.eligible().is_empty());
            assert!(rejected.conflicts().is_empty());
            assert_eq!(
                rejected.rejected()[&fixture.result.id()],
                BTreeSet::from([CompleteCoverageMismatch::SubjectManifest])
            );
            let mixed =
                evaluate_complete_evidence_coverage(&fixture.expected, &[candidate, claim, claim])?;
            assert_eq!(mixed.status(), CoverageStatus::Covered);
            assert_eq!(mixed.eligible(), accepted.eligible());
            assert_eq!(
                evaluate_complete_evidence_coverage(&fixture.expected, &[claim, candidate, claim])?,
                mixed
            );
        }
        let mut incomplete = fixture.expected.clone();
        incomplete.subjects = None;
        assert_eq!(
            evaluate_complete_evidence_coverage(&incomplete, &[]),
            Err(CompleteCoverageError::InvalidExpectation)
        );
        let mut inconsistent = fixture.expected.clone();
        inconsistent.subjects = Some(missing_actor);
        assert_eq!(
            evaluate_complete_evidence_coverage(&inconsistent, &[]),
            Err(CompleteCoverageError::InvalidExpectation)
        );
    }
    Ok(())
}

#[test]
fn complete_grouped_matching_rejects_each_subject_assignment_substitution()
-> Result<(), Box<dyn Error>> {
    use eqm_domain::*;
    use eqm_engine::{
        CompleteCoverageError, CompleteCoverageMismatch, evaluate_complete_evidence_coverage,
    };
    for provider in [false, true] {
        let (_root, fixture) = checked_coverage_fixture(provider)?;
        let original = fixture.expected.subjects.as_ref().ok_or("group manifest")?;
        let base = fixture.candidate();
        for axis in [
            "repository",
            "commit",
            "content",
            "configuration",
            "configuration_kind",
            "build",
            "artifact",
            "occurrence",
            "missing_content",
            "missing_build",
            "missing_artifact",
            "manifest_id",
            "manifest_revision",
        ] {
            let mut assignments = original.subjects.assignments().to_vec();
            let first = &assignments[0];
            let (target_configuration, provider_configuration) = if axis == "configuration_kind" {
                if first.target_configuration().is_some() {
                    (
                        None,
                        Some(ProviderConfigurationDigest::hash_content(b"wrong kind")),
                    )
                } else {
                    (
                        Some(TargetConfigurationDigest::hash_content(b"wrong kind")),
                        None,
                    )
                }
            } else if axis == "configuration" {
                if first.target_configuration().is_some() {
                    (
                        Some(TargetConfigurationDigest::hash_content(
                            b"other configuration",
                        )),
                        None,
                    )
                } else {
                    (
                        None,
                        Some(ProviderConfigurationDigest::hash_content(
                            b"other configuration",
                        )),
                    )
                }
            } else {
                (first.target_configuration(), first.provider_configuration())
            };
            assignments[0] = SubjectAssignment::new(
                if axis == "occurrence" {
                    OccurrenceId::new("foreign_actor")?
                } else {
                    first.occurrence().clone()
                },
                if axis == "repository" {
                    "https://example.com/other".parse()?
                } else {
                    first.repository().clone()
                },
                if axis == "commit" {
                    "b".repeat(40).parse()?
                } else {
                    first.source_commit().clone()
                },
                if axis == "missing_content" {
                    None
                } else if axis == "content" {
                    Some(Sha256Digest::hash_content(b"other source"))
                } else {
                    first.source_content()
                },
                target_configuration,
                provider_configuration,
                if axis == "missing_build" {
                    None
                } else if axis == "build" {
                    Some(SelectorText::new("other-build")?)
                } else {
                    first.build_id().cloned()
                },
                if axis == "missing_artifact" {
                    None
                } else if axis == "artifact" {
                    Some(Sha256Digest::hash_content(b"other artifact"))
                } else {
                    first.artifact_digest()
                },
            )?;
            let changed = eqm_engine::CoverageSubjectManifest {
                participants: original.participants.clone(),
                subjects: SubjectManifest::new(
                    if axis == "manifest_id" {
                        SelectorText::new("other_subjects")?
                    } else {
                        original.subjects.id().clone()
                    },
                    if axis == "manifest_revision" {
                        Revision::new(2)?
                    } else {
                        original.subjects.revision()
                    },
                    assignments,
                )?,
            };
            let rejected = eqm_engine::CompleteEvidenceCandidate {
                subjects: Some(&changed),
                ..base
            };
            let report = evaluate_complete_evidence_coverage(&fixture.expected, &[rejected])?;
            assert_eq!(
                report.status(),
                CoverageStatus::Missing,
                "{provider}/{axis}"
            );
            assert!(report.eligible().is_empty());
            assert!(report.conflicts().is_empty());
            assert_eq!(
                report.rejected()[&fixture.result.id()],
                BTreeSet::from([CompleteCoverageMismatch::SubjectManifest])
            );
            let mixed = evaluate_complete_evidence_coverage(
                &fixture.expected,
                &[rejected, base, rejected],
            )?;
            assert_eq!(mixed.status(), CoverageStatus::Covered);
            assert_eq!(
                mixed.eligible(),
                &BTreeSet::from([(fixture.identity, fixture.lineage.clone())])
            );
            assert_eq!(
                evaluate_complete_evidence_coverage(&fixture.expected, &[base, rejected])?,
                mixed
            );
            if matches!(
                axis,
                "configuration_kind"
                    | "occurrence"
                    | "missing_content"
                    | "missing_build"
                    | "missing_artifact"
            ) {
                let mut invalid_expected = fixture.expected.clone();
                invalid_expected.subjects = Some(changed);
                assert_eq!(
                    evaluate_complete_evidence_coverage(&invalid_expected, &[]),
                    Err(CompleteCoverageError::InvalidExpectation),
                    "{provider}/{axis}"
                );
            }
        }
        let mut inconsistent = fixture.expected.clone();
        inconsistent.structural.subject = EvidenceSubject::new(
            inconsistent.structural.subject.repository().clone(),
            inconsistent.structural.subject.repository_id_digest(),
            inconsistent.structural.subject.scope().clone(),
            inconsistent.structural.subject.source_commit().clone(),
            Some(SelectorText::new("unassociated-build")?),
            inconsistent.structural.subject.artifact_digest(),
            inconsistent
                .structural
                .subject
                .target_configuration_digest(),
        );
        assert_eq!(
            evaluate_complete_evidence_coverage(&inconsistent, &[]),
            Err(CompleteCoverageError::InvalidExpectation)
        );
        let mut flattened = fixture.expected.clone();
        flattened.coordinate.subject = match &flattened.coordinate.subject {
            ScopeSubject::Scenario(scope) => {
                ScopeSubject::TargetSet(scope.selected_targets().clone())
            }
            ScopeSubject::CheckedProvider(scope) => {
                ScopeSubject::Provider(scope.provider().clone())
            }
            _ => return Err("checked group".into()),
        };
        assert_eq!(
            evaluate_complete_evidence_coverage(&flattened, &[]),
            Err(CompleteCoverageError::InvalidExpectation)
        );
    }
    Ok(())
}

#[test]
fn complete_matching_enforces_delivery_and_context_work_boundaries() -> Result<(), Box<dyn Error>> {
    use eqm_domain::*;
    use eqm_engine::{
        CandidateNormalizationError, CompleteCoverageError, CompleteCoverageMismatch,
        evaluate_complete_evidence_coverage,
    };
    let (_root, mut fixture) = checked_coverage_fixture(false)?;
    fixture.expected.coordinate.subject = ScopeSubject::Target(TargetId::new("android")?);
    fixture.expected.subjects = None;
    let old = &fixture.expected.structural.subject;
    fixture.expected.structural.subject = EvidenceSubject::new(
        old.repository().clone(),
        old.repository_id_digest(),
        EvidenceScopeSubject::Target(TargetId::new("android")?),
        old.source_commit().clone(),
        old.build_id().cloned(),
        old.artifact_digest(),
        old.target_configuration_digest(),
    );
    fixture.result =
        coverage_assertion(&fixture.expected, fixture.identity, AttemptOutcome::Passed)?;
    let base = fixture.candidate();
    assert_eq!(
        evaluate_complete_evidence_coverage(&fixture.expected, &vec![base; 10_000])?.status(),
        CoverageStatus::Covered
    );
    assert_eq!(
        evaluate_complete_evidence_coverage(&fixture.expected, &vec![base; 10_001]),
        Err(CompleteCoverageError::Normalization(
            CandidateNormalizationError::LimitExceeded
        ))
    );
    // Literal boundary witness: 9,982 selected cases, one profile dimension,
    // one expected target and 64 foreign targets across 10,000 deliveries.
    // This consumes exactly the declared 1,000,000 context-work slots.
    let generated = (0..9981)
        .map(|i| LogicalCaseId::new(format!("case.c{i}")))
        .collect::<Result<Vec<_>, _>>()?;
    let cases = std::iter::once(fixture.lineage.logical_case().clone())
        .chain(generated)
        .collect();
    fixture.expected.selection = ExpectedSelectionManifest::new(
        fixture.expected.selection.source().clone(),
        EvidenceKind::Test,
        SelectionCountUnit::LogicalTestCase,
        cases,
        vec![],
        false,
    )?;
    let mut foreign = fixture.expected.coordinate.clone();
    foreign.subject = ScopeSubject::TargetSet(
        (0..64)
            .map(|i| TargetId::new(format!("target_{i}")))
            .collect::<Result<BTreeSet<_>, _>>()?,
    );
    let candidate = eqm_engine::CompleteEvidenceCandidate {
        coordinate: Some(&foreign),
        ..fixture.candidate()
    };
    let report = evaluate_complete_evidence_coverage(&fixture.expected, &vec![candidate; 10_000])?;
    assert_eq!(report.status(), CoverageStatus::Missing);
    assert!(report.eligible().is_empty());
    assert_eq!(
        report.rejected()[&fixture.result.id()],
        BTreeSet::from([CompleteCoverageMismatch::Coordinate])
    );
    let observations = eqm_engine::collect_logical_observations(
        std::slice::from_ref(&fixture.expected),
        &vec![candidate; 10_000],
    )?;
    assert_eq!(observations.status(), CoverageStatus::Missing);
    assert!(observations.observed().is_empty());
    assert_eq!(
        observations.missing()[fixture.expected.selection.source()],
        *fixture.expected.selection.selected()
    );
    assert_eq!(
        eqm_engine::collect_logical_observations(
            &[fixture.expected.clone(), fixture.expected.clone()],
            &vec![candidate; 10_000]
        ),
        Err(eqm_engine::LogicalObservationError::Matching(
            CompleteCoverageError::WorkLimitExceeded
        ))
    );

    foreign.subject = ScopeSubject::TargetSet(
        (0..65)
            .map(|i| TargetId::new(format!("target_{i}")))
            .collect::<Result<BTreeSet<_>, _>>()?,
    );
    let candidate = eqm_engine::CompleteEvidenceCandidate {
        coordinate: Some(&foreign),
        ..fixture.candidate()
    };
    assert_eq!(
        evaluate_complete_evidence_coverage(&fixture.expected, &vec![candidate; 10_000]),
        Err(CompleteCoverageError::WorkLimitExceeded)
    );
    Ok(())
}

#[test]
fn logical_observation_expectations_and_frozen_alternatives_are_checked()
-> Result<(), Box<dyn Error>> {
    use eqm_domain::*;
    use eqm_engine::{LogicalObservationError, collect_logical_observations};
    let (_root, fixture) = checked_coverage_fixture(false)?;
    let base = fixture.candidate();
    assert_eq!(
        collect_logical_observations(&[], &[base]),
        Err(LogicalObservationError::EmptyExpectations)
    );
    assert_eq!(
        collect_logical_observations(
            &[fixture.expected.clone(), fixture.expected.clone()],
            &[base]
        ),
        Err(LogicalObservationError::DuplicateSource)
    );
    let mut foreign = fixture.expected.clone();
    foreign.identity = ObligationDigest::hash_content(b"other context");
    assert_eq!(
        collect_logical_observations(&[fixture.expected.clone(), foreign], &[base]),
        Err(LogicalObservationError::Context)
    );
    let mut foreign = fixture.expected.clone();
    foreign.structural.policy_digest = Sha256Digest::hash_content(b"other policy");
    assert_eq!(
        collect_logical_observations(&[fixture.expected.clone(), foreign], &[]),
        Err(LogicalObservationError::Context)
    );
    let mut mixed = fixture.expected.clone();
    mixed.selection = ExpectedSelectionManifest::new(
        mixed.selection.source().clone(),
        EvidenceKind::StaticInventory,
        SelectionCountUnit::InventoryFact,
        vec![fixture.lineage.logical_case().clone()],
        vec![],
        false,
    )?;
    mixed.structural.kind = EvidenceKind::StaticInventory;
    assert_eq!(
        collect_logical_observations(&[fixture.expected.clone(), mixed], &[]),
        Err(LogicalObservationError::CountUnit)
    );
    // Explicit pure input-data alternative; it does not establish protected
    // authorization. The application preparation controls own that boundary.
    let a = LogicalCaseId::new("case.choice_a")?;
    let b = LogicalCaseId::new("case.choice_b")?;
    let alternative = FrozenSelectionAlternative::new(
        LogicalCaseId::new("group.choice")?,
        vec![
            (LogicalCaseId::new("option.a")?, vec![a.clone()]),
            (LogicalCaseId::new("option.b")?, vec![b.clone()]),
        ],
        LogicalCaseId::new("option.a")?,
    )?;
    let mut expected = fixture.expected.clone();
    expected.selection = ExpectedSelectionManifest::new(
        expected.selection.source().clone(),
        EvidenceKind::Test,
        SelectionCountUnit::LogicalTestCase,
        vec![fixture.lineage.logical_case().clone()],
        vec![alternative],
        false,
    )?;
    let a_lineage = EvidenceLineage::from_parts(EvidenceLineageParts {
        plan: Some(fixture.lineage.plan().clone()),
        campaign: Some(fixture.lineage.campaign().clone()),
        run: Some(fixture.lineage.run().clone()),
        logical_case: Some(a.clone()),
        attempt: Some(fixture.lineage.attempt().clone()),
    })?;
    let b_lineage = EvidenceLineage::from_parts(EvidenceLineageParts {
        plan: Some(fixture.lineage.plan().clone()),
        campaign: Some(fixture.lineage.campaign().clone()),
        run: Some(fixture.lineage.run().clone()),
        logical_case: Some(b.clone()),
        attempt: Some(fixture.lineage.attempt().clone()),
    })?;
    let a_candidate = eqm_engine::CompleteEvidenceCandidate {
        delivery: eqm_engine::EvidenceNormalizationCandidate {
            lineage: &a_lineage,
            ..base.delivery
        },
        ..base
    };
    let b_candidate = eqm_engine::CompleteEvidenceCandidate {
        delivery: eqm_engine::EvidenceNormalizationCandidate {
            lineage: &b_lineage,
            ..base.delivery
        },
        ..base
    };
    let incomplete = collect_logical_observations(
        std::slice::from_ref(&expected),
        &[base, b_candidate, b_candidate],
    )?;
    assert_eq!(incomplete.status(), CoverageStatus::Missing);
    assert_eq!(
        incomplete.observed(),
        &BTreeSet::from([fixture.lineage.logical_case().clone()])
    );
    assert_eq!(
        incomplete.missing()[expected.selection.source()],
        BTreeSet::from([a.clone()])
    );
    let complete = collect_logical_observations(
        std::slice::from_ref(&expected),
        &[b_candidate, base, a_candidate, a_candidate],
    )?;
    assert_eq!(complete.status(), CoverageStatus::Covered);
    assert_eq!(
        complete.observed(),
        &BTreeSet::from([fixture.lineage.logical_case().clone(), a])
    );
    assert!(complete.missing().values().all(BTreeSet::is_empty));
    assert_eq!(
        collect_logical_observations(&[expected], &[a_candidate, b_candidate, base])?,
        complete
    );
    Ok(())
}

#[test]
fn complete_matching_checks_selection_before_immutable_conflicts() -> Result<(), Box<dyn Error>> {
    use eqm_domain::*;
    use eqm_engine::{
        CompleteCoverageMismatch, CompleteEvidenceCandidate, evaluate_complete_evidence_coverage,
    };
    let (_root, fixture) = checked_coverage_fixture(false)?;
    let base = fixture.candidate();
    let failed_identity = SemanticEvidenceDigest::hash_content(b"synthetic pure failed assertion");
    let failed_record =
        coverage_assertion(&fixture.expected, failed_identity, AttemptOutcome::Failed)?;
    let failed = CompleteEvidenceCandidate {
        delivery: eqm_engine::EvidenceNormalizationCandidate {
            result: &failed_record,
            semantic_identity: failed_identity,
            raw_identity: RawObjectDigest::hash_content(b"synthetic failed raw"),
            ..base.delivery
        },
        ..base
    };
    let conflict = evaluate_complete_evidence_coverage(&fixture.expected, &[base, failed, base])?;
    assert_eq!(conflict.status(), CoverageStatus::Unknown);
    assert!(conflict.eligible().is_empty());
    let observations = eqm_engine::collect_logical_observations(
        std::slice::from_ref(&fixture.expected),
        &[base, failed, base],
    )?;
    assert_eq!(observations.status(), CoverageStatus::Unknown);
    assert!(observations.observed().is_empty());
    assert_eq!(
        observations.missing()[fixture.expected.selection.source()],
        BTreeSet::from([fixture.lineage.logical_case().clone()])
    );

    assert_eq!(
        conflict.conflicts(),
        &BTreeSet::from([(fixture.result.subject().clone(), fixture.lineage.clone())])
    );
    assert_eq!(
        evaluate_complete_evidence_coverage(&fixture.expected, &[failed, base, failed])?,
        conflict
    );
    for axis in [
        CompleteCoverageMismatch::Plan,
        CompleteCoverageMismatch::Campaign,
        CompleteCoverageMismatch::LogicalCase,
    ] {
        let changed = EvidenceLineage::from_parts(EvidenceLineageParts {
            plan: Some(if axis == CompleteCoverageMismatch::Plan {
                PlanId::new("other.plan")?
            } else {
                fixture.lineage.plan().clone()
            }),
            campaign: Some(if axis == CompleteCoverageMismatch::Campaign {
                CampaignId::new("other.campaign")?
            } else {
                fixture.lineage.campaign().clone()
            }),
            run: Some(fixture.lineage.run().clone()),
            logical_case: Some(if axis == CompleteCoverageMismatch::LogicalCase {
                LogicalCaseId::new("other.case")?
            } else {
                fixture.lineage.logical_case().clone()
            }),
            attempt: Some(fixture.lineage.attempt().clone()),
        })?;
        let rejected_failure = CompleteEvidenceCandidate {
            delivery: eqm_engine::EvidenceNormalizationCandidate {
                lineage: &changed,
                ..failed.delivery
            },
            ..failed
        };
        let mixed = evaluate_complete_evidence_coverage(
            &fixture.expected,
            &[rejected_failure, base, rejected_failure],
        )?;
        assert_eq!(mixed.status(), CoverageStatus::Covered, "{axis:?}");
        assert!(mixed.conflicts().is_empty());
        assert_eq!(
            mixed.eligible(),
            &BTreeSet::from([(fixture.identity, fixture.lineage.clone())])
        );
        assert_eq!(
            mixed.rejected()[&failed_record.id()],
            BTreeSet::from([axis])
        );
        assert_eq!(
            evaluate_complete_evidence_coverage(&fixture.expected, &[base, rejected_failure])?,
            mixed
        );
    }
    for (run, attempt) in [
        (
            RunId::new("independent.run")?,
            fixture.lineage.attempt().clone(),
        ),
        (
            fixture.lineage.run().clone(),
            AttemptId::new("retry.attempt")?,
        ),
    ] {
        let changed = EvidenceLineage::from_parts(EvidenceLineageParts {
            plan: Some(fixture.lineage.plan().clone()),
            campaign: Some(fixture.lineage.campaign().clone()),
            run: Some(run),
            logical_case: Some(fixture.lineage.logical_case().clone()),
            attempt: Some(attempt),
        })?;
        let history = CompleteEvidenceCandidate {
            delivery: eqm_engine::EvidenceNormalizationCandidate {
                lineage: &changed,
                ..failed.delivery
            },
            ..failed
        };
        let report = evaluate_complete_evidence_coverage(&fixture.expected, &[base, history])?;
        assert_eq!(report.status(), CoverageStatus::Covered);
        assert!(report.conflicts().is_empty());
        assert_eq!(report.eligible().len(), 2); // Eligibility is not passing outcome or observation credit.
    }
    Ok(())
}

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

#[test]
fn checked_count_constraints_reject_foreign_requests_and_observation_context()
-> Result<(), Box<dyn Error>> {
    use eqm_engine::{
        LogicalCountConstraintError as Error, LogicalCountConstraintStatus as Status,
        assess_logical_count_constraints, bind_expected_selections, collect_logical_observations,
        validate_logical_count_context,
    };
    for provider in [false, true] {
        let (_root, fixture, required) = checked_count_coverage_fixture(provider)?;
        let expected = vec![fixture.expected.clone()];
        let required = bind_expected_selections(
            required,
            BTreeMap::from([(
                fixture.expected.coordinate.clone(),
                vec![fixture.expected.selection.clone()],
            )]),
        )?;
        let observed = collect_logical_observations(&expected, &[fixture.candidate()])?;
        let assessed =
            assess_logical_count_constraints(&required, &expected, &[fixture.candidate()])?;
        assert_eq!(assessed.policy_minimum().get(), 1);
        assert_eq!(assessed.effective_minimum().get(), 1);
        assert_eq!(assessed.capacity(), 1);
        assert_eq!(assessed.status(), Status::CapacityMet);
        assert_eq!(assessed.observations(), &observed);
        // Engine fixtures are explicit structural data; application tests bind
        // actual S054 identities and S059 bytes. Neither proves independent trust.
        assert_eq!(
            validate_logical_count_context(&required, &[]),
            Err(Error::EmptyExpectations)
        );
        assert_eq!(
            validate_logical_count_context(
                &required,
                &vec![fixture.expected.clone(); eqm_engine::MAX_CANDIDATE_DELIVERIES + 1]
            ),
            Err(Error::Bound)
        );
        assert_eq!(
            validate_logical_count_context(
                &required,
                &[fixture.expected.clone(), fixture.expected.clone()]
            ),
            Err(Error::FrozenSelection)
        );
        let mut changed = fixture.expected.clone();
        changed.coordinate.policy_revision = Revision::new(99)?;
        assert_eq!(
            validate_logical_count_context(&required, &[changed]),
            Err(Error::Coordinate)
        );
        let mut changed = fixture.expected.clone();
        changed.selection = eqm_domain::ExpectedSelectionManifest::new(
            changed.selection.source().clone(),
            eqm_domain::EvidenceKind::Test,
            eqm_domain::SelectionCountUnit::LogicalTestCase,
            vec![eqm_domain::LogicalCaseId::new("case.foreign")?],
            vec![],
            false,
        )?;
        assert_eq!(
            validate_logical_count_context(&required, &[changed]),
            Err(Error::FrozenSelection)
        );
        let missing = assess_logical_count_constraints(&required, &expected, &[])?;
        assert_eq!(missing.status(), Status::Missing);
        // The old reusable-report route is absent: collection always uses the
        // complete validated frozen expectations inside the count operation.
        let second = eqm_domain::LogicalCaseId::new("case.frozen_second")?;
        let mut full = fixture.expected.clone();
        full.selection = eqm_domain::ExpectedSelectionManifest::new(
            full.selection.source().clone(),
            eqm_domain::EvidenceKind::Test,
            eqm_domain::SelectionCountUnit::LogicalTestCase,
            vec![fixture.lineage.logical_case().clone(), second.clone()],
            vec![],
            false,
        )?;
        let complete_universe = bind_expected_selections(
            required.clone(),
            BTreeMap::from([(full.coordinate.clone(), vec![full.selection.clone()])]),
        )?;
        let weaker_report = collect_logical_observations(&expected, &[fixture.candidate()])?;
        assert_eq!(weaker_report.status(), CoverageStatus::Covered);
        let full_expected = vec![full];
        let retained = assess_logical_count_constraints(
            &complete_universe,
            &full_expected,
            &[fixture.candidate()],
        )?;
        assert_eq!(retained.status(), Status::Missing);
        assert_eq!(retained.capacity(), 1);
        assert_eq!(
            retained.observations().missing()[fixture.expected.selection.source()],
            BTreeSet::from([second])
        );
        assert_eq!(
            assess_logical_count_constraints(&complete_universe, &expected, &[fixture.candidate()]),
            Err(Error::FrozenSelection)
        );
        for axis in 0..2 {
            let mut changed = fixture.expected.clone();
            if axis == 0 {
                changed.structural.policy_digest =
                    eqm_domain::Sha256Digest::hash_content(b"foreign policy facts");
            } else {
                let base = &changed.structural.subject;
                changed.structural.subject = eqm_domain::EvidenceSubject::new(
                    base.repository().clone(),
                    base.repository_id_digest(),
                    base.scope().clone(),
                    base.source_commit().clone(),
                    base.build_id().cloned(),
                    Some(eqm_domain::Sha256Digest::hash_content(b"foreign artifact")),
                    base.target_configuration_digest(),
                );
            }
            let identity = eqm_domain::SemanticEvidenceDigest::hash_content(if axis == 0 {
                b"foreign policy assertion"
            } else {
                b"foreign subject assertion"
            });
            let result =
                coverage_assertion(&changed, identity, eqm_domain::AttemptOutcome::Passed)?;
            let mut candidate = fixture.candidate();
            candidate.delivery.result = &result;
            candidate.delivery.semantic_identity = identity;
            let rejected = assess_logical_count_constraints(&required, &expected, &[candidate])?;
            assert_eq!(rejected.status(), Status::Missing);
            assert_eq!(rejected.capacity(), 0);
            assert!(
                !rejected.observations().matching()[fixture.expected.selection.source()]
                    .rejected()
                    .is_empty()
            );
        }
        let mut candidate = fixture.candidate();
        candidate.subjects = None;
        let grouped = assess_logical_count_constraints(&required, &expected, &[candidate])?;
        assert_eq!(grouped.status(), Status::Missing);
        assert_eq!(grouped.capacity(), 0);
        assert_eq!(
            grouped.observations().matching()[fixture.expected.selection.source()].rejected()
                [&fixture.result.id()],
            BTreeSet::from([eqm_engine::CompleteCoverageMismatch::SubjectManifest])
        );
    }
    Ok(())
}

#[test]
fn frozen_count_work_includes_unselected_alternatives_across_all_sources()
-> Result<(), Box<dyn Error>> {
    use eqm_domain::{
        EvidenceKind, EvidenceSpecId, ExpectedSelectionManifest, FrozenSelectionAlternative,
        LogicalCaseId, SelectionCountUnit,
    };
    use eqm_engine::{
        LogicalCountConstraintError as Error, bind_expected_selections,
        validate_logical_count_context,
    };
    let (_root, loaded) = support::loaded_example_with(|root| {
        let path = root.join("eqm/bindings/android.fixture_otp.toml");
        let original = fs::read_to_string(root.join("eqm/bindings/android.auth_signup.toml"))?
            .replace("binding.android.auth_signup", "binding.android.fixture_otp")
            .replace(
                "account.create.signup.identifier#email_default",
                REQUIREMENT,
            )
            .replace(
                "account.create.signup.identifier",
                "account.create.signup.otp",
            );
        let evidence = original.split_once("[[evidence]]").ok_or("evidence")?.1;
        let mut text = original.clone();
        for index in 1..7 {
            text.push_str("\n[[evidence]]");
            text.push_str(
                &evidence.replace("identifier_behavior", &format!("identifier_extra_{index}")),
            );
        }
        fs::write(path, text)?;
        Ok(())
    })?;
    let graph = finalize(&loaded)?;
    let required = universe(&graph, &bindings(REQUIREMENT, SCENARIO, 1)?, Some("us"))?;
    let key = required
        .coordinates()
        .iter()
        .find(|key| {
            key.requirement.as_str() == REQUIREMENT
                && key.facet == eqm_domain::Facet::Behavior
                && matches!(key.subject, ScopeSubject::Scenario(_))
        })
        .ok_or("scenario")?
        .clone();
    let fixture = coverage_fixture(&graph, key.clone())?;
    let binding = graph
        .graph()
        .bindings()
        .get(&fixture.expected.selection.source().binding)
        .ok_or("binding")?;
    let unselected = (0..79_999)
        .map(|index| LogicalCaseId::new(format!("case.unselected_{index}")))
        .collect::<Result<Vec<_>, _>>()?;
    let mut expected = Vec::new();
    for index in 0..7 {
        let specification = if index == 0 {
            EvidenceSpecId::new("identifier_behavior")?
        } else {
            EvidenceSpecId::new(format!("identifier_extra_{index}"))?
        };
        let definition = binding
            .evidence()
            .get(&specification)
            .ok_or("actual definition")?;
        let mut value = fixture.expected.clone();
        value.structural.evidence_spec_id = specification.clone();
        let digest = eqm_manifest::canonicalize_evidence_spec_v2(definition)?.digest();
        value.structural.evidence_spec_digest = digest.as_digest();
        let mut source = value.selection.source().clone();
        source.specification = specification;
        source.specification_digest = digest;
        value.selection = ExpectedSelectionManifest::new(
            source,
            EvidenceKind::Test,
            SelectionCountUnit::LogicalTestCase,
            vec![],
            vec![FrozenSelectionAlternative::new(
                LogicalCaseId::new("group.count")?,
                vec![
                    (
                        LogicalCaseId::new("option.selected")?,
                        vec![fixture.lineage.logical_case().clone()],
                    ),
                    (LogicalCaseId::new("option.unselected")?, unselected.clone()),
                ],
                LogicalCaseId::new("option.selected")?,
            )?],
            false,
        )?;
        assert_eq!(value.selection.selected().len(), 1);
        expected.push(value);
    }
    // Six complete 80,000-member manifests stay below the shared work budget;
    // adding the seventh exceeds it despite only seven selected logical labels.
    let frozen_six = bind_expected_selections(
        required.clone(),
        BTreeMap::from([(
            key.clone(),
            expected[..6]
                .iter()
                .map(|value| value.selection.clone())
                .collect(),
        )]),
    )?;
    assert_eq!(
        validate_logical_count_context(&frozen_six, &expected[..6]),
        Ok(())
    );
    drop(frozen_six);
    let frozen_seven = bind_expected_selections(
        required,
        BTreeMap::from([(
            key,
            expected
                .iter()
                .map(|value| value.selection.clone())
                .collect(),
        )]),
    )?;
    assert_eq!(
        validate_logical_count_context(&frozen_seven, &expected),
        Err(Error::Bound)
    );
    Ok(())
}
