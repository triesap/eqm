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
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ExtensionValueV2Dto {
    Boolean(bool),
    Integer(WireInt),
    String(String),
    Array(Vec<ExtensionValueV2Dto>),
    Object(BTreeMap<String, ExtensionValueV2Dto>),
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
    pub dimension: String,
    pub operator: String,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApplicabilityMembershipV2Dto {
    pub dimension: String,
    pub operator: String,
    pub values: CanonicalSet<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApplicabilityAllV2Dto {
    pub all: CanonicalSet<ApplicabilityV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApplicabilityAnyV2Dto {
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
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FragmentV2Dto {
    pub id: String,
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
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

// ---------------------------------------------------------------------------
// targets and sources
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TargetConfigurationV2Dto {
    pub id: String,
    pub root: String,
    pub platform: String,
    pub framework: String,
    pub owners: CanonicalSet<String>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub role: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "CanonicalSet<String>")]
    pub source_components: Option<CanonicalSet<String>>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "CanonicalSet<String>")]
    pub depends_on: Option<CanonicalSet<String>>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "CanonicalSet<String>")]
    pub providers: Option<CanonicalSet<String>>,
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SourceComponentV2Dto {
    pub id: String,
    pub roots: CanonicalSet<String>,
    pub owners: CanonicalSet<String>,
    pub dependencies: CanonicalSet<String>,
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderV2Dto {
    pub id: String,
    #[schemars(with = "Sha256Text")]
    pub configuration_digest: String,
    pub owners: CanonicalSet<String>,
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CohortAssignmentV2Dto {
    pub occurrence: String,
    pub repository: String,
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
    #[schemars(with = "String")]
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
    pub id: String,
    pub revision: WireUint,
    pub scenario: String,
    pub assignments: CanonicalSet<CohortAssignmentV2Dto>,
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ParticipationV2Dto {
    pub id: String,
    pub requirement: String,
    pub targets: CanonicalSet<String>,
    pub applicability: ApplicabilityV2Dto,
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
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

// ---------------------------------------------------------------------------
// scenarios
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ScenarioSubjectV2Dto {
    Target { target: String },
    Provider { provider: String },
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioParticipantV2Dto {
    pub id: String,
    pub role: String,
    pub subject: ScenarioSubjectV2Dto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioDependencyV2Dto {
    pub from: String,
    pub to: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioV2Dto {
    pub id: String,
    pub revision: WireUint,
    pub participants: CanonicalSet<ScenarioParticipantV2Dto>,
    pub dependencies: CanonicalSet<ScenarioDependencyV2Dto>,
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
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DimensionDeclarationV2Dto {
    pub id: String,
    pub values: CanonicalSet<String>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileDefinitionV2Dto {
    pub id: String,
    pub revision: WireUint,
    pub title: String,
    pub owners: CanonicalSet<String>,
    pub dimensions: CanonicalSet<DimensionDeclarationV2Dto>,
    pub defaults: BTreeMap<String, String>,
    #[serde(
        default,
        deserialize_with = "crate::v2::deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "String")]
    pub description: Option<String>,
    pub extensions: BTreeMap<String, ExtensionValueV2Dto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DimensionKnownV2Dto {
    pub dimension: String,
    pub state: KnownState,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DimensionUnknownV2Dto {
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
    pub profiles: CanonicalSet<SelectedProfileV2Dto>,
    #[schemars(with = "Sha256Text")]
    pub comparison_rule_digest: String,
}

// ---------------------------------------------------------------------------
// checked comparison-context ingress
// ---------------------------------------------------------------------------

use eqm_domain::{
    ComparisonContextDigest, DimensionId, ProfileId, Revision, Sha256Digest, SymbolicValueId,
};
use std::collections::BTreeSet;
use std::fmt::{self, Display, Formatter};

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
    fn declared_dimensions(
        &self,
    ) -> Result<BTreeMap<String, BTreeSet<String>>, ComparisonContextError> {
        let mut declared: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for dimension in self.dimensions.iter() {
            DimensionId::new(dimension.id.as_str())
                .map_err(|_| ComparisonContextError::InvalidVocabulary)?;
            if declared.contains_key(dimension.id.as_str()) {
                return Err(ComparisonContextError::InvalidDefinition);
            }
            let mut values = BTreeSet::new();
            for value in dimension.values.iter() {
                SymbolicValueId::new(value.as_str())
                    .map_err(|_| ComparisonContextError::InvalidVocabulary)?;
                if !values.insert(value.clone()) {
                    return Err(ComparisonContextError::InvalidDefinition);
                }
            }
            declared.insert(dimension.id.clone(), values);
        }
        for (dimension, value) in &self.defaults {
            let values = declared
                .get(dimension)
                .ok_or(ComparisonContextError::UndeclaredDimension)?;
            if !values.contains(value) {
                return Err(ComparisonContextError::UndeclaredValue);
            }
        }
        Ok(declared)
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
            ProfileId::new(selected.definition.id.as_str())
                .map_err(|_| ComparisonContextError::InvalidVocabulary)?;
            Revision::new(selected.definition.revision.get())
                .map_err(|_| ComparisonContextError::InvalidVocabulary)?;
            if !profiles.insert(selected.definition.id.clone()) {
                return Err(ComparisonContextError::DuplicateProfile);
            }
            let declared = selected.definition.declared_dimensions()?;
            let mut seen: BTreeSet<String> = BTreeSet::new();
            for value in selected.values.iter() {
                let dimension = value.dimension();
                DimensionId::new(dimension)
                    .map_err(|_| ComparisonContextError::InvalidVocabulary)?;
                let finite = declared
                    .get(dimension)
                    .ok_or(ComparisonContextError::UndeclaredDimension)?;
                if !seen.insert(dimension.to_owned()) {
                    return Err(ComparisonContextError::DuplicateDimension);
                }
                if let Some((_, known)) = value.known()
                    && !finite.contains(known)
                {
                    return Err(ComparisonContextError::UndeclaredValue);
                }
            }
            if seen.len() != declared.len() {
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
