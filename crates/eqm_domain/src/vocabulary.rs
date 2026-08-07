//! Closed ordered domain vocabularies.

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// Lifecycle status in forward transition order.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LifecycleStatus {
    /// Authority is being authored and is not active product intent.
    Draft,
    /// Authority is active product intent.
    Active,
    /// Authority remains recognized but should not gain new active children.
    Deprecated,
    /// Authority is terminal and inactive.
    Retired,
}

impl LifecycleStatus {
    /// Every value in stable wire order.
    pub const ALL: [Self; 4] = [Self::Draft, Self::Active, Self::Deprecated, Self::Retired];

    /// Returns whether a lifecycle may remain or move forward to `candidate`.
    #[must_use]
    pub const fn can_transition_to(self, candidate: Self) -> bool {
        self as u8 <= candidate as u8
    }

    /// Returns whether a child status is valid beneath this parent status.
    #[must_use]
    pub const fn allows_child(self, child: Self) -> bool {
        match self {
            Self::Draft => matches!(child, Self::Draft),
            Self::Active => true,
            Self::Deprecated => matches!(child, Self::Deprecated | Self::Retired),
            Self::Retired => matches!(child, Self::Retired),
        }
    }

    /// Returns the exact wire value.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Active => "active",
            Self::Deprecated => "deprecated",
            Self::Retired => "retired",
        }
    }
}

impl Display for LifecycleStatus {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for LifecycleStatus {
    type Err = VocabularyParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|status| status.as_str() == value)
            .ok_or(VocabularyParseError::InvalidLifecycleStatus)
    }
}

/// Risk class in monotonic elevation order.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RiskClass {
    /// Low risk.
    Low,
    /// Medium risk.
    Medium,
    /// High risk.
    High,
    /// Critical risk.
    Critical,
}

impl RiskClass {
    /// Every value from lowest to highest risk.
    pub const ALL: [Self; 4] = [Self::Low, Self::Medium, Self::High, Self::Critical];

    /// Returns the stronger of two risk classes.
    #[must_use]
    pub const fn elevate(self, candidate: Self) -> Self {
        if self as u8 >= candidate as u8 {
            self
        } else {
            candidate
        }
    }

    /// Returns whether `child` preserves or raises inherited risk.
    #[must_use]
    pub const fn allows_child(self, child: Self) -> bool {
        self as u8 <= child as u8
    }

    /// Returns the exact wire value.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }
}

impl Display for RiskClass {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RiskClass {
    type Err = VocabularyParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|risk| risk.as_str() == value)
            .ok_or(VocabularyParseError::InvalidRiskClass)
    }
}

/// Closed-vocabulary parsing failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VocabularyParseError {
    /// A lifecycle status was not in the v1 set.
    InvalidLifecycleStatus,
    /// A risk class was not in the v1 set.
    InvalidRiskClass,
    /// Requirement level was not in the v1 set.
    InvalidRequirementLevel,
    /// Requirement scope was not in the v1 set.
    InvalidRequirementScope,
    /// Facet was not in the v1 set.
    InvalidFacet,
    /// Artifact role was not in the v1 set.
    InvalidArtifactRole,
    /// HTTP route method was not in the v1 set.
    InvalidHttpMethod,
    /// Intended exposure state was not in the v1 set.
    InvalidIntendedExposureState,
    /// Evidence kind was not in the v1 set.
    InvalidEvidenceKind,
    /// Release channel was not in the v1 set.
    InvalidReleaseChannel,
    /// Trust level was not in the v1 set.
    InvalidTrustLevel,
    /// Attempt outcome was not in the v1 set.
    InvalidAttemptOutcome,
    /// Runner backend was not in the v1 set.
    InvalidRunnerBackend,
    /// Runner guarantee was not in the v1 set.
    InvalidRunnerGuarantee,
    /// Inventory completeness was not in the v1 set.
    InvalidInventoryCompleteness,
}

impl Display for VocabularyParseError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidLifecycleStatus => "invalid lifecycle status",
            Self::InvalidRiskClass => "invalid risk class",
            Self::InvalidRequirementLevel => "invalid requirement level",
            Self::InvalidRequirementScope => "invalid requirement scope",
            Self::InvalidFacet => "invalid facet",
            Self::InvalidArtifactRole => "invalid artifact role",
            Self::InvalidHttpMethod => "invalid HTTP method",
            Self::InvalidIntendedExposureState => "invalid intended exposure state",
            Self::InvalidEvidenceKind => "invalid evidence kind",
            Self::InvalidReleaseChannel => "invalid release channel",
            Self::InvalidTrustLevel => "invalid trust level",
            Self::InvalidAttemptOutcome => "invalid attempt outcome",
            Self::InvalidRunnerBackend => "invalid runner backend",
            Self::InvalidRunnerGuarantee => "invalid runner guarantee",
            Self::InvalidInventoryCompleteness => "invalid inventory completeness",
        })
    }
}

impl Error for VocabularyParseError {}

macro_rules! closed_vocabulary {
    ($(#[$meta:meta])* $name:ident, $error:ident, [$($variant:ident => $wire:literal),+ $(,)?]) => {
        $(#[$meta])*
        #[allow(missing_docs)]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum $name { $($variant),+ }

        impl $name {
            /// Every value in stable wire order.
            pub const ALL: &'static [Self] = &[$(Self::$variant),+];
            /// Returns the exact wire value.
            #[must_use]
            pub const fn as_str(self) -> &'static str {
                match self { $(Self::$variant => $wire),+ }
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl FromStr for $name {
            type Err = VocabularyParseError;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::ALL.iter().copied().find(|item| item.as_str() == value)
                    .ok_or(VocabularyParseError::$error)
            }
        }
    };
}

closed_vocabulary!(
    /// Requirement strength from weakest to strongest.
    RequirementLevel,
    InvalidRequirementLevel,
    [Optional => "optional", Recommended => "recommended", Required => "required"]
);
closed_vocabulary!(
    /// Obligation fan-out scope.
    RequirementScope,
    InvalidRequirementScope,
    [EachTarget => "each_target", SharedProvider => "shared_provider", EndToEnd => "end_to_end"]
);
closed_vocabulary!(
    /// Independently evaluated requirement facet.
    Facet,
    InvalidFacet,
    [
        Structure => "structure",
        Reachability => "reachability",
        Behavior => "behavior",
        Accessibility => "accessibility",
        Visual => "visual",
        Analytics => "analytics",
        RuntimeExposure => "runtime_exposure",
        ReleasePresence => "release_presence"
    ]
);
closed_vocabulary!(
    /// Runner execution backend.
    RunnerBackend,
    InvalidRunnerBackend,
    [Local => "local", Container => "container"]
);
closed_vocabulary!(
    /// A backend-enforced runner guarantee.
    RunnerGuarantee,
    InvalidRunnerGuarantee,
    [
        NetworkDenied => "network_denied", ReadOnlySource => "read_only_source",
        IsolatedProcess => "isolated_process", ResourceLimited => "resource_limited"
    ]
);
closed_vocabulary!(
    /// Inventory completeness claim.
    InventoryCompleteness,
    InvalidInventoryCompleteness,
    [Complete => "complete", Partial => "partial", Unknown => "unknown"]
);
closed_vocabulary!(
    /// The semantic role of a bound artifact.
    ArtifactRole,
    InvalidArtifactRole,
    [
        Entrypoint => "entrypoint", View => "view", Route => "route", Component => "component",
        Service => "service", Test => "test", Configuration => "configuration", Asset => "asset"
    ]
);
closed_vocabulary!(
    /// A provider-neutral HTTP route method.
    HttpMethod,
    InvalidHttpMethod,
    [Get => "get", Post => "post", Put => "put", Patch => "patch", Delete => "delete", Options => "options"]
);
closed_vocabulary!(
    /// Profile-relative intended exposure state.
    IntendedExposureState,
    InvalidIntendedExposureState,
    [Required => "required", Prohibited => "prohibited"]
);
closed_vocabulary!(
    /// Kind of expected or observed evidence.
    EvidenceKind,
    InvalidEvidenceKind,
    [
        StructuralCheck => "structural_check", StaticInventory => "static_inventory",
        Test => "test", Snapshot => "snapshot", ManualReview => "manual_review",
        RuntimeSnapshot => "runtime_snapshot", ReleaseRecord => "release_record"
    ]
);
closed_vocabulary!(
    /// Release channel identity without maturity ordering.
    ReleaseChannel,
    InvalidReleaseChannel,
    [Development => "development", Internal => "internal", Beta => "beta", Production => "production"]
);
closed_vocabulary!(
    /// Claimed or effective evidence trust from weakest to strongest.
    TrustLevel,
    InvalidTrustLevel,
    [UntrustedLocal => "untrusted_local", TrustedCi => "trusted_ci", SignedCi => "signed_ci"]
);
closed_vocabulary!(
    /// One immutable execution attempt outcome.
    AttemptOutcome,
    InvalidAttemptOutcome,
    [
        Passed => "passed", Failed => "failed", Skipped => "skipped", Filtered => "filtered",
        Quarantined => "quarantined", TimedOut => "timed_out", Cancelled => "cancelled", Error => "error"
    ]
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_values_round_trip_and_reject_unknowns() {
        for value in LifecycleStatus::ALL {
            assert_eq!(value.to_string().parse(), Ok(value));
        }
        for invalid in ["", "ACTIVE", "inactive", "removed", "deprecated "] {
            assert_eq!(
                invalid.parse::<LifecycleStatus>(),
                Err(VocabularyParseError::InvalidLifecycleStatus)
            );
        }
    }

    #[test]
    fn lifecycle_transitions_only_move_forward() {
        for from in LifecycleStatus::ALL {
            for to in LifecycleStatus::ALL {
                assert_eq!(from.can_transition_to(to), from <= to);
            }
        }
        assert!(!LifecycleStatus::Retired.can_transition_to(LifecycleStatus::Active));
    }

    #[test]
    fn parent_lifecycle_prevents_active_children_below_inactive_authority() {
        assert!(LifecycleStatus::Active.allows_child(LifecycleStatus::Draft));
        assert!(!LifecycleStatus::Draft.allows_child(LifecycleStatus::Active));
        assert!(!LifecycleStatus::Deprecated.allows_child(LifecycleStatus::Active));
        assert!(LifecycleStatus::Deprecated.allows_child(LifecycleStatus::Retired));
        assert!(!LifecycleStatus::Retired.allows_child(LifecycleStatus::Deprecated));
    }

    #[test]
    fn risk_values_round_trip_order_and_elevate() {
        for value in RiskClass::ALL {
            assert_eq!(value.to_string().parse(), Ok(value));
            for child in RiskClass::ALL {
                assert_eq!(value.allows_child(child), value <= child);
                assert_eq!(value.elevate(child), std::cmp::max(value, child));
            }
        }
        for invalid in ["", "LOW", "severe", "critical "] {
            assert_eq!(
                invalid.parse::<RiskClass>(),
                Err(VocabularyParseError::InvalidRiskClass)
            );
        }
    }

    #[test]
    fn requirement_vocabularies_are_closed_and_ordered() {
        assert!(RequirementLevel::Optional < RequirementLevel::Required);
        for value in RequirementLevel::ALL {
            assert_eq!(value.as_str().parse(), Ok(*value));
        }
        for value in RequirementScope::ALL {
            assert_eq!(value.as_str().parse(), Ok(*value));
        }
        for value in Facet::ALL {
            assert_eq!(value.as_str().parse(), Ok(*value));
        }
        assert_eq!(
            "mandatory".parse::<RequirementLevel>(),
            Err(VocabularyParseError::InvalidRequirementLevel)
        );
        assert_eq!(
            "per_target".parse::<RequirementScope>(),
            Err(VocabularyParseError::InvalidRequirementScope)
        );
        assert_eq!(
            "security".parse::<Facet>(),
            Err(VocabularyParseError::InvalidFacet)
        );
    }

    #[test]
    fn artifact_vocabularies_are_closed() {
        for value in ArtifactRole::ALL {
            assert_eq!(value.as_str().parse(), Ok(*value));
        }
        for value in HttpMethod::ALL {
            assert_eq!(value.as_str().parse(), Ok(*value));
        }
        assert_eq!(
            "page".parse::<ArtifactRole>(),
            Err(VocabularyParseError::InvalidArtifactRole)
        );
        assert_eq!(
            "head".parse::<HttpMethod>(),
            Err(VocabularyParseError::InvalidHttpMethod)
        );
        for value in IntendedExposureState::ALL {
            assert_eq!(value.as_str().parse(), Ok(*value));
        }
        assert_eq!(
            "optional".parse::<IntendedExposureState>(),
            Err(VocabularyParseError::InvalidIntendedExposureState)
        );
    }
}
