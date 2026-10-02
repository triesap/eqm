//! S025-R2: real prepared-session projections admitted at the v2 boundary.
//!
//! This integration test drives the production loader/prepared-session and the
//! existing canonical v2 projection APIs to obtain the contract, binding,
//! evidence-specification, target-configuration and selected-policy identities,
//! plus the production runner acquisition identity. Those exact typed results
//! are wired into one genuine v2 evidence assertion, sealed, parsed from exact
//! bytes and admitted against an independently computed typed context. No
//! test-local substitute projector or seed-string hash stands in for a
//! projection.

use eqm_application::{
    PolicyProvenanceV2, SessionRequest, prepare, prepare_policy_provenance,
    prepare_target_configuration,
};
use eqm_domain::{
    AdapterDigest, BindingDigest, BindingId, CampaignId, ContractDigest, DimensionId,
    EvidenceExecution, EvidenceKind, EvidenceLineage, EvidenceLineageParts, EvidenceScopeSubject,
    EvidenceSpecId, EvidenceSpecificationDigest, EvidenceSubject, ExpectedLineageAuthority, Facet,
    FullRequirementId, LogicalCaseId, PlanId, PolicyId, PolicySelectionDigest, ProfileId,
    ProfileSelection, Revision, RunId, RunnerDigest, RunnerGuarantee, RunnerId, RunnerProgram,
    SelectorText, SemanticEvidenceDigest, Sha256Digest, SymbolicValueId, TargetConfigurationDigest,
    TargetId, UnitId, UtcInstant,
};
use eqm_engine::{
    AuthorityOrigin, EvaluationMode, PolicyProfileRequest, PolicyRef, ProfileRequest,
};
use eqm_manifest::{canonicalize_binding_v2, canonicalize_evidence_spec_v2};
use eqm_protocol::{
    AdmittedEvidenceV2, AttemptV2Dto, CanonicalInstant, CanonicalSet, CheckedEvidenceResultV2,
    CountsV2Dto, EVIDENCE_RESULT_SCHEMA_V2, EvidenceAdmissionContext, EvidenceAdmissionError,
    EvidencePayloadV2Dto, EvidenceResultV2Dto, EvidenceSubjectV2Dto, ExecutionPayloadV2Dto,
    ProfileValueV2Dto, ScopeSubjectV2Dto, WireUint, admit_evidence,
};
use eqm_runner::{ResolvedRunner, RunnerResolutionAuthority, resolve_runner};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;
use std::path::Path;

const REPOSITORY: &str = "https://example.com/team/project";

fn copy_directory(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let target = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_directory(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

fn example() -> Result<tempfile::TempDir, Box<dyn Error>> {
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/android-ios");
    let repository = tempfile::tempdir()?;
    copy_directory(&source, repository.path())?;
    fs::create_dir(repository.path().join(".git"))?;
    Ok(repository)
}

fn evaluation_time() -> Result<UtcInstant, Box<dyn Error>> {
    Ok("2026-01-02T03:04:05Z".parse()?)
}

fn instant(value: &str) -> Result<CanonicalInstant, Box<dyn Error>> {
    Ok(CanonicalInstant::new(value.parse()?))
}

fn selection() -> Result<PolicyProfileRequest, Box<dyn Error>> {
    Ok(PolicyProfileRequest::new(
        AuthorityOrigin::CandidateLocal,
        PolicyRef::new(PolicyId::new("consumer.critical_flow")?, Revision::new(1)?),
        vec![ProfileRequest::new(
            ProfileId::new("audience.default")?,
            Revision::new(1)?,
            vec![(
                DimensionId::new("region")?,
                Some(SymbolicValueId::new("eu")?),
            )],
        )?],
    )?)
}

/// The genuine typed projections consumed by the assertion and its context.
struct Provenance {
    contract_digest: ContractDigest,
    binding_digest: BindingDigest,
    evidence_spec_digest: EvidenceSpecificationDigest,
    policy_digest: PolicySelectionDigest,
    target_configuration_digest: TargetConfigurationDigest,
    runner_digest: RunnerDigest,
}

/// Resolves the session's runner definition against a real executable read from
/// the prepared repository through the production resolution path.
fn resolve_session_runner(
    repository: &Path,
    definition: &eqm_domain::RunnerDefinition,
    relative: &str,
) -> Result<ResolvedRunner, Box<dyn Error>> {
    let absolute = repository.join(relative);
    fs::create_dir_all(
        absolute
            .parent()
            .ok_or("runner program has no parent directory")?,
    )?;
    fs::write(&absolute, b"#!/bin/sh\nexit 0\n")?;
    let digest = Sha256Digest::hash_content(&fs::read(&absolute)?);
    let program = match definition.program() {
        RunnerProgram::Repository(path) => path.clone(),
        RunnerProgram::Locked { .. } => {
            return Err("runner fixture must be repository-confined".into());
        }
    };
    let limits = definition.limits();
    let authority = RunnerResolutionAuthority {
        id: definition.id().clone(),
        revision: definition.revision(),
        backends: BTreeSet::from([definition.backend()]),
        repository_programs: BTreeMap::from([(program, digest)]),
        backend_guarantees: BTreeMap::from([(
            definition.backend(),
            BTreeSet::<RunnerGuarantee>::new(),
        )]),
        maximum_timeout: limits.timeout(),
        maximum_output_bytes: limits.max_output_bytes(),
        maximum_concurrency: limits.max_concurrency(),
    };
    Ok(resolve_runner(definition, &authority)?)
}

fn projections(session: &eqm_application::PreparedSession) -> Result<Provenance, Box<dyn Error>> {
    let graph = session.finalized().graph();

    let binding = graph
        .bindings()
        .get(&BindingId::new("binding.android.auth_signup")?)
        .ok_or("binding missing from prepared graph")?;
    let spec = binding
        .evidence()
        .get(&EvidenceSpecId::new("identifier_behavior")?)
        .ok_or("evidence specification missing from binding")?;
    let binding_digest = canonicalize_binding_v2(binding)?.digest();
    let evidence_spec_digest = canonicalize_evidence_spec_v2(spec)?.digest();

    let target = TargetId::new("android")?;
    let target_configuration_digest = prepare_target_configuration(session, &target)?.digest();

    let provenance: PolicyProvenanceV2 = prepare_policy_provenance(
        session,
        EvaluationMode::Development,
        Some(&selection()?),
        None,
    )?;
    let contract_digest = provenance.contract().digest();
    let policy_digest = provenance.policy_selection().digest();

    let runner_definition = graph
        .runners()
        .get(&(RunnerId::new("runner.android")?, Revision::new(1)?))
        .ok_or("runner missing from prepared graph")?;
    let runner_digest = resolve_session_runner(
        session.repository_root(),
        runner_definition,
        "apps/android/gradlew",
    )?
    .identity_v2();

    Ok(Provenance {
        contract_digest,
        binding_digest,
        evidence_spec_digest,
        policy_digest,
        target_configuration_digest,
        runner_digest,
    })
}

fn record(
    provenance: &Provenance,
    adapter: Option<AdapterDigest>,
) -> Result<EvidenceResultV2Dto, Box<dyn Error>> {
    let id = Sha256Digest::hash_content(b"placeholder").to_string();
    Ok(EvidenceResultV2Dto {
        schema: EVIDENCE_RESULT_SCHEMA_V2.to_string(),
        id: id.clone(),
        subject: EvidenceSubjectV2Dto {
            repository: REPOSITORY.to_owned(),
            repository_id_digest: Sha256Digest::hash_content(b"repository-identity").to_string(),
            scope: ScopeSubjectV2Dto::Target {
                target: "android".to_owned(),
            },
            source_commit: "a".repeat(40),
            build_id: Some("build.one".to_owned()),
            artifact_digest: Some(Sha256Digest::hash_content(b"artifact").to_string()),
            target_configuration_digest: provenance.target_configuration_digest.to_string(),
        },
        target: "android".to_owned(),
        unit: "account.create.signup.identifier".to_owned(),
        requirements: CanonicalSet::from_members(vec![
            "account.create.signup.identifier#email_default".to_owned(),
        ])?,
        facets: CanonicalSet::from_members(vec!["behavior".to_owned()])?,
        kind: "test".to_owned(),
        evidence_spec_digest: provenance.evidence_spec_digest.to_string(),
        contract_digest: provenance.contract_digest.to_string(),
        binding_digest: provenance.binding_digest.to_string(),
        policy_digest: provenance.policy_digest.to_string(),
        runner_digest: Some(provenance.runner_digest.to_string()),
        adapter_digest: adapter.map(|value| value.to_string()),
        runtime_facts_digest: None,
        release_record_digest: None,
        profile_values: CanonicalSet::from_members(vec![ProfileValueV2Dto {
            profile: "audience.default".to_owned(),
            revision: WireUint::new(1),
            dimension: "region".to_owned(),
            value: "eu".to_owned(),
        }])?,
        producer: "producer://ci/runner/1".to_owned(),
        claimed_trust: "signed_ci".to_owned(),
        observed_at: instant("2026-01-02T03:04:05Z")?,
        payload: EvidencePayloadV2Dto::Test {
            execution: ExecutionPayloadV2Dto {
                attempts: vec![AttemptV2Dto {
                    number: WireUint::new(1),
                    outcome: "passed".to_owned(),
                    started_at: instant("2026-01-02T03:04:05Z")?,
                    finished_at: instant("2026-01-02T03:04:06Z")?,
                    message: None,
                }],
                counts: CountsV2Dto {
                    selected: WireUint::new(1),
                    passed: WireUint::new(1),
                    failed: WireUint::new(0),
                    skipped: WireUint::new(0),
                    filtered: WireUint::new(0),
                    quarantined: WireUint::new(0),
                },
                started_at: instant("2026-01-02T03:00:00Z")?,
                finished_at: instant("2026-01-02T04:00:00Z")?,
            },
        },
        attachments: CanonicalSet::empty(),
        result_digest: id,
    })
}

fn context(
    provenance: &Provenance,
    adapter: Option<AdapterDigest>,
) -> Result<EvidenceAdmissionContext, Box<dyn Error>> {
    let profile = ProfileId::new("audience.default")?;
    Ok(EvidenceAdmissionContext {
        subject: EvidenceSubject::new(
            REPOSITORY.parse()?,
            Sha256Digest::hash_content(b"repository-identity"),
            EvidenceScopeSubject::Target(TargetId::new("android")?),
            "a".repeat(40).parse()?,
            Some(SelectorText::new("build.one")?),
            Some(Sha256Digest::hash_content(b"artifact")),
            provenance.target_configuration_digest.as_digest(),
        ),
        target: TargetId::new("android")?,
        unit: UnitId::new("account.create.signup.identifier")?,
        requirement: FullRequirementId::new("account.create.signup.identifier#email_default")?,
        facets: BTreeSet::from([Facet::Behavior]),
        kind: EvidenceKind::Test,
        evidence_spec_digest: provenance.evidence_spec_digest,
        contract_digest: provenance.contract_digest,
        binding_digest: provenance.binding_digest,
        policy_digest: provenance.policy_digest,
        target_configuration_digest: provenance.target_configuration_digest,
        runner_digest: Some(provenance.runner_digest),
        adapter_digest: adapter,
        runtime_facts_digest: None,
        release_record_digest: None,
        profiles: BTreeMap::from([(
            profile.clone(),
            ProfileSelection::new(
                profile,
                Revision::new(1)?,
                vec![(DimensionId::new("region")?, SymbolicValueId::new("eu")?)],
            )?,
        )]),
        provider_targets: BTreeMap::new(),
        expected_lineage: ExpectedLineageAuthority::new(
            PlanId::new("plan.release")?,
            CampaignId::new("campaign.signup")?,
        ),
    })
}

fn lineage() -> Result<EvidenceLineage, Box<dyn Error>> {
    Ok(EvidenceLineage::from_parts(EvidenceLineageParts {
        plan: Some(PlanId::new("plan.release")?),
        campaign: Some(CampaignId::new("campaign.signup")?),
        run: Some(RunId::new("run.one")?),
        logical_case: Some(LogicalCaseId::new("case.identifier")?),
        attempt: Some(eqm_domain::AttemptId::new("attempt.one")?),
    })?)
}

fn execution() -> Result<EvidenceExecution, Box<dyn Error>> {
    Ok(EvidenceExecution::new(
        RunId::new("run.one")?,
        LogicalCaseId::new("case.identifier")?,
        vec![eqm_domain::AttemptId::new("attempt.one")?],
    )?)
}

/// Sets `id`/`result_digest` to the recomputed semantic identity and parses it.
fn seal(mut value: EvidenceResultV2Dto) -> Result<CheckedEvidenceResultV2, Box<dyn Error>> {
    let provisional = CheckedEvidenceResultV2::from_bytes(&serde_json::to_vec(&value)?)?;
    let semantic = provisional.semantic_identity().to_string();
    value.id = semantic.clone();
    value.result_digest = semantic;
    Ok(CheckedEvidenceResultV2::from_bytes(&serde_json::to_vec(
        &value,
    )?)?)
}

fn session_for(repository: &Path) -> Result<eqm_application::PreparedSession, Box<dyn Error>> {
    Ok(prepare(
        &SessionRequest::new(None, evaluation_time()?),
        repository,
    )?)
}

#[test]
fn production_projections_admit_sealed_evidence() -> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let session = session_for(repository.path())?;
    let provenance = projections(&session)?;
    let checked = seal(record(&provenance, None)?)?;
    let admitted: AdmittedEvidenceV2 = admit_evidence(
        &checked,
        &lineage()?,
        &execution()?,
        &context(&provenance, None)?,
    )?;

    // The assertion binds the production projections, not seed-string hashes.
    assert_eq!(admitted.contract_digest(), provenance.contract_digest);
    assert_eq!(admitted.binding_digest(), provenance.binding_digest);
    assert_eq!(
        admitted.evidence_spec_digest(),
        provenance.evidence_spec_digest
    );
    assert_eq!(admitted.policy_digest(), provenance.policy_digest);
    assert_eq!(
        admitted.target_configuration_digest(),
        provenance.target_configuration_digest
    );
    assert_eq!(admitted.runner_digest(), Some(provenance.runner_digest));
    assert_eq!(admitted.semantic_identity(), checked.semantic_identity());
    assert_eq!(admitted.raw_bytes(), checked.raw_bytes());
    assert_eq!(admitted.execution(), &execution()?);
    Ok(())
}

#[test]
fn semantic_binding_mutation_rejects_old_evidence_and_formatting_preserves_identity()
-> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let baseline = projections(&session_for(repository.path())?)?;
    let checked = seal(record(&baseline, None)?)?;
    assert!(
        admit_evidence(
            &checked,
            &lineage()?,
            &execution()?,
            &context(&baseline, None)?
        )
        .is_ok()
    );

    // A real semantic edit to the authored binding moves the projected identity
    // through the production loader and canonical v2 projection.
    let binding_path = repository
        .path()
        .join("eqm/bindings/android.auth_signup.toml");
    let original = fs::read_to_string(&binding_path)?;
    let mutated = original.replace("SignupScreen.kt", "SignupScreenV2.kt");
    assert_ne!(mutated, original, "the semantic mutation must be real");
    fs::write(&binding_path, &mutated)?;
    let mutated_provenance = projections(&session_for(repository.path())?)?;
    assert_ne!(
        mutated_provenance.binding_digest, baseline.binding_digest,
        "the semantic binding edit must move the v2 binding identity"
    );

    // The old sealed evidence no longer matches the independently recomputed
    // typed context.
    let mut mutated_context = context(&baseline, None)?;
    mutated_context.binding_digest = mutated_provenance.binding_digest;
    assert_eq!(
        admit_evidence(&checked, &lineage()?, &execution()?, &mutated_context),
        Err(EvidenceAdmissionError::DigestMismatch)
    );

    // A formatting-only equivalent loaded input preserves the identity and the
    // old evidence still admits.
    fs::write(
        &binding_path,
        format!("# authored formatting comment, no semantics\n\n{original}\n"),
    )?;
    let formatted = projections(&session_for(repository.path())?)?;
    assert_eq!(formatted.binding_digest, baseline.binding_digest);
    assert!(
        admit_evidence(
            &checked,
            &lineage()?,
            &execution()?,
            &context(&formatted, None)?
        )
        .is_ok()
    );
    Ok(())
}

#[test]
fn adapter_typed_reference_comparison() -> Result<(), Box<dyn Error>> {
    // No adapter projection API exists in this step: the adapter identity
    // carrier is the v2 `adapter_digest` transport string compared against the
    // explicit typed `AdapterDigest` expectation.
    let repository = example()?;
    let provenance = projections(&session_for(repository.path())?)?;
    let adapter = AdapterDigest::hash_content(b"adapter-identity");
    let checked = seal(record(&provenance, Some(adapter))?)?;

    // The matching typed expectation admits and is retained.
    let admitted = admit_evidence(
        &checked,
        &lineage()?,
        &execution()?,
        &context(&provenance, Some(adapter))?,
    )?;
    assert_eq!(admitted.adapter_digest(), Some(adapter));

    // A different typed adapter identity rejects.
    let other = AdapterDigest::hash_content(b"other-adapter-identity");
    assert_eq!(
        admit_evidence(
            &checked,
            &lineage()?,
            &execution()?,
            &context(&provenance, Some(other))?
        ),
        Err(EvidenceAdmissionError::DigestMismatch)
    );
    Ok(())
}

#[test]
fn target_configuration_lookup_mutation_rejects_old_evidence() -> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let baseline = projections(&session_for(repository.path())?)?;
    let checked = seal(record(&baseline, None)?)?;

    // A semantic change to the android target definition moves the exact
    // target-configuration lookup identity.
    let config_path = repository.path().join("eqm.toml");
    let config = fs::read_to_string(&config_path)?;
    let mutated = config.replace("framework = \"compose\"", "framework = \"compose2\"");
    assert_ne!(mutated, config);
    fs::write(&config_path, mutated)?;
    let mutated_provenance = projections(&session_for(repository.path())?)?;
    assert_ne!(
        mutated_provenance.target_configuration_digest,
        baseline.target_configuration_digest
    );
    let mut mutated_context = context(&baseline, None)?;
    mutated_context.target_configuration_digest = mutated_provenance.target_configuration_digest;
    assert_eq!(
        admit_evidence(&checked, &lineage()?, &execution()?, &mutated_context),
        Err(EvidenceAdmissionError::SubjectMismatch)
    );
    Ok(())
}

#[test]
fn semantic_identity_is_a_distinct_domain() -> Result<(), Box<dyn Error>> {
    // The semantic evidence identity is not interchangeable with a projected
    // component identity, even over identical bytes.
    let repository = example()?;
    let provenance = projections(&session_for(repository.path())?)?;
    let checked = seal(record(&provenance, None)?)?;
    let semantic: SemanticEvidenceDigest = checked.semantic_identity();
    let bytes = checked.raw_bytes();
    assert_ne!(
        Sha256Digest::hash_content(bytes).to_string(),
        semantic.to_string()
    );
    assert_ne!(
        ContractDigest::hash_content(bytes).to_string(),
        semantic.to_string()
    );
    Ok(())
}
