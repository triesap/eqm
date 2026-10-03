//! Separate v2 comparison-context projection and `ComparisonContextDigest`.
//!
//! DEV-009/DEV-012 allocate `eqm:v2:comparison-context` over exactly the four
//! shared members `contract_digest`, `scenario_digest`, `profiles` and
//! `comparison_rule_digest`. The wire root adds only the exact schema marker.
//!
//! `scenario_digest` reuses the independent `eqm:v2:scenario` projection.
//! `profiles` reuses the approved selected-profile definition/value projection
//! (never a profile-ID-only list or the whole selected policy).
//! `comparison_rule_digest` is an explicitly supplied structural SHA reference
//! to approved rule authority: this step invents no rule format, domain,
//! default or policy-digest substitution.
//!
//! Exact participant build/source/artifact assignments never enter this shared
//! identity, so two distinct subjects can hold the same comparison context.
//! Possessing the digest establishes neither subject validity nor trust.

use super::policy_v2::policy_selection_v2;
use super::scenario_v2::canonicalize_scenario_v2;
use super::{CanonicalizationError, MAX_PROJECTION_BYTES, object};
use eqm_domain::{ComparisonContextDigest, ComparisonContextInputV2};
use serde_json::{Value, json};

/// Immutable canonical v2 comparison-context bytes and digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalComparisonContextV2 {
    bytes: Vec<u8>,
    digest: ComparisonContextDigest,
}

impl CanonicalComparisonContextV2 {
    /// Returns the exact one-line JCS bytes without a trailing newline.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the domain-separated comparison-context identity.
    #[must_use]
    pub const fn digest(&self) -> ComparisonContextDigest {
        self.digest
    }
}

/// Projects one comparison-context input into the exact four-member semantic
/// root (the wire carrier adds the schema marker separately).
pub fn comparison_context_v2(
    input: &ComparisonContextInputV2<'_>,
) -> Result<Value, CanonicalizationError> {
    let selection = policy_selection_v2(input.selection())?;
    let profiles = selection
        .get("profiles")
        .cloned()
        .ok_or(CanonicalizationError::Serialization)?;
    Ok(object([
        (
            "contract_digest",
            json!(input.contract_digest().to_string()),
        ),
        (
            "scenario_digest",
            json!(
                canonicalize_scenario_v2(input.scenario())?
                    .digest()
                    .to_string()
            ),
        ),
        ("profiles", profiles),
        (
            "comparison_rule_digest",
            json!(input.comparison_rule_digest().to_string()),
        ),
    ]))
}

/// Canonicalizes one comparison context under `eqm:v2:comparison-context`.
pub fn canonicalize_comparison_context_v2(
    input: &ComparisonContextInputV2<'_>,
) -> Result<CanonicalComparisonContextV2, CanonicalizationError> {
    let projected = comparison_context_v2(input)?;
    let bytes = serde_json_canonicalizer::to_vec(&projected)
        .map_err(|_| CanonicalizationError::Serialization)?;
    if bytes.len() > MAX_PROJECTION_BYTES {
        return Err(CanonicalizationError::ProjectionTooLarge);
    }
    let digest = ComparisonContextDigest::hash_content(&bytes);
    Ok(CanonicalComparisonContextV2 { bytes, digest })
}
