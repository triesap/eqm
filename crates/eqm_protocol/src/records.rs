//! Runtime-facts and release-record public DTOs.

#![allow(missing_docs)]

use crate::{
    EvidenceSubjectDto, FactValueDto, ProfileValueDto, RELEASE_RECORD_SCHEMA, RUNTIME_FACTS_SCHEMA,
};
use eqm_domain::{
    DimensionId, ProducerRef, ProviderId, RepositoryIdentity, SelectorText, Sha256Digest,
    SourceCommit, SurfaceId, TargetId, TrustLevel, UtcInstant,
};
use eqm_domain::{ReleaseRecord, RuntimeFact, RuntimeFactsSnapshot};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const MAX_RUNTIME_FACTS_BYTES: usize = 16 * 1024 * 1024;
const MAX_RUNTIME_FACTS: usize = 100_000;

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

impl RuntimeFactsDto {
    /// Parses, bounds, and validates one immutable runtime-facts snapshot.
    pub fn from_json(bytes: &[u8]) -> Result<Self, RuntimeFactsDtoError> {
        if bytes.len() > MAX_RUNTIME_FACTS_BYTES {
            return Err(RuntimeFactsDtoError::TooLarge);
        }
        let value: Self = serde_json::from_slice(bytes).map_err(|_| RuntimeFactsDtoError::Json)?;
        if value.schema != RUNTIME_FACTS_SCHEMA.to_string() {
            return Err(RuntimeFactsDtoError::InvalidSchema);
        }
        let claimed: Sha256Digest = value
            .facts_digest
            .parse()
            .map_err(|_| RuntimeFactsDtoError::InvalidDigest)?;
        let mut canonical_value: Value =
            serde_json::from_slice(bytes).map_err(|_| RuntimeFactsDtoError::Json)?;
        canonical_value
            .as_object_mut()
            .ok_or(RuntimeFactsDtoError::Json)?
            .remove("facts_digest");
        let canonical = serde_json_canonicalizer::to_vec(&canonical_value)
            .map_err(|_| RuntimeFactsDtoError::Json)?;
        if Sha256Digest::hash_content(&canonical) != claimed {
            return Err(RuntimeFactsDtoError::DigestMismatch);
        }
        validate_runtime_facts(&value)?;
        Ok(value)
    }
}

fn validate_runtime_facts(value: &RuntimeFactsDto) -> Result<(), RuntimeFactsDtoError> {
    let _: ProviderId = value
        .provider
        .parse()
        .map_err(|_| RuntimeFactsDtoError::InvalidIdentity)?;
    let _: TargetId = value
        .target
        .parse()
        .map_err(|_| RuntimeFactsDtoError::InvalidIdentity)?;
    let _: ProducerRef = value
        .producer
        .parse()
        .map_err(|_| RuntimeFactsDtoError::InvalidIdentity)?;
    let _: TrustLevel = value
        .claimed_trust
        .parse()
        .map_err(|_| RuntimeFactsDtoError::InvalidIdentity)?;
    let observed: UtcInstant = value
        .observed_at
        .parse()
        .map_err(|_| RuntimeFactsDtoError::InvalidTime)?;
    let expires: UtcInstant = value
        .expires_at
        .parse()
        .map_err(|_| RuntimeFactsDtoError::InvalidTime)?;
    if expires <= observed {
        return Err(RuntimeFactsDtoError::InvalidTime);
    }
    let _: RepositoryIdentity = value
        .subject
        .repository
        .parse()
        .map_err(|_| RuntimeFactsDtoError::InvalidSubject)?;
    let _: Sha256Digest = value
        .subject
        .repository_id_digest
        .parse()
        .map_err(|_| RuntimeFactsDtoError::InvalidSubject)?;
    let _: SourceCommit = value
        .subject
        .source_commit
        .parse()
        .map_err(|_| RuntimeFactsDtoError::InvalidSubject)?;
    let _: Sha256Digest = value
        .subject
        .target_configuration_digest
        .parse()
        .map_err(|_| RuntimeFactsDtoError::InvalidSubject)?;
    if value
        .subject
        .build_id
        .as_deref()
        .is_some_and(|item| SelectorText::new(item).is_err())
        || value
            .subject
            .artifact_digest
            .as_deref()
            .is_some_and(|item| item.parse::<Sha256Digest>().is_err())
        || !matches!(
            &value.subject.scope,
            crate::ScopeSubjectDto::Target { target } if target == &value.target
        )
    {
        return Err(RuntimeFactsDtoError::InvalidSubject);
    }
    if !strictly_sorted(&value.profile_values)
        || value.facts.len() > MAX_RUNTIME_FACTS
        || !value.facts.windows(2).all(|pair| {
            (&pair[0].surface, &pair[0].dimension, &pair[0].key)
                < (&pair[1].surface, &pair[1].dimension, &pair[1].key)
        })
    {
        return Err(RuntimeFactsDtoError::InvalidOrdering);
    }
    for fact in &value.facts {
        SurfaceId::new(fact.surface.as_str()).map_err(|_| RuntimeFactsDtoError::InvalidFact)?;
        DimensionId::new(fact.dimension.as_str()).map_err(|_| RuntimeFactsDtoError::InvalidFact)?;
        SelectorText::new(fact.key.as_str()).map_err(|_| RuntimeFactsDtoError::InvalidFact)?;
        match &fact.value {
            FactValueDto::Symbol(item) | FactValueDto::Text(item) => {
                SelectorText::new(item.as_str()).map_err(|_| RuntimeFactsDtoError::InvalidFact)?;
            }
            FactValueDto::Boolean(_) | FactValueDto::Integer(_) => {}
        }
    }
    Ok(())
}

fn strictly_sorted<T: Ord>(values: &[T]) -> bool {
    values.windows(2).all(|pair| pair[0] < pair[1])
}

/// Runtime-facts protocol validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeFactsDtoError {
    /// Input exceeded the 16 MiB normalized-result bound.
    TooLarge,
    /// Input was not one closed JSON object.
    Json,
    /// Schema identity was not current.
    InvalidSchema,
    /// Digest syntax was invalid.
    InvalidDigest,
    /// Canonical content did not match the claimed digest.
    DigestMismatch,
    /// Provider, target, producer, or trust identity was invalid.
    InvalidIdentity,
    /// Subject identity was invalid or did not bind the target.
    InvalidSubject,
    /// Observation or expiry time was invalid.
    InvalidTime,
    /// Profiles or fact coordinates were not unique canonical order.
    InvalidOrdering,
    /// A fact coordinate or value was invalid.
    InvalidFact,
}

impl std::fmt::Display for RuntimeFactsDtoError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for RuntimeFactsDtoError {}

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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::error::Error;

    fn runtime_facts() -> Result<Vec<u8>, Box<dyn Error>> {
        let mut value = json!({
            "schema":RUNTIME_FACTS_SCHEMA.to_string(),
            "provider":"runtime.fixture",
            "subject":{
                "repository":"https://example.com/repo",
                "repository_id_digest":Sha256Digest::hash_content(b"https://example.com/repo").to_string(),
                "scope":{"kind":"target","target":"web"},
                "source_commit":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "build_id":"42",
                "artifact_digest":Sha256Digest::hash_content(b"artifact").to_string(),
                "target_configuration_digest":Sha256Digest::hash_content(b"configuration").to_string()
            },
            "target":"web",
            "profile_values":[],
            "observed_at":"2026-08-08T00:00:00Z",
            "expires_at":"2026-08-09T00:00:00Z",
            "facts":[{"surface":"account.create.signup.identifier","dimension":"availability","key":"enabled","value":{"type":"boolean","value":true}}],
            "producer":"producer://runtime/fixture/snapshot-1",
            "claimed_trust":"signed_ci"
        });
        let digest = Sha256Digest::hash_content(&serde_json_canonicalizer::to_vec(&value)?);
        value["facts_digest"] = Value::String(digest.to_string());
        Ok(serde_json::to_vec(&value)?)
    }

    #[test]
    fn runtime_facts_digest_subject_and_order_are_strict() -> Result<(), Box<dyn Error>> {
        let bytes = runtime_facts()?;
        let parsed = RuntimeFactsDto::from_json(&bytes);
        assert!(parsed.is_ok(), "{parsed:?}");

        let mut tampered: Value = serde_json::from_slice(&bytes)?;
        tampered["target"] = Value::String("ios".to_owned());
        assert_eq!(
            RuntimeFactsDto::from_json(&serde_json::to_vec(&tampered)?),
            Err(RuntimeFactsDtoError::DigestMismatch)
        );

        let mut duplicate: Value = serde_json::from_slice(&bytes)?;
        let fact = duplicate["facts"][0].clone();
        duplicate["facts"].as_array_mut().ok_or("facts")?.push(fact);
        duplicate
            .as_object_mut()
            .ok_or("object")?
            .remove("facts_digest");
        let digest = Sha256Digest::hash_content(&serde_json_canonicalizer::to_vec(&duplicate)?);
        duplicate["facts_digest"] = Value::String(digest.to_string());
        assert_eq!(
            RuntimeFactsDto::from_json(&serde_json::to_vec(&duplicate)?),
            Err(RuntimeFactsDtoError::InvalidOrdering)
        );
        Ok(())
    }
}
