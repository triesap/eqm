//! Explicit `eqm:v2` protocol data transfer objects.
//!
//! These five families (`evidence-result`, `runtime-facts`, `inventory`,
//! `adapter-request`, `adapter-response`) are the first concrete v2 DTOs. They
//! keep the v1 field set but use the v2 representations: full-domain exact
//! integers ([`WireUint`]/[`WireInt`]), canonical v2 UTC instants
//! ([`CanonicalInstant`]) and absence-only optionals that reject explicit
//! `null`. They never parse or reinterpret a v1 record; a v1 payload under a v2
//! identity fails schema dispatch.

#![allow(missing_docs)]

use crate::{WireInt, WireUint};
use eqm_domain::{EvidenceCounts, PositiveCount, Sha256Digest, UtcInstantV2, V2_UTC_PATTERN};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

/// Rejects an explicit `null` for a v2 optional member.
///
/// Used with `#[serde(default, deserialize_with = "deserialize_present")]`, so a
/// missing member is absence while a present `null` fails closed.
pub fn deserialize_present<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    match Option::<T>::deserialize(deserializer)? {
        Some(value) => Ok(Some(value)),
        None => Err(de::Error::custom("explicit null is not a v2 absence form")),
    }
}

/// The shared v2 protocol validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum V2DtoError {
    Json,
    InvalidSchema,
    InvalidDigest,
    IdentityMismatch,
    DigestMismatch,
    InvalidCounts,
    InvalidAttempt,
    InvalidTime,
    InvalidVocabulary,
    InvalidLimits,
    InvalidStatus,
    InvalidSubject,
    InvalidOrdering,
    InvalidFact,
}

impl std::fmt::Display for V2DtoError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for V2DtoError {}

/// A canonical v2 UTC instant on the wire.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CanonicalInstant(UtcInstantV2);

impl CanonicalInstant {
    #[must_use]
    pub const fn new(value: UtcInstantV2) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn get(self) -> UtcInstantV2 {
        self.0
    }
}

impl Serialize for CanonicalInstant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for CanonicalInstant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;
        text.parse::<UtcInstantV2>()
            .map(Self)
            .map_err(de::Error::custom)
    }
}

impl JsonSchema for CanonicalInstant {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "CanonicalInstant".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "format": "date-time",
            "pattern": V2_UTC_PATTERN,
            "type": "string",
        })
    }
}

// ---------------------------------------------------------------------------
// evidence-result
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CountsV2Dto {
    pub selected: WireUint,
    pub passed: WireUint,
    pub failed: WireUint,
    pub skipped: WireUint,
    pub filtered: WireUint,
    pub quarantined: WireUint,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AttemptV2Dto {
    pub number: WireUint,
    pub outcome: String,
    pub started_at: CanonicalInstant,
    pub finished_at: CanonicalInstant,
    #[serde(
        default,
        deserialize_with = "deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub message: Option<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AttachmentV2Dto {
    pub name: String,
    pub media_type: String,
    pub digest: String,
    pub size: WireUint,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum EvidenceSelectorV2Dto {
    Symbol {
        name: String,
        #[serde(
            default,
            deserialize_with = "deserialize_present",
            skip_serializing_if = "Option::is_none"
        )]
        #[schemars(with = "String")]
        language: Option<String>,
    },
    Route {
        path: String,
        #[serde(
            default,
            deserialize_with = "deserialize_present",
            skip_serializing_if = "Option::is_none"
        )]
        #[schemars(with = "String")]
        method: Option<String>,
    },
    Test {
        framework: String,
        test_id: String,
        #[serde(
            default,
            deserialize_with = "deserialize_present",
            skip_serializing_if = "Option::is_none"
        )]
        #[schemars(with = "String")]
        suite: Option<String>,
    },
    Inventory {
        record_type: String,
        key: String,
        #[serde(
            default,
            deserialize_with = "deserialize_present",
            skip_serializing_if = "Option::is_none"
        )]
        #[schemars(with = "String")]
        value: Option<String>,
    },
    Snapshot {
        snapshot_id: String,
        #[serde(
            default,
            deserialize_with = "deserialize_present",
            skip_serializing_if = "Option::is_none"
        )]
        #[schemars(with = "String")]
        variant: Option<String>,
    },
    Release {
        channel: String,
    },
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ScopeSubjectV2Dto {
    Target { target: String },
    Provider { provider: String },
    TargetSet { targets: BTreeSet<String> },
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceSubjectV2Dto {
    pub repository: String,
    pub repository_id_digest: String,
    pub scope: ScopeSubjectV2Dto,
    pub source_commit: String,
    #[serde(
        default,
        deserialize_with = "deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub build_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub artifact_digest: Option<String>,
    pub target_configuration_digest: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionPayloadV2Dto {
    pub attempts: Vec<AttemptV2Dto>,
    pub counts: CountsV2Dto,
    pub started_at: CanonicalInstant,
    pub finished_at: CanonicalInstant,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum EvidencePayloadV2Dto {
    StructuralCheck {
        execution: ExecutionPayloadV2Dto,
    },
    StaticInventory {
        inventory_digest: String,
        counts: CountsV2Dto,
    },
    Test {
        execution: ExecutionPayloadV2Dto,
    },
    Snapshot {
        execution: ExecutionPayloadV2Dto,
    },
    ManualReview {
        outcome: String,
        reviewer: String,
        #[serde(
            default,
            deserialize_with = "deserialize_present",
            skip_serializing_if = "Option::is_none"
        )]
        #[schemars(with = "String")]
        message: Option<String>,
    },
    RuntimeSnapshot {
        runtime_facts_digest: String,
        counts: CountsV2Dto,
    },
    ReleaseRecord {
        release_record_digest: String,
    },
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileValueV2Dto {
    pub profile: String,
    pub revision: WireUint,
    pub dimension: String,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceResultV2Dto {
    pub schema: String,
    pub id: String,
    pub subject: EvidenceSubjectV2Dto,
    pub target: String,
    pub unit: String,
    pub requirements: BTreeSet<String>,
    pub facets: BTreeSet<String>,
    pub kind: String,
    pub evidence_spec_digest: String,
    pub contract_digest: String,
    pub binding_digest: String,
    pub policy_digest: String,
    #[serde(
        default,
        deserialize_with = "deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub runner_digest: Option<String>,
    #[serde(
        default,
        deserialize_with = "deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub adapter_digest: Option<String>,
    #[serde(
        default,
        deserialize_with = "deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub runtime_facts_digest: Option<String>,
    #[serde(
        default,
        deserialize_with = "deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub release_record_digest: Option<String>,
    pub profile_values: Vec<ProfileValueV2Dto>,
    pub producer: String,
    pub claimed_trust: String,
    pub observed_at: CanonicalInstant,
    pub payload: EvidencePayloadV2Dto,
    pub attachments: BTreeSet<AttachmentV2Dto>,
    pub result_digest: String,
}

impl CountsV2Dto {
    fn validate(&self) -> Result<(), V2DtoError> {
        EvidenceCounts::new(
            self.selected.get(),
            self.passed.get(),
            self.failed.get(),
            self.skipped.get(),
            self.filtered.get(),
            self.quarantined.get(),
        )
        .map(|_| ())
        .map_err(|_| V2DtoError::InvalidCounts)
    }
}

fn validate_execution(execution: &ExecutionPayloadV2Dto) -> Result<(), V2DtoError> {
    execution.counts.validate()?;
    if execution.finished_at < execution.started_at {
        return Err(V2DtoError::InvalidTime);
    }
    for attempt in &execution.attempts {
        PositiveCount::new(attempt.number.get()).map_err(|_| V2DtoError::InvalidAttempt)?;
        attempt
            .outcome
            .parse::<eqm_domain::AttemptOutcome>()
            .map_err(|_| V2DtoError::InvalidVocabulary)?;
        if attempt.finished_at < attempt.started_at {
            return Err(V2DtoError::InvalidTime);
        }
    }
    Ok(())
}

fn validate_digest(value: &str) -> Result<(), V2DtoError> {
    value
        .parse::<Sha256Digest>()
        .map(|_| ())
        .map_err(|_| V2DtoError::InvalidDigest)
}

fn validate_payload(payload: &EvidencePayloadV2Dto) -> Result<(), V2DtoError> {
    match payload {
        EvidencePayloadV2Dto::StructuralCheck { execution }
        | EvidencePayloadV2Dto::Test { execution }
        | EvidencePayloadV2Dto::Snapshot { execution } => validate_execution(execution),
        EvidencePayloadV2Dto::StaticInventory {
            inventory_digest,
            counts,
        }
        | EvidencePayloadV2Dto::RuntimeSnapshot {
            runtime_facts_digest: inventory_digest,
            counts,
        } => {
            validate_digest(inventory_digest)?;
            counts.validate()
        }
        EvidencePayloadV2Dto::ReleaseRecord {
            release_record_digest,
        } => validate_digest(release_record_digest),
        EvidencePayloadV2Dto::ManualReview { outcome, .. } => match outcome
            .parse::<eqm_domain::AttemptOutcome>(
        ) {
            Ok(eqm_domain::AttemptOutcome::Passed | eqm_domain::AttemptOutcome::Failed) => Ok(()),
            _ => Err(V2DtoError::InvalidVocabulary),
        },
    }
}

impl EvidenceResultV2Dto {
    pub fn from_json(bytes: &[u8]) -> Result<Self, V2DtoError> {
        let value: Self = crate::strict::from_slice(bytes).map_err(|_| V2DtoError::Json)?;
        value.validate()?;
        Ok(value)
    }

    fn validate(&self) -> Result<(), V2DtoError> {
        if self.schema != crate::EVIDENCE_RESULT_SCHEMA_V2.to_string() {
            return Err(V2DtoError::InvalidSchema);
        }
        let id = self
            .id
            .parse::<Sha256Digest>()
            .map_err(|_| V2DtoError::InvalidDigest)?;
        let result = self
            .result_digest
            .parse::<Sha256Digest>()
            .map_err(|_| V2DtoError::InvalidDigest)?;
        if id != result {
            return Err(V2DtoError::IdentityMismatch);
        }
        for digest in [
            &self.evidence_spec_digest,
            &self.contract_digest,
            &self.binding_digest,
            &self.policy_digest,
        ] {
            validate_digest(digest)?;
        }
        for digest in [
            &self.runner_digest,
            &self.adapter_digest,
            &self.runtime_facts_digest,
            &self.release_record_digest,
        ]
        .into_iter()
        .flatten()
        {
            validate_digest(digest)?;
        }
        self.kind
            .parse::<eqm_domain::EvidenceKind>()
            .map_err(|_| V2DtoError::InvalidVocabulary)?;
        self.claimed_trust
            .parse::<eqm_domain::TrustLevel>()
            .map_err(|_| V2DtoError::InvalidVocabulary)?;
        self.producer
            .parse::<eqm_domain::ProducerRef>()
            .map_err(|_| V2DtoError::InvalidVocabulary)?;
        for attachment in &self.attachments {
            validate_digest(&attachment.digest)?;
        }
        validate_payload(&self.payload)
    }
}

// ---------------------------------------------------------------------------
// runtime-facts
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(
    tag = "type",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum FactValueV2Dto {
    Boolean(bool),
    Integer(WireInt),
    Symbol(String),
    Text(String),
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeFactV2Dto {
    pub surface: String,
    pub dimension: String,
    pub key: String,
    pub value: FactValueV2Dto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeFactsV2Dto {
    pub schema: String,
    pub provider: String,
    pub subject: EvidenceSubjectV2Dto,
    pub target: String,
    pub profile_values: Vec<ProfileValueV2Dto>,
    pub observed_at: CanonicalInstant,
    pub expires_at: CanonicalInstant,
    pub facts: Vec<RuntimeFactV2Dto>,
    pub producer: String,
    pub claimed_trust: String,
    pub facts_digest: String,
}

impl RuntimeFactsV2Dto {
    pub fn from_json(bytes: &[u8]) -> Result<Self, V2DtoError> {
        if bytes.len() > 16 * 1024 * 1024 {
            return Err(V2DtoError::InvalidLimits);
        }
        let value: Self = crate::strict::from_slice_limited(bytes, 16 * 1024 * 1024)
            .map_err(|_| V2DtoError::Json)?;
        if value.schema != crate::RUNTIME_FACTS_SCHEMA_V2.to_string() {
            return Err(V2DtoError::InvalidSchema);
        }
        let claimed = value
            .facts_digest
            .parse::<Sha256Digest>()
            .map_err(|_| V2DtoError::InvalidDigest)?;
        let mut canonical: Value =
            crate::strict::parse_strict(bytes).map_err(|_| V2DtoError::Json)?;
        canonical
            .as_object_mut()
            .ok_or(V2DtoError::Json)?
            .remove("facts_digest");
        let canonical =
            serde_json_canonicalizer::to_vec(&canonical).map_err(|_| V2DtoError::InvalidDigest)?;
        if Sha256Digest::hash_content(&canonical) != claimed {
            return Err(V2DtoError::DigestMismatch);
        }
        value.validate()?;
        Ok(value)
    }

    fn validate(&self) -> Result<(), V2DtoError> {
        self.provider
            .parse::<eqm_domain::ProviderId>()
            .map_err(|_| V2DtoError::InvalidSubject)?;
        self.target
            .parse::<eqm_domain::TargetId>()
            .map_err(|_| V2DtoError::InvalidSubject)?;
        self.producer
            .parse::<eqm_domain::ProducerRef>()
            .map_err(|_| V2DtoError::InvalidVocabulary)?;
        self.claimed_trust
            .parse::<eqm_domain::TrustLevel>()
            .map_err(|_| V2DtoError::InvalidVocabulary)?;
        if self.expires_at <= self.observed_at {
            return Err(V2DtoError::InvalidTime);
        }
        if !matches!(
            &self.subject.scope,
            ScopeSubjectV2Dto::Target { target } if target == &self.target
        ) {
            return Err(V2DtoError::InvalidSubject);
        }
        if !self.profile_values.windows(2).all(|pair| pair[0] < pair[1]) {
            return Err(V2DtoError::InvalidOrdering);
        }
        if !self.facts.windows(2).all(|pair| {
            (&pair[0].surface, &pair[0].dimension, &pair[0].key)
                < (&pair[1].surface, &pair[1].dimension, &pair[1].key)
        }) {
            return Err(V2DtoError::InvalidOrdering);
        }
        for fact in &self.facts {
            eqm_domain::SurfaceId::new(fact.surface.as_str())
                .map_err(|_| V2DtoError::InvalidFact)?;
            eqm_domain::DimensionId::new(fact.dimension.as_str())
                .map_err(|_| V2DtoError::InvalidFact)?;
            eqm_domain::SelectorText::new(fact.key.as_str())
                .map_err(|_| V2DtoError::InvalidFact)?;
            match &fact.value {
                FactValueV2Dto::Symbol(value) | FactValueV2Dto::Text(value) => {
                    eqm_domain::SelectorText::new(value.as_str())
                        .map_err(|_| V2DtoError::InvalidFact)?;
                }
                FactValueV2Dto::Boolean(_) | FactValueV2Dto::Integer(_) => {}
            }
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// inventory
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InventoryEntryV2Dto {
    pub kind: String,
    pub key: String,
    pub attributes: BTreeMap<String, FactValueV2Dto>,
    pub source: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiagnosticV2Dto {
    pub code: String,
    pub severity: crate::SeverityDto,
    pub message: String,
    #[serde(
        default,
        deserialize_with = "deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "SourceLocationV2Dto")]
    pub source: Option<SourceLocationV2Dto>,
    pub related: Vec<SourceLocationV2Dto>,
    #[serde(
        default,
        deserialize_with = "deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub unit: Option<String>,
    #[serde(
        default,
        deserialize_with = "deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub target: Option<String>,
}

#[derive(
    Clone, Copy, Debug, Deserialize, JsonSchema, Eq, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(deny_unknown_fields)]
pub struct SourcePositionV2Dto {
    pub line: u32,
    pub column: u32,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SourceLocationV2Dto {
    pub uri: String,
    pub start: SourcePositionV2Dto,
    pub end: SourcePositionV2Dto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InventoryV2Dto {
    pub schema: String,
    pub adapter: String,
    pub adapter_digest: String,
    pub subject: EvidenceSubjectV2Dto,
    pub target: String,
    pub generated_at: CanonicalInstant,
    pub completeness: String,
    pub entries: Vec<InventoryEntryV2Dto>,
    pub diagnostics: Vec<DiagnosticV2Dto>,
    pub inventory_digest: String,
}

impl InventoryV2Dto {
    pub fn from_json(bytes: &[u8]) -> Result<Self, V2DtoError> {
        if bytes.len() > 16 * 1024 * 1024 {
            return Err(V2DtoError::InvalidLimits);
        }
        let value: Self = crate::strict::from_slice_limited(bytes, 16 * 1024 * 1024)
            .map_err(|_| V2DtoError::Json)?;
        if value.schema != crate::INVENTORY_SCHEMA_V2.to_string() {
            return Err(V2DtoError::InvalidSchema);
        }
        if value.entries.len() > 250_000 {
            return Err(V2DtoError::InvalidLimits);
        }
        validate_digest(&value.adapter_digest)?;
        validate_digest(&value.inventory_digest)?;
        Ok(value)
    }
}

// ---------------------------------------------------------------------------
// adapter-request / adapter-response
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterOperationV2Dto {
    Discover,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterLimitsV2Dto {
    pub timeout_ms: WireUint,
    pub max_input_bytes: WireUint,
    pub max_output_bytes: WireUint,
    pub max_entries: WireUint,
    pub max_depth: WireUint,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterRequestV2Dto {
    pub schema: String,
    pub request_id: String,
    pub adapter: String,
    pub adapter_digest: String,
    pub operation: AdapterOperationV2Dto,
    pub subject: EvidenceSubjectV2Dto,
    pub target: String,
    pub target_root: String,
    pub limits: AdapterLimitsV2Dto,
}

impl AdapterRequestV2Dto {
    pub fn from_json(bytes: &[u8]) -> Result<Self, V2DtoError> {
        if bytes.len() > 4 * 1024 * 1024 {
            return Err(V2DtoError::InvalidLimits);
        }
        let value: Self = crate::strict::from_slice_limited(bytes, 4 * 1024 * 1024)
            .map_err(|_| V2DtoError::Json)?;
        if value.schema != crate::ADAPTER_REQUEST_SCHEMA_V2.to_string() {
            return Err(V2DtoError::InvalidSchema);
        }
        validate_digest(&value.adapter_digest)?;
        let limits = &value.limits;
        if !(1_000..=3_600_000).contains(&limits.timeout_ms.get())
            || limits.max_input_bytes.get() == 0
            || limits.max_input_bytes.get() > 4 * 1024 * 1024
            || limits.max_output_bytes.get() == 0
            || limits.max_output_bytes.get() > 16 * 1024 * 1024
            || limits.max_entries.get() == 0
            || limits.max_entries.get() > 250_000
            || limits.max_depth.get() == 0
            || limits.max_depth.get() > 64
        {
            return Err(V2DtoError::InvalidLimits);
        }
        Ok(value)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterStatusV2Dto {
    Ok,
    Partial,
    Error,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterResponseV2Dto {
    pub schema: String,
    pub request_id: String,
    pub adapter: String,
    pub adapter_digest: String,
    pub status: AdapterStatusV2Dto,
    #[serde(
        default,
        deserialize_with = "deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "InventoryV2Dto")]
    pub inventory: Option<InventoryV2Dto>,
    pub diagnostics: Vec<DiagnosticV2Dto>,
}

impl AdapterResponseV2Dto {
    pub fn from_json(bytes: &[u8]) -> Result<Self, V2DtoError> {
        if bytes.len() > 16 * 1024 * 1024 {
            return Err(V2DtoError::InvalidLimits);
        }
        let value: Self = crate::strict::from_slice_limited(bytes, 16 * 1024 * 1024)
            .map_err(|_| V2DtoError::Json)?;
        if value.schema != crate::ADAPTER_RESPONSE_SCHEMA_V2.to_string() {
            return Err(V2DtoError::InvalidSchema);
        }
        if matches!(value.status, AdapterStatusV2Dto::Error) == value.inventory.is_some() {
            return Err(V2DtoError::InvalidStatus);
        }
        if value
            .inventory
            .as_ref()
            .is_some_and(|inventory| inventory.entries.len() > 250_000)
        {
            return Err(V2DtoError::InvalidLimits);
        }
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex_lite::Regex;
    use std::error::Error;
    use std::path::Path;

    fn digest(seed: &str) -> String {
        Sha256Digest::hash_content(seed.as_bytes()).to_string()
    }

    fn instant(value: &str) -> Result<CanonicalInstant, Box<dyn Error>> {
        Ok(CanonicalInstant::new(value.parse::<UtcInstantV2>()?))
    }

    fn subject() -> EvidenceSubjectV2Dto {
        EvidenceSubjectV2Dto {
            repository: "https://example.com/team/project".to_owned(),
            repository_id_digest: digest("repo"),
            scope: ScopeSubjectV2Dto::Target {
                target: "web".to_owned(),
            },
            source_commit: "a".repeat(40),
            build_id: None,
            artifact_digest: None,
            target_configuration_digest: digest("config"),
        }
    }

    fn counts(total: u64) -> CountsV2Dto {
        CountsV2Dto {
            selected: WireUint::new(total),
            passed: WireUint::new(total),
            failed: WireUint::new(0),
            skipped: WireUint::new(0),
            filtered: WireUint::new(0),
            quarantined: WireUint::new(0),
        }
    }

    fn execution() -> Result<ExecutionPayloadV2Dto, Box<dyn Error>> {
        Ok(ExecutionPayloadV2Dto {
            attempts: vec![AttemptV2Dto {
                number: WireUint::new(1),
                outcome: "passed".to_owned(),
                started_at: instant("2026-01-02T03:04:05Z")?,
                finished_at: instant("2026-01-02T03:04:06Z")?,
                message: None,
            }],
            counts: counts(1),
            started_at: instant("2026-01-02T03:04:05Z")?,
            finished_at: instant("2026-01-02T03:04:06Z")?,
        })
    }

    fn evidence_result() -> Result<EvidenceResultV2Dto, Box<dyn Error>> {
        let id = digest("identity");
        Ok(EvidenceResultV2Dto {
            schema: crate::EVIDENCE_RESULT_SCHEMA_V2.to_string(),
            id: id.clone(),
            subject: subject(),
            target: "web".to_owned(),
            unit: "unit.test".to_owned(),
            requirements: BTreeSet::from(["R18".to_owned()]),
            facets: BTreeSet::from(["facet.a".to_owned()]),
            kind: "test".to_owned(),
            evidence_spec_digest: digest("spec"),
            contract_digest: digest("contract"),
            binding_digest: digest("binding"),
            policy_digest: digest("policy"),
            runner_digest: None,
            adapter_digest: None,
            runtime_facts_digest: None,
            release_record_digest: None,
            profile_values: Vec::new(),
            producer: "producer://ci/runner/1".to_owned(),
            claimed_trust: "signed_ci".to_owned(),
            observed_at: instant("2026-01-02T03:04:05Z")?,
            payload: EvidencePayloadV2Dto::Test {
                execution: execution()?,
            },
            attachments: BTreeSet::new(),
            result_digest: id,
        })
    }

    fn inventory() -> Result<InventoryV2Dto, Box<dyn Error>> {
        Ok(InventoryV2Dto {
            schema: crate::INVENTORY_SCHEMA_V2.to_string(),
            adapter: "adapter.test".to_owned(),
            adapter_digest: digest("adapter"),
            subject: subject(),
            target: "web".to_owned(),
            generated_at: instant("2026-01-02T03:04:05Z")?,
            completeness: "complete".to_owned(),
            entries: Vec::new(),
            diagnostics: Vec::new(),
            inventory_digest: digest("inventory"),
        })
    }

    fn adapter_request() -> AdapterRequestV2Dto {
        AdapterRequestV2Dto {
            schema: crate::ADAPTER_REQUEST_SCHEMA_V2.to_string(),
            request_id: "request-1".to_owned(),
            adapter: "adapter.test".to_owned(),
            adapter_digest: digest("adapter"),
            operation: AdapterOperationV2Dto::Discover,
            subject: subject(),
            target: "web".to_owned(),
            target_root: "/tmp/target".to_owned(),
            limits: AdapterLimitsV2Dto {
                timeout_ms: WireUint::new(1_000),
                max_input_bytes: WireUint::new(4 * 1024 * 1024),
                max_output_bytes: WireUint::new(16 * 1024 * 1024),
                max_entries: WireUint::new(10),
                max_depth: WireUint::new(8),
            },
        }
    }

    fn runtime_facts() -> Result<RuntimeFactsV2Dto, Box<dyn Error>> {
        let mut facts = RuntimeFactsV2Dto {
            schema: crate::RUNTIME_FACTS_SCHEMA_V2.to_string(),
            provider: "runtime.fixture".to_owned(),
            subject: subject(),
            target: "web".to_owned(),
            profile_values: Vec::new(),
            observed_at: instant("2026-01-02T03:04:05Z")?,
            expires_at: instant("2026-01-03T03:04:05Z")?,
            facts: vec![RuntimeFactV2Dto {
                surface: "account.create.signup.identifier".to_owned(),
                dimension: "availability".to_owned(),
                key: "version".to_owned(),
                value: FactValueV2Dto::Integer(WireInt::new(i64::MAX)),
            }],
            producer: "producer://runtime/fixture/snapshot-1".to_owned(),
            claimed_trust: "signed_ci".to_owned(),
            facts_digest: String::new(),
        };
        let mut value = serde_json::to_value(&facts)?;
        value
            .as_object_mut()
            .ok_or("runtime facts object")?
            .remove("facts_digest");
        facts.facts_digest =
            Sha256Digest::hash_content(&serde_json_canonicalizer::to_vec(&value)?).to_string();
        Ok(facts)
    }

    #[test]
    fn v2_records_round_trip_and_reject_representation_mutations() -> Result<(), Box<dyn Error>> {
        let bytes = serde_json::to_vec(&adapter_request())?;
        assert!(AdapterRequestV2Dto::from_json(&bytes).is_ok());
        let mut value: Value = serde_json::from_slice(&bytes)?;
        value["limits"]["max_entries"] = serde_json::json!("18446744073709551616");
        assert!(serde_json::from_value::<AdapterRequestV2Dto>(value).is_err());
        let mut value: Value = serde_json::from_slice(&bytes)?;
        value["limits"]["max_entries"] = serde_json::json!("10");
        assert!(serde_json::from_value::<AdapterRequestV2Dto>(value).is_err());

        let bytes = serde_json::to_vec(&evidence_result()?)?;
        assert!(EvidenceResultV2Dto::from_json(&bytes).is_ok());
        let mut value: Value = serde_json::from_slice(&bytes)?;
        value["observed_at"] = serde_json::json!("2016-12-31T23:59:60Z");
        assert!(serde_json::from_value::<EvidenceResultV2Dto>(value).is_err());
        let mut value: Value = serde_json::from_slice(&bytes)?;
        value["subject"]["build_id"] = Value::Null;
        assert!(serde_json::from_value::<EvidenceResultV2Dto>(value).is_err());
        let mut value: Value = serde_json::from_slice(&bytes)?;
        value["payload"]["execution"]["counts"]["selected"] = serde_json::json!("1");
        assert!(serde_json::from_value::<EvidenceResultV2Dto>(value).is_err());
        let mut value: Value = serde_json::from_slice(&bytes)?;
        value["payload"]["execution"]["counts"]["selected"] =
            serde_json::json!("18446744073709551615");
        value["payload"]["execution"]["counts"]["passed"] =
            serde_json::json!("18446744073709551615");
        assert!(serde_json::from_value::<EvidenceResultV2Dto>(value).is_ok());

        let bytes = serde_json::to_vec(&runtime_facts()?)?;
        assert!(RuntimeFactsV2Dto::from_json(&bytes).is_ok());
        let mut value: Value = serde_json::from_slice(&bytes)?;
        value["facts"][0]["value"]["value"] = serde_json::json!("-9223372036854775809");
        assert!(serde_json::from_value::<RuntimeFactsV2Dto>(value).is_err());

        let bytes = serde_json::to_vec(&inventory()?)?;
        assert!(InventoryV2Dto::from_json(&bytes).is_ok());
        let mut value: Value = serde_json::from_slice(&bytes)?;
        value["generated_at"] = serde_json::json!("2026-01-02T03:04:05.1Z");
        assert!(serde_json::from_value::<InventoryV2Dto>(value).is_err());

        let response = AdapterResponseV2Dto {
            schema: crate::ADAPTER_RESPONSE_SCHEMA_V2.to_string(),
            request_id: "request-1".to_owned(),
            adapter: "adapter.test".to_owned(),
            adapter_digest: digest("adapter"),
            status: AdapterStatusV2Dto::Ok,
            inventory: Some(inventory()?),
            diagnostics: Vec::new(),
        };
        let bytes = serde_json::to_vec(&response)?;
        assert!(AdapterResponseV2Dto::from_json(&bytes).is_ok());
        let mut value: Value = serde_json::from_slice(&bytes)?;
        value["inventory"] = Value::Null;
        assert!(serde_json::from_value::<AdapterResponseV2Dto>(value).is_err());
        Ok(())
    }

    #[test]
    fn v2_from_json_enforces_schema_identity_and_absence() -> Result<(), Box<dyn Error>> {
        let mut value = serde_json::to_value(adapter_request())?;
        value["schema"] = serde_json::json!(crate::ADAPTER_REQUEST_SCHEMA.to_string());
        let bytes = serde_json::to_vec(&value)?;
        assert_eq!(
            AdapterRequestV2Dto::from_json(&bytes),
            Err(V2DtoError::InvalidSchema)
        );

        let text = String::from_utf8(serde_json::to_vec(&evidence_result()?)?)?;
        assert!(!text.contains("build_id"));
        assert!(!text.contains("runner_digest"));
        assert!(EvidenceResultV2Dto::from_json(text.as_bytes()).is_ok());
        Ok(())
    }

    #[test]
    fn v2_family_identities_are_exact_and_reject_other_versions() -> Result<(), Box<dyn Error>> {
        let identities = [
            (
                crate::EVIDENCE_RESULT_SCHEMA_V2,
                eqm_domain::SchemaKind::EvidenceResult,
            ),
            (
                crate::RUNTIME_FACTS_SCHEMA_V2,
                eqm_domain::SchemaKind::RuntimeFacts,
            ),
            (
                crate::INVENTORY_SCHEMA_V2,
                eqm_domain::SchemaKind::Inventory,
            ),
            (
                crate::ADAPTER_REQUEST_SCHEMA_V2,
                eqm_domain::SchemaKind::AdapterRequest,
            ),
            (
                crate::ADAPTER_RESPONSE_SCHEMA_V2,
                eqm_domain::SchemaKind::AdapterResponse,
            ),
        ];
        assert_eq!(identities.len(), crate::V2_PROTOCOL_SCHEMAS.len());
        for (uri, kind) in identities {
            assert_eq!(uri.kind(), kind);
            assert_eq!(uri.version(), eqm_domain::SchemaVersion::V2);
            assert!(uri.to_string().contains("/schemas/v2/"));
            assert_eq!(
                uri.to_string().parse::<eqm_domain::SchemaUri>().ok(),
                Some(uri)
            );
        }
        assert!(
            "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v3/protocol/evidence-result.schema.json"
                .parse::<eqm_domain::SchemaUri>()
                .is_err()
        );
        Ok(())
    }

    fn schema_document(name: &str) -> Result<Value, Box<dyn Error>> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../schemas/v2/protocol")
            .join(format!("{name}.schema.json"));
        Ok(serde_json::from_slice(&std::fs::read(path)?)?)
    }

    fn resolve<'a>(root: &'a Value, schema: &'a Value) -> &'a Value {
        let Some(reference) = schema.get("$ref").and_then(Value::as_str) else {
            return schema;
        };
        let Some(name) = reference.strip_prefix("#/$defs/") else {
            return schema;
        };
        root.get("$defs")
            .and_then(|defs| defs.get(name))
            .unwrap_or(schema)
    }

    #[test]
    fn v2_generated_schemas_agree_with_representation_rules() -> Result<(), Box<dyn Error>> {
        let schema = schema_document("adapter-request")?;
        let limits = resolve(&schema, &schema["properties"]["limits"]);
        let pattern = limits["properties"]["timeout_ms"]["anyOf"][1]["pattern"]
            .as_str()
            .ok_or("unsigned pattern")?;
        let regex = Regex::new(pattern)?;
        assert!(regex.is_match("18446744073709551615"));
        assert!(!regex.is_match("18446744073709551616"));
        assert!(!regex.is_match("0"));
        assert!(!regex.is_match("5"));

        let schema = schema_document("evidence-result")?;
        let pattern = schema["$defs"]["CanonicalInstant"]["pattern"]
            .as_str()
            .ok_or("time pattern")?;
        let regex = Regex::new(pattern)?;
        assert!(regex.is_match("2026-01-02T03:04:05.000000001Z"));
        assert!(!regex.is_match("2016-12-31T23:59:60Z"));
        assert!(!regex.is_match("0000-01-01T00:00:00Z"));
        assert!(!regex.is_match("2026-01-02T03:04:05.1Z"));
        assert!(!regex.is_match("2026-01-02T03:04:05+00:00"));
        let required: Vec<&str> = schema["required"]
            .as_array()
            .ok_or("required")?
            .iter()
            .filter_map(Value::as_str)
            .collect();
        assert!(!required.contains(&"runner_digest"));
        assert_eq!(
            schema["properties"]["runner_digest"]["type"].as_str(),
            Some("string")
        );
        Ok(())
    }
}
