//! In-toto Statement v1 and DSSE-compatible EQM attestation DTOs.

#![allow(missing_docs)]

use crate::{ATTESTATION_SCHEMA, EvidenceSubjectDto, ProfileValueDto};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const IN_TOTO_STATEMENT_V1: &str = "https://in-toto.io/Statement/v1";
pub const DSSE_PAYLOAD_TYPE: &str = "application/vnd.in-toto+json";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SubjectDigestDto {
    pub sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AttestationSubjectDto {
    pub name: String,
    pub digest: SubjectDigestDto,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AttestationPredicateDto {
    pub tool_version: String,
    pub command: String,
    pub workspace_digest: String,
    pub policy_digest: String,
    pub profile_values: Vec<ProfileValueDto>,
    pub evaluation_subject: EvidenceSubjectDto,
    pub evidence_digests: BTreeSet<String>,
    pub runtime_facts_digest: Option<String>,
    pub release_record_digest: Option<String>,
    pub trust_config_digest: String,
    pub evaluated_at: String,
    pub conformance: String,
    pub equivalence: String,
    pub release_status: String,
    pub waivers: BTreeSet<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InTotoStatementDto {
    #[serde(rename = "_type")]
    pub statement_type: String,
    pub subject: Vec<AttestationSubjectDto>,
    #[serde(rename = "predicateType")]
    pub predicate_type: String,
    pub predicate: AttestationPredicateDto,
}

impl InTotoStatementDto {
    pub fn new(
        mut subject: Vec<AttestationSubjectDto>,
        mut predicate: AttestationPredicateDto,
    ) -> Result<Self, AttestationDtoError> {
        if subject.is_empty() {
            return Err(AttestationDtoError::SubjectRequired);
        }
        subject.sort_by(|left, right| {
            (&left.name, &left.digest.sha256).cmp(&(&right.name, &right.digest.sha256))
        });
        let count = subject.len();
        subject.dedup();
        if subject.len() != count {
            return Err(AttestationDtoError::DuplicateSubject);
        }
        predicate.profile_values.sort_unstable();
        Ok(Self {
            statement_type: IN_TOTO_STATEMENT_V1.to_owned(),
            subject,
            predicate_type: ATTESTATION_SCHEMA.to_string(),
            predicate,
        })
    }

    pub fn from_json(bytes: &[u8]) -> Result<Self, AttestationDtoError> {
        let value: Self = serde_json::from_slice(bytes).map_err(|_| AttestationDtoError::Json)?;
        if value.statement_type != IN_TOTO_STATEMENT_V1
            || value.predicate_type != ATTESTATION_SCHEMA.to_string()
        {
            return Err(AttestationDtoError::InvalidType);
        }
        Self::new(value.subject, value.predicate)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DsseSignatureDto {
    pub keyid: String,
    pub sig: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DsseEnvelopeDto {
    #[serde(rename = "payloadType")]
    pub payload_type: String,
    pub payload: String,
    pub signatures: BTreeSet<DsseSignatureDto>,
}

impl DsseEnvelopeDto {
    #[must_use]
    pub fn unsigned(payload: String) -> Self {
        Self {
            payload_type: DSSE_PAYLOAD_TYPE.to_owned(),
            payload,
            signatures: BTreeSet::new(),
        }
    }
    #[must_use]
    pub fn is_signed(&self) -> bool {
        !self.signatures.is_empty()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AttestationDtoError {
    Json,
    InvalidType,
    SubjectRequired,
    DuplicateSubject,
}
impl std::fmt::Display for AttestationDtoError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl std::error::Error for AttestationDtoError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dsse_signature_state_is_explicit() {
        let envelope = DsseEnvelopeDto::unsigned("e30".to_owned());
        assert!(!envelope.is_signed());
        assert_eq!(
            serde_json::to_string(&envelope).ok().as_deref(),
            Some(
                "{\"payloadType\":\"application/vnd.in-toto+json\",\"payload\":\"e30\",\"signatures\":[]}"
            )
        );
    }

    #[test]
    fn statement_rejects_empty_subjects() {
        let predicate = AttestationPredicateDto {
            tool_version: "0.1.0".to_owned(),
            command: "attest".to_owned(),
            workspace_digest: "digest".to_owned(),
            policy_digest: "digest".to_owned(),
            profile_values: Vec::new(),
            evaluation_subject: EvidenceSubjectDto {
                repository: "https://github.com/example/repo".to_owned(),
                repository_id_digest: "digest".to_owned(),
                scope: crate::ScopeSubjectDto::Target {
                    target: "web".to_owned(),
                },
                source_commit: "a".repeat(40),
                build_id: None,
                artifact_digest: None,
                target_configuration_digest: "digest".to_owned(),
            },
            evidence_digests: BTreeSet::new(),
            runtime_facts_digest: None,
            release_record_digest: None,
            trust_config_digest: "digest".to_owned(),
            evaluated_at: "2026-08-07T12:00:00Z".to_owned(),
            conformance: "conformant".to_owned(),
            equivalence: "equivalent".to_owned(),
            release_status: "pass".to_owned(),
            waivers: BTreeSet::new(),
        };
        assert_eq!(
            InTotoStatementDto::new(Vec::new(), predicate),
            Err(AttestationDtoError::SubjectRequired)
        );
    }
}
