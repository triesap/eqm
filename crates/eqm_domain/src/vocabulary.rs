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
}

impl Display for VocabularyParseError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidLifecycleStatus => "invalid lifecycle status",
            Self::InvalidRiskClass => "invalid risk class",
        })
    }
}

impl Error for VocabularyParseError {}

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
}
