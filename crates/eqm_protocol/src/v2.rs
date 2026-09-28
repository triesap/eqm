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

use crate::{CanonicalSet, WireInt, WireUint};
use eqm_domain::{
    AdapterId, AttemptOutcome, DimensionId, EvidenceCounts, EvidenceKind, Facet, FullRequirementId,
    HttpMethod, InventoryCompleteness, OwnerRef, ProducerRef, ProfileId, ProviderId,
    ReleaseChannel, RepoPath, RepositoryIdentity, Revision, SelectorText, Sha256Digest,
    SourceCommit, SurfaceId, SymbolicValueId, TargetId, TrustLevel, UnitId, UtcInstantV2,
    V2_UTC_PATTERN,
};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

/// Declared v2 runtime-facts byte envelope, honored at every parse.
const MAX_RUNTIME_FACTS_BYTES: usize = 16 * 1024 * 1024;
/// Declared v2 runtime-facts count ceiling, carried over from v1.
const MAX_RUNTIME_FACTS: usize = 100_000;

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
    InvalidProfile,
    InvalidEntry,
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
// exact schema markers
// ---------------------------------------------------------------------------

macro_rules! v2_schema_marker {
    ($function:ident, $uri:path) => {
        fn $function(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
            schemars::json_schema!({
                "const": $uri.to_string(),
                "type": "string",
            })
        }
    };
}

v2_schema_marker!(
    evidence_result_schema_marker,
    crate::EVIDENCE_RESULT_SCHEMA_V2
);
v2_schema_marker!(runtime_facts_schema_marker, crate::RUNTIME_FACTS_SCHEMA_V2);
v2_schema_marker!(inventory_schema_marker, crate::INVENTORY_SCHEMA_V2);
v2_schema_marker!(
    adapter_request_schema_marker,
    crate::ADAPTER_REQUEST_SCHEMA_V2
);
v2_schema_marker!(
    adapter_response_schema_marker,
    crate::ADAPTER_RESPONSE_SCHEMA_V2
);

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

impl EvidenceSelectorV2Dto {
    /// Validates selector metadata with the existing domain field types:
    /// [`SelectorText`] for authored names and [`HttpMethod`]/[`ReleaseChannel`]
    /// for their closed vocabularies.
    ///
    /// No current v2 root owns an evidence selector, so this typed contract is
    /// not yet integrated at a real ingress. Its consumer wiring belongs to the
    /// first v2 step that introduces an owning selector root; this helper is not
    /// proof of root integration today.
    pub fn validate(&self) -> Result<(), V2DtoError> {
        match self {
            Self::Symbol { name, language } => {
                validate_selector_text(name, V2DtoError::InvalidVocabulary)?;
                if let Some(language) = language {
                    validate_selector_text(language, V2DtoError::InvalidVocabulary)?;
                }
            }
            Self::Route { path, method } => {
                validate_selector_text(path, V2DtoError::InvalidVocabulary)?;
                if let Some(method) = method {
                    parse_typed::<HttpMethod>(method, V2DtoError::InvalidVocabulary)?;
                }
            }
            Self::Test {
                framework,
                test_id,
                suite,
            } => {
                validate_selector_text(framework, V2DtoError::InvalidVocabulary)?;
                validate_selector_text(test_id, V2DtoError::InvalidVocabulary)?;
                if let Some(suite) = suite {
                    validate_selector_text(suite, V2DtoError::InvalidVocabulary)?;
                }
            }
            Self::Inventory {
                record_type,
                key,
                value,
            } => {
                validate_selector_text(record_type, V2DtoError::InvalidVocabulary)?;
                validate_selector_text(key, V2DtoError::InvalidVocabulary)?;
                if let Some(value) = value {
                    validate_selector_text(value, V2DtoError::InvalidVocabulary)?;
                }
            }
            Self::Snapshot {
                snapshot_id,
                variant,
            } => {
                validate_selector_text(snapshot_id, V2DtoError::InvalidVocabulary)?;
                if let Some(variant) = variant {
                    validate_selector_text(variant, V2DtoError::InvalidVocabulary)?;
                }
            }
            Self::Release { channel } => {
                parse_typed::<ReleaseChannel>(channel, V2DtoError::InvalidVocabulary)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ScopeSubjectV2Dto {
    Target { target: String },
    Provider { provider: String },
    TargetSet { targets: CanonicalSet<String> },
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

impl EvidenceSubjectV2Dto {
    /// Validates the shared subject metadata and typed scope members with the
    /// existing domain grammars. This is structural authored validation only;
    /// it confers no trust or relationship acceptance.
    fn validate(&self) -> Result<(), V2DtoError> {
        parse_typed::<RepositoryIdentity>(&self.repository, V2DtoError::InvalidSubject)?;
        parse_typed::<Sha256Digest>(&self.repository_id_digest, V2DtoError::InvalidSubject)?;
        parse_typed::<SourceCommit>(&self.source_commit, V2DtoError::InvalidSubject)?;
        parse_typed::<Sha256Digest>(
            &self.target_configuration_digest,
            V2DtoError::InvalidSubject,
        )?;
        if let Some(build_id) = &self.build_id {
            validate_selector_text(build_id, V2DtoError::InvalidSubject)?;
        }
        if let Some(artifact_digest) = &self.artifact_digest {
            parse_typed::<Sha256Digest>(artifact_digest, V2DtoError::InvalidSubject)?;
        }
        match &self.scope {
            ScopeSubjectV2Dto::Target { target } => {
                parse_typed::<TargetId>(target, V2DtoError::InvalidSubject)?;
            }
            ScopeSubjectV2Dto::Provider { provider } => {
                parse_typed::<ProviderId>(provider, V2DtoError::InvalidSubject)?;
            }
            ScopeSubjectV2Dto::TargetSet { targets } => {
                if targets.is_empty() {
                    return Err(V2DtoError::InvalidSubject);
                }
                for target in targets.iter() {
                    parse_typed::<TargetId>(target, V2DtoError::InvalidSubject)?;
                }
            }
        }
        Ok(())
    }
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
    #[schemars(schema_with = "evidence_result_schema_marker")]
    pub schema: String,
    pub id: String,
    pub subject: EvidenceSubjectV2Dto,
    pub target: String,
    pub unit: String,
    pub requirements: CanonicalSet<String>,
    pub facets: CanonicalSet<String>,
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
    pub profile_values: CanonicalSet<ProfileValueV2Dto>,
    pub producer: String,
    pub claimed_trust: String,
    pub observed_at: CanonicalInstant,
    pub payload: EvidencePayloadV2Dto,
    pub attachments: CanonicalSet<AttachmentV2Dto>,
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
    for (index, attempt) in execution.attempts.iter().enumerate() {
        // The existing attempt-index rule: attempts are numbered 1..=n in the
        // declared order. A reordered, duplicated or skipped index is invalid.
        if attempt.number.get() != index as u64 + 1 {
            return Err(V2DtoError::InvalidAttempt);
        }
        attempt
            .outcome
            .parse::<AttemptOutcome>()
            .map_err(|_| V2DtoError::InvalidVocabulary)?;
        if attempt.finished_at < attempt.started_at {
            return Err(V2DtoError::InvalidTime);
        }
    }
    Ok(())
}

fn validate_digest(value: &str) -> Result<(), V2DtoError> {
    parse_typed::<Sha256Digest>(value, V2DtoError::InvalidDigest)
}

/// Parses one authored field with its existing domain grammar or vocabulary.
///
/// This reuses the domain constructors instead of an NFC-only predicate: an
/// exact-NFC string can still violate a typed identifier's syntax or length.
fn parse_typed<T>(value: &str, error: V2DtoError) -> Result<(), V2DtoError>
where
    T: FromStr,
{
    value.parse::<T>().map(|_| ()).map_err(|_| error)
}

/// Validates one bounded, normalized authored text field ([`SelectorText`]).
fn validate_selector_text(value: &str, error: V2DtoError) -> Result<(), V2DtoError> {
    SelectorText::new(value).map(|_| ()).map_err(|_| error)
}

/// Validates one typed v2 fact value with its existing domain restrictions.
fn validate_fact_value(value: &FactValueV2Dto, error: V2DtoError) -> Result<(), V2DtoError> {
    match value {
        FactValueV2Dto::Boolean(_) | FactValueV2Dto::Integer(_) => Ok(()),
        FactValueV2Dto::Symbol(value) => parse_typed::<SymbolicValueId>(value, error),
        FactValueV2Dto::Text(value) => validate_selector_text(value, error),
    }
}

/// Validates one runtime fact's key coordinates and typed value.
fn validate_runtime_fact(fact: &RuntimeFactV2Dto) -> Result<(), V2DtoError> {
    parse_typed::<SurfaceId>(&fact.surface, V2DtoError::InvalidFact)?;
    parse_typed::<DimensionId>(&fact.dimension, V2DtoError::InvalidFact)?;
    validate_selector_text(&fact.key, V2DtoError::InvalidFact)?;
    validate_fact_value(&fact.value, V2DtoError::InvalidFact)
}

/// Validates one inventory entry's symbolic metadata and source path.
fn validate_inventory_entry(entry: &InventoryEntryV2Dto) -> Result<(), V2DtoError> {
    validate_selector_text(&entry.kind, V2DtoError::InvalidEntry)?;
    validate_selector_text(&entry.key, V2DtoError::InvalidEntry)?;
    for (name, value) in &entry.attributes {
        validate_selector_text(name, V2DtoError::InvalidEntry)?;
        validate_fact_value(value, V2DtoError::InvalidEntry)?;
    }
    parse_typed::<RepoPath>(&entry.source, V2DtoError::InvalidEntry)
}

/// Validates diagnostic unit/target references without restricting diagnostic
/// prose, source exports or other observed text.
fn validate_diagnostic(diagnostic: &DiagnosticV2Dto) -> Result<(), V2DtoError> {
    if let Some(unit) = &diagnostic.unit {
        parse_typed::<UnitId>(unit, V2DtoError::InvalidVocabulary)?;
    }
    if let Some(target) = &diagnostic.target {
        parse_typed::<TargetId>(target, V2DtoError::InvalidVocabulary)?;
    }
    Ok(())
}

/// Validates profile-value metadata and rejects duplicate `(profile, dimension)`
/// pairs and conflicting revisions within one profile. Ordering is handled
/// separately by [`CanonicalSet`].
fn validate_profile_values(values: &[ProfileValueV2Dto]) -> Result<(), V2DtoError> {
    let mut pairs = BTreeSet::new();
    let mut revisions: BTreeMap<&str, WireUint> = BTreeMap::new();
    for value in values {
        parse_typed::<ProfileId>(&value.profile, V2DtoError::InvalidProfile)?;
        Revision::new(value.revision.get()).map_err(|_| V2DtoError::InvalidProfile)?;
        parse_typed::<DimensionId>(&value.dimension, V2DtoError::InvalidProfile)?;
        parse_typed::<SymbolicValueId>(&value.value, V2DtoError::InvalidProfile)?;
        if !pairs.insert((value.profile.as_str(), value.dimension.as_str())) {
            return Err(V2DtoError::InvalidOrdering);
        }
        match revisions.get(value.profile.as_str()) {
            Some(revision) if *revision != value.revision => {
                return Err(V2DtoError::InvalidOrdering);
            }
            Some(_) => {}
            None => {
                revisions.insert(value.profile.as_str(), value.revision);
            }
        }
    }
    Ok(())
}

/// Rejects duplicate logical fact keys `(surface, dimension, key)`.
fn validate_fact_keys(facts: &[RuntimeFactV2Dto]) -> Result<(), V2DtoError> {
    let mut keys = BTreeSet::new();
    for fact in facts {
        if !keys.insert((
            fact.surface.as_str(),
            fact.dimension.as_str(),
            fact.key.as_str(),
        )) {
            return Err(V2DtoError::InvalidOrdering);
        }
    }
    Ok(())
}

/// Canonicalizes the unordered semantic arrays inside a runtime-facts object
/// so the digest binds the canonical semantic set projection rather than the
/// received array order. Duplicates are rejected, not collapsed.
fn canonicalize_semantic_arrays(root: &mut Value) -> Result<(), V2DtoError> {
    for key in ["profile_values", "facts"] {
        let Some(Value::Array(items)) = root.get_mut(key) else {
            continue;
        };
        let mut encoded: Vec<(Vec<u8>, Value)> = Vec::with_capacity(items.len());
        for item in items.drain(..) {
            let bytes =
                serde_json_canonicalizer::to_vec(&item).map_err(|_| V2DtoError::InvalidDigest)?;
            encoded.push((bytes, item));
        }
        encoded.sort_by(|left, right| left.0.cmp(&right.0));
        for window in encoded.windows(2) {
            if window[0].0 == window[1].0 {
                return Err(V2DtoError::InvalidOrdering);
            }
        }
        *items = encoded.into_iter().map(|(_, value)| value).collect();
    }
    Ok(())
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
        EvidencePayloadV2Dto::ManualReview {
            outcome, reviewer, ..
        } => {
            match outcome.parse::<AttemptOutcome>() {
                Ok(AttemptOutcome::Passed | AttemptOutcome::Failed) => {}
                _ => return Err(V2DtoError::InvalidVocabulary),
            }
            parse_typed::<OwnerRef>(reviewer, V2DtoError::InvalidVocabulary)
        }
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
        self.subject.validate()?;
        parse_typed::<TargetId>(&self.target, V2DtoError::InvalidVocabulary)?;
        parse_typed::<UnitId>(&self.unit, V2DtoError::InvalidVocabulary)?;
        for requirement in &self.requirements {
            parse_typed::<FullRequirementId>(requirement, V2DtoError::InvalidVocabulary)?;
        }
        for facet in &self.facets {
            parse_typed::<Facet>(facet, V2DtoError::InvalidVocabulary)?;
        }
        validate_profile_values(&self.profile_values)?;
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
            .parse::<EvidenceKind>()
            .map_err(|_| V2DtoError::InvalidVocabulary)?;
        self.claimed_trust
            .parse::<TrustLevel>()
            .map_err(|_| V2DtoError::InvalidVocabulary)?;
        self.producer
            .parse::<ProducerRef>()
            .map_err(|_| V2DtoError::InvalidVocabulary)?;
        let mut attachment_names = BTreeSet::new();
        for attachment in &self.attachments {
            validate_selector_text(&attachment.name, V2DtoError::InvalidVocabulary)?;
            validate_selector_text(&attachment.media_type, V2DtoError::InvalidVocabulary)?;
            validate_digest(&attachment.digest)?;
            if !attachment_names.insert(attachment.name.as_str()) {
                return Err(V2DtoError::InvalidOrdering);
            }
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
    #[schemars(schema_with = "runtime_facts_schema_marker")]
    pub schema: String,
    pub provider: String,
    pub subject: EvidenceSubjectV2Dto,
    pub target: String,
    pub profile_values: CanonicalSet<ProfileValueV2Dto>,
    pub observed_at: CanonicalInstant,
    pub expires_at: CanonicalInstant,
    pub facts: CanonicalSet<RuntimeFactV2Dto>,
    pub producer: String,
    pub claimed_trust: String,
    pub facts_digest: String,
}

impl RuntimeFactsV2Dto {
    pub fn from_json(bytes: &[u8]) -> Result<Self, V2DtoError> {
        if bytes.len() > MAX_RUNTIME_FACTS_BYTES {
            return Err(V2DtoError::InvalidLimits);
        }
        let value: Self = crate::strict::from_slice_limited(bytes, MAX_RUNTIME_FACTS_BYTES)
            .map_err(|_| V2DtoError::Json)?;
        if value.schema != crate::RUNTIME_FACTS_SCHEMA_V2.to_string() {
            return Err(V2DtoError::InvalidSchema);
        }
        let claimed = value
            .facts_digest
            .parse::<Sha256Digest>()
            .map_err(|_| V2DtoError::InvalidDigest)?;
        let mut canonical: Value =
            crate::strict::parse_strict_limited(bytes, MAX_RUNTIME_FACTS_BYTES)
                .map_err(|_| V2DtoError::Json)?;
        canonical
            .as_object_mut()
            .ok_or(V2DtoError::Json)?
            .remove("facts_digest");
        canonicalize_semantic_arrays(&mut canonical)?;
        let canonical =
            serde_json_canonicalizer::to_vec(&canonical).map_err(|_| V2DtoError::InvalidDigest)?;
        if Sha256Digest::hash_content(&canonical) != claimed {
            return Err(V2DtoError::DigestMismatch);
        }
        value.validate()?;
        Ok(value)
    }

    fn validate(&self) -> Result<(), V2DtoError> {
        parse_typed::<ProviderId>(&self.provider, V2DtoError::InvalidSubject)?;
        parse_typed::<TargetId>(&self.target, V2DtoError::InvalidSubject)?;
        parse_typed::<ProducerRef>(&self.producer, V2DtoError::InvalidVocabulary)?;
        parse_typed::<TrustLevel>(&self.claimed_trust, V2DtoError::InvalidVocabulary)?;
        if self.expires_at <= self.observed_at {
            return Err(V2DtoError::InvalidTime);
        }
        if !matches!(
            &self.subject.scope,
            ScopeSubjectV2Dto::Target { target } if target == &self.target
        ) {
            return Err(V2DtoError::InvalidSubject);
        }
        // The shared subject checks are the same structural checks the other
        // v2 roots apply (repository identity, commits, digests, build ID or
        // artifact digest). They are not later trust or lineage semantics.
        self.subject.validate()?;
        if self.facts.len() > MAX_RUNTIME_FACTS {
            return Err(V2DtoError::InvalidLimits);
        }
        validate_profile_values(&self.profile_values)?;
        validate_fact_keys(&self.facts)?;
        for fact in &self.facts {
            validate_runtime_fact(fact)?;
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
    #[schemars(schema_with = "inventory_schema_marker")]
    pub schema: String,
    pub adapter: String,
    pub adapter_digest: String,
    pub subject: EvidenceSubjectV2Dto,
    pub target: String,
    pub generated_at: CanonicalInstant,
    pub completeness: String,
    pub entries: CanonicalSet<InventoryEntryV2Dto>,
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
        value.validate_structure()?;
        Ok(value)
    }

    /// Structural checks shared by standalone ingestion and an embedded
    /// adapter-response inventory: exact v2 marker, declared byte/entry bounds,
    /// typed adapter/target/subject/completeness/entry metadata and digest
    /// syntax. It confers no trust and does not pull in later relationship or
    /// verification semantics.
    fn validate_structure(&self) -> Result<(), V2DtoError> {
        if self.schema != crate::INVENTORY_SCHEMA_V2.to_string() {
            return Err(V2DtoError::InvalidSchema);
        }
        if self.entries.len() > 250_000 {
            return Err(V2DtoError::InvalidLimits);
        }
        parse_typed::<AdapterId>(&self.adapter, V2DtoError::InvalidVocabulary)?;
        parse_typed::<TargetId>(&self.target, V2DtoError::InvalidVocabulary)?;
        parse_typed::<InventoryCompleteness>(&self.completeness, V2DtoError::InvalidVocabulary)?;
        self.subject.validate()?;
        for diagnostic in &self.diagnostics {
            validate_diagnostic(diagnostic)?;
        }
        let mut keys = BTreeSet::new();
        for entry in &self.entries {
            validate_inventory_entry(entry)?;
            if !keys.insert((entry.kind.as_str(), entry.key.as_str())) {
                return Err(V2DtoError::InvalidOrdering);
            }
        }
        validate_digest(&self.adapter_digest)?;
        validate_digest(&self.inventory_digest)?;
        Ok(())
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
    #[schemars(schema_with = "adapter_request_schema_marker")]
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
        parse_typed::<AdapterId>(&value.adapter, V2DtoError::InvalidVocabulary)?;
        parse_typed::<TargetId>(&value.target, V2DtoError::InvalidVocabulary)?;
        value.subject.validate()?;
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
    #[schemars(schema_with = "adapter_response_schema_marker")]
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
        parse_typed::<AdapterId>(&value.adapter, V2DtoError::InvalidVocabulary)?;
        for diagnostic in &value.diagnostics {
            validate_diagnostic(diagnostic)?;
        }
        // Validate the response's own adapter digest and, when present, the
        // nested inventory through the same structural checks standalone
        // ingestion applies. A weaker deserialize-only route is not used.
        validate_digest(&value.adapter_digest)?;
        if let Some(inventory) = &value.inventory {
            inventory.validate_structure()?;
        }
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonschema::{Draft, Validator};
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
            requirements: CanonicalSet::from_members(vec!["unit.test#r18".to_owned()])?,
            facets: CanonicalSet::from_members(vec!["behavior".to_owned()])?,
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
                execution: execution()?,
            },
            attachments: CanonicalSet::empty(),
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
            entries: CanonicalSet::empty(),
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
            profile_values: CanonicalSet::empty(),
            observed_at: instant("2026-01-02T03:04:05Z")?,
            expires_at: instant("2026-01-03T03:04:05Z")?,
            facts: CanonicalSet::from_members(vec![RuntimeFactV2Dto {
                surface: "account.create.signup.identifier".to_owned(),
                dimension: "availability".to_owned(),
                key: "version".to_owned(),
                value: FactValueV2Dto::Integer(WireInt::new(i64::MAX)),
            }])?,
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

    fn response() -> Result<AdapterResponseV2Dto, Box<dyn Error>> {
        Ok(AdapterResponseV2Dto {
            schema: crate::ADAPTER_RESPONSE_SCHEMA_V2.to_string(),
            request_id: "request-1".to_owned(),
            adapter: "adapter.test".to_owned(),
            adapter_digest: digest("adapter"),
            status: AdapterStatusV2Dto::Ok,
            inventory: Some(inventory()?),
            diagnostics: Vec::new(),
        })
    }

    fn assert_root_marker_rejects<T>(
        value: &Value,
        parse: impl Fn(&[u8]) -> Result<T, V2DtoError>,
    ) -> Result<(), Box<dyn Error>>
    where
        T: std::fmt::Debug + PartialEq,
    {
        let mut mutated = value.clone();
        // A v1 evidence-result URI is foreign to every v2 family, so each
        // reader must reject it at the exact-marker check.
        mutated["schema"] = serde_json::json!(crate::EVIDENCE_RESULT_SCHEMA.to_string());
        let bytes = serde_json::to_vec(&mutated)?;
        assert_eq!(parse(&bytes), Err(V2DtoError::InvalidSchema));
        Ok(())
    }

    #[test]
    fn every_v2_root_marker_rejects_a_foreign_identity() -> Result<(), Box<dyn Error>> {
        assert_root_marker_rejects(&serde_json::to_value(evidence_result()?)?, |bytes| {
            EvidenceResultV2Dto::from_json(bytes)
        })?;
        assert_root_marker_rejects(&serde_json::to_value(runtime_facts()?)?, |bytes| {
            RuntimeFactsV2Dto::from_json(bytes)
        })?;
        assert_root_marker_rejects(&serde_json::to_value(inventory()?)?, |bytes| {
            InventoryV2Dto::from_json(bytes)
        })?;
        assert_root_marker_rejects(&serde_json::to_value(adapter_request())?, |bytes| {
            AdapterRequestV2Dto::from_json(bytes)
        })?;
        assert_root_marker_rejects(&serde_json::to_value(response()?)?, |bytes| {
            AdapterResponseV2Dto::from_json(bytes)
        })?;
        Ok(())
    }

    #[test]
    fn adapter_response_enforces_nested_inventory_structure() -> Result<(), Box<dyn Error>> {
        let bytes = serde_json::to_vec(&response()?)?;
        assert!(AdapterResponseV2Dto::from_json(&bytes).is_ok());

        // A v2 inventory is accepted standalone and embedded with the same
        // structural checks; its marker must match in both routes.
        assert_eq!(
            InventoryV2Dto::from_json(&serde_json::to_vec(&inventory()?)?),
            Ok(inventory()?)
        );
        let mut standalone: Value = serde_json::to_value(inventory()?)?;
        standalone["schema"] = serde_json::json!(crate::INVENTORY_SCHEMA.to_string());
        assert_eq!(
            InventoryV2Dto::from_json(&serde_json::to_vec(&standalone)?),
            Err(V2DtoError::InvalidSchema)
        );
        let mut value: Value = serde_json::from_slice(&bytes)?;
        value["inventory"]["schema"] = serde_json::json!(crate::INVENTORY_SCHEMA.to_string());
        assert_eq!(
            AdapterResponseV2Dto::from_json(&serde_json::to_vec(&value)?),
            Err(V2DtoError::InvalidSchema)
        );

        // Digest syntax is checked on both the nested inventory fields and the
        // response's own adapter digest.
        let mut value: Value = serde_json::from_slice(&bytes)?;
        value["inventory"]["adapter_digest"] = serde_json::json!("sha256:bad");
        assert_eq!(
            AdapterResponseV2Dto::from_json(&serde_json::to_vec(&value)?),
            Err(V2DtoError::InvalidDigest)
        );
        let mut value: Value = serde_json::from_slice(&bytes)?;
        value["inventory"]["inventory_digest"] = serde_json::json!("not-a-digest");
        assert_eq!(
            AdapterResponseV2Dto::from_json(&serde_json::to_vec(&value)?),
            Err(V2DtoError::InvalidDigest)
        );
        let mut value: Value = serde_json::from_slice(&bytes)?;
        value["adapter_digest"] = serde_json::json!("sha256:bad");
        assert_eq!(
            AdapterResponseV2Dto::from_json(&serde_json::to_vec(&value)?),
            Err(V2DtoError::InvalidDigest)
        );

        // The declared byte and entry bounds hold for the response too.
        let oversized = vec![b' '; 16 * 1024 * 1024 + 1];
        assert_eq!(
            AdapterResponseV2Dto::from_json(&oversized),
            Err(V2DtoError::InvalidLimits)
        );
        let mut bounded = inventory()?;
        let entry = InventoryEntryV2Dto {
            kind: "kind".to_owned(),
            key: "key".to_owned(),
            attributes: BTreeMap::new(),
            source: "src/file.rs".to_owned(),
        };
        // Distinct logical keys keep the count bound, not duplicate rejection,
        // as the check under test.
        bounded.entries = CanonicalSet::from_members(
            (0..250_001)
                .map(|index| InventoryEntryV2Dto {
                    key: format!("key-{index:06}"),
                    ..entry.clone()
                })
                .collect(),
        )?;
        assert_eq!(bounded.validate_structure(), Err(V2DtoError::InvalidLimits));
        Ok(())
    }

    fn facts_record(count: usize) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut facts = RuntimeFactsV2Dto {
            schema: crate::RUNTIME_FACTS_SCHEMA_V2.to_string(),
            provider: "runtime.fixture".to_owned(),
            subject: subject(),
            target: "web".to_owned(),
            profile_values: CanonicalSet::empty(),
            observed_at: instant("2026-01-02T03:04:05Z")?,
            expires_at: instant("2026-01-03T03:04:05Z")?,
            facts: CanonicalSet::from_members(
                (0..count)
                    .map(|index| RuntimeFactV2Dto {
                        surface: "account.create.signup.identifier".to_owned(),
                        dimension: "availability".to_owned(),
                        key: format!("k{index:06}"),
                        value: FactValueV2Dto::Boolean(true),
                    })
                    .collect(),
            )?,
            producer: "producer://runtime/fixture/snapshot-1".to_owned(),
            claimed_trust: "signed_ci".to_owned(),
            facts_digest: String::new(),
        };
        recompute_facts_digest(&mut facts)?;
        Ok(serde_json::to_vec(&facts)?)
    }

    fn recompute_facts_digest(facts: &mut RuntimeFactsV2Dto) -> Result<(), Box<dyn Error>> {
        facts.facts_digest = facts_digest_of(facts)?;
        Ok(())
    }

    fn facts_digest_of(facts: &RuntimeFactsV2Dto) -> Result<String, Box<dyn Error>> {
        let mut value = serde_json::to_value(facts)?;
        value
            .as_object_mut()
            .ok_or("runtime facts object")?
            .remove("facts_digest");
        Ok(Sha256Digest::hash_content(&serde_json_canonicalizer::to_vec(&value)?).to_string())
    }

    #[test]
    fn v2_runtime_facts_bound_is_honored_at_every_parse() -> Result<(), Box<dyn Error>> {
        let base = serde_json::to_vec(&runtime_facts()?)?;
        assert!(base.len() < 8 * 1024 * 1024);
        // A digest-valid record past the default 8 MiB parser limit must still
        // parse: the digest preparation honors the declared 16 MiB envelope.
        let mut beyond_default = base.clone();
        beyond_default.resize(9 * 1024 * 1024, b' ');
        assert!(
            RuntimeFactsV2Dto::from_json(&beyond_default).is_ok(),
            "declared v2 16 MiB record was rejected at the default parser bound"
        );
        let mut at_bound = base.clone();
        at_bound.resize(MAX_RUNTIME_FACTS_BYTES, b' ');
        assert!(RuntimeFactsV2Dto::from_json(&at_bound).is_ok());
        let mut over = base;
        over.resize(MAX_RUNTIME_FACTS_BYTES + 1, b' ');
        assert_eq!(
            RuntimeFactsV2Dto::from_json(&over),
            Err(V2DtoError::InvalidLimits)
        );
        Ok(())
    }

    #[test]
    fn v2_runtime_facts_count_ceiling_is_enforced() -> Result<(), Box<dyn Error>> {
        // 100000 otherwise-valid facts pass; 100001 reject on the count ceiling.
        assert!(RuntimeFactsV2Dto::from_json(&facts_record(MAX_RUNTIME_FACTS)?).is_ok());
        assert_eq!(
            RuntimeFactsV2Dto::from_json(&facts_record(MAX_RUNTIME_FACTS + 1)?),
            Err(V2DtoError::InvalidLimits)
        );
        Ok(())
    }

    #[test]
    fn v2_runtime_facts_subject_syntax_is_enforced() -> Result<(), Box<dyn Error>> {
        // Each mutation changes one subject field and recomputes the enclosing
        // facts digest, so the rejection is the intended structural check and
        // not a digest mismatch.
        let base = serde_json::to_value(runtime_facts()?)?;
        let mutations: [(&str, Value); 6] = [
            ("repository", serde_json::json!("not-a-repository")),
            ("repository_id_digest", serde_json::json!("sha256:bad")),
            ("source_commit", serde_json::json!("zz")),
            (
                "target_configuration_digest",
                serde_json::json!("sha256:bad"),
            ),
            ("build_id", serde_json::json!("")),
            ("artifact_digest", serde_json::json!("not-a-digest")),
        ];
        for (field, value) in mutations {
            let mut mutated = base.clone();
            mutated["subject"][field] = value;
            let digest = {
                let mut unsigned = mutated.clone();
                unsigned
                    .as_object_mut()
                    .ok_or("runtime facts object")?
                    .remove("facts_digest");
                Sha256Digest::hash_content(&serde_json_canonicalizer::to_vec(&unsigned)?)
                    .to_string()
            };
            mutated["facts_digest"] = serde_json::json!(digest);
            assert_eq!(
                RuntimeFactsV2Dto::from_json(&serde_json::to_vec(&mutated)?),
                Err(V2DtoError::InvalidSubject),
                "malformed subject field {field} was accepted"
            );
        }
        Ok(())
    }

    fn schema_document(name: &str) -> Result<Value, Box<dyn Error>> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../schemas/v2/protocol")
            .join(format!("{name}.schema.json"));
        Ok(serde_json::from_slice(&std::fs::read(path)?)?)
    }

    fn validator(name: &str) -> Result<Validator, Box<dyn Error>> {
        let schema = schema_document(name)?;
        Ok(jsonschema::options()
            .with_draft(Draft::Draft202012)
            .should_validate_formats(true)
            .build(&schema)?)
    }

    fn assert_schema_accepts(name: &str, instance: &Value) -> Result<(), Box<dyn Error>> {
        let validator = validator(name)?;
        if let Err(error) = validator.validate(instance) {
            return Err(format!("{name} rejected a valid instance: {error}").into());
        }
        Ok(())
    }

    fn assert_schema_rejects(
        name: &str,
        instance: &Value,
        note: &str,
    ) -> Result<(), Box<dyn Error>> {
        let validator = validator(name)?;
        assert!(
            validator.validate(instance).is_err(),
            "{name} accepted {note}"
        );
        Ok(())
    }

    #[test]
    fn v2_schema_members_are_exact_constants() -> Result<(), Box<dyn Error>> {
        // `$id` alone is not the per-instance constraint: each family's `schema`
        // member must be pinned to its exact v2 URI, including the inventory
        // embedded in an adapter response.
        for (name, uri) in [
            ("evidence-result", crate::EVIDENCE_RESULT_SCHEMA_V2),
            ("runtime-facts", crate::RUNTIME_FACTS_SCHEMA_V2),
            ("inventory", crate::INVENTORY_SCHEMA_V2),
            ("adapter-request", crate::ADAPTER_REQUEST_SCHEMA_V2),
            ("adapter-response", crate::ADAPTER_RESPONSE_SCHEMA_V2),
        ] {
            let schema = schema_document(name)?;
            assert_eq!(
                schema["properties"]["schema"]["const"],
                serde_json::json!(uri.to_string()),
                "{name} schema member is not an exact constant"
            );
        }
        let response = schema_document("adapter-response")?;
        assert_eq!(
            response["$defs"]["InventoryV2Dto"]["properties"]["schema"]["const"],
            serde_json::json!(crate::INVENTORY_SCHEMA_V2.to_string()),
            "embedded inventory schema member is not an exact constant"
        );
        Ok(())
    }

    #[test]
    fn v2_committed_schemas_accept_valid_records_and_reject_mutants() -> Result<(), Box<dyn Error>>
    {
        // evidence-result: root marker, canonical/redundant/invalid-calendar
        // time, explicit null for an optional member, and integer endpoints.
        let base = serde_json::to_value(evidence_result()?)?;
        assert_schema_accepts("evidence-result", &base)?;
        let mut value = base.clone();
        value["schema"] = serde_json::json!(crate::EVIDENCE_RESULT_SCHEMA.to_string());
        assert_schema_rejects("evidence-result", &value, "a v1 root marker")?;
        let mut value = base.clone();
        value["observed_at"] = serde_json::json!("2026-01-02T03:04:05.000000001Z");
        assert_schema_accepts("evidence-result", &value)?;
        for (time, note) in [
            ("2026-01-02T03:04:05.000Z", "a redundant 3-digit fraction"),
            (
                "2026-01-02T03:04:05.123000Z",
                "a redundant 6-digit fraction",
            ),
            (
                "2026-01-02T03:04:05.123456000Z",
                "a redundant 9-digit fraction",
            ),
            ("2026-02-30T00:00:00Z", "an invalid calendar date"),
            ("2026-01-02T03:04:05+00:00", "a non-Z offset"),
        ] {
            let mut value = base.clone();
            value["observed_at"] = serde_json::json!(time);
            assert_schema_rejects("evidence-result", &value, note)?;
        }
        let mut value = base.clone();
        value["runner_digest"] = Value::Null;
        assert_schema_rejects("evidence-result", &value, "an explicit null optional")?;
        let mut value = base.clone();
        value["payload"]["execution"]["counts"]["selected"] =
            serde_json::json!(crate::MAX_SAFE_INTEGER);
        assert_schema_accepts("evidence-result", &value)?;
        let mut value = base.clone();
        value["payload"]["execution"]["counts"]["selected"] =
            serde_json::json!(crate::MAX_SAFE_INTEGER + 1);
        assert_schema_rejects("evidence-result", &value, "an out-of-range integer")?;
        let mut value = base.clone();
        value["payload"]["execution"]["counts"]["selected"] =
            serde_json::json!("18446744073709551615");
        value["payload"]["execution"]["counts"]["passed"] =
            serde_json::json!("18446744073709551615");
        assert_schema_accepts("evidence-result", &value)?;
        let mut value = base.clone();
        value["payload"]["execution"]["counts"]["selected"] = serde_json::json!("5");
        assert_schema_rejects("evidence-result", &value, "an in-range decimal string")?;

        // runtime-facts: root marker and signed integer encoding endpoints.
        let base = serde_json::to_value(runtime_facts()?)?;
        assert_schema_accepts("runtime-facts", &base)?;
        let mut value = base.clone();
        value["schema"] = serde_json::json!(crate::RUNTIME_FACTS_SCHEMA.to_string());
        assert_schema_rejects("runtime-facts", &value, "a v1 root marker")?;
        let mut value = base.clone();
        value["facts"][0]["value"]["value"] = serde_json::json!("9223372036854775807");
        assert_schema_accepts("runtime-facts", &value)?;
        let mut value = base.clone();
        value["facts"][0]["value"]["value"] = serde_json::json!(i64::MAX);
        assert_schema_rejects("runtime-facts", &value, "an out-of-range integer")?;

        // inventory: root marker, time and entry fields.
        let base = serde_json::to_value(inventory()?)?;
        assert_schema_accepts("inventory", &base)?;
        let mut value = base.clone();
        value["schema"] = serde_json::json!(crate::INVENTORY_SCHEMA.to_string());
        assert_schema_rejects("inventory", &value, "a v1 root marker")?;
        let mut value = base.clone();
        value["generated_at"] = serde_json::json!("2026-01-02T03:04:05.100Z");
        assert_schema_accepts("inventory", &value)?;
        let mut value = base.clone();
        value["generated_at"] = serde_json::json!("2026-01-02T03:04:05.100000Z");
        assert_schema_rejects("inventory", &value, "a redundant microsecond fraction")?;

        // adapter-request: root marker and numeric limits.
        let base = serde_json::to_value(adapter_request())?;
        assert_schema_accepts("adapter-request", &base)?;
        let mut value = base.clone();
        value["schema"] = serde_json::json!(crate::ADAPTER_REQUEST_SCHEMA.to_string());
        assert_schema_rejects("adapter-request", &value, "a v1 root marker")?;
        let mut value = base.clone();
        value["limits"]["max_input_bytes"] = serde_json::json!("18446744073709551615");
        assert_schema_accepts("adapter-request", &value)?;
        let mut value = base.clone();
        value["limits"]["max_input_bytes"] = serde_json::json!("0");
        assert_schema_rejects("adapter-request", &value, "an in-range decimal string")?;

        // adapter-response: root marker and the embedded inventory marker.
        let base = serde_json::to_value(response()?)?;
        assert_schema_accepts("adapter-response", &base)?;
        let mut value = base.clone();
        value["schema"] = serde_json::json!(crate::ADAPTER_RESPONSE_SCHEMA.to_string());
        assert_schema_rejects("adapter-response", &value, "a v1 root marker")?;
        let mut value = base.clone();
        value["inventory"]["schema"] = serde_json::json!(crate::INVENTORY_SCHEMA.to_string());
        assert_schema_rejects("adapter-response", &value, "a v1 embedded inventory marker")?;
        let mut value = base.clone();
        value["inventory"] = Value::Null;
        assert_schema_rejects("adapter-response", &value, "an explicit null inventory")?;
        Ok(())
    }

    #[test]
    fn schema_value_and_raw_token_layers_stay_separate() -> Result<(), Box<dyn Error>> {
        // JSON-value layer: a conforming validator treats integral numbers as
        // integers regardless of the original token spelling.
        let mut value = serde_json::to_value(adapter_request())?;
        value["limits"]["timeout_ms"] = serde_json::json!(1000.0);
        assert_schema_accepts("adapter-request", &value)?;
        let mut value = serde_json::to_value(adapter_request())?;
        value["limits"]["timeout_ms"] = serde_json::json!(1e3);
        assert_schema_accepts("adapter-request", &value)?;

        // Raw-token layer: the same spellings, and duplicate members, are
        // rejected before schema-value validation.
        let text = String::from_utf8(serde_json::to_vec(&adapter_request())?)?;
        assert!(text.contains("\"timeout_ms\":1000"));
        for token in ["1e3", "1000.0"] {
            let mutated = text.replace("\"timeout_ms\":1000", &format!("\"timeout_ms\":{token}"));
            assert!(
                AdapterRequestV2Dto::from_json(mutated.as_bytes()).is_err(),
                "raw parser accepted the token {token}"
            );
        }
        let duplicate = text.replace(
            "\"timeout_ms\":1000",
            "\"timeout_ms\":1000,\"timeout_ms\":1",
        );
        assert!(
            AdapterRequestV2Dto::from_json(duplicate.as_bytes()).is_err(),
            "raw parser accepted a duplicate member"
        );

        // Cross-field layer: the schema constrains values, not digest identity
        // or trust vocabulary, so those remain runtime responsibilities.
        let base = serde_json::to_value(evidence_result()?)?;
        let mut value = base.clone();
        value["result_digest"] = serde_json::json!(digest("other"));
        assert_schema_accepts("evidence-result", &value)?;
        assert_eq!(
            EvidenceResultV2Dto::from_json(&serde_json::to_vec(&value)?),
            Err(V2DtoError::IdentityMismatch)
        );
        let mut value = base;
        value["claimed_trust"] = serde_json::json!("not-a-trust");
        assert_schema_accepts("evidence-result", &value)?;
        assert_eq!(
            EvidenceResultV2Dto::from_json(&serde_json::to_vec(&value)?),
            Err(V2DtoError::InvalidVocabulary)
        );
        Ok(())
    }
}
