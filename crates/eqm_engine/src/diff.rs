//! Deterministic explicit-baseline semantic diff classification.

use eqm_domain::{Facet, FullRequirementId, TargetId, UnitId};
use std::collections::{BTreeMap, BTreeSet};

/// Semantic field class controlling normative diff classification.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SemanticFieldClass {
    /// Requirement presence or strength.
    Requirement,
    /// Required-target presence.
    Target,
    /// Required-facet presence.
    Facet,
    /// Ordered policy value.
    OrderedPolicy,
    /// Evidence specification, runner, adapter, or trust input.
    Evidence,
    /// Waiver authority.
    Waiver,
    /// Intended exposure.
    Exposure,
    /// Canonically excluded metadata.
    Metadata,
    /// Entity without ordered policy meaning.
    Entity,
}

/// Stable semantic field coordinate.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SemanticCoordinate {
    /// Optional unit coordinate.
    pub unit: Option<UnitId>,
    /// Optional requirement coordinate.
    pub requirement: Option<FullRequirementId>,
    /// Optional target coordinate.
    pub target: Option<TargetId>,
    /// Optional facet coordinate.
    pub facet: Option<Facet>,
    /// Field classification.
    pub class: SemanticFieldClass,
    /// Stable field name.
    pub field: Box<str>,
}

/// Canonical prepared value used only for equality and ordered comparisons.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SemanticValue {
    /// Opaque canonical semantic value.
    Opaque(Box<str>),
    /// Ordered nonnegative strength value.
    Strength(u64),
}

/// One explicit finalized semantic projection.
pub type SemanticProjection = BTreeMap<SemanticCoordinate, SemanticValue>;

/// Closed semantic change classification.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SemanticChangeKind {
    /// Ordered obligation strength increased or a protected axis was added.
    Strengthened,
    /// Ordered obligation strength decreased or a protected axis was removed.
    Weakened,
    /// Unordered entity was added.
    Added,
    /// Unordered entity was removed.
    Removed,
    /// Evidence, runner, adapter, or trust input changed.
    Evidence,
    /// Waiver authority changed.
    Waiver,
    /// Intended exposure changed.
    Exposure,
    /// Only canonically excluded metadata changed.
    Nonnormative,
}

/// One deterministic baseline-to-candidate change.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticChange {
    /// Changed coordinate.
    pub coordinate: SemanticCoordinate,
    /// Classification.
    pub kind: SemanticChangeKind,
    /// Baseline value.
    pub before: Option<SemanticValue>,
    /// Candidate value.
    pub after: Option<SemanticValue>,
}

/// Classifies changes between two explicit finalized projections.
#[must_use]
pub fn classify_diffs(
    baseline: &SemanticProjection,
    candidate: &SemanticProjection,
) -> Vec<SemanticChange> {
    let coordinates: BTreeSet<_> = baseline.keys().chain(candidate.keys()).cloned().collect();
    let mut changes = Vec::new();
    for coordinate in coordinates {
        let before = baseline.get(&coordinate);
        let after = candidate.get(&coordinate);
        match (before, after) {
            (Some(before), Some(after)) if before == after => {}
            (None, Some(after)) => {
                let kind = addition_kind(coordinate.class);
                changes.push(change(coordinate, kind, None, Some(after.clone())));
            }
            (Some(before), None) => {
                let kind = removal_kind(coordinate.class);
                changes.push(change(coordinate, kind, Some(before.clone()), None));
            }
            (Some(before), Some(after)) => match retained_kind(coordinate.class, before, after) {
                Some(kind) => changes.push(change(
                    coordinate,
                    kind,
                    Some(before.clone()),
                    Some(after.clone()),
                )),
                None => {
                    let removed = removal_kind(coordinate.class);
                    let added = addition_kind(coordinate.class);
                    changes.push(change(
                        coordinate.clone(),
                        removed,
                        Some(before.clone()),
                        None,
                    ));
                    changes.push(change(coordinate, added, None, Some(after.clone())));
                }
            },
            (None, None) => {}
        }
    }
    changes.sort_by(|left, right| {
        (
            &left.coordinate.unit,
            &left.coordinate.requirement,
            &left.coordinate.target,
            left.coordinate.facet,
            left.kind,
            &left.coordinate.field,
        )
            .cmp(&(
                &right.coordinate.unit,
                &right.coordinate.requirement,
                &right.coordinate.target,
                right.coordinate.facet,
                right.kind,
                &right.coordinate.field,
            ))
    });
    changes
}

fn change(
    coordinate: SemanticCoordinate,
    kind: SemanticChangeKind,
    before: Option<SemanticValue>,
    after: Option<SemanticValue>,
) -> SemanticChange {
    SemanticChange {
        coordinate,
        kind,
        before,
        after,
    }
}

const fn addition_kind(class: SemanticFieldClass) -> SemanticChangeKind {
    match class {
        SemanticFieldClass::Requirement
        | SemanticFieldClass::Target
        | SemanticFieldClass::Facet
        | SemanticFieldClass::OrderedPolicy => SemanticChangeKind::Strengthened,
        SemanticFieldClass::Evidence => SemanticChangeKind::Evidence,
        SemanticFieldClass::Waiver => SemanticChangeKind::Waiver,
        SemanticFieldClass::Exposure => SemanticChangeKind::Exposure,
        SemanticFieldClass::Metadata => SemanticChangeKind::Nonnormative,
        SemanticFieldClass::Entity => SemanticChangeKind::Added,
    }
}

const fn removal_kind(class: SemanticFieldClass) -> SemanticChangeKind {
    match class {
        SemanticFieldClass::Requirement
        | SemanticFieldClass::Target
        | SemanticFieldClass::Facet
        | SemanticFieldClass::OrderedPolicy => SemanticChangeKind::Weakened,
        SemanticFieldClass::Evidence => SemanticChangeKind::Evidence,
        SemanticFieldClass::Waiver => SemanticChangeKind::Waiver,
        SemanticFieldClass::Exposure => SemanticChangeKind::Exposure,
        SemanticFieldClass::Metadata => SemanticChangeKind::Nonnormative,
        SemanticFieldClass::Entity => SemanticChangeKind::Removed,
    }
}

fn retained_kind(
    class: SemanticFieldClass,
    before: &SemanticValue,
    after: &SemanticValue,
) -> Option<SemanticChangeKind> {
    match class {
        SemanticFieldClass::OrderedPolicy
        | SemanticFieldClass::Requirement
        | SemanticFieldClass::Target
        | SemanticFieldClass::Facet => match (before, after) {
            (SemanticValue::Strength(before), SemanticValue::Strength(after)) => {
                Some(if after > before {
                    SemanticChangeKind::Strengthened
                } else {
                    SemanticChangeKind::Weakened
                })
            }
            _ => None,
        },
        SemanticFieldClass::Evidence => Some(SemanticChangeKind::Evidence),
        SemanticFieldClass::Waiver => Some(SemanticChangeKind::Waiver),
        SemanticFieldClass::Exposure => Some(SemanticChangeKind::Exposure),
        SemanticFieldClass::Metadata => Some(SemanticChangeKind::Nonnormative),
        SemanticFieldClass::Entity => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    fn coordinate(
        class: SemanticFieldClass,
        field: &str,
    ) -> Result<SemanticCoordinate, Box<dyn Error>> {
        Ok(SemanticCoordinate {
            unit: Some(UnitId::new("account.create.signup.form")?),
            requirement: Some(FullRequirementId::new("account.create.signup.form#submit")?),
            target: Some(TargetId::new("web")?),
            facet: Some(Facet::Behavior),
            class,
            field: field.into(),
        })
    }

    #[test]
    fn classifications_are_complete_sorted_and_directionally_symmetric()
    -> Result<(), Box<dyn Error>> {
        let ordered = coordinate(SemanticFieldClass::OrderedPolicy, "minimum_trust")?;
        let evidence = coordinate(SemanticFieldClass::Evidence, "runner")?;
        let entity = coordinate(SemanticFieldClass::Entity, "entity")?;
        let metadata = coordinate(SemanticFieldClass::Metadata, "description")?;
        let baseline = BTreeMap::from([
            (ordered.clone(), SemanticValue::Strength(1)),
            (evidence.clone(), SemanticValue::Opaque("runner-a".into())),
            (entity.clone(), SemanticValue::Opaque("present".into())),
            (metadata.clone(), SemanticValue::Opaque("before".into())),
        ]);
        let added = coordinate(SemanticFieldClass::Requirement, "requirement")?;
        let candidate = BTreeMap::from([
            (ordered, SemanticValue::Strength(2)),
            (evidence, SemanticValue::Opaque("runner-b".into())),
            (metadata, SemanticValue::Opaque("after".into())),
            (added, SemanticValue::Opaque("present".into())),
        ]);
        let forward = classify_diffs(&baseline, &candidate);
        let reverse = classify_diffs(&candidate, &baseline);
        assert!(
            forward
                .iter()
                .any(|change| change.kind == SemanticChangeKind::Strengthened)
        );
        assert!(
            forward
                .iter()
                .any(|change| change.kind == SemanticChangeKind::Evidence)
        );
        assert!(
            forward
                .iter()
                .any(|change| change.kind == SemanticChangeKind::Removed)
        );
        assert!(
            forward
                .iter()
                .any(|change| change.kind == SemanticChangeKind::Nonnormative)
        );
        assert!(
            reverse
                .iter()
                .any(|change| change.kind == SemanticChangeKind::Weakened)
        );
        for window in forward.windows(2) {
            let left = &window[0];
            let right = &window[1];
            assert!(
                (
                    &left.coordinate.unit,
                    &left.coordinate.requirement,
                    &left.coordinate.target,
                    left.coordinate.facet,
                    left.kind,
                    &left.coordinate.field,
                ) <= (
                    &right.coordinate.unit,
                    &right.coordinate.requirement,
                    &right.coordinate.target,
                    right.coordinate.facet,
                    right.kind,
                    &right.coordinate.field,
                )
            );
        }
        Ok(())
    }
}
