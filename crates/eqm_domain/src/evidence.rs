//! Evidence specification values, independent from immutable evidence results.

use crate::{
    DurationMillis, EvidenceKind, EvidenceSpecId, Extensions, Facet, FullRequirementId, HttpMethod,
    ReleaseChannel, RunnerId, SelectorText,
};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::NonZeroU64;

/// A positive evidence match or observation count.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PositiveCount(NonZeroU64);

impl PositiveCount {
    /// The normalized default count.
    pub const ONE: Self = Self(NonZeroU64::MIN);

    /// Creates a positive count.
    pub fn new(value: u64) -> Result<Self, EvidenceSpecBuildError> {
        NonZeroU64::new(value)
            .map(Self)
            .ok_or(EvidenceSpecBuildError::CountMustBePositive)
    }

    /// Returns the exact positive value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

/// A typed provider-neutral evidence selector.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceSelector {
    /// A source symbol.
    Symbol {
        /// Exact symbol name.
        name: SelectorText,
        /// Optional source language.
        language: Option<SelectorText>,
    },
    /// A route coordinate.
    Route {
        /// Provider-neutral route path.
        path: SelectorText,
        /// Optional closed HTTP method.
        method: Option<HttpMethod>,
    },
    /// A test coordinate.
    Test {
        /// Test framework identity.
        framework: SelectorText,
        /// Exact framework test identity.
        test_id: SelectorText,
        /// Optional suite identity.
        suite: Option<SelectorText>,
    },
    /// A provider-neutral inventory coordinate.
    Inventory {
        /// Record type.
        record_type: SelectorText,
        /// Record key.
        key: SelectorText,
        /// Optional expected value.
        value: Option<SelectorText>,
    },
    /// A snapshot coordinate.
    Snapshot {
        /// Exact snapshot identity.
        snapshot_id: SelectorText,
        /// Optional variant identity.
        variant: Option<SelectorText>,
    },
    /// A release channel coordinate.
    Release {
        /// Exact release channel.
        channel: ReleaseChannel,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SelectorKind {
    Symbol,
    Route,
    Test,
    Inventory,
    Snapshot,
    Release,
}

impl EvidenceSelector {
    const fn kind(&self) -> SelectorKind {
        match self {
            Self::Symbol { .. } => SelectorKind::Symbol,
            Self::Route { .. } => SelectorKind::Route,
            Self::Test { .. } => SelectorKind::Test,
            Self::Inventory { .. } => SelectorKind::Inventory,
            Self::Snapshot { .. } => SelectorKind::Snapshot,
            Self::Release { .. } => SelectorKind::Release,
        }
    }
}

/// A source-controlled declaration of expected evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceSpecification {
    id: EvidenceSpecId,
    kind: EvidenceKind,
    requirements: BTreeSet<FullRequirementId>,
    facets: BTreeSet<Facet>,
    runner: Option<RunnerId>,
    selector: Option<EvidenceSelector>,
    minimum_count: Option<PositiveCount>,
    freshness: Option<DurationMillis>,
    extensions: Extensions,
}

impl EvidenceSpecification {
    /// Creates an expected-evidence declaration with kind-compatible fields.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: EvidenceSpecId,
        kind: EvidenceKind,
        requirements: Vec<FullRequirementId>,
        facets: Vec<Facet>,
        runner: Option<RunnerId>,
        selector: Option<EvidenceSelector>,
        minimum_count: Option<PositiveCount>,
        freshness: Option<DurationMillis>,
        extensions: Extensions,
    ) -> Result<Self, EvidenceSpecBuildError> {
        let requirements = unique_nonempty(
            requirements,
            EvidenceSpecBuildError::RequirementsRequired,
            EvidenceSpecBuildError::DuplicateRequirement,
        )?;
        let facets = unique_nonempty(
            facets,
            EvidenceSpecBuildError::FacetsRequired,
            EvidenceSpecBuildError::DuplicateFacet,
        )?;
        if kind.is_executable() != runner.is_some() {
            return Err(if kind.is_executable() {
                EvidenceSpecBuildError::RunnerRequired
            } else {
                EvidenceSpecBuildError::RunnerForbidden
            });
        }
        if !selector_is_compatible(kind, selector.as_ref()) {
            return Err(EvidenceSpecBuildError::IncompatibleSelector);
        }
        let minimum_count = if kind.is_countable() {
            Some(minimum_count.unwrap_or(PositiveCount::ONE))
        } else if minimum_count.is_some() {
            return Err(EvidenceSpecBuildError::MinimumCountForbidden);
        } else {
            None
        };
        Ok(Self {
            id,
            kind,
            requirements,
            facets,
            runner,
            selector,
            minimum_count,
            freshness,
            extensions,
        })
    }

    /// Returns the local evidence-specification ID.
    #[must_use]
    pub const fn id(&self) -> &EvidenceSpecId {
        &self.id
    }
    /// Returns the evidence kind.
    #[must_use]
    pub const fn kind(&self) -> EvidenceKind {
        self.kind
    }
    /// Returns covered requirements in canonical order.
    #[must_use]
    pub const fn requirements(&self) -> &BTreeSet<FullRequirementId> {
        &self.requirements
    }
    /// Returns covered facets in canonical order.
    #[must_use]
    pub const fn facets(&self) -> &BTreeSet<Facet> {
        &self.facets
    }
    /// Returns the executable runner when required.
    #[must_use]
    pub const fn runner(&self) -> Option<&RunnerId> {
        self.runner.as_ref()
    }
    /// Returns the kind-compatible selector.
    #[must_use]
    pub const fn selector(&self) -> Option<&EvidenceSelector> {
        self.selector.as_ref()
    }
    /// Returns the normalized count for countable kinds.
    #[must_use]
    pub const fn minimum_count(&self) -> Option<PositiveCount> {
        self.minimum_count
    }
    /// Returns the optional freshness ceiling.
    #[must_use]
    pub const fn freshness(&self) -> Option<DurationMillis> {
        self.freshness
    }
    /// Returns extensions.
    #[must_use]
    pub const fn extensions(&self) -> &Extensions {
        &self.extensions
    }
}

impl EvidenceKind {
    const fn is_executable(self) -> bool {
        matches!(self, Self::StructuralCheck | Self::Test | Self::Snapshot)
    }

    const fn is_countable(self) -> bool {
        matches!(
            self,
            Self::StructuralCheck | Self::StaticInventory | Self::Test | Self::RuntimeSnapshot
        )
    }
}

fn selector_is_compatible(kind: EvidenceKind, selector: Option<&EvidenceSelector>) -> bool {
    matches!(
        (kind, selector.map(EvidenceSelector::kind)),
        (
            EvidenceKind::StructuralCheck,
            Some(SelectorKind::Symbol | SelectorKind::Route | SelectorKind::Inventory),
        ) | (
            EvidenceKind::StaticInventory | EvidenceKind::RuntimeSnapshot,
            Some(SelectorKind::Inventory),
        ) | (EvidenceKind::Test, Some(SelectorKind::Test))
            | (EvidenceKind::Snapshot, Some(SelectorKind::Snapshot))
            | (EvidenceKind::ReleaseRecord, Some(SelectorKind::Release))
            | (EvidenceKind::ManualReview, None)
    )
}

fn unique_nonempty<T: Ord>(
    values: Vec<T>,
    empty: EvidenceSpecBuildError,
    duplicate: EvidenceSpecBuildError,
) -> Result<BTreeSet<T>, EvidenceSpecBuildError> {
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

/// Evidence-specification construction failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceSpecBuildError {
    /// No covered requirements were declared.
    RequirementsRequired,
    /// A covered requirement appeared more than once.
    DuplicateRequirement,
    /// No covered facets were declared.
    FacetsRequired,
    /// A covered facet appeared more than once.
    DuplicateFacet,
    /// An executable kind omitted its runner.
    RunnerRequired,
    /// A non-executable kind declared a runner.
    RunnerForbidden,
    /// The selector was absent or incompatible with the evidence kind.
    IncompatibleSelector,
    /// A non-countable kind declared a minimum count.
    MinimumCountForbidden,
    /// A count was zero.
    CountMustBePositive,
}

impl Display for EvidenceSpecBuildError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

impl Error for EvidenceSpecBuildError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn requirement() -> Result<FullRequirementId, crate::IdParseError> {
        FullRequirementId::new("account.create.signup.start#reachable")
    }

    #[test]
    fn executable_test_normalizes_count() -> Result<(), Box<dyn Error>> {
        let specification = EvidenceSpecification::new(
            EvidenceSpecId::new("signup_test")?,
            EvidenceKind::Test,
            vec![requirement()?],
            vec![Facet::Behavior],
            Some(RunnerId::new("runner.cargo")?),
            Some(EvidenceSelector::Test {
                framework: SelectorText::new("cargo")?,
                test_id: SelectorText::new("signup")?,
                suite: None,
            }),
            None,
            Some(DurationMillis::new(60_000)?),
            Extensions::default(),
        )?;
        assert_eq!(specification.minimum_count(), Some(PositiveCount::ONE));
        assert!(specification.runner().is_some());
        Ok(())
    }

    #[test]
    fn kind_field_rules_fail_closed() -> Result<(), Box<dyn Error>> {
        let build = |kind, runner, selector, minimum_count| {
            EvidenceSpecification::new(
                EvidenceSpecId::new("review")
                    .map_err(|_| EvidenceSpecBuildError::RequirementsRequired)?,
                kind,
                vec![requirement().map_err(|_| EvidenceSpecBuildError::RequirementsRequired)?],
                vec![Facet::Behavior],
                runner,
                selector,
                minimum_count,
                None,
                Extensions::default(),
            )
        };
        assert!(matches!(
            build(EvidenceKind::Test, None, None, None),
            Err(EvidenceSpecBuildError::RunnerRequired)
        ));
        assert!(matches!(
            build(
                EvidenceKind::ManualReview,
                Some(RunnerId::new("runner.review")?),
                None,
                None,
            ),
            Err(EvidenceSpecBuildError::RunnerForbidden)
        ));
        assert!(matches!(
            build(
                EvidenceKind::ManualReview,
                None,
                None,
                Some(PositiveCount::ONE)
            ),
            Err(EvidenceSpecBuildError::MinimumCountForbidden)
        ));
        assert_eq!(
            PositiveCount::new(0),
            Err(EvidenceSpecBuildError::CountMustBePositive)
        );
        Ok(())
    }
}
