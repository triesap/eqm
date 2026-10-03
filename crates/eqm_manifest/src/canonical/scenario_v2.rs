//! Separate v2 scenario-occurrence projection and `ScenarioDigest`.
//!
//! DEV-009/DEV-012 allocate `eqm:v2:scenario` over exactly `id`, `revision`,
//! `participants`, `dependencies` and `extensions`. Participants are a canonical
//! full-member set (occurrence `id`, explicit `role`, tagged target/provider
//! `subject`), so repeated actors on one shipped target remain distinct;
//! dependencies are a canonical set of explicit directed `from`/`to` occurrence
//! relations. Extensions use the recursive type-tagged v2 forms. This projection
//! contains no build, source or artifact assignment, so it is shared comparison
//! context rather than exact subject identity.

use super::support::{canonical_set, extensions_v2, wire_uint};
use super::{CanonicalizationError, MAX_PROJECTION_BYTES, object};
use eqm_domain::{Scenario, ScenarioDigest, ScenarioParticipant, ScenarioSubject};
use serde_json::{Value, json};

/// Immutable canonical v2 scenario bytes and digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalScenarioV2 {
    bytes: Vec<u8>,
    digest: ScenarioDigest,
}

impl CanonicalScenarioV2 {
    /// Returns the exact one-line JCS bytes without a trailing newline.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the domain-separated scenario identity.
    #[must_use]
    pub const fn digest(&self) -> ScenarioDigest {
        self.digest
    }
}

fn participant_v2(value: &ScenarioParticipant) -> Result<Value, CanonicalizationError> {
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

/// Projects one scenario into its exact v2 semantic shape.
pub fn scenario_v2(value: &Scenario) -> Result<Value, CanonicalizationError> {
    Ok(object([
        ("id", json!(value.id().as_str())),
        ("revision", wire_uint(value.revision().get())),
        (
            "participants",
            canonical_set(
                value
                    .participants()
                    .iter()
                    .map(participant_v2)
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

/// Canonicalizes one scenario under the `eqm:v2:scenario` domain.
pub fn canonicalize_scenario_v2(
    value: &Scenario,
) -> Result<CanonicalScenarioV2, CanonicalizationError> {
    let projected = scenario_v2(value)?;
    let bytes = serde_json_canonicalizer::to_vec(&projected)
        .map_err(|_| CanonicalizationError::Serialization)?;
    if bytes.len() > MAX_PROJECTION_BYTES {
        return Err(CanonicalizationError::ProjectionTooLarge);
    }
    let digest = ScenarioDigest::hash_content(&bytes);
    Ok(CanonicalScenarioV2 { bytes, digest })
}
