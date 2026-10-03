//! Validated domain carriers for v2 selected policy/profile provenance.
//!
//! These carriers retain the exact selected [`Policy`] and [`Profile`]
//! definitions plus fully resolved per-dimension state. Construction proves
//! structural selection validity only: it does not confer protected authority,
//! trusted execution, freshness or waiver standing.
//!
//! Dimension state is explicit. A known value is one declared finite value of
//! the owning profile dimension; [`SelectedDimensionStateV2::Unknown`] is an
//! explicit absence of knowledge. A constructor never invents a development
//! default and never turns an omitted dimension into `Unknown`: every declared
//! dimension must appear exactly once.

use crate::{
    ContractDigest, DimensionId, Policy, Profile, Scenario, Sha256Digest, SymbolicValueId,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Explicit selected state for one declared profile dimension.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SelectedDimensionStateV2 {
    /// A declared finite known value.
    Known(SymbolicValueId),
    /// Explicitly unknown; never a default and never an omitted dimension.
    Unknown,
}

/// One selected profile definition with every declared dimension resolved once.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedProfileInputV2<'a> {
    profile: &'a Profile,
    values: BTreeMap<DimensionId, SelectedDimensionStateV2>,
}

impl<'a> SelectedProfileInputV2<'a> {
    /// Builds a resolved selection for `profile`.
    ///
    /// Rejects an undeclared dimension, a repeated dimension, a `Known` value
    /// outside its finite declaration, and a profile whose declared dimensions
    /// are not all present. A missing dimension is an error, not `Unknown`.
    pub fn new(
        profile: &'a Profile,
        values: Vec<(DimensionId, SelectedDimensionStateV2)>,
    ) -> Result<Self, PolicySelectionInputError> {
        let mut resolved = BTreeMap::new();
        for (dimension, state) in values {
            let declaration = profile
                .dimensions()
                .get(&dimension)
                .ok_or(PolicySelectionInputError::UndeclaredDimension)?;
            if let SelectedDimensionStateV2::Known(value) = &state
                && !declaration.values().contains(value)
            {
                return Err(PolicySelectionInputError::UndeclaredValue);
            }
            if resolved.insert(dimension, state).is_some() {
                return Err(PolicySelectionInputError::DuplicateDimension);
            }
        }
        if resolved.len() != profile.dimensions().len() {
            return Err(PolicySelectionInputError::MissingDimension);
        }
        Ok(Self {
            profile,
            values: resolved,
        })
    }

    /// Returns the exact selected profile definition.
    #[must_use]
    pub const fn profile(&self) -> &'a Profile {
        self.profile
    }

    /// Returns every declared dimension with its explicit state.
    #[must_use]
    pub const fn values(&self) -> &BTreeMap<DimensionId, SelectedDimensionStateV2> {
        &self.values
    }
}

/// One validated policy selection carrying the exact selected definitions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicySelectionInputV2<'a> {
    policy: &'a Policy,
    profiles: Vec<SelectedProfileInputV2<'a>>,
}

impl<'a> PolicySelectionInputV2<'a> {
    /// Builds a validated selection for `policy`.
    ///
    /// The selected profile-ID set must equal the policy's required profile-ID
    /// set exactly, with no duplicate profile ID even across different
    /// revisions.
    pub fn new(
        policy: &'a Policy,
        profiles: Vec<SelectedProfileInputV2<'a>>,
    ) -> Result<Self, PolicySelectionInputError> {
        let mut ids = BTreeSet::new();
        for profile in &profiles {
            if !ids.insert(profile.profile().id().clone()) {
                return Err(PolicySelectionInputError::DuplicateProfile);
            }
        }
        if ids != *policy.profiles() {
            return Err(PolicySelectionInputError::ProfileSetMismatch);
        }
        Ok(Self { policy, profiles })
    }

    /// Returns the exact selected policy definition.
    #[must_use]
    pub const fn policy(&self) -> &'a Policy {
        self.policy
    }

    /// Returns the selected profile definitions.
    #[must_use]
    pub fn profiles(&self) -> &[SelectedProfileInputV2<'a>] {
        &self.profiles
    }
}

/// Shared comparison-context input.
///
/// Retains the exact shared comparison axes: an already-computed contract
/// identity, the exact authored scenario, the validated selected-profile
/// definitions/values and an explicitly supplied structural rule authority
/// reference. Exact participant build/source/artifact assignments are absent by
/// construction, so distinct subjects can share one comparison context. Holding
/// one confers no subject validity or trust.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComparisonContextInputV2<'a> {
    contract_digest: ContractDigest,
    scenario: &'a Scenario,
    selection: &'a PolicySelectionInputV2<'a>,
    comparison_rule_digest: Sha256Digest,
}

impl<'a> ComparisonContextInputV2<'a> {
    /// Creates one comparison-context input from explicit shared axes.
    #[must_use]
    pub const fn new(
        contract_digest: ContractDigest,
        scenario: &'a Scenario,
        selection: &'a PolicySelectionInputV2<'a>,
        comparison_rule_digest: Sha256Digest,
    ) -> Self {
        Self {
            contract_digest,
            scenario,
            selection,
            comparison_rule_digest,
        }
    }

    /// Returns the shared contract identity.
    #[must_use]
    pub const fn contract_digest(&self) -> ContractDigest {
        self.contract_digest
    }

    /// Returns the exact scenario.
    #[must_use]
    pub const fn scenario(&self) -> &'a Scenario {
        self.scenario
    }

    /// Returns the validated selected-profile definitions/values.
    #[must_use]
    pub const fn selection(&self) -> &'a PolicySelectionInputV2<'a> {
        self.selection
    }

    /// Returns the explicitly supplied structural rule authority reference.
    #[must_use]
    pub const fn comparison_rule_digest(&self) -> Sha256Digest {
        self.comparison_rule_digest
    }
}

/// Structural selection-carrier construction failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolicySelectionInputError {
    /// A profile ID was selected more than once, including across revisions.
    DuplicateProfile,
    /// The selected profile-ID set did not equal the policy's profile-ID set.
    ProfileSetMismatch,
    /// A declared dimension appeared more than once.
    DuplicateDimension,
    /// A declared dimension was omitted instead of being explicitly known or unknown.
    MissingDimension,
    /// A selected dimension was not declared by the profile.
    UndeclaredDimension,
    /// A known value was outside its dimension's finite declaration.
    UndeclaredValue,
}

impl Display for PolicySelectionInputError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateProfile => formatter.write_str("duplicate selected profile ID"),
            Self::ProfileSetMismatch => {
                formatter.write_str("selected profiles did not match the policy profile set")
            }
            Self::DuplicateDimension => formatter.write_str("duplicate selected dimension"),
            Self::MissingDimension => formatter.write_str("declared dimension was omitted"),
            Self::UndeclaredDimension => formatter.write_str("undeclared selected dimension"),
            Self::UndeclaredValue => formatter.write_str("undeclared selected value"),
        }
    }
}

impl Error for PolicySelectionInputError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Description, DurationMillis, Extensions, Facet, OwnerRef, PolicyBuildError, PolicyId,
        PolicyRule, PolicySelector, PositiveCount, ProfileDimension, ProfileId, RequirementLevel,
        Revision, TargetId, Title, TrustLevel, UnitId,
    };

    fn dimension(id: &str, values: &[&str]) -> Result<ProfileDimension, Box<dyn Error>> {
        Ok(ProfileDimension::new(
            DimensionId::new(id)?,
            values
                .iter()
                .map(|value| SymbolicValueId::new(*value))
                .collect::<Result<Vec<_>, _>>()?,
            None,
        )?)
    }

    fn profile(id: &str, revision: u64, default: bool) -> Result<Profile, Box<dyn Error>> {
        let defaults = if default {
            vec![(DimensionId::new("locale")?, SymbolicValueId::new("en")?)]
        } else {
            Vec::new()
        };
        Ok(Profile::new(
            ProfileId::new(id)?,
            Revision::new(revision)?,
            Title::new("Audience")?,
            vec!["owner://team/product".parse::<OwnerRef>()?],
            vec![
                dimension("locale", &["en", "fr"])?,
                dimension("region", &["eu", "us"])?,
            ],
            defaults,
            None::<Description>,
            Extensions::default(),
        )?)
    }

    fn policy(profiles: &[&str]) -> Result<Policy, Box<dyn Error>> {
        let rule = PolicyRule::new(
            PolicySelector::new(
                Some(vec![UnitId::new("account.create")?]),
                None,
                None,
                None,
                None,
            )?,
            RequirementLevel::Recommended,
            vec![Facet::Behavior],
            TrustLevel::TrustedCi,
            DurationMillis::new(86_400_000)?,
            Some(PositiveCount::ONE),
        )?;
        Ok(Policy::new(
            PolicyId::new("policy.default")?,
            Revision::new(1)?,
            Title::new("Default policy")?,
            vec!["owner://team/security".parse::<OwnerRef>()?],
            profiles
                .iter()
                .map(|profile| ProfileId::new(*profile))
                .collect::<Result<Vec<_>, _>>()?,
            vec![TargetId::new("web")?],
            vec![rule],
            crate::WaiverPolicy::deny(),
            None,
            Extensions::default(),
        )?)
    }

    fn selected<'a>(
        profile: &'a Profile,
        locale: SelectedDimensionStateV2,
    ) -> Result<SelectedProfileInputV2<'a>, Box<dyn Error>> {
        Ok(SelectedProfileInputV2::new(
            profile,
            vec![
                (DimensionId::new("locale")?, locale),
                (
                    DimensionId::new("region")?,
                    SelectedDimensionStateV2::Unknown,
                ),
            ],
        )?)
    }

    #[test]
    fn explicit_known_and_unknown_are_preserved_without_defaults() -> Result<(), Box<dyn Error>> {
        let profile = profile("audience.default", 1, true)?;
        let input = selected(&profile, SelectedDimensionStateV2::Unknown)?;
        // The profile declares an `en` default, but the explicit unknown must
        // remain unknown rather than becoming that default.
        assert_eq!(
            input.values().get(&DimensionId::new("locale")?),
            Some(&SelectedDimensionStateV2::Unknown)
        );
        assert_eq!(
            input.values().get(&DimensionId::new("region")?),
            Some(&SelectedDimensionStateV2::Unknown)
        );
        assert_eq!(input.profile(), &profile);
        Ok(())
    }

    #[test]
    fn dimension_invariants_are_enforced() -> Result<(), Box<dyn Error>> {
        let profile = profile("audience.default", 1, false)?;
        // Missing dimension.
        assert_eq!(
            SelectedProfileInputV2::new(
                &profile,
                vec![(
                    DimensionId::new("locale")?,
                    SelectedDimensionStateV2::Known(SymbolicValueId::new("en")?),
                )],
            ),
            Err(PolicySelectionInputError::MissingDimension)
        );
        // Duplicate dimension.
        assert_eq!(
            SelectedProfileInputV2::new(
                &profile,
                vec![
                    (
                        DimensionId::new("locale")?,
                        SelectedDimensionStateV2::Known(SymbolicValueId::new("en")?),
                    ),
                    (
                        DimensionId::new("locale")?,
                        SelectedDimensionStateV2::Unknown,
                    ),
                    (
                        DimensionId::new("region")?,
                        SelectedDimensionStateV2::Unknown
                    ),
                ],
            ),
            Err(PolicySelectionInputError::DuplicateDimension)
        );
        // Undeclared dimension.
        assert_eq!(
            SelectedProfileInputV2::new(
                &profile,
                vec![
                    (
                        DimensionId::new("locale")?,
                        SelectedDimensionStateV2::Unknown
                    ),
                    (
                        DimensionId::new("region")?,
                        SelectedDimensionStateV2::Unknown
                    ),
                    (
                        DimensionId::new("channel")?,
                        SelectedDimensionStateV2::Unknown,
                    ),
                ],
            ),
            Err(PolicySelectionInputError::UndeclaredDimension)
        );
        // Undeclared known value.
        assert_eq!(
            SelectedProfileInputV2::new(
                &profile,
                vec![
                    (
                        DimensionId::new("locale")?,
                        SelectedDimensionStateV2::Known(SymbolicValueId::new("de")?),
                    ),
                    (
                        DimensionId::new("region")?,
                        SelectedDimensionStateV2::Unknown
                    ),
                ],
            ),
            Err(PolicySelectionInputError::UndeclaredValue)
        );
        Ok(())
    }

    #[test]
    fn policy_profile_set_must_match_exactly() -> Result<(), Box<dyn Error>> {
        let policy = policy(&["audience.default", "audience.enterprise"])?;
        let first = profile("audience.default", 1, false)?;
        let second = profile("audience.enterprise", 1, false)?;

        // Valid exact set.
        let input = PolicySelectionInputV2::new(
            &policy,
            vec![
                selected(&first, SelectedDimensionStateV2::Unknown)?,
                selected(&second, SelectedDimensionStateV2::Unknown)?,
            ],
        )?;
        assert_eq!(input.policy(), &policy);
        assert_eq!(input.profiles().len(), 2);

        // Missing profile.
        assert_eq!(
            PolicySelectionInputV2::new(
                &policy,
                vec![selected(&first, SelectedDimensionStateV2::Unknown)?],
            ),
            Err(PolicySelectionInputError::ProfileSetMismatch)
        );

        // Undeclared extra profile.
        let extra = profile("audience.other", 1, false)?;
        assert_eq!(
            PolicySelectionInputV2::new(
                &policy,
                vec![
                    selected(&first, SelectedDimensionStateV2::Unknown)?,
                    selected(&second, SelectedDimensionStateV2::Unknown)?,
                    selected(&extra, SelectedDimensionStateV2::Unknown)?,
                ],
            ),
            Err(PolicySelectionInputError::ProfileSetMismatch)
        );

        // Duplicate profile ID even across revisions.
        let revision_two = profile("audience.default", 2, false)?;
        assert_eq!(
            PolicySelectionInputV2::new(
                &policy,
                vec![
                    selected(&first, SelectedDimensionStateV2::Unknown)?,
                    selected(&revision_two, SelectedDimensionStateV2::Unknown)?,
                    selected(&second, SelectedDimensionStateV2::Unknown)?,
                ],
            ),
            Err(PolicySelectionInputError::DuplicateProfile)
        );
        Ok(())
    }

    #[test]
    fn invalid_declaration_is_rejected_before_selection() -> Result<(), Box<dyn Error>> {
        // A defensive check that the underlying domain still validates defaults.
        assert_eq!(
            Profile::new(
                ProfileId::new("audience.default")?,
                Revision::new(1)?,
                Title::new("Audience")?,
                vec!["owner://team/product".parse::<OwnerRef>()?],
                vec![dimension("locale", &["en"])?],
                vec![(DimensionId::new("locale")?, SymbolicValueId::new("de")?)],
                None,
                Extensions::default(),
            ),
            Err(PolicyBuildError::InvalidDefault)
        );
        Ok(())
    }
}
