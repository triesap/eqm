//! Exact-context required-target equivalence.

use crate::TargetConformance;
use eqm_domain::{Sha256Digest, TargetId, WaiverId};
use std::collections::{BTreeMap, BTreeSet};

/// One prepared target conclusion with its exact evaluation context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetEvaluation {
    /// Exact context digest, absent when target evaluation did not complete.
    pub context_digest: Option<Sha256Digest>,
    /// Target conformance, absent when evaluation failed.
    pub conformance: Option<TargetConformance>,
    /// Visible waivers contributing to conditional conformance.
    pub waivers: BTreeSet<WaiverId>,
}

/// Closed target-set equivalence result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EquivalenceStatus {
    /// Every required target is conformant in one exact context.
    Equivalent,
    /// At least one required target is conditionally conformant and none is nonconformant.
    ConditionallyEquivalent,
    /// At least one required target is nonconformant.
    NotEquivalent,
    /// Required results or exact context are absent, mismatched, or invalid.
    Unknown,
}

/// Complete equivalence result and visible contributing metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EquivalenceReport {
    /// Overall equivalence.
    pub status: EquivalenceStatus,
    /// Every waiver contributing to conditional equivalence.
    pub waivers: BTreeSet<WaiverId>,
    /// Evaluated targets outside the required set.
    pub extra_targets: BTreeSet<TargetId>,
}

/// Derives equivalence from one complete required-target conformance set.
#[must_use]
pub fn evaluate_target_set_equivalence(
    required_targets: &BTreeSet<TargetId>,
    expected_context: Sha256Digest,
    evaluations: &BTreeMap<TargetId, TargetEvaluation>,
) -> EquivalenceReport {
    let extra_targets = evaluations
        .keys()
        .filter(|target| !required_targets.contains(*target))
        .cloned()
        .collect();
    let mut required = Vec::new();
    for target in required_targets {
        let Some(evaluation) = evaluations.get(target) else {
            return unknown(extra_targets);
        };
        if evaluation.context_digest != Some(expected_context) || evaluation.conformance.is_none() {
            return unknown(extra_targets);
        }
        required.push(evaluation);
    }
    if required
        .iter()
        .any(|result| result.conformance == Some(TargetConformance::Nonconformant))
    {
        return EquivalenceReport {
            status: EquivalenceStatus::NotEquivalent,
            waivers: BTreeSet::new(),
            extra_targets,
        };
    }
    let conditional: Vec<_> = required
        .iter()
        .filter(|result| result.conformance == Some(TargetConformance::ConditionallyConformant))
        .collect();
    if conditional.is_empty() {
        EquivalenceReport {
            status: EquivalenceStatus::Equivalent,
            waivers: BTreeSet::new(),
            extra_targets,
        }
    } else if conditional.iter().any(|result| result.waivers.is_empty()) {
        unknown(extra_targets)
    } else {
        EquivalenceReport {
            status: EquivalenceStatus::ConditionallyEquivalent,
            waivers: conditional
                .into_iter()
                .flat_map(|result| result.waivers.iter().cloned())
                .collect(),
            extra_targets,
        }
    }
}

fn unknown(extra_targets: BTreeSet<TargetId>) -> EquivalenceReport {
    EquivalenceReport {
        status: EquivalenceStatus::Unknown,
        waivers: BTreeSet::new(),
        extra_targets,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    fn evaluation(
        context: Sha256Digest,
        conformance: TargetConformance,
        waiver: Option<&str>,
    ) -> Result<TargetEvaluation, Box<dyn Error>> {
        Ok(TargetEvaluation {
            context_digest: Some(context),
            conformance: Some(conformance),
            waivers: waiver.map(WaiverId::new).transpose()?.into_iter().collect(),
        })
    }

    #[test]
    fn complete_three_target_table_and_preconditions_are_exact() -> Result<(), Box<dyn Error>> {
        let context = Sha256Digest::from_bytes([1; 32]);
        let required = BTreeSet::from([
            TargetId::new("android")?,
            TargetId::new("ios")?,
            TargetId::new("web")?,
        ]);
        let mut results = BTreeMap::new();
        for target in &required {
            results.insert(
                target.clone(),
                evaluation(context, TargetConformance::Conformant, None)?,
            );
        }
        assert_eq!(
            evaluate_target_set_equivalence(&required, context, &results).status,
            EquivalenceStatus::Equivalent
        );
        results.insert(
            TargetId::new("ios")?,
            evaluation(
                context,
                TargetConformance::ConditionallyConformant,
                Some("waiver.ios"),
            )?,
        );
        let conditional = evaluate_target_set_equivalence(&required, context, &results);
        assert_eq!(
            conditional.status,
            EquivalenceStatus::ConditionallyEquivalent
        );
        assert_eq!(conditional.waivers.len(), 1);
        results.insert(
            TargetId::new("android")?,
            evaluation(context, TargetConformance::Nonconformant, None)?,
        );
        assert_eq!(
            evaluate_target_set_equivalence(&required, context, &results).status,
            EquivalenceStatus::NotEquivalent
        );
        results.remove(&TargetId::new("web")?);
        assert_eq!(
            evaluate_target_set_equivalence(&required, context, &results).status,
            EquivalenceStatus::Unknown
        );
        results.insert(
            TargetId::new("web")?,
            evaluation(
                Sha256Digest::from_bytes([2; 32]),
                TargetConformance::Conformant,
                None,
            )?,
        );
        assert_eq!(
            evaluate_target_set_equivalence(&required, context, &results).status,
            EquivalenceStatus::Unknown
        );
        Ok(())
    }

    #[test]
    fn extra_targets_are_reported_without_changing_required_result() -> Result<(), Box<dyn Error>> {
        let context = Sha256Digest::from_bytes([1; 32]);
        let required = BTreeSet::from([TargetId::new("web")?]);
        let results = BTreeMap::from([
            (
                TargetId::new("web")?,
                evaluation(context, TargetConformance::Conformant, None)?,
            ),
            (
                TargetId::new("preview")?,
                evaluation(context, TargetConformance::Nonconformant, None)?,
            ),
        ]);
        let report = evaluate_target_set_equivalence(&required, context, &results);
        assert_eq!(report.status, EquivalenceStatus::Equivalent);
        assert_eq!(
            report.extra_targets,
            BTreeSet::from([TargetId::new("preview")?])
        );
        Ok(())
    }
}
