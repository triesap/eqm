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
#[derive(Clone)]
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
    complete_coordinates_with_selection(
        session,
        region,
        None,
        vec![lineage()?.logical_case().clone()],
        vec![],
        false,
    )
}

// Explicit test capability freezes alternatives before any evidence is read.
struct ObservationSelectionAuthority<'a> {
    graph: &'a eqm_engine::FinalizedWorkspaceGraph,
    scopes: eqm_domain::RequirementScenarioBindings,
    selections: eqm_engine::ExpectedSelectionMap,
}
impl eqm_application::AuthorityProvider for ObservationSelectionAuthority<'_> {
    fn load_authority(
        &self,
        _: &eqm_application::AuthorityRequest,
    ) -> Result<eqm_application::ProtectedAuthority<'_>, eqm_application::AuthorityError> {
        Ok(eqm_application::ProtectedAuthority::new(
            self.graph,
            &self.scopes,
        ))
    }
}
impl eqm_application::ExpectedSelectionProvider for ObservationSelectionAuthority<'_> {
    fn load_expected_selections(
        &self,
        _: &eqm_application::AuthorityRequest,
    ) -> Result<eqm_engine::ExpectedSelectionMap, eqm_application::AuthorityError> {
        Ok(self.selections.clone())
    }
}

fn complete_coordinates_with_selection(
    session: &eqm_application::PreparedSession,
    region: &str,
    release_context: Option<Sha256Digest>,
    cases: Vec<LogicalCaseId>,
    alternatives: Vec<eqm_domain::FrozenSelectionAlternative>,
    independent_principals: bool,
) -> Result<
    (
        eqm_engine::ObligationKey,
        eqm_domain::ObligationDigest,
        eqm_domain::ExpectedSelectionManifest,
    ),
    Box<dyn Error>,
> {
    let (prepared, key, selection) = prepared_coordinates_with_selection(
        session,
        region,
        release_context,
        cases,
        alternatives,
        independent_principals,
    )?;
    let identity = prepared
        .identities()
        .get(&key)
        .ok_or("complete reference missing")?
        .digest();
    Ok((key, identity, selection))
}

fn prepared_coordinates_with_selection(
    session: &eqm_application::PreparedSession,
    region: &str,
    release_context: Option<Sha256Digest>,
    cases: Vec<LogicalCaseId>,
    alternatives: Vec<eqm_domain::FrozenSelectionAlternative>,
    independent_principals: bool,
) -> Result<
    (
        eqm_application::PreparedObligationIdentities,
        eqm_engine::ObligationKey,
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
    let protected = !alternatives.is_empty();
    let authority = ObservationSelectionAuthority {
        graph: session.finalized(),
        scopes: eqm_domain::RequirementScenarioBindings::new(vec![])?,
        selections: BTreeMap::new(),
    };
    let prepared = if protected {
        eqm_application::prepare_protected_obligations(
            session.finalized(),
            EvaluationMode::Release,
            &request,
            Some(&authority),
            release_context,
        )?
    } else {
        eqm_application::prepare_obligations_from_request(
            session.finalized(),
            EvaluationMode::Development,
            &request,
            release_context,
        )?
    };
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
        match specification.kind() {
            EvidenceKind::Test => eqm_domain::SelectionCountUnit::LogicalTestCase,
            EvidenceKind::StructuralCheck => eqm_domain::SelectionCountUnit::StructuralFact,
            EvidenceKind::StaticInventory => eqm_domain::SelectionCountUnit::InventoryFact,
            EvidenceKind::Snapshot => eqm_domain::SelectionCountUnit::Snapshot,
            EvidenceKind::RuntimeSnapshot => eqm_domain::SelectionCountUnit::RuntimeFact,
            EvidenceKind::ManualReview => eqm_domain::SelectionCountUnit::ReviewSlot,
            EvidenceKind::ReleaseRecord => eqm_domain::SelectionCountUnit::VerifiedRecord,
        },
        cases,
        alternatives,
        independent_principals,
    )?;
    let claims = BTreeMap::from([(key.clone(), vec![selected.clone()])]);
    let selection_authority = ObservationSelectionAuthority {
        graph: session.finalized(),
        scopes: eqm_domain::RequirementScenarioBindings::new(vec![])?,
        selections: claims.clone(),
    };
    let expected = eqm_application::prepare_expected_selections(
        session.finalized(),
        &prepared,
        Some(claims),
        protected
            .then_some(&selection_authority as &dyn eqm_application::ExpectedSelectionProvider),
    )?;
    let identities =
        eqm_application::prepare_obligation_identities(session.finalized(), &expected)?;
    Ok((identities, key, selected))
}

#[test]
fn actual_matched_split_reports_and_ten_retries_preserve_missing_selected_cases()
-> Result<(), Box<dyn Error>> {
    use eqm_application::{CompleteEvidenceDelivery, collect_admitted_observations};
    use eqm_engine::{CompleteCoverageExpectation, LogicalObservationPrerequisite};
    let repository = example()?;
    let session = session_for(repository.path())?;
    let provenance = projections(&session)?;
    let context = context(&provenance, None)?;
    let spec = EvidenceSpecId::new("identifier_behavior")?;
    let a = lineage()?.logical_case().clone();
    let b = LogicalCaseId::new("case.second")?;
    let (key, identity, selected) = complete_coordinates_with_selection(
        &session,
        "eu",
        None,
        vec![a.clone(), b.clone()],
        vec![],
        false,
    )?;
    let expected = vec![CompleteCoverageExpectation {
        structural: coverage_context(&context, &spec),
        coordinate: key.clone(),
        identity,
        selection: selected.clone(),
        subjects: None,
    }];
    let checked = seal(record(&provenance, None)?)?;
    let receipt = SelectorText::new("receipt.selection")?;
    let admitted = (0..10)
        .map(|i| {
            let attempt = eqm_domain::AttemptId::new(format!("attempt.retry_{i}"))?;
            let lineage = EvidenceLineage::from_parts(EvidenceLineageParts {
                plan: Some(lineage()?.plan().clone()),
                campaign: Some(lineage()?.campaign().clone()),
                run: Some(RunId::new("run.one")?),
                logical_case: Some(a.clone()),
                attempt: Some(attempt.clone()),
            })?;
            let execution =
                EvidenceExecution::new(lineage.run().clone(), a.clone(), vec![attempt])?;
            admit_evidence_bytes(checked.raw_bytes(), &lineage, &execution, &context)
                .map_err(Box::<dyn Error>::from)
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    let deliveries = admitted
        .iter()
        .map(|value| CompleteEvidenceDelivery {
            delivery: AdmittedEvidenceDelivery {
                admitted: value,
                evidence_spec_id: &spec,
                provenance: &receipt,
            },
            coordinate: Some(&key),
            identity: Some(identity),
            subjects: None,
        })
        .collect::<Vec<_>>();
    fs::create_dir_all(repository.path().join(".eqm/results"))?;
    fs::write(
        repository.path().join(".eqm/results/poison.json"),
        b"invalid poison",
    )?;
    let before = file_snapshot(repository.path())?;
    let missing = collect_admitted_observations(&expected, &deliveries)?;
    assert_eq!(missing.status(), CoverageStatus::Missing);
    assert_eq!(missing.observed(), &BTreeSet::from([a.clone()]));
    assert_eq!(
        missing.units(),
        &BTreeSet::from([eqm_engine::LogicalObservationUnit::TestCase(a.clone())])
    );
    assert_eq!(
        missing.missing(),
        &BTreeMap::from([(selected.source().clone(), BTreeSet::from([b.clone()]))])
    );
    assert_eq!(
        missing.matching()[selected.source()]
            .normalized()
            .attempts()
            .len(),
        10
    );
    assert_eq!(
        missing.prerequisites(),
        &BTreeSet::from([
            LogicalObservationPrerequisite::IndependentTrust,
            LogicalObservationPrerequisite::ExecutionOutcomes
        ])
    );
    let at_limit = collect_admitted_observations(&expected, &vec![deliveries[0]; 10_000])?;
    assert_eq!(at_limit.status(), CoverageStatus::Missing);
    assert_eq!(at_limit.observed(), missing.observed());
    assert_eq!(at_limit.units(), missing.units());
    assert_eq!(at_limit.missing(), missing.missing());
    assert_eq!(
        collect_admitted_observations(&expected, &vec![deliveries[0]; 10_001]),
        Err(eqm_engine::LogicalObservationError::Matching(
            eqm_engine::CompleteCoverageError::Normalization(
                eqm_engine::CandidateNormalizationError::LimitExceeded
            )
        ))
    );
    let mut repeated = deliveries.clone();
    repeated.extend(deliveries.iter().rev().copied());
    assert_eq!(
        collect_admitted_observations(&expected, &repeated)?,
        missing
    );
    let pretty = serde_json::to_vec_pretty(checked.record())?;
    let alternate = admit_evidence_bytes(
        &pretty,
        admitted[0].lineage(),
        &EvidenceExecution::new(
            admitted[0].lineage().run().clone(),
            a.clone(),
            vec![admitted[0].lineage().attempt().clone()],
        )?,
        &context,
    )?;
    repeated.push(CompleteEvidenceDelivery {
        delivery: AdmittedEvidenceDelivery {
            admitted: &alternate,
            evidence_spec_id: &spec,
            provenance: &receipt,
        },
        ..deliveries[0]
    });
    let alternate_report = collect_admitted_observations(&expected, &repeated)?;
    assert_eq!(alternate_report.observed(), missing.observed());
    assert_eq!(alternate_report.missing(), missing.missing());
    assert!(
        alternate_report.matching()[selected.source()]
            .normalized()
            .attempts()
            .values()
            .any(|attempt| attempt
                .assertions()
                .values()
                .any(|assertion| assertion.deliveries().len() == 2))
    );
    let second_lineage = EvidenceLineage::from_parts(EvidenceLineageParts {
        plan: Some(lineage()?.plan().clone()),
        campaign: Some(lineage()?.campaign().clone()),
        run: Some(RunId::new("run.second")?),
        logical_case: Some(b.clone()),
        attempt: Some(eqm_domain::AttemptId::new("attempt.second")?),
    })?;
    let second = admit_evidence_bytes(
        checked.raw_bytes(),
        &second_lineage,
        &EvidenceExecution::new(
            second_lineage.run().clone(),
            b.clone(),
            vec![second_lineage.attempt().clone()],
        )?,
        &context,
    )?;
    repeated.push(CompleteEvidenceDelivery {
        delivery: AdmittedEvidenceDelivery {
            admitted: &second,
            evidence_spec_id: &spec,
            provenance: &receipt,
        },
        ..deliveries[0]
    });
    let complete = collect_admitted_observations(&expected, &repeated)?;
    assert_eq!(complete.status(), CoverageStatus::Covered);
    assert_eq!(
        complete.units(),
        &BTreeSet::from([
            eqm_engine::LogicalObservationUnit::TestCase(a.clone()),
            eqm_engine::LogicalObservationUnit::TestCase(b.clone())
        ])
    );
    assert_eq!(complete.observed(), &BTreeSet::from([a, b]));
    assert!(complete.missing().values().all(BTreeSet::is_empty));
    repeated.reverse();
    assert_eq!(
        collect_admitted_observations(&expected, &repeated)?,
        complete
    );
    assert_eq!(
        collect_admitted_observations(&expected, &[])?.status(),
        CoverageStatus::Missing
    );
    assert_eq!(file_snapshot(repository.path())?, before);
    Ok(())
}

#[test]
fn actual_frozen_alternative_is_not_selected_from_observed_passes() -> Result<(), Box<dyn Error>> {
    use eqm_application::{CompleteEvidenceDelivery, collect_admitted_observations};
    use eqm_domain::FrozenSelectionAlternative;
    use eqm_engine::{
        CompleteCoverageExpectation, CompleteCoverageMismatch, LogicalObservationUnit,
    };
    let repository = example()?;
    let session = session_for(repository.path())?;
    let provenance = projections(&session)?;
    let context = context(&provenance, None)?;
    let spec = EvidenceSpecId::new("identifier_behavior")?;
    let required = lineage()?.logical_case().clone();
    let chosen = LogicalCaseId::new("case.chosen")?;
    let unselected = LogicalCaseId::new("case.unselected")?;
    let alternative = FrozenSelectionAlternative::new(
        LogicalCaseId::new("group.choice")?,
        vec![
            (LogicalCaseId::new("option.a")?, vec![chosen.clone()]),
            (LogicalCaseId::new("option.b")?, vec![unselected.clone()]),
        ],
        LogicalCaseId::new("option.a")?,
    )?;
    let (prepared, key, selection) = prepared_coordinates_with_selection(
        &session,
        "eu",
        None,
        vec![required.clone()],
        vec![alternative],
        false,
    )?;
    let identity = prepared.identities().get(&key).ok_or("identity")?.digest();
    let expected = vec![CompleteCoverageExpectation {
        structural: coverage_context(&context, &spec),
        coordinate: key.clone(),
        identity,
        selection: selection.clone(),
        subjects: None,
    }];
    // The exact chosen option and full semantic identity exist before any result.
    let checked = seal(record(&provenance, None)?)?;
    let mut admissions = Vec::new();
    for case in [&required, &chosen, &unselected] {
        let lineage = EvidenceLineage::from_parts(EvidenceLineageParts {
            plan: Some(lineage()?.plan().clone()),
            campaign: Some(lineage()?.campaign().clone()),
            run: Some(lineage()?.run().clone()),
            logical_case: Some(case.clone()),
            attempt: Some(eqm_domain::AttemptId::new("attempt.choice")?),
        })?;
        let execution = EvidenceExecution::new(
            lineage.run().clone(),
            case.clone(),
            vec![lineage.attempt().clone()],
        )?;
        admissions.push(admit_evidence_bytes(
            checked.raw_bytes(),
            &lineage,
            &execution,
            &context,
        )?);
    }
    let receipt = SelectorText::new("receipt.choice")?;
    let deliveries = admissions
        .iter()
        .map(|admitted| CompleteEvidenceDelivery {
            delivery: AdmittedEvidenceDelivery {
                admitted,
                evidence_spec_id: &spec,
                provenance: &receipt,
            },
            coordinate: Some(&key),
            identity: Some(identity),
            subjects: None,
        })
        .collect::<Vec<_>>();
    let before = file_snapshot(repository.path())?;
    let missing =
        collect_admitted_observations(&expected, &[deliveries[2], deliveries[0], deliveries[2]])?;
    assert_eq!(missing.status(), CoverageStatus::Missing);
    assert_eq!(
        missing.units(),
        &BTreeSet::from([LogicalObservationUnit::TestCase(required.clone())])
    );
    assert_eq!(
        missing.missing()[selection.source()],
        BTreeSet::from([chosen.clone()])
    );
    assert_eq!(
        missing.matching()[selection.source()].rejected()[&admissions[2].result().id()],
        BTreeSet::from([CompleteCoverageMismatch::LogicalCase])
    );
    let complete = collect_admitted_observations(
        &expected,
        &[deliveries[2], deliveries[1], deliveries[0], deliveries[2]],
    )?;
    assert_eq!(complete.status(), CoverageStatus::Covered);
    assert_eq!(
        complete.units(),
        &BTreeSet::from([
            LogicalObservationUnit::TestCase(required),
            LogicalObservationUnit::TestCase(chosen)
        ])
    );
    assert!(!complete.observed().contains(&unselected));
    assert!(complete.conflicts().is_empty());
    assert!(complete.missing().values().all(BTreeSet::is_empty));
    assert_eq!(
        collect_admitted_observations(&expected, &[deliveries[0], deliveries[1], deliveries[2]])?,
        complete
    );
    let count_missing = eqm_application::assess_admitted_count_constraints(
        &prepared,
        &expected,
        &[deliveries[0], deliveries[2]],
    )?;
    assert_eq!(
        count_missing.status(),
        eqm_engine::LogicalCountConstraintStatus::Missing
    );
    assert_eq!(count_missing.capacity(), 1);
    assert_eq!(count_missing.observations(), &missing);
    let count =
        eqm_application::assess_admitted_count_constraints(&prepared, &expected, &deliveries)?;
    assert_eq!(
        count.status(),
        eqm_engine::LogicalCountConstraintStatus::CapacityMet
    );
    assert_eq!(count.capacity(), 2);
    assert_eq!(count.observations(), &complete);
    assert_eq!(file_snapshot(repository.path())?, before);
    Ok(())
}

fn prepared_multi_source_identities(
    session: &eqm_application::PreparedSession,
    key: &eqm_engine::ObligationKey,
    release_context: Option<Sha256Digest>,
    manifests: Vec<eqm_domain::ExpectedSelectionManifest>,
) -> Result<eqm_application::PreparedObligationIdentities, Box<dyn Error>> {
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
                Some(SymbolicValueId::new("eu")?),
            )],
        )],
    );
    let prepared = eqm_application::prepare_obligations_from_request(
        session.finalized(),
        EvaluationMode::Development,
        &request,
        release_context,
    )?;
    let selections = eqm_application::prepare_expected_selections(
        session.finalized(),
        &prepared,
        Some(BTreeMap::from([(key.clone(), manifests)])),
        None,
    )?;
    Ok(eqm_application::prepare_obligation_identities(
        session.finalized(),
        &selections,
    )?)
}

#[test]
fn actual_reference_aliases_preserve_two_required_labels_but_one_record_unit()
-> Result<(), Box<dyn Error>> {
    use eqm_application::{CompleteEvidenceDelivery, collect_admitted_observations};
    use eqm_domain::{ExpectedSelectionManifest, SelectionCountUnit};
    use eqm_engine::{
        CompleteCoverageExpectation, LogicalObservationPrerequisite, LogicalObservationUnit,
    };
    let repository = kind_fixture(EvidenceKind::ReleaseRecord)?;
    let path = repository
        .path()
        .join("eqm/bindings/android.auth_signup.toml");
    let mut text = fs::read_to_string(&path)?;
    text.push_str(
        r#"
[[evidence]]
id = "second_record"
kind = "release_record"
requirements = ["account.create.signup.identifier#email_default"]
facets = ["behavior"]
[evidence.selector]
kind = "release"
channel = "development"
"#,
    );
    fs::write(&path, text)?;
    let session = session_for(repository.path())?;
    let original = projections(&session)?;
    let (first_record, first_context) = record_for_kind(&original, EvidenceKind::ReleaseRecord)?;
    let first_spec = EvidenceSpecId::new("identifier_behavior")?;
    let second_spec = EvidenceSpecId::new("second_record")?;
    let (key, _, first_selection) = complete_coordinates_with_selection(
        &session,
        "eu",
        first_context.release_record_digest,
        vec![lineage()?.logical_case().clone()],
        vec![],
        false,
    )?;
    let binding = session
        .finalized()
        .graph()
        .bindings()
        .get(&BindingId::new("binding.android.auth_signup")?)
        .ok_or("binding")?;
    let mut second_provenance = original.clone();
    second_provenance.evidence_spec_digest = canonicalize_evidence_spec_v2(
        binding
            .evidence()
            .get(&second_spec)
            .ok_or("second specification")?,
    )?
    .digest();
    let (second_record, second_context) =
        record_for_kind(&second_provenance, EvidenceKind::ReleaseRecord)?;
    let alias = LogicalCaseId::new("case.record_alias")?;
    let mut source = first_selection.source().clone();
    source.specification = second_spec.clone();
    source.specification_digest = second_provenance.evidence_spec_digest;
    let second_selection = ExpectedSelectionManifest::new(
        source,
        EvidenceKind::ReleaseRecord,
        SelectionCountUnit::VerifiedRecord,
        vec![alias.clone()],
        vec![],
        false,
    )?;
    let prepared = prepared_multi_source_identities(
        &session,
        &key,
        first_context.release_record_digest,
        vec![first_selection.clone(), second_selection.clone()],
    )?;
    let identity = prepared.identities().get(&key).ok_or("identity")?.digest();
    let expected = vec![
        CompleteCoverageExpectation {
            structural: coverage_context(&first_context, &first_spec),
            coordinate: key.clone(),
            identity,
            selection: first_selection.clone(),
            subjects: None,
        },
        CompleteCoverageExpectation {
            structural: coverage_context(&second_context, &second_spec),
            coordinate: key.clone(),
            identity,
            selection: second_selection,
            subjects: None,
        },
    ];
    let first_checked = seal(first_record)?;
    let second_checked = seal(second_record)?;
    let first = admit_evidence_bytes(
        first_checked.raw_bytes(),
        &lineage()?,
        &execution()?,
        &first_context,
    )?;
    let alias_lineage = EvidenceLineage::from_parts(EvidenceLineageParts {
        plan: Some(lineage()?.plan().clone()),
        campaign: Some(lineage()?.campaign().clone()),
        run: Some(lineage()?.run().clone()),
        logical_case: Some(alias.clone()),
        attempt: Some(eqm_domain::AttemptId::new("attempt.record_alias")?),
    })?;
    let second = admit_evidence_bytes(
        second_checked.raw_bytes(),
        &alias_lineage,
        &EvidenceExecution::new(
            alias_lineage.run().clone(),
            alias.clone(),
            vec![alias_lineage.attempt().clone()],
        )?,
        &second_context,
    )?;
    let receipt = SelectorText::new("receipt.record_alias")?;
    let first_delivery = CompleteEvidenceDelivery {
        delivery: AdmittedEvidenceDelivery {
            admitted: &first,
            evidence_spec_id: &first_spec,
            provenance: &receipt,
        },
        coordinate: Some(&key),
        identity: Some(identity),
        subjects: None,
    };
    let second_delivery = CompleteEvidenceDelivery {
        delivery: AdmittedEvidenceDelivery {
            admitted: &second,
            evidence_spec_id: &second_spec,
            provenance: &receipt,
        },
        ..first_delivery
    };
    let before = file_snapshot(repository.path())?;
    let report = collect_admitted_observations(
        &expected,
        &[first_delivery, second_delivery, second_delivery],
    )?;
    assert_eq!(report.status(), CoverageStatus::Covered);
    assert_eq!(
        report.observed(),
        &BTreeSet::from([lineage()?.logical_case().clone(), alias])
    );
    assert_eq!(
        report.units(),
        &BTreeSet::from([LogicalObservationUnit::ReferencedRecord(
            Sha256Digest::hash_content(b"synthetic referenced record")
        )])
    );
    assert_eq!(
        report.prerequisites(),
        &BTreeSet::from([
            LogicalObservationPrerequisite::IndependentTrust,
            LogicalObservationPrerequisite::ReferencedRecord
        ])
    );
    assert!(report.missing().values().all(BTreeSet::is_empty));
    assert!(report.conflicts().is_empty());
    let count = eqm_application::assess_admitted_count_constraints(
        &prepared,
        &expected,
        &[first_delivery, second_delivery, second_delivery],
    )?;
    assert_eq!(
        count.status(),
        eqm_engine::LogicalCountConstraintStatus::CapacityMet
    );
    assert_eq!(count.capacity(), 1);
    assert_eq!(count.observations(), &report);
    assert_eq!(
        eqm_application::assess_admitted_count_constraints(
            &prepared,
            &expected,
            &[first_delivery]
        )?
        .status(),
        eqm_engine::LogicalCountConstraintStatus::Missing
    );
    let missing = collect_admitted_observations(&expected, &[first_delivery])?;
    assert_eq!(missing.status(), CoverageStatus::Missing);
    assert_eq!(missing.units(), report.units());
    let mut reverse = expected.clone();
    reverse.reverse();
    assert_eq!(
        collect_admitted_observations(&reverse, &[second_delivery, first_delivery])?,
        report
    );
    assert_eq!(file_snapshot(repository.path())?, before);
    Ok(())
}

#[test]
fn actual_selected_sources_compose_with_and_and_cannot_hide_cross_source_conflicts()
-> Result<(), Box<dyn Error>> {
    use eqm_application::{CompleteEvidenceDelivery, collect_admitted_observations};
    use eqm_domain::{ExpectedSelectionManifest, SelectionCountUnit};
    use eqm_engine::{CompleteCoverageExpectation, CompleteCoverageMismatch};
    let repository = example()?;
    let path = repository
        .path()
        .join("eqm/bindings/android.auth_signup.toml");
    let mut text = fs::read_to_string(&path)?;
    text.push_str(
        r#"
[[evidence]]
id = "second_test"
kind = "test"
requirements = ["account.create.signup.identifier#email_default"]
facets = ["behavior"]
runner = "runner.android"
[evidence.selector]
kind = "test"
framework = "junit"
test_id = "secondExactSelector"
"#,
    );
    fs::write(&path, text)?;
    let session = session_for(repository.path())?;
    let original = projections(&session)?;
    let first_context = context(&original, None)?;
    let first_spec = EvidenceSpecId::new("identifier_behavior")?;
    let second_spec = EvidenceSpecId::new("second_test")?;
    let (key, _, first_selection) = complete_coordinates(&session, "eu")?;
    let binding = session
        .finalized()
        .graph()
        .bindings()
        .get(&BindingId::new("binding.android.auth_signup")?)
        .ok_or("binding")?;
    let second_definition = binding
        .evidence()
        .get(&second_spec)
        .ok_or("second local specification")?;
    let mut second_provenance = original.clone();
    second_provenance.evidence_spec_digest =
        canonicalize_evidence_spec_v2(second_definition)?.digest();
    let second_context = context(&second_provenance, None)?;
    let mut second_source = first_selection.source().clone();
    second_source.specification = second_spec.clone();
    second_source.specification_digest = second_provenance.evidence_spec_digest;
    let second_case = LogicalCaseId::new("case.second")?;
    let second_same = ExpectedSelectionManifest::new(
        second_source.clone(),
        EvidenceKind::Test,
        SelectionCountUnit::LogicalTestCase,
        vec![lineage()?.logical_case().clone()],
        vec![],
        false,
    )?;
    let second_distinct = ExpectedSelectionManifest::new(
        second_source,
        EvidenceKind::Test,
        SelectionCountUnit::LogicalTestCase,
        vec![second_case.clone()],
        vec![],
        false,
    )?;
    // Both complete frozen identities are prepared before result bytes.
    let same_prepared = prepared_multi_source_identities(
        &session,
        &key,
        None,
        vec![first_selection.clone(), second_same.clone()],
    )?;
    let distinct_prepared = prepared_multi_source_identities(
        &session,
        &key,
        None,
        vec![first_selection.clone(), second_distinct.clone()],
    )?;
    let same_identity = same_prepared
        .identities()
        .get(&key)
        .ok_or("identity")?
        .digest();
    let distinct_identity = distinct_prepared
        .identities()
        .get(&key)
        .ok_or("identity")?
        .digest();
    assert_ne!(same_identity, distinct_identity);
    let first_expected = CompleteCoverageExpectation {
        structural: coverage_context(&first_context, &first_spec),
        coordinate: key.clone(),
        identity: same_identity,
        selection: first_selection.clone(),
        subjects: None,
    };
    let second_expected = CompleteCoverageExpectation {
        structural: coverage_context(&second_context, &second_spec),
        coordinate: key.clone(),
        identity: same_identity,
        selection: second_same.clone(),
        subjects: None,
    };
    let expected = vec![first_expected.clone(), second_expected.clone()];
    let first_checked = seal(record(&original, None)?)?;
    let second_checked = seal(record(&second_provenance, None)?)?;
    let first = admit_evidence_bytes(
        first_checked.raw_bytes(),
        &lineage()?,
        &execution()?,
        &first_context,
    )?;
    let other_attempt = eqm_domain::AttemptId::new("attempt.second_source")?;
    let second_lineage = EvidenceLineage::from_parts(EvidenceLineageParts {
        plan: Some(lineage()?.plan().clone()),
        campaign: Some(lineage()?.campaign().clone()),
        run: Some(lineage()?.run().clone()),
        logical_case: Some(lineage()?.logical_case().clone()),
        attempt: Some(other_attempt.clone()),
    })?;
    let second = admit_evidence_bytes(
        second_checked.raw_bytes(),
        &second_lineage,
        &EvidenceExecution::new(
            second_lineage.run().clone(),
            second_lineage.logical_case().clone(),
            vec![other_attempt.clone()],
        )?,
        &second_context,
    )?;
    let receipt = SelectorText::new("receipt.sources")?;
    let first_delivery = CompleteEvidenceDelivery {
        delivery: AdmittedEvidenceDelivery {
            admitted: &first,
            evidence_spec_id: &first_spec,
            provenance: &receipt,
        },
        coordinate: Some(&key),
        identity: Some(same_identity),
        subjects: None,
    };
    let second_delivery = CompleteEvidenceDelivery {
        delivery: AdmittedEvidenceDelivery {
            admitted: &second,
            evidence_spec_id: &second_spec,
            provenance: &receipt,
        },
        ..first_delivery
    };
    let before = file_snapshot(repository.path())?;
    let missing = collect_admitted_observations(&expected, &[first_delivery, first_delivery])?;
    assert_eq!(missing.status(), CoverageStatus::Missing);
    let counted_missing = eqm_application::assess_admitted_count_constraints(
        &same_prepared,
        &expected,
        &[first_delivery],
    )?;
    assert_eq!(
        counted_missing.status(),
        eqm_engine::LogicalCountConstraintStatus::Missing
    );
    assert_eq!(counted_missing.capacity(), 1);
    let missing_history = eqm_application::assess_admitted_campaign_retry_history(
        &same_prepared,
        &expected,
        &[first_delivery],
    )?;
    assert_eq!(
        missing_history.state(),
        eqm_engine::CampaignRetryState::Missing
    );
    assert_eq!(missing_history.constraints(), &counted_missing);
    assert_eq!(missing_history.source_cases().len(), 2);

    assert_eq!(
        missing.observed(),
        &BTreeSet::from([lineage()?.logical_case().clone()])
    );
    assert!(missing.missing()[first_selection.source()].is_empty());
    assert_eq!(
        missing.missing()[second_same.source()],
        BTreeSet::from([lineage()?.logical_case().clone()])
    );
    let complete = collect_admitted_observations(
        &expected,
        &[second_delivery, first_delivery, second_delivery],
    )?;
    assert_eq!(complete.status(), CoverageStatus::Covered);
    let counted = eqm_application::assess_admitted_count_constraints(
        &same_prepared,
        &expected,
        &[first_delivery, second_delivery],
    )?;
    assert_eq!(
        counted.status(),
        eqm_engine::LogicalCountConstraintStatus::CapacityMet
    );
    assert_eq!(counted.capacity(), 1);
    assert_eq!(counted.observations(), &complete);
    let history = eqm_application::assess_admitted_campaign_retry_history(
        &same_prepared,
        &expected,
        &[first_delivery, second_delivery],
    )?;
    assert_eq!(history.state(), eqm_engine::CampaignRetryState::Passed);
    assert_eq!(history.constraints(), &counted);
    assert_eq!(history.histories().len(), 1);
    assert_eq!(
        history
            .histories()
            .values()
            .next()
            .ok_or("history")?
            .attempts()
            .len(),
        2
    );
    assert_eq!(
        eqm_application::assess_admitted_campaign_retry_history(
            &same_prepared,
            &expected.iter().rev().cloned().collect::<Vec<_>>(),
            &[second_delivery, first_delivery, first_delivery]
        )?,
        history
    );

    assert_eq!(
        complete.observed(),
        &BTreeSet::from([lineage()?.logical_case().clone()])
    );
    assert!(complete.missing().values().all(BTreeSet::is_empty));
    assert!(complete.conflicts().is_empty());
    let mut reversed = expected.clone();
    reversed.reverse();
    assert_eq!(
        collect_admitted_observations(&reversed, &[first_delivery, second_delivery])?,
        complete
    );
    // Assertions claiming one immutable attempt stay conflicting even though
    // each matches a different required local specification on its own.
    let conflicting = admit_evidence_bytes(
        second_checked.raw_bytes(),
        &lineage()?,
        &execution()?,
        &second_context,
    )?;
    let conflict_delivery = CompleteEvidenceDelivery {
        delivery: AdmittedEvidenceDelivery {
            admitted: &conflicting,
            evidence_spec_id: &second_spec,
            provenance: &receipt,
        },
        ..first_delivery
    };
    let conflict = collect_admitted_observations(&expected, &[first_delivery, conflict_delivery])?;
    assert_eq!(conflict.status(), CoverageStatus::Unknown);
    let counted_conflict = eqm_application::assess_admitted_count_constraints(
        &same_prepared,
        &expected,
        &[first_delivery, conflict_delivery],
    )?;
    assert_eq!(
        counted_conflict.status(),
        eqm_engine::LogicalCountConstraintStatus::Unknown
    );
    assert_eq!(counted_conflict.capacity(), 0);
    assert_eq!(counted_conflict.observations(), &conflict);
    let conflicted_history = eqm_application::assess_admitted_campaign_retry_history(
        &same_prepared,
        &expected,
        &[first_delivery, conflict_delivery],
    )?;
    assert_eq!(
        conflicted_history.state(),
        eqm_engine::CampaignRetryState::Unknown
    );
    assert_eq!(conflicted_history.constraints(), &counted_conflict);
    assert!(
        conflicted_history
            .reasons()
            .contains(&eqm_engine::CampaignRetryReason::ImmutableConflict)
    );
    assert_eq!(
        conflicted_history
            .histories()
            .values()
            .next()
            .ok_or("history")?
            .attempts()
            .values()
            .next()
            .ok_or("attempt")?
            .len(),
        2
    );

    assert!(conflict.observed().is_empty());
    assert_eq!(conflict.conflicts().len(), 1);
    let original_lineage = lineage()?;
    assert!(
        conflict
            .conflicts()
            .iter()
            .all(|attempt| attempt.subject() == first.result().subject()
                && attempt.lineage() == &original_lineage)
    );
    assert!(
        conflict
            .matching()
            .values()
            .all(|report| report.status() == CoverageStatus::Covered
                && report.conflicts().is_empty())
    );
    assert_eq!(
        collect_admitted_observations(&reversed, &[conflict_delivery, first_delivery])?,
        conflict
    );
    let foreign_lineage = EvidenceLineage::from_parts(EvidenceLineageParts {
        plan: Some(second_lineage.plan().clone()),
        campaign: Some(CampaignId::new("campaign.foreign")?),
        run: Some(second_lineage.run().clone()),
        logical_case: Some(second_lineage.logical_case().clone()),
        attempt: Some(second_lineage.attempt().clone()),
    })?;
    let mut foreign_context = second_context.clone();
    foreign_context.expected_lineage = ExpectedLineageAuthority::new(
        foreign_lineage.plan().clone(),
        foreign_lineage.campaign().clone(),
    );
    let foreign = admit_evidence_bytes(
        second_checked.raw_bytes(),
        &foreign_lineage,
        &EvidenceExecution::new(
            foreign_lineage.run().clone(),
            foreign_lineage.logical_case().clone(),
            vec![foreign_lineage.attempt().clone()],
        )?,
        &foreign_context,
    )?;
    let foreign_delivery = CompleteEvidenceDelivery {
        delivery: AdmittedEvidenceDelivery {
            admitted: &foreign,
            evidence_spec_id: &second_spec,
            provenance: &receipt,
        },
        ..first_delivery
    };
    let mixed = collect_admitted_observations(
        &expected,
        &[
            foreign_delivery,
            first_delivery,
            second_delivery,
            foreign_delivery,
        ],
    )?;
    assert_eq!(mixed.status(), CoverageStatus::Covered);
    assert_eq!(mixed.observed(), complete.observed());
    assert!(mixed.conflicts().is_empty());
    assert_eq!(
        mixed.matching()[second_same.source()].rejected()[&foreign.result().id()],
        BTreeSet::from([CompleteCoverageMismatch::Campaign])
    );
    // Distinct selected case identities contribute two units, with every source
    // still required, even when underlying assertion bytes are repeated.
    let distinct_lineage = EvidenceLineage::from_parts(EvidenceLineageParts {
        plan: Some(second_lineage.plan().clone()),
        campaign: Some(second_lineage.campaign().clone()),
        run: Some(second_lineage.run().clone()),
        logical_case: Some(second_case.clone()),
        attempt: Some(other_attempt),
    })?;
    let distinct = admit_evidence_bytes(
        second_checked.raw_bytes(),
        &distinct_lineage,
        &EvidenceExecution::new(
            distinct_lineage.run().clone(),
            second_case.clone(),
            vec![distinct_lineage.attempt().clone()],
        )?,
        &second_context,
    )?;
    let mut distinct_expected = expected.clone();
    for value in &mut distinct_expected {
        value.identity = distinct_identity;
    }
    distinct_expected[1].selection = second_distinct;
    let first_distinct = CompleteEvidenceDelivery {
        identity: Some(distinct_identity),
        ..first_delivery
    };
    let second_distinct = CompleteEvidenceDelivery {
        delivery: AdmittedEvidenceDelivery {
            admitted: &distinct,
            evidence_spec_id: &second_spec,
            provenance: &receipt,
        },
        ..first_distinct
    };
    let distinct_report =
        collect_admitted_observations(&distinct_expected, &[second_distinct, first_distinct])?;
    assert_eq!(distinct_report.status(), CoverageStatus::Covered);
    assert_eq!(
        distinct_report.observed(),
        &BTreeSet::from([lineage()?.logical_case().clone(), second_case])
    );
    assert!(distinct_report.conflicts().is_empty());
    assert_eq!(file_snapshot(repository.path())?, before);
    Ok(())
}

#[test]
fn actual_all_kind_observations_retain_independent_prerequisites_and_ignore_totals()
-> Result<(), Box<dyn Error>> {
    use eqm_application::{CompleteEvidenceDelivery, collect_admitted_observations};
    use eqm_engine::{CompleteCoverageExpectation, LogicalObservationPrerequisite as Need};
    for (kind, needs) in [
        (
            EvidenceKind::Test,
            BTreeSet::from([Need::IndependentTrust, Need::ExecutionOutcomes]),
        ),
        (
            EvidenceKind::StructuralCheck,
            BTreeSet::from([
                Need::IndependentTrust,
                Need::ExecutionOutcomes,
                Need::StructuralFacts,
            ]),
        ),
        (
            EvidenceKind::Snapshot,
            BTreeSet::from([Need::IndependentTrust, Need::ExecutionOutcomes]),
        ),
        (
            EvidenceKind::StaticInventory,
            BTreeSet::from([Need::IndependentTrust, Need::InventoryFacts]),
        ),
        (
            EvidenceKind::RuntimeSnapshot,
            BTreeSet::from([Need::IndependentTrust, Need::RuntimeFacts]),
        ),
        (
            EvidenceKind::ManualReview,
            BTreeSet::from([
                Need::IndependentTrust,
                Need::ReviewOutcomes,
                Need::DistinctReviewPrincipals,
            ]),
        ),
        (
            EvidenceKind::ReleaseRecord,
            BTreeSet::from([Need::IndependentTrust, Need::ReferencedRecord]),
        ),
    ] {
        let repository = kind_fixture(kind)?;
        let session = session_for(repository.path())?;
        let provenance = projections(&session)?;
        let (mut value, context) = record_for_kind(&provenance, kind)?;
        // Large arithmetically valid totals do not declare 100 logical identities.
        match &mut value.payload {
            EvidencePayloadV2Dto::Test { execution }
            | EvidencePayloadV2Dto::StructuralCheck { execution }
            | EvidencePayloadV2Dto::Snapshot { execution } => {
                execution.counts.selected = WireUint::new(100);
                execution.counts.passed = WireUint::new(100);
            }
            EvidencePayloadV2Dto::StaticInventory { counts, .. }
            | EvidencePayloadV2Dto::RuntimeSnapshot { counts, .. } => {
                counts.selected = WireUint::new(100);
                counts.passed = WireUint::new(100);
            }
            EvidencePayloadV2Dto::ManualReview { .. }
            | EvidencePayloadV2Dto::ReleaseRecord { .. } => {}
        }
        let spec = EvidenceSpecId::new("identifier_behavior")?;
        let (key, identity, selection) = complete_coordinates_with_selection(
            &session,
            "eu",
            context.release_record_digest,
            vec![lineage()?.logical_case().clone()],
            vec![],
            kind == EvidenceKind::ManualReview,
        )?;
        let expected = vec![CompleteCoverageExpectation {
            structural: coverage_context(&context, &spec),
            coordinate: key.clone(),
            identity,
            selection: selection.clone(),
            subjects: None,
        }];
        let checked = seal(value)?;
        let admitted =
            admit_evidence_bytes(checked.raw_bytes(), &lineage()?, &execution()?, &context)?;
        let receipt = SelectorText::new("receipt.kind")?;
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
        let before = file_snapshot(repository.path())?;
        let report = collect_admitted_observations(&expected, &[delivery, delivery])?;
        assert_eq!(report.status(), CoverageStatus::Covered, "{kind}");
        assert_eq!(report.count_unit(), selection.count_unit());
        let case = lineage()?.logical_case().clone();
        let unit = match kind {
            EvidenceKind::Test => eqm_engine::LogicalObservationUnit::TestCase(case),
            EvidenceKind::StructuralCheck => {
                eqm_engine::LogicalObservationUnit::StructuralFact(case)
            }
            EvidenceKind::StaticInventory => {
                eqm_engine::LogicalObservationUnit::InventoryFact(case)
            }
            EvidenceKind::Snapshot => eqm_engine::LogicalObservationUnit::Snapshot(case),
            EvidenceKind::RuntimeSnapshot => eqm_engine::LogicalObservationUnit::RuntimeFact(case),
            EvidenceKind::ManualReview => eqm_engine::LogicalObservationUnit::ReviewSlot(case),
            EvidenceKind::ReleaseRecord => eqm_engine::LogicalObservationUnit::ReferencedRecord(
                Sha256Digest::hash_content(b"synthetic referenced record"),
            ),
        };
        assert_eq!(report.units(), &BTreeSet::from([unit]));

        assert_eq!(
            report.observed(),
            &BTreeSet::from([lineage()?.logical_case().clone()])
        );
        assert_eq!(report.prerequisites(), &needs);
        assert!(report.missing().values().all(BTreeSet::is_empty));
        assert_eq!(
            report.matching()[selection.source()]
                .normalized()
                .attempts()
                .len(),
            1
        );
        assert_eq!(file_snapshot(repository.path())?, before);
    }
    Ok(())
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
    let observations = eqm_application::collect_admitted_observations(
        std::slice::from_ref(&complete_expected),
        &[complete_other, complete_base, complete_other],
    )?;
    assert_eq!(observations.status(), CoverageStatus::Covered);
    assert_eq!(
        observations.observed(),
        &BTreeSet::from([lineage()?.logical_case().clone()])
    );
    assert_eq!(
        observations.units(),
        &BTreeSet::from([eqm_engine::LogicalObservationUnit::TestCase(
            lineage()?.logical_case().clone()
        )])
    );
    assert!(observations.conflicts().is_empty());
    assert_eq!(
        observations.matching()[complete_expected.selection.source()],
        complete
    );
    let foreign_only = eqm_application::collect_admitted_observations(
        std::slice::from_ref(&complete_expected),
        &[complete_other],
    )?;
    assert_eq!(foreign_only.status(), CoverageStatus::Missing);
    assert!(foreign_only.observed().is_empty());
    assert!(foreign_only.units().is_empty());
    assert_eq!(
        foreign_only.missing()[complete_expected.selection.source()],
        BTreeSet::from([lineage()?.logical_case().clone()])
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
    let observations = eqm_application::collect_admitted_observations(
        std::slice::from_ref(&complete_expected),
        &[other_complete, base_complete],
    )?;
    assert_eq!(observations.status(), CoverageStatus::Covered);
    assert_eq!(
        observations.observed(),
        &BTreeSet::from([lineage()?.logical_case().clone()])
    );
    assert!(observations.conflicts().is_empty());
    assert_eq!(
        observations.matching()[complete_expected.selection.source()],
        complete
    );
    let foreign_only = eqm_application::collect_admitted_observations(
        std::slice::from_ref(&complete_expected),
        &[other_complete],
    )?;
    assert_eq!(foreign_only.status(), CoverageStatus::Missing);
    assert!(foreign_only.units().is_empty());
    assert!(foreign_only.observed().is_empty());
    assert_eq!(file_snapshot(repository.path())?, before);
    Ok(())
}

#[test]
fn actual_prepared_count_constraints_use_strongest_minimum_and_full_and_selection()
-> Result<(), Box<dyn Error>> {
    use eqm_application::{CompleteEvidenceDelivery, assess_admitted_count_constraints};
    use eqm_engine::{
        CompleteCoverageExpectation, LogicalCountConstraintStatus as Status,
        LogicalObservationPrerequisite, LogicalObservationUnit,
    };
    // Literal boundaries: policy, specification, selected labels, effective minimum, conclusion.
    for (policy_minimum, specification_minimum, labels, effective, conclusion) in [
        (2, 1, vec!["case.a"], 2, Status::Missing),
        (2, 1, vec!["case.a", "case.b"], 2, Status::CapacityMet),
        (
            2,
            1,
            vec!["case.a", "case.b", "case.c"],
            2,
            Status::CapacityMet,
        ),
        (4, 1, vec!["case.a", "case.b", "case.c"], 4, Status::Missing),
        (1, 3, vec!["case.a", "case.b"], 3, Status::Missing),
        (
            2,
            3,
            vec!["case.a", "case.b", "case.c"],
            3,
            Status::CapacityMet,
        ),
    ] {
        let repository = example()?;
        let policy_path = repository
            .path()
            .join("eqm/policies/consumer.critical_flow.toml");
        let policy = fs::read_to_string(&policy_path)?.replace(
            "minimum_count = 1",
            &format!("minimum_count = {policy_minimum}"),
        );
        // A genuinely stronger unrelated facet cannot leak its count into behavior.
        let policy = policy.replace("[waivers]", "[[rules]]\nminimum_level = \"required\"\nfacets = [\"accessibility\"]\nminimum_trust = \"trusted_ci\"\nmaximum_age = 86400000\nminimum_count = 99\n\n[rules.selector]\nrisk_classes = [\"high\", \"critical\"]\n\n[waivers]");
        fs::write(&policy_path, policy)?;
        let binding_path = repository
            .path()
            .join("eqm/bindings/android.auth_signup.toml");
        fs::write(
            &binding_path,
            fs::read_to_string(&binding_path)?.replace(
                "kind = \"test\"\nrequirements",
                &format!("kind = \"test\"\nminimum_count = {specification_minimum}\nrequirements"),
            ),
        )?;
        let session = session_for(repository.path())?;
        let provenance = projections(&session)?;
        let context = context(&provenance, None)?;
        let cases = labels
            .into_iter()
            .map(LogicalCaseId::new)
            .collect::<Result<Vec<_>, _>>()?;
        let (prepared, key, selection) = prepared_coordinates_with_selection(
            &session,
            "eu",
            None,
            cases.clone(),
            vec![],
            false,
        )?;
        let identity = prepared
            .identities()
            .get(&key)
            .ok_or("identity missing")?
            .digest();
        let spec = EvidenceSpecId::new("identifier_behavior")?;
        let expected = vec![CompleteCoverageExpectation {
            structural: coverage_context(&context, &spec),
            coordinate: key.clone(),
            identity,
            selection: selection.clone(),
            subjects: None,
        }];
        // Freeze graph, all selected labels and S054 identity before result bytes.
        let checked = seal(record(&provenance, None)?)?;
        let mut admissions = Vec::new();
        for case in &cases {
            let selected_lineage = EvidenceLineage::from_parts(EvidenceLineageParts {
                plan: Some(lineage()?.plan().clone()),
                campaign: Some(lineage()?.campaign().clone()),
                run: Some(lineage()?.run().clone()),
                logical_case: Some(case.clone()),
                attempt: Some(eqm_domain::AttemptId::new("attempt.count")?),
            })?;
            let execution = EvidenceExecution::new(
                selected_lineage.run().clone(),
                case.clone(),
                vec![selected_lineage.attempt().clone()],
            )?;
            admissions.push(admit_evidence_bytes(
                checked.raw_bytes(),
                &selected_lineage,
                &execution,
                &context,
            )?);
        }
        let receipt = SelectorText::new("receipt.count")?;
        let deliveries = admissions
            .iter()
            .map(|admitted| CompleteEvidenceDelivery {
                delivery: AdmittedEvidenceDelivery {
                    admitted,
                    evidence_spec_id: &spec,
                    provenance: &receipt,
                },
                coordinate: Some(&key),
                identity: Some(identity),
                subjects: None,
            })
            .collect::<Vec<_>>();
        let before = file_snapshot(repository.path())?;
        let report = assess_admitted_count_constraints(&prepared, &expected, &deliveries)?;
        assert_eq!(report.policy_minimum().get(), policy_minimum);
        assert_eq!(
            report.specification_minima()[selection.source()].get(),
            specification_minimum
        );
        assert_eq!(report.effective_minimum().get(), effective);
        assert_eq!(report.capacity(), cases.len());
        assert_eq!(report.status(), conclusion);
        assert_eq!(report.observations().status(), CoverageStatus::Covered);
        assert_eq!(
            report.observations().units(),
            &cases
                .iter()
                .cloned()
                .map(LogicalObservationUnit::TestCase)
                .collect::<BTreeSet<_>>()
        );
        assert_eq!(
            report.observations().prerequisites(),
            &BTreeSet::from([
                LogicalObservationPrerequisite::IndependentTrust,
                LogicalObservationPrerequisite::ExecutionOutcomes,
            ])
        );
        let mut reversed = deliveries.clone();
        reversed.reverse();
        reversed.extend(deliveries.iter().copied());
        assert_eq!(
            assess_admitted_count_constraints(&prepared, &expected, &reversed)?,
            report
        );
        let copied =
            assess_admitted_count_constraints(&prepared, &expected, &vec![deliveries[0]; 10])?;
        assert_eq!(copied.capacity(), 1);
        assert_eq!(copied.status(), Status::Missing);
        if cases.len() == 3 && policy_minimum == 2 && specification_minimum == 1 {
            let missing =
                assess_admitted_count_constraints(&prepared, &expected, &deliveries[..2])?;
            assert_eq!(missing.capacity(), 2);
            assert_eq!(missing.status(), Status::Missing);
            assert_eq!(
                missing.observations().missing()[selection.source()],
                BTreeSet::from([LogicalCaseId::new("case.c")?])
            );
        }
        let empty = assess_admitted_count_constraints(&prepared, &expected, &[])?;
        assert_eq!(empty.status(), Status::Missing);
        assert_eq!(empty.capacity(), 0);
        assert_eq!(file_snapshot(repository.path())?, before);
    }
    Ok(())
}

#[test]
fn actual_all_kind_count_constraints_preserve_one_unit_and_independent_prerequisites()
-> Result<(), Box<dyn Error>> {
    use eqm_engine::{
        CompleteCoverageExpectation, LogicalCountConstraintStatus as Status,
        LogicalObservationPrerequisite as Need,
    };
    for (kind, needs) in [
        (
            EvidenceKind::Test,
            BTreeSet::from([Need::IndependentTrust, Need::ExecutionOutcomes]),
        ),
        (
            EvidenceKind::StructuralCheck,
            BTreeSet::from([
                Need::IndependentTrust,
                Need::ExecutionOutcomes,
                Need::StructuralFacts,
            ]),
        ),
        (
            EvidenceKind::StaticInventory,
            BTreeSet::from([Need::IndependentTrust, Need::InventoryFacts]),
        ),
        (
            EvidenceKind::Snapshot,
            BTreeSet::from([Need::IndependentTrust, Need::ExecutionOutcomes]),
        ),
        (
            EvidenceKind::RuntimeSnapshot,
            BTreeSet::from([Need::IndependentTrust, Need::RuntimeFacts]),
        ),
        (
            EvidenceKind::ManualReview,
            BTreeSet::from([
                Need::IndependentTrust,
                Need::ReviewOutcomes,
                Need::DistinctReviewPrincipals,
            ]),
        ),
        (
            EvidenceKind::ReleaseRecord,
            BTreeSet::from([Need::IndependentTrust, Need::ReferencedRecord]),
        ),
    ] {
        for (minimum, conclusion) in [(1, Status::CapacityMet), (2, Status::Missing)] {
            let repository = kind_fixture(kind)?;
            let policy_path = repository
                .path()
                .join("eqm/policies/consumer.critical_flow.toml");
            fs::write(
                &policy_path,
                fs::read_to_string(&policy_path)?
                    .replace("minimum_count = 1", &format!("minimum_count = {minimum}")),
            )?;
            let session = session_for(repository.path())?;
            let provenance = projections(&session)?;
            let (value, context) = record_for_kind(&provenance, kind)?;
            let (prepared, key, selection) = prepared_coordinates_with_selection(
                &session,
                "eu",
                context.release_record_digest,
                vec![lineage()?.logical_case().clone()],
                vec![],
                kind == EvidenceKind::ManualReview,
            )?;
            let identity = prepared.identities().get(&key).ok_or("identity")?.digest();
            let spec = EvidenceSpecId::new("identifier_behavior")?;
            let expected = vec![CompleteCoverageExpectation {
                structural: coverage_context(&context, &spec),
                coordinate: key.clone(),
                identity,
                selection: selection.clone(),
                subjects: None,
            }];
            let checked = seal(value)?;
            let admitted =
                admit_evidence_bytes(checked.raw_bytes(), &lineage()?, &execution()?, &context)?;
            let receipt = SelectorText::new("receipt.kind.count")?;
            let delivery = eqm_application::CompleteEvidenceDelivery {
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
            let report = eqm_application::assess_admitted_count_constraints(
                &prepared,
                &expected,
                &[delivery; 10],
            )?;
            assert_eq!(report.policy_minimum().get(), minimum);
            assert_eq!(report.effective_minimum().get(), minimum);
            assert_eq!(report.specification_minima()[selection.source()].get(), 1);
            assert_eq!(report.capacity(), 1, "{kind}");
            assert_eq!(report.status(), conclusion, "{kind}");
            assert_eq!(report.observations().prerequisites(), &needs);
            if kind != EvidenceKind::ManualReview {
                assert!(matches!(
                    eqm_application::assess_admitted_manual_reviews(
                        &prepared,
                        &expected,
                        &[delivery]
                    ),
                    Err(eqm_application::AdmittedManualReviewError::Review(
                        eqm_engine::ManualReviewError::Kind
                    ))
                ));
            }
            let history = eqm_application::assess_admitted_campaign_retry_history(
                &prepared,
                &expected,
                &[delivery; 10],
            )?;
            assert_eq!(history.constraints(), &report);
            let history_state = match kind {
                EvidenceKind::Test | EvidenceKind::Snapshot | EvidenceKind::StructuralCheck => {
                    eqm_engine::CampaignRetryState::Passed
                }
                EvidenceKind::StaticInventory
                | EvidenceKind::RuntimeSnapshot
                | EvidenceKind::ManualReview
                | EvidenceKind::ReleaseRecord => {
                    eqm_engine::CampaignRetryState::PendingKindEvaluation
                }
            };
            assert_eq!(history.state(), history_state);
            assert_eq!(history.histories().len(), 1);
            assert_eq!(
                history
                    .histories()
                    .values()
                    .next()
                    .ok_or("history")?
                    .attempts()
                    .len(),
                1
            );
            if history_state == eqm_engine::CampaignRetryState::PendingKindEvaluation {
                assert!(
                    history
                        .reasons()
                        .contains(&eqm_engine::CampaignRetryReason::PendingKind(kind))
                );
            }
            assert_eq!(
                eqm_application::assess_admitted_campaign_retry_history(
                    &prepared,
                    &expected,
                    &[delivery]
                )?,
                history
            );

            assert_eq!(report.observations().status(), CoverageStatus::Covered);
            assert_eq!(report.observations().count_unit(), selection.count_unit());
            assert_eq!(report.observations().units().len(), 1);
            if kind == EvidenceKind::ReleaseRecord {
                assert_eq!(
                    report.observations().units(),
                    &BTreeSet::from([eqm_engine::LogicalObservationUnit::ReferencedRecord(
                        Sha256Digest::hash_content(b"synthetic referenced record")
                    )])
                );
            }
            assert_eq!(
                eqm_application::assess_admitted_count_constraints(
                    &prepared,
                    &expected,
                    &[delivery]
                )?,
                report
            );
            assert_eq!(file_snapshot(repository.path())?, before);
        }
    }
    Ok(())
}

#[test]
fn actual_count_constraints_reject_frozen_context_substitution_and_keep_conflicts()
-> Result<(), Box<dyn Error>> {
    use eqm_application::{
        AdmittedCountConstraintError as Error, CompleteEvidenceDelivery,
        assess_admitted_count_constraints,
    };
    use eqm_domain::ExpectedSelectionManifest;
    use eqm_engine::{
        CompleteCoverageExpectation, CompleteCoverageMismatch,
        LogicalCountConstraintError as Constraint, LogicalCountConstraintStatus as Status,
    };
    let repository = example()?;
    let session = session_for(repository.path())?;
    let provenance = projections(&session)?;
    let context = context(&provenance, None)?;
    let cases = vec![LogicalCaseId::new("case.a")?, LogicalCaseId::new("case.b")?];
    let (prepared, key, selection) =
        prepared_coordinates_with_selection(&session, "eu", None, cases.clone(), vec![], false)?;
    let identity = prepared.identities().get(&key).ok_or("identity")?.digest();
    let spec = EvidenceSpecId::new("identifier_behavior")?;
    let expected = vec![CompleteCoverageExpectation {
        structural: coverage_context(&context, &spec),
        coordinate: key.clone(),
        identity,
        selection: selection.clone(),
        subjects: None,
    }];
    let checked = seal(record(&provenance, None)?)?;
    let mut admissions = Vec::new();
    for case in &cases {
        let selected = EvidenceLineage::from_parts(EvidenceLineageParts {
            plan: Some(lineage()?.plan().clone()),
            campaign: Some(lineage()?.campaign().clone()),
            run: Some(lineage()?.run().clone()),
            logical_case: Some(case.clone()),
            attempt: Some(eqm_domain::AttemptId::new("attempt.count")?),
        })?;
        let execution = EvidenceExecution::new(
            selected.run().clone(),
            case.clone(),
            vec![selected.attempt().clone()],
        )?;
        admissions.push(admit_evidence_bytes(
            checked.raw_bytes(),
            &selected,
            &execution,
            &context,
        )?);
    }
    let receipt = SelectorText::new("receipt.count.context")?;
    let deliveries = admissions
        .iter()
        .map(|admitted| CompleteEvidenceDelivery {
            delivery: AdmittedEvidenceDelivery {
                admitted,
                evidence_spec_id: &spec,
                provenance: &receipt,
            },
            coordinate: Some(&key),
            identity: Some(identity),
            subjects: None,
        })
        .collect::<Vec<_>>();
    let before = file_snapshot(repository.path())?;
    assert_eq!(
        assess_admitted_count_constraints(&prepared, &expected, &deliveries)?.status(),
        Status::CapacityMet
    );
    assert!(matches!(
        assess_admitted_count_constraints(&prepared, &[], &deliveries),
        Err(Error::Constraints(Constraint::EmptyExpectations))
    ));
    assert!(matches!(
        assess_admitted_count_constraints(
            &prepared,
            &[expected[0].clone(), expected[0].clone()],
            &deliveries
        ),
        Err(Error::Constraints(Constraint::FrozenSelection))
    ));
    assert!(matches!(
        assess_admitted_count_constraints(
            &prepared,
            &vec![expected[0].clone(); eqm_engine::MAX_CANDIDATE_DELIVERIES + 1],
            &deliveries
        ),
        Err(Error::Constraints(Constraint::Bound))
    ));
    let mut changed = expected.clone();
    changed[0].identity =
        eqm_domain::ObligationDigest::hash_content(b"foreign pre-result identity");
    assert!(matches!(
        assess_admitted_count_constraints(&prepared, &changed, &deliveries),
        Err(Error::FrozenContext)
    ));
    for change_campaign in [false, true] {
        let mut changed = expected.clone();
        let mut source = selection.source().clone();
        if change_campaign {
            source.campaign = CampaignId::new("campaign.foreign")?;
        }
        changed[0].selection = ExpectedSelectionManifest::new(
            source,
            EvidenceKind::Test,
            eqm_domain::SelectionCountUnit::LogicalTestCase,
            if change_campaign {
                cases.clone()
            } else {
                vec![cases[0].clone()]
            },
            vec![],
            false,
        )?;
        assert!(matches!(
            assess_admitted_count_constraints(&prepared, &changed, &deliveries),
            Err(Error::Constraints(Constraint::FrozenSelection))
        ));
    }
    let foreign_identity =
        eqm_domain::ObligationDigest::hash_content(b"foreign candidate identity");
    let rejected = CompleteEvidenceDelivery {
        identity: Some(foreign_identity),
        ..deliveries[0]
    };
    let missing = assess_admitted_count_constraints(&prepared, &expected, &[rejected])?;
    assert_eq!(missing.status(), Status::Missing);
    assert_eq!(missing.capacity(), 0);
    assert_eq!(
        missing.observations().matching()[selection.source()].rejected()
            [&admissions[0].result().id()],
        BTreeSet::from([CompleteCoverageMismatch::ObligationIdentity])
    );
    let mixed = assess_admitted_count_constraints(
        &prepared,
        &expected,
        &[rejected, deliveries[1], deliveries[0]],
    )?;
    assert_eq!(mixed.status(), Status::CapacityMet);
    assert_eq!(mixed.capacity(), 2);
    assert!(mixed.observations().conflicts().is_empty());
    let over = vec![deliveries[0]; eqm_engine::MAX_CANDIDATE_DELIVERIES + 1];
    assert!(matches!(
        assess_admitted_count_constraints(&prepared, &expected, &over),
        Err(Error::Observations(
            eqm_engine::LogicalObservationError::Matching(
                eqm_engine::CompleteCoverageError::Normalization(
                    eqm_engine::CandidateNormalizationError::LimitExceeded
                )
            )
        ))
    ));
    let mut retries = Vec::new();
    for attempt in 0..10 {
        let selected = EvidenceLineage::from_parts(EvidenceLineageParts {
            plan: Some(lineage()?.plan().clone()),
            campaign: Some(lineage()?.campaign().clone()),
            run: Some(RunId::new(format!("run.retry_{attempt}"))?),
            logical_case: Some(cases[0].clone()),
            attempt: Some(eqm_domain::AttemptId::new(format!(
                "attempt.retry_{attempt}"
            ))?),
        })?;
        let execution = EvidenceExecution::new(
            selected.run().clone(),
            cases[0].clone(),
            vec![selected.attempt().clone()],
        )?;
        retries.push(admit_evidence_bytes(
            checked.raw_bytes(),
            &selected,
            &execution,
            &context,
        )?);
    }
    let retry_deliveries = retries
        .iter()
        .map(|admitted| CompleteEvidenceDelivery {
            delivery: AdmittedEvidenceDelivery {
                admitted,
                evidence_spec_id: &spec,
                provenance: &receipt,
            },
            ..deliveries[0]
        })
        .collect::<Vec<_>>();
    let retried = assess_admitted_count_constraints(&prepared, &expected, &retry_deliveries)?;
    assert_eq!(retried.status(), Status::Missing);
    assert_eq!(retried.capacity(), 1);
    assert_eq!(
        retried.observations().missing()[selection.source()],
        BTreeSet::from([cases[1].clone()])
    );
    assert_eq!(
        retried.observations().matching()[selection.source()]
            .normalized()
            .attempts()
            .len(),
        10
    );
    let mut failed = checked.record().clone();
    let EvidencePayloadV2Dto::Test { execution: payload } = &mut failed.payload else {
        return Err("test payload".into());
    };
    payload.attempts[0].outcome = "failed".to_owned();
    payload.counts.passed = WireUint::new(0);
    payload.counts.failed = WireUint::new(1);
    let failed = seal(failed)?;
    let failed = admit_evidence_bytes(
        failed.raw_bytes(),
        admissions[0].lineage(),
        &EvidenceExecution::new(
            admissions[0].lineage().run().clone(),
            cases[0].clone(),
            vec![admissions[0].lineage().attempt().clone()],
        )?,
        &context,
    )?;
    let failed_delivery = CompleteEvidenceDelivery {
        delivery: AdmittedEvidenceDelivery {
            admitted: &failed,
            evidence_spec_id: &spec,
            provenance: &receipt,
        },
        ..deliveries[0]
    };
    let conflict = assess_admitted_count_constraints(
        &prepared,
        &expected,
        &[deliveries[1], failed_delivery, deliveries[0]],
    )?;
    assert_eq!(conflict.status(), Status::Unknown);
    assert_eq!(conflict.capacity(), 1);
    assert_eq!(conflict.observations().conflicts().len(), 1);
    assert_eq!(
        conflict.observations().missing()[selection.source()],
        BTreeSet::from([cases[0].clone()])
    );
    assert_eq!(
        assess_admitted_count_constraints(
            &prepared,
            &expected,
            &[deliveries[0], failed_delivery, deliveries[1]]
        )?,
        conflict
    );
    assert_eq!(file_snapshot(repository.path())?, before);
    let (foreign_profile, _, _) =
        prepared_coordinates_with_selection(&session, "us", None, cases.clone(), vec![], false)?;
    assert!(matches!(
        assess_admitted_count_constraints(&foreign_profile, &expected, &deliveries),
        Err(Error::Constraints(Constraint::Coordinate))
    ));
    let policy_path = repository
        .path()
        .join("eqm/policies/consumer.critical_flow.toml");
    fs::write(
        &policy_path,
        fs::read_to_string(&policy_path)?.replace("minimum_count = 1", "minimum_count = 2"),
    )?;
    let changed_session = session_for(repository.path())?;
    let (changed_policy, _, _) =
        prepared_coordinates_with_selection(&changed_session, "eu", None, cases, vec![], false)?;
    let after_edit = file_snapshot(repository.path())?;
    assert!(matches!(
        assess_admitted_count_constraints(&changed_policy, &expected, &deliveries),
        Err(Error::FrozenContext)
    ));
    assert_eq!(file_snapshot(repository.path())?, after_edit);
    Ok(())
}

#[test]
fn actual_selected_campaign_retry_history_preserves_case_conjunction_and_reasons()
-> Result<(), Box<dyn Error>> {
    use eqm_application::{CompleteEvidenceDelivery, assess_admitted_campaign_retry_history};
    use eqm_domain::{AttemptId, AttemptOutcome};
    use eqm_engine::{
        CampaignRetryReason as Reason, CampaignRetryState as State, CompleteCoverageExpectation,
    };
    let repository = example()?;
    let session = session_for(repository.path())?;
    let provenance = projections(&session)?;
    let context = context(&provenance, None)?;
    let cases = vec![LogicalCaseId::new("case.a")?, LogicalCaseId::new("case.b")?];
    let (prepared, key, selection) =
        prepared_coordinates_with_selection(&session, "eu", None, cases.clone(), vec![], false)?;
    let identity = prepared.identities().get(&key).ok_or("identity")?.digest();
    let spec = EvidenceSpecId::new("identifier_behavior")?;
    let expected = vec![CompleteCoverageExpectation {
        structural: coverage_context(&context, &spec),
        coordinate: key.clone(),
        identity,
        selection: selection.clone(),
        subjects: None,
    }];
    let receipt = SelectorText::new("receipt.retry.history")?;
    let before = file_snapshot(repository.path())?;
    // Literal rows distinguish AND failures, retries, immutable contradictions,
    // uncertainty, and non-executed selections. IDs name actual immutable attempts.
    let rows = [
        (
            vec![(0, "first", "passed"), (1, "first", "passed")],
            State::Passed,
        ),
        (
            vec![(0, "first", "failed"), (1, "first", "passed")],
            State::Failed,
        ),
        (
            vec![
                (0, "first", "failed"),
                (0, "retry", "passed"),
                (1, "first", "passed"),
            ],
            State::Unstable,
        ),
        (
            vec![
                (0, "first", "failed"),
                (0, "retry", "passed"),
                (0, "timeout", "timed_out"),
                (1, "first", "passed"),
            ],
            State::Unknown,
        ),
        (
            vec![(0, "first", "cancelled"), (1, "first", "passed")],
            State::Unknown,
        ),
        (
            vec![(0, "first", "error"), (1, "first", "passed")],
            State::Unknown,
        ),
        (
            vec![(0, "first", "skipped"), (1, "first", "passed")],
            State::Missing,
        ),
        (
            vec![(0, "first", "filtered"), (1, "first", "passed")],
            State::Missing,
        ),
        (
            vec![(0, "first", "quarantined"), (1, "first", "passed")],
            State::Missing,
        ),
        (vec![(0, "first", "passed")], State::Missing),
        (
            vec![
                (0, "first", "failed"),
                (0, "first", "passed"),
                (1, "first", "passed"),
            ],
            State::Unknown,
        ),
    ];
    for (row, literal) in rows {
        let mut admissions = Vec::new();
        for (case, name, outcome) in &row {
            let mut dto = record(&provenance, None)?;
            let EvidencePayloadV2Dto::Test { execution: payload } = &mut dto.payload else {
                return Err("test payload".into());
            };
            payload.attempts[0].outcome = (*outcome).to_owned();
            payload.counts.passed = WireUint::new(u64::from(*outcome == "passed"));
            payload.counts.failed = WireUint::new(u64::from(*outcome == "failed"));
            payload.counts.skipped = WireUint::new(u64::from(*outcome == "skipped"));
            payload.counts.filtered = WireUint::new(u64::from(*outcome == "filtered"));
            payload.counts.quarantined = WireUint::new(u64::from(*outcome == "quarantined"));
            payload.counts.selected = WireUint::new(u64::from(matches!(
                *outcome,
                "passed" | "failed" | "skipped" | "filtered" | "quarantined"
            )));
            let checked = seal(dto)?;
            let attempt = AttemptId::new(format!("attempt.{name}"))?;
            let selected = EvidenceLineage::from_parts(EvidenceLineageParts {
                plan: Some(selection.source().plan.clone()),
                campaign: Some(selection.source().campaign.clone()),
                run: Some(RunId::new(format!("run.{name}"))?),
                logical_case: Some(cases[*case].clone()),
                attempt: Some(attempt.clone()),
            })?;
            let execution = EvidenceExecution::new(
                selected.run().clone(),
                cases[*case].clone(),
                vec![attempt],
            )?;
            admissions.push(admit_evidence_bytes(
                checked.raw_bytes(),
                &selected,
                &execution,
                &context,
            )?);
        }
        let deliveries = admissions
            .iter()
            .map(|admitted| CompleteEvidenceDelivery {
                delivery: AdmittedEvidenceDelivery {
                    admitted,
                    evidence_spec_id: &spec,
                    provenance: &receipt,
                },
                coordinate: Some(&key),
                identity: Some(identity),
                subjects: None,
            })
            .collect::<Vec<_>>();
        let report = assess_admitted_campaign_retry_history(&prepared, &expected, &deliveries)?;
        assert_eq!(report.state(), literal, "{row:?}");
        assert_eq!(report.source_cases().len(), 2);
        assert_eq!(report.histories().len(), if row.len() == 1 { 1 } else { 2 });
        let actual_terminal = report
            .reasons()
            .iter()
            .filter_map(|r| {
                if let Reason::Terminal(outcome) = r {
                    Some(*outcome)
                } else {
                    None
                }
            })
            .collect::<BTreeSet<_>>();
        let literal_terminal = row
            .iter()
            .map(|(_, _, value)| value.parse::<AttemptOutcome>())
            .collect::<Result<BTreeSet<_>, _>>()?;
        assert_eq!(actual_terminal, literal_terminal);
        let retained = report
            .histories()
            .values()
            .map(|h| {
                h.attempts()
                    .values()
                    .map(std::collections::BTreeMap::len)
                    .sum::<usize>()
            })
            .sum::<usize>();
        assert_eq!(retained, row.len());
        if row.len() == 1 {
            assert!(report.reasons().contains(&Reason::MissingSelectedCase(
                selection.source().clone(),
                cases[1].clone()
            )));
        }
        if row
            .iter()
            .filter(|(case, name, _)| *case == 0 && *name == "first")
            .count()
            == 2
        {
            assert!(report.reasons().contains(&Reason::ImmutableConflict));
            assert_eq!(report.constraints().observations().conflicts().len(), 1);
        }
        assert!(
            report
                .constraints()
                .observations()
                .prerequisites()
                .contains(&eqm_engine::LogicalObservationPrerequisite::IndependentTrust)
        );
        let mut permuted = deliveries.clone();
        permuted.reverse();
        permuted.extend(deliveries.iter().copied());
        assert_eq!(
            assess_admitted_campaign_retry_history(&prepared, &expected, &permuted)?,
            report
        );
        assert_eq!(file_snapshot(repository.path())?, before);
    }
    // One actual admitted payload can itself retain ordered internal retries.
    let mut dto = record(&provenance, None)?;
    let EvidencePayloadV2Dto::Test { execution: payload } = &mut dto.payload else {
        return Err("test payload".into());
    };
    payload.attempts[0].outcome = "failed".to_owned();
    let mut second = payload.attempts[0].clone();
    second.number = WireUint::new(2);
    second.outcome = "passed".to_owned();
    second.started_at = second.finished_at;
    payload.attempts.push(second);
    payload.counts.selected = WireUint::new(2);
    payload.counts.passed = WireUint::new(1);
    payload.counts.failed = WireUint::new(1);
    let checked = seal(dto)?;
    let selected = EvidenceLineage::from_parts(EvidenceLineageParts {
        plan: Some(selection.source().plan.clone()),
        campaign: Some(selection.source().campaign.clone()),
        run: Some(RunId::new("run.internal_retries")?),
        logical_case: Some(cases[0].clone()),
        attempt: Some(AttemptId::new("attempt.internal_first")?),
    })?;
    let execution = EvidenceExecution::new(
        selected.run().clone(),
        cases[0].clone(),
        vec![
            selected.attempt().clone(),
            AttemptId::new("attempt.internal_second")?,
        ],
    )?;
    let internal = admit_evidence_bytes(checked.raw_bytes(), &selected, &execution, &context)?;
    let delivery = CompleteEvidenceDelivery {
        delivery: AdmittedEvidenceDelivery {
            admitted: &internal,
            evidence_spec_id: &spec,
            provenance: &receipt,
        },
        coordinate: Some(&key),
        identity: Some(identity),
        subjects: None,
    };
    let internal_history =
        assess_admitted_campaign_retry_history(&prepared, &expected, &[delivery])?;
    assert_eq!(internal_history.state(), State::Unstable);
    assert_eq!(internal_history.constraints().capacity(), 1);
    assert!(
        internal_history
            .reasons()
            .contains(&Reason::MissingSelectedCase(
                selection.source().clone(),
                cases[1].clone()
            ))
    );
    assert!(
        internal_history
            .reasons()
            .contains(&Reason::Terminal(AttemptOutcome::Failed))
    );
    assert!(
        internal_history
            .reasons()
            .contains(&Reason::Terminal(AttemptOutcome::Passed))
    );
    // A failed result admitted under an explicitly different campaign remains audit,
    // and never adds eligible terminal history to this frozen selected campaign.
    let mut foreign_context = context.clone();
    foreign_context.expected_lineage = eqm_domain::ExpectedLineageAuthority::new(
        selection.source().plan.clone(),
        CampaignId::new("campaign.superseded")?,
    );
    let foreign_lineage = EvidenceLineage::from_parts(EvidenceLineageParts {
        plan: Some(selection.source().plan.clone()),
        campaign: Some(CampaignId::new("campaign.superseded")?),
        run: Some(selected.run().clone()),
        logical_case: Some(cases[0].clone()),
        attempt: Some(selected.attempt().clone()),
    })?;
    let foreign = admit_evidence_bytes(
        checked.raw_bytes(),
        &foreign_lineage,
        &execution,
        &foreign_context,
    )?;
    let foreign_delivery = CompleteEvidenceDelivery {
        delivery: AdmittedEvidenceDelivery {
            admitted: &foreign,
            evidence_spec_id: &spec,
            provenance: &receipt,
        },
        ..delivery
    };
    let rejected =
        assess_admitted_campaign_retry_history(&prepared, &expected, &[foreign_delivery])?;
    assert_eq!(rejected.state(), State::Missing);
    assert!(rejected.histories().is_empty());
    assert!(
        !rejected
            .reasons()
            .contains(&Reason::Terminal(AttemptOutcome::Failed))
    );
    assert_eq!(
        rejected.constraints().observations().matching()[selection.source()].rejected()
            [&foreign.result().id()],
        BTreeSet::from([eqm_engine::CompleteCoverageMismatch::Campaign])
    );
    let foreign_identity =
        eqm_domain::ObligationDigest::hash_content(b"foreign retry history pre-result claim");
    assert!(matches!(
        assess_admitted_campaign_retry_history(
            &prepared,
            &[CompleteCoverageExpectation {
                identity: foreign_identity,
                ..expected[0].clone()
            }],
            &[delivery]
        ),
        Err(eqm_application::AdmittedCampaignRetryError::Context(
            eqm_application::AdmittedCountConstraintError::FrozenContext
        ))
    ));
    assert!(matches!(
        assess_admitted_campaign_retry_history(&prepared, &[], &[delivery]),
        Err(eqm_application::AdmittedCampaignRetryError::Context(
            eqm_application::AdmittedCountConstraintError::Constraints(
                eqm_engine::LogicalCountConstraintError::EmptyExpectations
            )
        ))
    ));
    assert!(matches!(
        assess_admitted_campaign_retry_history(
            &prepared,
            &expected,
            &vec![delivery; eqm_engine::MAX_CANDIDATE_DELIVERIES + 1]
        ),
        Err(eqm_application::AdmittedCampaignRetryError::Context(
            eqm_application::AdmittedCountConstraintError::Observations(_)
        ))
    ));
    assert_eq!(file_snapshot(repository.path())?, before);
    Ok(())
}

#[test]
fn actual_manual_reviews_preserve_outcomes_slots_and_unverified_authority()
-> Result<(), Box<dyn Error>> {
    use eqm_application::{CompleteEvidenceDelivery, assess_admitted_manual_reviews};
    use eqm_domain::{AttemptId, AttemptOutcome};
    use eqm_engine::{
        CompleteCoverageExpectation, EvidenceOutcome as State, ManualReviewReason as Reason,
    };
    for minimum in [1, 2] {
        for independent in [false, true] {
            let repository = kind_fixture(EvidenceKind::ManualReview)?;
            let policy_path = repository
                .path()
                .join("eqm/policies/consumer.critical_flow.toml");
            fs::write(
                &policy_path,
                fs::read_to_string(&policy_path)?
                    .replace("minimum_count = 1", &format!("minimum_count = {minimum}")),
            )?;
            let session = session_for(repository.path())?;
            let provenance = projections(&session)?;
            let (template, context) = record_for_kind(&provenance, EvidenceKind::ManualReview)?;
            let cases = vec![
                LogicalCaseId::new("case.review_a")?,
                LogicalCaseId::new("case.review_b")?,
            ];
            let (prepared, key, selection) = prepared_coordinates_with_selection(
                &session,
                "eu",
                None,
                cases.clone(),
                vec![],
                independent,
            )?;
            let identity = prepared.identities().get(&key).ok_or("identity")?.digest();
            let spec = EvidenceSpecId::new("identifier_behavior")?;
            let expected = vec![CompleteCoverageExpectation {
                structural: coverage_context(&context, &spec),
                coordinate: key.clone(),
                identity,
                selection: selection.clone(),
                subjects: None,
            }];
            let receipt = SelectorText::new("receipt.explicit.review")?;
            let before = file_snapshot(repository.path())?;
            // Literal conjunction, retry, conflict and duplicate controls use actual strict bytes.
            let rows = [
                (
                    vec![(0, "first", "passed"), (1, "first", "passed")],
                    State::Satisfied,
                    2,
                ),
                (
                    vec![(0, "first", "failed"), (1, "first", "passed")],
                    State::Failed,
                    1,
                ),
                (
                    vec![
                        (0, "first", "failed"),
                        (0, "retry", "passed"),
                        (1, "first", "passed"),
                    ],
                    State::Unstable,
                    1,
                ),
                (
                    vec![
                        (0, "first", "failed"),
                        (0, "first", "passed"),
                        (1, "first", "passed"),
                    ],
                    State::Unknown,
                    1,
                ),
                (vec![(0, "first", "passed")], State::Missing, 1),
                (vec![], State::Missing, 0),
            ];
            for (row, literal, passing) in rows {
                let mut admissions = Vec::new();
                for (case, attempt_name, outcome) in &row {
                    let mut value = template.clone();
                    value.payload = EvidencePayloadV2Dto::ManualReview {
                        outcome: (*outcome).to_owned(),
                        reviewer: format!("owner://role/{attempt_name}"),
                        message: Some("Synthetic claim, not verified authority".to_owned()),
                    };
                    let checked = seal(value)?;
                    let attempt = AttemptId::new(format!("attempt.{attempt_name}"))?;
                    let selected = EvidenceLineage::from_parts(EvidenceLineageParts {
                        plan: Some(selection.source().plan.clone()),
                        campaign: Some(selection.source().campaign.clone()),
                        run: Some(RunId::new(format!("run.{attempt_name}"))?),
                        logical_case: Some(cases[*case].clone()),
                        attempt: Some(attempt.clone()),
                    })?;
                    let execution = EvidenceExecution::new(
                        selected.run().clone(),
                        cases[*case].clone(),
                        vec![attempt],
                    )?;
                    admissions.push(admit_evidence_bytes(
                        checked.raw_bytes(),
                        &selected,
                        &execution,
                        &context,
                    )?);
                }
                let deliveries = admissions
                    .iter()
                    .map(|admitted| CompleteEvidenceDelivery {
                        delivery: AdmittedEvidenceDelivery {
                            admitted,
                            evidence_spec_id: &spec,
                            provenance: &receipt,
                        },
                        coordinate: Some(&key),
                        identity: Some(identity),
                        subjects: None,
                    })
                    .collect::<Vec<_>>();
                let report = assess_admitted_manual_reviews(&prepared, &expected, &deliveries)?;
                assert_eq!(report.outcome(), literal, "{row:?}");
                assert_eq!(report.passing_slots().len(), passing, "{row:?}");
                assert_eq!(
                    report.history().constraints().effective_minimum().get(),
                    minimum
                );
                assert_eq!(
                    report.acceptance(),
                    if row.is_empty() {
                        State::Missing
                    } else {
                        State::Unknown
                    }
                );
                assert_eq!(
                    report
                        .reasons()
                        .contains(&Reason::IndependentVerificationRequired),
                    !row.is_empty()
                );
                assert_eq!(
                    report
                        .reasons()
                        .contains(&Reason::DistinctAuthorizedPrincipalsRequired),
                    independent && !row.is_empty()
                );
                assert_eq!(
                    report.reasons().contains(&Reason::InsufficientPassingSlots),
                    passing < minimum as usize
                );
                let literal_terminal = row
                    .iter()
                    .map(|(_, _, outcome)| outcome.parse::<AttemptOutcome>())
                    .collect::<Result<BTreeSet<_>, _>>()?;
                let actual_terminal = report
                    .reasons()
                    .iter()
                    .filter_map(|r| {
                        if let Reason::Terminal(o) = r {
                            Some(*o)
                        } else {
                            None
                        }
                    })
                    .collect::<BTreeSet<_>>();
                assert_eq!(actual_terminal, literal_terminal);
                let mut permuted = deliveries.clone();
                permuted.reverse();
                permuted.extend(deliveries.iter().copied());
                assert_eq!(
                    assess_admitted_manual_reviews(&prepared, &expected, &permuted)?,
                    report
                );
                if row.len() == 1 {
                    assert!(report.reasons().contains(&Reason::MissingSelectedCase(
                        selection.source().clone(),
                        cases[1].clone()
                    )));
                }
                if literal == State::Unknown {
                    assert!(report.reasons().contains(&Reason::ImmutableConflict));
                }
                assert_eq!(file_snapshot(repository.path())?, before);
            }
            // Unsupported wire outcomes reject, leaving the declared slot unresolved.
            for outcome in [
                "skipped",
                "filtered",
                "quarantined",
                "cancelled",
                "timed_out",
                "error",
            ] {
                let mut invalid = template.clone();
                let EvidencePayloadV2Dto::ManualReview { outcome: value, .. } =
                    &mut invalid.payload
                else {
                    return Err("manual payload".into());
                };
                *value = outcome.to_owned();
                assert!(
                    CheckedEvidenceResultV2::from_bytes(&serde_json::to_vec(&invalid)?).is_err(),
                    "{outcome}"
                );
                let unresolved = assess_admitted_manual_reviews(&prepared, &expected, &[])?;
                assert_eq!(unresolved.outcome(), State::Missing);
                assert_eq!(unresolved.passing_slots().len(), 0);
                assert_eq!(unresolved.history().source_cases().len(), 2);
            }
            // Reviewer spelling cannot manufacture independent authority or extra slots.
            let (single_prepared, single_key, single_selection) =
                prepared_coordinates_with_selection(
                    &session,
                    "eu",
                    None,
                    vec![lineage()?.logical_case().clone()],
                    vec![],
                    independent,
                )?;
            let single_identity = single_prepared
                .identities()
                .get(&single_key)
                .ok_or("single identity")?
                .digest();
            let single_expected = vec![CompleteCoverageExpectation {
                structural: coverage_context(&context, &spec),
                coordinate: single_key.clone(),
                identity: single_identity,
                selection: single_selection,
                subjects: None,
            }];
            for reviewer in ["owner://team/reviewers", "owner://role/distinct_principal"] {
                let mut value = template.clone();
                let EvidencePayloadV2Dto::ManualReview {
                    reviewer: claim, ..
                } = &mut value.payload
                else {
                    return Err("manual payload".into());
                };
                *claim = reviewer.to_owned();
                let checked = seal(value)?;
                let admitted = admit_evidence_bytes(
                    checked.raw_bytes(),
                    &lineage()?,
                    &execution()?,
                    &context,
                )?;
                let delivery = CompleteEvidenceDelivery {
                    delivery: AdmittedEvidenceDelivery {
                        admitted: &admitted,
                        evidence_spec_id: &spec,
                        provenance: &receipt,
                    },
                    coordinate: Some(&single_key),
                    identity: Some(single_identity),
                    subjects: None,
                };
                let report = assess_admitted_manual_reviews(
                    &single_prepared,
                    &single_expected,
                    &[delivery; 20],
                )?;
                assert_eq!(report.passing_slots().len(), 1);
                assert_eq!(
                    report.outcome(),
                    if minimum == 1 {
                        State::Satisfied
                    } else {
                        State::Missing
                    }
                );
                assert_eq!(report.acceptance(), State::Unknown);
                assert_eq!(
                    report.slots().values().next().ok_or("slot")?.reviewers(),
                    &BTreeSet::from([reviewer.parse()?])
                );
                assert_eq!(report.history().histories().len(), 1);
                let mut foreign = single_expected.clone();
                foreign[0].identity = identity;
                assert!(matches!(
                    assess_admitted_manual_reviews(&single_prepared, &foreign, &[delivery]),
                    Err(eqm_application::AdmittedManualReviewError::Context(
                        eqm_application::AdmittedCountConstraintError::FrozenContext
                    ))
                ));
                assert!(
                    assess_admitted_manual_reviews(&single_prepared, &[], &[delivery]).is_err()
                );
            }
            assert_eq!(file_snapshot(repository.path())?, before);
        }
    }
    Ok(())
}

#[test]
fn actual_manual_review_sources_require_complete_slots_without_principal_claim_credit()
-> Result<(), Box<dyn Error>> {
    use eqm_application::{CompleteEvidenceDelivery, assess_admitted_manual_reviews};
    use eqm_domain::{ExpectedSelectionManifest, SelectionCountUnit};
    use eqm_engine::{
        CompleteCoverageExpectation, EvidenceOutcome as State, ManualReviewReason as Reason,
    };
    for minimum in [1, 2] {
        let repository = kind_fixture(EvidenceKind::ManualReview)?;
        let policy = repository
            .path()
            .join("eqm/policies/consumer.critical_flow.toml");
        fs::write(
            &policy,
            fs::read_to_string(&policy)?
                .replace("minimum_count = 1", &format!("minimum_count = {minimum}")),
        )?;
        let path = repository
            .path()
            .join("eqm/bindings/android.auth_signup.toml");
        let mut text = fs::read_to_string(&path)?;
        text.push_str("\n[[evidence]]\nid = \"second_review\"\nkind = \"manual_review\"\nrequirements = [\"account.create.signup.identifier#email_default\"]\nfacets = [\"behavior\"]\n");
        fs::write(&path, text)?;
        let session = session_for(repository.path())?;
        let original = projections(&session)?;
        let (_, key, first_selection) = prepared_coordinates_with_selection(
            &session,
            "eu",
            None,
            vec![lineage()?.logical_case().clone()],
            vec![],
            true,
        )?;
        let first_spec = EvidenceSpecId::new("identifier_behavior")?;
        let second_spec = EvidenceSpecId::new("second_review")?;
        let binding = session
            .finalized()
            .graph()
            .bindings()
            .get(&BindingId::new("binding.android.auth_signup")?)
            .ok_or("binding")?;
        let mut second_provenance = original.clone();
        second_provenance.evidence_spec_digest = canonicalize_evidence_spec_v2(
            binding.evidence().get(&second_spec).ok_or("second spec")?,
        )?
        .digest();
        let mut second_source = first_selection.source().clone();
        second_source.specification = second_spec.clone();
        second_source.specification_digest = second_provenance.evidence_spec_digest;
        let second_selection = ExpectedSelectionManifest::new(
            second_source,
            EvidenceKind::ManualReview,
            SelectionCountUnit::ReviewSlot,
            vec![lineage()?.logical_case().clone()],
            vec![],
            true,
        )?;
        // Freeze both required sources before admitting either assertion.
        let prepared = prepared_multi_source_identities(
            &session,
            &key,
            None,
            vec![first_selection.clone(), second_selection.clone()],
        )?;
        let identity = prepared.identities().get(&key).ok_or("identity")?.digest();
        let (first_value, first_context) = record_for_kind(&original, EvidenceKind::ManualReview)?;
        let (second_value, second_context) =
            record_for_kind(&second_provenance, EvidenceKind::ManualReview)?;
        let expected = vec![
            CompleteCoverageExpectation {
                structural: coverage_context(&first_context, &first_spec),
                coordinate: key.clone(),
                identity,
                selection: first_selection.clone(),
                subjects: None,
            },
            CompleteCoverageExpectation {
                structural: coverage_context(&second_context, &second_spec),
                coordinate: key.clone(),
                identity,
                selection: second_selection.clone(),
                subjects: None,
            },
        ];
        let first_checked = seal(first_value.clone())?;
        let second_checked = seal(second_value)?;
        let first = admit_evidence_bytes(
            first_checked.raw_bytes(),
            &lineage()?,
            &execution()?,
            &first_context,
        )?;
        let second_attempt = eqm_domain::AttemptId::new("attempt.second_review")?;
        let second_lineage = EvidenceLineage::from_parts(EvidenceLineageParts {
            plan: Some(lineage()?.plan().clone()),
            campaign: Some(lineage()?.campaign().clone()),
            run: Some(lineage()?.run().clone()),
            logical_case: Some(lineage()?.logical_case().clone()),
            attempt: Some(second_attempt.clone()),
        })?;
        let second = admit_evidence_bytes(
            second_checked.raw_bytes(),
            &second_lineage,
            &EvidenceExecution::new(
                second_lineage.run().clone(),
                second_lineage.logical_case().clone(),
                vec![second_attempt],
            )?,
            &second_context,
        )?;
        let receipt = SelectorText::new("receipt.manual.sources")?;
        let first_delivery = CompleteEvidenceDelivery {
            delivery: AdmittedEvidenceDelivery {
                admitted: &first,
                evidence_spec_id: &first_spec,
                provenance: &receipt,
            },
            coordinate: Some(&key),
            identity: Some(identity),
            subjects: None,
        };
        let second_delivery = CompleteEvidenceDelivery {
            delivery: AdmittedEvidenceDelivery {
                admitted: &second,
                evidence_spec_id: &second_spec,
                provenance: &receipt,
            },
            ..first_delivery
        };
        let before = file_snapshot(repository.path())?;
        let missing = assess_admitted_manual_reviews(&prepared, &expected, &[first_delivery; 10])?;
        assert_eq!(missing.outcome(), State::Missing);
        assert_eq!(missing.acceptance(), State::Unknown);
        assert!(missing.passing_slots().is_empty());
        assert!(missing.reasons().contains(&Reason::MissingSelectedCase(
            second_selection.source().clone(),
            lineage()?.logical_case().clone()
        )));
        let complete = assess_admitted_manual_reviews(
            &prepared,
            &expected,
            &[first_delivery, second_delivery],
        )?;
        assert_eq!(
            complete.passing_slots(),
            &BTreeSet::from([lineage()?.logical_case().clone()])
        );
        assert_eq!(
            complete.outcome(),
            if minimum == 1 {
                State::Satisfied
            } else {
                State::Missing
            }
        );
        assert_eq!(complete.acceptance(), State::Unknown);
        assert!(
            complete
                .reasons()
                .contains(&Reason::DistinctAuthorizedPrincipalsRequired)
        );
        let mut reversed_expected = expected.clone();
        reversed_expected.reverse();
        assert_eq!(
            assess_admitted_manual_reviews(
                &prepared,
                &reversed_expected,
                &[
                    second_delivery,
                    first_delivery,
                    second_delivery,
                    first_delivery
                ]
            )?,
            complete
        );
        // Rejected assertions remain raw audit and cannot poison the matched review outcome.
        let mut failed_value = first_value;
        let EvidencePayloadV2Dto::ManualReview { outcome, .. } = &mut failed_value.payload else {
            return Err("manual payload".into());
        };
        *outcome = "failed".to_owned();
        let failed_checked = seal(failed_value)?;
        let failed = admit_evidence_bytes(
            failed_checked.raw_bytes(),
            &lineage()?,
            &execution()?,
            &first_context,
        )?;
        let failed_delivery = CompleteEvidenceDelivery {
            delivery: AdmittedEvidenceDelivery {
                admitted: &failed,
                evidence_spec_id: &first_spec,
                provenance: &receipt,
            },
            ..first_delivery
        };
        let foreign_identity =
            eqm_domain::ObligationDigest::hash_content(b"synthetic foreign review claim");
        let rejected = CompleteEvidenceDelivery {
            identity: Some(foreign_identity),
            ..failed_delivery
        };
        let audited = assess_admitted_manual_reviews(
            &prepared,
            &expected,
            &[first_delivery, second_delivery, rejected],
        )?;
        assert_eq!(audited.outcome(), complete.outcome());
        assert_eq!(audited.passing_slots(), complete.passing_slots());
        assert!(
            !audited
                .reasons()
                .contains(&Reason::Terminal(eqm_domain::AttemptOutcome::Failed))
        );
        assert!(
            !audited.history().constraints().observations().matching()[first_selection.source()]
                .rejected()
                .is_empty()
        );
        let conflicted = assess_admitted_manual_reviews(
            &prepared,
            &expected,
            &[first_delivery, second_delivery, failed_delivery],
        )?;
        assert_eq!(conflicted.outcome(), State::Unknown);
        assert!(conflicted.passing_slots().is_empty());
        assert!(conflicted.reasons().contains(&Reason::ImmutableConflict));
        assert!(matches!(
            assess_admitted_manual_reviews(
                &prepared,
                &expected,
                &vec![first_delivery; MAX_CANDIDATE_DELIVERIES + 1]
            ),
            Err(eqm_application::AdmittedManualReviewError::Context(
                eqm_application::AdmittedCountConstraintError::Observations(
                    eqm_engine::LogicalObservationError::Matching(
                        eqm_engine::CompleteCoverageError::Normalization(
                            eqm_engine::CandidateNormalizationError::LimitExceeded
                        )
                    )
                )
            ))
        ));
        assert!(
            assess_admitted_manual_reviews(&prepared, &expected[..1], &[first_delivery]).is_err()
        );
        assert_eq!(file_snapshot(repository.path())?, before);
    }
    Ok(())
}
