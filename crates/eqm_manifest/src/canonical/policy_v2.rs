//! Separate canonical v2 selected-policy/profile projection.
//!
//! DEV-006 chooses a narrow validated domain carrier
//! ([`PolicySelectionInputV2`]) and manifest-owned serialization. This module
//! projects the carrier's exact selected [`Policy`] and [`Profile`] definitions
//! plus explicit per-dimension known/unknown state into the v2 root
//! `{policy, profiles}`:
//!
//! - `policy` is the complete selected policy definition with v2 numeric and
//!   extension forms;
//! - `profiles` is a canonical set of `{definition, values}` records; the
//!   `definition` is the complete selected profile definition and `values` has
//!   exactly one `{"dimension","state","value"?}` record per declared dimension.
//!   `unknown` carries no `value` member and is never a default or an omission.
//!
//! Canonical sets are ordered by complete member bytes and reject exact
//! duplicates. Ordered extension arrays keep their order; defaults remain
//! object maps. Only `dev.equivalencematrix.display` is excluded. Mode,
//! authority origin, clock, waiver records, unrelated graph extensions and the
//! workspace summary are separate context axes and are not injected here. The
//! projection is pure and adds no engine/serialization dependency.

use super::support::{canonical_set, canonical_strings, extensions_v2, wire_uint};
use super::{CanonicalizationError, MAX_PROJECTION_BYTES, insert_optional, object};
use eqm_domain::{
    Policy, PolicyRule, PolicySelectionDigest, PolicySelectionInputV2, PolicySelector, Profile,
    ProfileDimension, SelectedDimensionStateV2, WaiverPolicy,
};
use serde_json::{Value, json};

/// Immutable canonical v2 policy-selection bytes and digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalPolicySelectionV2 {
    bytes: Vec<u8>,
    digest: PolicySelectionDigest,
}

impl CanonicalPolicySelectionV2 {
    /// Returns the exact one-line JCS bytes without a trailing newline.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the domain-separated policy-selection identity.
    #[must_use]
    pub const fn digest(&self) -> PolicySelectionDigest {
        self.digest
    }
}

fn selector_v2(value: &PolicySelector) -> Result<Value, CanonicalizationError> {
    let mut projected = serde_json::Map::new();
    if let Some(units) = value.units() {
        projected.insert(
            "units".to_owned(),
            canonical_strings(units.iter().map(|item| item.as_str().to_owned()))?,
        );
    }
    if let Some(requirements) = value.requirements() {
        projected.insert(
            "requirements".to_owned(),
            canonical_strings(requirements.iter().map(|item| item.as_str().to_owned()))?,
        );
    }
    if let Some(risk_classes) = value.risk_classes() {
        projected.insert(
            "risk_classes".to_owned(),
            canonical_strings(risk_classes.iter().map(|item| item.as_str().to_owned()))?,
        );
    }
    if let Some(facets) = value.facets() {
        projected.insert(
            "facets".to_owned(),
            canonical_strings(facets.iter().map(|item| item.as_str().to_owned()))?,
        );
    }
    if let Some(scopes) = value.scopes() {
        projected.insert(
            "scopes".to_owned(),
            canonical_strings(scopes.iter().map(|item| item.as_str().to_owned()))?,
        );
    }
    Ok(Value::Object(projected))
}

fn rule_v2(value: &PolicyRule) -> Result<Value, CanonicalizationError> {
    Ok(object([
        ("selector", selector_v2(value.selector())?),
        ("minimum_level", json!(value.minimum_level().as_str())),
        (
            "facets",
            canonical_strings(value.facets().iter().map(|item| item.as_str().to_owned()))?,
        ),
        ("minimum_trust", json!(value.minimum_trust().as_str())),
        ("maximum_age", wire_uint(value.maximum_age().get())),
        ("minimum_count", wire_uint(value.minimum_count().get())),
    ]))
}

fn waiver_v2(value: &WaiverPolicy) -> Result<Value, CanonicalizationError> {
    let mut projected = object([
        ("allowed", json!(value.allowed())),
        (
            "minimum_approvers",
            wire_uint(value.minimum_approvers().get()),
        ),
        (
            "required_controls",
            canonical_strings(
                value
                    .required_controls()
                    .iter()
                    .map(|item| item.as_str().to_owned()),
            )?,
        ),
    ]);
    insert_optional(
        &mut projected,
        "maximum_days",
        value
            .maximum_days()
            .map(|item| wire_uint(u64::from(item.get()))),
    );
    Ok(projected)
}

pub(super) fn policy_v2(value: &Policy) -> Result<Value, CanonicalizationError> {
    let mut projected = object([
        ("id", json!(value.id().as_str())),
        ("revision", wire_uint(value.revision().get())),
        ("title", json!(value.title().as_str())),
        (
            "owners",
            canonical_strings(value.owners().iter().map(|item| item.as_str().to_owned()))?,
        ),
        (
            "profiles",
            canonical_strings(value.profiles().iter().map(|item| item.as_str().to_owned()))?,
        ),
        (
            "required_targets",
            canonical_strings(
                value
                    .required_targets()
                    .iter()
                    .map(|item| item.as_str().to_owned()),
            )?,
        ),
        (
            "rules",
            canonical_set(
                value
                    .rules()
                    .iter()
                    .map(rule_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        ("waivers", waiver_v2(value.waivers())?),
        ("extensions", extensions_v2(value.extensions())?),
    ]);
    insert_optional(
        &mut projected,
        "description",
        value.description().map(|item| json!(item.as_str())),
    );
    Ok(projected)
}

fn dimension_declaration_v2(value: &ProfileDimension) -> Result<Value, CanonicalizationError> {
    let mut projected = object([
        ("id", json!(value.id().as_str())),
        (
            "values",
            canonical_strings(value.values().iter().map(|item| item.as_str().to_owned()))?,
        ),
    ]);
    insert_optional(
        &mut projected,
        "description",
        value.description().map(|item| json!(item.as_str())),
    );
    Ok(projected)
}

pub(super) fn profile_definition_v2(value: &Profile) -> Result<Value, CanonicalizationError> {
    let defaults = value
        .defaults()
        .iter()
        .map(|(dimension, item)| (dimension.as_str().to_owned(), json!(item.as_str())))
        .collect::<serde_json::Map<_, _>>();
    let mut projected = object([
        ("id", json!(value.id().as_str())),
        ("revision", wire_uint(value.revision().get())),
        ("title", json!(value.title().as_str())),
        (
            "owners",
            canonical_strings(value.owners().iter().map(|item| item.as_str().to_owned()))?,
        ),
        (
            "dimensions",
            canonical_set(
                value
                    .dimensions()
                    .values()
                    .map(dimension_declaration_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        ("defaults", Value::Object(defaults)),
        ("extensions", extensions_v2(value.extensions())?),
    ]);
    insert_optional(
        &mut projected,
        "description",
        value.description().map(|item| json!(item.as_str())),
    );
    Ok(projected)
}

fn dimension_value_v2(
    dimension: &eqm_domain::DimensionId,
    state: &SelectedDimensionStateV2,
) -> Value {
    match state {
        SelectedDimensionStateV2::Known(value) => object([
            ("dimension", json!(dimension.as_str())),
            ("state", json!("known")),
            ("value", json!(value.as_str())),
        ]),
        SelectedDimensionStateV2::Unknown => object([
            ("dimension", json!(dimension.as_str())),
            ("state", json!("unknown")),
        ]),
    }
}

/// Projects one validated carrier into the exact v2 policy-selection shape.
pub fn policy_selection_v2(
    value: &PolicySelectionInputV2<'_>,
) -> Result<Value, CanonicalizationError> {
    let profiles = value
        .profiles()
        .iter()
        .map(|selected| {
            Ok(object([
                ("definition", profile_definition_v2(selected.profile())?),
                (
                    "values",
                    canonical_set(
                        selected
                            .values()
                            .iter()
                            .map(|(dimension, state)| dimension_value_v2(dimension, state))
                            .collect::<Vec<_>>(),
                    )?,
                ),
            ]))
        })
        .collect::<Result<Vec<_>, CanonicalizationError>>()?;
    Ok(object([
        ("policy", policy_v2(value.policy())?),
        ("profiles", canonical_set(profiles)?),
    ]))
}

/// Canonicalizes one carrier under the existing `PolicySelectionDigest` domain.
pub fn canonicalize_policy_selection_v2(
    value: &PolicySelectionInputV2<'_>,
) -> Result<CanonicalPolicySelectionV2, CanonicalizationError> {
    let projected = policy_selection_v2(value)?;
    let bytes = serde_json_canonicalizer::to_vec(&projected)
        .map_err(|_| CanonicalizationError::Serialization)?;
    if bytes.len() > MAX_PROJECTION_BYTES {
        return Err(CanonicalizationError::ProjectionTooLarge);
    }
    let digest = PolicySelectionDigest::hash_content(&bytes);
    Ok(CanonicalPolicySelectionV2 { bytes, digest })
}
