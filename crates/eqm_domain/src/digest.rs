//! Fixed SHA-256 content and semantic digests.

use sha2::{Digest as _, Sha256};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

const PREFIX: &str = "sha256:";
const HEX_LENGTH: usize = 64;

/// An approved domain-separation label.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DigestDomain {
    /// One canonical EQM v1 fragment authority.
    Fragment,
    /// Finalized EQM v1 semantic graph identity.
    SemanticGraph,
    /// Complete finalized EQM v2 semantic graph identity.
    SemanticGraphV2,
    /// Exact authored scenario occurrence projection (v2).
    Scenario,
    /// Explicit requirement-to-scenario binding identity (v2).
    RequirementScenarioBindings,
    /// Frozen expected logical selection manifest (v2).
    ExpectedSelectionManifest,
    /// Complete pre-evidence obligation coordinate (v2).
    Obligation,
    /// Shared comparison-context identity (v2).
    ComparisonContext,
    /// Exact stored raw object bytes (v2).
    RawObject,
    /// Normalized semantic evidence assertion (v2).
    SemanticEvidence,
    /// Approved acquisition plan (v2).
    Plan,
    /// Authorized campaign (v2).
    Campaign,
    /// One immutable execution attempt (v2).
    Attempt,
    /// Frozen evaluation context (v2).
    EvaluationContext,
    /// Contract projection (v2).
    Contract,
    /// Binding projection (v2).
    Binding,
    /// Evidence specification projection (v2).
    EvidenceSpecification,
    /// Target configuration projection (v2).
    TargetConfiguration,
    /// Runner definition and executable projection (v2).
    Runner,
    /// Adapter projection (v2).
    Adapter,
    /// Policy and profile selection projection (v2).
    PolicySelection,
    /// Protected authority projection (v2).
    Authority,
    /// Trust configuration projection (v2).
    TrustConfiguration,
    /// Capability-participation rule projection (v2).
    Participation,
    /// Exact multi-participant subject manifest (v2).
    SubjectManifest,
    /// Validated provider definition projection (v2).
    ProviderConfiguration,
}

impl DigestDomain {
    /// Returns the exact bytes prepended before the zero delimiter.
    #[must_use]
    pub const fn as_bytes(self) -> &'static [u8] {
        match self {
            Self::Fragment => b"eqm:v1:fragment",
            Self::SemanticGraph => b"eqm:v1:semantic-graph",
            Self::SemanticGraphV2 => b"eqm:v2:semantic-graph",
            Self::Scenario => b"eqm:v2:scenario",
            Self::Obligation => b"eqm:v2:obligation",
            Self::ExpectedSelectionManifest => b"eqm:v2:expected-selection-manifest",
            Self::RequirementScenarioBindings => b"eqm:v2:requirement-scenario-bindings",
            Self::ComparisonContext => b"eqm:v2:comparison-context",
            Self::RawObject => b"eqm:v2:raw-object",
            Self::SemanticEvidence => b"eqm:v2:semantic-evidence",
            Self::Plan => b"eqm:v2:plan",
            Self::Campaign => b"eqm:v2:campaign",
            Self::Attempt => b"eqm:v2:attempt",
            Self::EvaluationContext => b"eqm:v2:evaluation-context",
            Self::Contract => b"eqm:v2:contract",
            Self::Binding => b"eqm:v2:binding",
            Self::EvidenceSpecification => b"eqm:v2:evidence-specification",
            Self::TargetConfiguration => b"eqm:v2:target-configuration",
            Self::Runner => b"eqm:v2:runner",
            Self::Adapter => b"eqm:v2:adapter",
            Self::PolicySelection => b"eqm:v2:policy-selection",
            Self::Authority => b"eqm:v2:authority",
            Self::TrustConfiguration => b"eqm:v2:trust-configuration",
            Self::Participation => b"eqm:v2:participation",
            Self::ProviderConfiguration => b"eqm:v2:provider-configuration",
            Self::SubjectManifest => b"eqm:v2:subject-manifest",
        }
    }
}

/// A fixed 32-byte SHA-256 digest.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Sha256Digest([u8; 32]);

impl Sha256Digest {
    /// Creates a digest from exact raw bytes.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Hashes raw content without a semantic domain label.
    #[must_use]
    pub fn hash_content(content: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(content);
        Self(hasher.finalize().into())
    }

    /// Hashes the approved domain label, zero delimiter, and content bytes.
    #[must_use]
    pub fn hash_domain(domain: DigestDomain, content: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(domain.as_bytes());
        hasher.update([0]);
        hasher.update(content);
        Self(hasher.finalize().into())
    }

    /// Returns the exact 32 bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Consumes the value and returns its exact bytes.
    #[must_use]
    pub const fn into_bytes(self) -> [u8; 32] {
        self.0
    }
}

impl Display for Sha256Digest {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(PREFIX)?;
        for byte in self.0 {
            write!(formatter, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl FromStr for Sha256Digest {
    type Err = DigestParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let Some(hex) = value.strip_prefix(PREFIX) else {
            return Err(DigestParseError::MissingPrefix);
        };
        if hex.len() != HEX_LENGTH {
            return Err(DigestParseError::WrongLength);
        }
        if hex.bytes().any(|byte| byte.is_ascii_uppercase()) {
            return Err(DigestParseError::UppercaseHex);
        }

        let mut bytes = [0_u8; 32];
        for (index, pair) in hex.as_bytes().chunks_exact(2).enumerate() {
            let high = decode_hex(pair[0]).ok_or(DigestParseError::InvalidHex)?;
            let low = decode_hex(pair[1]).ok_or(DigestParseError::InvalidHex)?;
            bytes[index] = (high << 4) | low;
        }
        Ok(Self(bytes))
    }
}

/// Defines a domain-specific digest identity.
///
/// Each newtype wraps a [`Sha256Digest`] produced only under its documented
/// domain, so an unrelated digest cannot be passed where a distinct identity
/// is required. There is deliberately no `Default`, no cross-type conversion
/// and no public field constructor.
macro_rules! define_identity_digest {
    ($(#[$meta:meta])* $name:ident, $domain:expr) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(Sha256Digest);

        impl $name {
            /// Hashes content under this identity's documented domain.
            #[must_use]
            pub fn hash_content(content: &[u8]) -> Self {
                Self(Sha256Digest::hash_domain($domain, content))
            }

            /// Returns the underlying digest for exact wire encoding.
            #[must_use]
            pub const fn as_digest(self) -> Sha256Digest {
                self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
                Display::fmt(&self.0, formatter)
            }
        }
    };
}

define_identity_digest!(
    /// Exact stored raw object bytes.
    RawObjectDigest,
    DigestDomain::RawObject
);
define_identity_digest!(
    /// Normalized semantic evidence assertion.
    SemanticEvidenceDigest,
    DigestDomain::SemanticEvidence
);
define_identity_digest!(
    /// Approved acquisition plan.
    PlanDigest,
    DigestDomain::Plan
);
define_identity_digest!(
    /// Authorized campaign.
    CampaignDigest,
    DigestDomain::Campaign
);
define_identity_digest!(
    /// One immutable execution attempt.
    AttemptDigest,
    DigestDomain::Attempt
);
define_identity_digest!(
    /// Frozen evaluation context.
    EvaluationContextDigest,
    DigestDomain::EvaluationContext
);
define_identity_digest!(
    /// Contract projection.
    ContractDigest,
    DigestDomain::Contract
);
define_identity_digest!(
    /// Binding projection.
    BindingDigest,
    DigestDomain::Binding
);
define_identity_digest!(
    /// Evidence-specification projection.
    EvidenceSpecificationDigest,
    DigestDomain::EvidenceSpecification
);
define_identity_digest!(
    /// Target-configuration projection.
    TargetConfigurationDigest,
    DigestDomain::TargetConfiguration
);
define_identity_digest!(
    /// Runner definition and executable projection.
    RunnerDigest,
    DigestDomain::Runner
);
define_identity_digest!(
    /// Adapter projection.
    AdapterDigest,
    DigestDomain::Adapter
);
define_identity_digest!(
    /// Policy and profile selection projection.
    PolicySelectionDigest,
    DigestDomain::PolicySelection
);
define_identity_digest!(
    /// Protected authority projection.
    AuthorityDigest,
    DigestDomain::Authority
);
define_identity_digest!(
    /// Trust-configuration projection.
    TrustConfigurationDigest,
    DigestDomain::TrustConfiguration
);

define_identity_digest!(
    /// Capability-participation rule projection.
    ParticipationDigest,
    DigestDomain::Participation
);

define_identity_digest!(
    /// Exact multi-participant subject manifest projection.
    SubjectManifestDigest,
    DigestDomain::SubjectManifest
);

define_identity_digest!(
    /// Complete finalized v2 semantic graph projection.
    SemanticGraphV2Digest,
    DigestDomain::SemanticGraphV2
);

define_identity_digest!(
    /// Authored v2 scenario occurrence projection.
    ScenarioDigest,
    DigestDomain::Scenario
);

define_identity_digest!(
    /// Shared comparison-context projection.
    ComparisonContextDigest,
    DigestDomain::ComparisonContext
);

define_identity_digest!(
    /// Validated provider definition projection.
    ProviderConfigurationDigest,
    DigestDomain::ProviderConfiguration
);

define_identity_digest!(
    /// Explicit requirement-to-scenario binding projection.
    ///
    /// This identity cannot substitute a scenario's own projection identity:
    /// ```compile_fail
    /// let scope = eqm_domain::RequirementScenarioBindingsDigest::hash_content(b"x");
    /// let _: eqm_domain::ScenarioDigest = scope;
    /// ```
    RequirementScenarioBindingsDigest,
    DigestDomain::RequirementScenarioBindings
);
define_identity_digest!(
    /// Distinct expected-selection identity; cannot substitute for a plan.
    /// ```compile_fail
    /// let expected = eqm_domain::ExpectedSelectionManifestDigest::hash_content(b"x");
    /// let _: eqm_domain::PlanDigest = expected;
    /// ```
    ExpectedSelectionManifestDigest,
    DigestDomain::ExpectedSelectionManifest
);

fn decode_hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}

/// SHA-256 wire parsing failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DigestParseError {
    /// The exact `sha256:` prefix was absent.
    MissingPrefix,
    /// The hexadecimal payload was not exactly 64 bytes.
    WrongLength,
    /// Uppercase hexadecimal is not canonical.
    UppercaseHex,
    /// The payload contained a non-hexadecimal byte.
    InvalidHex,
}

impl Display for DigestParseError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::MissingPrefix => "SHA-256 digest requires the sha256: prefix",
            Self::WrongLength => "SHA-256 digest requires 64 hexadecimal digits",
            Self::UppercaseHex => "SHA-256 digest requires lowercase hexadecimal",
            Self::InvalidHex => "SHA-256 digest contains invalid hexadecimal",
        })
    }
}

impl Error for DigestParseError {}

define_identity_digest!(
    /// Complete pre-evidence obligation coordinate, not release authorization.
    /// ```compile_fail
    /// let coordinate = eqm_domain::ObligationDigest::hash_content(b"coordinate");
    /// let _: eqm_domain::EvaluationContextDigest = coordinate;
    /// ```
    ObligationDigest,
    DigestDomain::Obligation
);

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    const EMPTY_GRAPH: &[u8] = br#"{"adapters":[],"bindings":[],"capabilities":[],"extensions":{},"fragments":[],"imports":[],"journeys":[],"policies":[],"profiles":[],"runners":[],"schema":"https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/protocol/semantic-graph.schema.json","surfaces":[],"targets":[],"waivers":[]}"#;

    #[test]
    fn content_hash_matches_standard_empty_vector() {
        assert_eq!(
            Sha256Digest::hash_content(b"").to_string(),
            "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn semantic_domain_matches_authority_vector() {
        assert_eq!(
            Sha256Digest::hash_domain(DigestDomain::SemanticGraph, EMPTY_GRAPH).to_string(),
            "sha256:d157825d4fbaafe9fa2b0313c4f2465a13cf30fb088a86f445192b042376d2b3"
        );
    }

    #[test]
    fn bytes_display_and_parse_round_trip() {
        let digest = Sha256Digest::from_bytes([0xab; 32]);
        let rendered = digest.to_string();
        assert_eq!(rendered.parse(), Ok(digest));
        assert_eq!(digest.as_bytes(), &[0xab; 32]);
        assert_eq!(digest.into_bytes(), [0xab; 32]);
    }

    #[test]
    fn malformed_digest_forms_fail_closed() {
        for (value, error) in [
            ("", DigestParseError::MissingPrefix),
            ("SHA256:00", DigestParseError::MissingPrefix),
            ("sha256:00", DigestParseError::WrongLength),
            (
                "sha256:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
                DigestParseError::UppercaseHex,
            ),
            (
                "sha256:gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg",
                DigestParseError::InvalidHex,
            ),
        ] {
            assert_eq!(value.parse::<Sha256Digest>(), Err(error));
        }
        let parsed = "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
            .parse::<Sha256Digest>();
        assert_eq!(
            parsed.map(Sha256Digest::into_bytes),
            Ok([
                0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
                0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67,
                0x89, 0xab, 0xcd, 0xef,
            ])
        );
        for error in [
            DigestParseError::MissingPrefix,
            DigestParseError::WrongLength,
            DigestParseError::UppercaseHex,
            DigestParseError::InvalidHex,
        ] {
            assert!(!error.to_string().is_empty());
        }
    }

    #[test]
    fn semantic_domain_is_not_raw_content_hashing() {
        assert_ne!(
            Sha256Digest::hash_domain(DigestDomain::SemanticGraph, EMPTY_GRAPH),
            Sha256Digest::hash_content(EMPTY_GRAPH)
        );
    }

    #[test]
    fn identity_domains_are_documented_and_distinct() {
        let domains = [
            DigestDomain::RawObject,
            DigestDomain::SemanticEvidence,
            DigestDomain::Plan,
            DigestDomain::Campaign,
            DigestDomain::Attempt,
            DigestDomain::EvaluationContext,
            DigestDomain::Contract,
            DigestDomain::Binding,
            DigestDomain::EvidenceSpecification,
            DigestDomain::TargetConfiguration,
            DigestDomain::Runner,
            DigestDomain::Adapter,
            DigestDomain::PolicySelection,
            DigestDomain::Authority,
            DigestDomain::TrustConfiguration,
            DigestDomain::SubjectManifest,
            DigestDomain::SemanticGraphV2,
            DigestDomain::Participation,
            DigestDomain::Scenario,
            DigestDomain::RequirementScenarioBindings,
            DigestDomain::ExpectedSelectionManifest,
            DigestDomain::Obligation,
            DigestDomain::ComparisonContext,
        ];
        let mut labels = BTreeSet::new();
        for domain in domains {
            let label = domain.as_bytes();
            assert!(label.starts_with(b"eqm:v2:"), "{label:?}");
            assert!(labels.insert(label), "duplicate domain label {label:?}");
            assert_ne!(
                Sha256Digest::hash_domain(domain, EMPTY_GRAPH),
                Sha256Digest::hash_content(EMPTY_GRAPH)
            );
        }
        assert_eq!(labels.len(), domains.len());
    }

    #[test]
    fn identity_digests_are_repeatable_and_domain_separated() {
        assert_eq!(
            RunnerDigest::hash_content(b"x"),
            RunnerDigest::hash_content(b"x")
        );
        assert_ne!(
            RunnerDigest::hash_content(b"x"),
            RunnerDigest::hash_content(b"y")
        );
        assert_ne!(
            RunnerDigest::hash_content(b"x").as_digest(),
            ContractDigest::hash_content(b"x").as_digest()
        );
        assert_ne!(
            RawObjectDigest::hash_content(b"x").as_digest(),
            SemanticEvidenceDigest::hash_content(b"x").as_digest()
        );
        assert_eq!(
            PlanDigest::hash_content(b"plan").to_string(),
            PlanDigest::hash_content(b"plan").as_digest().to_string()
        );
    }
}
