//! Exact constructor boundaries, not trust or product policy values.
use eqm_domain::*;
use std::error::Error;
fn source() -> Result<ExpectedSelectionSource, Box<dyn Error>> {
    Ok(ExpectedSelectionSource {
        binding: BindingId::new("binding.example")?,
        binding_revision: Revision::new(1)?,
        binding_digest: BindingDigest::hash_content(b"binding"),
        specification: EvidenceSpecId::new("cases")?,
        specification_digest: EvidenceSpecificationDigest::hash_content(b"spec"),
        plan: PlanId::new("plan.expected")?,
        campaign: CampaignId::new("campaign.selected")?,
    })
}
#[test]
fn count_units_are_closed_per_kind_and_empty_or_duplicate_claims_reject()
-> Result<(), Box<dyn Error>> {
    let units = [
        SelectionCountUnit::LogicalTestCase,
        SelectionCountUnit::StructuralFact,
        SelectionCountUnit::InventoryFact,
        SelectionCountUnit::Snapshot,
        SelectionCountUnit::RuntimeFact,
        SelectionCountUnit::ReviewSlot,
        SelectionCountUnit::VerifiedRecord,
    ];
    for unit in units {
        let case = LogicalCaseId::new("case.one")?;
        let valid = ExpectedSelectionManifest::new(
            source()?,
            unit.kind(),
            unit,
            vec![case.clone()],
            vec![],
            false,
        )?;
        assert_eq!(valid.selected().len(), 1);
        for other in units {
            if unit == other {
                continue;
            }
            assert_eq!(
                ExpectedSelectionManifest::new(
                    source()?,
                    other.kind(),
                    unit,
                    vec![case.clone()],
                    vec![],
                    false
                ),
                Err(ExpectedSelectionError::CountUnit)
            );
        }
        assert_eq!(
            ExpectedSelectionManifest::new(source()?, unit.kind(), unit, vec![], vec![], false),
            Err(ExpectedSelectionError::Empty)
        );
        assert_eq!(
            ExpectedSelectionManifest::new(
                source()?,
                unit.kind(),
                unit,
                vec![case.clone(), case],
                vec![],
                false
            ),
            Err(ExpectedSelectionError::Duplicate)
        );
    }
    assert_eq!(
        ExpectedSelectionManifest::new(
            source()?,
            EvidenceKind::ReleaseRecord,
            SelectionCountUnit::VerifiedRecord,
            vec![
                LogicalCaseId::new("record.one")?,
                LogicalCaseId::new("record.two")?
            ],
            vec![],
            false
        ),
        Err(ExpectedSelectionError::CountUnit)
    );
    assert_eq!(
        ExpectedSelectionManifest::new(
            source()?,
            EvidenceKind::Test,
            SelectionCountUnit::LogicalTestCase,
            vec![LogicalCaseId::new("case.one")?],
            vec![],
            true
        ),
        Err(ExpectedSelectionError::CountUnit)
    );
    let independent = ExpectedSelectionManifest::new(
        source()?,
        EvidenceKind::ManualReview,
        SelectionCountUnit::ReviewSlot,
        vec![LogicalCaseId::new("review.one")?],
        vec![],
        true,
    )?;
    assert!(independent.independent_review_principals());
    Ok(())
}
#[test]
fn alternatives_are_explicit_preselected_and_retain_all_definitions() -> Result<(), Box<dyn Error>>
{
    let id = LogicalCaseId::new("alternative.group")?;
    let option = LogicalCaseId::new("option.first")?;
    let other = LogicalCaseId::new("option.second")?;
    let a = LogicalCaseId::new("case.a")?;
    let b = LogicalCaseId::new("case.b")?;
    let choices = vec![
        (option.clone(), vec![a.clone()]),
        (other.clone(), vec![b.clone()]),
    ];
    let group = FrozenSelectionAlternative::new(id.clone(), choices.clone(), option.clone())?;
    let frozen = ExpectedSelectionManifest::new(
        source()?,
        EvidenceKind::Test,
        SelectionCountUnit::LogicalTestCase,
        vec![],
        vec![group.clone()],
        false,
    )?;
    assert!(frozen.selected().contains(&a));
    assert!(!frozen.selected().contains(&b));
    assert_eq!(frozen.alternatives()[&id].options().len(), 2);
    assert_eq!(
        FrozenSelectionAlternative::new(id.clone(), choices, LogicalCaseId::new("option.unknown")?),
        Err(ExpectedSelectionError::InvalidAlternative)
    );
    assert_eq!(
        FrozenSelectionAlternative::new(
            id.clone(),
            vec![(option.clone(), vec![a.clone()]), (option.clone(), vec![b])],
            option
        ),
        Err(ExpectedSelectionError::Duplicate)
    );
    assert_eq!(
        ExpectedSelectionManifest::new(
            source()?,
            EvidenceKind::Test,
            SelectionCountUnit::LogicalTestCase,
            vec![a],
            vec![group],
            false
        ),
        Err(ExpectedSelectionError::Duplicate)
    );
    Ok(())
}
