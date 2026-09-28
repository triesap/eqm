//! S019-B: complete v2 contract projection and `ContractDigest` identity.
//!
//! Literal byte/digest oracles are reviewed constants computed independently from
//! the projection code. The controls prove entity/extension isolation, canonical
//! set ordering, ordered journey-surface and applicability-child sequences, and
//! graph-semantic extension inclusion with display-only exclusion.

use eqm_domain::{
    Applicability, Capability, CapabilityId, ContractDigest, Description, DimensionId, Extensions,
    Facet, FinalizedWorkspaceGraph, Fragment, FragmentId, FragmentUse, Journey, JourneyId,
    LifecycleStatus, LocalRequirementId, MembershipOperator, OwnerRef, Requirement,
    RequirementLevel, RequirementScope, RequirementStatement, Revision, RiskClass, Sha256Digest,
    Surface, SurfaceId, SymbolicValueId, Title, Transition, TransitionTrigger, WorkspaceGraph,
    WorkspaceGraphInput,
};
use eqm_domain::{ExtensionNamespace, ExtensionValue};
use eqm_manifest::{canonicalize_contract_v2, project_contract_v2};
use std::collections::BTreeMap;
use std::error::Error;

const CONTRACT_BYTES: &str = r#"{"capabilities":[{"description":"Create an account","extensions":{},"id":"account.create","owners":["owner://team/accounts"],"status":"active","title":"Account creation"}],"extensions":{},"fragments":[{"extensions":{},"id":"auth.otp","owners":["owner://team/auth"],"requirements":[{"applicability":{"dimension":"region","operator":"in","values":["eu","us"]},"extensions":{},"facets":["behavior"],"id":"entry","level":"required","scope":"each_target","statement":"The OTP entry exists"}],"revision":1,"risk_class":"low","title":"OTP fragment"}],"journeys":[{"capability":"account.create","extensions":{},"id":"account.create.signup","owners":["owner://team/accounts"],"revision":1,"risk_class":"high","status":"active","surfaces":["account.create.signup.identifier","account.create.signup.otp"],"title":"Signup","transitions":[{"from":"account.create.signup.identifier","to":"account.create.signup.otp","trigger":"submit"}]}],"surfaces":[{"description":"Identifier surface","extensions":{},"fragment_origins":[{"digest":"sha256:ff23fd6ec113d29129b905fb1b481806b4dbe9b5cb3e06e69ae1f44968ba5210","fragment":"auth.otp","revision":1}],"id":"account.create.signup.identifier","journey":"account.create.signup","owners":["owner://team/accounts"],"requirements":[{"applicability":{"dimension":"region","operator":"in","values":["eu","us"]},"extensions":{},"facets":["behavior"],"id":"visible","level":"required","scope":"each_target","statement":"The identifier is visible"}],"revision":1,"status":"active","title":"Identifier"},{"extensions":{},"fragment_origins":[],"id":"account.create.signup.otp","journey":"account.create.signup","owners":["owner://team/accounts"],"requirements":[{"applicability":{"dimension":"region","operator":"in","values":["eu","us"]},"extensions":{},"facets":["behavior"],"id":"reachable","level":"required","scope":"each_target","statement":"The OTP is reachable"}],"revision":1,"status":"active","title":"OTP"}]}"#;
const CONTRACT_DIGEST: &str =
    "sha256:d36523b4fc01f696d1066d81dfece83166a04f0d07dbd4e7700c0c39e2fd6f14";

fn owner(value: &str) -> Result<OwnerRef, Box<dyn Error>> {
    Ok(value.parse()?)
}

fn title(value: &str) -> Result<Title, Box<dyn Error>> {
    Ok(Title::new(value)?)
}

fn requirement(id: &str, statement: &str) -> Result<Requirement, Box<dyn Error>> {
    Ok(Requirement::new(
        LocalRequirementId::new(id)?,
        RequirementLevel::Required,
        RequirementScope::EachTarget,
        RequirementStatement::new(statement)?,
        vec![Facet::Behavior],
        Applicability::membership(
            eqm_domain::DimensionId::new("region")?,
            MembershipOperator::In,
            vec![SymbolicValueId::new("us")?, SymbolicValueId::new("eu")?],
        )?,
        None,
        None,
        Extensions::default(),
    )?)
}

fn fragment_use() -> Result<FragmentUse, Box<dyn Error>> {
    Ok(FragmentUse::new(
        FragmentId::new("auth.otp")?,
        Revision::new(1)?,
        Sha256Digest::hash_content(b"fragment-one"),
        None,
    ))
}

fn input() -> Result<WorkspaceGraphInput, Box<dyn Error>> {
    let capability = Capability::new(
        CapabilityId::new("account.create")?,
        title("Account creation")?,
        LifecycleStatus::Active,
        vec![owner("owner://team/accounts")?],
        Some(Description::new("Create an account")?),
        Extensions::default(),
    )?;
    let journey = Journey::new(
        JourneyId::new("account.create.signup")?,
        Revision::new(1)?,
        title("Signup")?,
        CapabilityId::new("account.create")?,
        LifecycleStatus::Active,
        RiskClass::High,
        vec![owner("owner://team/accounts")?],
        vec![
            SurfaceId::new("account.create.signup.identifier")?,
            SurfaceId::new("account.create.signup.otp")?,
        ],
        vec![Transition::new(
            SurfaceId::new("account.create.signup.identifier")?,
            SurfaceId::new("account.create.signup.otp")?,
            TransitionTrigger::new("submit")?,
        )],
        None,
        Extensions::default(),
    )?;
    let first = Surface::new(
        SurfaceId::new("account.create.signup.identifier")?,
        Revision::new(1)?,
        title("Identifier")?,
        JourneyId::new("account.create.signup")?,
        LifecycleStatus::Active,
        vec![owner("owner://team/accounts")?],
        vec![requirement("visible", "The identifier is visible")?],
        vec![fragment_use()?],
        Some(Description::new("Identifier surface")?),
        Extensions::default(),
    )?;
    let second = Surface::new(
        SurfaceId::new("account.create.signup.otp")?,
        Revision::new(1)?,
        title("OTP")?,
        JourneyId::new("account.create.signup")?,
        LifecycleStatus::Active,
        vec![owner("owner://team/accounts")?],
        vec![requirement("reachable", "The OTP is reachable")?],
        Vec::new(),
        None,
        Extensions::default(),
    )?;
    let fragment = Fragment::new(
        FragmentId::new("auth.otp")?,
        Revision::new(1)?,
        title("OTP fragment")?,
        RiskClass::Low,
        vec![owner("owner://team/auth")?],
        vec![requirement("entry", "The OTP entry exists")?],
        None,
        Extensions::default(),
    )?;
    Ok(WorkspaceGraphInput {
        capabilities: vec![capability],
        journeys: vec![journey],
        surfaces: vec![first, second],
        fragments: vec![fragment],
        ..WorkspaceGraphInput::default()
    })
}

fn finalize(input: WorkspaceGraphInput) -> Result<FinalizedWorkspaceGraph, Box<dyn Error>> {
    Ok(FinalizedWorkspaceGraph::from_engine(WorkspaceGraph::new(
        input,
    )?))
}

fn workspace() -> Result<FinalizedWorkspaceGraph, Box<dyn Error>> {
    finalize(input()?)
}

#[test]
fn literal_contract_bytes_and_digest_match_independent_oracle() -> Result<(), Box<dyn Error>> {
    let canonical = canonicalize_contract_v2(&workspace()?)?;
    eprintln!("CONTRACT_BYTES={}", std::str::from_utf8(canonical.bytes())?);
    eprintln!("CONTRACT_DIGEST={}", canonical.digest());
    assert_eq!(std::str::from_utf8(canonical.bytes())?, CONTRACT_BYTES);
    assert_eq!(canonical.digest().to_string(), CONTRACT_DIGEST);
    assert!(!canonical.bytes().ends_with(b"\n"));
    Ok(())
}

#[test]
fn contract_identity_is_the_typed_domain_wrapper() -> Result<(), Box<dyn Error>> {
    let canonical = canonicalize_contract_v2(&workspace()?)?;
    let typed: ContractDigest = canonical.digest();
    assert_eq!(typed, ContractDigest::hash_content(canonical.bytes()));
    Ok(())
}

#[test]
fn projection_root_has_exactly_the_contract_components() -> Result<(), Box<dyn Error>> {
    let projected = project_contract_v2(&workspace()?)?;
    let object = projected.as_object().ok_or("contract root object")?;
    let keys: Vec<&str> = object.keys().map(String::as_str).collect();
    assert_eq!(
        keys,
        vec![
            "capabilities",
            "extensions",
            "fragments",
            "journeys",
            "surfaces"
        ]
    );
    for excluded in [
        "bindings", "targets", "policies", "profiles", "runners", "waivers",
    ] {
        assert!(!object.contains_key(excluded));
    }
    Ok(())
}

fn extra_extension(value: ExtensionValue) -> Result<Extensions, Box<dyn Error>> {
    Ok(Extensions::new(BTreeMap::from([(
        ExtensionNamespace::new("dev.equivalencematrix.extra")?,
        value,
    )]))?)
}

fn display_extension() -> Result<Extensions, Box<dyn Error>> {
    Ok(Extensions::new(BTreeMap::from([(
        ExtensionNamespace::new("dev.equivalencematrix.display")?,
        ExtensionValue::String("hidden".into()),
    )]))?)
}

fn journey_with(surfaces: Vec<SurfaceId>) -> Result<Journey, Box<dyn Error>> {
    Ok(Journey::new(
        JourneyId::new("account.create.signup")?,
        Revision::new(1)?,
        title("Signup")?,
        CapabilityId::new("account.create")?,
        LifecycleStatus::Active,
        RiskClass::High,
        vec![owner("owner://team/accounts")?],
        surfaces,
        vec![Transition::new(
            SurfaceId::new("account.create.signup.identifier")?,
            SurfaceId::new("account.create.signup.otp")?,
            TransitionTrigger::new("submit")?,
        )],
        None,
        Extensions::default(),
    )?)
}

fn requirement_with_applicability(
    id: &str,
    applicability: Applicability,
) -> Result<Requirement, Box<dyn Error>> {
    Ok(Requirement::new(
        LocalRequirementId::new(id)?,
        RequirementLevel::Required,
        RequirementScope::EachTarget,
        RequirementStatement::new("The requirement holds")?,
        vec![Facet::Behavior],
        applicability,
        None,
        None,
        Extensions::default(),
    )?)
}

fn surface_with_requirements(requirements: Vec<Requirement>) -> Result<Surface, Box<dyn Error>> {
    Ok(Surface::new(
        SurfaceId::new("account.create.signup.otp")?,
        Revision::new(1)?,
        title("OTP")?,
        JourneyId::new("account.create.signup")?,
        LifecycleStatus::Active,
        vec![owner("owner://team/accounts")?],
        requirements,
        Vec::new(),
        None,
        Extensions::default(),
    )?)
}

fn comparison(dimension: &str, value: &str) -> Result<Applicability, Box<dyn Error>> {
    Ok(Applicability::compare(
        DimensionId::new(dimension)?,
        eqm_domain::ComparisonOperator::Equal,
        SymbolicValueId::new(value)?,
    ))
}

#[test]
fn semantic_graph_extensions_change_contract_and_display_does_not() -> Result<(), Box<dyn Error>> {
    let base = canonicalize_contract_v2(&workspace()?)?;
    let mut semantic = input()?;
    semantic.extensions = extra_extension(ExtensionValue::String("x".into()))?;
    assert_ne!(
        base.digest(),
        canonicalize_contract_v2(&finalize(semantic)?)?.digest(),
        "a normative graph extension must change the contract identity"
    );
    let mut display = input()?;
    display.extensions = display_extension()?;
    assert_eq!(
        base,
        canonicalize_contract_v2(&finalize(display)?)?,
        "the display-only namespace must not change the contract identity"
    );
    Ok(())
}

#[test]
fn entity_extension_change_alters_contract() -> Result<(), Box<dyn Error>> {
    let base = canonicalize_contract_v2(&workspace()?)?;
    let mut changed = input()?;
    changed.capabilities = vec![Capability::new(
        CapabilityId::new("account.create")?,
        title("Account creation")?,
        LifecycleStatus::Active,
        vec![owner("owner://team/accounts")?],
        Some(Description::new("Create an account")?),
        extra_extension(ExtensionValue::String("cap".into()))?,
    )?];
    assert_ne!(
        base.digest(),
        canonicalize_contract_v2(&finalize(changed)?)?.digest()
    );
    Ok(())
}

#[test]
fn ordered_journey_surfaces_are_not_sorted() -> Result<(), Box<dyn Error>> {
    let otp = SurfaceId::new("account.create.signup.otp")?;
    let identifier = SurfaceId::new("account.create.signup.identifier")?;
    let mut reversed = input()?;
    reversed.journeys = vec![journey_with(vec![otp, identifier])?];
    let mut forward = input()?;
    forward.journeys = vec![journey_with(vec![
        SurfaceId::new("account.create.signup.identifier")?,
        SurfaceId::new("account.create.signup.otp")?,
    ])?];
    assert_ne!(
        canonicalize_contract_v2(&finalize(forward)?)?.digest(),
        canonicalize_contract_v2(&finalize(reversed)?)?.digest(),
        "journey surfaces are an ordered sequence"
    );
    Ok(())
}

#[test]
fn applicability_children_follow_domain_canonical_order() -> Result<(), Box<dyn Error>> {
    let build = |applicability: Applicability| -> Result<_, Box<dyn Error>> {
        let mut graph = input()?;
        graph.surfaces = vec![surface_with_requirements(vec![
            requirement_with_applicability("only", applicability)?,
        ])?];
        finalize(graph)
    };
    // The domain canonicalizes `all`/`any` child order, so a permutation of the
    // input children is the same expression and the projection must not re-sort
    // it into something else or reject it as a duplicate.
    let forward = Applicability::all(vec![
        comparison("region", "us")?,
        comparison("tier", "gold")?,
    ])?;
    let permuted = Applicability::all(vec![
        comparison("tier", "gold")?,
        comparison("region", "us")?,
    ])?;
    assert_eq!(
        canonicalize_contract_v2(&build(forward)?)?,
        canonicalize_contract_v2(&build(permuted)?)?
    );

    // A real child change (distinct semantics) does change the projection.
    let changed = Applicability::all(vec![
        comparison("region", "eu")?,
        comparison("tier", "gold")?,
    ])?;
    assert_ne!(
        canonicalize_contract_v2(&build(Applicability::all(vec![
            comparison("region", "us")?,
            comparison("tier", "gold")?,
        ])?)?)?
        .digest(),
        canonicalize_contract_v2(&build(changed)?)?.digest()
    );
    Ok(())
}

#[test]
fn canonical_requirement_set_order_is_irrelevant() -> Result<(), Box<dyn Error>> {
    let first = requirement_with_applicability("alpha", comparison("region", "us")?)?;
    let second = requirement_with_applicability("beta", comparison("region", "eu")?)?;
    let forward = {
        let mut graph = input()?;
        graph.surfaces = vec![surface_with_requirements(vec![
            first.clone(),
            second.clone(),
        ])?];
        finalize(graph)?
    };
    let reversed = {
        let mut graph = input()?;
        graph.surfaces = vec![surface_with_requirements(vec![second, first])?];
        finalize(graph)?
    };
    assert_eq!(
        canonicalize_contract_v2(&forward)?,
        canonicalize_contract_v2(&reversed)?,
        "requirement records are a canonical set"
    );
    Ok(())
}

#[test]
fn contract_extension_integer_and_text_never_collide() -> Result<(), Box<dyn Error>> {
    let build = |value: ExtensionValue| -> Result<_, Box<dyn Error>> {
        let mut graph = input()?;
        graph.extensions = extra_extension(value)?;
        finalize(graph)
    };
    for (integer, text) in [
        (ExtensionValue::Integer(i64::MIN), "-9223372036854775808"),
        (
            ExtensionValue::Integer(9_007_199_254_740_992),
            "9007199254740992",
        ),
    ] {
        let integer = canonicalize_contract_v2(&build(integer)?)?;
        let string = canonicalize_contract_v2(&build(ExtensionValue::String(text.into()))?)?;
        assert_ne!(integer.bytes(), string.bytes());
        assert_ne!(integer.digest(), string.digest());
    }
    Ok(())
}
