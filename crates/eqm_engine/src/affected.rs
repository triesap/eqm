//! Conservative affected-set analysis over explicit reverse indexes.

use crate::{ObligationKey, SemanticChange, SemanticFieldClass};
use eqm_domain::{TargetId, UnitId};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// One changed repository path and its optional known target classification.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ChangedFile {
    /// Normalized repository-relative path.
    pub path: Box<str>,
    /// Target containing the path, when classification is complete.
    pub target: Option<TargetId>,
}

/// Explicit finalized-graph reverse indexes consumed by affected analysis.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AffectedIndexes {
    /// Every workspace unit.
    pub all_units: BTreeSet<UnitId>,
    /// Every derived obligation.
    pub all_obligations: BTreeSet<ObligationKey>,
    /// Direct reverse dependency and fragment-consumer edges.
    pub unit_dependents: BTreeMap<UnitId, BTreeSet<UnitId>>,
    /// Obligations derived from each unit.
    pub unit_obligations: BTreeMap<UnitId, BTreeSet<ObligationKey>>,
    /// Units bound to each target.
    pub target_units: BTreeMap<TargetId, BTreeSet<UnitId>>,
    /// Artifact paths mapped to their bound units.
    pub artifact_units: BTreeMap<Box<str>, BTreeSet<UnitId>>,
    /// Exact semantic coordinates with additional affected units.
    pub semantic_units: BTreeMap<crate::SemanticCoordinate, BTreeSet<UnitId>>,
    /// Exact semantic coordinates with directly affected obligations.
    pub semantic_obligations: BTreeMap<crate::SemanticCoordinate, BTreeSet<ObligationKey>>,
}

/// Deterministic conservative affected result.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AffectedSet {
    /// Every potentially affected unit.
    pub units: BTreeSet<UnitId>,
    /// Every potentially affected obligation.
    pub obligations: BTreeSet<ObligationKey>,
    /// Whether a conservative fallback expanded the result.
    pub conservative: bool,
}

/// Computes affected units and obligations from changed files and semantic changes.
#[must_use]
pub fn analyze_affected_set(
    indexes: &AffectedIndexes,
    changed_files: &BTreeSet<ChangedFile>,
    semantic_changes: &[SemanticChange],
) -> AffectedSet {
    let mut result = AffectedSet::default();

    for changed in changed_files {
        if let Some(units) = indexes.artifact_units.get(changed.path.as_ref()) {
            result.units.extend(units.iter().cloned());
        } else if let Some(target) = &changed.target {
            result.conservative = true;
            if let Some(units) = indexes.target_units.get(target) {
                result.units.extend(units.iter().cloned());
            }
        } else {
            result.conservative = true;
            result.units.extend(indexes.all_units.iter().cloned());
            result
                .obligations
                .extend(indexes.all_obligations.iter().cloned());
        }
    }

    for change in semantic_changes {
        if change.coordinate.class == SemanticFieldClass::Metadata {
            continue;
        }
        let mut classified = false;
        if let Some(units) = indexes.semantic_units.get(&change.coordinate) {
            classified = true;
            result.units.extend(units.iter().cloned());
        }
        if let Some(obligations) = indexes.semantic_obligations.get(&change.coordinate) {
            classified = true;
            result.obligations.extend(obligations.iter().cloned());
        }
        if let Some(unit) = &change.coordinate.unit {
            classified = true;
            result.units.insert(unit.clone());
        }
        if let Some(target) = &change.coordinate.target {
            classified = true;
            if let Some(units) = indexes.target_units.get(target) {
                result.units.extend(units.iter().cloned());
            }
        }
        if !classified {
            result.conservative = true;
            result.units.extend(indexes.all_units.iter().cloned());
            result
                .obligations
                .extend(indexes.all_obligations.iter().cloned());
        }
    }

    expand_dependents(indexes, &mut result.units);
    for unit in &result.units {
        if let Some(obligations) = indexes.unit_obligations.get(unit) {
            result.obligations.extend(obligations.iter().cloned());
        }
    }
    result
}

fn expand_dependents(indexes: &AffectedIndexes, units: &mut BTreeSet<UnitId>) {
    let mut pending: VecDeque<_> = units.iter().cloned().collect();
    while let Some(unit) = pending.pop_front() {
        if let Some(dependents) = indexes.unit_dependents.get(&unit) {
            for dependent in dependents {
                if units.insert(dependent.clone()) {
                    pending.push_back(dependent.clone());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ScopeSubject, SemanticChangeKind, SemanticCoordinate, SemanticValue};
    use eqm_domain::{Facet, FullRequirementId, PolicyId, Revision};
    use std::error::Error;

    fn unit(value: &str) -> Result<UnitId, Box<dyn Error>> {
        Ok(UnitId::new(value)?)
    }

    fn obligation(owner: &UnitId) -> Result<ObligationKey, Box<dyn Error>> {
        Ok(ObligationKey {
            policy: PolicyId::new("quality.default")?,
            policy_revision: Revision::new(1)?,
            profiles: BTreeMap::new(),
            unit: owner.clone(),
            requirement: FullRequirementId::new(format!("{owner}#works"))?,
            subject: ScopeSubject::Target(TargetId::new("web")?),
            facet: Facet::Behavior,
            release_context: None,
        })
    }

    fn change(
        unit: Option<UnitId>,
        target: Option<TargetId>,
        class: SemanticFieldClass,
    ) -> SemanticChange {
        SemanticChange {
            coordinate: SemanticCoordinate {
                unit,
                requirement: None,
                target,
                facet: None,
                class,
                field: "value".into(),
            },
            kind: SemanticChangeKind::Added,
            before: None,
            after: Some(SemanticValue::Opaque("changed".into())),
        }
    }

    fn indexes() -> Result<(AffectedIndexes, UnitId, UnitId, UnitId), Box<dyn Error>> {
        let core = unit("account.create.flow.core")?;
        let form = unit("account.create.flow.form")?;
        let admin = unit("admin.manage.flow.panel")?;
        let core_obligation = obligation(&core)?;
        let form_obligation = obligation(&form)?;
        let admin_obligation = obligation(&admin)?;
        Ok((
            AffectedIndexes {
                all_units: BTreeSet::from([core.clone(), form.clone(), admin.clone()]),
                all_obligations: BTreeSet::from([
                    core_obligation.clone(),
                    form_obligation.clone(),
                    admin_obligation.clone(),
                ]),
                unit_dependents: BTreeMap::from([(core.clone(), BTreeSet::from([form.clone()]))]),
                unit_obligations: BTreeMap::from([
                    (core.clone(), BTreeSet::from([core_obligation])),
                    (form.clone(), BTreeSet::from([form_obligation])),
                    (admin.clone(), BTreeSet::from([admin_obligation])),
                ]),
                target_units: BTreeMap::from([(
                    TargetId::new("web")?,
                    BTreeSet::from([core.clone(), form.clone()]),
                )]),
                artifact_units: BTreeMap::from([(
                    Box::from("src/core.rs"),
                    BTreeSet::from([core.clone()]),
                )]),
                semantic_units: BTreeMap::new(),
                semantic_obligations: BTreeMap::new(),
            },
            core,
            form,
            admin,
        ))
    }

    #[test]
    fn mapped_files_are_precise_and_expand_transitive_dependents() -> Result<(), Box<dyn Error>> {
        let (indexes, core, form, _) = indexes()?;
        let result = analyze_affected_set(
            &indexes,
            &BTreeSet::from([ChangedFile {
                path: "src/core.rs".into(),
                target: Some(TargetId::new("web")?),
            }]),
            &[],
        );
        assert_eq!(result.units, BTreeSet::from([core, form]));
        assert_eq!(result.obligations.len(), 2);
        assert!(!result.conservative);
        Ok(())
    }

    #[test]
    fn unmapped_target_and_repository_files_fail_conservatively_open() -> Result<(), Box<dyn Error>>
    {
        let (indexes, core, form, admin) = indexes()?;
        let target = analyze_affected_set(
            &indexes,
            &BTreeSet::from([ChangedFile {
                path: "src/new.rs".into(),
                target: Some(TargetId::new("web")?),
            }]),
            &[],
        );
        assert_eq!(target.units, BTreeSet::from([core.clone(), form]));
        assert!(target.conservative);
        let global = analyze_affected_set(
            &indexes,
            &BTreeSet::from([ChangedFile {
                path: "workspace.toml".into(),
                target: None,
            }]),
            &[],
        );
        assert_eq!(
            global.units,
            BTreeSet::from([core, unit("account.create.flow.form")?, admin])
        );
        assert_eq!(global.obligations, indexes.all_obligations);
        assert!(global.conservative);
        Ok(())
    }

    #[test]
    fn semantic_changes_never_omit_coordinate_or_fallback_obligations() -> Result<(), Box<dyn Error>>
    {
        let (indexes, core, form, admin) = indexes()?;
        let direct = analyze_affected_set(
            &indexes,
            &BTreeSet::new(),
            &[change(
                Some(core.clone()),
                None,
                SemanticFieldClass::Requirement,
            )],
        );
        assert_eq!(direct.units, BTreeSet::from([core, form]));
        assert_eq!(direct.obligations.len(), 2);

        let authority = analyze_affected_set(
            &indexes,
            &BTreeSet::new(),
            &[change(None, None, SemanticFieldClass::Evidence)],
        );
        assert_eq!(authority.units, indexes.all_units);
        assert_eq!(authority.obligations, indexes.all_obligations);
        assert!(authority.conservative);

        let metadata = analyze_affected_set(
            &indexes,
            &BTreeSet::new(),
            &[change(None, None, SemanticFieldClass::Metadata)],
        );
        assert_eq!(metadata, AffectedSet::default());
        assert!(indexes.all_units.contains(&admin));
        Ok(())
    }
}
