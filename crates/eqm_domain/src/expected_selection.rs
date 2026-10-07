//! Explicit pre-result expected selections. Construction does not grant authority.

use crate::{
    BindingDigest, BindingId, CampaignId, EvidenceKind, EvidenceSpecId,
    EvidenceSpecificationDigest, LogicalCaseId, PlanId, Revision,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Closed logical credit unit, never a file, transport copy or attempt.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SelectionCountUnit {
    /// One expected test case.
    LogicalTestCase,
    /// One independently checked structural fact.
    StructuralFact,
    /// One independently checked inventory fact.
    InventoryFact,
    /// One independently checked snapshot.
    Snapshot,
    /// One independently checked runtime fact.
    RuntimeFact,
    /// One explicit review slot; principal independence is a separate obligation.
    ReviewSlot,
    /// One independently verified referenced record, at most one per selection.
    VerifiedRecord,
}

impl SelectionCountUnit {
    /// Returns the one compatible evidence kind.
    #[must_use]
    pub const fn kind(self) -> EvidenceKind {
        match self {
            Self::LogicalTestCase => EvidenceKind::Test,
            Self::StructuralFact => EvidenceKind::StructuralCheck,
            Self::InventoryFact => EvidenceKind::StaticInventory,
            Self::Snapshot => EvidenceKind::Snapshot,
            Self::RuntimeFact => EvidenceKind::RuntimeSnapshot,
            Self::ReviewSlot => EvidenceKind::ManualReview,
            Self::VerifiedRecord => EvidenceKind::ReleaseRecord,
        }
    }
    /// Stable semantic wire label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LogicalTestCase => "logical_test_case",
            Self::StructuralFact => "structural_fact",
            Self::InventoryFact => "inventory_fact",
            Self::Snapshot => "snapshot",
            Self::RuntimeFact => "runtime_fact",
            Self::ReviewSlot => "review_slot",
            Self::VerifiedRecord => "verified_record",
        }
    }
}

/// Exact local specification identity under its binding, plus frozen campaign.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ExpectedSelectionSource {
    /// Owning binding ID.
    pub binding: BindingId,
    /// Exact binding revision.
    pub binding_revision: Revision,
    /// Exact binding semantics.
    pub binding_digest: BindingDigest,
    /// Specification ID is local to this binding.
    pub specification: EvidenceSpecId,
    /// Exact specification semantics.
    pub specification_digest: EvidenceSpecificationDigest,
    /// Pre-result authorized plan.
    pub plan: PlanId,
    /// Explicitly selected campaign, never latest by timestamp.
    pub campaign: CampaignId,
}

/// Explicit options and the option frozen before any observation is read.
/// This is a claim; only the application authority boundary admits it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenSelectionAlternative {
    id: LogicalCaseId,
    options: BTreeMap<LogicalCaseId, BTreeSet<LogicalCaseId>>,
    selected: LogicalCaseId,
}
impl FrozenSelectionAlternative {
    /// Rejects duplicate/empty options, duplicate members and undeclared choices.
    pub fn new(
        id: LogicalCaseId,
        options: Vec<(LogicalCaseId, Vec<LogicalCaseId>)>,
        selected: LogicalCaseId,
    ) -> Result<Self, ExpectedSelectionError> {
        let mut collected = BTreeMap::new();
        for (key, cases) in options {
            let cases = case_set(cases)?;
            if collected.insert(key, cases).is_some() {
                return Err(ExpectedSelectionError::Duplicate);
            }
        }
        if collected.len() < 2 || !collected.contains_key(&selected) {
            return Err(ExpectedSelectionError::InvalidAlternative);
        }
        if collected.values().map(BTreeSet::len).sum::<usize>() > 100_000 {
            return Err(ExpectedSelectionError::Limit);
        }
        Ok(Self {
            id,
            options: collected,
            selected,
        })
    }
    /// Group identity.
    #[must_use]
    pub const fn id(&self) -> &LogicalCaseId {
        &self.id
    }
    /// All options, retaining unselected definitions for semantic identity.
    #[must_use]
    pub const fn options(&self) -> &BTreeMap<LogicalCaseId, BTreeSet<LogicalCaseId>> {
        &self.options
    }
    /// Exact preselected option.
    #[must_use]
    pub const fn selected(&self) -> &LogicalCaseId {
        &self.selected
    }
}

/// Immutable declared expectation; no Default or unchecked deserialization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpectedSelectionManifest {
    source: ExpectedSelectionSource,
    kind: EvidenceKind,
    unit: SelectionCountUnit,
    required: BTreeSet<LogicalCaseId>,
    alternatives: BTreeMap<LogicalCaseId, FrozenSelectionAlternative>,
    selected: BTreeSet<LogicalCaseId>,
    independent_review_principals: bool,
}
impl ExpectedSelectionManifest {
    /// Freezes required AND members and explicit chosen alternatives.
    /// Cases cannot be counted twice across groups. Empty expectation is invalid.
    pub fn new(
        source: ExpectedSelectionSource,
        kind: EvidenceKind,
        unit: SelectionCountUnit,
        required: Vec<LogicalCaseId>,
        alternatives: Vec<FrozenSelectionAlternative>,
        independent_review_principals: bool,
    ) -> Result<Self, ExpectedSelectionError> {
        if unit.kind() != kind
            || (independent_review_principals && kind != EvidenceKind::ManualReview)
        {
            return Err(ExpectedSelectionError::CountUnit);
        }
        let required = unique_cases(required)?;
        let mut selected = required.clone();
        let mut groups = BTreeMap::new();
        for group in alternatives {
            for member in &group.options[&group.selected] {
                if !selected.insert(member.clone()) {
                    return Err(ExpectedSelectionError::Duplicate);
                }
            }
            if groups.insert(group.id.clone(), group).is_some() {
                return Err(ExpectedSelectionError::Duplicate);
            }
        }
        if required.len()
            + groups
                .values()
                .flat_map(|group| group.options.values())
                .map(BTreeSet::len)
                .sum::<usize>()
            > 100_000
        {
            return Err(ExpectedSelectionError::Limit);
        }
        if selected.is_empty() {
            return Err(ExpectedSelectionError::Empty);
        }
        if unit == SelectionCountUnit::VerifiedRecord && selected.len() != 1 {
            return Err(ExpectedSelectionError::CountUnit);
        }
        Ok(Self {
            source,
            kind,
            unit,
            required,
            alternatives: groups,
            selected,
            independent_review_principals,
        })
    }
    /// Exact local specification and campaign identity.
    #[must_use]
    pub const fn source(&self) -> &ExpectedSelectionSource {
        &self.source
    }
    /// Compatible evidence kind.
    #[must_use]
    pub const fn kind(&self) -> EvidenceKind {
        self.kind
    }
    /// Declared logical credit unit.
    #[must_use]
    pub const fn count_unit(&self) -> SelectionCountUnit {
        self.unit
    }
    /// Unconditional required AND members.
    #[must_use]
    pub const fn required(&self) -> &BTreeSet<LogicalCaseId> {
        &self.required
    }
    /// Explicit preselected alternatives.
    #[must_use]
    pub const fn alternatives(&self) -> &BTreeMap<LogicalCaseId, FrozenSelectionAlternative> {
        &self.alternatives
    }
    /// Full effective expected set, independent of results.
    #[must_use]
    pub const fn selected(&self) -> &BTreeSet<LogicalCaseId> {
        &self.selected
    }
    /// Whether review admission must prove authorized distinct principals.
    #[must_use]
    pub const fn independent_review_principals(&self) -> bool {
        self.independent_review_principals
    }
}
fn unique_cases(
    values: Vec<LogicalCaseId>,
) -> Result<BTreeSet<LogicalCaseId>, ExpectedSelectionError> {
    let count = values.len();
    let set: BTreeSet<_> = values.into_iter().collect();
    if set.len() != count {
        return Err(ExpectedSelectionError::Duplicate);
    }
    if set.len() > 100_000 {
        return Err(ExpectedSelectionError::Limit);
    }
    Ok(set)
}
fn case_set(values: Vec<LogicalCaseId>) -> Result<BTreeSet<LogicalCaseId>, ExpectedSelectionError> {
    let set = unique_cases(values)?;
    if set.is_empty() {
        return Err(ExpectedSelectionError::Empty);
    }
    Ok(set)
}
/// Invalid local expected-selection claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExpectedSelectionError {
    /// No expected logical member.
    Empty,
    /// Duplicate group/option/member.
    Duplicate,
    /// Kind/count-unit or independence mismatch.
    CountUnit,
    /// Alternative is undeclared or has fewer than two options.
    InvalidAlternative,
    /// Manifest logical member limit exceeded.
    Limit,
}
impl Display for ExpectedSelectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "invalid expected selection: {self:?}")
    }
}
impl Error for ExpectedSelectionError {}
