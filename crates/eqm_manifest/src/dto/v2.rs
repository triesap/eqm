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

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceV2Dto {
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
    pub targets: BTreeMap<String, TargetV2Dto>,
    #[serde(default)]
    pub source_components: BTreeMap<String, SourceComponentV2Dto>,
    #[serde(default)]
    pub extensions: ExtensionsDto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TargetV2Dto {
    pub root: String,
    pub platform: String,
    pub framework: String,
    pub role: Option<String>,
    #[serde(default)]
    pub source_components: Vec<String>,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default)]
    pub providers: Vec<String>,
    pub owners: Vec<String>,
    #[serde(default)]
    pub extensions: ExtensionsDto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SourceComponentV2Dto {
    pub roots: Vec<String>,
    pub owners: Vec<String>,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub extensions: ExtensionsDto,
}

/// One strict v2 capability-participation rule document.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ParticipationV2Dto {
    pub schema: String,
    pub id: String,
    pub requirement: String,
    pub targets: Vec<String>,
    pub applicability: ApplicabilityDto,
    #[serde(default)]
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
    pub schema: String,
    pub id: String,
    pub revision: u64,
    pub participants: Vec<ScenarioParticipantV2Dto>,
    #[serde(default)]
    pub dependencies: Vec<ScenarioDependencyV2Dto>,
    #[serde(default)]
    pub extensions: ExtensionsDto,
}

/// One scenario participant occurrence.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ScenarioParticipantV2Dto {
    pub id: String,
    pub role: String,
    pub subject: ScenarioSubjectV2Dto,
}

/// One tagged target/provider scenario subject. Exactly one tag must be present.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ScenarioSubjectV2Dto {
    pub target: Option<String>,
    pub provider: Option<String>,
}

/// One explicit directed occurrence dependency.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ScenarioDependencyV2Dto {
    pub from: String,
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
    pub id: String,
    #[schemars(with = "Sha256Text")]
    pub configuration_digest: String,
    #[schemars(length(min = 1), inner(length(min = 1)))]
    pub owners: Vec<String>,
    #[serde(default)]
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
    pub id: String,
    pub revision: u64,
    pub scenario: String,
    #[schemars(length(min = 1))]
    pub assignments: Vec<CohortAssignmentV2Dto>,
    #[serde(default)]
    pub extensions: ExtensionsDto,
}

/// One authored cohort assignment for an exact scenario occurrence.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CohortAssignmentV2Dto {
    pub occurrence: String,
    pub repository: String,
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
    #[schemars(with = "String")]
    pub build_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(with = "Sha256Text")]
    pub artifact_digest: Option<String>,
}
