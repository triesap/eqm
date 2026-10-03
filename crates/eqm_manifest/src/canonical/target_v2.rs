//! Separate canonical v2 target-configuration projection.
//!
//! DEV-007 chooses an exact **target-definition component identity**, separate
//! from selected-policy, source snapshot, build artifact and execution
//! identities. At this step [`Target`] contains exactly `id`, `root`,
//! `platform`, `framework`, optional `role`, `owners` and `extensions`. When a
//! target models them it also carries canonical `source_components` and
//! `depends_on` ID sets. The internal JSON root therefore has those members,
//! with `role` and the two ID sets inserted only when present, without a schema
//! URI or a nested `target` wrapper. An absent role or empty set is omitted; it
//! is not a `null`, an invented default, a successful verification or a `"none"`
//! authority.
//!
//! Strings retain their validated exact values. `root` remains the authored
//! [`RepoPath`](eqm_domain::RepoPath): no absolute expansion, filesystem
//! canonicalization or case folding. Owners are a complete member-byte
//! canonical set. Extensions use the recursive DEV-006 type tags and exclude
//! only `dev.equivalencematrix.display`. The projection is pure and enforces the
//! existing projection byte bound.
//!
//! The resulting [`TargetConfigurationDigest`] is a target-definition identity
//! only. It does not establish a selected policy/profile, a source revision,
//! resolved source-component content or a build artifact; those remain separate
//! subject axes with their own identities in later steps. Consumers must not use
//! it as a full build or trusted-subject identity.

use super::support::{canonical_strings, extensions_v2};
use super::{CanonicalizationError, MAX_PROJECTION_BYTES, insert_optional, object};
use eqm_domain::{Target, TargetConfigurationDigest};
use serde_json::{Value, json};

/// Immutable canonical v2 target-configuration bytes and digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalTargetConfigurationV2 {
    bytes: Vec<u8>,
    digest: TargetConfigurationDigest,
}

impl CanonicalTargetConfigurationV2 {
    /// Returns the exact one-line JCS bytes without a trailing newline.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the domain-separated target-configuration identity.
    #[must_use]
    pub const fn digest(&self) -> TargetConfigurationDigest {
        self.digest
    }
}

/// Projects one target into its exact v2 target-configuration semantic shape.
///
/// The root has `id`, `root`, `platform`, `framework`, `owners` and
/// `extensions`, plus `role` only when the target declares one. There are no
/// build/profile/source-component fields to project at this step.
pub fn target_configuration_v2(value: &Target) -> Result<Value, CanonicalizationError> {
    let mut projected = object([
        ("id", json!(value.id().as_str())),
        ("root", json!(value.root().as_str())),
        ("platform", json!(value.platform().as_str())),
        ("framework", json!(value.framework().as_str())),
        (
            "owners",
            canonical_strings(value.owners().iter().map(|owner| owner.as_str().to_owned()))?,
        ),
        ("extensions", extensions_v2(value.extensions())?),
    ]);
    insert_optional(
        &mut projected,
        "role",
        value.role().map(|role| json!(role.as_str())),
    );
    if !value.source_components().is_empty() {
        insert_optional(
            &mut projected,
            "source_components",
            Some(canonical_strings(
                value
                    .source_components()
                    .iter()
                    .map(|component| component.as_str().to_owned()),
            )?),
        );
    }
    if !value.depends_on().is_empty() {
        insert_optional(
            &mut projected,
            "depends_on",
            Some(canonical_strings(
                value
                    .depends_on()
                    .iter()
                    .map(|target| target.as_str().to_owned()),
            )?),
        );
    }
    if !value.providers().is_empty() {
        insert_optional(
            &mut projected,
            "providers",
            Some(canonical_strings(
                value
                    .providers()
                    .iter()
                    .map(|provider| provider.as_str().to_owned()),
            )?),
        );
    }
    Ok(projected)
}

/// Canonicalizes one target under the existing `TargetConfigurationDigest` domain.
pub fn canonicalize_target_configuration_v2(
    value: &Target,
) -> Result<CanonicalTargetConfigurationV2, CanonicalizationError> {
    let projected = target_configuration_v2(value)?;
    let bytes = serde_json_canonicalizer::to_vec(&projected)
        .map_err(|_| CanonicalizationError::Serialization)?;
    if bytes.len() > MAX_PROJECTION_BYTES {
        return Err(CanonicalizationError::ProjectionTooLarge);
    }
    let digest = TargetConfigurationDigest::hash_content(&bytes);
    Ok(CanonicalTargetConfigurationV2 { bytes, digest })
}
