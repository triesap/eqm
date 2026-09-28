//! Separate complete v2 binding projection and `BindingDigest` identity.
//!
//! DEV-005 replaces the ID-only binding digest shortcut. The existing v1
//! [`super::binding`] function is a field inventory, not reusable v2 bytes: it
//! uses v1 integer forms and set ordering. This module builds an independent v2
//! projection that:
//!
//! - includes every existing binding, artifact, exposure and
//!   evidence-specification semantic field plus extensions at each level;
//! - encodes revisions, counts and freshness with the approved v2 integer wire
//!   rules and extension integers the same way;
//! - orders owners, artifacts, exposures, evidence specifications,
//!   requirements, facets and applicability membership values by each member's
//!   complete canonical JSON UTF-8 bytes, rejecting exact duplicates;
//! - preserves the order of ordered expression and extension arrays;
//! - excludes only the `dev.equivalencematrix.display` extension namespace;
//! - includes an artifact's declared path (semantics) and never invents a
//!   binding description/comment field.
//!
//! The v1 binding/fragment/graph APIs, bytes and hashes are unchanged. This
//! projection uses the existing [`DigestDomain::Binding`] domain and adds no
//! dependency from `eqm_manifest` to the protocol/transport layer. Later owning
//! v2 construction/relationship steps integrate this component identity; this
//! step does not reinterpret legacy v1 transports.

use super::support::{canonical_set, canonical_strings, extensions_v2, wire_uint};
use super::{CanonicalizationError, MAX_PROJECTION_BYTES, insert_optional, object};
use eqm_domain::{
    Applicability, ApplicabilityView, Artifact, Binding, DigestDomain, Exposure, Sha256Digest,
};
use serde_json::{Value, json};

/// Immutable canonical v2 binding bytes and domain-separated digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalBindingV2 {
    bytes: Vec<u8>,
    digest: Sha256Digest,
}

impl CanonicalBindingV2 {
    /// Returns the exact one-line JCS bytes without a trailing newline.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the domain-separated `BindingDigest` identity.
    #[must_use]
    pub const fn digest(&self) -> Sha256Digest {
        self.digest
    }
}

fn applicability_v2(value: &Applicability) -> Result<Value, CanonicalizationError> {
    Ok(match value.view() {
        ApplicabilityView::Constant(always) => object([("always", json!(always))]),
        ApplicabilityView::Comparison(dimension, operator, item) => object([
            ("dimension", json!(dimension.as_str())),
            ("operator", json!(operator.as_str())),
            ("value", json!(item.as_str())),
        ]),
        ApplicabilityView::Membership(dimension, operator, values) => object([
            ("dimension", json!(dimension.as_str())),
            ("operator", json!(operator.as_str())),
            (
                "values",
                canonical_strings(values.iter().map(|item| item.as_str().to_owned()))?,
            ),
        ]),
        ApplicabilityView::All(values) => object([(
            "all",
            Value::Array(
                values
                    .iter()
                    .map(applicability_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            ),
        )]),
        ApplicabilityView::Any(values) => object([(
            "any",
            Value::Array(
                values
                    .iter()
                    .map(applicability_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            ),
        )]),
        ApplicabilityView::Not(value) => object([("not", applicability_v2(value)?)]),
    })
}

fn artifact_v2(value: &Artifact) -> Result<Value, CanonicalizationError> {
    let mut projected = object([
        ("id", json!(value.id().as_str())),
        ("role", json!(value.role().as_str())),
        ("path", json!(value.path().as_str())),
        ("extensions", extensions_v2(value.extensions())?),
    ]);
    insert_optional(
        &mut projected,
        "surface",
        value.surface().map(|item| json!(item.as_str())),
    );
    insert_optional(
        &mut projected,
        "symbol",
        value.symbol().map(|item| json!(item.as_str())),
    );
    insert_optional(
        &mut projected,
        "selector",
        value.selector().map(super::artifact_selector),
    );
    Ok(projected)
}

fn exposure_v2(value: &Exposure) -> Result<Value, CanonicalizationError> {
    let mut projected = object([
        ("surface", json!(value.surface().as_str())),
        ("state", json!(value.state().as_str())),
        ("applicability", applicability_v2(value.applicability())?),
        ("extensions", extensions_v2(value.extensions())?),
    ]);
    insert_optional(
        &mut projected,
        "route",
        value.route().map(|item| json!(item.as_str())),
    );
    Ok(projected)
}

/// Projects one binding into its complete v2 canonical semantic JSON shape.
///
/// Set-valued fields are ordered by complete member bytes and exact duplicates
/// are rejected. Ordered expression and extension arrays keep their order.
pub fn binding_v2(value: &Binding) -> Result<Value, CanonicalizationError> {
    Ok(object([
        ("id", json!(value.id().as_str())),
        ("revision", wire_uint(value.revision().get())),
        ("target", json!(value.target().as_str())),
        ("unit", json!(value.unit().as_str())),
        (
            "owners",
            canonical_strings(value.owners().iter().map(|item| item.as_str().to_owned()))?,
        ),
        (
            "artifacts",
            canonical_set(
                value
                    .artifacts()
                    .values()
                    .values()
                    .map(artifact_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        (
            "exposures",
            canonical_set(
                value
                    .exposures()
                    .iter()
                    .map(exposure_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        (
            "evidence",
            canonical_set(
                value
                    .evidence()
                    .values()
                    .map(super::evidence_spec_v2::evidence_spec_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        ("extensions", extensions_v2(value.extensions())?),
    ]))
}

/// Canonicalizes one binding under the existing `BindingDigest` domain.
pub fn canonicalize_binding_v2(
    value: &Binding,
) -> Result<CanonicalBindingV2, CanonicalizationError> {
    let projected = binding_v2(value)?;
    let bytes = serde_json_canonicalizer::to_vec(&projected)
        .map_err(|_| CanonicalizationError::Serialization)?;
    if bytes.len() > MAX_PROJECTION_BYTES {
        return Err(CanonicalizationError::ProjectionTooLarge);
    }
    let digest = Sha256Digest::hash_domain(DigestDomain::Binding, &bytes);
    Ok(CanonicalBindingV2 { bytes, digest })
}
