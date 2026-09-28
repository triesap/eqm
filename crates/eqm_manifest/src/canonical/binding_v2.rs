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

use super::{CanonicalizationError, MAX_PROJECTION_BYTES, insert_optional, object};
use eqm_domain::{
    Applicability, ApplicabilityView, Artifact, Binding, DigestDomain, EvidenceSpecification,
    Exposure, ExtensionValue, Extensions, Sha256Digest,
};
use serde_json::{Map, Value, json};

/// The largest integer that survives an IEEE-754 double exactly.
const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

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

/// Returns the approved v2 canonical unsigned integer form.
fn wire_uint(value: u64) -> Value {
    if value <= MAX_SAFE_INTEGER {
        json!(value)
    } else {
        json!(value.to_string())
    }
}

/// Returns the approved v2 canonical signed integer form.
fn wire_int(value: i64) -> Value {
    if value.unsigned_abs() <= MAX_SAFE_INTEGER {
        json!(value)
    } else {
        json!(value.to_string())
    }
}

/// Canonically orders set members by complete JCS member bytes and rejects
/// exact duplicates instead of silently erasing them.
fn canonical_set(members: impl IntoIterator<Item = Value>) -> Result<Value, CanonicalizationError> {
    let mut encoded: Vec<(Vec<u8>, Value)> = Vec::new();
    for member in members {
        let bytes = serde_json_canonicalizer::to_vec(&member)
            .map_err(|_| CanonicalizationError::Serialization)?;
        encoded.push((bytes, member));
    }
    encoded.sort_by(|left, right| left.0.cmp(&right.0));
    for window in encoded.windows(2) {
        if window[0].0 == window[1].0 {
            return Err(CanonicalizationError::DuplicateMember);
        }
    }
    Ok(Value::Array(
        encoded.into_iter().map(|(_, member)| member).collect(),
    ))
}

/// Canonically orders a set of authored string members.
fn canonical_strings(
    values: impl IntoIterator<Item = String>,
) -> Result<Value, CanonicalizationError> {
    canonical_set(values.into_iter().map(Value::String))
}

/// Projects extensions with v2 integer forms, excluding only the display
/// namespace. Nested extension arrays keep their declared order.
fn extensions_v2(values: &Extensions) -> Result<Value, CanonicalizationError> {
    let mut projected = Map::new();
    for (namespace, value) in values.values() {
        if namespace.is_display_only() {
            continue;
        }
        projected.insert(namespace.as_str().to_owned(), extension_value_v2(value)?);
    }
    Ok(Value::Object(projected))
}

fn extension_value_v2(value: &ExtensionValue) -> Result<Value, CanonicalizationError> {
    Ok(match value {
        ExtensionValue::Boolean(value) => json!(value),
        ExtensionValue::Integer(value) => wire_int(*value),
        ExtensionValue::String(value) => json!(value),
        ExtensionValue::Array(values) => Value::Array(
            values
                .iter()
                .map(extension_value_v2)
                .collect::<Result<Vec<_>, _>>()?,
        ),
        ExtensionValue::Object(values) => {
            let mut projected = Map::new();
            for (key, value) in values {
                projected.insert(key.as_str().to_owned(), extension_value_v2(value)?);
            }
            Value::Object(projected)
        }
    })
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

fn evidence_v2(value: &EvidenceSpecification) -> Result<Value, CanonicalizationError> {
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
                    .map(evidence_v2)
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
