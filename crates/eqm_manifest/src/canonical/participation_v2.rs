//! Separate v2 capability-participation projection and `ParticipationDigest`.
//!
//! DEV-009/DEV-012 allocate `eqm:v2:participation` over exactly `id`,
//! `requirement`, `targets`, `applicability` and `extensions`. Targets are a
//! canonical member-byte set with duplicate rejection; applicability uses the
//! approved recursive v2 forms (its `all`/`any` children are semantic sets) and
//! extensions use the recursive type-tagged v2 forms, excluding only
//! `dev.equivalencematrix.display`. The projection is pure and independent of
//! the frozen `ContractDigest` root.

use super::support::{applicability_v2, canonical_strings, extensions_v2};
use super::{CanonicalizationError, MAX_PROJECTION_BYTES, object};
use eqm_domain::{ParticipationDigest, ParticipationRule};
use serde_json::{Value, json};

/// Immutable canonical v2 participation-rule bytes and digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalParticipationV2 {
    bytes: Vec<u8>,
    digest: ParticipationDigest,
}

impl CanonicalParticipationV2 {
    /// Returns the exact one-line JCS bytes without a trailing newline.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the domain-separated participation identity.
    #[must_use]
    pub const fn digest(&self) -> ParticipationDigest {
        self.digest
    }
}

/// Projects one participation rule into its exact v2 semantic shape.
pub fn participation_v2(value: &ParticipationRule) -> Result<Value, CanonicalizationError> {
    Ok(object([
        ("id", json!(value.id().as_str())),
        ("requirement", json!(value.requirement().as_str())),
        (
            "targets",
            canonical_strings(
                value
                    .targets()
                    .iter()
                    .map(|target| target.as_str().to_owned()),
            )?,
        ),
        ("applicability", applicability_v2(value.applicability())?),
        ("extensions", extensions_v2(value.extensions())?),
    ]))
}

/// Canonicalizes one participation rule under `eqm:v2:participation`.
pub fn canonicalize_participation_v2(
    value: &ParticipationRule,
) -> Result<CanonicalParticipationV2, CanonicalizationError> {
    let projected = participation_v2(value)?;
    let bytes = serde_json_canonicalizer::to_vec(&projected)
        .map_err(|_| CanonicalizationError::Serialization)?;
    if bytes.len() > MAX_PROJECTION_BYTES {
        return Err(CanonicalizationError::ProjectionTooLarge);
    }
    let digest = ParticipationDigest::hash_content(&bytes);
    Ok(CanonicalParticipationV2 { bytes, digest })
}
