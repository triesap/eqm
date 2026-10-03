//! Typed schema projections for the explicit `eqm:v2` semantic-graph and
//! comparison-context protocol families.
//!
//! DEV-014 P03-R3 replaces the placeholder graph/comparison schemas with real
//! Draft 2020-12 member schemas generated from these typed carriers. The types
//! mirror the manifest-owned canonical projections exactly: canonical sets are
//! `CanonicalSet`, v2 integers are [`WireUint`], digests are explicit SHA-256
//! text, optional members are absent-or-present (never `null`), and the
//! runner, scope, applicability, selector and extension unions are closed tagged
//! forms. These carriers are schema/wire descriptions; the manifest projection
//! remains the byte authority and this module adds no `eqm_manifest` dependency.

#![allow(missing_docs)]

use crate::{CanonicalSet, WireInt, WireUint};
use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;

/// Exact canonical `sha256:<64 lowercase hex>` digest text.
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

fn owners_schema(_: &mut SchemaGenerator) -> Schema {
    json_schema!({
        "type": "array",
        "minItems": 1,
        "uniqueItems": true,
        "items": {"type": "string", "pattern": "^owner://(?:team|role)/[a-z][a-z0-9_]{0,62}$"},
    })
}

fn title_schema(_: &mut SchemaGenerator) -> Schema {
    json_schema!({
        "type": "string",
        "minLength": 1,
        "maxLength": 160,
    })
}

fn selector_text_schema(_: &mut SchemaGenerator) -> Schema {
    json_schema!({
        "type": "string",
        "minLength": 1,
        "maxLength": 512,
    })
}

fn description_schema(_: &mut SchemaGenerator) -> Schema {
    json_schema!({
        "type": "string",
        "minLength": 1,
        "maxLength": 4096,
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

// Two or more lowercase dotted segments, each <= 63 bytes.
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
bounded_pattern_text!(SourceComponentIdText, DOTTED_ID_PATTERN, 255);
bounded_pattern_text!(ParticipationRuleIdText, DOTTED_ID_PATTERN, 255);
bounded_pattern_text!(RoleText, EXTENSIBLE_ID_PATTERN, 255);
bounded_pattern_text!(PlatformText, EXTENSIBLE_ID_PATTERN, 255);
bounded_pattern_text!(FrameworkText, EXTENSIBLE_ID_PATTERN, 255);
bounded_pattern_text!(ProfileIdText, DOTTED_ID_PATTERN, 255);
pattern_text!(OccurrenceText, SEGMENT_ID_PATTERN);
pattern_text!(DimensionIdText, SEGMENT_ID_PATTERN);
pattern_text!(SymbolicValueText, SEGMENT_ID_PATTERN);
pattern_text!(SourceCommitText, "^(?:[0-9a-f]{40}|[0-9a-f]{64})$");
bounded_pattern_text!(RequirementText, REQUIREMENT_PATTERN, 320);
pattern_text!(ComparisonOperatorText, "^(?:eq|ne)$");
pattern_text!(MembershipOperatorText, "^(?:in|not_in)$");

// The schema-expressible lexical subset of a portable repository-relative path.
//
// The owning `RepoPath` contract additionally enforces Unicode NFC, UTF-8 byte
// lengths and portable collision keys; those are not faithfully expressible in
// JSON Schema and remain owning layers. This pattern covers the relative form,
// separator, empty/traversal segment, forbidden-punctuation, trailing space or
// dot, drive-prefix, reserved-device and segment-count rules.
const REPO_PATH_PATTERN: &str = r#"^(?!.*(?:/[^/]*){128})(?!^(?i:CON|PRN|AUX|NUL|COM[1-9]|LPT[1-9])(?:\.|/|$))(?!.*/(?i:CON|PRN|AUX|NUL|COM[1-9]|LPT[1-9])(?:\.|/|$))(?:[^/\\\x00-\x1f\x7f:*?"<>| .]|[^/\\\x00-\x1f\x7f:*?"<>|]+[^/\\\x00-\x1f\x7f:*?"<>| .])(?:/(?:[^/\\\x00-\x1f\x7f:*?"<>| .]|[^/\\\x00-\x1f\x7f:*?"<>|]+[^/\\\x00-\x1f\x7f:*?"<>| .]))*$"#;

/// The schema-expressible subset of the owning portable `RepoPath` contract.
///
/// This expresses only the lexical rules JSON Schema can faithfully prove. The
/// owning `RepoPath` still enforces NFC, exact UTF-8 byte length and portable
/// collision keys, and must accept every value this pattern admits.
struct RepoPathText;

impl JsonSchema for RepoPathText {
    fn schema_name() -> Cow<'static, str> {
        "RepoPathText".into()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "string",
            "pattern": REPO_PATH_PATTERN,
            "maxLength": 1024,
        })
    }
}

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

/// A namespaced-and-recursively-keyed map of tagged v2 extension values.
struct NamespacedExtensions;

impl JsonSchema for NamespacedExtensions {
    fn schema_name() -> Cow<'static, str> {
        "NamespacedExtensions".into()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let value = generator.subschema_for::<ExtensionValueV2Dto>();
        json_schema!({
            "type": "object",
            "propertyNames": {"pattern": EXTENSION_NAMESPACE_PATTERN},
            "additionalProperties": value,
        })
    }
}

/// The recursive object payload of a tagged extension value, keyed by one
/// lowercase segment.
struct ExtensionObjectMap;

impl JsonSchema for ExtensionObjectMap {
    fn schema_name() -> Cow<'static, str> {
        "ExtensionObjectMap".into()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let value = generator.subschema_for::<ExtensionValueV2Dto>();
        json_schema!({
            "type": "object",
            "propertyNames": {"pattern": EXTENSION_KEY_PATTERN},
            "additionalProperties": value,
        })
    }
}

/// Profile defaults keyed by a dimension ID with symbolic-value values.
struct ProfileDefaults;

impl JsonSchema for ProfileDefaults {
    fn schema_name() -> Cow<'static, str> {
        "ProfileDefaults".into()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "object",
            "propertyNames": {"pattern": SEGMENT_ID_PATTERN},
            "additionalProperties": {"type": "string", "pattern": SEGMENT_ID_PATTERN},
        })
    }
}

/// A nonempty, duplicate-free canonical set of a constrained member form.
struct NonEmptySet<T>(std::marker::PhantomData<T>);

impl<T: JsonSchema> JsonSchema for NonEmptySet<T> {
    fn schema_name() -> Cow<'static, str> {
        format!("NonEmptySet_{}", T::schema_name()).into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let items = generator.subschema_for::<T>();
        json_schema!({
            "type": "array",
            "items": items,
            "uniqueItems": true,
            "minItems": 1,
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

fn semantic_graph_schema_marker(_: &mut SchemaGenerator) -> Schema {
    json_schema!({
        "const": crate::schema::SEMANTIC_GRAPH_SCHEMA_V2.to_string(),
        "type": "string",
    })
}

fn comparison_context_schema_marker(_: &mut SchemaGenerator) -> Schema {
    json_schema!({
        "const": crate::schema::COMPARISON_CONTEXT_SCHEMA_V2.to_string(),
        "type": "string",
    })
}

// ---------------------------------------------------------------------------
// recursive extension values
// ---------------------------------------------------------------------------

/// One recursive type-tagged v2 extension value.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(
    tag = "type",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ExtensionValueV2Dto {
    Boolean(bool),
    Integer(WireInt),
    String(String),
    Array(Vec<ExtensionValueV2Dto>),
    Object(#[schemars(with = "ExtensionObjectMap")] BTreeMap<String, ExtensionValueV2Dto>),
}

// ---------------------------------------------------------------------------
// applicability
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApplicabilityConstantV2Dto {
    pub always: bool,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApplicabilityComparisonV2Dto {
    #[schemars(with = "DimensionIdText")]
    pub dimension: String,
    #[schemars(with = "ComparisonOperatorText")]
    pub operator: String,
    #[schemars(with = "SymbolicValueText")]
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApplicabilityMembershipV2Dto {
    #[schemars(with = "DimensionIdText")]
    pub dimension: String,
    #[schemars(with = "MembershipOperatorText")]
    pub operator: String,
    #[schemars(with = "NonEmptySet<SymbolicValueText>")]
    pub values: CanonicalSet<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApplicabilityAllV2Dto {
    #[schemars(with = "NonEmptySet<ApplicabilityV2Dto>")]
    pub all: CanonicalSet<ApplicabilityV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApplicabilityAnyV2Dto {
    #[schemars(with = "NonEmptySet<ApplicabilityV2Dto>")]
    pub any: CanonicalSet<ApplicabilityV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApplicabilityNotV2Dto {
    pub not: Box<ApplicabilityV2Dto>,
}

/// One recursive applicability expression.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ApplicabilityV2Dto {
    Constant(ApplicabilityConstantV2Dto),
    Comparison(ApplicabilityComparisonV2Dto),
    Membership(ApplicabilityMembershipV2Dto),
    All(ApplicabilityAllV2Dto),
    Any(ApplicabilityAnyV2Dto),
    Not(ApplicabilityNotV2Dto),
}

// ---------------------------------------------------------------------------
// selectors
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ArtifactSelectorV2Dto {
    Symbol {
        name: String,
        #[serde(
            default,
            deserialize_with = "crate::v2::deserialize_present",
            skip_serializing_if = "Option::is_none"
        )]
        #[schemars(with = "String")]
        language: Option<String>,
    },
    Route {
        path: String,
        #[serde(
            default,
            deserialize_with = "crate::v2::deserialize_present",
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
            deserialize_with = "crate::v2::deserialize_present",
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
            deserialize_with = "crate::v2::deserialize_present",
            skip_serializing_if = "Option::is_none"
        )]
        #[schemars(with = "String")]
        value: Option<String>,
    },
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum EvidenceSelectorV2Dto {
    Symbol {
        name: String,
        #[serde(
            default,
            deserialize_with = "crate::v2::deserialize_present",
            skip_serializing_if = "Option::is_none"
        )]
        #[schemars(with = "String")]
        language: Option<String>,
    },
    Route {
        path: String,
        #[serde(
            default,
            deserialize_with = "crate::v2::deserialize_present",
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
            deserialize_with = "crate::v2::deserialize_present",
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
            deserialize_with = "crate::v2::deserialize_present",
            skip_serializing_if = "Option::is_none"
        )]
        #[schemars(with = "String")]
        value: Option<String>,
    },
    Snapshot {
        snapshot_id: String,
        #[serde(
            default,
            deserialize_with = "crate::v2::deserialize_present",
            skip_serializing_if = "Option::is_none"
        )]
        #[schemars(with = "String")]
        variant: Option<String>,
    },
}

// ---------------------------------------------------------------------------
// contract components
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityV2Dto {
    pub id: String,
    pub title: String,
    pub status: String,
    pub owners: CanonicalSet<String>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub description: Option<String>,
    #[schemars(with = "NamespacedExtensions")]
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TransitionV2Dto {
    pub from: String,
    pub to: String,
    pub trigger: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JourneyV2Dto {
    pub id: String,
    pub revision: WireUint,
    pub title: String,
    pub capability: String,
    pub status: String,
    pub risk_class: String,
    pub owners: CanonicalSet<String>,
    pub surfaces: Vec<String>,
    pub transitions: CanonicalSet<TransitionV2Dto>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub description: Option<String>,
    #[schemars(with = "NamespacedExtensions")]
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RequirementV2Dto {
    pub id: String,
    pub level: String,
    pub scope: String,
    pub statement: String,
    pub facets: CanonicalSet<String>,
    pub applicability: ApplicabilityV2Dto,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub risk_class: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub provider: Option<String>,
    #[schemars(with = "NamespacedExtensions")]
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FragmentUseV2Dto {
    pub fragment: String,
    pub revision: WireUint,
    #[schemars(with = "Sha256Text")]
    pub digest: String,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub prefix: Option<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SurfaceV2Dto {
    pub id: String,
    pub revision: WireUint,
    pub title: String,
    pub journey: String,
    pub status: String,
    pub owners: CanonicalSet<String>,
    pub requirements: CanonicalSet<RequirementV2Dto>,
    pub fragment_origins: CanonicalSet<FragmentUseV2Dto>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub description: Option<String>,
    #[schemars(with = "NamespacedExtensions")]
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FragmentV2Dto {
    pub id: String,
    #[schemars(schema_with = "crate::positive_unsigned_schema")]
    pub revision: WireUint,
    pub title: String,
    pub risk_class: String,
    pub owners: CanonicalSet<String>,
    pub requirements: CanonicalSet<RequirementV2Dto>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub description: Option<String>,
    #[schemars(with = "NamespacedExtensions")]
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

// ---------------------------------------------------------------------------
// targets and sources
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TargetConfigurationV2Dto {
    #[schemars(with = "TargetIdText")]
    pub id: String,
    pub root: String,
    #[schemars(with = "PlatformText")]
    pub platform: String,
    #[schemars(with = "FrameworkText")]
    pub framework: String,
    #[schemars(schema_with = "owners_schema")]
    pub owners: CanonicalSet<String>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "RoleText")]
    pub role: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "CanonicalSet<SourceComponentIdText>")]
    pub source_components: Option<CanonicalSet<String>>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "CanonicalSet<TargetIdText>")]
    pub depends_on: Option<CanonicalSet<String>>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "CanonicalSet<ProviderIdText>")]
    pub providers: Option<CanonicalSet<String>>,
    #[schemars(with = "NamespacedExtensions")]
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SourceComponentV2Dto {
    #[schemars(with = "SourceComponentIdText")]
    pub id: String,
    #[schemars(with = "NonEmptySet<RepoPathText>")]
    pub roots: CanonicalSet<String>,
    #[schemars(schema_with = "owners_schema")]
    pub owners: CanonicalSet<String>,
    #[schemars(with = "CanonicalSet<SourceComponentIdText>")]
    pub dependencies: CanonicalSet<String>,
    #[schemars(with = "NamespacedExtensions")]
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderV2Dto {
    #[schemars(with = "ProviderIdText")]
    pub id: String,
    #[schemars(with = "Sha256Text")]
    pub configuration_digest: String,
    #[schemars(schema_with = "owners_schema")]
    pub owners: CanonicalSet<String>,
    #[schemars(with = "NamespacedExtensions")]
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CohortAssignmentV2Dto {
    #[schemars(with = "OccurrenceText")]
    pub occurrence: String,
    #[schemars(schema_with = "repository_schema")]
    pub repository: String,
    #[schemars(with = "SourceCommitText")]
    pub source_commit: String,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "Sha256Text")]
    pub source_content_digest: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "Sha256Text")]
    pub target_configuration_digest: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "Sha256Text")]
    pub provider_configuration_digest: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(schema_with = "selector_text_schema")]
    pub build_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "Sha256Text")]
    pub artifact_digest: Option<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CohortV2Dto {
    #[schemars(schema_with = "selector_text_schema")]
    pub id: String,
    #[schemars(schema_with = "crate::positive_unsigned_schema")]
    pub revision: WireUint,
    #[schemars(with = "ScenarioIdText")]
    pub scenario: String,
    #[schemars(schema_with = "cohort_assignments_schema")]
    pub assignments: CanonicalSet<CohortAssignmentV2Dto>,
    #[schemars(with = "NamespacedExtensions")]
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ParticipationV2Dto {
    #[schemars(with = "ParticipationRuleIdText")]
    pub id: String,
    #[schemars(with = "RequirementText")]
    pub requirement: String,
    #[schemars(with = "NonEmptySet<TargetIdText>")]
    pub targets: CanonicalSet<String>,
    pub applicability: ApplicabilityV2Dto,
    #[schemars(with = "NamespacedExtensions")]
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

// ---------------------------------------------------------------------------
// bindings and evidence
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactV2Dto {
    pub id: String,
    pub role: String,
    pub path: String,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub surface: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub symbol: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "ArtifactSelectorV2Dto")]
    pub selector: Option<ArtifactSelectorV2Dto>,
    #[schemars(with = "NamespacedExtensions")]
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExposureV2Dto {
    pub surface: String,
    pub state: String,
    pub applicability: ApplicabilityV2Dto,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub route: Option<String>,
    #[schemars(with = "NamespacedExtensions")]
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceSpecV2Dto {
    pub id: String,
    pub kind: String,
    pub requirements: CanonicalSet<String>,
    pub facets: CanonicalSet<String>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub runner: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "EvidenceSelectorV2Dto")]
    pub selector: Option<EvidenceSelectorV2Dto>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "WireUint")]
    pub minimum_count: Option<WireUint>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "WireUint")]
    pub freshness: Option<WireUint>,
    #[schemars(with = "NamespacedExtensions")]
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BindingV2Dto {
    pub id: String,
    pub revision: WireUint,
    pub target: String,
    pub unit: String,
    pub owners: CanonicalSet<String>,
    pub artifacts: CanonicalSet<ArtifactV2Dto>,
    pub exposures: CanonicalSet<ExposureV2Dto>,
    pub evidence: CanonicalSet<EvidenceSpecV2Dto>,
    #[schemars(with = "NamespacedExtensions")]
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

// ---------------------------------------------------------------------------
// scenarios
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ScenarioSubjectV2Dto {
    Target {
        #[schemars(with = "TargetIdText")]
        target: String,
    },
    Provider {
        #[schemars(with = "ProviderIdText")]
        provider: String,
    },
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioParticipantV2Dto {
    #[schemars(with = "OccurrenceText")]
    pub id: String,
    #[schemars(with = "RoleText")]
    pub role: String,
    pub subject: ScenarioSubjectV2Dto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioDependencyV2Dto {
    #[schemars(with = "OccurrenceText")]
    pub from: String,
    #[schemars(with = "OccurrenceText")]
    pub to: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioV2Dto {
    #[schemars(with = "ScenarioIdText")]
    pub id: String,
    #[schemars(schema_with = "crate::positive_unsigned_schema")]
    pub revision: WireUint,
    #[schemars(with = "NonEmptySet<ScenarioParticipantV2Dto>")]
    pub participants: CanonicalSet<ScenarioParticipantV2Dto>,
    pub dependencies: CanonicalSet<ScenarioDependencyV2Dto>,
    #[schemars(with = "NamespacedExtensions")]
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

// ---------------------------------------------------------------------------
// policy and profiles
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PolicySelectorV2Dto {
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "CanonicalSet<String>")]
    pub units: Option<CanonicalSet<String>>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "CanonicalSet<String>")]
    pub requirements: Option<CanonicalSet<String>>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "CanonicalSet<String>")]
    pub risk_classes: Option<CanonicalSet<String>>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "CanonicalSet<String>")]
    pub facets: Option<CanonicalSet<String>>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "CanonicalSet<String>")]
    pub scopes: Option<CanonicalSet<String>>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyRuleV2Dto {
    pub selector: PolicySelectorV2Dto,
    pub minimum_level: String,
    pub facets: CanonicalSet<String>,
    pub minimum_trust: String,
    pub maximum_age: WireUint,
    pub minimum_count: WireUint,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WaiverPolicyV2Dto {
    pub allowed: bool,
    pub minimum_approvers: WireUint,
    pub required_controls: CanonicalSet<String>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "WireUint")]
    pub maximum_days: Option<WireUint>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyV2Dto {
    pub id: String,
    pub revision: WireUint,
    pub title: String,
    pub owners: CanonicalSet<String>,
    pub profiles: CanonicalSet<String>,
    pub required_targets: CanonicalSet<String>,
    pub rules: CanonicalSet<PolicyRuleV2Dto>,
    pub waivers: WaiverPolicyV2Dto,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub description: Option<String>,
    #[schemars(with = "NamespacedExtensions")]
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DimensionDeclarationV2Dto {
    #[schemars(with = "DimensionIdText")]
    pub id: String,
    #[schemars(with = "NonEmptySet<SymbolicValueText>")]
    pub values: CanonicalSet<String>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(schema_with = "description_schema")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileDefinitionV2Dto {
    #[schemars(with = "ProfileIdText")]
    pub id: String,
    #[schemars(schema_with = "crate::positive_unsigned_schema")]
    pub revision: WireUint,
    #[schemars(schema_with = "title_schema")]
    pub title: String,
    #[schemars(schema_with = "owners_schema")]
    pub owners: CanonicalSet<String>,
    #[schemars(with = "NonEmptySet<DimensionDeclarationV2Dto>")]
    pub dimensions: CanonicalSet<DimensionDeclarationV2Dto>,
    #[schemars(with = "ProfileDefaults")]
    pub defaults: BTreeMap<String, String>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(schema_with = "description_schema")]
    pub description: Option<String>,
    #[schemars(with = "NamespacedExtensions")]
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DimensionKnownV2Dto {
    #[schemars(with = "DimensionIdText")]
    pub dimension: String,
    pub state: KnownState,
    #[schemars(with = "SymbolicValueText")]
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DimensionUnknownV2Dto {
    #[schemars(with = "DimensionIdText")]
    pub dimension: String,
    pub state: UnknownState,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DimensionValueV2Dto {
    Known(DimensionKnownV2Dto),
    Unknown(DimensionUnknownV2Dto),
}

/// The exact `known` state marker.
#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub enum KnownState {
    #[serde(rename = "known")]
    Known,
}

/// The exact `unknown` state marker.
#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub enum UnknownState {
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelectedProfileV2Dto {
    pub definition: ProfileDefinitionV2Dto,
    #[schemars(with = "NonEmptySet<DimensionValueV2Dto>")]
    pub values: CanonicalSet<DimensionValueV2Dto>,
}

// ---------------------------------------------------------------------------
// runners
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum RunnerProgramV2Dto {
    Repository {
        path: String,
    },
    Locked {
        resolved: String,
        #[schemars(with = "Sha256Text")]
        digest: String,
    },
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ArgumentV2Dto {
    Literal { value: String },
    TargetRoot,
    SelectorJson,
    ResultPath,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum WorkingDirectoryV2Dto {
    TargetRoot,
    Repository { path: String },
    ResultPath,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentBindingV2Dto {
    pub name: String,
    pub source: String,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBindingV2Dto {
    pub name: String,
    pub provider: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RunnerV2Dto {
    pub id: String,
    pub revision: WireUint,
    pub owners: CanonicalSet<String>,
    pub backend: String,
    pub program: RunnerProgramV2Dto,
    pub args: Vec<ArgumentV2Dto>,
    pub cwd: WorkingDirectoryV2Dto,
    pub environment: CanonicalSet<EnvironmentBindingV2Dto>,
    pub secrets: CanonicalSet<SecretBindingV2Dto>,
    pub timeout_ms: WireUint,
    pub max_output_bytes: WireUint,
    pub max_concurrency: WireUint,
    pub guarantees: CanonicalSet<String>,
    #[schemars(with = "NamespacedExtensions")]
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

// ---------------------------------------------------------------------------
// waivers
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WaiverScopeV2Dto {
    pub target: crate::v2::ScopeSubjectV2Dto,
    pub unit: String,
    pub requirement: String,
    pub facets: CanonicalSet<String>,
    pub profiles: BTreeMap<String, BTreeMap<String, String>>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WaiverV2Dto {
    pub id: String,
    pub revision: WireUint,
    pub owners: CanonicalSet<String>,
    pub policy: String,
    pub scope: WaiverScopeV2Dto,
    pub reason: String,
    pub issue: String,
    pub approvers: CanonicalSet<String>,
    pub starts_on: String,
    pub expires_on: String,
    pub controls: CanonicalSet<String>,
    #[schemars(with = "NamespacedExtensions")]
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

// ---------------------------------------------------------------------------
// locks
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ImportLockV2Dto {
    pub id: String,
    pub revision: WireUint,
    pub source: String,
    pub resolved: String,
    #[schemars(with = "Sha256Text")]
    pub digest: String,
    pub trust: String,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub signature: Option<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterLockV2Dto {
    pub id: String,
    pub version: String,
    pub source: String,
    pub resolved: String,
    #[schemars(with = "Sha256Text")]
    pub digest: String,
    pub protocol: WireUint,
    pub trust: String,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub signature: Option<String>,
}

// ---------------------------------------------------------------------------
// roots
// ---------------------------------------------------------------------------

/// The complete `eqm:v2:semantic-graph` wire root.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticGraphV2Dto {
    #[schemars(schema_with = "semantic_graph_schema_marker")]
    pub schema: String,
    pub capabilities: CanonicalSet<CapabilityV2Dto>,
    pub journeys: CanonicalSet<JourneyV2Dto>,
    pub surfaces: CanonicalSet<SurfaceV2Dto>,
    pub fragments: CanonicalSet<FragmentV2Dto>,
    pub targets: CanonicalSet<TargetConfigurationV2Dto>,
    pub source_components: CanonicalSet<SourceComponentV2Dto>,
    pub providers: CanonicalSet<ProviderV2Dto>,
    pub cohorts: CanonicalSet<CohortV2Dto>,
    pub participation: CanonicalSet<ParticipationV2Dto>,
    pub bindings: CanonicalSet<BindingV2Dto>,
    pub scenarios: CanonicalSet<ScenarioV2Dto>,
    pub policies: CanonicalSet<PolicyV2Dto>,
    pub profiles: CanonicalSet<ProfileDefinitionV2Dto>,
    pub runners: CanonicalSet<RunnerV2Dto>,
    pub waivers: CanonicalSet<WaiverV2Dto>,
    pub imports: CanonicalSet<ImportLockV2Dto>,
    pub adapters: CanonicalSet<AdapterLockV2Dto>,
    #[schemars(with = "NamespacedExtensions")]
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

/// The exact `eqm:v2:comparison-context` wire root.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ComparisonContextV2Dto {
    #[schemars(schema_with = "comparison_context_schema_marker")]
    pub schema: String,
    #[schemars(with = "Sha256Text")]
    pub contract_digest: String,
    #[schemars(with = "Sha256Text")]
    pub scenario_digest: String,
    #[schemars(with = "NonEmptySet<SelectedProfileV2Dto>")]
    pub profiles: CanonicalSet<SelectedProfileV2Dto>,
    #[schemars(with = "Sha256Text")]
    pub comparison_rule_digest: String,
}

// ---------------------------------------------------------------------------
// checked comparison-context ingress
// ---------------------------------------------------------------------------

use eqm_domain::{
    ComparisonContextDigest, Description, DimensionId, ExtensionKey, ExtensionNamespace,
    ExtensionValue, Extensions, OwnerRef, Profile, ProfileDimension, ProfileId, Revision,
    Sha256Digest, SymbolicValueId, Title,
};
use std::collections::BTreeSet;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// Strict comparison-context ingress failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComparisonContextError {
    /// The bytes were not strict bounded JSON or did not match the DTO.
    Json,
    /// The exact schema marker was absent or wrong.
    InvalidSchema,
    /// A digest/reference member was not canonical SHA-256 text.
    InvalidDigest,
    /// A symbolic member used an invalid domain vocabulary.
    InvalidVocabulary,
    /// The same selected profile identity appeared more than once.
    DuplicateProfile,
    /// A selected dimension appeared more than once.
    DuplicateDimension,
    /// A declared dimension was omitted from the selection.
    MissingDimension,
    /// A selected dimension was not declared by its profile.
    UndeclaredDimension,
    /// A known value was outside its dimension's finite declaration.
    UndeclaredValue,
    /// A dimension declaration repeated an identity or value.
    InvalidDefinition,
    /// A recursive extension value, namespace or bound was invalid.
    InvalidExtension,
    /// The supplied bytes did not parse to an equal record.
    RecordMismatch,
}

impl Display for ComparisonContextError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for ComparisonContextError {}

impl DimensionValueV2Dto {
    fn dimension(&self) -> &str {
        match self {
            Self::Known(value) => value.dimension.as_str(),
            Self::Unknown(value) => value.dimension.as_str(),
        }
    }

    fn known(&self) -> Option<(&str, &str)> {
        match self {
            Self::Known(value) => Some((value.dimension.as_str(), value.value.as_str())),
            Self::Unknown(_) => None,
        }
    }
}

impl ProfileDefinitionV2Dto {
    /// Rebuilds the owning domain profile so that full construction invariants
    /// (nonempty valid title/owners/dimensions, duplicate logical keys, finite
    /// values, defaults, optional description and recursive extension bounds)
    /// are enforced by the shared domain validator. The rebuilt value is only a
    /// validation artifact and grants no trust.
    fn to_domain_profile(&self) -> Result<Profile, ComparisonContextError> {
        let id = ProfileId::new(self.id.as_str())
            .map_err(|_| ComparisonContextError::InvalidVocabulary)?;
        let revision = Revision::new(self.revision.get())
            .map_err(|_| ComparisonContextError::InvalidVocabulary)?;
        let title = Title::new(self.title.as_str())
            .map_err(|_| ComparisonContextError::InvalidDefinition)?;
        let owners = self
            .owners
            .iter()
            .map(|owner| {
                OwnerRef::from_str(owner.as_str())
                    .map_err(|_| ComparisonContextError::InvalidDefinition)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let dimensions = self
            .dimensions
            .iter()
            .map(|dimension| {
                let id = DimensionId::new(dimension.id.as_str())
                    .map_err(|_| ComparisonContextError::InvalidVocabulary)?;
                let values = dimension
                    .values
                    .iter()
                    .map(|value| {
                        SymbolicValueId::new(value.as_str())
                            .map_err(|_| ComparisonContextError::InvalidVocabulary)
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let description = dimension
                    .description
                    .as_deref()
                    .map(Description::new)
                    .transpose()
                    .map_err(|_| ComparisonContextError::InvalidDefinition)?;
                ProfileDimension::new(id, values, description)
                    .map_err(|_| ComparisonContextError::InvalidDefinition)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let defaults = self
            .defaults
            .iter()
            .map(|(dimension, value)| {
                Ok((
                    DimensionId::new(dimension.as_str())
                        .map_err(|_| ComparisonContextError::InvalidVocabulary)?,
                    SymbolicValueId::new(value.as_str())
                        .map_err(|_| ComparisonContextError::InvalidVocabulary)?,
                ))
            })
            .collect::<Result<Vec<_>, ComparisonContextError>>()?;
        let description = self
            .description
            .as_deref()
            .map(Description::new)
            .transpose()
            .map_err(|_| ComparisonContextError::InvalidDefinition)?;
        let extensions = domain_extensions(&self.extensions)?;
        Profile::new(
            id,
            revision,
            title,
            owners,
            dimensions,
            defaults,
            description,
            extensions,
        )
        .map_err(|_| ComparisonContextError::InvalidDefinition)
    }
}

fn domain_extensions(
    values: &BTreeMap<String, ExtensionValueV2Dto>,
) -> Result<Extensions, ComparisonContextError> {
    let converted = values
        .iter()
        .map(|(namespace, value)| {
            Ok((
                ExtensionNamespace::new(namespace.as_str())
                    .map_err(|_| ComparisonContextError::InvalidExtension)?,
                domain_extension_value(value)?,
            ))
        })
        .collect::<Result<BTreeMap<_, _>, ComparisonContextError>>()?;
    Extensions::new(converted).map_err(|_| ComparisonContextError::InvalidExtension)
}

fn domain_extension_value(
    value: &ExtensionValueV2Dto,
) -> Result<ExtensionValue, ComparisonContextError> {
    match value {
        ExtensionValueV2Dto::Boolean(value) => Ok(ExtensionValue::Boolean(*value)),
        ExtensionValueV2Dto::Integer(value) => Ok(ExtensionValue::Integer(value.get())),
        ExtensionValueV2Dto::String(value) => ExtensionValue::string(value.as_str())
            .map_err(|_| ComparisonContextError::InvalidExtension),
        ExtensionValueV2Dto::Array(values) => values
            .iter()
            .map(domain_extension_value)
            .collect::<Result<Vec<_>, _>>()
            .map(ExtensionValue::Array),
        ExtensionValueV2Dto::Object(values) => values
            .iter()
            .map(|(key, value)| {
                Ok((
                    ExtensionKey::new(key.as_str())
                        .map_err(|_| ComparisonContextError::InvalidExtension)?,
                    domain_extension_value(value)?,
                ))
            })
            .collect::<Result<BTreeMap<_, _>, ComparisonContextError>>()
            .map(ExtensionValue::Object),
    }
}

impl ComparisonContextV2Dto {
    /// Strictly decodes and validates one v2 comparison-context wire document.
    pub fn from_json(bytes: &[u8]) -> Result<Self, ComparisonContextError> {
        let value: Self =
            crate::strict::from_slice(bytes).map_err(|_| ComparisonContextError::Json)?;
        value.validate()?;
        Ok(value)
    }

    /// Revalidates the mutable DTO and binds it to the exact supplied bytes.
    pub fn checked(
        &self,
        raw_bytes: &[u8],
    ) -> Result<CheckedComparisonContextV2, ComparisonContextError> {
        let reparsed = Self::from_json(raw_bytes)?;
        if &reparsed != self {
            return Err(ComparisonContextError::RecordMismatch);
        }
        CheckedComparisonContextV2::from_validated(reparsed, raw_bytes.to_vec())
    }

    fn validate(&self) -> Result<(), ComparisonContextError> {
        if self.schema != crate::COMPARISON_CONTEXT_SCHEMA_V2.to_string() {
            return Err(ComparisonContextError::InvalidSchema);
        }
        for digest in [
            &self.contract_digest,
            &self.scenario_digest,
            &self.comparison_rule_digest,
        ] {
            digest
                .parse::<Sha256Digest>()
                .map_err(|_| ComparisonContextError::InvalidDigest)?;
        }
        let mut profiles: BTreeSet<String> = BTreeSet::new();
        for selected in self.profiles.iter() {
            if !profiles.insert(selected.definition.id.clone()) {
                return Err(ComparisonContextError::DuplicateProfile);
            }
            // Full owning-domain profile construction invariants are enforced
            // by the shared validator, not a smaller parallel notion.
            let profile = selected.definition.to_domain_profile()?;
            let mut seen: BTreeSet<String> = BTreeSet::new();
            for value in selected.values.iter() {
                let dimension = value.dimension();
                let dimension_id = DimensionId::new(dimension)
                    .map_err(|_| ComparisonContextError::InvalidVocabulary)?;
                let declared = profile
                    .dimensions()
                    .get(&dimension_id)
                    .ok_or(ComparisonContextError::UndeclaredDimension)?;
                if !seen.insert(dimension.to_owned()) {
                    return Err(ComparisonContextError::DuplicateDimension);
                }
                if let Some((_, known)) = value.known() {
                    let known = SymbolicValueId::new(known)
                        .map_err(|_| ComparisonContextError::InvalidVocabulary)?;
                    if !declared.values().contains(&known) {
                        return Err(ComparisonContextError::UndeclaredValue);
                    }
                }
            }
            if seen.len() != profile.dimensions().len() {
                return Err(ComparisonContextError::MissingDimension);
            }
        }
        Ok(())
    }

    /// Returns the exact canonical semantic bytes over the four shared members,
    /// excluding only the transport `schema` marker.
    pub fn canonical_semantic_bytes(&self) -> Result<Vec<u8>, ComparisonContextError> {
        let mut value = serde_json::to_value(self).map_err(|_| ComparisonContextError::Json)?;
        value
            .as_object_mut()
            .ok_or(ComparisonContextError::Json)?
            .remove("schema");
        serde_json_canonicalizer::to_vec(&value).map_err(|_| ComparisonContextError::Json)
    }

    /// Returns the independently defined shared comparison identity.
    pub fn comparison_context_digest(
        &self,
    ) -> Result<ComparisonContextDigest, ComparisonContextError> {
        Ok(ComparisonContextDigest::hash_content(
            &self.canonical_semantic_bytes()?,
        ))
    }
}

/// Immutable bytes-first checked v2 comparison context.
#[derive(Clone, Debug, PartialEq)]
pub struct CheckedComparisonContextV2 {
    record: ComparisonContextV2Dto,
    raw_bytes: Vec<u8>,
    semantic_bytes: Vec<u8>,
    digest: ComparisonContextDigest,
}

impl CheckedComparisonContextV2 {
    /// Strictly decodes exact bytes once and retains them.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ComparisonContextError> {
        let record = ComparisonContextV2Dto::from_json(bytes)?;
        Self::from_validated(record, bytes.to_vec())
    }

    fn from_validated(
        record: ComparisonContextV2Dto,
        raw_bytes: Vec<u8>,
    ) -> Result<Self, ComparisonContextError> {
        let semantic_bytes = record.canonical_semantic_bytes()?;
        let digest = ComparisonContextDigest::hash_content(&semantic_bytes);
        Ok(Self {
            record,
            raw_bytes,
            semantic_bytes,
            digest,
        })
    }

    /// Returns the validated record.
    #[must_use]
    pub const fn record(&self) -> &ComparisonContextV2Dto {
        &self.record
    }

    /// Returns the exact original bytes.
    #[must_use]
    pub fn raw_bytes(&self) -> &[u8] {
        &self.raw_bytes
    }

    /// Returns the canonical four-member semantic bytes.
    #[must_use]
    pub fn semantic_bytes(&self) -> &[u8] {
        &self.semantic_bytes
    }

    /// Returns the typed shared comparison identity.
    #[must_use]
    pub const fn digest(&self) -> ComparisonContextDigest {
        self.digest
    }
}
