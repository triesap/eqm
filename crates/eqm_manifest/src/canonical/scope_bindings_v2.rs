//! Separate canonical identity for explicit requirement-to-scenario bindings.

use super::support::{canonical_set, wire_uint};
use super::{CanonicalizationError, MAX_PROJECTION_BYTES, object};
use eqm_domain::{RequirementScenarioBindings, RequirementScenarioBindingsDigest};
use serde_json::{Value, json};

/// Immutable v2 scope-binding bytes and their distinct domain-separated digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalScopeBindingsV2 {
    bytes: Vec<u8>,
    digest: RequirementScenarioBindingsDigest,
}

impl CanonicalScopeBindingsV2 {
    /// Returns exact JCS bytes without a trailing newline.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the separate identity; it grants no authority.
    #[must_use]
    pub const fn digest(&self) -> RequirementScenarioBindingsDigest {
        self.digest
    }
}

/// Projects exactly the approved bindings root, using v2 numeric/set rules.
pub fn scope_bindings_v2(
    value: &RequirementScenarioBindings,
) -> Result<Value, CanonicalizationError> {
    let members: Vec<Value> = value
        .bindings()
        .iter()
        .map(|(requirement, scenario)| {
            object([
                ("requirement", json!(requirement.as_str())),
                (
                    "scenario",
                    object([
                        ("id", json!(scenario.id().as_str())),
                        ("revision", wire_uint(scenario.revision().get())),
                    ]),
                ),
            ])
        })
        .collect();
    Ok(object([("bindings", canonical_set(members)?)]))
}

/// Canonicalizes only the explicit map; graph membership is checked separately.
pub fn canonicalize_scope_bindings_v2(
    value: &RequirementScenarioBindings,
) -> Result<CanonicalScopeBindingsV2, CanonicalizationError> {
    let bytes = serde_json_canonicalizer::to_vec(&scope_bindings_v2(value)?)
        .map_err(|_| CanonicalizationError::Serialization)?;
    if bytes.len() > MAX_PROJECTION_BYTES {
        return Err(CanonicalizationError::ProjectionTooLarge);
    }
    let digest = RequirementScenarioBindingsDigest::hash_content(&bytes);
    Ok(CanonicalScopeBindingsV2 { bytes, digest })
}
