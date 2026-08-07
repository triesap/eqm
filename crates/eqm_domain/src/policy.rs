//! Fixed symbolic profiles and source-controlled policy authority.

use crate::{
    Description, DimensionId, DurationMillis, Extensions, Facet, FullRequirementId, OwnerRef,
    PolicyId, PositiveCount, ProfileId, RequirementLevel, RequirementScope, Revision, RiskClass,
    SymbolicValueId, TargetId, Title, TrustLevel, UnitId,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::NonZeroU32;

/// A positive waiver duration in calendar days.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PositiveDays(NonZeroU32);

impl PositiveDays {
    /// Creates a positive day count.
    pub fn new(value: u32) -> Result<Self, PolicyBuildError> {
        NonZeroU32::new(value)
            .map(Self)
            .ok_or(PolicyBuildError::PositiveValueRequired)
    }
    /// Returns the exact day count.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0.get()
    }
}

/// One finite symbolic profile dimension.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileDimension {
    id: DimensionId,
    values: BTreeSet<SymbolicValueId>,
    description: Option<Description>,
}

impl ProfileDimension {
    /// Creates a dimension with a nonempty duplicate-free value set.
    pub fn new(
        id: DimensionId,
        values: Vec<SymbolicValueId>,
        description: Option<Description>,
    ) -> Result<Self, PolicyBuildError> {
        Ok(Self {
            id,
            values: unique_nonempty(
                values,
                PolicyBuildError::ValuesRequired,
                PolicyBuildError::DuplicateValue,
            )?,
            description,
        })
    }
    /// Returns the local dimension ID.
    #[must_use]
    pub const fn id(&self) -> &DimensionId {
        &self.id
    }
    /// Returns symbolic values in canonical order.
    #[must_use]
    pub const fn values(&self) -> &BTreeSet<SymbolicValueId> {
        &self.values
    }
    /// Returns optional normative description.
    #[must_use]
    pub const fn description(&self) -> Option<&Description> {
        self.description.as_ref()
    }
}

/// A versioned fixed symbolic profile family.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Profile {
    id: ProfileId,
    revision: Revision,
    title: Title,
    owners: BTreeSet<OwnerRef>,
    dimensions: BTreeMap<DimensionId, ProfileDimension>,
    defaults: BTreeMap<DimensionId, SymbolicValueId>,
    description: Option<Description>,
    extensions: Extensions,
}

impl Profile {
    /// Creates a profile and verifies every default against its dimension.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: ProfileId,
        revision: Revision,
        title: Title,
        owners: Vec<OwnerRef>,
        dimensions: Vec<ProfileDimension>,
        defaults: Vec<(DimensionId, SymbolicValueId)>,
        description: Option<Description>,
        extensions: Extensions,
    ) -> Result<Self, PolicyBuildError> {
        let owners = unique_nonempty(
            owners,
            PolicyBuildError::OwnersRequired,
            PolicyBuildError::DuplicateOwner,
        )?;
        if dimensions.is_empty() {
            return Err(PolicyBuildError::DimensionsRequired);
        }
        let dimension_count = dimensions.len();
        let dimensions: BTreeMap<_, _> = dimensions
            .into_iter()
            .map(|dimension| (dimension.id().clone(), dimension))
            .collect();
        if dimensions.len() != dimension_count {
            return Err(PolicyBuildError::DuplicateDimension);
        }
        let default_count = defaults.len();
        let defaults: BTreeMap<_, _> = defaults.into_iter().collect();
        if defaults.len() != default_count {
            return Err(PolicyBuildError::DuplicateDefault);
        }
        for (dimension, value) in &defaults {
            if !dimensions
                .get(dimension)
                .is_some_and(|item| item.values().contains(value))
            {
                return Err(PolicyBuildError::InvalidDefault);
            }
        }
        Ok(Self {
            id,
            revision,
            title,
            owners,
            dimensions,
            defaults,
            description,
            extensions,
        })
    }
    /// Returns the profile ID.
    #[must_use]
    pub const fn id(&self) -> &ProfileId {
        &self.id
    }
    /// Returns revision.
    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }
    /// Returns title.
    #[must_use]
    pub const fn title(&self) -> &Title {
        &self.title
    }
    /// Returns owners.
    #[must_use]
    pub const fn owners(&self) -> &BTreeSet<OwnerRef> {
        &self.owners
    }
    /// Returns dimensions by ID.
    #[must_use]
    pub const fn dimensions(&self) -> &BTreeMap<DimensionId, ProfileDimension> {
        &self.dimensions
    }
    /// Returns valid defaults by dimension.
    #[must_use]
    pub const fn defaults(&self) -> &BTreeMap<DimensionId, SymbolicValueId> {
        &self.defaults
    }
    /// Returns optional description.
    #[must_use]
    pub const fn description(&self) -> Option<&Description> {
        self.description.as_ref()
    }
    /// Returns extensions.
    #[must_use]
    pub const fn extensions(&self) -> &Extensions {
        &self.extensions
    }
}

/// A closed exact-match policy selector with at least one populated axis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicySelector {
    units: Option<BTreeSet<UnitId>>,
    requirements: Option<BTreeSet<FullRequirementId>>,
    risk_classes: Option<BTreeSet<RiskClass>>,
    facets: Option<BTreeSet<Facet>>,
    scopes: Option<BTreeSet<RequirementScope>>,
}

impl PolicySelector {
    /// Creates a selector, rejecting empty fields and duplicate values.
    pub fn new(
        units: Option<Vec<UnitId>>,
        requirements: Option<Vec<FullRequirementId>>,
        risk_classes: Option<Vec<RiskClass>>,
        facets: Option<Vec<Facet>>,
        scopes: Option<Vec<RequirementScope>>,
    ) -> Result<Self, PolicyBuildError> {
        let units = optional_set(units)?;
        let requirements = optional_set(requirements)?;
        let risk_classes = optional_set(risk_classes)?;
        let facets = optional_set(facets)?;
        let scopes = optional_set(scopes)?;
        if units.is_none()
            && requirements.is_none()
            && risk_classes.is_none()
            && facets.is_none()
            && scopes.is_none()
        {
            return Err(PolicyBuildError::SelectorRequired);
        }
        Ok(Self {
            units,
            requirements,
            risk_classes,
            facets,
            scopes,
        })
    }
    /// Returns selected units.
    #[must_use]
    pub const fn units(&self) -> Option<&BTreeSet<UnitId>> {
        self.units.as_ref()
    }
    /// Returns selected requirements.
    #[must_use]
    pub const fn requirements(&self) -> Option<&BTreeSet<FullRequirementId>> {
        self.requirements.as_ref()
    }
    /// Returns selected risks.
    #[must_use]
    pub const fn risk_classes(&self) -> Option<&BTreeSet<RiskClass>> {
        self.risk_classes.as_ref()
    }
    /// Returns selected facets.
    #[must_use]
    pub const fn facets(&self) -> Option<&BTreeSet<Facet>> {
        self.facets.as_ref()
    }
    /// Returns selected scopes.
    #[must_use]
    pub const fn scopes(&self) -> Option<&BTreeSet<RequirementScope>> {
        self.scopes.as_ref()
    }
}

fn optional_set<T: Ord>(values: Option<Vec<T>>) -> Result<Option<BTreeSet<T>>, PolicyBuildError> {
    values
        .map(|values| {
            unique_nonempty(
                values,
                PolicyBuildError::SelectorFieldEmpty,
                PolicyBuildError::DuplicateSelectorValue,
            )
        })
        .transpose()
}

/// One monotonic policy requirement rule.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyRule {
    selector: PolicySelector,
    minimum_level: RequirementLevel,
    facets: BTreeSet<Facet>,
    minimum_trust: TrustLevel,
    maximum_age: DurationMillis,
    minimum_count: PositiveCount,
}

impl PolicyRule {
    /// Creates a rule with nonempty required facets.
    pub fn new(
        selector: PolicySelector,
        minimum_level: RequirementLevel,
        facets: Vec<Facet>,
        minimum_trust: TrustLevel,
        maximum_age: DurationMillis,
        minimum_count: Option<PositiveCount>,
    ) -> Result<Self, PolicyBuildError> {
        Ok(Self {
            selector,
            minimum_level,
            facets: unique_nonempty(
                facets,
                PolicyBuildError::FacetsRequired,
                PolicyBuildError::DuplicateFacet,
            )?,
            minimum_trust,
            maximum_age,
            minimum_count: minimum_count.unwrap_or(PositiveCount::ONE),
        })
    }

    /// Returns whether this rule is at least as strong on every ordered axis.
    #[must_use]
    pub fn strengthens_or_equals(&self, baseline: &Self) -> bool {
        self.selector == baseline.selector
            && self.minimum_level >= baseline.minimum_level
            && self.facets.is_superset(&baseline.facets)
            && self.minimum_trust >= baseline.minimum_trust
            && self.maximum_age <= baseline.maximum_age
            && self.minimum_count >= baseline.minimum_count
    }
    /// Returns selector.
    #[must_use]
    pub const fn selector(&self) -> &PolicySelector {
        &self.selector
    }
    /// Returns minimum requirement level.
    #[must_use]
    pub const fn minimum_level(&self) -> RequirementLevel {
        self.minimum_level
    }
    /// Returns required facets.
    #[must_use]
    pub const fn facets(&self) -> &BTreeSet<Facet> {
        &self.facets
    }
    /// Returns minimum trust.
    #[must_use]
    pub const fn minimum_trust(&self) -> TrustLevel {
        self.minimum_trust
    }
    /// Returns maximum evidence age.
    #[must_use]
    pub const fn maximum_age(&self) -> DurationMillis {
        self.maximum_age
    }
    /// Returns minimum evidence count.
    #[must_use]
    pub const fn minimum_count(&self) -> PositiveCount {
        self.minimum_count
    }
}

/// Policy-level waiver constraints.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WaiverPolicy {
    allowed: bool,
    maximum_days: Option<PositiveDays>,
    minimum_approvers: PositiveCount,
    required_controls: BTreeSet<Facet>,
}

impl WaiverPolicy {
    /// Creates a waiver policy with coherent allowed and duration fields.
    pub fn new(
        allowed: bool,
        maximum_days: Option<PositiveDays>,
        minimum_approvers: Option<PositiveCount>,
        required_controls: Vec<Facet>,
    ) -> Result<Self, PolicyBuildError> {
        if allowed != maximum_days.is_some() {
            return Err(if allowed {
                PolicyBuildError::MaximumDaysRequired
            } else {
                PolicyBuildError::MaximumDaysForbidden
            });
        }
        let control_count = required_controls.len();
        let required_controls: BTreeSet<_> = required_controls.into_iter().collect();
        if required_controls.len() != control_count {
            return Err(PolicyBuildError::DuplicateControl);
        }
        Ok(Self {
            allowed,
            maximum_days,
            minimum_approvers: minimum_approvers.unwrap_or(PositiveCount::ONE),
            required_controls,
        })
    }

    /// Returns the default-deny waiver policy.
    #[must_use]
    pub fn deny() -> Self {
        Self {
            allowed: false,
            maximum_days: None,
            minimum_approvers: PositiveCount::ONE,
            required_controls: BTreeSet::new(),
        }
    }

    /// Returns whether this policy is no weaker than a baseline.
    #[must_use]
    pub fn strengthens_or_equals(&self, baseline: &Self) -> bool {
        (!self.allowed || baseline.allowed)
            && match (self.maximum_days, baseline.maximum_days) {
                (Some(candidate), Some(base)) => candidate <= base,
                (None, _) => true,
                (Some(_), None) => false,
            }
            && self.minimum_approvers >= baseline.minimum_approvers
            && self
                .required_controls
                .is_superset(&baseline.required_controls)
    }
    /// Returns whether waivers are allowed.
    #[must_use]
    pub const fn allowed(&self) -> bool {
        self.allowed
    }
    /// Returns maximum allowed calendar days.
    #[must_use]
    pub const fn maximum_days(&self) -> Option<PositiveDays> {
        self.maximum_days
    }
    /// Returns minimum approvers.
    #[must_use]
    pub const fn minimum_approvers(&self) -> PositiveCount {
        self.minimum_approvers
    }
    /// Returns required controls.
    #[must_use]
    pub const fn required_controls(&self) -> &BTreeSet<Facet> {
        &self.required_controls
    }
}

impl Default for WaiverPolicy {
    fn default() -> Self {
        Self::deny()
    }
}

/// A versioned policy authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Policy {
    id: PolicyId,
    revision: Revision,
    title: Title,
    owners: BTreeSet<OwnerRef>,
    profiles: BTreeSet<ProfileId>,
    required_targets: BTreeSet<TargetId>,
    rules: Vec<PolicyRule>,
    waivers: WaiverPolicy,
    description: Option<Description>,
    extensions: Extensions,
}

impl Policy {
    /// Creates a policy with nonempty unique profiles, targets, and rules.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: PolicyId,
        revision: Revision,
        title: Title,
        owners: Vec<OwnerRef>,
        profiles: Vec<ProfileId>,
        required_targets: Vec<TargetId>,
        rules: Vec<PolicyRule>,
        waivers: WaiverPolicy,
        description: Option<Description>,
        extensions: Extensions,
    ) -> Result<Self, PolicyBuildError> {
        let owners = unique_nonempty(
            owners,
            PolicyBuildError::OwnersRequired,
            PolicyBuildError::DuplicateOwner,
        )?;
        let profiles = unique_nonempty(
            profiles,
            PolicyBuildError::ProfilesRequired,
            PolicyBuildError::DuplicateProfile,
        )?;
        let required_targets = unique_nonempty(
            required_targets,
            PolicyBuildError::TargetsRequired,
            PolicyBuildError::DuplicateTarget,
        )?;
        if rules.is_empty() {
            return Err(PolicyBuildError::RulesRequired);
        }
        for (index, rule) in rules.iter().enumerate() {
            if rules[..index].contains(rule) {
                return Err(PolicyBuildError::DuplicateRule);
            }
        }
        Ok(Self {
            id,
            revision,
            title,
            owners,
            profiles,
            required_targets,
            rules,
            waivers,
            description,
            extensions,
        })
    }
    /// Returns policy ID.
    #[must_use]
    pub const fn id(&self) -> &PolicyId {
        &self.id
    }
    /// Returns revision.
    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }
    /// Returns title.
    #[must_use]
    pub const fn title(&self) -> &Title {
        &self.title
    }
    /// Returns owners.
    #[must_use]
    pub const fn owners(&self) -> &BTreeSet<OwnerRef> {
        &self.owners
    }
    /// Returns required profiles.
    #[must_use]
    pub const fn profiles(&self) -> &BTreeSet<ProfileId> {
        &self.profiles
    }
    /// Returns required targets.
    #[must_use]
    pub const fn required_targets(&self) -> &BTreeSet<TargetId> {
        &self.required_targets
    }
    /// Returns authored rules.
    #[must_use]
    pub fn rules(&self) -> &[PolicyRule] {
        &self.rules
    }
    /// Returns waiver rules.
    #[must_use]
    pub const fn waivers(&self) -> &WaiverPolicy {
        &self.waivers
    }
    /// Returns optional description.
    #[must_use]
    pub const fn description(&self) -> Option<&Description> {
        self.description.as_ref()
    }
    /// Returns extensions.
    #[must_use]
    pub const fn extensions(&self) -> &Extensions {
        &self.extensions
    }
}

fn unique_nonempty<T: Ord>(
    values: Vec<T>,
    empty: PolicyBuildError,
    duplicate: PolicyBuildError,
) -> Result<BTreeSet<T>, PolicyBuildError> {
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

/// Policy or profile construction failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolicyBuildError {
    /// A positive value was zero.
    PositiveValueRequired,
    /// Owners were empty.
    OwnersRequired,
    /// Owners contained a duplicate.
    DuplicateOwner,
    /// Profile dimensions were empty.
    DimensionsRequired,
    /// Profile dimensions contained a duplicate ID.
    DuplicateDimension,
    /// Dimension values were empty.
    ValuesRequired,
    /// Dimension values contained a duplicate.
    DuplicateValue,
    /// Profile defaults contained a duplicate key.
    DuplicateDefault,
    /// A default named an absent dimension or value.
    InvalidDefault,
    /// Policy selector had no fields.
    SelectorRequired,
    /// A present selector field was empty.
    SelectorFieldEmpty,
    /// A selector field contained a duplicate.
    DuplicateSelectorValue,
    /// Rule facets were empty.
    FacetsRequired,
    /// Rule facets contained a duplicate.
    DuplicateFacet,
    /// Allowed waiver policy omitted maximum days.
    MaximumDaysRequired,
    /// Denied waiver policy declared maximum days.
    MaximumDaysForbidden,
    /// Waiver controls contained a duplicate.
    DuplicateControl,
    /// Policy profiles were empty.
    ProfilesRequired,
    /// Policy profiles contained a duplicate.
    DuplicateProfile,
    /// Policy targets were empty.
    TargetsRequired,
    /// Policy targets contained a duplicate.
    DuplicateTarget,
    /// Policy rules were empty.
    RulesRequired,
    /// Policy rules contained an exact duplicate.
    DuplicateRule,
}

impl Display for PolicyBuildError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

impl Error for PolicyBuildError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_defaults_must_be_declared() -> Result<(), Box<dyn Error>> {
        let dimension = ProfileDimension::new(
            DimensionId::new("region")?,
            vec![SymbolicValueId::new("eu")?, SymbolicValueId::new("us")?],
            None,
        )?;
        let profile = Profile::new(
            ProfileId::new("audience.default")?,
            Revision::new(1)?,
            Title::new("Audience")?,
            vec!["owner://team/product".parse()?],
            vec![dimension],
            vec![(DimensionId::new("region")?, SymbolicValueId::new("eu")?)],
            None,
            Extensions::default(),
        )?;
        assert_eq!(profile.defaults().len(), 1);
        Ok(())
    }

    #[test]
    fn rule_and_waiver_strength_are_monotonic() -> Result<(), Box<dyn Error>> {
        let unit = UnitId::new("account.create")?;
        let selector = || PolicySelector::new(Some(vec![unit.clone()]), None, None, None, None);
        let baseline = PolicyRule::new(
            selector()?,
            RequirementLevel::Recommended,
            vec![Facet::Behavior],
            TrustLevel::TrustedCi,
            DurationMillis::new(86_400_000)?,
            Some(PositiveCount::ONE),
        )?;
        let stronger = PolicyRule::new(
            selector()?,
            RequirementLevel::Required,
            vec![Facet::Behavior, Facet::Accessibility],
            TrustLevel::SignedCi,
            DurationMillis::new(3_600_000)?,
            Some(PositiveCount::new(2)?),
        )?;
        assert!(stronger.strengthens_or_equals(&baseline));
        assert!(!baseline.strengthens_or_equals(&stronger));
        let allowed = WaiverPolicy::new(true, Some(PositiveDays::new(7)?), None, Vec::new())?;
        assert!(WaiverPolicy::deny().strengthens_or_equals(&allowed));
        assert!(!allowed.strengthens_or_equals(&WaiverPolicy::deny()));
        Ok(())
    }

    #[test]
    fn selectors_and_policy_sets_are_nonempty_and_unique() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            PolicySelector::new(None, None, None, None, None),
            Err(PolicyBuildError::SelectorRequired)
        );
        assert_eq!(
            PolicySelector::new(Some(Vec::new()), None, None, None, None),
            Err(PolicyBuildError::SelectorFieldEmpty)
        );
        assert_eq!(
            WaiverPolicy::new(true, None, None, Vec::new()),
            Err(PolicyBuildError::MaximumDaysRequired)
        );
        Ok(())
    }
}
