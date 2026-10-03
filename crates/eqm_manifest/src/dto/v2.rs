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
