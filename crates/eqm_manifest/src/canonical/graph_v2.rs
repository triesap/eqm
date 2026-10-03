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
use super::support::{canonical_set, canonical_strings, extensions_v2, wire_uint};
use super::target_v2::target_configuration_v2;
use super::{
    CanonicalizationError, MAX_PROJECTION_BYTES, binding_v2::binding_v2, insert_optional, object,
};
use eqm_domain::{
    AdapterLockIdentity, ArgumentTemplate, EnvironmentSource, EvidenceScopeSubject,
    FinalizedWorkspaceGraph, ImportLockIdentity, RunnerDefinition, RunnerProgram, Scenario,
    ScenarioParticipant, ScenarioSubject, SchemaKind, SchemaUri, SchemaVersion,
    SemanticGraphV2Digest, SourceComponent, Waiver, WorkingDirectoryTemplate,
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

fn scenario_participant_v2(value: &ScenarioParticipant) -> Result<Value, CanonicalizationError> {
    let subject = match value.subject() {
        ScenarioSubject::Target(target) => object([
            ("kind", json!("target")),
            ("target", json!(target.as_str())),
        ]),
        ScenarioSubject::Provider(provider) => object([
            ("kind", json!("provider")),
            ("provider", json!(provider.as_str())),
        ]),
    };
    Ok(object([
        ("id", json!(value.occurrence().as_str())),
        ("role", json!(value.role().as_str())),
        ("subject", subject),
    ]))
}

fn scenario_v2(value: &Scenario) -> Result<Value, CanonicalizationError> {
    Ok(object([
        ("id", json!(value.id().as_str())),
        ("revision", wire_uint(value.revision().get())),
        (
            "participants",
            canonical_set(
                value
                    .participants()
                    .iter()
                    .map(scenario_participant_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        (
            "dependencies",
            canonical_set(value.dependencies().iter().map(|dependency| {
                object([
                    ("from", json!(dependency.from().as_str())),
                    ("to", json!(dependency.to().as_str())),
                ])
            }))?,
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

fn argument_v2(value: &ArgumentTemplate) -> Value {
    json!(match value {
        ArgumentTemplate::Literal(value) => value.as_str(),
        ArgumentTemplate::TargetRoot => "{target_root}",
        ArgumentTemplate::SelectorJson => "{selector_json}",
        ArgumentTemplate::ResultPath => "{result_path}",
    })
}

fn working_directory_v2(value: &WorkingDirectoryTemplate) -> Value {
    json!(match value {
        WorkingDirectoryTemplate::TargetRoot => "{target_root}",
        WorkingDirectoryTemplate::Repository(path) => path.as_str(),
        WorkingDirectoryTemplate::ResultPath => "{result_path}",
    })
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
        EvidenceScopeSubject::Target(target) => json!(target.as_str()),
        EvidenceScopeSubject::Provider(provider) => json!(provider.as_str()),
        EvidenceScopeSubject::TargetSet(targets) => {
            canonical_strings(targets.iter().map(|target| target.as_str().to_owned()))?
        }
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
