//! Closed complete pre-evidence obligation views. Parsing identifies claims;
//! neither a pending coordinate nor a retained reason is evidence or trust.
#![allow(missing_docs)]

use crate::schema_graph_v2::*;
use crate::v2::{CanonicalInstant, deserialize_present};
use crate::{CanonicalSet, ToolVersionDto, WireUint};
use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;

mod validation;
pub use validation::ObligationQueryError;

pub const MAX_OBLIGATION_QUERY_BYTES: usize = 16 * 1024 * 1024;
pub const MAX_OBLIGATION_QUERY_RECORDS: usize = 100_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QuerySchemaV2;
impl Serialize for QuerySchemaV2 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&crate::RESULT_SCHEMA_V2.to_string())
    }
}
impl<'de> Deserialize<'de> for QuerySchemaV2 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if String::deserialize(deserializer)? == crate::RESULT_SCHEMA_V2.to_string() {
            Ok(Self)
        } else {
            Err(serde::de::Error::custom(
                "unsupported obligation result schema",
            ))
        }
    }
}
impl JsonSchema for QuerySchemaV2 {
    fn schema_name() -> Cow<'static, str> {
        "QuerySchemaV2".into()
    }
    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({"type":"string", "const":crate::RESULT_SCHEMA_V2.to_string()})
    }
}

#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ObligationQueryCommand {
    Obligations,
}

#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryModeV2 {
    Development,
    PullRequest,
    Release,
}
#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryOriginV2 {
    CandidateLocal,
    TrustedInvocation,
    ProtectedBaseline,
}
#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ObligationQueryState {
    Pending,
    Unknown,
    NotApplicable,
}
#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ObligationQueryReason {
    NotApplicable,
    UnknownApplicability,
    UnknownParticipation,
    MissingScopeAuthority,
    UnmatchedPolicy,
}
#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ObligationQueryErrorCode {
    InvalidRequest,
    InvalidAuthority,
    UnavailableWorkspace,
    ResourceLimit,
    InternalFailure,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QueryPolicyReferenceV2 {
    #[schemars(with = "PolicyIdText")]
    pub id: String,
    #[schemars(schema_with = "crate::positive_unsigned_schema")]
    pub revision: WireUint,
}
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QueryProfileV2 {
    #[schemars(with = "ProfileIdText")]
    pub id: String,
    #[schemars(schema_with = "crate::positive_unsigned_schema")]
    pub revision: WireUint,
    #[schemars(with = "NonEmptySet<DimensionValueV2Dto>")]
    pub values: CanonicalSet<DimensionValueV2Dto>,
}
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObligationQueryContextV2 {
    #[schemars(with = "Sha256Text")]
    pub semantic_graph_digest: String,
    #[schemars(with = "Sha256Text")]
    pub scope_bindings_digest: String,
    pub policy: QueryPolicyReferenceV2,
    #[schemars(with = "NonEmptySet<QueryProfileV2>")]
    pub profiles: CanonicalSet<QueryProfileV2>,
    pub mode: QueryModeV2,
    pub authority_origin: QueryOriginV2,
    #[serde(
        default,
        deserialize_with = "deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "Sha256Text")]
    pub release_context: Option<String>,
    pub offline: bool,
    pub evaluated_at: CanonicalInstant,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QueryTargetConfigurationV2 {
    #[schemars(with = "TargetIdText")]
    pub target: String,
    #[schemars(with = "Sha256Text")]
    pub configuration_digest: String,
}
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QueryTargetDependencyV2 {
    #[schemars(with = "TargetIdText")]
    pub from: String,
    #[schemars(with = "TargetIdText")]
    pub to: String,
}
#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryParticipationStateV2 {
    True,
    False,
    Unknown,
}
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QueryParticipationV2 {
    #[schemars(with = "TargetIdText")]
    pub target: String,
    pub state: QueryParticipationStateV2,
}
#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryProviderKindV2 {
    Provider,
}
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QueryProviderScopeV2 {
    pub kind: QueryProviderKindV2,
    #[schemars(with = "ProviderIdText")]
    pub provider: String,
    #[schemars(with = "Sha256Text")]
    pub configuration_digest: String,
    pub dependers: CanonicalSet<QueryTargetConfigurationV2>,
    pub assignments: CanonicalSet<QueryTargetConfigurationV2>,
    pub dependencies: CanonicalSet<QueryTargetDependencyV2>,
    pub dependency_configurations: CanonicalSet<QueryTargetConfigurationV2>,
    pub participation: CanonicalSet<QueryParticipationV2>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ObligationQuerySubjectV2 {
    Target {
        #[schemars(with = "TargetIdText")]
        target: String,
        #[schemars(with = "Sha256Text")]
        configuration_digest: String,
    },
    Provider {
        #[schemars(with = "ProviderIdText")]
        provider: String,
        #[schemars(with = "Sha256Text")]
        configuration_digest: String,
        dependers: CanonicalSet<QueryTargetConfigurationV2>,
        assignments: CanonicalSet<QueryTargetConfigurationV2>,
        dependencies: CanonicalSet<QueryTargetDependencyV2>,
        dependency_configurations: CanonicalSet<QueryTargetConfigurationV2>,
        participation: CanonicalSet<QueryParticipationV2>,
    },
    Scenario {
        scenario: ScenarioV2Dto,
        #[schemars(with = "Sha256Text")]
        scenario_digest: String,
        selected_targets: CanonicalSet<QueryTargetConfigurationV2>,
        providers: CanonicalSet<QueryProviderScopeV2>,
        participation: CanonicalSet<QueryParticipationV2>,
    },
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QueryPolicySelectionV2 {
    pub policy: PolicyV2Dto,
    #[schemars(with = "NonEmptySet<SelectedProfileV2Dto>")]
    pub profiles: CanonicalSet<SelectedProfileV2Dto>,
}
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "state", rename_all = "snake_case", deny_unknown_fields)]
pub enum QueryExpectedSelectionsV2 {
    Missing,
    Present {
        #[schemars(with = "CanonicalSet<Sha256Text>")]
        digests: CanonicalSet<String>,
    },
}
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QueryObligationStrengthV2 {
    #[schemars(with = "RequirementLevelText")]
    pub minimum_level: String,
    #[schemars(with = "TrustLevelText")]
    pub minimum_trust: String,
    #[schemars(schema_with = "duration_millis_schema")]
    pub maximum_age_ms: WireUint,
    #[schemars(schema_with = "crate::positive_unsigned_schema")]
    pub minimum_count: WireUint,
}
pub(super) fn duration_millis_schema(_: &mut SchemaGenerator) -> Schema {
    crate::bounded_unsigned_schema(0, i64::MAX as u64)
}
/// Typed exact existing manifest coordinate projection; its bytes retain the
/// frozen obligation digest domain, rather than defining a second identity.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObligationCoordinateV2Dto {
    #[schemars(with = "UnitIdText")]
    pub unit: String,
    #[schemars(with = "RequirementText")]
    pub requirement: String,
    #[schemars(with = "FacetText")]
    pub facet: String,
    pub policy_selection: QueryPolicySelectionV2,
    #[schemars(with = "Sha256Text")]
    pub semantic_graph_digest: String,
    #[schemars(with = "Sha256Text")]
    pub contract_digest: String,
    #[schemars(with = "Sha256Text")]
    pub scope_bindings_digest: String,
    #[schemars(with = "CanonicalSet<Sha256Text>")]
    pub participation_digests: CanonicalSet<String>,
    pub subject: ObligationQuerySubjectV2,
    pub expected_selections: QueryExpectedSelectionsV2,
    pub strength: QueryObligationStrengthV2,
    pub mode: QueryModeV2,
    pub authority_origin: QueryOriginV2,
    #[serde(
        default,
        deserialize_with = "deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "Sha256Text")]
    pub release_context: Option<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(extend("allOf" = [
    {"if":{"properties":{"kind":{"const":"required"}},"required":["kind"]},"then":{"properties":{"state":{"const":"pending"}}}},
    {"if":{"properties":{"kind":{"const":"unresolved"},"reason":{"const":"not_applicable"}},"required":["kind","reason"]},"then":{"properties":{"state":{"const":"not_applicable"}}}},
    {"if":{"properties":{"kind":{"const":"unresolved"},"reason":{"enum":["unknown_applicability","unknown_participation","missing_scope_authority","unmatched_policy"]}},"required":["kind","reason"]},"then":{"properties":{"state":{"const":"unknown"}}}}
]))]
pub enum ObligationViewV2Dto {
    Required {
        state: ObligationQueryState,
        #[schemars(regex(pattern = "^eqm:v2:obligation:sha256:[0-9a-f]{64}$"))]
        reference: String,
        coordinate: Box<ObligationCoordinateV2Dto>,
    },
    Unresolved {
        state: ObligationQueryState,
        reason: ObligationQueryReason,
        #[schemars(with = "UnitIdText")]
        unit: String,
        #[schemars(with = "RequirementText")]
        requirement: String,
        #[schemars(with = "NonEmptySet<QueryProfileV2>")]
        profiles: CanonicalSet<QueryProfileV2>,
        #[serde(
            default,
            deserialize_with = "deserialize_present",
            skip_serializing_if = "Option::is_none"
        )]
        #[schemars(with = "ObligationQuerySubjectV2")]
        subject: Option<Box<ObligationQuerySubjectV2>>,
        #[serde(
            default,
            deserialize_with = "deserialize_present",
            skip_serializing_if = "Option::is_none"
        )]
        #[schemars(with = "Sha256Text")]
        release_context: Option<String>,
    },
}
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObligationQueryFiltersV2 {
    #[serde(
        default,
        deserialize_with = "deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "UnitIdText")]
    pub unit: Option<String>,
    #[serde(
        default,
        deserialize_with = "deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "TargetIdText")]
    pub target: Option<String>,
    pub statuses: CanonicalSet<ObligationQueryState>,
}
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObligationQueryDenominatorV2 {
    #[schemars(schema_with = "query_count_schema")]
    pub required: WireUint,
    #[schemars(schema_with = "query_count_schema")]
    pub unresolved: WireUint,
}
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObligationsResultV2Dto {
    pub filters: ObligationQueryFiltersV2,
    pub denominator: ObligationQueryDenominatorV2,
    #[schemars(schema_with = "query_count_schema")]
    pub returned_count: WireUint,
    pub presentation_filtered: bool,
    #[schemars(length(max = 100_000))]
    pub records: CanonicalSet<ObligationViewV2Dto>,
}
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObligationQueryFailureV2 {
    pub code: ObligationQueryErrorCode,
    #[schemars(length(min = 1, max = 4096))]
    pub message: String,
    #[serde(
        default,
        deserialize_with = "deserialize_present",
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(with = "RepoPathText")]
    pub source: Option<String>,
}
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case", deny_unknown_fields)]
pub enum ObligationsReportV2Dto {
    Success {
        schema: QuerySchemaV2,
        #[schemars(schema_with = "tool_version_schema")]
        tool_version: ToolVersionDto,
        command: ObligationQueryCommand,
        context: Box<ObligationQueryContextV2>,
        result: ObligationsResultV2Dto,
    },
    Error {
        schema: QuerySchemaV2,
        #[schemars(schema_with = "tool_version_schema")]
        tool_version: ToolVersionDto,
        command: ObligationQueryCommand,
        error: ObligationQueryFailureV2,
    },
}

fn tool_version_schema(_: &mut SchemaGenerator) -> Schema {
    json_schema!({"type":"string","const":eqm_domain::ToolVersion::CURRENT.as_str()})
}

fn query_count_schema(_: &mut SchemaGenerator) -> Schema {
    crate::bounded_unsigned_schema(0, MAX_OBLIGATION_QUERY_RECORDS as u64)
}
