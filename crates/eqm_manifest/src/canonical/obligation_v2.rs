//! Complete pre-evidence coordinate projection; no execution or trust inference.
use super::policy_v2::policy_selection_v2;
use super::scenario_v2::{canonicalize_scenario_v2, scenario_v2};
use super::support::{canonical_set, wire_uint};
use super::{
    CanonicalizationError, MAX_PROJECTION_BYTES, canonicalize_provider_configuration_v2,
    canonicalize_target_configuration_v2, insert_optional, object,
};
use eqm_domain::{
    ExpectedSelectionIdentityV2, ObligationDigest, ObligationIdentityInputV2,
    ObligationScopeInputV2, ProviderScopeInputV2, ScopeParticipationV2, TargetConfigurationDigest,
    TargetId,
};
use serde_json::{Value, json};
use std::collections::BTreeMap;

/// Exact coordinate bytes and identity, not evaluated candidate authorization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalObligationV2 {
    bytes: Vec<u8>,
    digest: ObligationDigest,
}
impl CanonicalObligationV2 {
    /// Exact JCS bytes without a trailing newline.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// Complete domain-separated pre-evidence coordinate identity.
    #[must_use]
    pub const fn digest(&self) -> ObligationDigest {
        self.digest
    }
    /// Explicitly versioned transport reference shared by every report adapter.
    #[must_use]
    pub fn reference(&self) -> String {
        format!("eqm:v2:obligation:{}", self.digest)
    }
}
fn targets(
    values: &BTreeMap<TargetId, TargetConfigurationDigest>,
) -> Result<Value, CanonicalizationError> {
    canonical_set(values.iter().map(|(id, digest)| {
        object([
            ("target", json!(id.as_str())),
            ("configuration_digest", json!(digest.to_string())),
        ])
    }))
}
fn participation(
    values: &BTreeMap<TargetId, ScopeParticipationV2>,
) -> Result<Value, CanonicalizationError> {
    canonical_set(values.iter().map(|(target, state)| {
        object([
            ("target", json!(target.as_str())),
            ("state", json!(state.as_str())),
        ])
    }))
}
fn provider(value: &ProviderScopeInputV2<'_>) -> Result<Value, CanonicalizationError> {
    Ok(object([
        ("kind", json!("provider")),
        ("provider", json!(value.provider.id().as_str())),
        (
            "configuration_digest",
            json!(
                canonicalize_provider_configuration_v2(value.provider)?
                    .digest()
                    .to_string()
            ),
        ),
        ("dependers", targets(&value.dependers)?),
        ("assignments", targets(&value.assignments)?),
        (
            "dependencies",
            canonical_set(value.dependencies.iter().map(|(from, to)| {
                object([("from", json!(from.as_str())), ("to", json!(to.as_str()))])
            }))?,
        ),
        (
            "dependency_configurations",
            targets(&value.dependency_configurations)?,
        ),
        ("participation", participation(&value.participation)?),
    ]))
}
fn subject(value: &ObligationScopeInputV2<'_>) -> Result<Value, CanonicalizationError> {
    Ok(match value {
        ObligationScopeInputV2::Target(value) => object([
            ("kind", json!("target")),
            ("target", json!(value.id().as_str())),
            (
                "configuration_digest",
                json!(
                    canonicalize_target_configuration_v2(value)?
                        .digest()
                        .to_string()
                ),
            ),
        ]),
        ObligationScopeInputV2::Provider(value) => provider(value)?,
        ObligationScopeInputV2::Scenario {
            scenario,
            selected_targets,
            providers,
            participation: states,
        } => object([
            ("kind", json!("scenario")),
            ("scenario", scenario_v2(scenario)?),
            (
                "scenario_digest",
                json!(canonicalize_scenario_v2(scenario)?.digest().to_string()),
            ),
            ("selected_targets", targets(selected_targets)?),
            (
                "providers",
                canonical_set(
                    providers
                        .iter()
                        .map(provider)
                        .collect::<Result<Vec<_>, _>>()?,
                )?,
            ),
            ("participation", participation(states)?),
        ]),
    })
}
/// Projects every available immutable coordinate axis. Missing expectations
/// remain explicit; a pure Present(empty) claim is distinct and grants no trust.
pub fn obligation_v2(
    value: &ObligationIdentityInputV2<'_>,
) -> Result<Value, CanonicalizationError> {
    let expectations = match &value.expected_selections {
        ExpectedSelectionIdentityV2::Missing => object([("state", json!("missing"))]),
        ExpectedSelectionIdentityV2::Present(digests) => object([
            ("state", json!("present")),
            (
                "digests",
                canonical_set(digests.iter().map(|digest| json!(digest.to_string())))?,
            ),
        ]),
    };
    let strength = value.strength;
    let mut projected = object([
        ("unit", json!(value.unit.as_str())),
        ("requirement", json!(value.requirement.as_str())),
        ("facet", json!(value.facet.as_str())),
        ("policy_selection", policy_selection_v2(value.selection)?),
        (
            "semantic_graph_digest",
            json!(value.semantic_graph_digest.to_string()),
        ),
        ("contract_digest", json!(value.contract_digest.to_string())),
        (
            "scope_bindings_digest",
            json!(value.scope_bindings_digest.to_string()),
        ),
        (
            "participation_digests",
            canonical_set(
                value
                    .participation_digests
                    .iter()
                    .map(|digest| json!(digest.to_string())),
            )?,
        ),
        ("subject", subject(&value.subject)?),
        ("expected_selections", expectations),
        (
            "strength",
            object([
                ("minimum_level", json!(strength.minimum_level.as_str())),
                ("minimum_trust", json!(strength.minimum_trust.as_str())),
                ("maximum_age_ms", wire_uint(strength.maximum_age.get())),
                ("minimum_count", wire_uint(strength.minimum_count.get())),
            ]),
        ),
        ("mode", json!(value.mode.as_str())),
        ("authority_origin", json!(value.authority_origin.as_str())),
    ]);
    insert_optional(
        &mut projected,
        "release_context",
        value
            .release_context
            .map(|digest| json!(digest.to_string())),
    );
    Ok(projected)
}
/// Canonicalizes the complete pre-evidence coordinate using the approved domain.
pub fn canonicalize_obligation_v2(
    value: &ObligationIdentityInputV2<'_>,
) -> Result<CanonicalObligationV2, CanonicalizationError> {
    let bytes = serde_json_canonicalizer::to_vec(&obligation_v2(value)?)
        .map_err(|_| CanonicalizationError::Serialization)?;
    if bytes.len() > MAX_PROJECTION_BYTES {
        return Err(CanonicalizationError::ProjectionTooLarge);
    }
    let digest = ObligationDigest::hash_content(&bytes);
    Ok(CanonicalObligationV2 { bytes, digest })
}
