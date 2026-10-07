//! Exact pre-result selection projection, separate from obligation/plan identity.
use super::support::{canonical_set, wire_uint};
use super::{CanonicalizationError, MAX_PROJECTION_BYTES, object};
use eqm_domain::{ExpectedSelectionManifest, ExpectedSelectionManifestDigest, LogicalCaseId};
use serde_json::{Value, json};
use std::collections::BTreeSet;

/// Immutable canonical bytes and their distinct domain-separated identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalExpectedSelectionV2 {
    bytes: Vec<u8>,
    digest: ExpectedSelectionManifestDigest,
}
impl CanonicalExpectedSelectionV2 {
    /// Exact JCS bytes without a trailing newline.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// Semantic identity, not authority.
    #[must_use]
    pub const fn digest(&self) -> ExpectedSelectionManifestDigest {
        self.digest
    }
}
fn cases(values: &BTreeSet<LogicalCaseId>) -> Result<Value, CanonicalizationError> {
    canonical_set(values.iter().map(|id| json!(id.as_str())))
}
/// Projects every defined semantic axis; retains all unselected alternatives.
pub fn expected_selection_v2(
    value: &ExpectedSelectionManifest,
) -> Result<Value, CanonicalizationError> {
    let source = value.source();
    let groups = value
        .alternatives()
        .values()
        .map(|group| {
            let options = group
                .options()
                .iter()
                .map(|(id, members)| {
                    Ok(object([
                        ("id", json!(id.as_str())),
                        ("cases", cases(members)?),
                    ]))
                })
                .collect::<Result<Vec<_>, CanonicalizationError>>()?;
            Ok(object([
                ("id", json!(group.id().as_str())),
                ("selected", json!(group.selected().as_str())),
                ("options", canonical_set(options)?),
            ]))
        })
        .collect::<Result<Vec<_>, CanonicalizationError>>()?;
    Ok(object([
        (
            "source",
            object([
                ("binding", json!(source.binding.as_str())),
                ("binding_revision", wire_uint(source.binding_revision.get())),
                ("binding_digest", json!(source.binding_digest.to_string())),
                ("specification", json!(source.specification.as_str())),
                (
                    "specification_digest",
                    json!(source.specification_digest.to_string()),
                ),
                ("plan", json!(source.plan.as_str())),
                ("campaign", json!(source.campaign.as_str())),
            ]),
        ),
        ("kind", json!(value.kind().as_str())),
        ("count_unit", json!(value.count_unit().as_str())),
        ("required", cases(value.required())?),
        ("alternatives", canonical_set(groups)?),
        (
            "independent_review_principals",
            json!(value.independent_review_principals()),
        ),
    ]))
}
/// Applies existing strict NFC/JCS/set/numeric bounds, without changing frozen roots.
pub fn canonicalize_expected_selection_v2(
    value: &ExpectedSelectionManifest,
) -> Result<CanonicalExpectedSelectionV2, CanonicalizationError> {
    let bytes = serde_json_canonicalizer::to_vec(&expected_selection_v2(value)?)
        .map_err(|_| CanonicalizationError::Serialization)?;
    if bytes.len() > MAX_PROJECTION_BYTES {
        return Err(CanonicalizationError::ProjectionTooLarge);
    }
    let digest = ExpectedSelectionManifestDigest::hash_content(&bytes);
    Ok(CanonicalExpectedSelectionV2 { bytes, digest })
}
