//! Validated external provider authority.
//!
//! A provider is an independently implemented shared service referenced by
//! targets, requirements and scenario participants. Its declared
//! `configuration_digest` is an *external content reference* supplied by the
//! author; it is deliberately distinct from the
//! [`ProviderConfigurationDigest`](crate::ProviderConfigurationDigest) that the
//! canonical projection derives over the whole validated provider definition.
//! Neither value proves that the external material was acquired or trusted.

use crate::{Extensions, OwnerRef, ProviderId, Sha256Digest};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// One declared provider authority record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Provider {
    id: ProviderId,
    configuration_digest: Sha256Digest,
    owners: BTreeSet<OwnerRef>,
    extensions: Extensions,
}

impl Provider {
    /// Creates a provider with nonempty unique owners.
    pub fn new(
        id: ProviderId,
        configuration_digest: Sha256Digest,
        owners: Vec<OwnerRef>,
        extensions: Extensions,
    ) -> Result<Self, ProviderBuildError> {
        if owners.is_empty() {
            return Err(ProviderBuildError::OwnersRequired);
        }
        let count = owners.len();
        let owners: BTreeSet<_> = owners.into_iter().collect();
        if owners.len() != count {
            return Err(ProviderBuildError::DuplicateOwner);
        }
        Ok(Self {
            id,
            configuration_digest,
            owners,
            extensions,
        })
    }

    /// Returns the provider identity.
    #[must_use]
    pub const fn id(&self) -> &ProviderId {
        &self.id
    }

    /// Returns the declared external configuration content reference.
    #[must_use]
    pub const fn configuration_digest(&self) -> Sha256Digest {
        self.configuration_digest
    }

    /// Returns owners in deterministic order.
    #[must_use]
    pub const fn owners(&self) -> &BTreeSet<OwnerRef> {
        &self.owners
    }

    /// Returns semantic extensions.
    #[must_use]
    pub const fn extensions(&self) -> &Extensions {
        &self.extensions
    }
}

/// Provider construction failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderBuildError {
    /// Owners were empty.
    OwnersRequired,
    /// Owners contained a duplicate.
    DuplicateOwner,
}

impl Display for ProviderBuildError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::OwnersRequired => "provider requires at least one owner",
            Self::DuplicateOwner => "provider owners contain a duplicate",
        })
    }
}

impl Error for ProviderBuildError {}
