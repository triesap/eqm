//! Exact, visible, approved, and expiring waiver authority.

use crate::{
    CalendarDate, DimensionId, EvidenceScopeSubject, Extensions, Facet, FullRequirementId,
    IssueRef, OwnerRef, PolicyId, ProfileId, Revision, SymbolicValueId, UnitId, WaiverId,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use unicode_normalization::UnicodeNormalization;

/// A normalized waiver reason of at most 2,048 UTF-8 bytes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WaiverReason(Box<str>);

impl WaiverReason {
    /// Creates a nonempty normalized reason.
    pub fn new(value: impl Into<Box<str>>) -> Result<Self, WaiverBuildError> {
        let value = value.into();
        if value.is_empty()
            || value.len() > 2_048
            || !value.nfc().eq(value.chars())
            || value
                .chars()
                .any(|character| character.is_control() && !matches!(character, '\n' | '\t'))
        {
            return Err(WaiverBuildError::InvalidReason);
        }
        Ok(Self(value))
    }
    /// Returns the exact reason.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// One exact profile-value coordinate in a waiver scope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WaiverProfileScope {
    profile: ProfileId,
    values: BTreeMap<DimensionId, SymbolicValueId>,
}

impl WaiverProfileScope {
    /// Creates a nonempty duplicate-free exact profile coordinate.
    pub fn new(
        profile: ProfileId,
        values: Vec<(DimensionId, SymbolicValueId)>,
    ) -> Result<Self, WaiverBuildError> {
        if values.is_empty() {
            return Err(WaiverBuildError::ProfileValuesRequired);
        }
        let count = values.len();
        let values: BTreeMap<_, _> = values.into_iter().collect();
        if values.len() != count {
            return Err(WaiverBuildError::DuplicateDimension);
        }
        Ok(Self { profile, values })
    }
    /// Returns profile ID.
    #[must_use]
    pub const fn profile(&self) -> &ProfileId {
        &self.profile
    }
    /// Returns exact dimension values.
    #[must_use]
    pub const fn values(&self) -> &BTreeMap<DimensionId, SymbolicValueId> {
        &self.values
    }
}

/// Exact obligation coordinates authorized by a waiver.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WaiverScope {
    target: EvidenceScopeSubject,
    unit: UnitId,
    requirement: FullRequirementId,
    facets: BTreeSet<Facet>,
    profiles: BTreeMap<ProfileId, WaiverProfileScope>,
}

impl WaiverScope {
    /// Creates a scope with no wildcard or implicit expansion.
    pub fn new(
        target: EvidenceScopeSubject,
        unit: UnitId,
        requirement: FullRequirementId,
        facets: Vec<Facet>,
        profiles: Vec<WaiverProfileScope>,
    ) -> Result<Self, WaiverBuildError> {
        let facets = unique_nonempty(
            facets,
            WaiverBuildError::FacetsRequired,
            WaiverBuildError::DuplicateFacet,
        )?;
        if profiles.is_empty() {
            return Err(WaiverBuildError::ProfilesRequired);
        }
        let count = profiles.len();
        let profiles: BTreeMap<_, _> = profiles
            .into_iter()
            .map(|profile| (profile.profile().clone(), profile))
            .collect();
        if profiles.len() != count {
            return Err(WaiverBuildError::DuplicateProfile);
        }
        Ok(Self {
            target,
            unit,
            requirement,
            facets,
            profiles,
        })
    }
    /// Returns exact target/provider/set subject.
    #[must_use]
    pub const fn target(&self) -> &EvidenceScopeSubject {
        &self.target
    }
    /// Returns exact unit.
    #[must_use]
    pub const fn unit(&self) -> &UnitId {
        &self.unit
    }
    /// Returns exact requirement.
    #[must_use]
    pub const fn requirement(&self) -> &FullRequirementId {
        &self.requirement
    }
    /// Returns exact facets.
    #[must_use]
    pub const fn facets(&self) -> &BTreeSet<Facet> {
        &self.facets
    }
    /// Returns exact profile coordinates.
    #[must_use]
    pub const fn profiles(&self) -> &BTreeMap<ProfileId, WaiverProfileScope> {
        &self.profiles
    }
}

/// A valid waiver's only positive evaluation effect.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WaiverApplication {
    /// A waivable blocker remains visible as waived.
    Waived,
}

/// A versioned externally approved waiver authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Waiver {
    id: WaiverId,
    revision: Revision,
    owners: BTreeSet<OwnerRef>,
    policy: PolicyId,
    scope: WaiverScope,
    reason: WaiverReason,
    issue: IssueRef,
    approvers: BTreeSet<OwnerRef>,
    starts_on: CalendarDate,
    expires_on: CalendarDate,
    controls: BTreeSet<Facet>,
    extensions: Extensions,
}

impl Waiver {
    /// Creates a waiver with complete authority and a strictly ordered date window.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: WaiverId,
        revision: Revision,
        owners: Vec<OwnerRef>,
        policy: PolicyId,
        scope: WaiverScope,
        reason: WaiverReason,
        issue: IssueRef,
        approvers: Vec<OwnerRef>,
        starts_on: CalendarDate,
        expires_on: CalendarDate,
        controls: Vec<Facet>,
        extensions: Extensions,
    ) -> Result<Self, WaiverBuildError> {
        if expires_on <= starts_on {
            return Err(WaiverBuildError::InvalidDateWindow);
        }
        let owners = unique_nonempty(
            owners,
            WaiverBuildError::OwnersRequired,
            WaiverBuildError::DuplicateOwner,
        )?;
        let approvers = unique_nonempty(
            approvers,
            WaiverBuildError::ApproversRequired,
            WaiverBuildError::DuplicateApprover,
        )?;
        let control_count = controls.len();
        let controls: BTreeSet<_> = controls.into_iter().collect();
        if controls.len() != control_count {
            return Err(WaiverBuildError::DuplicateControl);
        }
        Ok(Self {
            id,
            revision,
            owners,
            policy,
            scope,
            reason,
            issue,
            approvers,
            starts_on,
            expires_on,
            controls,
            extensions,
        })
    }

    /// Returns whether the date lies in the inclusive-start, exclusive-expiry window.
    #[must_use]
    pub fn is_active_on(&self, date: CalendarDate) -> bool {
        self.starts_on <= date && date < self.expires_on
    }
    /// Returns the only valid positive application result.
    #[must_use]
    pub const fn application(&self) -> WaiverApplication {
        WaiverApplication::Waived
    }
    /// Returns waiver ID.
    #[must_use]
    pub const fn id(&self) -> &WaiverId {
        &self.id
    }
    /// Returns revision.
    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }
    /// Returns owners.
    #[must_use]
    pub const fn owners(&self) -> &BTreeSet<OwnerRef> {
        &self.owners
    }
    /// Returns protected policy ID.
    #[must_use]
    pub const fn policy(&self) -> &PolicyId {
        &self.policy
    }
    /// Returns exact scope.
    #[must_use]
    pub const fn scope(&self) -> &WaiverScope {
        &self.scope
    }
    /// Returns reason.
    #[must_use]
    pub const fn reason(&self) -> &WaiverReason {
        &self.reason
    }
    /// Returns issue authority.
    #[must_use]
    pub const fn issue(&self) -> &IssueRef {
        &self.issue
    }
    /// Returns approvers.
    #[must_use]
    pub const fn approvers(&self) -> &BTreeSet<OwnerRef> {
        &self.approvers
    }
    /// Returns start date.
    #[must_use]
    pub const fn starts_on(&self) -> CalendarDate {
        self.starts_on
    }
    /// Returns exclusive expiry date.
    #[must_use]
    pub const fn expires_on(&self) -> CalendarDate {
        self.expires_on
    }
    /// Returns compensating controls.
    #[must_use]
    pub const fn controls(&self) -> &BTreeSet<Facet> {
        &self.controls
    }
    /// Returns extensions.
    #[must_use]
    pub const fn extensions(&self) -> &Extensions {
        &self.extensions
    }
}

fn unique_nonempty<T: Ord>(
    values: Vec<T>,
    empty: WaiverBuildError,
    duplicate: WaiverBuildError,
) -> Result<BTreeSet<T>, WaiverBuildError> {
    if values.is_empty() {
        return Err(empty);
    }
    let count = values.len();
    let values: BTreeSet<_> = values.into_iter().collect();
    if values.len() != count {
        return Err(duplicate);
    }
    Ok(values)
}

/// Waiver construction failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WaiverBuildError {
    /// Reason was empty, non-normalized, or too long.
    InvalidReason,
    /// Owners were empty.
    OwnersRequired,
    /// Owners contained a duplicate.
    DuplicateOwner,
    /// Approvers were empty.
    ApproversRequired,
    /// Approvers contained a duplicate.
    DuplicateApprover,
    /// Expiry was not strictly after start.
    InvalidDateWindow,
    /// Scope facets were empty.
    FacetsRequired,
    /// Scope facets contained a duplicate.
    DuplicateFacet,
    /// Scope profiles were empty.
    ProfilesRequired,
    /// Scope profiles contained a duplicate.
    DuplicateProfile,
    /// Profile coordinate values were empty.
    ProfileValuesRequired,
    /// Profile coordinate repeated a dimension.
    DuplicateDimension,
    /// Controls contained a duplicate.
    DuplicateControl,
}

impl Display for WaiverBuildError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

impl Error for WaiverBuildError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn scope() -> Result<WaiverScope, Box<dyn Error>> {
        Ok(WaiverScope::new(
            EvidenceScopeSubject::Target(crate::TargetId::new("web")?),
            UnitId::new("account.create.signup.start")?,
            FullRequirementId::new("account.create.signup.start#reachable")?,
            vec![Facet::Behavior],
            vec![WaiverProfileScope::new(
                ProfileId::new("audience.default")?,
                vec![(DimensionId::new("region")?, SymbolicValueId::new("eu")?)],
            )?],
        )?)
    }

    #[test]
    fn waiver_requires_authority_scope_and_ordered_dates() -> Result<(), Box<dyn Error>> {
        let start: CalendarDate = "2026-08-01".parse()?;
        let expiry: CalendarDate = "2026-08-08".parse()?;
        let waiver = Waiver::new(
            WaiverId::new("waiver.signup")?,
            Revision::new(1)?,
            vec!["owner://team/product".parse()?],
            PolicyId::new("release.default")?,
            scope()?,
            WaiverReason::new("Temporary accessibility remediation")?,
            "issue://PRODUCT-42".parse()?,
            vec!["owner://role/reviewer".parse()?],
            start,
            expiry,
            vec![Facet::Accessibility],
            Extensions::default(),
        )?;
        assert!(waiver.is_active_on(start));
        assert!(!waiver.is_active_on(expiry));
        assert_eq!(waiver.application(), WaiverApplication::Waived);
        assert!(matches!(
            Waiver::new(
                WaiverId::new("waiver.signup")?,
                Revision::new(1)?,
                Vec::new(),
                PolicyId::new("release.default")?,
                scope()?,
                WaiverReason::new("Reason")?,
                "issue://PRODUCT-42".parse()?,
                vec!["owner://role/reviewer".parse()?],
                start,
                expiry,
                Vec::new(),
                Extensions::default(),
            ),
            Err(WaiverBuildError::OwnersRequired)
        ));
        Ok(())
    }

    #[test]
    fn reversed_dates_and_incomplete_scope_fail() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            WaiverScope::new(
                EvidenceScopeSubject::Target(crate::TargetId::new("web")?),
                UnitId::new("account.create.signup.start")?,
                FullRequirementId::new("account.create.signup.start#reachable")?,
                Vec::new(),
                Vec::new(),
            ),
            Err(WaiverBuildError::FacetsRequired)
        );
        let date: CalendarDate = "2026-08-01".parse()?;
        assert!(matches!(
            Waiver::new(
                WaiverId::new("waiver.signup")?,
                Revision::new(1)?,
                vec!["owner://team/product".parse()?],
                PolicyId::new("release.default")?,
                scope()?,
                WaiverReason::new("Reason")?,
                "issue://PRODUCT-42".parse()?,
                vec!["owner://role/reviewer".parse()?],
                date,
                date,
                Vec::new(),
                Extensions::default(),
            ),
            Err(WaiverBuildError::InvalidDateWindow)
        ));
        Ok(())
    }
}
