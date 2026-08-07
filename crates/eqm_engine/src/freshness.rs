//! Exact evidence freshness and cache-identity evaluation.

use eqm_domain::{
    DurationMillis, EvidenceSubject, ProducerRef, ProfileId, ProfileSelection, Sha256Digest,
    ToolVersion, UtcInstant,
};
use std::collections::{BTreeMap, BTreeSet};

const CLOCK_TOLERANCE_MILLIS: i128 = 5 * 60 * 1_000;

/// Complete exact semantic freshness tuple.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FreshnessKey {
    /// Exact evaluated subject.
    pub subject: EvidenceSubject,
    /// Final contract digest.
    pub contract_digest: Sha256Digest,
    /// Binding digest.
    pub binding_digest: Sha256Digest,
    /// Evidence specification digest.
    pub evidence_spec_digest: Sha256Digest,
    /// Runner digest or typed absence.
    pub runner_digest: Option<Sha256Digest>,
    /// Adapter digest or typed absence.
    pub adapter_digest: Option<Sha256Digest>,
    /// Policy digest.
    pub policy_digest: Sha256Digest,
    /// Exact selected profiles.
    pub profile_values: BTreeMap<ProfileId, ProfileSelection>,
    /// Target configuration digest.
    pub target_configuration_digest: Sha256Digest,
    /// Runtime-facts digest or typed absence.
    pub runtime_facts_digest: Option<Sha256Digest>,
    /// Release-record digest or typed absence.
    pub release_record_digest: Option<Sha256Digest>,
    /// Trust-root and algorithm-policy digest.
    pub trust_config_digest: Sha256Digest,
    /// Producer identity.
    pub producer: ProducerRef,
    /// Exact tool version.
    pub tool_version: ToolVersion,
}

/// Freshness-key mismatch axis.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum FreshnessMismatch {
    /// Subject identity changed.
    Subject,
    /// Contract digest changed.
    Contract,
    /// Binding digest changed.
    Binding,
    /// Evidence specification changed.
    EvidenceSpecification,
    /// Runner identity changed.
    Runner,
    /// Adapter identity changed.
    Adapter,
    /// Policy digest changed.
    Policy,
    /// Profile values changed.
    Profiles,
    /// Target configuration changed.
    TargetConfiguration,
    /// Runtime facts changed.
    RuntimeFacts,
    /// Release record changed.
    ReleaseRecord,
    /// Trust configuration changed.
    TrustConfiguration,
    /// Producer identity changed.
    Producer,
    /// Tool version changed.
    ToolVersion,
}

/// Closed freshness result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FreshnessStatus {
    /// Exact key and age are valid.
    Fresh,
    /// A semantic key changed or the age ceiling elapsed.
    Stale,
    /// Required temporal input was missing or implausibly future-dated.
    Unknown,
}

/// Complete deterministic freshness analysis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FreshnessReport {
    /// Overall result.
    pub status: FreshnessStatus,
    /// Every changed semantic key axis.
    pub mismatches: BTreeSet<FreshnessMismatch>,
}

/// Evaluates exact semantic identity and age against an injected clock.
#[must_use]
pub fn evaluate_evidence_freshness(
    observed: &FreshnessKey,
    expected: &FreshnessKey,
    observed_at: Option<UtcInstant>,
    maximum_age: Option<DurationMillis>,
    evaluated_at: UtcInstant,
) -> FreshnessReport {
    let mismatches = key_mismatches(observed, expected);
    let Some(observed_at) = observed_at else {
        return FreshnessReport {
            status: FreshnessStatus::Unknown,
            mismatches,
        };
    };
    let Some(maximum_age) = maximum_age else {
        return FreshnessReport {
            status: FreshnessStatus::Unknown,
            mismatches,
        };
    };
    let observed_millis = instant_nanos(observed_at) / 1_000_000;
    let evaluated_millis = instant_nanos(evaluated_at) / 1_000_000;
    if observed_millis > evaluated_millis + CLOCK_TOLERANCE_MILLIS {
        return FreshnessReport {
            status: FreshnessStatus::Unknown,
            mismatches,
        };
    }
    let stale_by_age = observed_millis + i128::from(maximum_age.get()) < evaluated_millis;
    FreshnessReport {
        status: if mismatches.is_empty() && !stale_by_age {
            FreshnessStatus::Fresh
        } else {
            FreshnessStatus::Stale
        },
        mismatches,
    }
}

fn instant_nanos(value: UtcInstant) -> i128 {
    i128::from(value.unix_seconds()) * 1_000_000_000 + i128::from(value.subsec_nanos())
}

fn key_mismatches(observed: &FreshnessKey, expected: &FreshnessKey) -> BTreeSet<FreshnessMismatch> {
    [
        (
            observed.subject == expected.subject,
            FreshnessMismatch::Subject,
        ),
        (
            observed.contract_digest == expected.contract_digest,
            FreshnessMismatch::Contract,
        ),
        (
            observed.binding_digest == expected.binding_digest,
            FreshnessMismatch::Binding,
        ),
        (
            observed.evidence_spec_digest == expected.evidence_spec_digest,
            FreshnessMismatch::EvidenceSpecification,
        ),
        (
            observed.runner_digest == expected.runner_digest,
            FreshnessMismatch::Runner,
        ),
        (
            observed.adapter_digest == expected.adapter_digest,
            FreshnessMismatch::Adapter,
        ),
        (
            observed.policy_digest == expected.policy_digest,
            FreshnessMismatch::Policy,
        ),
        (
            observed.profile_values == expected.profile_values,
            FreshnessMismatch::Profiles,
        ),
        (
            observed.target_configuration_digest == expected.target_configuration_digest,
            FreshnessMismatch::TargetConfiguration,
        ),
        (
            observed.runtime_facts_digest == expected.runtime_facts_digest,
            FreshnessMismatch::RuntimeFacts,
        ),
        (
            observed.release_record_digest == expected.release_record_digest,
            FreshnessMismatch::ReleaseRecord,
        ),
        (
            observed.trust_config_digest == expected.trust_config_digest,
            FreshnessMismatch::TrustConfiguration,
        ),
        (
            observed.producer == expected.producer,
            FreshnessMismatch::Producer,
        ),
        (
            observed.tool_version == expected.tool_version,
            FreshnessMismatch::ToolVersion,
        ),
    ]
    .into_iter()
    .filter_map(|(matches, mismatch)| (!matches).then_some(mismatch))
    .collect()
}
