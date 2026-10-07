//! S053 actual preparation and evaluated-carrier guards over authored workspace.
use eqm_application::{
    AuthorityError, AuthorityProvider, AuthorityRequest, ExpectedSelectionPreparationError,
    ExpectedSelectionProvider, PolicyReference, PolicySelectionRequest, PreparedSession,
    ProfileSelection, ProtectedAuthority, SessionRequest, prepare, prepare_expected_selections,
    prepare_obligations_from_request, prepare_protected_obligations,
};
use eqm_domain::*;
use eqm_engine::{
    CompletePolicyResolution, ConformanceError, CoverageStatus, EvaluationMode, EvidenceOutcome,
    ExpectedSelectionMap, FacetEvaluationInput, FacetStatus, FreshnessStatus, ScopeSubject,
    SelectionObservation, SupportingCheck, TrustEvaluation, TruthValue, WaiverEvaluation,
    evaluate_required_facets, evaluate_required_facets_with_expected_selections,
};
use std::cell::Cell;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::Path;

fn copy_tree(source: &Path, target: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(target)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &target.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), target.join(entry.file_name()))?;
        }
    }
    Ok(())
}
fn fixture() -> Result<(tempfile::TempDir, PreparedSession), Box<dyn Error>> {
    fixture_with(|_| Ok(()))
}
fn fixture_with(
    mutate: impl FnOnce(&Path) -> Result<(), Box<dyn Error>>,
) -> Result<(tempfile::TempDir, PreparedSession), Box<dyn Error>> {
    let root = tempfile::tempdir()?;
    copy_tree(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/android-ios"),
        root.path(),
    )?;
    fs::create_dir(root.path().join(".git"))?;
    mutate(root.path())?;
    let session = prepare(
        &SessionRequest::new(None, "2026-01-02T03:04:05Z".parse()?),
        root.path(),
    )?;
    Ok((root, session))
}
fn request() -> Result<PolicySelectionRequest, Box<dyn Error>> {
    Ok(PolicySelectionRequest::development(
        Some(PolicyReference::new(
            PolicyId::new("consumer.critical_flow")?,
            Some(Revision::new(1)?),
        )),
        vec![ProfileSelection::new(
            ProfileId::new("audience.default")?,
            Some(Revision::new(1)?),
            vec![(
                DimensionId::new("region")?,
                Some(SymbolicValueId::new("us")?),
            )],
        )],
    ))
}
fn source(binding: &Binding) -> Result<ExpectedSelectionSource, Box<dyn Error>> {
    let spec = binding.evidence().values().next().ok_or("missing spec")?;
    Ok(ExpectedSelectionSource {
        binding: binding.id().clone(),
        binding_revision: binding.revision(),
        binding_digest: eqm_manifest::canonicalize_binding_v2(binding)?.digest(),
        specification: spec.id().clone(),
        specification_digest: eqm_manifest::canonicalize_evidence_spec_v2(spec)?.digest(),
        plan: PlanId::new("plan.expected")?,
        campaign: CampaignId::new("campaign.selected")?,
    })
}
fn map(
    prepared: &eqm_application::PreparedObligations<'_>,
    session: &PreparedSession,
) -> Result<ExpectedSelectionMap, Box<dyn Error>> {
    let mut map = BTreeMap::new();
    for key in prepared.universe().coordinates() {
        if key.requirement.as_str() != "account.create.signup.identifier#email_default"
            || key.facet != Facet::Behavior
        {
            continue;
        }
        let target = match &key.subject {
            ScopeSubject::Target(target) => target,
            _ => continue,
        };
        let binding = session
            .finalized()
            .graph()
            .bindings()
            .values()
            .find(|binding| binding.target() == target)
            .ok_or("binding absent")?;
        let manifest = ExpectedSelectionManifest::new(
            source(binding)?,
            EvidenceKind::Test,
            SelectionCountUnit::LogicalTestCase,
            vec![LogicalCaseId::new("case.a")?, LogicalCaseId::new("case.b")?],
            vec![],
            false,
        )?;
        map.insert(key.clone(), vec![manifest]);
    }
    assert_eq!(map.len(), 2);
    Ok(map)
}
fn facts() -> FacetEvaluationInput {
    // Synthetic admitted facts exercise selection guarding, not real crypto/trust.
    FacetEvaluationInput {
        applicability: TruthValue::True,
        context_valid: true,
        coverage: CoverageStatus::Covered,
        outcome: EvidenceOutcome::Satisfied,
        freshness: FreshnessStatus::Fresh,
        trust: TrustEvaluation::Sufficient,
        structure: SupportingCheck::NotRequired,
        exposure: SupportingCheck::NotRequired,
        release: SupportingCheck::NotRequired,
        waiver: WaiverEvaluation::NotApplied(BTreeMap::new()),
    }
}
fn observations(
    manifest: &ExpectedSelectionManifest,
    cases: &[&str],
) -> Result<Vec<SelectionObservation>, Box<dyn Error>> {
    cases
        .iter()
        .map(|case| {
            Ok(SelectionObservation {
                source: manifest.source().clone(),
                case: LogicalCaseId::new(*case)?,
            })
        })
        .collect()
}
#[test]
fn missing_expected_case_cannot_be_hidden_by_passing_copies_or_ordinary_evaluation()
-> Result<(), Box<dyn Error>> {
    let (_root, session) = fixture()?;
    let prepared = prepare_obligations_from_request(
        session.finalized(),
        EvaluationMode::Development,
        &request()?,
        None,
    )?;
    let claims = map(&prepared, &session)?;
    let checked =
        prepare_expected_selections(session.finalized(), &prepared, Some(claims.clone()), None)?;
    let (key, manifests) = claims.first_key_value().ok_or("empty claims")?;
    let inputs = BTreeMap::from([(key.clone(), facts())]);
    let full = BTreeMap::from([(
        key.clone(),
        observations(&manifests[0], &["case.a", "case.b"])?,
    )]);
    let positive = evaluate_required_facets_with_expected_selections(
        checked.universe(),
        inputs.clone(),
        &full,
    )?;
    assert_eq!(
        CompletePolicyResolution::new(checked.universe().clone(), positive)?.facets()[key],
        FacetStatus::Satisfied
    );
    let copies = BTreeMap::from([(
        key.clone(),
        observations(&manifests[0], &["case.a", "case.a", "case.a"])?,
    )]);
    let negative = evaluate_required_facets_with_expected_selections(
        checked.universe(),
        inputs.clone(),
        &copies,
    )?;
    assert_ne!(
        CompletePolicyResolution::new(checked.universe().clone(), negative)?.facets()[key],
        FacetStatus::Satisfied
    );
    let ordinary = evaluate_required_facets(checked.universe(), inputs)?;
    assert_ne!(
        CompletePolicyResolution::new(checked.universe().clone(), ordinary)?.facets()[key],
        FacetStatus::Satisfied
    );
    Ok(())
}
#[test]
fn exact_lineage_case_and_component_claims_are_not_interchangeable() -> Result<(), Box<dyn Error>> {
    let (_root, session) = fixture()?;
    let prepared = prepare_obligations_from_request(
        session.finalized(),
        EvaluationMode::Development,
        &request()?,
        None,
    )?;
    let claims = map(&prepared, &session)?;
    let checked =
        prepare_expected_selections(session.finalized(), &prepared, Some(claims.clone()), None)?;
    let (key, manifests) = claims.first_key_value().ok_or("empty")?;
    let positive = observations(&manifests[0], &["case.a", "case.b"])?;
    for axis in 0..8 {
        let mut changed = positive.clone();
        match axis {
            0 => changed[0].source.plan = PlanId::new("plan.foreign")?,
            1 => changed[0].source.campaign = CampaignId::new("campaign.foreign")?,
            2 => changed[0].source.binding = BindingId::new("binding.foreign")?,
            3 => changed[0].source.specification = EvidenceSpecId::new("foreign_spec")?,
            4 => changed[0].source.binding_digest = BindingDigest::hash_content(b"foreign"),
            5 => {
                changed[0].source.specification_digest =
                    EvidenceSpecificationDigest::hash_content(b"foreign")
            }
            6 => changed[0].source.binding_revision = Revision::new(2)?,
            _ => changed[0].case = LogicalCaseId::new("case.foreign")?,
        }
        let evaluated = evaluate_required_facets_with_expected_selections(
            checked.universe(),
            BTreeMap::from([(key.clone(), facts())]),
            &BTreeMap::from([(key.clone(), changed)]),
        )?;
        assert_eq!(
            CompletePolicyResolution::new(checked.universe().clone(), evaluated)?.facets()[key],
            FacetStatus::Unknown,
            "axis{axis}"
        );
    }
    for binding_axis in [true, false] {
        let mut changed = claims.clone();
        let manifest = changed.get_mut(key).ok_or("missing")?;
        let mut source = manifest[0].source().clone();
        if binding_axis {
            source.binding_digest = BindingDigest::hash_content(b"foreign");
        } else {
            source.specification_digest = EvidenceSpecificationDigest::hash_content(b"foreign");
        }
        manifest[0] = ExpectedSelectionManifest::new(
            source,
            EvidenceKind::Test,
            SelectionCountUnit::LogicalTestCase,
            vec![LogicalCaseId::new("case.a")?],
            vec![],
            false,
        )?;
        assert!(matches!(
            prepare_expected_selections(session.finalized(), &prepared, Some(changed), None),
            Err(ExpectedSelectionPreparationError::Component)
        ));
    }
    Ok(())
}
#[test]
fn changed_expected_set_invalidates_private_carrier_and_missing_map_is_unknown()
-> Result<(), Box<dyn Error>> {
    let (_root, session) = fixture()?;
    let prepared = prepare_obligations_from_request(
        session.finalized(),
        EvaluationMode::Development,
        &request()?,
        None,
    )?;
    let claims = map(&prepared, &session)?;
    let (key, manifests) = claims.first_key_value().ok_or("empty")?;
    let checked =
        prepare_expected_selections(session.finalized(), &prepared, Some(claims.clone()), None)?;
    let evaluated = evaluate_required_facets_with_expected_selections(
        checked.universe(),
        BTreeMap::from([(key.clone(), facts())]),
        &BTreeMap::from([(
            key.clone(),
            observations(&manifests[0], &["case.a", "case.b"])?,
        )]),
    )?;
    let mut changed = claims.clone();
    changed.get_mut(key).ok_or("absent")?[0] = ExpectedSelectionManifest::new(
        manifests[0].source().clone(),
        EvidenceKind::Test,
        SelectionCountUnit::LogicalTestCase,
        vec![LogicalCaseId::new("case.a")?],
        vec![],
        false,
    )?;
    let foreign = prepare_expected_selections(session.finalized(), &prepared, Some(changed), None)?;
    assert_ne!(checked.digests(), foreign.digests());
    assert_eq!(
        CompletePolicyResolution::new(foreign.universe().clone(), evaluated),
        Err(ConformanceError::ForeignEvaluationContext)
    );
    let missing = prepare_expected_selections(session.finalized(), &prepared, None, None)?;
    let evaluated =
        evaluate_required_facets(missing.universe(), BTreeMap::from([(key.clone(), facts())]))?;
    assert_eq!(
        CompletePolicyResolution::new(missing.universe().clone(), evaluated)?.facets()[key],
        FacetStatus::Unknown
    );
    Ok(())
}
struct Provider<'a> {
    graph: &'a eqm_engine::FinalizedWorkspaceGraph,
    scopes: RequirementScenarioBindings,
    claims: ExpectedSelectionMap,
    calls: Cell<usize>,
}
impl AuthorityProvider for Provider<'_> {
    fn load_authority(
        &self,
        _: &AuthorityRequest,
    ) -> Result<ProtectedAuthority<'_>, AuthorityError> {
        Ok(ProtectedAuthority::new(self.graph, &self.scopes))
    }
}
impl ExpectedSelectionProvider for Provider<'_> {
    fn load_expected_selections(
        &self,
        request: &AuthorityRequest,
    ) -> Result<ExpectedSelectionMap, AuthorityError> {
        assert_eq!(request.mode, EvaluationMode::Release);
        self.calls.set(self.calls.get() + 1);
        Ok(self.claims.clone())
    }
}
#[test]
fn protected_selection_loaded_once_omission_uses_authority_and_candidate_cannot_replace_it()
-> Result<(), Box<dyn Error>> {
    let (_root, session) = fixture()?;
    let development = prepare_obligations_from_request(
        session.finalized(),
        EvaluationMode::Development,
        &request()?,
        None,
    )?;
    let mut provider = Provider {
        graph: session.finalized(),
        scopes: RequirementScenarioBindings::new(vec![])?,
        claims: BTreeMap::new(),
        calls: Cell::new(0),
    };
    // Coordinates include protected mode only in private context, not labels.
    provider.claims = map(&development, &session)?;
    let release = prepare_protected_obligations(
        session.finalized(),
        EvaluationMode::Release,
        &request()?,
        Some(&provider),
        None,
    )?;
    let checked =
        prepare_expected_selections(session.finalized(), &release, None, Some(&provider))?;
    assert_eq!(provider.calls.get(), 1);
    assert_eq!(checked.digests().len(), 2);
    assert!(matches!(
        prepare_expected_selections(
            session.finalized(),
            &release,
            Some(BTreeMap::new()),
            Some(&provider)
        ),
        Err(ExpectedSelectionPreparationError::CandidateMismatch)
    ));
    assert_eq!(provider.calls.get(), 2);
    assert!(matches!(
        prepare_expected_selections(session.finalized(), &release, None, None),
        Err(ExpectedSelectionPreparationError::Authority(
            AuthorityError::MissingProvider
        ))
    ));
    Ok(())
}

#[test]
fn caller_tag_cannot_substitute_for_verified_protected_graph() -> Result<(), Box<dyn Error>> {
    let (_root, session) = fixture()?;
    let development = prepare_obligations_from_request(
        session.finalized(),
        EvaluationMode::Development,
        &request()?,
        None,
    )?;
    let provider = Provider {
        graph: session.finalized(),
        scopes: RequirementScenarioBindings::new(vec![])?,
        claims: map(&development, &session)?,
        calls: Cell::new(0),
    };
    let exact = eqm_engine::PolicyProfileRequest::new(
        eqm_engine::AuthorityOrigin::ProtectedBaseline,
        eqm_engine::PolicyRef::new(PolicyId::new("consumer.critical_flow")?, Revision::new(1)?),
        vec![eqm_engine::ProfileRequest::new(
            ProfileId::new("audience.default")?,
            Revision::new(1)?,
            vec![(
                DimensionId::new("region")?,
                Some(SymbolicValueId::new("us")?),
            )],
        )?],
    )?;
    let raw = eqm_application::prepare_obligations(
        session.finalized(),
        EvaluationMode::Release,
        Some(&exact),
        None,
        None,
    )?;
    assert!(matches!(
        prepare_expected_selections(session.finalized(), &raw, None, Some(&provider)),
        Err(ExpectedSelectionPreparationError::ProtectedGraph)
    ));
    assert_eq!(provider.calls.get(), 0);
    let protected = prepare_protected_obligations(
        session.finalized(),
        EvaluationMode::Release,
        &request()?,
        Some(&provider),
        None,
    )?;
    assert!(
        prepare_expected_selections(session.finalized(), &protected, None, Some(&provider)).is_ok()
    );
    assert_eq!(provider.calls.get(), 1);
    Ok(())
}
#[test]
fn prior_unknown_coverage_cannot_become_waivable_missing() -> Result<(), Box<dyn Error>> {
    let (_root, session) = fixture()?;
    let prepared = prepare_obligations_from_request(
        session.finalized(),
        EvaluationMode::Development,
        &request()?,
        None,
    )?;
    let claims = map(&prepared, &session)?;
    let checked =
        prepare_expected_selections(session.finalized(), &prepared, Some(claims.clone()), None)?;
    let (key, manifests) = claims.first_key_value().ok_or("empty")?;
    let mut input = facts();
    input.coverage = CoverageStatus::Unknown;
    input.waiver = WaiverEvaluation::Waived(WaiverId::new("waiver.synthetic")?);
    // Same guard with known coverage admits complete positive; prior Unknown stays Unknown.
    let positive = evaluate_required_facets_with_expected_selections(
        checked.universe(),
        BTreeMap::from([(key.clone(), facts())]),
        &BTreeMap::from([(
            key.clone(),
            observations(&manifests[0], &["case.a", "case.b"])?,
        )]),
    )?;
    assert_eq!(
        CompletePolicyResolution::new(checked.universe().clone(), positive)?.facets()[key],
        FacetStatus::Satisfied
    );
    let negative = evaluate_required_facets_with_expected_selections(
        checked.universe(),
        BTreeMap::from([(key.clone(), input)]),
        &BTreeMap::from([(key.clone(), observations(&manifests[0], &["case.a"])?)]),
    )?;
    assert_eq!(
        CompletePolicyResolution::new(checked.universe().clone(), negative)?.facets()[key],
        FacetStatus::Unknown
    );
    Ok(())
}
#[test]
fn explicit_alternative_cannot_be_chosen_from_observed_passes() -> Result<(), Box<dyn Error>> {
    let (_root, session) = fixture()?;
    let development = prepare_obligations_from_request(
        session.finalized(),
        EvaluationMode::Development,
        &request()?,
        None,
    )?;
    let mut claims = map(&development, &session)?;
    let key = claims.first_key_value().ok_or("empty")?.0.clone();
    let original = claims[&key][0].source().clone();
    let group = FrozenSelectionAlternative::new(
        LogicalCaseId::new("alternative.group")?,
        vec![
            (
                LogicalCaseId::new("option.frozen")?,
                vec![LogicalCaseId::new("case.a")?],
            ),
            (
                LogicalCaseId::new("option.other")?,
                vec![LogicalCaseId::new("case.b")?],
            ),
        ],
        LogicalCaseId::new("option.frozen")?,
    )?;
    claims.get_mut(&key).ok_or("missing")?[0] = ExpectedSelectionManifest::new(
        original,
        EvidenceKind::Test,
        SelectionCountUnit::LogicalTestCase,
        vec![],
        vec![group],
        false,
    )?;
    assert!(matches!(
        prepare_expected_selections(
            session.finalized(),
            &development,
            Some(claims.clone()),
            None
        ),
        Err(ExpectedSelectionPreparationError::UnprotectedAlternative)
    ));
    let provider = Provider {
        graph: session.finalized(),
        scopes: RequirementScenarioBindings::new(vec![])?,
        claims,
        calls: Cell::new(0),
    };
    let prepared = prepare_protected_obligations(
        session.finalized(),
        EvaluationMode::Release,
        &request()?,
        Some(&provider),
        None,
    )?;
    let checked =
        prepare_expected_selections(session.finalized(), &prepared, None, Some(&provider))?;
    let manifest = &provider.claims[&key][0];
    for (case, status) in [
        ("case.a", FacetStatus::Satisfied),
        ("case.b", FacetStatus::Unknown),
    ] {
        let evaluated = evaluate_required_facets_with_expected_selections(
            checked.universe(),
            BTreeMap::from([(key.clone(), facts())]),
            &BTreeMap::from([(key.clone(), observations(manifest, &[case])?)]),
        )?;
        assert_eq!(
            CompletePolicyResolution::new(checked.universe().clone(), evaluated)?.facets()[&key],
            status
        );
    }
    Ok(())
}
#[test]
fn duplicate_unit_across_specifications_does_not_increase_credit_and_mixed_units_reject()
-> Result<(), Box<dyn Error>> {
    let (_root, session) = fixture()?;
    let prepared = prepare_obligations_from_request(
        session.finalized(),
        EvaluationMode::Development,
        &request()?,
        None,
    )?;
    let claims = map(&prepared, &session)?;
    let manifest = &claims.first_key_value().ok_or("empty")?.1[0];
    let mut source = manifest.source().clone();
    source.specification = EvidenceSpecId::new("second_spec")?;
    let first = ExpectedSelectionManifest::new(
        manifest.source().clone(),
        EvidenceKind::Test,
        SelectionCountUnit::LogicalTestCase,
        vec![LogicalCaseId::new("case.a")?],
        vec![],
        false,
    )?;
    let copied = ExpectedSelectionManifest::new(
        source.clone(),
        EvidenceKind::Test,
        SelectionCountUnit::LogicalTestCase,
        vec![LogicalCaseId::new("case.a")?],
        vec![],
        false,
    )?;
    let distinct = ExpectedSelectionManifest::new(
        source.clone(),
        EvidenceKind::Test,
        SelectionCountUnit::LogicalTestCase,
        vec![LogicalCaseId::new("case.b")?],
        vec![],
        false,
    )?;
    let mut copies = observations(&first, &["case.a"])?;
    copies.extend(observations(&copied, &["case.a"])?);
    assert_eq!(
        eqm_engine::expected_selection_coverage(
            &[first.clone(), copied],
            &copies,
            PositiveCount::new(2)?
        ),
        CoverageStatus::Missing
    );
    let mut actual = observations(&first, &["case.a"])?;
    actual.extend(observations(&distinct, &["case.b"])?);
    assert_eq!(
        eqm_engine::expected_selection_coverage(
            &[first.clone(), distinct],
            &actual,
            PositiveCount::new(2)?
        ),
        CoverageStatus::Covered
    );
    let inventory = ExpectedSelectionManifest::new(
        source,
        EvidenceKind::StaticInventory,
        SelectionCountUnit::InventoryFact,
        vec![LogicalCaseId::new("case.b")?],
        vec![],
        false,
    )?;
    assert_eq!(
        eqm_engine::expected_selection_coverage(
            &[first, inventory],
            &actual,
            PositiveCount::new(2)?
        ),
        CoverageStatus::Unknown
    );
    Ok(())
}

#[test]
fn actual_binding_local_specifications_have_and_coverage_and_ambiguous_units_reject()
-> Result<(), Box<dyn Error>> {
    let (_root, session) = fixture_with(|root| {
        let path = root.join("eqm/bindings/android.auth_signup.toml");
        let mut text = fs::read_to_string(&path)?;
        text.push_str(
            r#"
[[evidence]]
id = "second_test"
kind = "test"
requirements = ["account.create.signup.identifier#email_default"]
facets = ["behavior"]
runner = "runner.android"
[evidence.selector]
kind = "test"
framework = "junit"
test_id = "secondExactCase"
[[evidence]]
id = "inventory_control"
kind = "static_inventory"
requirements = ["account.create.signup.identifier#email_default"]
facets = ["behavior"]
[evidence.selector]
kind = "inventory"
record_type = "feature"
key = "identifier"
"#,
        );
        fs::write(path, text)?;
        let policy = root.join("eqm/policies/consumer.critical_flow.toml");
        fs::write(
            &policy,
            fs::read_to_string(&policy)?.replace("minimum_count = 1", "minimum_count = 2"),
        )?;
        Ok(())
    })?;
    let prepared = prepare_obligations_from_request(
        session.finalized(),
        EvaluationMode::Development,
        &request()?,
        None,
    )?;
    let mut claims = map(&prepared, &session)?;
    let android = TargetId::new("android")?;
    let key = claims
        .keys()
        .find(|key| key.subject == ScopeSubject::Target(android.clone()))
        .ok_or("android absent")?
        .clone();
    let binding = session
        .finalized()
        .graph()
        .bindings()
        .get(&BindingId::new("binding.android.auth_signup")?)
        .ok_or("binding absent")?;
    let make =
        |id: &str, kind, unit, case: &str| -> Result<ExpectedSelectionManifest, Box<dyn Error>> {
            let spec = binding
                .evidence()
                .get(&EvidenceSpecId::new(id)?)
                .ok_or("spec absent")?;
            let mut source = source(binding)?;
            source.specification = spec.id().clone();
            source.specification_digest =
                eqm_manifest::canonicalize_evidence_spec_v2(spec)?.digest();
            Ok(ExpectedSelectionManifest::new(
                source,
                kind,
                unit,
                vec![LogicalCaseId::new(case)?],
                vec![],
                false,
            )?)
        };
    let first = make(
        "identifier_behavior",
        EvidenceKind::Test,
        SelectionCountUnit::LogicalTestCase,
        "case.a",
    )?;
    let second = make(
        "second_test",
        EvidenceKind::Test,
        SelectionCountUnit::LogicalTestCase,
        "case.b",
    )?;
    claims.insert(key.clone(), vec![first.clone(), second.clone()]);
    let checked =
        prepare_expected_selections(session.finalized(), &prepared, Some(claims.clone()), None)?;
    let mut full = observations(&first, &["case.a"])?;
    full.extend(observations(&second, &["case.b"])?);
    let evaluated = evaluate_required_facets_with_expected_selections(
        checked.universe(),
        BTreeMap::from([(key.clone(), facts())]),
        &BTreeMap::from([(key.clone(), full)]),
    )?;
    assert_eq!(
        CompletePolicyResolution::new(checked.universe().clone(), evaluated)?.facets()[&key],
        FacetStatus::Satisfied
    );
    let omitted = evaluate_required_facets_with_expected_selections(
        checked.universe(),
        BTreeMap::from([(key.clone(), facts())]),
        &BTreeMap::from([(key.clone(), observations(&first, &["case.a"])?)]),
    )?;
    assert_ne!(
        CompletePolicyResolution::new(checked.universe().clone(), omitted)?.facets()[&key],
        FacetStatus::Satisfied
    );
    let repeated = make(
        "second_test",
        EvidenceKind::Test,
        SelectionCountUnit::LogicalTestCase,
        "case.a",
    )?;
    let mut duplicate_claims = claims.clone();
    duplicate_claims.insert(key.clone(), vec![first.clone(), repeated.clone()]);
    let duplicates =
        prepare_expected_selections(session.finalized(), &prepared, Some(duplicate_claims), None)?;
    let mut copies = observations(&first, &["case.a", "case.a"])?;
    copies.extend(observations(&repeated, &["case.a"])?);
    let evaluated = evaluate_required_facets_with_expected_selections(
        duplicates.universe(),
        BTreeMap::from([(key.clone(), facts())]),
        &BTreeMap::from([(key.clone(), copies)]),
    )?;
    assert_eq!(
        CompletePolicyResolution::new(duplicates.universe().clone(), evaluated)?.facets()[&key],
        FacetStatus::Missing
    );
    let inventory = make(
        "inventory_control",
        EvidenceKind::StaticInventory,
        SelectionCountUnit::InventoryFact,
        "case.b",
    )?;
    claims.insert(key, vec![first, inventory]);
    assert!(matches!(
        prepare_expected_selections(session.finalized(), &prepared, Some(claims), None),
        Err(ExpectedSelectionPreparationError::Binding(
            eqm_engine::ExpectedSelectionBindingError::AmbiguousCountUnit
        ))
    ));
    Ok(())
}

#[test]
fn graph_scope_profile_and_exact_binding_revision_cannot_be_substituted()
-> Result<(), Box<dyn Error>> {
    let (_root, session) = fixture()?;
    let prepared = prepare_obligations_from_request(
        session.finalized(),
        EvaluationMode::Development,
        &request()?,
        None,
    )?;
    let claims = map(&prepared, &session)?;
    assert!(
        prepare_expected_selections(session.finalized(), &prepared, Some(claims.clone()), None)
            .is_ok()
    );
    let android = TargetId::new("android")?;
    let key = claims
        .keys()
        .find(|key| key.subject == ScopeSubject::Target(android.clone()))
        .ok_or("android absent")?
        .clone();
    let ios_binding = session
        .finalized()
        .graph()
        .bindings()
        .get(&BindingId::new("binding.ios.auth_signup")?)
        .ok_or("ios binding absent")?;
    let foreign = ExpectedSelectionManifest::new(
        source(ios_binding)?,
        EvidenceKind::Test,
        SelectionCountUnit::LogicalTestCase,
        vec![LogicalCaseId::new("case.a")?],
        vec![],
        false,
    )?;
    let mut swapped = claims.clone();
    swapped.insert(key.clone(), vec![foreign]);
    assert!(matches!(
        prepare_expected_selections(session.finalized(), &prepared, Some(swapped), None),
        Err(ExpectedSelectionPreparationError::Binding(
            eqm_engine::ExpectedSelectionBindingError::Specification
        ))
    ));
    let mut foreign_key = key.clone();
    foreign_key.profiles.clear();
    let mut changed = claims.clone();
    let members = changed.remove(&key).ok_or("missing")?;
    changed.insert(foreign_key, members);
    assert!(matches!(
        prepare_expected_selections(session.finalized(), &prepared, Some(changed), None),
        Err(ExpectedSelectionPreparationError::Binding(
            eqm_engine::ExpectedSelectionBindingError::Coordinate
        ))
    ));
    let mut source = claims[&key][0].source().clone();
    source.binding_revision = Revision::new(2)?;
    let mut changed = claims.clone();
    changed.insert(
        key,
        vec![ExpectedSelectionManifest::new(
            source,
            EvidenceKind::Test,
            SelectionCountUnit::LogicalTestCase,
            vec![LogicalCaseId::new("case.a")?],
            vec![],
            false,
        )?],
    );
    assert!(matches!(
        prepare_expected_selections(session.finalized(), &prepared, Some(changed), None),
        Err(ExpectedSelectionPreparationError::Binding(
            eqm_engine::ExpectedSelectionBindingError::Specification
        ))
    ));
    let (_other, foreign) = fixture_with(|root| {
        let policy = root.join("eqm/policies/consumer.critical_flow.toml");
        fs::write(
            &policy,
            fs::read_to_string(&policy)?.replace("minimum_count = 1", "minimum_count = 2"),
        )?;
        Ok(())
    })?;
    assert!(matches!(
        prepare_expected_selections(foreign.finalized(), &prepared, Some(claims), None),
        Err(ExpectedSelectionPreparationError::Graph)
    ));
    Ok(())
}
