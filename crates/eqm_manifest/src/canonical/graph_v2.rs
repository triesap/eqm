//! Complete v2 semantic-graph projection and `SemanticGraphV2Digest` identity.
//!
//! DEV-012 P03-R1 replaces the unconditional v1 [`super::canonicalize_graph`]
//! preparation path with an explicit, complete v2 graph identity. The v2 root
//! keeps the existing graph collection names and adds the modeled
//! `source_components` and `participation` collections introduced by the v2
//! workspace and participation families. Every collection is projected with the
//! approved v2 forms:
//!
//! - revisions, counts, runner limits and extension integers use the v2 wire
//!   forms from [`super::support`];
//! - the v2 component projections (`contract`, `target`, `binding`,
//!   `policy/profile`) are reused where semantically complete rather than the
//!   lossy v1 shapes;
//! - canonical sets are ordered by complete member bytes and reject exact
//!   duplicates; ordered extension arrays keep their order;
//! - only the `dev.equivalencematrix.display` extension namespace is excluded.
//!
//! The v1 projection in [`super::canonicalize_graph`] explicitly rejects a graph
//! carrying any v2-only metadata/authority instead of silently omitting it. This
//! projection is the graph-identity authority only: it implies no trust,
//! selected policy/profile, source acquisition or build verification.

use super::contract_v2::{capability_v2, fragment_v2, journey_v2, surface_v2};
use super::participation_v2::participation_v2;
use super::policy_v2::{policy_v2, profile_definition_v2};
use super::scenario_v2::scenario_v2;
use super::support::{canonical_set, canonical_strings, extensions_v2, wire_uint};
use super::target_v2::target_configuration_v2;
use super::{
    CanonicalizationError, MAX_PROJECTION_BYTES, binding_v2::binding_v2, insert_optional, object,
};
use eqm_domain::{
    AdapterLockIdentity, ArgumentTemplate, Cohort, CohortAssignment, EnvironmentSource,
    EvidenceScopeSubject, FinalizedWorkspaceGraph, ImportLockIdentity, Provider,
    ProviderConfigurationDigest, RunnerDefinition, RunnerProgram, SchemaKind, SchemaUri,
    SchemaVersion, SemanticGraphV2Digest, SourceComponent, Waiver, WorkingDirectoryTemplate,
};
use serde_json::{Value, json};

/// Immutable canonical v2 semantic-graph bytes and digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalGraphV2 {
    bytes: Vec<u8>,
    digest: SemanticGraphV2Digest,
}

impl CanonicalGraphV2 {
    /// Returns the exact one-line JCS bytes without a trailing newline.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the domain-separated complete v2 graph identity.
    #[must_use]
    pub const fn digest(&self) -> SemanticGraphV2Digest {
        self.digest
    }
}

/// The exact v2 semantic-graph schema marker.
#[must_use]
pub fn semantic_graph_v2_schema_uri() -> String {
    SchemaUri::versioned(SchemaKind::SemanticGraph, SchemaVersion::V2).to_string()
}

fn source_component_v2(value: &SourceComponent) -> Result<Value, CanonicalizationError> {
    Ok(object([
        ("id", json!(value.id().as_str())),
        (
            "roots",
            canonical_strings(value.roots().iter().map(|root| root.as_str().to_owned()))?,
        ),
        (
            "owners",
            canonical_strings(value.owners().iter().map(|owner| owner.as_str().to_owned()))?,
        ),
        (
            "dependencies",
            canonical_strings(
                value
                    .dependencies()
                    .iter()
                    .map(|dependency| dependency.as_str().to_owned()),
            )?,
        ),
        ("extensions", extensions_v2(value.extensions())?),
    ]))
}

fn provider_v2(value: &Provider) -> Result<Value, CanonicalizationError> {
    Ok(object([
        ("id", json!(value.id().as_str())),
        (
            "configuration_digest",
            json!(value.configuration_digest().to_string()),
        ),
        (
            "owners",
            canonical_strings(value.owners().iter().map(|owner| owner.as_str().to_owned()))?,
        ),
        ("extensions", extensions_v2(value.extensions())?),
    ]))
}

/// Immutable canonical v2 provider-configuration bytes and digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalProviderConfigurationV2 {
    bytes: Vec<u8>,
    digest: ProviderConfigurationDigest,
}

impl CanonicalProviderConfigurationV2 {
    /// Returns the exact one-line JCS bytes without a trailing newline.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the domain-separated provider-configuration identity.
    #[must_use]
    pub const fn digest(&self) -> ProviderConfigurationDigest {
        self.digest
    }
}

/// Canonicalizes one provider under `eqm:v2:provider-configuration`.
///
/// This identity is derived over the whole validated provider definition; the
/// authored `configuration_digest` member remains a distinct external content
/// reference.
pub fn canonicalize_provider_configuration_v2(
    value: &Provider,
) -> Result<CanonicalProviderConfigurationV2, CanonicalizationError> {
    let projected = provider_v2(value)?;
    let bytes = serde_json_canonicalizer::to_vec(&projected)
        .map_err(|_| CanonicalizationError::Serialization)?;
    if bytes.len() > MAX_PROJECTION_BYTES {
        return Err(CanonicalizationError::ProjectionTooLarge);
    }
    let digest = ProviderConfigurationDigest::hash_content(&bytes);
    Ok(CanonicalProviderConfigurationV2 { bytes, digest })
}

fn cohort_assignment_v2(value: &CohortAssignment) -> Result<Value, CanonicalizationError> {
    let mut projected = object([
        ("occurrence", json!(value.occurrence().as_str())),
        ("repository", json!(value.repository().as_str())),
        ("source_commit", json!(value.source_commit().as_str())),
    ]);
    insert_optional(
        &mut projected,
        "source_content_digest",
        value.source_content().map(|item| json!(item.to_string())),
    );
    insert_optional(
        &mut projected,
        "build_id",
        value.build_id().map(|item| json!(item.as_str())),
    );
    insert_optional(
        &mut projected,
        "artifact_digest",
        value.artifact_digest().map(|item| json!(item.to_string())),
    );
    Ok(projected)
}

fn cohort_v2(value: &Cohort) -> Result<Value, CanonicalizationError> {
    Ok(object([
        ("id", json!(value.id().as_str())),
        ("revision", wire_uint(value.revision().get())),
        ("scenario", json!(value.scenario().as_str())),
        (
            "assignments",
            canonical_set(
                value
                    .assignments()
                    .iter()
                    .map(cohort_assignment_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        ("extensions", extensions_v2(value.extensions())?),
    ]))
}

fn runner_v2(value: &RunnerDefinition) -> Result<Value, CanonicalizationError> {
    let limits = value.limits();
    Ok(object([
        ("id", json!(value.id().as_str())),
        ("revision", wire_uint(value.revision().get())),
        (
            "owners",
            canonical_strings(value.owners().iter().map(|owner| owner.as_str().to_owned()))?,
        ),
        ("backend", json!(value.backend().as_str())),
        ("program", runner_program_v2(value.program())),
        (
            "args",
            Value::Array(value.args().iter().map(argument_v2).collect()),
        ),
        ("cwd", working_directory_v2(value.cwd())),
        (
            "environment",
            canonical_set(
                value
                    .environment()
                    .values()
                    .map(environment_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        (
            "secrets",
            canonical_set(value.secrets().values().map(|binding| {
                object([
                    ("name", json!(binding.name().as_str())),
                    ("provider", json!(binding.provider().as_str())),
                ])
            }))?,
        ),
        ("timeout_ms", wire_uint(limits.timeout().get())),
        (
            "max_output_bytes",
            wire_uint(limits.max_output_bytes().get()),
        ),
        ("max_concurrency", wire_uint(limits.max_concurrency().get())),
        (
            "guarantees",
            canonical_strings(
                value
                    .guarantees()
                    .iter()
                    .map(|item| item.as_str().to_owned()),
            )?,
        ),
        ("extensions", extensions_v2(value.extensions())?),
    ]))
}

fn runner_program_v2(value: &RunnerProgram) -> Value {
    match value {
        RunnerProgram::Repository(path) => object([
            ("kind", json!("repository")),
            ("path", json!(path.as_str())),
        ]),
        RunnerProgram::Locked { resolved, digest } => object([
            ("kind", json!("locked")),
            ("resolved", json!(resolved.as_str())),
            ("digest", json!(digest.to_string())),
        ]),
    }
}

/// Projects one complete argument template as an explicit tagged record. A
/// literal `{target_root}` text and the substitution template are distinct
/// graph identity members, matching the approved runner v2 representation.
fn argument_v2(value: &ArgumentTemplate) -> Value {
    match value {
        ArgumentTemplate::Literal(text) => {
            object([("kind", json!("literal")), ("value", json!(text.as_str()))])
        }
        ArgumentTemplate::TargetRoot => object([("kind", json!("target_root"))]),
        ArgumentTemplate::SelectorJson => object([("kind", json!("selector_json"))]),
        ArgumentTemplate::ResultPath => object([("kind", json!("result_path"))]),
    }
}

/// Projects one complete working-directory template as a tagged record. A
/// repository path that merely looks like a placeholder stays distinct from an
/// expanded substitution template.
fn working_directory_v2(value: &WorkingDirectoryTemplate) -> Value {
    match value {
        WorkingDirectoryTemplate::TargetRoot => object([("kind", json!("target_root"))]),
        WorkingDirectoryTemplate::Repository(path) => object([
            ("kind", json!("repository")),
            ("path", json!(path.as_str())),
        ]),
        WorkingDirectoryTemplate::ResultPath => object([("kind", json!("result_path"))]),
    }
}

fn environment_v2(
    binding: &eqm_domain::EnvironmentBinding,
) -> Result<Value, CanonicalizationError> {
    let (source, literal) = match binding.source() {
        EnvironmentSource::Literal(value) => ("literal", Some(json!(value.as_str()))),
        EnvironmentSource::TrustedPath => ("trusted_path", None),
        EnvironmentSource::CanonicalLocale => ("canonical_locale", None),
        EnvironmentSource::UtcTimezone => ("utc_timezone", None),
    };
    let mut projected = object([
        ("name", json!(binding.name().as_str())),
        ("source", json!(source)),
    ]);
    insert_optional(&mut projected, "value", literal);
    Ok(projected)
}

fn waiver_v2(value: &Waiver) -> Result<Value, CanonicalizationError> {
    Ok(object([
        ("id", json!(value.id().as_str())),
        ("revision", wire_uint(value.revision().get())),
        (
            "owners",
            canonical_strings(value.owners().iter().map(|owner| owner.as_str().to_owned()))?,
        ),
        ("policy", json!(value.policy().as_str())),
        ("scope", waiver_scope_v2(value.scope())?),
        ("reason", json!(value.reason().as_str())),
        ("issue", json!(value.issue().as_str())),
        (
            "approvers",
            canonical_strings(
                value
                    .approvers()
                    .iter()
                    .map(|approver| approver.as_str().to_owned()),
            )?,
        ),
        ("starts_on", json!(value.starts_on().to_string())),
        ("expires_on", json!(value.expires_on().to_string())),
        (
            "controls",
            canonical_strings(value.controls().iter().map(|item| item.as_str().to_owned()))?,
        ),
        ("extensions", extensions_v2(value.extensions())?),
    ]))
}

fn waiver_scope_v2(value: &eqm_domain::WaiverScope) -> Result<Value, CanonicalizationError> {
    let target = match value.target() {
        EvidenceScopeSubject::Target(target) => object([
            ("kind", json!("target")),
            ("target", json!(target.as_str())),
        ]),
        EvidenceScopeSubject::Provider(provider) => object([
            ("kind", json!("provider")),
            ("provider", json!(provider.as_str())),
        ]),
        EvidenceScopeSubject::TargetSet(targets) => object([
            ("kind", json!("target_set")),
            (
                "targets",
                canonical_strings(targets.iter().map(|target| target.as_str().to_owned()))?,
            ),
        ]),
    };
    Ok(object([
        ("target", target),
        ("unit", json!(value.unit().as_str())),
        ("requirement", json!(value.requirement().as_str())),
        (
            "facets",
            canonical_strings(value.facets().iter().map(|facet| facet.as_str().to_owned()))?,
        ),
        (
            "profiles",
            Value::Object(
                value
                    .profiles()
                    .iter()
                    .map(|(profile, scope)| {
                        (
                            profile.as_str().to_owned(),
                            Value::Object(
                                scope
                                    .values()
                                    .iter()
                                    .map(|(dimension, item)| {
                                        (dimension.as_str().to_owned(), json!(item.as_str()))
                                    })
                                    .collect(),
                            ),
                        )
                    })
                    .collect(),
            ),
        ),
    ]))
}

fn import_lock_v2(value: &ImportLockIdentity) -> Result<Value, CanonicalizationError> {
    let mut projected = object([
        ("id", json!(value.id.as_str())),
        ("revision", wire_uint(value.revision.get())),
        ("source", json!(value.source.as_str())),
        ("resolved", json!(value.resolved.as_str())),
        ("digest", json!(value.digest.to_string())),
        ("trust", json!(value.trust.as_str())),
    ]);
    insert_optional(
        &mut projected,
        "signature",
        value.signature.as_ref().map(|v| json!(v.as_str())),
    );
    Ok(projected)
}

fn adapter_lock_v2(value: &AdapterLockIdentity) -> Result<Value, CanonicalizationError> {
    let mut projected = object([
        ("id", json!(value.id.as_str())),
        ("version", json!(value.version.as_str())),
        ("source", json!(value.source.as_str())),
        ("resolved", json!(value.resolved.as_str())),
        ("digest", json!(value.digest.to_string())),
        ("protocol", wire_uint(value.protocol.get())),
        ("trust", json!(value.trust.as_str())),
    ]);
    insert_optional(
        &mut projected,
        "signature",
        value.signature.as_ref().map(|v| json!(v.as_str())),
    );
    Ok(projected)
}

/// Projects one finalized graph into the complete v2 semantic-graph shape.
pub fn graph_v2(finalized: &FinalizedWorkspaceGraph) -> Result<Value, CanonicalizationError> {
    let graph = finalized.graph();
    Ok(object([
        ("schema", json!(semantic_graph_v2_schema_uri())),
        (
            "capabilities",
            canonical_set(
                graph
                    .capabilities()
                    .values()
                    .map(capability_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        (
            "journeys",
            canonical_set(
                graph
                    .journeys()
                    .values()
                    .map(journey_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        (
            "surfaces",
            canonical_set(
                graph
                    .surfaces()
                    .values()
                    .map(surface_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        (
            "fragments",
            canonical_set(
                graph
                    .fragments()
                    .values()
                    .map(fragment_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        (
            "targets",
            canonical_set(
                graph
                    .targets()
                    .values()
                    .map(target_configuration_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        (
            "source_components",
            canonical_set(
                graph
                    .source_components()
                    .values()
                    .map(source_component_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        (
            "providers",
            canonical_set(
                graph
                    .providers()
                    .values()
                    .map(provider_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        (
            "cohorts",
            canonical_set(
                graph
                    .cohorts()
                    .values()
                    .map(cohort_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        (
            "participation",
            canonical_set(
                graph
                    .participation()
                    .values()
                    .map(participation_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        (
            "bindings",
            canonical_set(
                graph
                    .bindings()
                    .values()
                    .map(binding_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        (
            "scenarios",
            canonical_set(
                graph
                    .scenarios()
                    .values()
                    .map(scenario_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        (
            "policies",
            canonical_set(
                graph
                    .policies()
                    .values()
                    .map(policy_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        (
            "profiles",
            canonical_set(
                graph
                    .profiles()
                    .values()
                    .map(profile_definition_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        (
            "runners",
            canonical_set(
                graph
                    .runners()
                    .values()
                    .map(runner_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        (
            "waivers",
            canonical_set(
                graph
                    .waivers()
                    .values()
                    .map(waiver_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        (
            "imports",
            canonical_set(
                graph
                    .imports()
                    .values()
                    .map(import_lock_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        (
            "adapters",
            canonical_set(
                graph
                    .adapter_locks()
                    .values()
                    .map(adapter_lock_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        ("extensions", extensions_v2(graph.extensions())?),
    ]))
}

/// Canonicalizes one finalized graph under the complete v2 graph domain.
pub fn canonicalize_graph_v2(
    finalized: &FinalizedWorkspaceGraph,
) -> Result<CanonicalGraphV2, CanonicalizationError> {
    let projected = graph_v2(finalized)?;
    let bytes = serde_json_canonicalizer::to_vec(&projected)
        .map_err(|_| CanonicalizationError::Serialization)?;
    if bytes.len() > MAX_PROJECTION_BYTES {
        return Err(CanonicalizationError::ProjectionTooLarge);
    }
    let digest = SemanticGraphV2Digest::hash_content(&bytes);
    Ok(CanonicalGraphV2 { bytes, digest })
}
