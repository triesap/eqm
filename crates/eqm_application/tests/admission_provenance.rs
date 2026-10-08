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
    AdmittedEvidenceDelivery, EvidenceConversionError, PolicyProvenanceV2, SessionRequest,
    admit_evidence_bytes, normalize_admitted_evidence, prepare, prepare_policy_provenance,
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
    AuthorityOrigin, CandidateNormalizationError, CoverageExpectation, CoverageMismatch,
    CoverageStatus, EvaluationMode, MAX_CANDIDATE_DELIVERIES, PolicyProfileRequest, PolicyRef,
    ProfileRequest, evaluate_normalized_evidence_coverage,
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
    projections_for_selection(session, &selection()?)
}

fn projections_for_selection(
    session: &eqm_application::PreparedSession,
    selected: &PolicyProfileRequest,
) -> Result<Provenance, Box<dyn Error>> {
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

    let provenance: PolicyProvenanceV2 =
        prepare_policy_provenance(session, EvaluationMode::Development, Some(selected), None)?;
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
    let admitted: AdmittedEvidenceV2 = admit_evidence_bytes(
        checked.raw_bytes(),
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

/// Loads an actual kind-compatible declaration before projecting its identities.
fn kind_fixture(kind: EvidenceKind) -> Result<tempfile::TempDir, Box<dyn Error>> {
    let repository = example()?;
    let path = repository
        .path()
        .join("eqm/bindings/android.auth_signup.toml");
    let original = fs::read_to_string(&path)?;
    let prefix = original
        .split_once("[[evidence]]")
        .ok_or("evidence declaration")?
        .0;
    let runner = if matches!(
        kind,
        EvidenceKind::StructuralCheck | EvidenceKind::Test | EvidenceKind::Snapshot
    ) {
        "runner = \"runner.android\"\n"
    } else {
        ""
    };
    let selector = match kind {
        EvidenceKind::Test => {
            "[evidence.selector]\nkind = \"test\"\nframework = \"junit\"\ntest_id = \"signupIdentifierDefaultsToEmail\"\n"
        }
        EvidenceKind::Snapshot => {
            "[evidence.selector]\nkind = \"snapshot\"\nsnapshot_id = \"signup.identifier\"\n"
        }
        EvidenceKind::StructuralCheck
        | EvidenceKind::StaticInventory
        | EvidenceKind::RuntimeSnapshot => {
            "[evidence.selector]\nkind = \"inventory\"\nrecord_type = \"surface\"\nkey = \"signup.identifier\"\n"
        }
        EvidenceKind::ReleaseRecord => {
            "[evidence.selector]\nkind = \"release\"\nchannel = \"development\"\n"
        }
        EvidenceKind::ManualReview => "",
    };
    fs::write(
        path,
        format!(
            "{prefix}[[evidence]]\nid = \"identifier_behavior\"\nkind = \"{kind}\"\nrequirements = [\"account.create.signup.identifier#email_default\"]\nfacets = [\"behavior\"]\n{runner}\n{selector}"
        ),
    )?;
    Ok(repository)
}

fn record_for_kind(
    provenance: &Provenance,
    kind: EvidenceKind,
) -> Result<(EvidenceResultV2Dto, EvidenceAdmissionContext), Box<dyn Error>> {
    let mut value = record(provenance, None)?;
    let mut expected = context(provenance, None)?;
    let EvidencePayloadV2Dto::Test { execution } = value.payload else {
        return Err("execution fixture".into());
    };
    let counts = execution.counts.clone();
    let reference = Sha256Digest::hash_content(b"synthetic referenced record");
    value.kind = kind.to_string();
    expected.kind = kind;
    if !matches!(
        kind,
        EvidenceKind::StructuralCheck | EvidenceKind::Test | EvidenceKind::Snapshot
    ) {
        value.runner_digest = None;
        expected.runner_digest = None;
    }
    value.payload = match kind {
        EvidenceKind::StructuralCheck => EvidencePayloadV2Dto::StructuralCheck { execution },
        EvidenceKind::Test => EvidencePayloadV2Dto::Test { execution },
        EvidenceKind::Snapshot => EvidencePayloadV2Dto::Snapshot { execution },
        EvidenceKind::StaticInventory => EvidencePayloadV2Dto::StaticInventory {
            inventory_digest: reference.to_string(),
            counts,
        },
        EvidenceKind::RuntimeSnapshot => {
            value.runtime_facts_digest = Some(reference.to_string());
            expected.runtime_facts_digest = Some(reference);
            EvidencePayloadV2Dto::RuntimeSnapshot {
                runtime_facts_digest: reference.to_string(),
                counts,
            }
        }
        EvidenceKind::ReleaseRecord => {
            value.release_record_digest = Some(reference.to_string());
            expected.release_record_digest = Some(reference);
            EvidencePayloadV2Dto::ReleaseRecord {
                release_record_digest: reference.to_string(),
            }
        }
        EvidenceKind::ManualReview => EvidencePayloadV2Dto::ManualReview {
            outcome: "passed".to_owned(),
            reviewer: "owner://team/reviewers".to_owned(),
            message: Some("Synthetic review assertion; no independent verification".to_owned()),
        },
    };
    Ok((value, expected))
}

fn file_snapshot(root: &Path) -> Result<BTreeMap<std::path::PathBuf, Vec<u8>>, Box<dyn Error>> {
    fn visit(
        root: &Path,
        path: &Path,
        files: &mut BTreeMap<std::path::PathBuf, Vec<u8>>,
    ) -> Result<(), Box<dyn Error>> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                visit(root, &entry.path(), files)?;
            } else {
                files.insert(
                    entry.path().strip_prefix(root)?.to_owned(),
                    fs::read(entry.path())?,
                );
            }
        }
        Ok(())
    }
    let mut files = BTreeMap::new();
    visit(root, root, &mut files)?;
    Ok(files)
}

#[test]
fn application_converts_all_seven_real_kind_projections_without_effects()
-> Result<(), Box<dyn Error>> {
    let kinds = [
        EvidenceKind::StructuralCheck,
        EvidenceKind::StaticInventory,
        EvidenceKind::Test,
        EvidenceKind::Snapshot,
        EvidenceKind::ManualReview,
        EvidenceKind::RuntimeSnapshot,
        EvidenceKind::ReleaseRecord,
    ];
    assert_eq!(kinds.len(), 7);
    for kind in kinds {
        let repository = kind_fixture(kind)?;
        let session = session_for(repository.path())?;
        let provenance = projections(&session)?;
        let spec = session
            .finalized()
            .graph()
            .bindings()
            .get(&BindingId::new("binding.android.auth_signup")?)
            .ok_or("binding")?
            .evidence()
            .get(&EvidenceSpecId::new("identifier_behavior")?)
            .ok_or("spec")?;
        assert_eq!(spec.kind(), kind);
        assert_eq!(
            canonicalize_evidence_spec_v2(spec)?.digest(),
            provenance.evidence_spec_digest
        );
        let (value, expected) = record_for_kind(&provenance, kind)?;
        let checked = seal(value)?;
        fs::create_dir_all(repository.path().join(".eqm/results"))?;
        fs::write(
            repository.path().join(".eqm/results/poison.json"),
            b"malformed evidence must never be read",
        )?;
        let before = file_snapshot(repository.path())?;
        let admitted =
            admit_evidence_bytes(checked.raw_bytes(), &lineage()?, &execution()?, &expected)?;
        let direct = admit_evidence(&checked, &lineage()?, &execution()?, &expected)?;
        assert_eq!(admitted, direct, "{kind}");
        assert_eq!(admitted.result().kind(), kind);
        assert_eq!(
            admitted.result().claimed_trust(),
            eqm_domain::TrustLevel::SignedCi
        );
        assert_eq!(admitted.raw_bytes(), checked.raw_bytes());
        assert_eq!(admitted.lineage(), &lineage()?);
        assert_eq!(admitted.execution(), &execution()?);
        let pretty = serde_json::to_vec_pretty(checked.record())?;
        assert_ne!(pretty, checked.raw_bytes());
        let alternate = admit_evidence_bytes(&pretty, &lineage()?, &execution()?, &expected)?;
        assert_eq!(alternate.semantic_identity(), admitted.semantic_identity());
        assert_eq!(alternate.result(), admitted.result());
        assert_eq!(alternate.raw_bytes(), pretty);
        assert_eq!(file_snapshot(repository.path())?, before);
    }
    Ok(())
}

#[test]
fn application_execution_count_outcome_and_chronology_contradictions_reject()
-> Result<(), Box<dyn Error>> {
    for kind in [
        EvidenceKind::StructuralCheck,
        EvidenceKind::Test,
        EvidenceKind::Snapshot,
    ] {
        let repository = kind_fixture(kind)?;
        let session = session_for(repository.path())?;
        let provenance = projections(&session)?;
        let (value, expected) = record_for_kind(&provenance, kind)?;
        for outcome in [
            "failed",
            "skipped",
            "filtered",
            "quarantined",
            "timed_out",
            "cancelled",
        ] {
            let mut mutant = value.clone();
            let execution = match &mut mutant.payload {
                EvidencePayloadV2Dto::StructuralCheck { execution }
                | EvidencePayloadV2Dto::Test { execution }
                | EvidencePayloadV2Dto::Snapshot { execution } => execution,
                _ => return Err("executable payload required".into()),
            };
            execution.attempts[0].outcome = outcome.to_owned();
            let checked = seal(mutant)?;
            assert_eq!(
                admit_evidence_bytes(
                    checked.raw_bytes(),
                    &lineage()?,
                    &self::execution()?,
                    &expected
                ),
                Err(EvidenceConversionError::Admission(
                    EvidenceAdmissionError::CountOutcomeContradiction
                )),
                "{kind} {outcome}"
            );
        }
        let mut mutant = value.clone();
        let execution_payload = match &mut mutant.payload {
            EvidencePayloadV2Dto::StructuralCheck { execution }
            | EvidencePayloadV2Dto::Test { execution }
            | EvidencePayloadV2Dto::Snapshot { execution } => execution,
            _ => return Err("executable payload required".into()),
        };
        // Both windows are inside the aggregate and have canonical nanoseconds,
        // but retry two begins one nanosecond before the first finishes.
        execution_payload.attempts[0].finished_at = instant("2026-01-02T03:04:05.000000002Z")?;
        let mut retry = execution_payload.attempts[0].clone();
        retry.number = WireUint::new(2);
        retry.started_at = instant("2026-01-02T03:04:05.000000001Z")?;
        execution_payload.attempts.push(retry);
        let checked = seal(mutant)?;
        assert_eq!(
            admit_evidence_bytes(checked.raw_bytes(), &lineage()?, &execution()?, &expected),
            Err(EvidenceConversionError::Admission(
                EvidenceAdmissionError::RetryChronology
            ))
        );
    }
    Ok(())
}

#[test]
fn application_one_axis_relationships_use_the_same_checked_conversion() -> Result<(), Box<dyn Error>>
{
    let repository = kind_fixture(EvidenceKind::Test)?;
    let session = session_for(repository.path())?;
    let provenance = projections(&session)?;
    let (value, expected) = record_for_kind(&provenance, EvidenceKind::Test)?;
    let mut cases = Vec::new();
    macro_rules! mutant {
        ($name:literal, $error:expr, $change:expr) => {{
            let mut next = value.clone();
            $change(&mut next);
            cases.push(($name, next, $error));
        }};
    }
    mutant!(
        "kind",
        EvidenceAdmissionError::KindMismatch,
        |v: &mut EvidenceResultV2Dto| v.kind = "snapshot".to_owned()
    );
    mutant!(
        "repository",
        EvidenceAdmissionError::SubjectMismatch,
        |v: &mut EvidenceResultV2Dto| v.subject.repository =
            "https://example.com/team/other".to_owned()
    );
    mutant!(
        "repository identity",
        EvidenceAdmissionError::SubjectMismatch,
        |v: &mut EvidenceResultV2Dto| v.subject.repository_id_digest =
            Sha256Digest::hash_content(b"other repository").to_string()
    );
    mutant!(
        "commit",
        EvidenceAdmissionError::SubjectMismatch,
        |v: &mut EvidenceResultV2Dto| v.subject.source_commit = "b".repeat(40)
    );
    mutant!(
        "build",
        EvidenceAdmissionError::SubjectMismatch,
        |v: &mut EvidenceResultV2Dto| v.subject.build_id = Some("build.other".to_owned())
    );
    mutant!(
        "artifact",
        EvidenceAdmissionError::SubjectMismatch,
        |v: &mut EvidenceResultV2Dto| v.subject.artifact_digest =
            Some(Sha256Digest::hash_content(b"other artifact").to_string())
    );
    mutant!(
        "configuration",
        EvidenceAdmissionError::SubjectMismatch,
        |v: &mut EvidenceResultV2Dto| v.subject.target_configuration_digest =
            TargetConfigurationDigest::hash_content(b"other configuration").to_string()
    );
    mutant!(
        "scope",
        EvidenceAdmissionError::SubjectMismatch,
        |v: &mut EvidenceResultV2Dto| v.subject.scope = ScopeSubjectV2Dto::Target {
            target: "ios".to_owned()
        }
    );
    mutant!(
        "target",
        EvidenceAdmissionError::TargetMismatch,
        |v: &mut EvidenceResultV2Dto| v.target = "ios".to_owned()
    );
    mutant!(
        "unit",
        EvidenceAdmissionError::UnitMismatch,
        |v: &mut EvidenceResultV2Dto| v.unit = "other.unit".to_owned()
    );
    mutant!(
        "specification",
        EvidenceAdmissionError::DigestMismatch,
        |v: &mut EvidenceResultV2Dto| v.evidence_spec_digest =
            EvidenceSpecificationDigest::hash_content(b"other specification").to_string()
    );
    mutant!(
        "contract",
        EvidenceAdmissionError::DigestMismatch,
        |v: &mut EvidenceResultV2Dto| v.contract_digest =
            ContractDigest::hash_content(b"other contract").to_string()
    );
    mutant!(
        "binding",
        EvidenceAdmissionError::DigestMismatch,
        |v: &mut EvidenceResultV2Dto| v.binding_digest =
            BindingDigest::hash_content(b"other binding").to_string()
    );
    mutant!(
        "policy",
        EvidenceAdmissionError::DigestMismatch,
        |v: &mut EvidenceResultV2Dto| v.policy_digest =
            PolicySelectionDigest::hash_content(b"other policy").to_string()
    );
    mutant!(
        "runner",
        EvidenceAdmissionError::DigestMismatch,
        |v: &mut EvidenceResultV2Dto| v.runner_digest = None
    );
    mutant!(
        "adapter",
        EvidenceAdmissionError::DigestMismatch,
        |v: &mut EvidenceResultV2Dto| v.adapter_digest =
            Some(AdapterDigest::hash_content(b"other adapter").to_string())
    );
    mutant!(
        "runtime envelope",
        EvidenceAdmissionError::DigestMismatch,
        |v: &mut EvidenceResultV2Dto| v.runtime_facts_digest =
            Some(Sha256Digest::hash_content(b"unexpected runtime").to_string())
    );
    mutant!(
        "release envelope",
        EvidenceAdmissionError::DigestMismatch,
        |v: &mut EvidenceResultV2Dto| v.release_record_digest =
            Some(Sha256Digest::hash_content(b"unexpected release").to_string())
    );
    let mut facet = value.clone();
    facet.facets = CanonicalSet::from_members(vec!["structure".to_owned()])?;
    cases.push(("facet", facet, EvidenceAdmissionError::FacetMismatch));
    let mut requirement = value.clone();
    requirement.requirements =
        CanonicalSet::from_members(vec!["account.create.signup.identifier#other".to_owned()])?;
    cases.push((
        "requirement",
        requirement,
        EvidenceAdmissionError::RequirementMismatch,
    ));
    let mut owner = value.clone();
    owner.requirements = CanonicalSet::from_members(vec!["other.unit#email_default".to_owned()])?;
    cases.push((
        "requirement owner",
        owner,
        EvidenceAdmissionError::RequirementMismatch,
    ));
    for (label, mutate) in [("profile revision", true), ("profile value", false)] {
        let mut profile = value.clone();
        profile.profile_values = CanonicalSet::from_members(vec![ProfileValueV2Dto {
            profile: "audience.default".to_owned(),
            revision: WireUint::new(if mutate { 2 } else { 1 }),
            dimension: "region".to_owned(),
            value: if mutate { "eu" } else { "us" }.to_owned(),
        }])?;
        cases.push((label, profile, EvidenceAdmissionError::ProfileMismatch));
    }
    for (label, mutant, error) in cases {
        // A fresh self-consistent semantic hash cannot repair an unrelated axis.
        let checked = seal(mutant)?;
        let application =
            admit_evidence_bytes(checked.raw_bytes(), &lineage()?, &execution()?, &expected);
        assert_eq!(
            application,
            Err(EvidenceConversionError::Admission(error)),
            "{label}"
        );
        assert_eq!(
            application,
            admit_evidence(&checked, &lineage()?, &execution()?, &expected)
                .map_err(EvidenceConversionError::Admission),
            "{label} parity"
        );
    }
    let mut wrong_payload = value.clone();
    let EvidencePayloadV2Dto::Test { execution: payload } = wrong_payload.payload else {
        return Err("test payload required".into());
    };
    wrong_payload.payload = EvidencePayloadV2Dto::Snapshot { execution: payload };
    let wrong_payload = seal(wrong_payload)?;
    assert_eq!(
        admit_evidence_bytes(
            wrong_payload.raw_bytes(),
            &lineage()?,
            &execution()?,
            &expected
        ),
        Err(EvidenceConversionError::Admission(
            EvidenceAdmissionError::Domain(
                eqm_domain::EvidenceResultBuildError::PayloadKindMismatch
            )
        ))
    );
    let checked = seal(value)?;
    for (authority, error) in [
        (
            ExpectedLineageAuthority::new(
                PlanId::new("plan.other")?,
                CampaignId::new("campaign.signup")?,
            ),
            eqm_domain::EvidenceResultBuildError::PlanMismatch,
        ),
        (
            ExpectedLineageAuthority::new(
                PlanId::new("plan.release")?,
                CampaignId::new("campaign.other")?,
            ),
            eqm_domain::EvidenceResultBuildError::CampaignMismatch,
        ),
    ] {
        let mut context = expected.clone();
        context.expected_lineage = authority;
        assert_eq!(
            admit_evidence_bytes(checked.raw_bytes(), &lineage()?, &execution()?, &context),
            Err(EvidenceConversionError::Admission(
                EvidenceAdmissionError::Lineage(error)
            ))
        );
    }
    for (run, case, attempt, error) in [
        (
            "run.other",
            "case.identifier",
            "attempt.one",
            eqm_domain::EvidenceResultBuildError::RunMismatch,
        ),
        (
            "run.one",
            "case.other",
            "attempt.one",
            eqm_domain::EvidenceResultBuildError::LogicalCaseMismatch,
        ),
        (
            "run.one",
            "case.identifier",
            "attempt.other",
            eqm_domain::EvidenceResultBuildError::AttemptNotDeclared,
        ),
    ] {
        let declared = EvidenceExecution::new(
            RunId::new(run)?,
            LogicalCaseId::new(case)?,
            vec![eqm_domain::AttemptId::new(attempt)?],
        )?;
        assert_eq!(
            admit_evidence_bytes(checked.raw_bytes(), &lineage()?, &declared, &expected),
            Err(EvidenceConversionError::Admission(
                EvidenceAdmissionError::Lineage(error)
            ))
        );
    }
    Ok(())
}

#[test]
fn application_reference_identity_and_decode_errors_remain_distinct() -> Result<(), Box<dyn Error>>
{
    for (kind, error) in [
        (
            EvidenceKind::RuntimeSnapshot,
            EvidenceAdmissionError::RuntimeReferenceMismatch,
        ),
        (
            EvidenceKind::ReleaseRecord,
            EvidenceAdmissionError::ReleaseReferenceMismatch,
        ),
    ] {
        let repository = kind_fixture(kind)?;
        let session = session_for(repository.path())?;
        let provenance = projections(&session)?;
        let (mut value, expected) = record_for_kind(&provenance, kind)?;
        let other = Sha256Digest::hash_content(b"other referenced record").to_string();
        match &mut value.payload {
            EvidencePayloadV2Dto::RuntimeSnapshot {
                runtime_facts_digest,
                ..
            } => *runtime_facts_digest = other,
            EvidencePayloadV2Dto::ReleaseRecord {
                release_record_digest,
            } => *release_record_digest = other,
            _ => return Err("referenced payload".into()),
        }
        let checked = seal(value)?;
        assert_eq!(
            admit_evidence_bytes(checked.raw_bytes(), &lineage()?, &execution()?, &expected),
            Err(EvidenceConversionError::Admission(error))
        );
    }
    let repository = example()?;
    let session = session_for(repository.path())?;
    let provenance = projections(&session)?;
    let expected = context(&provenance, None)?;
    let checked = seal(record(&provenance, None)?)?;
    let mut identity = checked.record().clone();
    identity.id = Sha256Digest::hash_content(b"unrelated but equal declared identity").to_string();
    identity.result_digest = identity.id.clone();
    let bytes = serde_json::to_vec(&identity)?;
    assert_eq!(
        admit_evidence_bytes(&bytes, &lineage()?, &execution()?, &expected),
        Err(EvidenceConversionError::Admission(
            EvidenceAdmissionError::IdentityMismatch
        ))
    );
    let mut unsupported = checked.record().clone();
    unsupported.schema = unsupported.schema.replace("/v2/", "/v1/");
    let unsupported = serde_json::to_vec(&unsupported)?;
    let too_large = vec![b' '; eqm_protocol::MAX_JSON_BYTES + 1];
    for bytes in [
        b"{".as_slice(),
        unsupported.as_slice(),
        too_large.as_slice(),
    ] {
        let Err(error @ EvidenceConversionError::Decode(_)) =
            admit_evidence_bytes(bytes, &lineage()?, &execution()?, &expected)
        else {
            return Err("strict decoding must fail before admission".into());
        };
        let direct =
            CheckedEvidenceResultV2::from_bytes(bytes).map_err(EvidenceConversionError::Decode);
        assert_eq!(direct.err(), Some(error));
        assert!(error.source().is_some());
    }
    let error = EvidenceConversionError::Admission(EvidenceAdmissionError::IdentityMismatch);
    assert!(error.source().is_some());
    Ok(())
}

fn coverage_context(
    context: &EvidenceAdmissionContext,
    spec: &EvidenceSpecId,
) -> CoverageExpectation {
    CoverageExpectation {
        evidence_spec_id: spec.clone(),
        evidence_spec_digest: context.evidence_spec_digest.as_digest(),
        requirement: context.requirement.clone(),
        facet: Facet::Behavior,
        subject: context.subject.clone(),
        target: context.target.clone(),
        unit: context.unit.clone(),
        kind: context.kind,
        contract_digest: context.contract_digest.as_digest(),
        binding_digest: context.binding_digest.as_digest(),
        policy_digest: context.policy_digest.as_digest(),
        runner_digest: context.runner_digest.map(RunnerDigest::as_digest),
        adapter_digest: context.adapter_digest.map(AdapterDigest::as_digest),
        runtime_facts_digest: context.runtime_facts_digest,
        profiles: context.profiles.clone(),
        release_record_digest: context.release_record_digest,
    }
}

fn complete_coordinates(
    session: &eqm_application::PreparedSession,
    region: &str,
) -> Result<
    (
        eqm_engine::ObligationKey,
        eqm_domain::ObligationDigest,
        eqm_domain::ExpectedSelectionManifest,
    ),
    Box<dyn Error>,
> {
    let request = eqm_application::PolicySelectionRequest::development(
        Some(eqm_application::PolicyReference::new(
            PolicyId::new("consumer.critical_flow")?,
            Some(Revision::new(1)?),
        )),
        vec![eqm_application::ProfileSelection::new(
            ProfileId::new("audience.default")?,
            Some(Revision::new(1)?),
            vec![(
                DimensionId::new("region")?,
                Some(SymbolicValueId::new(region)?),
            )],
        )],
    );
    let prepared = eqm_application::prepare_obligations_from_request(
        session.finalized(),
        EvaluationMode::Development,
        &request,
        None,
    )?;
    let target = eqm_engine::ScopeSubject::Target(TargetId::new("android")?);
    let key = prepared
        .universe()
        .coordinates()
        .iter()
        .find(|key| {
            key.requirement.as_str() == "account.create.signup.identifier#email_default"
                && key.facet == Facet::Behavior
                && key.subject == target
        })
        .ok_or("independent target obligation missing")?
        .clone();
    let binding = session
        .finalized()
        .graph()
        .bindings()
        .get(&BindingId::new("binding.android.auth_signup")?)
        .ok_or("binding missing")?;
    let specification = binding
        .evidence()
        .get(&EvidenceSpecId::new("identifier_behavior")?)
        .ok_or("specification missing")?;
    let selected = eqm_domain::ExpectedSelectionManifest::new(
        eqm_domain::ExpectedSelectionSource {
            binding: binding.id().clone(),
            binding_revision: binding.revision(),
            binding_digest: canonicalize_binding_v2(binding)?.digest(),
            specification: specification.id().clone(),
            specification_digest: canonicalize_evidence_spec_v2(specification)?.digest(),
            plan: lineage()?.plan().clone(),
            campaign: lineage()?.campaign().clone(),
        },
        specification.kind(),
        eqm_domain::SelectionCountUnit::LogicalTestCase,
        vec![lineage()?.logical_case().clone()],
        vec![],
        false,
    )?;
    let expected = eqm_application::prepare_expected_selections(
        session.finalized(),
        &prepared,
        Some(BTreeMap::from([(key.clone(), vec![selected.clone()])])),
        None,
    )?;
    let identities =
        eqm_application::prepare_obligation_identities(session.finalized(), &expected)?;
    let identity = identities
        .identities()
        .get(&key)
        .ok_or("complete reference missing")?
        .digest();
    Ok((key, identity, selected))
}

#[test]
fn complete_matching_uses_real_pre_result_context_and_rejects_claim_substitution()
-> Result<(), Box<dyn Error>> {
    use eqm_application::{CompleteEvidenceDelivery, match_admitted_evidence};
    use eqm_engine::{CompleteCoverageExpectation, CompleteCoverageMismatch};
    let repository = example()?;
    let session = session_for(repository.path())?;
    let provenance = projections(&session)?;
    let context = context(&provenance, None)?;
    let spec = EvidenceSpecId::new("identifier_behavior")?;
    let (key, identity, selection) = complete_coordinates(&session, "eu")?;
    let expected = CompleteCoverageExpectation {
        structural: coverage_context(&context, &spec),
        coordinate: key.clone(),
        identity,
        selection,
        subjects: None,
    };
    // Independent expectations exist before constructing or reading any result.
    let value = seal(record(&provenance, None)?)?;
    let admitted = admit_evidence_bytes(value.raw_bytes(), &lineage()?, &execution()?, &context)?;
    let pretty = serde_json::to_vec_pretty(value.record())?;
    let alternate = admit_evidence_bytes(&pretty, &lineage()?, &execution()?, &context)?;
    let receipt = SelectorText::new("receipt.complete")?;
    let other_receipt = SelectorText::new("receipt.alternate")?;
    let delivery = CompleteEvidenceDelivery {
        delivery: AdmittedEvidenceDelivery {
            admitted: &admitted,
            evidence_spec_id: &spec,
            provenance: &receipt,
        },
        coordinate: Some(&key),
        identity: Some(identity),
        subjects: None,
    };
    let other = CompleteEvidenceDelivery {
        delivery: AdmittedEvidenceDelivery {
            admitted: &alternate,
            evidence_spec_id: &spec,
            provenance: &other_receipt,
        },
        ..delivery
    };
    fs::create_dir_all(repository.path().join(".eqm/results"))?;
    fs::write(
        repository.path().join(".eqm/results/poison.json"),
        b"untrusted poison",
    )?;
    let before = file_snapshot(repository.path())?;
    let accepted = match_admitted_evidence(&expected, &[delivery, other, delivery])?;
    assert_eq!(accepted.status(), CoverageStatus::Covered);
    assert_eq!(
        accepted.eligible(),
        &BTreeSet::from([(admitted.semantic_identity(), lineage()?)])
    );
    assert!(accepted.rejected().is_empty());
    assert!(accepted.conflicts().is_empty());
    assert_eq!(accepted.normalized().attempts().len(), 1);
    let assertion = accepted
        .normalized()
        .attempts()
        .values()
        .next()
        .ok_or("attempt missing")?
        .assertions()
        .values()
        .next()
        .ok_or("assertion missing")?;
    assert_eq!(
        assertion
            .deliveries()
            .iter()
            .map(|r| r.raw_identity())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([admitted.raw_identity(), alternate.raw_identity()])
    );
    assert_eq!(
        match_admitted_evidence(&expected, &[other, delivery, other])?,
        accepted
    );
    let (foreign_key, foreign_identity, _) = complete_coordinates(&session, "us")?;
    for (claim, axis) in [
        (
            CompleteEvidenceDelivery {
                coordinate: None,
                ..delivery
            },
            CompleteCoverageMismatch::Coordinate,
        ),
        (
            CompleteEvidenceDelivery {
                coordinate: Some(&foreign_key),
                ..delivery
            },
            CompleteCoverageMismatch::Coordinate,
        ),
        (
            CompleteEvidenceDelivery {
                identity: None,
                ..delivery
            },
            CompleteCoverageMismatch::ObligationIdentity,
        ),
        (
            CompleteEvidenceDelivery {
                identity: Some(foreign_identity),
                ..delivery
            },
            CompleteCoverageMismatch::ObligationIdentity,
        ),
    ] {
        let rejected = match_admitted_evidence(&expected, &[claim, claim])?;
        assert_eq!(rejected.status(), CoverageStatus::Missing);
        assert!(rejected.eligible().is_empty());
        assert!(rejected.conflicts().is_empty());
        assert_eq!(
            rejected.rejected()[&admitted.result().id()],
            BTreeSet::from([axis])
        );
        let mixed = match_admitted_evidence(&expected, &[claim, delivery, claim])?;
        assert_eq!(mixed.status(), CoverageStatus::Covered);
        assert_eq!(mixed.eligible(), accepted.eligible());
        assert!(mixed.conflicts().is_empty());
        assert_eq!(
            match_admitted_evidence(&expected, &[delivery, claim, claim])?,
            mixed
        );
    }
    assert_eq!(
        match_admitted_evidence(&expected, &[])?.status(),
        CoverageStatus::Missing
    );
    assert_eq!(file_snapshot(repository.path())?, before);
    Ok(())
}

#[test]
fn complete_matching_rejects_fresh_admitted_build_runner_and_profile() -> Result<(), Box<dyn Error>>
{
    use eqm_application::{CompleteEvidenceDelivery, match_admitted_evidence};
    use eqm_engine::{CompleteCoverageExpectation, CompleteCoverageMismatch};
    let repository = example()?;
    let session = session_for(repository.path())?;
    let original = projections(&session)?;
    let expected_context = context(&original, None)?;
    let spec = EvidenceSpecId::new("identifier_behavior")?;
    let (key, identity, selection) = complete_coordinates(&session, "eu")?;
    let expected = CompleteCoverageExpectation {
        structural: coverage_context(&expected_context, &spec),
        coordinate: key.clone(),
        identity,
        selection,
        subjects: None,
    };
    let checked = seal(record(&original, None)?)?;
    let admitted = admit_evidence_bytes(
        checked.raw_bytes(),
        &lineage()?,
        &execution()?,
        &expected_context,
    )?;
    let receipt = SelectorText::new("receipt.original")?;
    let fresh_receipt = SelectorText::new("receipt.fresh")?;
    let base = CompleteEvidenceDelivery {
        delivery: AdmittedEvidenceDelivery {
            admitted: &admitted,
            evidence_spec_id: &spec,
            provenance: &receipt,
        },
        coordinate: Some(&key),
        identity: Some(identity),
        subjects: None,
    };
    let before = file_snapshot(repository.path())?;
    // Explicit fresh build context, independently supplied to admission.
    let mut build_context = expected_context.clone();
    let subject = &expected_context.subject;
    build_context.subject = EvidenceSubject::new(
        subject.repository().clone(),
        subject.repository_id_digest(),
        subject.scope().clone(),
        subject.source_commit().clone(),
        Some(SelectorText::new("build.fresh")?),
        subject.artifact_digest(),
        subject.target_configuration_digest(),
    );
    let mut build_record = record(&original, None)?;
    build_record.subject.build_id = Some("build.fresh".to_owned());
    let build_checked = seal(build_record)?;
    let build = admit_evidence_bytes(
        build_checked.raw_bytes(),
        &lineage()?,
        &execution()?,
        &build_context,
    )?;
    // A real independently prepared selected profile produces its own policy
    // identity and complete pre-result obligation, rather than relabeling a tag.
    let us_request = PolicyProfileRequest::new(
        AuthorityOrigin::CandidateLocal,
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
    let us = projections_for_selection(&session, &us_request)?;
    assert_ne!(us.policy_digest, original.policy_digest);
    let (us_key, us_identity, _) = complete_coordinates(&session, "us")?;
    let mut us_context = context(&us, None)?;
    let profile_id = ProfileId::new("audience.default")?;
    us_context.profiles = BTreeMap::from([(
        profile_id.clone(),
        ProfileSelection::new(
            profile_id,
            Revision::new(1)?,
            vec![(DimensionId::new("region")?, SymbolicValueId::new("us")?)],
        )?,
    )]);
    let mut us_record = record(&us, None)?;
    us_record.profile_values = CanonicalSet::from_members(vec![ProfileValueV2Dto {
        profile: "audience.default".to_owned(),
        revision: WireUint::new(1),
        dimension: "region".to_owned(),
        value: "us".to_owned(),
    }])?;
    let us_checked = seal(us_record)?;
    let profiled = admit_evidence_bytes(
        us_checked.raw_bytes(),
        &lineage()?,
        &execution()?,
        &us_context,
    )?;
    for (fresh, coordinate, claim_identity, axes) in [
        (
            &build,
            &key,
            identity,
            BTreeSet::from([CompleteCoverageMismatch::Structural(
                CoverageMismatch::Build,
            )]),
        ),
        (
            &profiled,
            &us_key,
            us_identity,
            BTreeSet::from([
                CompleteCoverageMismatch::Coordinate,
                CompleteCoverageMismatch::ObligationIdentity,
                CompleteCoverageMismatch::Structural(CoverageMismatch::Profiles),
                CompleteCoverageMismatch::Structural(CoverageMismatch::PolicyDigest),
            ]),
        ),
    ] {
        let delivery = CompleteEvidenceDelivery {
            delivery: AdmittedEvidenceDelivery {
                admitted: fresh,
                evidence_spec_id: &spec,
                provenance: &fresh_receipt,
            },
            coordinate: Some(coordinate),
            identity: Some(claim_identity),
            subjects: None,
        };
        let rejected = match_admitted_evidence(&expected, &[delivery, delivery])?;
        assert_eq!(rejected.status(), CoverageStatus::Missing);
        assert!(rejected.eligible().is_empty());
        assert_eq!(rejected.rejected()[&fresh.result().id()], axes);
        let mixed = match_admitted_evidence(&expected, &[delivery, base, delivery])?;
        assert_eq!(mixed.status(), CoverageStatus::Covered);
        assert_eq!(
            mixed.eligible(),
            &BTreeSet::from([(admitted.semantic_identity(), lineage()?)])
        );
        assert!(mixed.conflicts().is_empty());
        assert_eq!(
            match_admitted_evidence(&expected, &[base, delivery])?,
            mixed
        );
    }
    assert_eq!(file_snapshot(repository.path())?, before);
    // Mutate the actual runner definition, reload it, and resolve its acquisition
    // identity through the production runner path against the fixture executable.
    let path = repository.path().join("eqm/runners/runner.android.toml");
    fs::write(
        &path,
        fs::read_to_string(&path)?.replace("timeout_ms = 60000", "timeout_ms = 59000"),
    )?;
    let fresh_session = session_for(repository.path())?;
    let runner_provenance = projections(&fresh_session)?;
    assert_ne!(runner_provenance.runner_digest, original.runner_digest);
    let runner_context = context(&runner_provenance, None)?;
    let runner_checked = seal(record(&runner_provenance, None)?)?;
    let runner = admit_evidence_bytes(
        runner_checked.raw_bytes(),
        &lineage()?,
        &execution()?,
        &runner_context,
    )?;
    let delivery = CompleteEvidenceDelivery {
        delivery: AdmittedEvidenceDelivery {
            admitted: &runner,
            evidence_spec_id: &spec,
            provenance: &fresh_receipt,
        },
        ..base
    };
    let rejected = match_admitted_evidence(&expected, &[delivery])?;
    assert_eq!(rejected.status(), CoverageStatus::Missing);
    assert!(rejected.eligible().is_empty());
    assert_eq!(
        rejected.rejected()[&runner.result().id()],
        BTreeSet::from([CompleteCoverageMismatch::Structural(
            CoverageMismatch::RunnerDigest
        )])
    );
    let mixed = match_admitted_evidence(&expected, &[delivery, base, delivery])?;
    assert_eq!(mixed.status(), CoverageStatus::Covered);
    assert_eq!(
        mixed.eligible(),
        &BTreeSet::from([(admitted.semantic_identity(), lineage()?)])
    );
    assert!(mixed.conflicts().is_empty());
    assert_eq!(
        match_admitted_evidence(&expected, &[base, delivery])?,
        mixed
    );
    let mut after = before;
    after.insert(
        path.strip_prefix(repository.path())?.to_owned(),
        fs::read(path)?,
    );
    assert_eq!(file_snapshot(repository.path())?, after);
    Ok(())
}

#[test]
fn real_admitted_delivery_normalization_is_idempotent_and_conflicts_are_relevant()
-> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let session = session_for(repository.path())?;
    let provenance = projections(&session)?;
    let context = context(&provenance, None)?;
    let (coordinate, identity, selection) = complete_coordinates(&session, "eu")?;
    let complete_expected = eqm_engine::CompleteCoverageExpectation {
        structural: coverage_context(&context, &EvidenceSpecId::new("identifier_behavior")?),
        coordinate: coordinate.clone(),
        identity,
        selection,
        subjects: None,
    };
    let value = record(&provenance, None)?;
    let checked = seal(value.clone())?;
    let passed = admit_evidence_bytes(checked.raw_bytes(), &lineage()?, &execution()?, &context)?;
    let pretty = serde_json::to_vec_pretty(checked.record())?;
    let alternate = admit_evidence_bytes(&pretty, &lineage()?, &execution()?, &context)?;
    assert_eq!(
        passed.raw_identity(),
        eqm_domain::RawObjectDigest::hash_content(checked.raw_bytes())
    );
    assert_eq!(
        alternate.raw_identity(),
        eqm_domain::RawObjectDigest::hash_content(&pretty)
    );
    assert_ne!(passed.raw_identity(), alternate.raw_identity());
    assert_eq!(passed.semantic_identity(), alternate.semantic_identity());
    let spec = EvidenceSpecId::new("identifier_behavior")?;
    let receipt1 = SelectorText::new("receipt.one")?;
    let receipt2 = SelectorText::new("receipt.two")?;
    let receipt3 = SelectorText::new("receipt.three")?;
    let receipts = [
        AdmittedEvidenceDelivery {
            admitted: &passed,
            evidence_spec_id: &spec,
            provenance: &receipt1,
        },
        AdmittedEvidenceDelivery {
            admitted: &alternate,
            evidence_spec_id: &spec,
            provenance: &receipt2,
        },
        AdmittedEvidenceDelivery {
            admitted: &passed,
            evidence_spec_id: &spec,
            provenance: &receipt3,
        },
    ];
    fs::create_dir_all(repository.path().join(".eqm/results"))?;
    fs::write(
        repository.path().join(".eqm/results/poison.json"),
        b"not evidence input",
    )?;
    let before = file_snapshot(repository.path())?;
    let normalized = normalize_admitted_evidence(&receipts)?;
    assert_eq!(normalized.attempts().len(), 1);
    let (key, attempt) = normalized
        .attempts()
        .first_key_value()
        .ok_or("attempt group")?;
    assert_eq!(key.subject(), &context.subject);
    assert_eq!(key.lineage(), &lineage()?);
    assert!(!attempt.has_conflict());
    assert_eq!(attempt.assertions().len(), 1);
    let assertion = attempt
        .assertions()
        .get(&passed.semantic_identity())
        .ok_or("semantic assertion")?;
    assert_eq!(assertion.deliveries().len(), 3);
    assert_eq!(
        assertion
            .deliveries()
            .iter()
            .map(|x| x.raw_identity())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([passed.raw_identity(), alternate.raw_identity()])
    );
    assert_eq!(
        assertion
            .deliveries()
            .iter()
            .map(|x| x.provenance().clone())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([receipt1.clone(), receipt2.clone(), receipt3.clone()])
    );
    let expected = coverage_context(&context, &spec);
    let coverage = evaluate_normalized_evidence_coverage(&expected, &normalized);
    assert_eq!(coverage.coverage.status, CoverageStatus::Covered);
    assert_eq!(coverage.coverage.covered.len(), 1);
    assert!(coverage.conflicting_attempts.is_empty());
    for order in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        let repeated = [
            receipts[order[0]],
            receipts[order[1]],
            receipts[order[2]],
            receipts[order[0]],
        ];
        let permuted = normalize_admitted_evidence(&repeated)?;
        assert_eq!(permuted, normalized);
        assert_eq!(
            evaluate_normalized_evidence_coverage(&expected, &permuted),
            coverage
        );
    }
    let mut failed = value;
    let EvidencePayloadV2Dto::Test { execution: payload } = &mut failed.payload else {
        return Err("test payload".into());
    };
    payload.attempts[0].outcome = "failed".to_owned();
    payload.counts.passed = WireUint::new(0);
    payload.counts.failed = WireUint::new(1);
    let failed = seal(failed)?;
    let failed = admit_evidence_bytes(failed.raw_bytes(), &lineage()?, &execution()?, &context)?;
    let failed_receipt = SelectorText::new("receipt.failed")?;
    let failed_delivery = AdmittedEvidenceDelivery {
        admitted: &failed,
        evidence_spec_id: &spec,
        provenance: &failed_receipt,
    };
    let conflicts =
        normalize_admitted_evidence(&[receipts[0], failed_delivery, receipts[1], failed_delivery])?;
    let conflict_report = evaluate_normalized_evidence_coverage(&expected, &conflicts);
    assert_eq!(conflicts.attempts().len(), 1);
    assert!(
        conflicts
            .attempts()
            .values()
            .all(|x| x.has_conflict() && x.assertions().len() == 2)
    );
    assert_eq!(conflict_report.coverage.status, CoverageStatus::Unknown);
    assert!(conflict_report.coverage.covered.is_empty());
    assert_eq!(conflict_report.conflicting_attempts.len(), 1);
    assert_eq!(
        normalize_admitted_evidence(&[failed_delivery, receipts[1], receipts[0]])?,
        conflicts
    );
    let complete_base = eqm_application::CompleteEvidenceDelivery {
        delivery: receipts[0],
        coordinate: Some(&coordinate),
        identity: Some(identity),
        subjects: None,
    };
    let complete_failed = eqm_application::CompleteEvidenceDelivery {
        delivery: failed_delivery,
        ..complete_base
    };
    let complete_conflict = eqm_application::match_admitted_evidence(
        &complete_expected,
        &[complete_base, complete_failed, complete_failed],
    )?;
    assert_eq!(complete_conflict.status(), CoverageStatus::Unknown);
    assert!(complete_conflict.eligible().is_empty());
    assert_eq!(
        complete_conflict.conflicts(),
        &BTreeSet::from([(passed.result().subject().clone(), lineage()?)])
    );
    assert_eq!(
        eqm_application::match_admitted_evidence(
            &complete_expected,
            &[complete_failed, complete_base]
        )?,
        complete_conflict
    );
    // A genuinely different prepared binding/specification is rejected before
    // conflict relevance, while every delivery remains in the audit grouping.
    assert_eq!(file_snapshot(repository.path())?, before);
    let path = repository
        .path()
        .join("eqm/bindings/android.auth_signup.toml");
    fs::write(
        &path,
        fs::read_to_string(&path)?.replace("signupIdentifierDefaultsToEmail", "otherSelection"),
    )?;
    let other_session = session_for(repository.path())?;
    let other_provenance = projections(&other_session)?;
    assert_ne!(other_provenance.binding_digest, provenance.binding_digest);
    assert_ne!(
        other_provenance.evidence_spec_digest,
        provenance.evidence_spec_digest
    );
    let other_checked = seal(record(&other_provenance, None)?)?;
    let other = admit_evidence_bytes(
        other_checked.raw_bytes(),
        &lineage()?,
        &execution()?,
        &self::context(&other_provenance, None)?,
    )?;
    let other_receipt = SelectorText::new("receipt.unrelated")?;
    let other_delivery = AdmittedEvidenceDelivery {
        admitted: &other,
        evidence_spec_id: &spec,
        provenance: &other_receipt,
    };
    let complete_other = eqm_application::CompleteEvidenceDelivery {
        delivery: other_delivery,
        ..complete_base
    };
    let complete = eqm_application::match_admitted_evidence(
        &complete_expected,
        &[complete_other, complete_base, complete_other],
    )?;
    assert_eq!(complete.status(), CoverageStatus::Covered);
    assert_eq!(
        complete.eligible(),
        &BTreeSet::from([(passed.semantic_identity(), lineage()?)])
    );
    assert!(complete.conflicts().is_empty());
    assert_eq!(
        complete.rejected()[&other.result().id()],
        BTreeSet::from([
            eqm_engine::CompleteCoverageMismatch::Structural(CoverageMismatch::BindingDigest),
            eqm_engine::CompleteCoverageMismatch::Structural(CoverageMismatch::EvidenceSpecDigest),
        ])
    );
    assert_eq!(
        eqm_application::match_admitted_evidence(
            &complete_expected,
            &[complete_base, complete_other]
        )?,
        complete
    );
    let unrelated = normalize_admitted_evidence(&[other_delivery, receipts[0], other_delivery])?;
    let unrelated_report = evaluate_normalized_evidence_coverage(&expected, &unrelated);
    assert!(unrelated.attempts().values().all(|x| x.has_conflict()));
    assert_eq!(unrelated_report.coverage.status, CoverageStatus::Covered);
    assert_eq!(
        unrelated_report.coverage.covered,
        BTreeSet::from([passed.result().id()])
    );
    assert!(unrelated_report.conflicting_attempts.is_empty());
    assert!(
        unrelated_report.coverage.rejected[&other.result().id()]
            .contains(&CoverageMismatch::BindingDigest)
    );
    assert!(
        unrelated_report.coverage.rejected[&other.result().id()]
            .contains(&CoverageMismatch::EvidenceSpecDigest)
    );
    // Check the no-effects assertion before the deliberate fixture edit, and
    // again against the exact state after it. Neither normalization reads poison.
    let mut expected_before = before;
    expected_before.insert(
        path.strip_prefix(repository.path())?.to_owned(),
        fs::read(&path)?,
    );
    assert_eq!(file_snapshot(repository.path())?, expected_before);
    let at_bound = vec![receipts[0]; MAX_CANDIDATE_DELIVERIES];
    assert_eq!(normalize_admitted_evidence(&at_bound)?.attempts().len(), 1);
    let over_bound = vec![receipts[0]; MAX_CANDIDATE_DELIVERIES + 1];
    assert_eq!(
        normalize_admitted_evidence(&over_bound),
        Err(CandidateNormalizationError::LimitExceeded)
    );
    assert!(normalize_admitted_evidence(&[])?.attempts().is_empty());
    Ok(())
}

#[test]
fn admitted_normalization_keeps_each_lineage_axis_and_full_subject_independent()
-> Result<(), Box<dyn Error>> {
    let repository = example()?;
    let session = session_for(repository.path())?;
    let provenance = projections(&session)?;
    let context = context(&provenance, None)?;
    let spec = EvidenceSpecId::new("identifier_behavior")?;
    let (key, identity, selection) = complete_coordinates(&session, "eu")?;
    let complete_expected = eqm_engine::CompleteCoverageExpectation {
        structural: coverage_context(&context, &spec),
        coordinate: key.clone(),
        identity,
        selection,
        subjects: None,
    };
    let before = file_snapshot(repository.path())?;
    let checked = seal(record(&provenance, None)?)?;
    let base = admit_evidence_bytes(checked.raw_bytes(), &lineage()?, &execution()?, &context)?;
    let spec = EvidenceSpecId::new("identifier_behavior")?;
    let base_receipt = SelectorText::new("receipt.base")?;
    let changed_receipt = SelectorText::new("receipt.changed")?;
    let base_delivery = AdmittedEvidenceDelivery {
        admitted: &base,
        evidence_spec_id: &spec,
        provenance: &base_receipt,
    };
    let expected = coverage_context(&context, &spec);
    for (plan, campaign, run, case, attempt) in [
        (
            "plan.other",
            "campaign.signup",
            "run.one",
            "case.identifier",
            "attempt.one",
        ),
        (
            "plan.release",
            "campaign.other",
            "run.one",
            "case.identifier",
            "attempt.one",
        ),
        (
            "plan.release",
            "campaign.signup",
            "run.other",
            "case.identifier",
            "attempt.one",
        ),
        (
            "plan.release",
            "campaign.signup",
            "run.one",
            "case.other",
            "attempt.one",
        ),
        (
            "plan.release",
            "campaign.signup",
            "run.one",
            "case.identifier",
            "attempt.other",
        ),
    ] {
        let lineage = EvidenceLineage::from_parts(EvidenceLineageParts {
            plan: Some(PlanId::new(plan)?),
            campaign: Some(CampaignId::new(campaign)?),
            run: Some(RunId::new(run)?),
            logical_case: Some(LogicalCaseId::new(case)?),
            attempt: Some(eqm_domain::AttemptId::new(attempt)?),
        })?;
        let execution = EvidenceExecution::new(
            RunId::new(run)?,
            LogicalCaseId::new(case)?,
            vec![eqm_domain::AttemptId::new(attempt)?],
        )?;
        let mut independent = context.clone();
        independent.expected_lineage =
            ExpectedLineageAuthority::new(PlanId::new(plan)?, CampaignId::new(campaign)?);
        let other = admit_evidence_bytes(checked.raw_bytes(), &lineage, &execution, &independent)?;
        let normalized = normalize_admitted_evidence(&[
            base_delivery,
            AdmittedEvidenceDelivery {
                admitted: &other,
                evidence_spec_id: &spec,
                provenance: &changed_receipt,
            },
        ])?;
        assert_eq!(
            normalized.attempts().len(),
            2,
            "{plan}/{campaign}/{run}/{case}/{attempt}"
        );
        assert!(normalized.attempts().values().all(|x| !x.has_conflict()));
        assert!(
            normalized
                .attempts()
                .keys()
                .any(|x| x.lineage() == &lineage)
        );
        let base_complete = eqm_application::CompleteEvidenceDelivery {
            delivery: base_delivery,
            coordinate: Some(&key),
            identity: Some(identity),
            subjects: None,
        };
        let other_complete = eqm_application::CompleteEvidenceDelivery {
            delivery: AdmittedEvidenceDelivery {
                admitted: &other,
                evidence_spec_id: &spec,
                provenance: &changed_receipt,
            },
            ..base_complete
        };
        let complete = eqm_application::match_admitted_evidence(
            &complete_expected,
            &[other_complete, base_complete, other_complete],
        )?;
        assert_eq!(complete.status(), CoverageStatus::Covered);
        assert!(complete.conflicts().is_empty());
        let rejected_axis = if plan != "plan.release" {
            Some(eqm_engine::CompleteCoverageMismatch::Plan)
        } else if campaign != "campaign.signup" {
            Some(eqm_engine::CompleteCoverageMismatch::Campaign)
        } else if case != "case.identifier" {
            Some(eqm_engine::CompleteCoverageMismatch::LogicalCase)
        } else {
            None
        };
        if let Some(axis) = rejected_axis {
            assert_eq!(
                complete.eligible(),
                &BTreeSet::from([(base.semantic_identity(), self::lineage()?)])
            );
            assert_eq!(
                complete.rejected()[&other.result().id()],
                BTreeSet::from([axis])
            );
            assert_eq!(
                eqm_application::match_admitted_evidence(&complete_expected, &[other_complete])?
                    .status(),
                CoverageStatus::Missing
            );
        } else {
            assert_eq!(
                complete.eligible(),
                &BTreeSet::from([
                    (base.semantic_identity(), self::lineage()?),
                    (other.semantic_identity(), lineage.clone())
                ])
            );
            assert!(complete.rejected().is_empty());
        }
        assert_eq!(
            eqm_application::match_admitted_evidence(
                &complete_expected,
                &[base_complete, other_complete]
            )?,
            complete
        );
        let report = evaluate_normalized_evidence_coverage(&expected, &normalized);
        assert_eq!(report.coverage.status, CoverageStatus::Covered);
        assert!(report.conflicting_attempts.is_empty());
    }
    let mut changed = checked.record().clone();
    let artifact = Sha256Digest::hash_content(b"different synthetic artifact");
    changed.subject.artifact_digest = Some(artifact.to_string());
    let mut independent = context.clone();
    let subject = &context.subject;
    independent.subject = EvidenceSubject::new(
        subject.repository().clone(),
        subject.repository_id_digest(),
        subject.scope().clone(),
        subject.source_commit().clone(),
        subject.build_id().cloned(),
        Some(artifact),
        subject.target_configuration_digest(),
    );
    let changed = seal(changed)?;
    let other = admit_evidence_bytes(
        changed.raw_bytes(),
        &lineage()?,
        &execution()?,
        &independent,
    )?;
    let normalized = normalize_admitted_evidence(&[
        base_delivery,
        AdmittedEvidenceDelivery {
            admitted: &other,
            evidence_spec_id: &spec,
            provenance: &changed_receipt,
        },
    ])?;
    assert_eq!(normalized.attempts().len(), 2);
    assert!(normalized.attempts().values().all(|x| !x.has_conflict()));
    assert_eq!(
        normalized
            .attempts()
            .keys()
            .map(|x| x.subject().artifact_digest())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([subject.artifact_digest(), Some(artifact)])
    );
    let base_complete = eqm_application::CompleteEvidenceDelivery {
        delivery: base_delivery,
        coordinate: Some(&key),
        identity: Some(identity),
        subjects: None,
    };
    let other_complete = eqm_application::CompleteEvidenceDelivery {
        delivery: AdmittedEvidenceDelivery {
            admitted: &other,
            evidence_spec_id: &spec,
            provenance: &changed_receipt,
        },
        ..base_complete
    };
    let complete = eqm_application::match_admitted_evidence(
        &complete_expected,
        &[other_complete, base_complete],
    )?;
    assert_eq!(complete.status(), CoverageStatus::Covered);
    assert_eq!(
        complete.eligible(),
        &BTreeSet::from([(base.semantic_identity(), lineage()?)])
    );
    assert_eq!(
        complete.rejected()[&other.result().id()],
        BTreeSet::from([eqm_engine::CompleteCoverageMismatch::Structural(
            CoverageMismatch::Artifact
        )])
    );
    assert!(complete.conflicts().is_empty());
    assert_eq!(file_snapshot(repository.path())?, before);
    Ok(())
}
