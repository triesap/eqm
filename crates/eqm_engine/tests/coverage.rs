//! Exact evidence coverage fixtures.

use eqm_domain::{
    AttemptOutcome, DimensionId, EvidenceKind, EvidencePayload, EvidenceResult,
    EvidenceScopeSubject, EvidenceSpecId, EvidenceSubject, Extensions, Facet, FullRequirementId,
    ProfileId, ProfileSelection, Revision, SelectorText, Sha256Digest, SymbolicValueId, TargetId,
    TrustLevel, UnitId,
};
use eqm_engine::{
    CandidateNormalizationError, CoverageExpectation, CoverageMismatch, CoverageStatus,
    EvidenceCandidate, EvidenceNormalizationCandidate, MAX_CANDIDATE_DELIVERIES,
    evaluate_evidence_coverage, normalize_evidence_candidates,
};
use std::collections::BTreeMap;
use std::error::Error;

fn digest(value: &[u8]) -> Sha256Digest {
    Sha256Digest::hash_content(value)
}

fn result() -> Result<EvidenceResult, Box<dyn Error>> {
    result_with(AttemptOutcome::Passed, digest(b"spec"))
}

fn result_with(
    outcome: AttemptOutcome,
    specification: Sha256Digest,
) -> Result<EvidenceResult, Box<dyn Error>> {
    let id = digest(b"result");
    result_with_subject(outcome, specification, fixture_subject()?, id)
}

fn fixture_subject() -> Result<EvidenceSubject, Box<dyn Error>> {
    Ok(EvidenceSubject::new(
        "https://github.com/example/project".parse()?,
        digest(b"repository"),
        EvidenceScopeSubject::Target(TargetId::new("web")?),
        "a".repeat(40).parse()?,
        None,
        None,
        digest(b"target-config"),
    ))
}

fn result_with_subject(
    outcome: AttemptOutcome,
    specification: Sha256Digest,
    subject: EvidenceSubject,
    id: Sha256Digest,
) -> Result<EvidenceResult, Box<dyn Error>> {
    Ok(EvidenceResult::new(
        id,
        subject,
        TargetId::new("web")?,
        UnitId::new("account.create.signup.identifier")?,
        vec![FullRequirementId::new(
            "account.create.signup.identifier#email_default",
        )?],
        vec![Facet::Behavior],
        EvidenceKind::ManualReview,
        specification,
        digest(b"contract"),
        digest(b"binding"),
        digest(b"policy"),
        None,
        None,
        None,
        None,
        vec![ProfileSelection::new(
            ProfileId::new("audience.default")?,
            Revision::new(1)?,
            vec![(DimensionId::new("region")?, SymbolicValueId::new("us")?)],
        )?],
        "producer://human/review/reviewer-1".parse()?,
        TrustLevel::TrustedCi,
        "2026-08-07T12:00:00Z".parse()?,
        EvidencePayload::manual_review(
            outcome,
            "owner://team/reviewers".parse()?,
            Some(SelectorText::new("Approved")?),
        )?,
        Vec::new(),
        id,
        Extensions::default(),
    )?)
}

fn expectation(result: &EvidenceResult) -> Result<CoverageExpectation, Box<dyn Error>> {
    Ok(CoverageExpectation {
        evidence_spec_id: EvidenceSpecId::new("identifier_review")?,
        evidence_spec_digest: result.evidence_spec_digest(),
        requirement: result
            .requirements()
            .iter()
            .next()
            .ok_or("requirement missing")?
            .clone(),
        facet: Facet::Behavior,
        subject: result.subject().clone(),
        target: result.target().clone(),
        unit: result.unit().clone(),
        kind: result.kind(),
        contract_digest: result.contract_digest(),
        binding_digest: result.binding_digest(),
        policy_digest: result.policy_digest(),
        runner_digest: result.runner_digest(),
        adapter_digest: result.adapter_digest(),
        runtime_facts_digest: result.runtime_facts_digest(),
        profiles: result.profile_values().clone(),
        release_record_digest: result.release_record_digest(),
    })
}

#[test]
fn exact_partial_missing_and_duplicate_coverage_are_distinct() -> Result<(), Box<dyn Error>> {
    let result = result()?;
    let spec = EvidenceSpecId::new("identifier_review")?;
    let candidate = EvidenceCandidate {
        evidence_spec_id: &spec,
        result: &result,
    };
    let expected = expectation(&result)?;
    let exact = evaluate_evidence_coverage(&expected, &[candidate]);
    assert_eq!(exact.status, CoverageStatus::Covered);
    assert_eq!(exact.covered.len(), 1);

    let mut partial = expected.clone();
    partial.contract_digest = digest(b"other-contract");
    let partial = evaluate_evidence_coverage(&partial, &[candidate]);
    assert_eq!(partial.status, CoverageStatus::Missing);
    assert!(partial.rejected[&result.id()].contains(&CoverageMismatch::ContractDigest));

    assert_eq!(
        evaluate_evidence_coverage(&expected, &[]).status,
        CoverageStatus::Missing
    );
    let duplicate = evaluate_evidence_coverage(&expected, &[candidate, candidate]);
    assert_eq!(duplicate.status, CoverageStatus::Covered);
    assert_eq!(duplicate.duplicate_result_ids.len(), 1);
    assert_eq!(duplicate.covered.len(), 1);
    assert!(duplicate.conflicting_result_ids.is_empty());
    Ok(())
}

#[test]
fn every_exact_coordinate_is_checked() -> Result<(), Box<dyn Error>> {
    let result = result()?;
    let spec = EvidenceSpecId::new("wrong_spec")?;
    let candidate = EvidenceCandidate {
        evidence_spec_id: &spec,
        result: &result,
    };
    let mut expected = expectation(&result)?;
    expected.evidence_spec_digest = digest(b"wrong-spec");
    expected.facet = Facet::Accessibility;
    expected.target = TargetId::new("ios")?;
    expected.unit = UnitId::new("account.create.signup.otp")?;
    expected.runner_digest = Some(digest(b"runner"));
    expected.adapter_digest = Some(digest(b"adapter"));
    expected.runtime_facts_digest = Some(digest(b"runtime"));
    expected.release_record_digest = Some(digest(b"release"));
    expected.profiles = BTreeMap::new();
    let report = evaluate_evidence_coverage(&expected, &[candidate]);
    let mismatches = &report.rejected[&result.id()];
    for expected in [
        CoverageMismatch::EvidenceSpecId,
        CoverageMismatch::EvidenceSpecDigest,
        CoverageMismatch::Facet,
        CoverageMismatch::Target,
        CoverageMismatch::Unit,
        CoverageMismatch::RunnerDigest,
        CoverageMismatch::AdapterDigest,
        CoverageMismatch::RuntimeFacts,
        CoverageMismatch::Profiles,
        CoverageMismatch::ReleaseContext,
    ] {
        assert!(mismatches.contains(&expected));
    }
    Ok(())
}

#[test]
fn every_component_and_profile_axis_is_rejected_independently() -> Result<(), Box<dyn Error>> {
    let result = result()?;
    let expected = expectation(&result)?;
    let spec = expected.evidence_spec_id.clone();
    let candidate = EvidenceCandidate {
        evidence_spec_id: &spec,
        result: &result,
    };
    assert_eq!(
        evaluate_evidence_coverage(&expected, &[candidate]).status,
        CoverageStatus::Covered
    );
    for axis in [
        CoverageMismatch::EvidenceSpecId,
        CoverageMismatch::EvidenceSpecDigest,
        CoverageMismatch::ContractDigest,
        CoverageMismatch::BindingDigest,
        CoverageMismatch::PolicyDigest,
        CoverageMismatch::RunnerDigest,
        CoverageMismatch::AdapterDigest,
        CoverageMismatch::RuntimeFacts,
        CoverageMismatch::Profiles,
        CoverageMismatch::ReleaseContext,
    ] {
        let mut changed = expected.clone();
        match axis {
            CoverageMismatch::EvidenceSpecId => {
                changed.evidence_spec_id = EvidenceSpecId::new("foreign_spec")?
            }
            CoverageMismatch::EvidenceSpecDigest => {
                changed.evidence_spec_digest = digest(b"foreign spec")
            }
            CoverageMismatch::ContractDigest => {
                changed.contract_digest = digest(b"foreign contract")
            }
            CoverageMismatch::BindingDigest => changed.binding_digest = digest(b"foreign binding"),
            CoverageMismatch::PolicyDigest => changed.policy_digest = digest(b"foreign policy"),
            CoverageMismatch::RunnerDigest => {
                changed.runner_digest = Some(digest(b"foreign runner"))
            }
            CoverageMismatch::AdapterDigest => {
                changed.adapter_digest = Some(digest(b"foreign adapter"))
            }
            CoverageMismatch::RuntimeFacts => {
                changed.runtime_facts_digest = Some(digest(b"foreign runtime"))
            }
            CoverageMismatch::Profiles => {
                let profile = ProfileId::new("audience.default")?;
                changed.profiles = BTreeMap::from([(
                    profile.clone(),
                    ProfileSelection::new(
                        profile,
                        Revision::new(1)?,
                        vec![(DimensionId::new("region")?, SymbolicValueId::new("eu")?)],
                    )?,
                )]);
            }
            CoverageMismatch::ReleaseContext => {
                changed.release_record_digest = Some(digest(b"foreign release"))
            }
            _ => return Err("unexpected component axis".into()),
        }
        let report = evaluate_evidence_coverage(&changed, &[candidate, candidate]);
        assert_eq!(report.status, CoverageStatus::Missing, "{axis:?}");
        assert!(report.covered.is_empty());
        assert!(report.conflicting_result_ids.is_empty());
        assert_eq!(
            report.rejected[&result.id()],
            std::collections::BTreeSet::from([axis])
        );
    }
    Ok(())
}

#[test]
fn every_primary_subject_axis_is_rejected_independently() -> Result<(), Box<dyn Error>> {
    let result = result()?;
    let expected = expectation(&result)?;
    let spec = expected.evidence_spec_id.clone();
    let candidate = EvidenceCandidate {
        evidence_spec_id: &spec,
        result: &result,
    };
    let original = result.subject();
    for axis in [
        CoverageMismatch::Repository,
        CoverageMismatch::RepositoryIdentity,
        CoverageMismatch::Subject,
        CoverageMismatch::SourceCommit,
        CoverageMismatch::Build,
        CoverageMismatch::Artifact,
        CoverageMismatch::TargetConfiguration,
    ] {
        let mut changed = expected.clone();
        changed.subject = EvidenceSubject::new(
            if axis == CoverageMismatch::Repository {
                "https://example.com/different".parse()?
            } else {
                original.repository().clone()
            },
            if axis == CoverageMismatch::RepositoryIdentity {
                digest(b"other-repository")
            } else {
                original.repository_id_digest()
            },
            if axis == CoverageMismatch::Subject {
                EvidenceScopeSubject::Target(TargetId::new("other")?)
            } else {
                original.scope().clone()
            },
            if axis == CoverageMismatch::SourceCommit {
                "b".repeat(40).parse()?
            } else {
                original.source_commit().clone()
            },
            if axis == CoverageMismatch::Build {
                Some(SelectorText::new("other-build")?)
            } else {
                original.build_id().cloned()
            },
            if axis == CoverageMismatch::Artifact {
                Some(digest(b"other-artifact"))
            } else {
                original.artifact_digest()
            },
            if axis == CoverageMismatch::TargetConfiguration {
                digest(b"other-configuration")
            } else {
                original.target_configuration_digest()
            },
        );
        let rejected = evaluate_evidence_coverage(&changed, &[candidate, candidate]);
        assert_eq!(rejected.status, CoverageStatus::Missing, "{axis:?}");
        assert!(rejected.covered.is_empty());
        assert_eq!(
            rejected.rejected[&result.id()],
            std::collections::BTreeSet::from([axis])
        );
        assert!(rejected.conflicting_result_ids.is_empty());
    }
    assert_eq!(
        evaluate_evidence_coverage(&expected, &[candidate]).status,
        CoverageStatus::Covered
    );
    Ok(())
}

#[test]
fn relevant_result_identity_conflicts_are_unknown_in_both_orders() -> Result<(), Box<dyn Error>> {
    let passed = result()?;
    let failed = result_with(AttemptOutcome::Failed, digest(b"spec"))?;
    let spec = EvidenceSpecId::new("identifier_review")?;
    let expected = expectation(&passed)?;
    let candidates = [
        EvidenceCandidate {
            evidence_spec_id: &spec,
            result: &passed,
        },
        EvidenceCandidate {
            evidence_spec_id: &spec,
            result: &failed,
        },
    ];
    let report = evaluate_evidence_coverage(&expected, &candidates);
    assert_eq!(report.status, CoverageStatus::Unknown);
    assert_eq!(
        report.conflicting_result_ids,
        std::collections::BTreeSet::from([passed.id()])
    );
    assert!(report.covered.is_empty());
    assert_eq!(
        evaluate_evidence_coverage(&expected, &[candidates[1], candidates[0], candidates[1]]),
        report
    );
    Ok(())
}

#[test]
fn unrelated_rejected_duplicates_never_poison_matching_coverage() -> Result<(), Box<dyn Error>> {
    let passed = result()?;
    let unrelated = result_with(AttemptOutcome::Failed, digest(b"unrelated specification"))?;
    let spec = EvidenceSpecId::new("identifier_review")?;
    let wrong_spec = EvidenceSpecId::new("unrelated_review")?;
    let expected = expectation(&passed)?;
    let compatible = EvidenceCandidate {
        evidence_spec_id: &spec,
        result: &passed,
    };
    let rejected = EvidenceCandidate {
        evidence_spec_id: &spec,
        result: &unrelated,
    };
    let wrong_mapping = EvidenceCandidate {
        evidence_spec_id: &wrong_spec,
        result: &passed,
    };
    let report = evaluate_evidence_coverage(
        &expected,
        &[rejected, compatible, wrong_mapping, rejected, compatible],
    );
    assert_eq!(report.status, CoverageStatus::Covered);
    assert_eq!(
        report.covered,
        std::collections::BTreeSet::from([passed.id()])
    );
    assert!(report.conflicting_result_ids.is_empty());
    assert!(report.rejected[&passed.id()].contains(&CoverageMismatch::EvidenceSpecDigest));
    assert!(report.rejected[&passed.id()].contains(&CoverageMismatch::EvidenceSpecId));
    let absent = evaluate_evidence_coverage(&expected, &[rejected, rejected]);
    assert_eq!(absent.status, CoverageStatus::Missing);
    assert!(absent.conflicting_result_ids.is_empty());
    Ok(())
}

fn fixture_lineage() -> Result<eqm_domain::EvidenceLineage, Box<dyn Error>> {
    Ok(eqm_domain::EvidenceLineage::from_parts(
        eqm_domain::EvidenceLineageParts {
            plan: Some(eqm_domain::PlanId::new("plan.one")?),
            campaign: Some(eqm_domain::CampaignId::new("campaign.one")?),
            run: Some(eqm_domain::RunId::new("run.one")?),
            logical_case: Some(eqm_domain::LogicalCaseId::new("case.one")?),
            attempt: Some(eqm_domain::AttemptId::new("attempt.one")?),
        },
    )?)
}

// These engine inputs are synthetic structural data, not authenticated wire.
// Application controls independently drive actual byte admission/projections.
#[test]
fn every_subject_axis_partitions_immutable_attempts() -> Result<(), Box<dyn Error>> {
    let subject = fixture_subject()?;
    let baseline_identity =
        eqm_domain::SemanticEvidenceDigest::hash_content(b"synthetic baseline assertion");
    let baseline = result_with_subject(
        AttemptOutcome::Passed,
        digest(b"spec"),
        subject.clone(),
        baseline_identity.as_digest(),
    )?;
    let lineage = fixture_lineage()?;
    let spec = EvidenceSpecId::new("identifier_review")?;
    let provenance = SelectorText::new("receipt.fixture")?;
    let base = EvidenceNormalizationCandidate {
        result: &baseline,
        lineage: &lineage,
        semantic_identity: baseline_identity,
        raw_identity: eqm_domain::RawObjectDigest::hash_content(b"synthetic baseline raw"),
        evidence_spec_id: &spec,
        provenance: &provenance,
    };
    for axis in [
        "repository",
        "repository identity",
        "scope",
        "commit",
        "build",
        "artifact",
        "configuration",
    ] {
        let mut repository = subject.repository().clone();
        let mut repository_id = subject.repository_id_digest();
        let mut scope = subject.scope().clone();
        let mut commit = subject.source_commit().clone();
        let mut build = subject.build_id().cloned();
        let mut artifact = subject.artifact_digest();
        let mut configuration = subject.target_configuration_digest();
        match axis {
            "repository" => repository = "https://github.com/example/other".parse()?,
            "repository identity" => repository_id = digest(b"other repository identity"),
            "scope" => {
                scope = EvidenceScopeSubject::target_set(vec![
                    TargetId::new("web")?,
                    TargetId::new("ios")?,
                ])?
            }
            "commit" => commit = "b".repeat(40).parse()?,
            "build" => build = Some(SelectorText::new("build.other")?),
            "artifact" => artifact = Some(digest(b"other artifact")),
            "configuration" => configuration = digest(b"other configuration"),
            _ => return Err("unlisted subject axis".into()),
        }
        let changed_subject = EvidenceSubject::new(
            repository,
            repository_id,
            scope,
            commit,
            build,
            artifact,
            configuration,
        );
        let identity = eqm_domain::SemanticEvidenceDigest::hash_content(
            format!("synthetic assertion {axis}").as_bytes(),
        );
        let changed = result_with_subject(
            AttemptOutcome::Passed,
            digest(b"spec"),
            changed_subject.clone(),
            identity.as_digest(),
        )?;
        let other = EvidenceNormalizationCandidate {
            result: &changed,
            lineage: &lineage,
            semantic_identity: identity,
            raw_identity: eqm_domain::RawObjectDigest::hash_content(axis.as_bytes()),
            evidence_spec_id: &spec,
            provenance: &provenance,
        };
        let normalized = normalize_evidence_candidates(&[base, other])?;
        assert_eq!(normalized.attempts().len(), 2, "{axis}");
        assert!(
            normalized.attempts().values().all(|x| !x.has_conflict()),
            "{axis}"
        );
        assert!(
            normalized
                .attempts()
                .keys()
                .any(|x| x.subject() == &changed_subject),
            "{axis}"
        );
        assert_eq!(
            normalize_evidence_candidates(&[other, base, other])?,
            normalized,
            "{axis}"
        );
    }
    Ok(())
}

#[test]
fn inconsistent_candidate_identities_and_over_bound_input_fail_explicitly()
-> Result<(), Box<dyn Error>> {
    let identity = eqm_domain::SemanticEvidenceDigest::hash_content(b"synthetic assertion");
    let passed = result_with_subject(
        AttemptOutcome::Passed,
        digest(b"spec"),
        fixture_subject()?,
        identity.as_digest(),
    )?;
    let failed = result_with_subject(
        AttemptOutcome::Failed,
        digest(b"spec"),
        fixture_subject()?,
        identity.as_digest(),
    )?;
    let lineage = fixture_lineage()?;
    let spec = EvidenceSpecId::new("identifier_review")?;
    let provenance = SelectorText::new("receipt.fixture")?;
    let candidate = EvidenceNormalizationCandidate {
        result: &passed,
        lineage: &lineage,
        semantic_identity: identity,
        raw_identity: eqm_domain::RawObjectDigest::hash_content(b"synthetic raw"),
        evidence_spec_id: &spec,
        provenance: &provenance,
    };
    let wrong = EvidenceNormalizationCandidate {
        semantic_identity: eqm_domain::SemanticEvidenceDigest::hash_content(b"wrong identity"),
        ..candidate
    };
    assert_eq!(
        normalize_evidence_candidates(&[wrong]),
        Err(CandidateNormalizationError::IdentityMismatch)
    );
    let contradiction = EvidenceNormalizationCandidate {
        result: &failed,
        ..candidate
    };
    assert_eq!(
        normalize_evidence_candidates(&[candidate, contradiction]),
        Err(CandidateNormalizationError::ConflictingSemanticIdentity)
    );
    assert_eq!(
        normalize_evidence_candidates(&[contradiction, candidate]),
        Err(CandidateNormalizationError::ConflictingSemanticIdentity)
    );
    assert_eq!(
        normalize_evidence_candidates(&vec![candidate; MAX_CANDIDATE_DELIVERIES])?
            .attempts()
            .len(),
        1
    );
    assert_eq!(
        normalize_evidence_candidates(&vec![candidate; MAX_CANDIDATE_DELIVERIES + 1]),
        Err(CandidateNormalizationError::LimitExceeded)
    );
    Ok(())
}
