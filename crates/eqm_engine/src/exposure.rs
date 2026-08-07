//! Independent expected, declared, discovered, enabled, released, and conformance facts.

/// Policy-relative expected exposure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExpectedExposure {
    /// Exposure is required.
    Required,
    /// Exposure is prohibited.
    Prohibited,
    /// Expected state is unknown.
    Unknown,
}

/// One independently observed Boolean fact.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservedExposure {
    /// Authoritative observation is true.
    True,
    /// Authoritative complete observation is false.
    False,
    /// Input is partial, failed, missing, stale, or otherwise unknown.
    Unknown,
}

/// Target conformance reported alongside, never substituted for, exposure facts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConformanceFact {
    /// Target is conformant.
    True,
    /// Target is conditionally conformant.
    Conditional,
    /// Target is nonconformant.
    False,
    /// Target result is unavailable.
    Unknown,
}

/// Complete independent exposure facts for one exact coordinate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExposureFacts {
    /// Expected exposure.
    pub expected: ExpectedExposure,
    /// Binding-declared exposure.
    pub declared: ObservedExposure,
    /// Prepared inventory observation.
    pub discovered: ObservedExposure,
    /// Prepared runtime-facts observation.
    pub enabled: ObservedExposure,
    /// Exact release-record observation.
    pub released: ObservedExposure,
    /// Target conformance alongside the facts.
    pub conformant: ConformanceFact,
}

/// Comparison of one observed fact to expectation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExposureComparison {
    /// Observed Boolean agrees with required/prohibited expectation.
    Match,
    /// Observed Boolean contradicts required/prohibited expectation.
    Mismatch,
    /// Expected or observed input is unknown.
    Unknown,
}

/// Independent comparisons with the original facts preserved.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExposureReconciliation {
    /// Unmodified input facts.
    pub facts: ExposureFacts,
    /// Declared comparison.
    pub declared: ExposureComparison,
    /// Discovered comparison.
    pub discovered: ExposureComparison,
    /// Enabled comparison.
    pub enabled: ExposureComparison,
    /// Released comparison.
    pub released: ExposureComparison,
}

/// Reconciles each Boolean fact independently; no fact implies another.
#[must_use]
pub fn reconcile_exposure(facts: ExposureFacts) -> ExposureReconciliation {
    ExposureReconciliation {
        facts,
        declared: compare(facts.expected, facts.declared),
        discovered: compare(facts.expected, facts.discovered),
        enabled: compare(facts.expected, facts.enabled),
        released: compare(facts.expected, facts.released),
    }
}

const fn compare(expected: ExpectedExposure, observed: ObservedExposure) -> ExposureComparison {
    match (expected, observed) {
        (ExpectedExposure::Unknown, _) | (_, ObservedExposure::Unknown) => {
            ExposureComparison::Unknown
        }
        (ExpectedExposure::Required, ObservedExposure::True)
        | (ExpectedExposure::Prohibited, ObservedExposure::False) => ExposureComparison::Match,
        (ExpectedExposure::Required, ObservedExposure::False)
        | (ExpectedExposure::Prohibited, ObservedExposure::True) => ExposureComparison::Mismatch,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_observed_cross_product_matches_the_normative_table() {
        let cases = [
            (
                ExpectedExposure::Required,
                ObservedExposure::True,
                ExposureComparison::Match,
            ),
            (
                ExpectedExposure::Required,
                ObservedExposure::False,
                ExposureComparison::Mismatch,
            ),
            (
                ExpectedExposure::Required,
                ObservedExposure::Unknown,
                ExposureComparison::Unknown,
            ),
            (
                ExpectedExposure::Prohibited,
                ObservedExposure::True,
                ExposureComparison::Mismatch,
            ),
            (
                ExpectedExposure::Prohibited,
                ObservedExposure::False,
                ExposureComparison::Match,
            ),
            (
                ExpectedExposure::Prohibited,
                ObservedExposure::Unknown,
                ExposureComparison::Unknown,
            ),
            (
                ExpectedExposure::Unknown,
                ObservedExposure::True,
                ExposureComparison::Unknown,
            ),
            (
                ExpectedExposure::Unknown,
                ObservedExposure::False,
                ExposureComparison::Unknown,
            ),
            (
                ExpectedExposure::Unknown,
                ObservedExposure::Unknown,
                ExposureComparison::Unknown,
            ),
        ];
        for (expected, observed, result) in cases {
            assert_eq!(compare(expected, observed), result);
        }
    }

    #[test]
    fn facts_remain_independent_and_conformance_overwrites_nothing() {
        let facts = ExposureFacts {
            expected: ExpectedExposure::Prohibited,
            declared: ObservedExposure::False,
            discovered: ObservedExposure::True,
            enabled: ObservedExposure::Unknown,
            released: ObservedExposure::False,
            conformant: ConformanceFact::True,
        };
        let result = reconcile_exposure(facts);
        assert_eq!(result.facts, facts);
        assert_eq!(result.declared, ExposureComparison::Match);
        assert_eq!(result.discovered, ExposureComparison::Mismatch);
        assert_eq!(result.enabled, ExposureComparison::Unknown);
        assert_eq!(result.released, ExposureComparison::Match);
    }
}
