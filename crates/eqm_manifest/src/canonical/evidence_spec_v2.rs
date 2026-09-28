//! Separate canonical v2 evidence-specification projection.
//!
//! DEV-005 removes the ID-pair evidence-specification shortcut. The v1
//! [`super::binding`] field inventory embeds evidence specifications but uses
//! v1 integer forms and set ordering, so it is not reusable v2 bytes. This
//! module builds an independent v2 projection under the existing
//! [`DigestDomain::EvidenceSpecification`] domain.
//!
//! The projection covers the declared input domains of an
//! [`EvidenceSpecification`]:
//!
//! - `id` is a local `EvidenceSpecId` unique within one binding;
//! - `kind` is the closed `EvidenceKind` vocabulary; it determines whether a
//!   `runner` and a positive `minimum_count` are required or forbidden;
//! - `requirements` is a nonempty set of fully qualified `FullRequirementId`
//!   values (`<unit>#<local>`) and `facets` a nonempty set of the closed
//!   `Facet` vocabulary;
//! - `selector` is the kind-compatible `EvidenceSelector` when present and
//!   must not conflict with `kind`;
//! - `minimum_count` is the normalized positive count for countable kinds;
//! - `freshness` is an optional bounded `DurationMillis` age ceiling;
//! - `extensions` are the semantic namespaced values at this level.
//!
//! Requirements, facets and any applicability set are ordered by complete
//! canonical member bytes. Ordered extension arrays keep their order. Only the
//! `dev.equivalencematrix.display` namespace is excluded. The v1 evidence
//! projection, bytes and hashes are unchanged, and no `eqm_manifest` ->
//! protocol/transport dependency is added.

use super::support::{canonical_strings, extensions_v2, wire_uint};
use super::{CanonicalizationError, MAX_PROJECTION_BYTES, insert_optional, object};
use eqm_domain::{DigestDomain, EvidenceSpecification, Sha256Digest};
use serde_json::{Value, json};

/// Immutable canonical v2 evidence-specification bytes and digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalEvidenceSpecV2 {
    bytes: Vec<u8>,
    digest: Sha256Digest,
}

impl CanonicalEvidenceSpecV2 {
    /// Returns the exact one-line JCS bytes without a trailing newline.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the domain-separated evidence-specification identity.
    #[must_use]
    pub const fn digest(&self) -> Sha256Digest {
        self.digest
    }
}

/// Projects one evidence specification into its complete v2 semantic shape.
pub fn evidence_spec_v2(value: &EvidenceSpecification) -> Result<Value, CanonicalizationError> {
    let mut projected = object([
        ("id", json!(value.id().as_str())),
        ("kind", json!(value.kind().as_str())),
        (
            "requirements",
            canonical_strings(
                value
                    .requirements()
                    .iter()
                    .map(|item| item.as_str().to_owned()),
            )?,
        ),
        (
            "facets",
            canonical_strings(value.facets().iter().map(|item| item.as_str().to_owned()))?,
        ),
        ("extensions", extensions_v2(value.extensions())?),
    ]);
    insert_optional(
        &mut projected,
        "runner",
        value.runner().map(|item| json!(item.as_str())),
    );
    insert_optional(
        &mut projected,
        "selector",
        value.selector().map(super::evidence_selector),
    );
    insert_optional(
        &mut projected,
        "minimum_count",
        value.minimum_count().map(|item| wire_uint(item.get())),
    );
    insert_optional(
        &mut projected,
        "freshness",
        value.freshness().map(|item| wire_uint(item.get())),
    );
    Ok(projected)
}

/// Canonicalizes one evidence specification under the existing
/// `EvidenceSpecificationDigest` domain.
pub fn canonicalize_evidence_spec_v2(
    value: &EvidenceSpecification,
) -> Result<CanonicalEvidenceSpecV2, CanonicalizationError> {
    let projected = evidence_spec_v2(value)?;
    let bytes = serde_json_canonicalizer::to_vec(&projected)
        .map_err(|_| CanonicalizationError::Serialization)?;
    if bytes.len() > MAX_PROJECTION_BYTES {
        return Err(CanonicalizationError::ProjectionTooLarge);
    }
    let digest = Sha256Digest::hash_domain(DigestDomain::EvidenceSpecification, &bytes);
    Ok(CanonicalEvidenceSpecV2 { bytes, digest })
}
