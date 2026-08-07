//! Exact freshness-key and injected-clock fixtures.

use eqm_domain::{
    DimensionId, DurationMillis, EvidenceScopeSubject, EvidenceSubject, ProfileId,
    ProfileSelection, Revision, Sha256Digest, SymbolicValueId, TargetId, ToolVersion,
};
use eqm_engine::{FreshnessKey, FreshnessMismatch, FreshnessStatus, evaluate_evidence_freshness};
use std::collections::BTreeMap;
use std::error::Error;

fn digest(value: u8) -> Sha256Digest {
    Sha256Digest::from_bytes([value; 32])
}

fn key() -> Result<FreshnessKey, Box<dyn Error>> {
    Ok(FreshnessKey {
        subject: EvidenceSubject::new(
            "https://github.com/example/project".parse()?,
            digest(1),
            EvidenceScopeSubject::Target(TargetId::new("web")?),
            "a".repeat(40).parse()?,
            None,
            None,
            digest(2),
        ),
        contract_digest: digest(3),
        binding_digest: digest(4),
        evidence_spec_digest: digest(5),
        runner_digest: Some(digest(6)),
        adapter_digest: None,
        policy_digest: digest(7),
        profile_values: BTreeMap::from([(
            ProfileId::new("audience.default")?,
            ProfileSelection::new(
                ProfileId::new("audience.default")?,
                Revision::new(1)?,
                vec![(DimensionId::new("region")?, SymbolicValueId::new("us")?)],
            )?,
        )]),
        target_configuration_digest: digest(2),
        runtime_facts_digest: Some(digest(8)),
        release_record_digest: None,
        trust_config_digest: digest(9),
        producer: "producer://ci/github/run-42".parse()?,
        tool_version: ToolVersion::CURRENT,
    })
}

#[test]
fn unchanged_boundary_is_fresh_and_age_or_future_time_fails() -> Result<(), Box<dyn Error>> {
    let key = key()?;
    let observed = "2026-08-07T12:00:00Z".parse()?;
    let boundary = "2026-08-07T12:01:00Z".parse()?;
    assert_eq!(
        evaluate_evidence_freshness(
            &key,
            &key,
            Some(observed),
            Some(DurationMillis::new(60_000)?),
            boundary,
        )
        .status,
        FreshnessStatus::Fresh
    );
    assert_eq!(
        evaluate_evidence_freshness(
            &key,
            &key,
            Some(observed),
            Some(DurationMillis::new(59_999)?),
            boundary,
        )
        .status,
        FreshnessStatus::Stale
    );
    assert_eq!(
        evaluate_evidence_freshness(
            &key,
            &key,
            Some("2026-08-07T12:05:01Z".parse()?),
            Some(DurationMillis::new(60_000)?),
            observed,
        )
        .status,
        FreshnessStatus::Unknown
    );
    assert_eq!(
        evaluate_evidence_freshness(&key, &key, None, None, boundary).status,
        FreshnessStatus::Unknown
    );
    Ok(())
}

#[test]
fn every_mutable_semantic_key_axis_is_independently_stale() -> Result<(), Box<dyn Error>> {
    let expected = key()?;
    let observed_at = Some("2026-08-07T12:00:00Z".parse()?);
    let evaluated_at = "2026-08-07T12:00:01Z".parse()?;
    let maximum_age = Some(DurationMillis::new(60_000)?);
    let cases: Vec<(FreshnessMismatch, FreshnessKey)> = vec![
        (FreshnessMismatch::Subject, {
            let mut value = expected.clone();
            value.subject = EvidenceSubject::new(
                "https://github.com/example/project".parse()?,
                digest(1),
                EvidenceScopeSubject::Target(TargetId::new("web")?),
                "b".repeat(40).parse()?,
                None,
                None,
                digest(2),
            );
            value
        }),
        (FreshnessMismatch::Contract, {
            let mut value = expected.clone();
            value.contract_digest = digest(20);
            value
        }),
        (FreshnessMismatch::Binding, {
            let mut value = expected.clone();
            value.binding_digest = digest(20);
            value
        }),
        (FreshnessMismatch::EvidenceSpecification, {
            let mut value = expected.clone();
            value.evidence_spec_digest = digest(20);
            value
        }),
        (FreshnessMismatch::Runner, {
            let mut value = expected.clone();
            value.runner_digest = None;
            value
        }),
        (FreshnessMismatch::Adapter, {
            let mut value = expected.clone();
            value.adapter_digest = Some(digest(20));
            value
        }),
        (FreshnessMismatch::Policy, {
            let mut value = expected.clone();
            value.policy_digest = digest(20);
            value
        }),
        (FreshnessMismatch::Profiles, {
            let mut value = expected.clone();
            value.profile_values.clear();
            value
        }),
        (FreshnessMismatch::TargetConfiguration, {
            let mut value = expected.clone();
            value.target_configuration_digest = digest(20);
            value
        }),
        (FreshnessMismatch::RuntimeFacts, {
            let mut value = expected.clone();
            value.runtime_facts_digest = None;
            value
        }),
        (FreshnessMismatch::ReleaseRecord, {
            let mut value = expected.clone();
            value.release_record_digest = Some(digest(20));
            value
        }),
        (FreshnessMismatch::TrustConfiguration, {
            let mut value = expected.clone();
            value.trust_config_digest = digest(20);
            value
        }),
        (FreshnessMismatch::Producer, {
            let mut value = expected.clone();
            value.producer = "producer://ci/github/run-99".parse()?;
            value
        }),
    ];
    for (axis, observed) in cases {
        let report = evaluate_evidence_freshness(
            &observed,
            &expected,
            observed_at,
            maximum_age,
            evaluated_at,
        );
        assert_eq!(report.status, FreshnessStatus::Stale);
        assert_eq!(
            report.mismatches.into_iter().collect::<Vec<_>>(),
            vec![axis]
        );
    }
    Ok(())
}
