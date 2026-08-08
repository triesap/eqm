//! Exact release-subject and release-gate evaluation.

use crate::{FacetStatus, TargetConformance};
use eqm_domain::{
    AppVersion, BuildNumber, ReleaseChannel, ReleaseRecord, Sha256Digest, SourceCommit, TargetId,
    TrustLevel, UtcInstant, WaiverId,
};
use std::collections::BTreeSet;

/// Immutable release subject tuple.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseSubject {
    /// Target identity.
    pub target: TargetId,
    /// Application version.
    pub app_version: AppVersion,
    /// Build number.
    pub build_number: BuildNumber,
    /// Exact source commit.
    pub source_commit: SourceCommit,
    /// Exact artifact digest.
    pub artifact_digest: Sha256Digest,
    /// Exact channel identity.
    pub channel: ReleaseChannel,
}

impl From<&ReleaseRecord> for ReleaseSubject {
    fn from(value: &ReleaseRecord) -> Self {
        Self {
            target: value.target().clone(),
            app_version: value.app_version().clone(),
            build_number: value.build_number().clone(),
            source_commit: value.source_commit().clone(),
            artifact_digest: value.artifact_digest(),
            channel: value.channel(),
        }
    }
}

/// Complete digest-bound release evaluation context.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReleaseContext {
    /// Contract digest.
    pub contract_digest: Sha256Digest,
    /// Policy digest.
    pub policy_digest: Sha256Digest,
    /// Exact profile-value digest.
    pub profile_values_digest: Sha256Digest,
    /// Evidence-set digest.
    pub evidence_set_digest: Sha256Digest,
    /// Runtime-facts digest.
    pub runtime_facts_digest: Sha256Digest,
    /// Trust-configuration digest.
    pub trust_config_digest: Sha256Digest,
    /// Exact release-record digest.
    pub release_record_digest: Sha256Digest,
    /// Injected evaluation clock.
    pub evaluated_at: UtcInstant,
}

/// One required release check.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReleaseCheck {
    /// Exact required state matches.
    Match,
    /// A valid visible waiver covers the deviation.
    Waived,
    /// Complete input proves a mismatch.
    Mismatch,
    /// Input is absent, stale, ambiguous, invalid, or unverifiable.
    Unknown,
}

/// Complete prepared release-gate input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseGateInput {
    /// Expected immutable subject.
    pub expected_subject: ReleaseSubject,
    /// Observed immutable subject, absent when no valid record exists.
    pub observed_subject: Option<ReleaseSubject>,
    /// Whether the release-record envelope and digest verified.
    pub release_record_verified: bool,
    /// Expected exact context.
    pub expected_context: ReleaseContext,
    /// Observed exact context.
    pub observed_context: Option<ReleaseContext>,
    /// Independent required exposure comparisons.
    pub exposure: Vec<ReleaseCheck>,
    /// Complete target conformance.
    pub conformance: Option<TargetConformance>,
    /// All required release facet statuses.
    pub release_facets: Vec<FacetStatus>,
    /// Independently verified effective trust.
    pub effective_trust: Option<TrustLevel>,
    /// Visible waivers contributing to conditional state.
    pub waivers: BTreeSet<WaiverId>,
}

/// Closed exact release gate result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReleaseGateStatus {
    /// Every exact release precondition passed.
    Pass,
    /// Every deviation is validly waived and visible.
    Conditional,
    /// Complete exact input proves an unwaived mismatch.
    Fail,
    /// Required input is absent, mismatched, invalid, stale, or unverifiable.
    Unknown,
}

/// Evaluates one exact release gate with a default `signed_ci` trust floor.
#[must_use]
pub fn evaluate_release_gate(input: &ReleaseGateInput) -> ReleaseGateStatus {
    if !input.release_record_verified
        || input.observed_subject.as_ref() != Some(&input.expected_subject)
        || input.observed_context != Some(input.expected_context)
        || input.conformance.is_none()
        || input.effective_trust != Some(TrustLevel::SignedCi)
        || input.exposure.contains(&ReleaseCheck::Unknown)
        || input.release_facets.iter().any(|status| {
            matches!(
                status,
                FacetStatus::Unknown | FacetStatus::Unstable | FacetStatus::Stale
            )
        })
    {
        return ReleaseGateStatus::Unknown;
    }
    if input.exposure.contains(&ReleaseCheck::Mismatch)
        || input.conformance == Some(TargetConformance::Nonconformant)
        || input
            .release_facets
            .iter()
            .any(|status| matches!(status, FacetStatus::Failed | FacetStatus::Missing))
    {
        return ReleaseGateStatus::Fail;
    }
    let conditional = input.exposure.contains(&ReleaseCheck::Waived)
        || input.conformance == Some(TargetConformance::ConditionallyConformant)
        || input.release_facets.contains(&FacetStatus::Waived);
    if conditional {
        if input.waivers.is_empty() {
            ReleaseGateStatus::Unknown
        } else {
            ReleaseGateStatus::Conditional
        }
    } else {
        ReleaseGateStatus::Pass
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    fn digest(value: u8) -> Sha256Digest {
        Sha256Digest::from_bytes([value; 32])
    }

    fn base() -> Result<ReleaseGateInput, Box<dyn Error>> {
        let subject = ReleaseSubject {
            target: TargetId::new("web")?,
            app_version: AppVersion::new("1.2.3")?,
            build_number: BuildNumber::new("42")?,
            source_commit: "a".repeat(40).parse()?,
            artifact_digest: digest(1),
            channel: ReleaseChannel::Production,
        };
        let context = ReleaseContext {
            contract_digest: digest(2),
            policy_digest: digest(3),
            profile_values_digest: digest(4),
            evidence_set_digest: digest(5),
            runtime_facts_digest: digest(6),
            trust_config_digest: digest(7),
            release_record_digest: digest(8),
            evaluated_at: "2026-08-07T12:00:00Z".parse()?,
        };
        Ok(ReleaseGateInput {
            expected_subject: subject.clone(),
            observed_subject: Some(subject),
            release_record_verified: true,
            expected_context: context,
            observed_context: Some(context),
            exposure: vec![ReleaseCheck::Match],
            conformance: Some(TargetConformance::Conformant),
            release_facets: vec![FacetStatus::Satisfied],
            effective_trust: Some(TrustLevel::SignedCi),
            waivers: BTreeSet::new(),
        })
    }

    #[test]
    fn pass_conditional_fail_and_unknown_bind_exact_subject_and_context()
    -> Result<(), Box<dyn Error>> {
        assert_eq!(evaluate_release_gate(&base()?), ReleaseGateStatus::Pass);
        let mut conditional = base()?;
        conditional.release_facets = vec![FacetStatus::Waived];
        conditional
            .waivers
            .insert(eqm_domain::WaiverId::new("waiver.release")?);
        assert_eq!(
            evaluate_release_gate(&conditional),
            ReleaseGateStatus::Conditional
        );
        let mut failed = base()?;
        failed.exposure = vec![ReleaseCheck::Mismatch];
        assert_eq!(evaluate_release_gate(&failed), ReleaseGateStatus::Fail);
        let mut wrong_build = base()?;
        wrong_build
            .observed_subject
            .as_mut()
            .ok_or("subject missing")?
            .build_number = BuildNumber::new("43")?;
        assert_eq!(
            evaluate_release_gate(&wrong_build),
            ReleaseGateStatus::Unknown
        );
        let mut weak_trust = base()?;
        weak_trust.effective_trust = Some(TrustLevel::TrustedCi);
        assert_eq!(
            evaluate_release_gate(&weak_trust),
            ReleaseGateStatus::Unknown
        );
        let mut wrong_context = base()?;
        wrong_context
            .observed_context
            .as_mut()
            .ok_or("context missing")?
            .policy_digest = digest(9);
        assert_eq!(
            evaluate_release_gate(&wrong_context),
            ReleaseGateStatus::Unknown
        );
        Ok(())
    }

    #[test]
    fn every_gate_precondition_and_terminal_branch_is_explicit() -> Result<(), Box<dyn Error>> {
        let mut cases = Vec::new();

        let mut value = base()?;
        value.release_record_verified = false;
        cases.push((value, ReleaseGateStatus::Unknown));
        let mut value = base()?;
        value.observed_subject = None;
        cases.push((value, ReleaseGateStatus::Unknown));
        let mut value = base()?;
        value.observed_context = None;
        cases.push((value, ReleaseGateStatus::Unknown));
        let mut value = base()?;
        value.conformance = None;
        cases.push((value, ReleaseGateStatus::Unknown));
        let mut value = base()?;
        value.effective_trust = None;
        cases.push((value, ReleaseGateStatus::Unknown));
        let mut value = base()?;
        value.exposure = vec![ReleaseCheck::Unknown];
        cases.push((value, ReleaseGateStatus::Unknown));
        for status in [
            FacetStatus::Unknown,
            FacetStatus::Unstable,
            FacetStatus::Stale,
        ] {
            let mut value = base()?;
            value.release_facets = vec![status];
            cases.push((value, ReleaseGateStatus::Unknown));
        }

        let mut value = base()?;
        value.conformance = Some(TargetConformance::Nonconformant);
        cases.push((value, ReleaseGateStatus::Fail));
        for status in [FacetStatus::Failed, FacetStatus::Missing] {
            let mut value = base()?;
            value.release_facets = vec![status];
            cases.push((value, ReleaseGateStatus::Fail));
        }

        for configure in [
            |value: &mut ReleaseGateInput| value.exposure = vec![ReleaseCheck::Waived],
            |value: &mut ReleaseGateInput| {
                value.conformance = Some(TargetConformance::ConditionallyConformant);
            },
            |value: &mut ReleaseGateInput| value.release_facets = vec![FacetStatus::Waived],
        ] {
            let mut without_authority = base()?;
            configure(&mut without_authority);
            assert_eq!(
                evaluate_release_gate(&without_authority),
                ReleaseGateStatus::Unknown
            );
            without_authority
                .waivers
                .insert(WaiverId::new("waiver.release")?);
            cases.push((without_authority, ReleaseGateStatus::Conditional));
        }

        for (input, expected) in cases {
            assert_eq!(evaluate_release_gate(&input), expected);
        }
        Ok(())
    }
}
