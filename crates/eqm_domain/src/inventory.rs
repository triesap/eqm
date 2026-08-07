//! Immutable discovered inventory, runtime facts, and release records.

use crate::{
    AdapterId, Diagnostic, DimensionId, EvidenceSubject, InventoryCompleteness, ProducerRef,
    ProfileId, ProfileSelection, ProviderId, ReleaseChannel, RepoPath, SelectorText, Sha256Digest,
    SourceCommit, SurfaceId, SymbolicValueId, TargetId, TrustLevel, UtcInstant,
};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// A bounded provider-neutral fact value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FactValue {
    /// Boolean fact.
    Boolean(bool),
    /// Signed integer fact.
    Integer(i64),
    /// Finite symbolic value.
    Symbol(SymbolicValueId),
    /// Bounded normalized text.
    Text(SelectorText),
}

/// One sorted provider-neutral inventory entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InventoryEntry {
    kind: SelectorText,
    key: SelectorText,
    attributes: BTreeMap<SelectorText, FactValue>,
    source: RepoPath,
}

impl InventoryEntry {
    /// Creates an entry and rejects duplicate attribute keys.
    pub fn new(
        kind: SelectorText,
        key: SelectorText,
        attributes: Vec<(SelectorText, FactValue)>,
        source: RepoPath,
    ) -> Result<Self, InventoryBuildError> {
        let count = attributes.len();
        let attributes: BTreeMap<_, _> = attributes.into_iter().collect();
        if attributes.len() != count {
            return Err(InventoryBuildError::DuplicateAttribute);
        }
        Ok(Self {
            kind,
            key,
            attributes,
            source,
        })
    }
    /// Returns entry kind.
    #[must_use]
    pub const fn kind(&self) -> &SelectorText {
        &self.kind
    }
    /// Returns entry key.
    #[must_use]
    pub const fn key(&self) -> &SelectorText {
        &self.key
    }
    /// Returns attributes by key.
    #[must_use]
    pub const fn attributes(&self) -> &BTreeMap<SelectorText, FactValue> {
        &self.attributes
    }
    /// Returns repository source path.
    #[must_use]
    pub const fn source(&self) -> &RepoPath {
        &self.source
    }
}

/// An immutable adapter-produced inventory.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Inventory {
    adapter: AdapterId,
    adapter_digest: Sha256Digest,
    subject: EvidenceSubject,
    target: TargetId,
    generated_at: UtcInstant,
    completeness: InventoryCompleteness,
    entries: BTreeMap<(SelectorText, SelectorText), InventoryEntry>,
    diagnostics: Vec<Diagnostic>,
    inventory_digest: Sha256Digest,
}

impl Inventory {
    /// Creates an inventory with unique `(kind, key)` entries.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        adapter: AdapterId,
        adapter_digest: Sha256Digest,
        subject: EvidenceSubject,
        target: TargetId,
        generated_at: UtcInstant,
        completeness: InventoryCompleteness,
        entries: Vec<InventoryEntry>,
        diagnostics: Vec<Diagnostic>,
        inventory_digest: Sha256Digest,
    ) -> Result<Self, InventoryBuildError> {
        let count = entries.len();
        let entries: BTreeMap<_, _> = entries
            .into_iter()
            .map(|entry| ((entry.kind().clone(), entry.key().clone()), entry))
            .collect();
        if entries.len() != count {
            return Err(InventoryBuildError::DuplicateEntry);
        }
        Ok(Self {
            adapter,
            adapter_digest,
            subject,
            target,
            generated_at,
            completeness,
            entries,
            diagnostics,
            inventory_digest,
        })
    }
    /// Returns whether absence may be treated as false.
    #[must_use]
    pub const fn can_prove_absence(&self) -> bool {
        matches!(self.completeness, InventoryCompleteness::Complete)
    }
    /// Returns adapter ID.
    #[must_use]
    pub const fn adapter(&self) -> &AdapterId {
        &self.adapter
    }
    /// Returns adapter digest.
    #[must_use]
    pub const fn adapter_digest(&self) -> Sha256Digest {
        self.adapter_digest
    }
    /// Returns exact subject.
    #[must_use]
    pub const fn subject(&self) -> &EvidenceSubject {
        &self.subject
    }
    /// Returns target.
    #[must_use]
    pub const fn target(&self) -> &TargetId {
        &self.target
    }
    /// Returns generation time.
    #[must_use]
    pub const fn generated_at(&self) -> UtcInstant {
        self.generated_at
    }
    /// Returns completeness claim.
    #[must_use]
    pub const fn completeness(&self) -> InventoryCompleteness {
        self.completeness
    }
    /// Returns entries in `(kind, key)` order.
    #[must_use]
    pub const fn entries(&self) -> &BTreeMap<(SelectorText, SelectorText), InventoryEntry> {
        &self.entries
    }
    /// Returns normalized diagnostics.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
    /// Returns inventory digest.
    #[must_use]
    pub const fn inventory_digest(&self) -> Sha256Digest {
        self.inventory_digest
    }
}

/// One runtime fact keyed by surface, dimension, and provider-neutral key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeFact {
    surface: SurfaceId,
    dimension: DimensionId,
    key: SelectorText,
    value: FactValue,
}

impl RuntimeFact {
    /// Creates a typed symbolic runtime fact.
    #[must_use]
    pub const fn new(
        surface: SurfaceId,
        dimension: DimensionId,
        key: SelectorText,
        value: FactValue,
    ) -> Self {
        Self {
            surface,
            dimension,
            key,
            value,
        }
    }
    /// Returns surface.
    #[must_use]
    pub const fn surface(&self) -> &SurfaceId {
        &self.surface
    }
    /// Returns dimension.
    #[must_use]
    pub const fn dimension(&self) -> &DimensionId {
        &self.dimension
    }
    /// Returns fact key.
    #[must_use]
    pub const fn key(&self) -> &SelectorText {
        &self.key
    }
    /// Returns typed value.
    #[must_use]
    pub const fn value(&self) -> &FactValue {
        &self.value
    }
}

/// An immutable expiring runtime-facts snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeFactsSnapshot {
    provider: ProviderId,
    subject: EvidenceSubject,
    target: TargetId,
    profile_values: BTreeMap<ProfileId, ProfileSelection>,
    observed_at: UtcInstant,
    expires_at: UtcInstant,
    facts: BTreeMap<(SurfaceId, DimensionId, SelectorText), RuntimeFact>,
    producer: ProducerRef,
    claimed_trust: TrustLevel,
    facts_digest: Sha256Digest,
}

impl RuntimeFactsSnapshot {
    /// Creates an exact snapshot with unique profiles and fact coordinates.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        provider: ProviderId,
        subject: EvidenceSubject,
        target: TargetId,
        profile_values: Vec<ProfileSelection>,
        observed_at: UtcInstant,
        expires_at: UtcInstant,
        facts: Vec<RuntimeFact>,
        producer: ProducerRef,
        claimed_trust: TrustLevel,
        facts_digest: Sha256Digest,
    ) -> Result<Self, InventoryBuildError> {
        if expires_at <= observed_at {
            return Err(InventoryBuildError::InvalidExpiry);
        }
        let profile_count = profile_values.len();
        let profile_values: BTreeMap<_, _> = profile_values
            .into_iter()
            .map(|value| (value.profile().clone(), value))
            .collect();
        if profile_values.len() != profile_count {
            return Err(InventoryBuildError::DuplicateProfile);
        }
        let fact_count = facts.len();
        let facts: BTreeMap<_, _> = facts
            .into_iter()
            .map(|fact| {
                (
                    (
                        fact.surface().clone(),
                        fact.dimension().clone(),
                        fact.key().clone(),
                    ),
                    fact,
                )
            })
            .collect();
        if facts.len() != fact_count {
            return Err(InventoryBuildError::DuplicateFact);
        }
        Ok(Self {
            provider,
            subject,
            target,
            profile_values,
            observed_at,
            expires_at,
            facts,
            producer,
            claimed_trust,
            facts_digest,
        })
    }
    /// Returns whether the snapshot is valid at an instant.
    #[must_use]
    pub fn is_fresh_at(&self, instant: UtcInstant) -> bool {
        self.observed_at <= instant && instant < self.expires_at
    }
    /// Returns provider.
    #[must_use]
    pub const fn provider(&self) -> &ProviderId {
        &self.provider
    }
    /// Returns subject.
    #[must_use]
    pub const fn subject(&self) -> &EvidenceSubject {
        &self.subject
    }
    /// Returns target.
    #[must_use]
    pub const fn target(&self) -> &TargetId {
        &self.target
    }
    /// Returns profile values.
    #[must_use]
    pub const fn profile_values(&self) -> &BTreeMap<ProfileId, ProfileSelection> {
        &self.profile_values
    }
    /// Returns observation time.
    #[must_use]
    pub const fn observed_at(&self) -> UtcInstant {
        self.observed_at
    }
    /// Returns expiry time.
    #[must_use]
    pub const fn expires_at(&self) -> UtcInstant {
        self.expires_at
    }
    /// Returns facts by exact coordinate.
    #[must_use]
    pub const fn facts(&self) -> &BTreeMap<(SurfaceId, DimensionId, SelectorText), RuntimeFact> {
        &self.facts
    }
    /// Returns producer.
    #[must_use]
    pub const fn producer(&self) -> &ProducerRef {
        &self.producer
    }
    /// Returns claimed trust.
    #[must_use]
    pub const fn claimed_trust(&self) -> TrustLevel {
        self.claimed_trust
    }
    /// Returns facts digest.
    #[must_use]
    pub const fn facts_digest(&self) -> Sha256Digest {
        self.facts_digest
    }
}

/// Exact three-state reconciliation result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReconcileStatus {
    /// Expected and observed values agree.
    Match,
    /// Expected and observed values disagree.
    Mismatch,
    /// Either side lacks authoritative knowledge.
    Unknown,
}

/// A validated application version token.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AppVersion(Box<str>);

impl AppVersion {
    /// Creates a 1-64 byte alphanumeric/dot/hyphen token.
    pub fn new(value: impl Into<Box<str>>) -> Result<Self, InventoryBuildError> {
        let value = value.into();
        if !version_token(&value) {
            return Err(InventoryBuildError::InvalidAppVersion);
        }
        Ok(Self(value))
    }
    /// Returns exact version.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A canonical decimal release build number.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BuildNumber(Box<str>);

impl BuildNumber {
    /// Creates a 1-32 digit canonical build number.
    pub fn new(value: impl Into<Box<str>>) -> Result<Self, InventoryBuildError> {
        let value = value.into();
        if value.is_empty()
            || value.len() > 32
            || (value.len() > 1 && value.starts_with('0'))
            || !value.bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(InventoryBuildError::InvalidBuildNumber);
        }
        Ok(Self(value))
    }
    /// Returns exact decimal value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn version_token(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value.as_bytes()[0].is_ascii_alphanumeric()
        && value
            .bytes()
            .skip(1)
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-'))
}

/// An immutable exact release record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseRecord {
    target: TargetId,
    app_version: AppVersion,
    build_number: BuildNumber,
    source_commit: SourceCommit,
    artifact_digest: Sha256Digest,
    channel: ReleaseChannel,
    released_at: UtcInstant,
    producer: ProducerRef,
    claimed_trust: TrustLevel,
    record_digest: Sha256Digest,
}

impl ReleaseRecord {
    /// Creates an exact digest-covered release identity.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        target: TargetId,
        app_version: AppVersion,
        build_number: BuildNumber,
        source_commit: SourceCommit,
        artifact_digest: Sha256Digest,
        channel: ReleaseChannel,
        released_at: UtcInstant,
        producer: ProducerRef,
        claimed_trust: TrustLevel,
        record_digest: Sha256Digest,
    ) -> Self {
        Self {
            target,
            app_version,
            build_number,
            source_commit,
            artifact_digest,
            channel,
            released_at,
            producer,
            claimed_trust,
            record_digest,
        }
    }
    /// Returns target.
    #[must_use]
    pub const fn target(&self) -> &TargetId {
        &self.target
    }
    /// Returns app version.
    #[must_use]
    pub const fn app_version(&self) -> &AppVersion {
        &self.app_version
    }
    /// Returns build number.
    #[must_use]
    pub const fn build_number(&self) -> &BuildNumber {
        &self.build_number
    }
    /// Returns source commit.
    #[must_use]
    pub const fn source_commit(&self) -> &SourceCommit {
        &self.source_commit
    }
    /// Returns artifact digest.
    #[must_use]
    pub const fn artifact_digest(&self) -> Sha256Digest {
        self.artifact_digest
    }
    /// Returns channel.
    #[must_use]
    pub const fn channel(&self) -> ReleaseChannel {
        self.channel
    }
    /// Returns release instant.
    #[must_use]
    pub const fn released_at(&self) -> UtcInstant {
        self.released_at
    }
    /// Returns producer.
    #[must_use]
    pub const fn producer(&self) -> &ProducerRef {
        &self.producer
    }
    /// Returns claimed trust.
    #[must_use]
    pub const fn claimed_trust(&self) -> TrustLevel {
        self.claimed_trust
    }
    /// Returns record digest.
    #[must_use]
    pub const fn record_digest(&self) -> Sha256Digest {
        self.record_digest
    }
}

/// Inventory, runtime-facts, or release construction failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InventoryBuildError {
    /// Inventory attributes repeated a key.
    DuplicateAttribute,
    /// Inventory repeated a `(kind, key)` entry.
    DuplicateEntry,
    /// Runtime facts repeated a profile.
    DuplicateProfile,
    /// Runtime facts repeated a fact coordinate.
    DuplicateFact,
    /// Runtime-facts expiry was not after observation.
    InvalidExpiry,
    /// App version was malformed.
    InvalidAppVersion,
    /// Build number was malformed.
    InvalidBuildNumber,
}

impl Display for InventoryBuildError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

impl Error for InventoryBuildError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EvidenceScopeSubject;

    fn subject() -> Result<EvidenceSubject, Box<dyn Error>> {
        Ok(EvidenceSubject::new(
            "https://github.com/example/project".parse()?,
            Sha256Digest::hash_content(b"repo"),
            EvidenceScopeSubject::Target(TargetId::new("web")?),
            "a".repeat(40).parse()?,
            None,
            None,
            Sha256Digest::hash_content(b"config"),
        ))
    }

    #[test]
    fn only_complete_inventory_proves_absence() -> Result<(), Box<dyn Error>> {
        for (completeness, expected) in [
            (InventoryCompleteness::Complete, true),
            (InventoryCompleteness::Partial, false),
            (InventoryCompleteness::Unknown, false),
        ] {
            let inventory = Inventory::new(
                AdapterId::new("adapter.rust")?,
                Sha256Digest::hash_content(b"adapter"),
                subject()?,
                TargetId::new("web")?,
                "2026-08-07T12:00:00Z".parse()?,
                completeness,
                Vec::new(),
                Vec::new(),
                Sha256Digest::hash_content(b"inventory"),
            )?;
            assert_eq!(inventory.can_prove_absence(), expected);
        }
        Ok(())
    }

    #[test]
    fn runtime_expiry_and_release_identity_are_exact() -> Result<(), Box<dyn Error>> {
        let observed: UtcInstant = "2026-08-07T12:00:00Z".parse()?;
        let expires: UtcInstant = "2026-08-07T13:00:00Z".parse()?;
        let snapshot = RuntimeFactsSnapshot::new(
            ProviderId::new("runtime.primary")?,
            subject()?,
            TargetId::new("web")?,
            Vec::new(),
            observed,
            expires,
            Vec::new(),
            "producer://runtime/platform/snapshot-1".parse()?,
            TrustLevel::SignedCi,
            Sha256Digest::hash_content(b"facts"),
        )?;
        assert!(snapshot.is_fresh_at(observed));
        assert!(!snapshot.is_fresh_at(expires));
        let record = ReleaseRecord::new(
            TargetId::new("web")?,
            AppVersion::new("1.2.3")?,
            BuildNumber::new("42")?,
            "a".repeat(40).parse()?,
            Sha256Digest::hash_content(b"artifact"),
            ReleaseChannel::Production,
            observed,
            "producer://release/platform/build-42".parse()?,
            TrustLevel::SignedCi,
            Sha256Digest::hash_content(b"record"),
        );
        assert_eq!(record.build_number().as_str(), "42");
        assert_eq!(
            BuildNumber::new("01"),
            Err(InventoryBuildError::InvalidBuildNumber)
        );
        Ok(())
    }
}
