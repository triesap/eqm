//! Strict source DTOs for the version-2 authored workspace family.
//!
//! The v2 workspace keeps every v1 shared authored field and adds an explicit
//! `source_components` table plus target `role`, `source_components` and
//! `depends_on` references. It is decoded by its own reader; there is no
//! implicit v1/v2 conversion and the v1 schema, reader and oracles stay frozen.

#![allow(missing_docs)]

use super::{ApplicabilityDto, ExtensionsDto};
use schemars::JsonSchema;
use serde::Deserialize;
use std::collections::BTreeMap;

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
    pub schema: String,
    pub id: String,
    pub configuration_digest: String,
    pub owners: Vec<String>,
    #[serde(default)]
    pub extensions: ExtensionsDto,
}
