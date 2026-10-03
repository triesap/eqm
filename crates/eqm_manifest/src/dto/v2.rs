//! Strict source DTOs for the version-2 authored workspace family.
//!
//! The v2 workspace keeps every v1 shared authored field and adds an explicit
//! `source_components` table plus target `role`, `source_components` and
//! `depends_on` references. It is decoded by its own reader; there is no
//! implicit v1/v2 conversion and the v1 schema, reader and oracles stay frozen.

#![allow(missing_docs)]

use super::{ApplicabilityDto, ExtensionsDto};
use eqm_domain::{SchemaKind, SchemaUri, SchemaVersion};
use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::BTreeMap;

/// Exact canonical `sha256:<64 lowercase hex>` authored digest text.
pub struct Sha256Text;

impl JsonSchema for Sha256Text {
    fn schema_name() -> Cow<'static, str> {
        "Sha256Text".into()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "string",
            "pattern": "^sha256:[0-9a-f]{64}$",
        })
    }
}

fn provider_schema_marker(_: &mut SchemaGenerator) -> Schema {
    json_schema!({
        "const": SchemaUri::versioned(SchemaKind::Provider, SchemaVersion::V2).to_string(),
        "type": "string",
    })
}

fn cohort_schema_marker(_: &mut SchemaGenerator) -> Schema {
    json_schema!({
        "const": SchemaUri::versioned(SchemaKind::Cohort, SchemaVersion::V2).to_string(),
        "type": "string",
    })
}

fn scenario_schema_marker(_: &mut SchemaGenerator) -> Schema {
    json_schema!({
        "const": SchemaUri::versioned(SchemaKind::Scenario, SchemaVersion::V2).to_string(),
        "type": "string",
    })
}

fn participation_schema_marker(_: &mut SchemaGenerator) -> Schema {
    json_schema!({
        "const": SchemaUri::versioned(SchemaKind::Participation, SchemaVersion::V2).to_string(),
        "type": "string",
    })
}

fn workspace_schema_marker(_: &mut SchemaGenerator) -> Schema {
    json_schema!({
        "const": SchemaUri::versioned(SchemaKind::Workspace, SchemaVersion::V2).to_string(),
        "type": "string",
    })
}

/// Exact nonempty bounded selector text (512 UTF-8 bytes at the owning layer).
fn selector_text_schema(_: &mut SchemaGenerator) -> Schema {
    json_schema!({
        "type": "string",
        "minLength": 1,
        "maxLength": 512,
    })
}

/// A pattern-constrained string schema for one owning identifier grammar.
macro_rules! pattern_text {
    ($name:ident, $pattern:expr) => {
        struct $name;

        impl JsonSchema for $name {
            fn schema_name() -> Cow<'static, str> {
                stringify!($name).into()
            }

            fn inline_schema() -> bool {
                true
            }

            fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
                json_schema!({
                    "type": "string",
                    "pattern": $pattern,
                })
            }
        }
    };
}

/// A pattern-constrained string schema with an exact ASCII code-point bound.
///
/// Every owning identifier grammar is restricted to ASCII, so a JSON Schema
/// `maxLength` in code points equals the owning byte bound here.
macro_rules! bounded_pattern_text {
    ($name:ident, $pattern:expr, $max:literal) => {
        struct $name;

        impl JsonSchema for $name {
            fn schema_name() -> Cow<'static, str> {
                stringify!($name).into()
            }

            fn inline_schema() -> bool {
                true
            }

            fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
                json_schema!({
                    "type": "string",
                    "pattern": $pattern,
                    "maxLength": $max,
                })
            }
        }
    };
}

// Two or more lowercase dotted segments separated by `.`, each <= 63 bytes.
const DOTTED_ID_PATTERN: &str = "^[a-z][a-z0-9_]{0,62}(?:\\.[a-z][a-z0-9_]{0,62})+$";
// A single lowercase segment of <= 63 bytes.
const SEGMENT_ID_PATTERN: &str = "^[a-z][a-z0-9_]{0,62}$";
// One or more lowercase dotted segments, each <= 63 bytes.
const EXTENSIBLE_ID_PATTERN: &str = "^[a-z][a-z0-9_]{0,62}(?:\\.[a-z][a-z0-9_]{0,62})*$";
// A full requirement: owner (>= 2 segments, <= 255 bytes) `#` single local.
const REQUIREMENT_PATTERN: &str =
    "^(?=[^#]{2,255}#)[a-z][a-z0-9_]{0,62}(?:\\.[a-z][a-z0-9_]{0,62})+#[a-z][a-z0-9_]{0,62}$";
// A reverse-domain extension namespace of at least three lowercase segments.
const EXTENSION_NAMESPACE_PATTERN: &str = "^[a-z][a-z0-9_]{0,62}(?:\\.[a-z][a-z0-9_]{0,62}){2,}$";
// One lowercase extension object key segment of <= 63 bytes.
const EXTENSION_KEY_PATTERN: &str = "^[a-z][a-z0-9_]{0,62}$";

bounded_pattern_text!(ProviderIdText, DOTTED_ID_PATTERN, 255);
bounded_pattern_text!(ScenarioIdText, DOTTED_ID_PATTERN, 255);
bounded_pattern_text!(TargetIdText, EXTENSIBLE_ID_PATTERN, 255);
bounded_pattern_text!(ParticipationRuleIdText, DOTTED_ID_PATTERN, 255);
bounded_pattern_text!(RoleText, EXTENSIBLE_ID_PATTERN, 255);
bounded_pattern_text!(PlatformText, EXTENSIBLE_ID_PATTERN, 255);
bounded_pattern_text!(FrameworkText, EXTENSIBLE_ID_PATTERN, 255);
pattern_text!(OccurrenceText, SEGMENT_ID_PATTERN);
pattern_text!(SourceCommitText, "^(?:[0-9a-f]{40}|[0-9a-f]{64})$");
bounded_pattern_text!(RequirementText, REQUIREMENT_PATTERN, 320);

/// The existing `RepositoryIdentity` HTTPS parser, expressible in JSON Schema.
///
/// The owning parser requires a nonempty ASCII host (no uppercase or control
/// characters), a nonempty path without control characters or spaces, no
/// `?`/`#`/`@`, no trailing slash and at most 1,024 bytes. Explicit ports and
/// other host punctuation remain accepted. The code-point `maxLength` is the
/// expressible approximation of the owning UTF-8 byte bound.
fn repository_schema(_: &mut SchemaGenerator) -> Schema {
    json_schema!({
        "type": "string",
        "maxLength": 1024,
        "pattern": "^https://[\\x20-\\x22\\x24-\\x2e\\x30-\\x3e\\x5b-\\x7e]+/[^\\x00-\\x20\\x7f?#@]*[^\\x00-\\x20\\x7f/?#@]$",
    })
}

/// A nonempty duplicate-free owner array matching the owner grammar.
struct OwnerList;

impl JsonSchema for OwnerList {
    fn schema_name() -> Cow<'static, str> {
        "OwnerList".into()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "array",
            "minItems": 1,
            "uniqueItems": true,
            "items": {"type": "string", "pattern": "^owner://(?:team|role)/[a-z][a-z0-9_]{0,62}$"},
        })
    }
}

/// A duplicate-free source-component reference array.
struct SourceComponentIdList;

impl JsonSchema for SourceComponentIdList {
    fn schema_name() -> Cow<'static, str> {
        "SourceComponentIdList".into()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "array",
            "uniqueItems": true,
            "items": {"type": "string", "pattern": DOTTED_ID_PATTERN, "maxLength": 255},
        })
    }
}

/// A duplicate-free target reference array.
struct TargetIdList;

impl JsonSchema for TargetIdList {
    fn schema_name() -> Cow<'static, str> {
        "TargetIdList".into()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "array",
            "uniqueItems": true,
            "items": {"type": "string", "pattern": EXTENSIBLE_ID_PATTERN, "maxLength": 255},
        })
    }
}

/// A duplicate-free provider reference array.
struct ProviderIdList;

impl JsonSchema for ProviderIdList {
    fn schema_name() -> Cow<'static, str> {
        "ProviderIdList".into()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "array",
            "uniqueItems": true,
            "items": {"type": "string", "pattern": DOTTED_ID_PATTERN, "maxLength": 255},
        })
    }
}

/// A nonempty duplicate-free target reference array.
struct NonEmptyTargetIdList;

impl JsonSchema for NonEmptyTargetIdList {
    fn schema_name() -> Cow<'static, str> {
        "NonEmptyTargetIdList".into()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "array",
            "minItems": 1,
            "uniqueItems": true,
            "items": {"type": "string", "pattern": EXTENSIBLE_ID_PATTERN, "maxLength": 255},
        })
    }
}

/// Authored extension data keyed by the owning >= 3 segment namespace.
pub struct AuthoredExtensions;

impl JsonSchema for AuthoredExtensions {
    fn schema_name() -> Cow<'static, str> {
        "AuthoredExtensions".into()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let value = generator.subschema_for::<AuthoredExtensionValue>();
        json_schema!({
            "type": "object",
            "propertyNames": {"pattern": EXTENSION_NAMESPACE_PATTERN},
            "additionalProperties": value,
        })
    }
}

/// One recursively untagged authored extension value (never `null`).
pub struct AuthoredExtensionValue;

impl JsonSchema for AuthoredExtensionValue {
    fn schema_name() -> Cow<'static, str> {
        "AuthoredExtensionValue".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let recursive = generator.subschema_for::<AuthoredExtensionValue>();
        json_schema!({
            "oneOf": [
                {"type": "boolean"},
                {
                    "type": "integer",
                    "format": "int64",
                    "minimum": i64::MIN,
                    "maximum": i64::MAX,
                },
                {"type": "string"},
                {"type": "array", "items": recursive},
                {
                    "type": "object",
                    "propertyNames": {"pattern": EXTENSION_KEY_PATTERN},
                    "additionalProperties": recursive,
                },
            ]
        })
    }
}

/// The authored `source_components` table keyed by the owning dotted ID.
pub struct SourceComponentMap;

impl JsonSchema for SourceComponentMap {
    fn schema_name() -> Cow<'static, str> {
        "SourceComponentMap".into()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let value = generator.subschema_for::<SourceComponentV2Dto>();
        json_schema!({
            "type": "object",
            "propertyNames": {"pattern": DOTTED_ID_PATTERN, "maxLength": 255},
            "additionalProperties": value,
        })
    }
}

/// The authored `targets` table keyed by the owning extensible ID.
pub struct TargetMap;

impl JsonSchema for TargetMap {
    fn schema_name() -> Cow<'static, str> {
        "TargetMap".into()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let value = generator.subschema_for::<TargetV2Dto>();
        json_schema!({
            "type": "object",
            "propertyNames": {"pattern": EXTENSIBLE_ID_PATTERN, "maxLength": 255},
            "additionalProperties": value,
        })
    }
}

/// Exactly one of the two cohort assignment configuration claims must be present.
fn exactly_one_configuration() -> serde_json::Value {
    serde_json::json!({
        "oneOf": [
            {
                "required": ["target_configuration_digest"],
                "not": {"required": ["provider_configuration_digest"]},
            },
            {
                "required": ["provider_configuration_digest"],
                "not": {"required": ["target_configuration_digest"]},
            },
        ]
    })
}

/// Nonempty duplicate-free cohort assignments with exactly-one configuration.
fn cohort_assignments_schema(generator: &mut SchemaGenerator) -> Schema {
    let item = serde_json::to_value(generator.subschema_for::<CohortAssignmentV2Dto>())
        .unwrap_or(serde_json::Value::Null);
    let value = serde_json::json!({
        "type": "array",
        "minItems": 1,
        "uniqueItems": true,
        "items": {"allOf": [item, exactly_one_configuration()]},
    });
    Schema::from(value.as_object().cloned().unwrap_or_default())
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceV2Dto {
    #[schemars(schema_with = "workspace_schema_marker")]
    pub schema: String,
    pub contract_sources: Vec<String>,
    pub binding_sources: Vec<String>,
    pub policy_sources: Vec<String>,
    pub profile_sources: Vec<String>,
    pub runner_sources: Vec<String>,
    pub waiver_sources: Vec<String>,
    pub lockfile: Option<String>,
    pub generated_root: Option<String>,
    #[serde(default)]
    #[schemars(with = "TargetMap")]
    pub targets: BTreeMap<String, TargetV2Dto>,
    #[serde(default)]
    #[schemars(with = "SourceComponentMap")]
    pub source_components: BTreeMap<String, SourceComponentV2Dto>,
    #[serde(default)]
    #[schemars(with = "AuthoredExtensions")]
    pub extensions: ExtensionsDto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TargetV2Dto {
    pub root: String,
    #[schemars(with = "PlatformText")]
    pub platform: String,
    #[schemars(with = "FrameworkText")]
    pub framework: String,
    #[serde(default)]
    #[schemars(with = "RoleText")]
    pub role: Option<String>,
    #[serde(default)]
    #[schemars(with = "SourceComponentIdList")]
    pub source_components: Vec<String>,
    #[serde(default)]
    #[schemars(with = "TargetIdList")]
    pub depends_on: Vec<String>,
    #[serde(default)]
    #[schemars(with = "ProviderIdList")]
    pub providers: Vec<String>,
    #[schemars(with = "OwnerList")]
    pub owners: Vec<String>,
    #[serde(default)]
    #[schemars(with = "AuthoredExtensions")]
    pub extensions: ExtensionsDto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SourceComponentV2Dto {
    pub roots: Vec<String>,
    #[schemars(with = "OwnerList")]
    pub owners: Vec<String>,
    #[serde(default)]
    #[schemars(with = "SourceComponentIdList")]
    pub dependencies: Vec<String>,
    #[serde(default)]
    #[schemars(with = "AuthoredExtensions")]
    pub extensions: ExtensionsDto,
}

/// One strict v2 capability-participation rule document.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ParticipationV2Dto {
    #[schemars(schema_with = "participation_schema_marker")]
    pub schema: String,
    #[schemars(with = "ParticipationRuleIdText")]
    pub id: String,
    #[schemars(with = "RequirementText")]
    pub requirement: String,
    #[schemars(with = "NonEmptyTargetIdList")]
    pub targets: Vec<String>,
    pub applicability: ApplicabilityDto,
    #[serde(default)]
    #[schemars(with = "AuthoredExtensions")]
    pub extensions: ExtensionsDto,
}

/// One strict v2 role-bearing scenario document.
///
/// Participants carry a unique occurrence ID, an explicit role and a tagged
/// target/provider subject; dependencies are explicit directed occurrence
/// references. Repeated target/provider references with different occurrence
/// IDs remain distinct members. Provider references stay unresolved until their
/// owning step supplies provider authority.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ScenarioV2Dto {
    #[schemars(schema_with = "scenario_schema_marker")]
    pub schema: String,
    #[schemars(with = "ScenarioIdText")]
    pub id: String,
    #[schemars(range(min = 1))]
    pub revision: u64,
    #[schemars(length(min = 1))]
    pub participants: Vec<ScenarioParticipantV2Dto>,
    #[serde(default)]
    pub dependencies: Vec<ScenarioDependencyV2Dto>,
    #[serde(default)]
    #[schemars(with = "AuthoredExtensions")]
    pub extensions: ExtensionsDto,
}

/// One scenario participant occurrence.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ScenarioParticipantV2Dto {
    #[schemars(with = "OccurrenceText")]
    pub id: String,
    #[schemars(with = "RoleText")]
    pub role: String,
    pub subject: ScenarioSubjectV2Dto,
}

/// One tagged target/provider scenario subject. Exactly one tag must be present.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
#[schemars(extend("oneOf" = [
    {"required": ["target"], "not": {"required": ["provider"]}},
    {"required": ["provider"], "not": {"required": ["target"]}},
]))]
pub struct ScenarioSubjectV2Dto {
    #[serde(default)]
    #[schemars(with = "TargetIdText")]
    pub target: Option<String>,
    #[serde(default)]
    #[schemars(with = "ProviderIdText")]
    pub provider: Option<String>,
}

/// One explicit directed occurrence dependency.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ScenarioDependencyV2Dto {
    #[schemars(with = "OccurrenceText")]
    pub from: String,
    #[schemars(with = "OccurrenceText")]
    pub to: String,
}

/// One strict v2 provider authority document.
///
/// `configuration_digest` is an external content reference declared by the
/// author. It is distinct from the `eqm:v2:provider-configuration` identity the
/// canonical projection derives over this whole validated definition, and it
/// proves neither acquisition nor trust of the referenced material.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ProviderV2Dto {
    #[schemars(schema_with = "provider_schema_marker")]
    pub schema: String,
    #[schemars(with = "ProviderIdText")]
    pub id: String,
    #[schemars(with = "Sha256Text")]
    pub configuration_digest: String,
    #[schemars(with = "OwnerList")]
    pub owners: Vec<String>,
    #[serde(default)]
    #[schemars(with = "AuthoredExtensions")]
    pub extensions: ExtensionsDto,
}

/// One strict v2 compatibility-cohort document.
///
/// Assignments reuse the S038 exact-subject members and are keyed by unique
/// occurrence. Target/provider configuration is graph-backed authority and is
/// validated against the immutable projections; the authored source/build/
/// artifact axes are independently supplied when matching candidate subjects.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CohortV2Dto {
    #[schemars(schema_with = "cohort_schema_marker")]
    pub schema: String,
    #[schemars(schema_with = "selector_text_schema")]
    pub id: String,
    #[schemars(range(min = 1))]
    pub revision: u64,
    #[schemars(with = "ScenarioIdText")]
    pub scenario: String,
    #[schemars(schema_with = "cohort_assignments_schema")]
    pub assignments: Vec<CohortAssignmentV2Dto>,
    #[serde(default)]
    #[schemars(with = "AuthoredExtensions")]
    pub extensions: ExtensionsDto,
}

/// One authored cohort assignment for an exact scenario occurrence.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CohortAssignmentV2Dto {
    #[schemars(with = "OccurrenceText")]
    pub occurrence: String,
    #[schemars(schema_with = "repository_schema")]
    pub repository: String,
    #[schemars(with = "SourceCommitText")]
    pub source_commit: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(with = "Sha256Text")]
    pub source_content_digest: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(with = "Sha256Text")]
    pub target_configuration_digest: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(with = "Sha256Text")]
    pub provider_configuration_digest: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(schema_with = "selector_text_schema")]
    pub build_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(with = "Sha256Text")]
    pub artifact_digest: Option<String>,
}
