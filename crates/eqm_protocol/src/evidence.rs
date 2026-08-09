//! Normalized test-result and immutable evidence-result DTOs.

#![allow(missing_docs)]

use crate::{EVIDENCE_RESULT_SCHEMA, ProfileValueDto, TEST_RESULT_SCHEMA};
use eqm_domain::{
    AttemptOutcome, EvidenceAttachment, EvidenceAttempt, EvidenceCounts, EvidenceKind,
    EvidencePayload, EvidenceResult, EvidenceScopeSubject, EvidenceSelector, EvidenceSubject,
    ExecutionPayload, PositiveCount, ProducerRef, Sha256Digest, TrustLevel, UtcInstant,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CountsDto {
    pub selected: u64,
    pub passed: u64,
    pub failed: u64,
    pub skipped: u64,
    pub filtered: u64,
    pub quarantined: u64,
}

impl From<EvidenceCounts> for CountsDto {
    fn from(value: EvidenceCounts) -> Self {
        Self {
            selected: value.selected,
            passed: value.passed,
            failed: value.failed,
            skipped: value.skipped,
            filtered: value.filtered,
            quarantined: value.quarantined,
        }
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AttemptDto {
    pub number: u64,
    pub outcome: String,
    pub started_at: String,
    pub finished_at: String,
    pub message: Option<String>,
}

impl From<&EvidenceAttempt> for AttemptDto {
    fn from(value: &EvidenceAttempt) -> Self {
        Self {
            number: value.number().get(),
            outcome: value.outcome().to_string(),
            started_at: value.started_at().to_string(),
            finished_at: value.finished_at().to_string(),
            message: value.message().map(|item| item.as_str().to_owned()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum EvidenceSelectorDto {
    Symbol {
        name: String,
        language: Option<String>,
    },
    Route {
        path: String,
        method: Option<String>,
    },
    Test {
        framework: String,
        test_id: String,
        suite: Option<String>,
    },
    Inventory {
        record_type: String,
        key: String,
        value: Option<String>,
    },
    Snapshot {
        snapshot_id: String,
        variant: Option<String>,
    },
    Release {
        channel: String,
    },
}

impl From<&EvidenceSelector> for EvidenceSelectorDto {
    fn from(value: &EvidenceSelector) -> Self {
        match value {
            EvidenceSelector::Symbol { name, language } => Self::Symbol {
                name: name.as_str().to_owned(),
                language: language.as_ref().map(|item| item.as_str().to_owned()),
            },
            EvidenceSelector::Route { path, method } => Self::Route {
                path: path.as_str().to_owned(),
                method: method.map(|item| item.to_string()),
            },
            EvidenceSelector::Test {
                framework,
                test_id,
                suite,
            } => Self::Test {
                framework: framework.as_str().to_owned(),
                test_id: test_id.as_str().to_owned(),
                suite: suite.as_ref().map(|item| item.as_str().to_owned()),
            },
            EvidenceSelector::Inventory {
                record_type,
                key,
                value,
            } => Self::Inventory {
                record_type: record_type.as_str().to_owned(),
                key: key.as_str().to_owned(),
                value: value.as_ref().map(|item| item.as_str().to_owned()),
            },
            EvidenceSelector::Snapshot {
                snapshot_id,
                variant,
            } => Self::Snapshot {
                snapshot_id: snapshot_id.as_str().to_owned(),
                variant: variant.as_ref().map(|item| item.as_str().to_owned()),
            },
            EvidenceSelector::Release { channel } => Self::Release {
                channel: channel.to_string(),
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AttachmentDto {
    pub name: String,
    pub media_type: String,
    pub digest: String,
    pub size: u64,
}

impl From<&EvidenceAttachment> for AttachmentDto {
    fn from(value: &EvidenceAttachment) -> Self {
        Self {
            name: value.name().as_str().to_owned(),
            media_type: value.media_type().as_str().to_owned(),
            digest: value.digest().to_string(),
            size: value.size(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TestResultDto {
    pub schema: String,
    pub selector: EvidenceSelectorDto,
    pub attempts: Vec<AttemptDto>,
    pub counts: CountsDto,
    pub started_at: String,
    pub finished_at: String,
    pub attachments: BTreeSet<AttachmentDto>,
}

impl TestResultDto {
    pub fn from_execution(
        selector: &EvidenceSelector,
        payload: &ExecutionPayload,
        attachments: impl Iterator<Item = AttachmentDto>,
    ) -> Self {
        Self {
            schema: TEST_RESULT_SCHEMA.to_string(),
            selector: selector.into(),
            attempts: payload.attempts().iter().map(AttemptDto::from).collect(),
            counts: payload.counts().into(),
            started_at: payload.started_at().to_string(),
            finished_at: payload.finished_at().to_string(),
            attachments: attachments.collect(),
        }
    }

    pub fn from_json(bytes: &[u8]) -> Result<Self, EvidenceDtoError> {
        let value: Self = serde_json::from_slice(bytes).map_err(|_| EvidenceDtoError::Json)?;
        if value.schema != TEST_RESULT_SCHEMA.to_string() {
            return Err(EvidenceDtoError::InvalidSchema);
        }
        validate_execution(
            &value.attempts,
            &value.counts,
            &value.started_at,
            &value.finished_at,
        )?;
        Ok(value)
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ScopeSubjectDto {
    Target { target: String },
    Provider { provider: String },
    TargetSet { targets: BTreeSet<String> },
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceSubjectDto {
    pub repository: String,
    pub repository_id_digest: String,
    pub scope: ScopeSubjectDto,
    pub source_commit: String,
    pub build_id: Option<String>,
    pub artifact_digest: Option<String>,
    pub target_configuration_digest: String,
}

impl From<&EvidenceSubject> for EvidenceSubjectDto {
    fn from(value: &EvidenceSubject) -> Self {
        let scope = match value.scope() {
            EvidenceScopeSubject::Target(target) => ScopeSubjectDto::Target {
                target: target.as_str().to_owned(),
            },
            EvidenceScopeSubject::Provider(provider) => ScopeSubjectDto::Provider {
                provider: provider.as_str().to_owned(),
            },
            EvidenceScopeSubject::TargetSet(targets) => ScopeSubjectDto::TargetSet {
                targets: targets
                    .iter()
                    .map(|item| item.as_str().to_owned())
                    .collect(),
            },
        };
        Self {
            repository: value.repository().as_str().to_owned(),
            repository_id_digest: value.repository_id_digest().to_string(),
            scope,
            source_commit: value.source_commit().as_str().to_owned(),
            build_id: value.build_id().map(|item| item.as_str().to_owned()),
            artifact_digest: value.artifact_digest().map(|item| item.to_string()),
            target_configuration_digest: value.target_configuration_digest().to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionPayloadDto {
    pub attempts: Vec<AttemptDto>,
    pub counts: CountsDto,
    pub started_at: String,
    pub finished_at: String,
}

impl From<&ExecutionPayload> for ExecutionPayloadDto {
    fn from(value: &ExecutionPayload) -> Self {
        Self {
            attempts: value.attempts().iter().map(AttemptDto::from).collect(),
            counts: value.counts().into(),
            started_at: value.started_at().to_string(),
            finished_at: value.finished_at().to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum EvidencePayloadDto {
    StructuralCheck {
        execution: ExecutionPayloadDto,
    },
    StaticInventory {
        inventory_digest: String,
        counts: CountsDto,
    },
    Test {
        execution: ExecutionPayloadDto,
    },
    Snapshot {
        execution: ExecutionPayloadDto,
    },
    ManualReview {
        outcome: String,
        reviewer: String,
        message: Option<String>,
    },
    RuntimeSnapshot {
        runtime_facts_digest: String,
        counts: CountsDto,
    },
    ReleaseRecord {
        release_record_digest: String,
    },
}

impl From<&EvidencePayload> for EvidencePayloadDto {
    fn from(value: &EvidencePayload) -> Self {
        match value {
            EvidencePayload::StructuralCheck(item) => Self::StructuralCheck {
                execution: item.into(),
            },
            EvidencePayload::StaticInventory {
                inventory_digest,
                counts,
            } => Self::StaticInventory {
                inventory_digest: inventory_digest.to_string(),
                counts: (*counts).into(),
            },
            EvidencePayload::Test(item) => Self::Test {
                execution: item.into(),
            },
            EvidencePayload::Snapshot(item) => Self::Snapshot {
                execution: item.into(),
            },
            EvidencePayload::ManualReview {
                outcome,
                reviewer,
                message,
            } => Self::ManualReview {
                outcome: outcome.to_string(),
                reviewer: reviewer.as_str().to_owned(),
                message: message.as_ref().map(|item| item.as_str().to_owned()),
            },
            EvidencePayload::RuntimeSnapshot {
                runtime_facts_digest,
                counts,
            } => Self::RuntimeSnapshot {
                runtime_facts_digest: runtime_facts_digest.to_string(),
                counts: (*counts).into(),
            },
            EvidencePayload::ReleaseRecord {
                release_record_digest,
            } => Self::ReleaseRecord {
                release_record_digest: release_record_digest.to_string(),
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceResultDto {
    pub schema: String,
    pub id: String,
    pub subject: EvidenceSubjectDto,
    pub target: String,
    pub unit: String,
    pub requirements: BTreeSet<String>,
    pub facets: BTreeSet<String>,
    pub kind: String,
    pub evidence_spec_digest: String,
    pub contract_digest: String,
    pub binding_digest: String,
    pub policy_digest: String,
    pub runner_digest: Option<String>,
    pub adapter_digest: Option<String>,
    pub runtime_facts_digest: Option<String>,
    pub release_record_digest: Option<String>,
    pub profile_values: Vec<ProfileValueDto>,
    pub producer: String,
    pub claimed_trust: String,
    pub observed_at: String,
    pub payload: EvidencePayloadDto,
    pub attachments: BTreeSet<AttachmentDto>,
    pub result_digest: String,
}

impl From<&EvidenceResult> for EvidenceResultDto {
    fn from(value: &EvidenceResult) -> Self {
        let mut profile_values: Vec<_> = value
            .profile_values()
            .values()
            .flat_map(ProfileValueDto::from_selection)
            .collect();
        profile_values.sort_unstable();
        Self {
            schema: EVIDENCE_RESULT_SCHEMA.to_string(),
            id: value.id().to_string(),
            subject: value.subject().into(),
            target: value.target().as_str().to_owned(),
            unit: value.unit().as_str().to_owned(),
            requirements: value
                .requirements()
                .iter()
                .map(|item| item.as_str().to_owned())
                .collect(),
            facets: value.facets().iter().map(|item| item.to_string()).collect(),
            kind: value.kind().to_string(),
            evidence_spec_digest: value.evidence_spec_digest().to_string(),
            contract_digest: value.contract_digest().to_string(),
            binding_digest: value.binding_digest().to_string(),
            policy_digest: value.policy_digest().to_string(),
            runner_digest: value.runner_digest().map(|item| item.to_string()),
            adapter_digest: value.adapter_digest().map(|item| item.to_string()),
            runtime_facts_digest: value.runtime_facts_digest().map(|item| item.to_string()),
            release_record_digest: value.release_record_digest().map(|item| item.to_string()),
            profile_values,
            producer: value.producer().as_str().to_owned(),
            claimed_trust: value.claimed_trust().to_string(),
            observed_at: value.observed_at().to_string(),
            payload: value.payload().into(),
            attachments: value
                .attachments()
                .values()
                .map(AttachmentDto::from)
                .collect(),
            result_digest: value.result_digest().to_string(),
        }
    }
}

impl EvidenceResultDto {
    pub fn from_json(bytes: &[u8]) -> Result<Self, EvidenceDtoError> {
        let value: Self = serde_json::from_slice(bytes).map_err(|_| EvidenceDtoError::Json)?;
        if value.schema != EVIDENCE_RESULT_SCHEMA.to_string() {
            return Err(EvidenceDtoError::InvalidSchema);
        }
        let id: Sha256Digest = value
            .id
            .parse()
            .map_err(|_| EvidenceDtoError::InvalidDigest)?;
        let result: Sha256Digest = value
            .result_digest
            .parse()
            .map_err(|_| EvidenceDtoError::InvalidDigest)?;
        if id != result {
            return Err(EvidenceDtoError::IdentityMismatch);
        }
        for digest in [
            &value.evidence_spec_digest,
            &value.contract_digest,
            &value.binding_digest,
            &value.policy_digest,
        ] {
            let _: Sha256Digest = digest
                .parse()
                .map_err(|_| EvidenceDtoError::InvalidDigest)?;
        }
        for digest in [
            &value.runner_digest,
            &value.adapter_digest,
            &value.runtime_facts_digest,
            &value.release_record_digest,
        ]
        .into_iter()
        .flatten()
        {
            let _: Sha256Digest = digest
                .parse()
                .map_err(|_| EvidenceDtoError::InvalidDigest)?;
        }
        let _: EvidenceKind = value
            .kind
            .parse()
            .map_err(|_| EvidenceDtoError::InvalidVocabulary)?;
        let _: TrustLevel = value
            .claimed_trust
            .parse()
            .map_err(|_| EvidenceDtoError::InvalidVocabulary)?;
        let _: ProducerRef = value
            .producer
            .parse()
            .map_err(|_| EvidenceDtoError::InvalidVocabulary)?;
        let _: UtcInstant = value
            .observed_at
            .parse()
            .map_err(|_| EvidenceDtoError::InvalidTime)?;
        validate_payload(&value.payload)?;
        for attachment in &value.attachments {
            let _: Sha256Digest = attachment
                .digest
                .parse()
                .map_err(|_| EvidenceDtoError::InvalidDigest)?;
        }
        Ok(value)
    }
}

fn validate_counts(value: &CountsDto) -> Result<EvidenceCounts, EvidenceDtoError> {
    EvidenceCounts::new(
        value.selected,
        value.passed,
        value.failed,
        value.skipped,
        value.filtered,
        value.quarantined,
    )
    .map_err(|_| EvidenceDtoError::InvalidCounts)
}

fn validate_execution(
    attempts: &[AttemptDto],
    counts: &CountsDto,
    started_at: &str,
    finished_at: &str,
) -> Result<(), EvidenceDtoError> {
    let started: UtcInstant = started_at
        .parse()
        .map_err(|_| EvidenceDtoError::InvalidTime)?;
    let finished: UtcInstant = finished_at
        .parse()
        .map_err(|_| EvidenceDtoError::InvalidTime)?;
    let attempts = attempts
        .iter()
        .map(|attempt| {
            EvidenceAttempt::new(
                PositiveCount::new(attempt.number).map_err(|_| EvidenceDtoError::InvalidAttempt)?,
                attempt
                    .outcome
                    .parse::<AttemptOutcome>()
                    .map_err(|_| EvidenceDtoError::InvalidVocabulary)?,
                attempt
                    .started_at
                    .parse()
                    .map_err(|_| EvidenceDtoError::InvalidTime)?,
                attempt
                    .finished_at
                    .parse()
                    .map_err(|_| EvidenceDtoError::InvalidTime)?,
                None,
            )
            .map_err(|_| EvidenceDtoError::InvalidAttempt)
        })
        .collect::<Result<Vec<_>, _>>()?;
    ExecutionPayload::new(attempts, validate_counts(counts)?, started, finished)
        .map_err(|_| EvidenceDtoError::InvalidAttempt)?;
    Ok(())
}

fn validate_payload(value: &EvidencePayloadDto) -> Result<(), EvidenceDtoError> {
    match value {
        EvidencePayloadDto::StructuralCheck { execution }
        | EvidencePayloadDto::Test { execution }
        | EvidencePayloadDto::Snapshot { execution } => validate_execution(
            &execution.attempts,
            &execution.counts,
            &execution.started_at,
            &execution.finished_at,
        ),
        EvidencePayloadDto::StaticInventory {
            inventory_digest,
            counts,
        } => {
            let _: Sha256Digest = inventory_digest
                .parse()
                .map_err(|_| EvidenceDtoError::InvalidDigest)?;
            validate_counts(counts).map(|_| ())
        }
        EvidencePayloadDto::RuntimeSnapshot {
            runtime_facts_digest,
            counts,
        } => {
            let _: Sha256Digest = runtime_facts_digest
                .parse()
                .map_err(|_| EvidenceDtoError::InvalidDigest)?;
            validate_counts(counts).map(|_| ())
        }
        EvidencePayloadDto::ReleaseRecord {
            release_record_digest,
        } => release_record_digest
            .parse::<Sha256Digest>()
            .map(|_| ())
            .map_err(|_| EvidenceDtoError::InvalidDigest),
        EvidencePayloadDto::ManualReview { outcome, .. } => match outcome.parse::<AttemptOutcome>()
        {
            Ok(AttemptOutcome::Passed | AttemptOutcome::Failed) => Ok(()),
            _ => Err(EvidenceDtoError::InvalidVocabulary),
        },
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceDtoError {
    Json,
    InvalidSchema,
    InvalidDigest,
    IdentityMismatch,
    InvalidCounts,
    InvalidAttempt,
    InvalidTime,
    InvalidVocabulary,
}

impl std::fmt::Display for EvidenceDtoError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for EvidenceDtoError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalized_counts_have_exact_fields_and_reject_unknowns() {
        let json =
            r#"{"selected":1,"passed":1,"failed":0,"skipped":0,"filtered":0,"quarantined":0}"#;
        assert!(serde_json::from_str::<CountsDto>(json).is_ok());
        assert!(serde_json::from_str::<CountsDto>(&json.replace('}', ",\"other\":0}")).is_err());
    }

    #[test]
    fn payload_variants_are_closed_and_discriminated() {
        let value = EvidencePayloadDto::ReleaseRecord {
            release_record_digest:
                "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_owned(),
        };
        assert_eq!(
            serde_json::to_string(&value).ok().as_deref(),
            Some(
                "{\"kind\":\"release_record\",\"release_record_digest\":\"sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\"}"
            )
        );
    }

    #[test]
    fn executable_validation_rejects_inconsistent_counts() {
        let json = br#"{"schema":"https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/protocol/test-result.schema.json","selector":{"kind":"release","channel":"production"},"attempts":[{"number":1,"outcome":"passed","started_at":"2026-08-07T12:00:00Z","finished_at":"2026-08-07T12:00:01Z","message":null}],"counts":{"selected":2,"passed":1,"failed":0,"skipped":0,"filtered":0,"quarantined":0},"started_at":"2026-08-07T12:00:00Z","finished_at":"2026-08-07T12:00:01Z","attachments":[]}"#;
        assert_eq!(
            TestResultDto::from_json(json),
            Err(EvidenceDtoError::InvalidCounts)
        );
        assert_eq!(
            validate_payload(&EvidencePayloadDto::ManualReview {
                outcome: "skipped".to_owned(),
                reviewer: "owner://team/reviewers".to_owned(),
                message: None,
            }),
            Err(EvidenceDtoError::InvalidVocabulary)
        );
    }
}
