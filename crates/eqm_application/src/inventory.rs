//! Snapshot-bound independently verified inventory supporting facts.

use eqm_domain::{
    AdapterId, DurationMillis, EvidenceScopeSubject, EvidenceSubject, InventoryCompleteness,
    SelectorText, Sha256Digest, TargetId, TrustLevel, UtcInstant,
};
use eqm_engine::{ExposureFacts, ExposureReconciliation, ObservedExposure, reconcile_exposure};
use eqm_protocol::{EvidenceSubjectDto, InventoryDto};
use eqm_runner::{InventoryExposureInput, InventoryObservation};
use std::collections::BTreeSet;

/// Independent expected target subject, never inferred from imported inventory.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InventoryExpectation {
    /// Complete independently expected subject.
    pub subject: EvidenceSubject,
    /// Exact target whose inventory is expected.
    pub target: TargetId,
    /// Exact approved adapter.
    pub adapter: AdapterId,
    /// Locked executable/adapter identity.
    pub adapter_digest: Sha256Digest,
}

/// Explicit inventory eligibility policy; there are no ambient defaults.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InventoryPolicy {
    /// Inclusive maximum age at the captured evaluation instant.
    pub maximum_age: DurationMillis,
    /// Inclusive permitted future clock skew.
    pub future_skew: DurationMillis,
    /// Minimum independently established trust, never producer-claimed trust.
    pub minimum_trust: TrustLevel,
    /// Exact protected configuration checked by the configured verifier.
    pub trust_configuration_digest: Sha256Digest,
}

/// Immutable exact input to the explicitly configured verification capability.
pub struct InventoryVerificationRequest<'a> {
    bytes: &'a [u8],
    raw_digest: Sha256Digest,
    inventory: &'a InventoryDto,
    expectation: &'a InventoryExpectation,
    policy: InventoryPolicy,
    evaluated_at: UtcInstant,
}

impl InventoryVerificationRequest<'_> {
    /// Original bytes, including their original signed representation.
    #[must_use]
    pub const fn bytes(&self) -> &[u8] {
        self.bytes
    }
    /// Independently computed identity of those exact bytes.
    #[must_use]
    pub const fn raw_digest(&self) -> Sha256Digest {
        self.raw_digest
    }
    /// Locally validated claims; these are not a trust receipt.
    #[must_use]
    pub const fn inventory(&self) -> &InventoryDto {
        self.inventory
    }
    /// Independently expected subject and adapter authority.
    #[must_use]
    pub const fn expectation(&self) -> &InventoryExpectation {
        self.expectation
    }
    /// Exact configured age/skew/trust policy.
    #[must_use]
    pub const fn policy(&self) -> InventoryPolicy {
        self.policy
    }
    /// Captured instant for current eligibility and revocation checks.
    #[must_use]
    pub const fn evaluated_at(&self) -> UtcInstant {
        self.evaluated_at
    }
}

/// Independent verification did not establish current inventory eligibility.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum InventoryVerificationError {
    /// Supporting verification material or protected configuration is unavailable.
    Unavailable,
    /// Signature, issuer, identity, producer, builder, subject or execution rejected.
    Rejected,
    /// Current revocation/validity requirements rejected.
    Revoked,
    /// Independently established trust is below the configured requirement.
    InsufficientTrust,
}

/// Explicit read-only independent verification authority, without a default.
/// Success must check original signed bytes, permitted signer/issuer/producer/
/// builder/execution, exact independent subject and configured trust, current
/// time and revocation material. Even a low minimum trust requires independent
/// verification. Imported flags/receipts and parsed claims cannot implement it.
/// Implementations never fetch, execute, sign or mutate during preparation.
pub trait InventoryVerifier {
    /// Establishes eligibility for this exact immutable request only.
    fn verify_current_inventory(
        &self,
        request: &InventoryVerificationRequest<'_>,
    ) -> Result<(), InventoryVerificationError>;
}

/// Retained reason why a supporting inventory conclusion is unknown.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum InventoryFactReason {
    /// No validated inventory was supplied.
    MissingInventory,
    /// Original bytes were absent, over bound, malformed or substituted.
    Content,
    /// Independent expected subject/adapter context was absent.
    MissingExpectation,
    /// Subject lacks build/artifact axes or uses unsupported lossy scope.
    IncompleteSubject,
    /// Imported subject differs from independent authority.
    SubjectMismatch,
    /// Adapter or target differs from independent authority.
    AdapterMismatch,
    /// Explicit age/skew/trust policy was absent.
    MissingPolicy,
    /// Inventory generation time is invalid.
    GenerationTime,
    /// Inventory exceeds the exact maximum age.
    Stale,
    /// Inventory exceeds the exact allowed future skew.
    Future,
    /// No independent verifier was configured.
    MissingVerifier,
    /// Configured independent verification failed.
    Verification(InventoryVerificationError),
    /// Missing coordinate cannot be proven absent from incomplete inventory.
    IncompleteInventory,
}

/// Opaque supporting facts bound to one immutable evaluation and trust policy.
/// No deserialization or public constructor can grant a successful conclusion.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedInventoryFacts {
    observation: Option<InventoryObservation>,
    expectation: Option<InventoryExpectation>,
    policy: Option<InventoryPolicy>,
    evaluated_at: UtcInstant,
    raw_digest: Option<Sha256Digest>,
    reasons: BTreeSet<InventoryFactReason>,
    eligible: bool,
}

impl PreparedInventoryFacts {
    /// Exact independently expected identity, if configured.
    #[must_use]
    pub const fn expectation(&self) -> Option<&InventoryExpectation> {
        self.expectation.as_ref()
    }
    /// Exact policy context; this is not a reusable verification receipt.
    #[must_use]
    pub const fn policy(&self) -> Option<InventoryPolicy> {
        self.policy
    }
    /// Evaluation instant to which these conclusions belong.
    #[must_use]
    pub const fn evaluated_at(&self) -> UtcInstant {
        self.evaluated_at
    }
    /// Identity of original bytes, even when they are ineligible.
    #[must_use]
    pub const fn raw_digest(&self) -> Option<Sha256Digest> {
        self.raw_digest
    }
    /// Failed preparation prerequisites, retained together rather than discarded.
    #[must_use]
    pub const fn reasons(&self) -> &BTreeSet<InventoryFactReason> {
        &self.reasons
    }
    /// Whether independent verification established current eligibility here.
    #[must_use]
    pub const fn is_current_verified(&self) -> bool {
        self.eligible
    }
    /// Exact coordinate result and all reasons; incomplete absence is Unknown.
    #[must_use]
    pub fn fact(
        &self,
        kind: &SelectorText,
        key: &SelectorText,
    ) -> (ObservedExposure, BTreeSet<InventoryFactReason>) {
        let mut reasons = self.reasons.clone();
        if !self.eligible {
            return (ObservedExposure::Unknown, reasons);
        }
        let Some(observation) = &self.observation else {
            reasons.insert(InventoryFactReason::MissingInventory);
            return (ObservedExposure::Unknown, reasons);
        };
        if observation.inventory().is_some_and(|inventory| {
            inventory
                .entries
                .iter()
                .any(|entry| entry.kind == kind.as_str() && entry.key == key.as_str())
        }) {
            (ObservedExposure::True, reasons)
        } else if observation.completeness() == InventoryCompleteness::Complete {
            (ObservedExposure::False, reasons)
        } else {
            reasons.insert(InventoryFactReason::IncompleteInventory);
            (ObservedExposure::Unknown, reasons)
        }
    }
}

/// Prepares supporting claims without I/O, default policy, subject self-echo,
/// clock capture or execution. Claims and configured verification are separate.
#[must_use]
pub fn prepare_inventory_facts(
    observation: Option<&InventoryObservation>,
    bytes: Option<&[u8]>,
    expectation: Option<&InventoryExpectation>,
    policy: Option<InventoryPolicy>,
    evaluated_at: UtcInstant,
    verifier: Option<&dyn InventoryVerifier>,
) -> PreparedInventoryFacts {
    let mut reasons = BTreeSet::new();
    let raw_digest = bytes
        .filter(|bytes| bytes.len() <= 16 * 1024 * 1024)
        .map(Sha256Digest::hash_content);
    let inventory = observation.and_then(InventoryObservation::inventory);
    if inventory.is_none() {
        reasons.insert(InventoryFactReason::MissingInventory);
    }
    if bytes.is_none_or(|bytes| {
        let parsed = InventoryDto::from_json(bytes).ok();
        parsed.is_none() || parsed.as_ref() != inventory
    }) || raw_digest.is_none()
    {
        reasons.insert(InventoryFactReason::Content);
    }
    if let Some(expected) = expectation {
        if !matches!(expected.subject.scope(), EvidenceScopeSubject::Target(target) if target == &expected.target)
            || expected.subject.build_id().is_none()
            || expected.subject.artifact_digest().is_none()
        {
            reasons.insert(InventoryFactReason::IncompleteSubject);
        }
        if let Some(inventory) = inventory {
            if inventory.subject != EvidenceSubjectDto::from(&expected.subject) {
                reasons.insert(InventoryFactReason::SubjectMismatch);
            }
            if inventory.adapter != expected.adapter.as_str()
                || inventory.adapter_digest != expected.adapter_digest.to_string()
                || inventory.target != expected.target.as_str()
            {
                reasons.insert(InventoryFactReason::AdapterMismatch);
            }
        }
    } else {
        reasons.insert(InventoryFactReason::MissingExpectation);
    }
    if let Some(policy) = policy {
        if let Some(inventory) = inventory {
            if let Ok(generated) = inventory.generated_at.parse::<UtcInstant>() {
                let age = instant_nanos(evaluated_at) - instant_nanos(generated);
                if age > i128::from(policy.maximum_age.get()) * 1_000_000 {
                    reasons.insert(InventoryFactReason::Stale);
                }
                if -age > i128::from(policy.future_skew.get()) * 1_000_000 {
                    reasons.insert(InventoryFactReason::Future);
                }
            } else {
                reasons.insert(InventoryFactReason::GenerationTime);
            }
        }
    } else {
        reasons.insert(InventoryFactReason::MissingPolicy);
    }
    if verifier.is_none() {
        reasons.insert(InventoryFactReason::MissingVerifier);
    }
    if reasons.is_empty()
        && let (
            Some(bytes),
            Some(raw_digest),
            Some(inventory),
            Some(expectation),
            Some(policy),
            Some(verifier),
        ) = (bytes, raw_digest, inventory, expectation, policy, verifier)
        && let Err(error) = verifier.verify_current_inventory(&InventoryVerificationRequest {
            bytes,
            raw_digest,
            inventory,
            expectation,
            policy,
            evaluated_at,
        })
    {
        reasons.insert(InventoryFactReason::Verification(error));
    }
    let eligible = reasons.is_empty();
    PreparedInventoryFacts {
        observation: observation.cloned(),
        expectation: expectation.cloned(),
        policy,
        evaluated_at,
        raw_digest,
        reasons,
        eligible,
    }
}

fn instant_nanos(instant: UtcInstant) -> i128 {
    i128::from(instant.unix_seconds()) * 1_000_000_000 + i128::from(instant.subsec_nanos())
}

/// Actual checked reconciliation consumes opaque prepared facts, never raw claims.
#[must_use]
pub fn reconcile_prepared_inventory_exposure(
    input: InventoryExposureInput,
    inventory: &PreparedInventoryFacts,
    kind: &SelectorText,
    key: &SelectorText,
) -> ExposureReconciliation {
    reconcile_exposure(ExposureFacts {
        expected: input.expected,
        declared: input.declared,
        discovered: inventory.fact(kind, key).0,
        enabled: input.enabled,
        released: input.released,
        conformant: input.conformant,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use eqm_domain::{AdapterDefinition, AdapterLimits, PositiveCount, Revision};
    use eqm_engine::{ConformanceFact, ExpectedExposure, ExposureComparison};
    use eqm_protocol::InventoryEntryDto;
    use eqm_runner::validate_inventory_claim;
    use std::cell::Cell;
    use std::collections::BTreeMap;
    use std::error::Error;

    struct Fixture {
        definition: AdapterDefinition,
        expected: InventoryExpectation,
        policy: InventoryPolicy,
        inventory: InventoryDto,
        evaluated_at: UtcInstant,
    }

    impl Fixture {
        fn new() -> Result<Self, Box<dyn Error>> {
            let expected = InventoryExpectation {
                subject: EvidenceSubject::new(
                    "https://example.com/product".parse()?,
                    Sha256Digest::hash_content(b"repository"),
                    EvidenceScopeSubject::Target(TargetId::new("web")?),
                    "a".repeat(40).parse()?,
                    Some(SelectorText::new("build-fixture")?),
                    Some(Sha256Digest::hash_content(b"artifact")),
                    Sha256Digest::hash_content(b"target configuration"),
                ),
                target: TargetId::new("web")?,
                adapter: AdapterId::new("adapter.test")?,
                adapter_digest: Sha256Digest::hash_content(b"adapter"),
            };
            let definition = AdapterDefinition::new(
                expected.adapter.clone(),
                SelectorText::new("1.0.0")?,
                "https://example.com/adapter".parse()?,
                expected.adapter_digest,
                Revision::new(1)?,
                InventoryCompleteness::Complete,
                AdapterLimits::new(
                    DurationMillis::new(1_000)?,
                    PositiveCount::new(4_096)?,
                    PositiveCount::new(16_384)?,
                    PositiveCount::new(10)?,
                    PositiveCount::new(8)?,
                )?,
            )?;
            let inventory = InventoryDto {
                schema: eqm_protocol::INVENTORY_SCHEMA.to_string(),
                adapter: "adapter.test".to_owned(),
                adapter_digest: Sha256Digest::hash_content(b"adapter").to_string(),
                // Authored fixture assertions are separate from acceptance.
                subject: EvidenceSubjectDto {
                    repository: "https://example.com/product".to_owned(),
                    repository_id_digest: Sha256Digest::hash_content(b"repository").to_string(),
                    scope: eqm_protocol::ScopeSubjectDto::Target {
                        target: "web".to_owned(),
                    },
                    source_commit: "a".repeat(40),
                    build_id: Some("build-fixture".to_owned()),
                    artifact_digest: Some(Sha256Digest::hash_content(b"artifact").to_string()),
                    target_configuration_digest: Sha256Digest::hash_content(
                        b"target configuration",
                    )
                    .to_string(),
                },
                target: "web".to_owned(),
                generated_at: "2026-10-08T00:00:00Z".to_owned(),
                completeness: "complete".to_owned(),
                entries: vec![InventoryEntryDto {
                    kind: "route".to_owned(),
                    key: "/signup".to_owned(),
                    attributes: BTreeMap::new(),
                    source: "src/signup.rs".to_owned(),
                }],
                diagnostics: Vec::new(),
                inventory_digest: String::new(),
            };
            Ok(Self {
                definition,
                expected,
                inventory,
                policy: InventoryPolicy {
                    maximum_age: DurationMillis::new(1_000)?,
                    future_skew: DurationMillis::new(100)?,
                    minimum_trust: TrustLevel::SignedCi,
                    trust_configuration_digest: Sha256Digest::hash_content(
                        b"protected configuration",
                    ),
                },
                evaluated_at: "2026-10-08T00:00:01Z".parse()?,
            })
        }

        fn claim(&self) -> Result<(InventoryObservation, Vec<u8>), Box<dyn Error>> {
            let mut inventory = self.inventory.clone();
            let mut value = serde_json::to_value(&inventory)?;
            value
                .as_object_mut()
                .ok_or("object")?
                .remove("inventory_digest");
            // This literal ASCII-key/string/integer fixture has the same bytes
            // under sorted compact JSON and JCS. No production canonicalizer.
            inventory.inventory_digest =
                Sha256Digest::hash_content(&serde_json::to_vec(&value)?).to_string();
            let bytes = serde_json::to_vec(&inventory)?;
            Ok((
                validate_inventory_claim(&self.definition, &self.expected.target, inventory)?,
                bytes,
            ))
        }

        fn verifier(&self, bytes: &[u8]) -> SyntheticVerifier {
            SyntheticVerifier {
                raw_digest: Sha256Digest::hash_content(bytes),
                expected: self.expected.clone(),
                policy: self.policy,
                evaluated_at: self.evaluated_at,
                rejection: None,
                calls: Cell::new(0),
            }
        }

        fn prepare(
            &self,
            observation: &InventoryObservation,
            bytes: &[u8],
            verifier: Option<&dyn InventoryVerifier>,
        ) -> PreparedInventoryFacts {
            prepare_inventory_facts(
                Some(observation),
                Some(bytes),
                Some(&self.expected),
                Some(self.policy),
                self.evaluated_at,
                verifier,
            )
        }
    }

    // Independent deterministic protected stand-in is inaccessible to production.
    struct SyntheticVerifier {
        raw_digest: Sha256Digest,
        expected: InventoryExpectation,
        policy: InventoryPolicy,
        evaluated_at: UtcInstant,
        rejection: Option<InventoryVerificationError>,
        calls: Cell<usize>,
    }
    impl InventoryVerifier for SyntheticVerifier {
        fn verify_current_inventory(
            &self,
            request: &InventoryVerificationRequest<'_>,
        ) -> Result<(), InventoryVerificationError> {
            self.calls.set(self.calls.get() + 1);
            if request.raw_digest() != self.raw_digest
                || Sha256Digest::hash_content(request.bytes()) != self.raw_digest
                || request.expectation() != &self.expected
                || request.policy() != self.policy
                || request.evaluated_at() != self.evaluated_at
            {
                return Err(InventoryVerificationError::Rejected);
            }
            self.rejection.map_or(Ok(()), Err)
        }
    }

    fn reconcile(
        prepared: &PreparedInventoryFacts,
        key: &str,
    ) -> Result<ExposureReconciliation, Box<dyn Error>> {
        Ok(reconcile_prepared_inventory_exposure(
            InventoryExposureInput {
                expected: ExpectedExposure::Prohibited,
                declared: ObservedExposure::Unknown,
                enabled: ObservedExposure::Unknown,
                released: ObservedExposure::Unknown,
                conformant: ConformanceFact::Unknown,
            },
            prepared,
            &SelectorText::new("route")?,
            &SelectorText::new(key)?,
        ))
    }

    #[test]
    fn checked_presence_and_complete_absence_share_real_preparation() -> Result<(), Box<dyn Error>>
    {
        let fixture = Fixture::new()?;
        let (observation, bytes) = fixture.claim()?;
        assert!(!observation.can_prove_absence());
        let verifier = fixture.verifier(&bytes);
        let prepared = fixture.prepare(&observation, &bytes, Some(&verifier));
        assert!(prepared.is_current_verified());
        assert_eq!(verifier.calls.get(), 1);
        assert_eq!(prepared.expectation(), Some(&fixture.expected));
        assert_eq!(prepared.policy(), Some(fixture.policy));
        assert_eq!(prepared.evaluated_at(), fixture.evaluated_at);
        assert_eq!(
            prepared.raw_digest(),
            Some(Sha256Digest::hash_content(&bytes))
        );
        assert_eq!(
            reconcile(&prepared, "/signup")?.facts.discovered,
            ObservedExposure::True
        );
        let absence = reconcile(&prepared, "/missing")?;
        assert_eq!(absence.facts.discovered, ObservedExposure::False);
        assert_eq!(absence.discovered, ExposureComparison::Match);
        assert_eq!(absence.facts.enabled, ObservedExposure::Unknown);
        Ok(())
    }

    #[test]
    fn partial_inventory_supports_verified_presence_but_never_absence() -> Result<(), Box<dyn Error>>
    {
        for completeness in ["partial", "unknown"] {
            let mut fixture = Fixture::new()?;
            fixture.inventory.completeness = completeness.to_owned();
            let (observation, bytes) = fixture.claim()?;
            let verifier = fixture.verifier(&bytes);
            let prepared = fixture.prepare(&observation, &bytes, Some(&verifier));
            assert!(prepared.is_current_verified());
            assert_eq!(
                reconcile(&prepared, "/signup")?.facts.discovered,
                ObservedExposure::True
            );
            let (fact, reasons) = prepared.fact(
                &SelectorText::new("route")?,
                &SelectorText::new("/missing")?,
            );
            assert_eq!(fact, ObservedExposure::Unknown);
            assert!(reasons.contains(&InventoryFactReason::IncompleteInventory));
        }
        Ok(())
    }

    #[test]
    fn every_missing_prerequisite_preserves_unknown_and_retained_reasons()
    -> Result<(), Box<dyn Error>> {
        let fixture = Fixture::new()?;
        let (observation, bytes) = fixture.claim()?;
        let verifier = fixture.verifier(&bytes);
        for (observation, bytes, expected, policy, verifier, reason) in [
            (
                None,
                Some(bytes.as_slice()),
                Some(&fixture.expected),
                Some(fixture.policy),
                Some(&verifier as &dyn InventoryVerifier),
                InventoryFactReason::MissingInventory,
            ),
            (
                Some(&observation),
                None,
                Some(&fixture.expected),
                Some(fixture.policy),
                Some(&verifier),
                InventoryFactReason::Content,
            ),
            (
                Some(&observation),
                Some(bytes.as_slice()),
                None,
                Some(fixture.policy),
                Some(&verifier),
                InventoryFactReason::MissingExpectation,
            ),
            (
                Some(&observation),
                Some(bytes.as_slice()),
                Some(&fixture.expected),
                None,
                Some(&verifier),
                InventoryFactReason::MissingPolicy,
            ),
            (
                Some(&observation),
                Some(bytes.as_slice()),
                Some(&fixture.expected),
                Some(fixture.policy),
                None,
                InventoryFactReason::MissingVerifier,
            ),
        ] {
            let prepared = prepare_inventory_facts(
                observation,
                bytes,
                expected,
                policy,
                fixture.evaluated_at,
                verifier,
            );
            assert!(prepared.reasons().contains(&reason));
            assert!(!prepared.is_current_verified());
            for key in ["/signup", "/missing"] {
                assert_eq!(
                    reconcile(&prepared, key)?.facts.discovered,
                    ObservedExposure::Unknown
                );
            }
        }
        assert_eq!(verifier.calls.get(), 0);
        Ok(())
    }

    #[test]
    fn age_and_future_skew_keep_inclusive_nanosecond_boundaries() -> Result<(), Box<dyn Error>> {
        for (generated, expected_reason) in [
            ("2026-10-08T00:00:00Z", None),
            (
                "2026-10-07T23:59:59.999999999Z",
                Some(InventoryFactReason::Stale),
            ),
            ("2026-10-08T00:00:01.100Z", None),
            (
                "2026-10-08T00:00:01.100000001Z",
                Some(InventoryFactReason::Future),
            ),
        ] {
            let mut fixture = Fixture::new()?;
            fixture.inventory.generated_at = generated.to_owned();
            let (observation, bytes) = fixture.claim()?;
            let verifier = fixture.verifier(&bytes);
            let prepared = fixture.prepare(&observation, &bytes, Some(&verifier));
            assert_eq!(prepared.is_current_verified(), expected_reason.is_none());
            if let Some(reason) = expected_reason {
                assert!(prepared.reasons().contains(&reason));
                assert_eq!(verifier.calls.get(), 0);
                assert_eq!(
                    reconcile(&prepared, "/missing")?.facts.discovered,
                    ObservedExposure::Unknown
                );
            } else {
                assert_eq!(verifier.calls.get(), 1);
            }
        }
        Ok(())
    }

    #[test]
    fn failed_revoked_or_insufficient_verification_never_proves_a_fact()
    -> Result<(), Box<dyn Error>> {
        let fixture = Fixture::new()?;
        let (observation, bytes) = fixture.claim()?;
        for error in [
            InventoryVerificationError::Unavailable,
            InventoryVerificationError::Rejected,
            InventoryVerificationError::Revoked,
            InventoryVerificationError::InsufficientTrust,
        ] {
            let mut verifier = fixture.verifier(&bytes);
            verifier.rejection = Some(error);
            let prepared = fixture.prepare(&observation, &bytes, Some(&verifier));
            assert!(
                prepared
                    .reasons()
                    .contains(&InventoryFactReason::Verification(error))
            );
            for key in ["/signup", "/missing"] {
                assert_eq!(
                    reconcile(&prepared, key)?.facts.discovered,
                    ObservedExposure::Unknown
                );
            }
        }
        Ok(())
    }

    #[test]
    fn independent_subject_and_adapter_axes_cannot_be_substituted() -> Result<(), Box<dyn Error>> {
        let fixture = Fixture::new()?;
        let (observation, bytes) = fixture.claim()?;
        let verifier = fixture.verifier(&bytes);
        for axis in 0..9 {
            let mut expected = fixture.expected.clone();
            let subject = &fixture.expected.subject;
            expected.subject = EvidenceSubject::new(
                if axis == 0 {
                    "https://example.com/foreign".parse()?
                } else {
                    subject.repository().clone()
                },
                if axis == 1 {
                    Sha256Digest::hash_content(b"foreign repo")
                } else {
                    subject.repository_id_digest()
                },
                if axis == 2 {
                    EvidenceScopeSubject::Target(TargetId::new("ios")?)
                } else {
                    subject.scope().clone()
                },
                if axis == 3 {
                    "b".repeat(40).parse()?
                } else {
                    subject.source_commit().clone()
                },
                if axis == 4 {
                    Some(SelectorText::new("foreign-build")?)
                } else {
                    subject.build_id().cloned()
                },
                if axis == 5 {
                    Some(Sha256Digest::hash_content(b"foreign artifact"))
                } else {
                    subject.artifact_digest()
                },
                if axis == 6 {
                    Sha256Digest::hash_content(b"foreign config")
                } else {
                    subject.target_configuration_digest()
                },
            );
            if axis == 7 {
                expected.adapter = AdapterId::new("adapter.foreign")?;
            }
            if axis == 8 {
                expected.adapter_digest = Sha256Digest::hash_content(b"foreign adapter");
            }
            let prepared = prepare_inventory_facts(
                Some(&observation),
                Some(&bytes),
                Some(&expected),
                Some(fixture.policy),
                fixture.evaluated_at,
                Some(&verifier),
            );
            assert!(!prepared.is_current_verified(), "axis {axis}");
            assert_eq!(
                reconcile(&prepared, "/missing")?.facts.discovered,
                ObservedExposure::Unknown
            );
        }
        assert_eq!(verifier.calls.get(), 0);
        Ok(())
    }

    #[test]
    fn original_bytes_policy_and_evaluation_context_remain_bound() -> Result<(), Box<dyn Error>> {
        let fixture = Fixture::new()?;
        let (observation, bytes) = fixture.claim()?;
        let verifier = fixture.verifier(&bytes);
        let mut changed = bytes.clone();
        changed.push(b' ');
        let altered_transport = fixture.prepare(&observation, &changed, Some(&verifier));
        assert!(
            altered_transport
                .reasons()
                .contains(&InventoryFactReason::Verification(
                    InventoryVerificationError::Rejected
                ))
        );
        let mut substituted = Fixture::new()?;
        substituted.inventory.entries.clear();
        let (_, foreign_bytes) = substituted.claim()?;
        let foreign = fixture.prepare(&observation, &foreign_bytes, Some(&verifier));
        assert!(foreign.reasons().contains(&InventoryFactReason::Content));
        let mut policy = fixture.policy;
        policy.trust_configuration_digest = Sha256Digest::hash_content(b"foreign trust config");
        let foreign = prepare_inventory_facts(
            Some(&observation),
            Some(&bytes),
            Some(&fixture.expected),
            Some(policy),
            fixture.evaluated_at,
            Some(&verifier),
        );
        assert!(
            foreign
                .reasons()
                .contains(&InventoryFactReason::Verification(
                    InventoryVerificationError::Rejected
                ))
        );
        policy = fixture.policy;
        policy.minimum_trust = TrustLevel::UntrustedLocal;
        let lower = prepare_inventory_facts(
            Some(&observation),
            Some(&bytes),
            Some(&fixture.expected),
            Some(policy),
            fixture.evaluated_at,
            Some(&verifier),
        );
        assert!(!lower.is_current_verified());
        let later = prepare_inventory_facts(
            Some(&observation),
            Some(&bytes),
            Some(&fixture.expected),
            Some(fixture.policy),
            "2026-10-08T00:00:01.000000001Z".parse()?,
            Some(&verifier),
        );
        assert!(later.reasons().contains(&InventoryFactReason::Stale));
        Ok(())
    }

    #[test]
    fn legacy_valid_missing_subject_axes_and_lossy_scopes_cannot_authorize()
    -> Result<(), Box<dyn Error>> {
        for axis in 0..5 {
            let mut fixture = Fixture::new()?;
            let subject = fixture.expected.subject.clone();
            let scope = match axis {
                2 => EvidenceScopeSubject::Provider(eqm_domain::ProviderId::new("provider.test")?),
                3 => EvidenceScopeSubject::target_set(vec![
                    TargetId::new("web")?,
                    TargetId::new("ios")?,
                ])?,
                _ => subject.scope().clone(),
            };
            fixture.expected.subject = EvidenceSubject::new(
                subject.repository().clone(),
                subject.repository_id_digest(),
                scope,
                subject.source_commit().clone(),
                if axis == 0 {
                    None
                } else {
                    subject.build_id().cloned()
                },
                if axis == 1 {
                    None
                } else {
                    subject.artifact_digest()
                },
                subject.target_configuration_digest(),
            );
            fixture.inventory.subject = EvidenceSubjectDto::from(&fixture.expected.subject);
            let (observation, bytes) = fixture.claim()?;
            if axis == 4 {
                fixture.expected.target = TargetId::new("ios")?;
            }
            let verifier = fixture.verifier(&bytes);
            let prepared = fixture.prepare(&observation, &bytes, Some(&verifier));
            assert!(
                prepared
                    .reasons()
                    .contains(&InventoryFactReason::IncompleteSubject)
            );
            for key in ["/signup", "/missing"] {
                assert_eq!(
                    reconcile(&prepared, key)?.facts.discovered,
                    ObservedExposure::Unknown
                );
            }
            assert_eq!(verifier.calls.get(), 0);
        }
        Ok(())
    }

    #[test]
    fn original_byte_ceiling_is_inclusive_without_truncation() -> Result<(), Box<dyn Error>> {
        let fixture = Fixture::new()?;
        let (observation, original) = fixture.claim()?;
        for size in [16 * 1024 * 1024 - 1, 16 * 1024 * 1024, 16 * 1024 * 1024 + 1] {
            let mut bytes = original.clone();
            bytes.resize(size, b' ');
            let verifier = fixture.verifier(&bytes);
            let prepared = fixture.prepare(&observation, &bytes, Some(&verifier));
            assert_eq!(prepared.is_current_verified(), size <= 16 * 1024 * 1024);
            assert_eq!(verifier.calls.get(), usize::from(size <= 16 * 1024 * 1024));
            if size > 16 * 1024 * 1024 {
                assert!(prepared.reasons().contains(&InventoryFactReason::Content));
                assert_eq!(prepared.raw_digest(), None);
            } else {
                assert_eq!(
                    prepared.raw_digest(),
                    Some(Sha256Digest::hash_content(&bytes))
                );
            }
        }
        Ok(())
    }

    #[test]
    fn malformed_and_duplicate_original_bytes_never_reach_verification()
    -> Result<(), Box<dyn Error>> {
        let fixture = Fixture::new()?;
        let (observation, original) = fixture.claim()?;
        let text = std::str::from_utf8(&original)?;
        let duplicate = format!(
            "{{\"schema\":\"{}\",{}",
            eqm_protocol::INVENTORY_SCHEMA,
            text.strip_prefix('{').ok_or("object")?
        )
        .into_bytes();
        for bytes in [b"{".to_vec(), duplicate] {
            let verifier = fixture.verifier(&bytes);
            let prepared = fixture.prepare(&observation, &bytes, Some(&verifier));
            assert!(prepared.reasons().contains(&InventoryFactReason::Content));
            for key in ["/signup", "/missing"] {
                assert_eq!(
                    reconcile(&prepared, key)?.facts.discovered,
                    ObservedExposure::Unknown
                );
            }
            assert_eq!(verifier.calls.get(), 0);
        }
        Ok(())
    }
}
