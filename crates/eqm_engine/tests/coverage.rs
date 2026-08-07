//! Exact evidence coverage fixtures.

use eqm_domain::{
    AttemptOutcome, DimensionId, EvidenceKind, EvidencePayload, EvidenceResult,
    EvidenceScopeSubject, EvidenceSpecId, EvidenceSubject, Extensions, Facet, FullRequirementId,
    ProfileId, ProfileSelection, Revision, SelectorText, Sha256Digest, SymbolicValueId, TargetId,
    TrustLevel, UnitId,
};
use eqm_engine::{
    CoverageExpectation, CoverageMismatch, CoverageStatus, EvidenceCandidate,
    evaluate_evidence_coverage,
};
use std::collections::BTreeMap;
use std::error::Error;

fn digest(value: &[u8]) -> Sha256Digest {
    Sha256Digest::hash_content(value)
}

fn result() -> Result<EvidenceResult, Box<dyn Error>> {
    let id = digest(b"result");
    Ok(EvidenceResult::new(
        id,
        EvidenceSubject::new(
            "https://github.com/example/project".parse()?,
            digest(b"repository"),
            EvidenceScopeSubject::Target(TargetId::new("web")?),
            "a".repeat(40).parse()?,
            None,
            None,
            digest(b"target-config"),
        ),
        TargetId::new("web")?,
        UnitId::new("account.create.signup.identifier")?,
        vec![FullRequirementId::new(
            "account.create.signup.identifier#email_default",
        )?],
        vec![Facet::Behavior],
        EvidenceKind::ManualReview,
        digest(b"spec"),
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
            AttemptOutcome::Passed,
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
        subject: result.subject().scope().clone(),
        target: result.target().clone(),
        unit: result.unit().clone(),
        kind: result.kind(),
        contract_digest: result.contract_digest(),
        binding_digest: result.binding_digest(),
        policy_digest: result.policy_digest(),
        runner_digest: result.runner_digest(),
        adapter_digest: result.adapter_digest(),
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
    assert_eq!(duplicate.status, CoverageStatus::Unknown);
    assert_eq!(duplicate.duplicate_result_ids.len(), 1);
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
        CoverageMismatch::Profiles,
        CoverageMismatch::ReleaseContext,
    ] {
        assert!(mismatches.contains(&expected));
    }
    Ok(())
}
