//! Runtime-facts and release-record public DTOs.

#![allow(missing_docs)]

use crate::{
    EvidenceSubjectDto, FactValueDto, ProfileValueDto, RELEASE_RECORD_SCHEMA, RUNTIME_FACTS_SCHEMA,
};
use eqm_domain::{ReleaseRecord, RuntimeFact, RuntimeFactsSnapshot};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeFactDto {
    pub surface: String,
    pub dimension: String,
    pub key: String,
    pub value: FactValueDto,
}

impl From<&RuntimeFact> for RuntimeFactDto {
    fn from(value: &RuntimeFact) -> Self {
        Self {
            surface: value.surface().as_str().to_owned(),
            dimension: value.dimension().as_str().to_owned(),
            key: value.key().as_str().to_owned(),
            value: value.value().into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeFactsDto {
    pub schema: String,
    pub provider: String,
    pub subject: EvidenceSubjectDto,
    pub target: String,
    pub profile_values: Vec<ProfileValueDto>,
    pub observed_at: String,
    pub expires_at: String,
    pub facts: Vec<RuntimeFactDto>,
    pub producer: String,
    pub claimed_trust: String,
    pub facts_digest: String,
}

impl From<&RuntimeFactsSnapshot> for RuntimeFactsDto {
    fn from(value: &RuntimeFactsSnapshot) -> Self {
        let mut profile_values: Vec<_> = value
            .profile_values()
            .values()
            .flat_map(ProfileValueDto::from_selection)
            .collect();
        profile_values.sort_unstable();
        Self {
            schema: RUNTIME_FACTS_SCHEMA.to_string(),
            provider: value.provider().as_str().to_owned(),
            subject: value.subject().into(),
            target: value.target().as_str().to_owned(),
            profile_values,
            observed_at: value.observed_at().to_string(),
            expires_at: value.expires_at().to_string(),
            facts: value.facts().values().map(RuntimeFactDto::from).collect(),
            producer: value.producer().as_str().to_owned(),
            claimed_trust: value.claimed_trust().to_string(),
            facts_digest: value.facts_digest().to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseRecordDto {
    pub schema: String,
    pub target: String,
    pub app_version: String,
    pub build_number: String,
    pub source_commit: String,
    pub artifact_digest: String,
    pub channel: String,
    pub released_at: String,
    pub producer: String,
    pub claimed_trust: String,
    pub record_digest: String,
}

impl From<&ReleaseRecord> for ReleaseRecordDto {
    fn from(value: &ReleaseRecord) -> Self {
        Self {
            schema: RELEASE_RECORD_SCHEMA.to_string(),
            target: value.target().as_str().to_owned(),
            app_version: value.app_version().as_str().to_owned(),
            build_number: value.build_number().as_str().to_owned(),
            source_commit: value.source_commit().as_str().to_owned(),
            artifact_digest: value.artifact_digest().to_string(),
            channel: value.channel().to_string(),
            released_at: value.released_at().to_string(),
            producer: value.producer().as_str().to_owned(),
            claimed_trust: value.claimed_trust().to_string(),
            record_digest: value.record_digest().to_string(),
        }
    }
}
