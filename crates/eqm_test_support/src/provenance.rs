//! Shared cross-domain provenance regression corpus.
//!
//! This module is the single deterministic source for the raw, semantic,
//! lineage and record-history vectors that owning crates execute against their
//! real boundaries. It carries exact inputs and literal golden identities, not
//! a test-local substitute for a production projection. The manifest component
//! projections (binding, evidence-specification, contract, selected-policy and
//! target configuration) are locked by the existing owning `eqm_manifest` tests
//! against the checked-in fixture workspace; this module supplies the protocol,
//! lineage and record-history axes that connect them.
//!
//! Every builder delegates to the same DTO and domain constructors used in
//! production, so a vector cannot invent a value the production parser would
//! reject.

use crate::error::FixtureError;
use eqm_domain::{RecordDescriptor, RecordKind, RecordRef, Sha256Digest};
use eqm_protocol::{
    AttemptV2Dto, CanonicalInstant, CanonicalSet, CountsV2Dto, EVIDENCE_RESULT_SCHEMA_V2,
    EvidencePayloadV2Dto, EvidenceResultV2Dto, EvidenceSubjectV2Dto, ExecutionPayloadV2Dto,
    ScopeSubjectV2Dto, WireUint,
};
/// The reviewed literal canonical semantic bytes for [`standard_evidence_record`].
///
/// Independently recomputed under `eqm:v2:semantic-evidence`; these bytes are a
/// constant, not the output of the code under test.
pub const STANDARD_EVIDENCE_SEMANTIC_BYTES: &str = r##"{"attachments":[],"binding_digest":"sha256:80f70afeef3caa57646fd20afb95be9c3f2c03d38e091906de31838813dcc22e","claimed_trust":"signed_ci","contract_digest":"sha256:cc8321d6375c494d043fdd0260f21bc0ec51dacc9f6abb7f909cdcd3041b78bf","evidence_spec_digest":"sha256:d4f02eaafd1a9e9de7d10972ca8e47fa7a985825c3c9c1e249c72683cb3e4f19","facets":["behavior"],"kind":"test","observed_at":"2026-01-02T03:04:05Z","payload":{"execution":{"attempts":[{"finished_at":"2026-01-02T03:04:06Z","number":1,"outcome":"passed","started_at":"2026-01-02T03:04:05Z"}],"counts":{"failed":0,"filtered":0,"passed":1,"quarantined":0,"selected":1,"skipped":0},"finished_at":"2026-01-02T03:04:06Z","started_at":"2026-01-02T03:04:05Z"},"kind":"test"},"policy_digest":"sha256:823412d1eacb67956220e532959f0104603057c88704863ca38e7cd188fda812","producer":"producer://ci/runner/1","profile_values":[],"requirements":["unit.test#r18"],"subject":{"repository":"https://example.com/team/project","repository_id_digest":"sha256:071ca2227754705837aa3ef9748ed59e9f8a015fd765c42f391a4cbc271c6d5e","scope":{"kind":"target","target":"web"},"source_commit":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","target_configuration_digest":"sha256:b79606fb3afea5bd1609ed40b622142f1c98125abcfe89a76a661b0e8e343910"},"target":"web","unit":"unit.test"}"##;

/// The independently recomputed literal semantic identity of the standard record.
pub const STANDARD_EVIDENCE_SEMANTIC_DIGEST: &str =
    "sha256:ac16d29887befa9ca8f5dbe8ab3379c3478858d248b573582f752e33e91c7f5f";

fn digest(seed: &str) -> String {
    Sha256Digest::hash_content(seed.as_bytes()).to_string()
}

fn instant(value: &str) -> Result<CanonicalInstant, FixtureError> {
    let parsed = value.parse().map_err(FixtureError::new)?;
    Ok(CanonicalInstant::new(parsed))
}

/// Builds the exact standard evidence record used by the shared corpus.
pub fn standard_evidence_record() -> Result<EvidenceResultV2Dto, FixtureError> {
    let id = digest("identity");
    Ok(EvidenceResultV2Dto {
        schema: EVIDENCE_RESULT_SCHEMA_V2.to_string(),
        id: id.clone(),
        subject: EvidenceSubjectV2Dto {
            repository: "https://example.com/team/project".to_owned(),
            repository_id_digest: digest("repo"),
            scope: ScopeSubjectV2Dto::Target {
                target: "web".to_owned(),
            },
            source_commit: "a".repeat(40),
            build_id: None,
            artifact_digest: None,
            target_configuration_digest: digest("config"),
        },
        target: "web".to_owned(),
        unit: "unit.test".to_owned(),
        requirements: CanonicalSet::from_members(vec!["unit.test#r18".to_owned()])
            .map_err(FixtureError::new)?,
        facets: CanonicalSet::from_members(vec!["behavior".to_owned()])
            .map_err(FixtureError::new)?,
        kind: "test".to_owned(),
        evidence_spec_digest: digest("spec"),
        contract_digest: digest("contract"),
        binding_digest: digest("binding"),
        policy_digest: digest("policy"),
        runner_digest: None,
        adapter_digest: None,
        runtime_facts_digest: None,
        release_record_digest: None,
        profile_values: CanonicalSet::empty(),
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
                started_at: instant("2026-01-02T03:04:05Z")?,
                finished_at: instant("2026-01-02T03:04:06Z")?,
            },
        },
        attachments: CanonicalSet::empty(),
        result_digest: id,
    })
}

/// One bound semantic axis of the standard evidence record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceAxis {
    /// The envelope kind token.
    Kind,
    /// The subject repository identity.
    SubjectRepository,
    /// The subject repository-id digest.
    SubjectRepositoryId,
    /// The subject scope.
    SubjectScope,
    /// The subject source commit.
    SubjectSourceCommit,
    /// The subject build identifier.
    SubjectBuild,
    /// The subject target-configuration digest.
    SubjectTargetConfiguration,
    /// The execution target.
    Target,
    /// The owning unit.
    Unit,
    /// A covered requirement.
    Requirement,
    /// A facet.
    Facet,
    /// The evidence-specification digest.
    EvidenceSpecDigest,
    /// The contract digest.
    ContractDigest,
    /// The binding digest.
    BindingDigest,
    /// The selected-policy digest.
    PolicyDigest,
    /// The selected profile values.
    ProfileValues,
    /// The observation instant.
    ObservedAt,
    /// The payload kind and body.
    PayloadKind,
}

impl EvidenceAxis {
    /// Every bound semantic axis, so a corpus test can execute all of them.
    pub const ALL: [Self; 18] = [
        Self::Kind,
        Self::SubjectRepository,
        Self::SubjectRepositoryId,
        Self::SubjectScope,
        Self::SubjectSourceCommit,
        Self::SubjectBuild,
        Self::SubjectTargetConfiguration,
        Self::Target,
        Self::Unit,
        Self::Requirement,
        Self::Facet,
        Self::EvidenceSpecDigest,
        Self::ContractDigest,
        Self::BindingDigest,
        Self::PolicyDigest,
        Self::ProfileValues,
        Self::ObservedAt,
        Self::PayloadKind,
    ];
}

/// Returns a copy of `record` with exactly one bound semantic axis changed.
pub fn mutate_axis(
    record: &EvidenceResultV2Dto,
    axis: EvidenceAxis,
) -> Result<EvidenceResultV2Dto, FixtureError> {
    let mut value = record.clone();
    match axis {
        EvidenceAxis::Kind => value.kind = "snapshot".to_owned(),
        EvidenceAxis::SubjectRepository => {
            value.subject.repository = "https://example.com/team/other".to_owned();
        }
        EvidenceAxis::SubjectRepositoryId => {
            value.subject.repository_id_digest = digest("repo-2");
        }
        EvidenceAxis::SubjectScope => {
            value.subject.scope = ScopeSubjectV2Dto::TargetSet {
                targets: CanonicalSet::from_members(vec!["web".to_owned(), "ios".to_owned()])
                    .map_err(FixtureError::new)?,
            };
        }
        EvidenceAxis::SubjectSourceCommit => value.subject.source_commit = "b".repeat(40),
        EvidenceAxis::SubjectBuild => value.subject.build_id = Some("build.one".to_owned()),
        EvidenceAxis::SubjectTargetConfiguration => {
            value.subject.target_configuration_digest = digest("config-2");
        }
        EvidenceAxis::Target => value.target = "ios".to_owned(),
        EvidenceAxis::Unit => value.unit = "unit.other".to_owned(),
        EvidenceAxis::Requirement => {
            value.requirements = CanonicalSet::from_members(vec!["unit.test#r19".to_owned()])
                .map_err(FixtureError::new)?;
        }
        EvidenceAxis::Facet => {
            value.facets = CanonicalSet::from_members(vec!["structure".to_owned()])
                .map_err(FixtureError::new)?;
        }
        EvidenceAxis::EvidenceSpecDigest => value.evidence_spec_digest = digest("spec-2"),
        EvidenceAxis::ContractDigest => value.contract_digest = digest("contract-2"),
        EvidenceAxis::BindingDigest => value.binding_digest = digest("binding-2"),
        EvidenceAxis::PolicyDigest => value.policy_digest = digest("policy-2"),
        EvidenceAxis::ProfileValues => {
            value.profile_values =
                CanonicalSet::from_members(vec![eqm_protocol::ProfileValueV2Dto {
                    profile: "audience.default".to_owned(),
                    revision: WireUint::new(1),
                    dimension: "region".to_owned(),
                    value: "eu".to_owned(),
                }])
                .map_err(FixtureError::new)?;
        }
        EvidenceAxis::ObservedAt => value.observed_at = instant("2026-01-02T03:04:06Z")?,
        EvidenceAxis::PayloadKind => {
            value.payload = EvidencePayloadV2Dto::Snapshot {
                execution: match &record.payload {
                    EvidencePayloadV2Dto::Test { execution }
                    | EvidencePayloadV2Dto::StructuralCheck { execution }
                    | EvidencePayloadV2Dto::Snapshot { execution } => execution.clone(),
                    other => {
                        return Err(FixtureError::new(format!(
                            "standard record payload is not execution-bearing: {other:?}"
                        )));
                    }
                },
            };
        }
    }
    Ok(value)
}

fn reference(kind: RecordKind, name: &str) -> Result<RecordRef, FixtureError> {
    Ok(RecordRef::new(
        kind,
        name.parse().map_err(FixtureError::new)?,
    ))
}

fn record(
    kind: RecordKind,
    name: &str,
    references: Vec<RecordRef>,
) -> Result<RecordDescriptor, FixtureError> {
    RecordDescriptor::new(kind, name.parse().map_err(FixtureError::new)?, references)
        .map_err(FixtureError::new)
}

/// The expected outcome of one record-history vector.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordHistoryOutcome {
    /// The supplied definition history is realizable and validates.
    Accept,
    /// A reference points to a later definition or lifecycle row.
    Future,
    /// Two records share one kind and identity.
    Duplicate,
    /// A reference names a definition absent from the history.
    Missing,
    /// The directed references form a cycle.
    Cyclic,
    /// A descriptor referenced itself.
    SelfReference,
    /// A descriptor declared the same reference twice.
    DuplicateReference,
}

/// One exact record-history vector.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordHistoryVector {
    /// Stable vector identifier.
    pub id: &'static str,
    /// The caller-declared definition history.
    pub records: Vec<RecordDescriptor>,
    /// The expected checked outcome.
    pub outcome: RecordHistoryOutcome,
}

/// Returns every shared record-history vector.
///
/// Each vector is a deterministic, explicitly ordered definition history. The
/// owning `eqm_domain` test executes them against `validate_records`.
#[allow(clippy::vec_init_then_push)]
pub fn record_history_vectors() -> Result<Vec<RecordHistoryVector>, FixtureError> {
    let mut vectors = Vec::with_capacity(8);

    vectors.push(RecordHistoryVector {
        id: "RCH-LIFECYCLE",
        records: vec![
            record(RecordKind::Authority, "authority.baseline", vec![])?,
            record(
                RecordKind::Candidate,
                "candidate.one",
                vec![reference(RecordKind::Authority, "authority.baseline")?],
            )?,
            record(
                RecordKind::Plan,
                "plan.release",
                vec![reference(RecordKind::Candidate, "candidate.one")?],
            )?,
            record(
                RecordKind::Campaign,
                "campaign.signup",
                vec![reference(RecordKind::Plan, "plan.release")?],
            )?,
            record(
                RecordKind::Evidence,
                "evidence.one",
                vec![reference(RecordKind::Campaign, "campaign.signup")?],
            )?,
            record(
                RecordKind::Decision,
                "decision.one",
                vec![reference(RecordKind::Evidence, "evidence.one")?],
            )?,
            record(
                RecordKind::Statement,
                "statement.one",
                vec![reference(RecordKind::Decision, "decision.one")?],
            )?,
            record(
                RecordKind::Authorization,
                "authorization.one",
                vec![reference(RecordKind::Statement, "statement.one")?],
            )?,
            record(
                RecordKind::DeploymentReceipt,
                "deployment.one",
                vec![reference(RecordKind::Authorization, "authorization.one")?],
            )?,
        ],
        outcome: RecordHistoryOutcome::Accept,
    });

    vectors.push(RecordHistoryVector {
        id: "RCH-HISTORICAL-DEPLOYMENT-INPUT",
        records: vec![
            record(RecordKind::DeploymentReceipt, "deployment.one", vec![])?,
            record(
                RecordKind::Runtime,
                "runtime.post_deploy",
                vec![reference(RecordKind::DeploymentReceipt, "deployment.one")?],
            )?,
            record(
                RecordKind::FrozenInput,
                "frozen.post_deploy",
                vec![
                    reference(RecordKind::Runtime, "runtime.post_deploy")?,
                    reference(RecordKind::DeploymentReceipt, "deployment.one")?,
                ],
            )?,
            record(
                RecordKind::Decision,
                "decision.post_deploy",
                vec![reference(RecordKind::FrozenInput, "frozen.post_deploy")?],
            )?,
        ],
        outcome: RecordHistoryOutcome::Accept,
    });

    vectors.push(RecordHistoryVector {
        id: "RCH-FORWARD-DEFINITION",
        records: vec![
            record(
                RecordKind::Plan,
                "plan.a",
                vec![reference(RecordKind::Plan, "plan.b")?],
            )?,
            record(RecordKind::Plan, "plan.b", vec![])?,
        ],
        outcome: RecordHistoryOutcome::Future,
    });

    vectors.push(RecordHistoryVector {
        id: "RCH-FUTURE-RECEIPT",
        records: vec![
            record(
                RecordKind::Runtime,
                "runtime.one",
                vec![reference(
                    RecordKind::DeploymentReceipt,
                    "deployment.later",
                )?],
            )?,
            record(RecordKind::DeploymentReceipt, "deployment.later", vec![])?,
        ],
        outcome: RecordHistoryOutcome::Future,
    });

    vectors.push(RecordHistoryVector {
        id: "RCH-INITIAL-READINESS-DEPLOYMENT",
        records: vec![
            record(RecordKind::DeploymentReceipt, "deployment.one", vec![])?,
            record(
                RecordKind::Decision,
                "decision.one",
                vec![reference(RecordKind::DeploymentReceipt, "deployment.one")?],
            )?,
        ],
        outcome: RecordHistoryOutcome::Future,
    });

    vectors.push(RecordHistoryVector {
        id: "RCH-DUPLICATE-RECORD",
        records: vec![
            record(RecordKind::Plan, "plan.release", vec![])?,
            record(RecordKind::Plan, "plan.release", vec![])?,
        ],
        outcome: RecordHistoryOutcome::Duplicate,
    });

    vectors.push(RecordHistoryVector {
        id: "RCH-MISSING",
        records: vec![record(
            RecordKind::Evidence,
            "evidence.one",
            vec![reference(RecordKind::Plan, "plan.absent")?],
        )?],
        outcome: RecordHistoryOutcome::Missing,
    });

    vectors.push(RecordHistoryVector {
        id: "RCH-CYCLE",
        records: vec![
            record(
                RecordKind::Plan,
                "plan.a",
                vec![reference(RecordKind::Plan, "plan.b")?],
            )?,
            record(
                RecordKind::Plan,
                "plan.b",
                vec![reference(RecordKind::Plan, "plan.a")?],
            )?,
        ],
        outcome: RecordHistoryOutcome::Cyclic,
    });

    Ok(vectors)
}
