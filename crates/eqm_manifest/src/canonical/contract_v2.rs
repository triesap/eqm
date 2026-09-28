//! Separate complete v2 contract projection and `ContractDigest` identity.
//!
//! DEV-006 chooses contract authority *plus* normative graph extensions: the v2
//! contract root is exactly `capabilities`, `journeys`, `surfaces`, `fragments`
//! and `extensions`. It is built from the prepared [`FinalizedWorkspaceGraph`]
//! (all loaded exact fragment revisions and expanded surface requirements), not
//! from a filtered CLI view, and it never reuses the v1 semantic-graph bytes or
//! its schema marker.
//!
//! Every entity is projected explicitly with the approved v2 forms:
//!
//! - revisions use the v2 integer wire form and extension values are recursively
//!   type-tagged (see [`super::support::extensions_v2`] for the S018-R1 rules);
//! - canonical sets are ordered by complete member bytes and reject exact
//!   duplicates;
//! - ordered sequences (a journey's declared surfaces, applicability `all`/`any`
//!   children and extension arrays) keep their order and are never sorted;
//! - only the `dev.equivalencematrix.display` extension namespace is excluded;
//!   titles and descriptions are not display-only.
//!
//! Targets, bindings, policies, profiles, runners, waivers, imports and adapter
//! locks are not top-level contract components; their own identities and later
//! provenance retain those axes. The projection is pure and adds no
//! `eqm_manifest` -> engine/protocol dependency.

use super::support::{canonical_set, canonical_strings, extensions_v2, wire_uint};
use super::{CanonicalizationError, MAX_PROJECTION_BYTES, insert_optional, object};
use eqm_domain::{
    Applicability, ApplicabilityView, Capability, ContractDigest, FinalizedWorkspaceGraph,
    Fragment, FragmentUse, Journey, Requirement, Surface, Transition,
};
use serde_json::{Value, json};

/// Immutable canonical v2 contract bytes and domain-separated digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalContractV2 {
    bytes: Vec<u8>,
    digest: ContractDigest,
}

impl CanonicalContractV2 {
    /// Returns the exact one-line JCS bytes without a trailing newline.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the domain-separated contract identity.
    #[must_use]
    pub const fn digest(&self) -> ContractDigest {
        self.digest
    }
}

fn with_description(mut value: Value, description: Option<&str>) -> Value {
    insert_optional(
        &mut value,
        "description",
        description.map(|item| json!(item)),
    );
    value
}

fn capability_v2(value: &Capability) -> Result<Value, CanonicalizationError> {
    Ok(with_description(
        object([
            ("id", json!(value.id().as_str())),
            ("title", json!(value.title().as_str())),
            ("status", json!(value.status().as_str())),
            (
                "owners",
                canonical_strings(value.owners().iter().map(|owner| owner.as_str().to_owned()))?,
            ),
            ("extensions", extensions_v2(value.extensions())?),
        ]),
        value.description().map(|item| item.as_str()),
    ))
}

fn transition_v2(value: &Transition) -> Value {
    object([
        ("from", json!(value.from().as_str())),
        ("to", json!(value.to().as_str())),
        ("trigger", json!(value.trigger().as_str())),
    ])
}

fn journey_v2(value: &Journey) -> Result<Value, CanonicalizationError> {
    Ok(with_description(
        object([
            ("id", json!(value.id().as_str())),
            ("revision", wire_uint(value.revision().get())),
            ("title", json!(value.title().as_str())),
            ("capability", json!(value.capability().as_str())),
            ("status", json!(value.status().as_str())),
            ("risk_class", json!(value.risk_class().as_str())),
            (
                "owners",
                canonical_strings(value.owners().iter().map(|owner| owner.as_str().to_owned()))?,
            ),
            // A journey's declared surfaces are an ordered sequence.
            (
                "surfaces",
                Value::Array(
                    value
                        .surfaces()
                        .iter()
                        .map(|surface| json!(surface.as_str()))
                        .collect(),
                ),
            ),
            (
                "transitions",
                canonical_set(value.transitions().iter().map(transition_v2))?,
            ),
            ("extensions", extensions_v2(value.extensions())?),
        ]),
        value.description().map(|item| item.as_str()),
    ))
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

fn requirement_v2(value: &Requirement) -> Result<Value, CanonicalizationError> {
    let mut projected = object([
        ("id", json!(value.id().as_str())),
        ("level", json!(value.level().as_str())),
        ("scope", json!(value.scope().as_str())),
        ("statement", json!(value.statement().as_str())),
        (
            "facets",
            canonical_strings(value.facets().iter().map(|item| item.as_str().to_owned()))?,
        ),
        ("applicability", applicability_v2(value.applicability())?),
        ("extensions", extensions_v2(value.extensions())?),
    ]);
    insert_optional(
        &mut projected,
        "risk_class",
        value.risk_class().map(|item| json!(item.as_str())),
    );
    insert_optional(
        &mut projected,
        "provider",
        value.provider().map(|item| json!(item.as_str())),
    );
    Ok(projected)
}

fn fragment_use_v2(value: &FragmentUse) -> Result<Value, CanonicalizationError> {
    let mut projected = object([
        ("fragment", json!(value.fragment().as_str())),
        ("revision", wire_uint(value.revision().get())),
        ("digest", json!(value.digest().to_string())),
    ]);
    insert_optional(
        &mut projected,
        "prefix",
        value.prefix().map(|item| json!(item.as_str())),
    );
    Ok(projected)
}

fn surface_v2(value: &Surface) -> Result<Value, CanonicalizationError> {
    Ok(with_description(
        object([
            ("id", json!(value.id().as_str())),
            ("revision", wire_uint(value.revision().get())),
            ("title", json!(value.title().as_str())),
            ("journey", json!(value.journey().as_str())),
            ("status", json!(value.status().as_str())),
            (
                "owners",
                canonical_strings(value.owners().iter().map(|owner| owner.as_str().to_owned()))?,
            ),
            (
                "requirements",
                canonical_set(
                    value
                        .requirements()
                        .values()
                        .map(requirement_v2)
                        .collect::<Result<Vec<_>, _>>()?,
                )?,
            ),
            (
                "fragment_origins",
                canonical_set(
                    value
                        .fragments()
                        .iter()
                        .map(fragment_use_v2)
                        .collect::<Result<Vec<_>, _>>()?,
                )?,
            ),
            ("extensions", extensions_v2(value.extensions())?),
        ]),
        value.description().map(|item| item.as_str()),
    ))
}

fn fragment_v2(value: &Fragment) -> Result<Value, CanonicalizationError> {
    Ok(with_description(
        object([
            ("id", json!(value.id().as_str())),
            ("revision", wire_uint(value.revision().get())),
            ("title", json!(value.title().as_str())),
            ("risk_class", json!(value.risk_class().as_str())),
            (
                "owners",
                canonical_strings(value.owners().iter().map(|owner| owner.as_str().to_owned()))?,
            ),
            (
                "requirements",
                canonical_set(
                    value
                        .requirements()
                        .values()
                        .map(requirement_v2)
                        .collect::<Result<Vec<_>, _>>()?,
                )?,
            ),
            ("extensions", extensions_v2(value.extensions())?),
        ]),
        value.description().map(|item| item.as_str()),
    ))
}

/// Projects one finalized graph into the exact v2 contract semantic shape.
pub fn contract_v2(finalized: &FinalizedWorkspaceGraph) -> Result<Value, CanonicalizationError> {
    let graph = finalized.graph();
    Ok(object([
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
        ("extensions", extensions_v2(graph.extensions())?),
    ]))
}

/// Canonicalizes one finalized graph under the existing `ContractDigest` domain.
pub fn canonicalize_contract_v2(
    finalized: &FinalizedWorkspaceGraph,
) -> Result<CanonicalContractV2, CanonicalizationError> {
    let projected = contract_v2(finalized)?;
    let bytes = serde_json_canonicalizer::to_vec(&projected)
        .map_err(|_| CanonicalizationError::Serialization)?;
    if bytes.len() > MAX_PROJECTION_BYTES {
        return Err(CanonicalizationError::ProjectionTooLarge);
    }
    let digest = ContractDigest::hash_content(&bytes);
    Ok(CanonicalContractV2 { bytes, digest })
}
