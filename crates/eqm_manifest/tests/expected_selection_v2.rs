//! Independent literal bytes/domain hash and selection representation mutations.
use eqm_domain::*;
use eqm_manifest::canonicalize_expected_selection_v2;
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
fn manifest(
    source: ExpectedSelectionSource,
    cases: &[&str],
) -> Result<ExpectedSelectionManifest, Box<dyn Error>> {
    Ok(ExpectedSelectionManifest::new(
        source,
        EvidenceKind::Test,
        SelectionCountUnit::LogicalTestCase,
        cases
            .iter()
            .map(|id| LogicalCaseId::new(*id))
            .collect::<Result<Vec<_>, _>>()?,
        vec![],
        false,
    )?)
}
const LITERAL:&[u8]=br#"{"alternatives":[],"count_unit":"logical_test_case","independent_review_principals":false,"kind":"test","required":["case.a","case.b"],"source":{"binding":"binding.example","binding_digest":"sha256:89e0c6b395973dd3e07a13b4fe7989c4f94bbd460692a448eb4a633699d0b4c0","binding_revision":1,"campaign":"campaign.selected","plan":"plan.expected","specification":"cases","specification_digest":"sha256:36b22a93c84879e92d731c3f771b16ae700525c246d2496c6f29d8e51a3d4846"}}"#;
#[test]
fn literal_bytes_and_independently_computed_domain_digest_match() -> Result<(), Box<dyn Error>> {
    let projected =
        canonicalize_expected_selection_v2(&manifest(source()?, &["case.b", "case.a"])?)?;
    assert_eq!(projected.bytes(), LITERAL);
    assert_eq!(
        projected.digest().to_string(),
        "sha256:ad3e707047ea2fc5f5afe203261a575c04c65d551b139f9c7442dbbdadc2ffdd"
    );
    assert_ne!(
        projected.digest().as_digest(),
        PlanDigest::hash_content(LITERAL).as_digest()
    );
    assert_ne!(
        projected.digest().as_digest(),
        Sha256Digest::hash_content(LITERAL)
    );
    Ok(())
}
#[test]
fn order_is_canonical_and_every_source_or_selected_member_mutation_changes_identity()
-> Result<(), Box<dyn Error>> {
    let original =
        canonicalize_expected_selection_v2(&manifest(source()?, &["case.a", "case.b"])?)?;
    assert_eq!(
        original,
        canonicalize_expected_selection_v2(&manifest(source()?, &["case.b", "case.a"])?)?
    );
    for axis in 0..7 {
        let mut changed = source()?;
        match axis {
            0 => changed.binding = BindingId::new("binding.other")?,
            1 => changed.binding_revision = Revision::new(2)?,
            2 => changed.binding_digest = BindingDigest::hash_content(b"changed"),
            3 => changed.specification = EvidenceSpecId::new("other")?,
            4 => {
                changed.specification_digest = EvidenceSpecificationDigest::hash_content(b"changed")
            }
            5 => changed.plan = PlanId::new("plan.other")?,
            _ => changed.campaign = CampaignId::new("campaign.other")?,
        }
        assert_ne!(
            original.digest(),
            canonicalize_expected_selection_v2(&manifest(changed, &["case.a", "case.b"])?)?
                .digest(),
            "axis{axis}"
        );
    }
    assert_ne!(
        original.digest(),
        canonicalize_expected_selection_v2(&manifest(source()?, &["case.a"])?)?.digest()
    );
    let mut boundary = source()?;
    boundary.binding_revision = Revision::new(u64::MAX)?;
    let projected = canonicalize_expected_selection_v2(&manifest(boundary, &["case.a"])?)?;
    assert!(
        std::str::from_utf8(projected.bytes())?
            .contains("\"binding_revision\":\"18446744073709551615\"")
    );
    Ok(())
}
#[test]
fn unselected_alternative_definitions_and_frozen_choice_remain_in_identity()
-> Result<(), Box<dyn Error>> {
    let build =
        |selected: &str, unselected: &str| -> Result<ExpectedSelectionManifest, Box<dyn Error>> {
            let group = FrozenSelectionAlternative::new(
                LogicalCaseId::new("group.one")?,
                vec![
                    (
                        LogicalCaseId::new("option.a")?,
                        vec![LogicalCaseId::new("case.a")?],
                    ),
                    (
                        LogicalCaseId::new("option.b")?,
                        vec![LogicalCaseId::new(unselected)?],
                    ),
                ],
                LogicalCaseId::new(selected)?,
            )?;
            Ok(ExpectedSelectionManifest::new(
                source()?,
                EvidenceKind::Test,
                SelectionCountUnit::LogicalTestCase,
                vec![],
                vec![group],
                false,
            )?)
        };
    let first = canonicalize_expected_selection_v2(&build("option.a", "case.b")?)?;
    let changed = canonicalize_expected_selection_v2(&build("option.a", "case.c")?)?;
    assert_ne!(first.digest(), changed.digest());
    assert_ne!(
        first.digest(),
        canonicalize_expected_selection_v2(&build("option.b", "case.b")?)?.digest()
    );
    let review = |independent| -> Result<ExpectedSelectionManifest, Box<dyn Error>> {
        Ok(ExpectedSelectionManifest::new(
            source()?,
            EvidenceKind::ManualReview,
            SelectionCountUnit::ReviewSlot,
            vec![LogicalCaseId::new("review.one")?],
            vec![],
            independent,
        )?)
    };
    assert_ne!(
        canonicalize_expected_selection_v2(&review(false)?)?.digest(),
        canonicalize_expected_selection_v2(&review(true)?)?.digest()
    );
    Ok(())
}
