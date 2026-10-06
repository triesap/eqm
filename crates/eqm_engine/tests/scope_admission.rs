//! P04-R2 scope-admission controls: checked graph token, complete denominator,
//! participation truth and unresolved non-authorization.

mod support;

use eqm_domain::{DimensionId, PolicyId, ProfileId, Revision, SymbolicValueId, TargetId};
use eqm_engine::{
    ApplicabilityContext, AuthorityOrigin, CompletePolicyResolution, ConformanceError,
    EvaluationMode, FacetStatus, FragmentDigestMap, PolicyProfileRequest, PolicyRef,
    ProfileRequest, ScopeSubject, TargetConformance, UnresolvedReason, derive_required_universe,
    evaluate_complete_target_conformance, expand_fragments, resolve_graph, select_policy_profiles,
};
use eqm_manifest::canonicalize_fragment;
use std::collections::BTreeMap;
use std::error::Error;

type Finalized = eqm_engine::FinalizedWorkspaceGraph;

fn request() -> Result<PolicyProfileRequest, Box<dyn Error>> {
    Ok(PolicyProfileRequest::new(
        AuthorityOrigin::TrustedInvocation,
        PolicyRef::new(PolicyId::new("consumer.critical_flow")?, Revision::new(1)?),
        vec![ProfileRequest::new(
            ProfileId::new("audience.default")?,
            Revision::new(1)?,
            vec![(
                DimensionId::new("region")?,
                Some(SymbolicValueId::new("us")?),
            )],
        )?],
    )?)
}

fn finalize(loaded: &eqm_manifest::LoadedWorkspace) -> Result<Finalized, Box<dyn Error>> {
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let digests: FragmentDigestMap = graph
        .fragments()
        .iter()
        .map(|(key, fragment)| Ok((key.clone(), canonicalize_fragment(fragment)?.digest())))
        .collect::<Result<_, Box<dyn Error>>>()?;
    Ok(expand_fragments(graph, &digests, loaded.source_map())?)
}

fn with_participation() -> Result<(tempfile::TempDir, Finalized), Box<dyn Error>> {
    let (repository, loaded) = support::loaded_example_with(|root| {
        // Replace every authored participation document with the asymmetric
        // fixture: android declared true, ios deliberately absent so both the
        // identifier coordinate and every end-to-end target set stay Unknown and
        // non-authorizing.
        for entry in std::fs::read_dir(root.join("eqm/contracts"))? {
            let entry = entry?;
            if entry
                .file_name()
                .to_string_lossy()
                .starts_with("participation.")
            {
                std::fs::remove_file(entry.path())?;
            }
        }
        std::fs::write(
            root.join("eqm/contracts/participation.toml"),
            r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/participation.schema.json"
id = "participation.android_identifier"
requirement = "account.create.signup.identifier#email_default"
targets = ["android"]

[applicability]
always = true
"#,
        )?;
        Ok(())
    })?;
    Ok((repository, finalize(&loaded)?))
}

fn context(finalized: &Finalized) -> Result<ApplicabilityContext, Box<dyn Error>> {
    let selection = select_policy_profiles(
        finalized.graph(),
        EvaluationMode::PullRequest,
        Some(&request()?),
        None,
    )?;
    let mut inputs = Vec::new();
    for selected in selection.profiles().values() {
        let definition = finalized
            .graph()
            .profiles()
            .get(&(selected.id().clone(), selected.revision()))
            .ok_or("profile definition missing")?;
        inputs.push((definition, selected.values()));
    }
    Ok(ApplicabilityContext::from_profiles(inputs)?)
}

#[test]
fn omitted_target_set_coordinate_cannot_yield_conformant() -> Result<(), Box<dyn Error>> {
    let (_repository, loaded) = support::loaded_example()?;
    let finalized = finalize(&loaded)?;
    let selection = select_policy_profiles(
        finalized.graph(),
        EvaluationMode::PullRequest,
        Some(&request()?),
        None,
    )?;
    let applicability = context(&finalized)?;
    let universe = derive_required_universe(&finalized, &selection, &applicability, None)?;
    // The end-to-end requirement produces a target-set coordinate.
    let target_set = universe
        .coordinates()
        .iter()
        .find(|key| matches!(key.subject, ScopeSubject::TargetSet(_)))
        .cloned()
        .ok_or("target-set coordinate missing")?;
    let mut facets: BTreeMap<_, _> = universe
        .coordinates()
        .iter()
        .cloned()
        .map(|key| (key, FacetStatus::Satisfied))
        .collect();
    facets.remove(&target_set);
    let resolution = CompletePolicyResolution::new(universe, facets)?;
    assert_eq!(
        evaluate_complete_target_conformance(&TargetId::new("android")?, &resolution),
        Err(ConformanceError::IncompletePolicyResolution)
    );
    Ok(())
}

#[test]
fn asymmetric_and_unknown_participation_is_non_authorizing() -> Result<(), Box<dyn Error>> {
    let (_repository, finalized) = with_participation()?;
    let selection = select_policy_profiles(
        finalized.graph(),
        EvaluationMode::PullRequest,
        Some(&request()?),
        None,
    )?;
    let applicability = context(&finalized)?;
    let universe = derive_required_universe(&finalized, &selection, &applicability, None)?;
    let requirement: eqm_domain::FullRequirementId =
        "account.create.signup.identifier#email_default".parse()?;
    let android = TargetId::new("android")?;
    let ios = TargetId::new("ios")?;
    // Declared-true android participation yields an obligation.
    assert!(universe.coordinates().iter().any(|key| {
        key.requirement == requirement
            && matches!(&key.subject, ScopeSubject::Target(t) if t == &android)
    }));
    // A missing rule for ios is Unknown, not implicit global inclusion, so no
    // obligation is derived and the exact unknown coordinate stays visible.
    assert!(!universe.coordinates().iter().any(|key| {
        key.requirement == requirement
            && matches!(&key.subject, ScopeSubject::Target(t) if t == &ios)
    }));
    assert!(universe.unresolved().iter().any(|unresolved| {
        unresolved.requirement == requirement
            && unresolved.subject == Some(ScopeSubject::Target(ios.clone()))
            && unresolved.reason == UnresolvedReason::UnknownParticipation
    }));
    // Satisfied reported statuses for every derived coordinate still cannot
    // make a target conformant while participation is unknown anywhere in its
    // scope; the declared-true android coordinate participates, the missing ios
    // rule stays Unknown and non-authorizing.
    let facets: BTreeMap<_, _> = universe
        .coordinates()
        .iter()
        .cloned()
        .map(|key| (key, FacetStatus::Satisfied))
        .collect();
    let resolution = CompletePolicyResolution::new(universe, facets)?;
    assert_eq!(
        evaluate_complete_target_conformance(&ios, &resolution)?,
        TargetConformance::Nonconformant
    );
    assert_eq!(
        evaluate_complete_target_conformance(&android, &resolution)?,
        TargetConformance::Nonconformant
    );
    Ok(())
}

#[test]
fn distinct_context_status_maps_cannot_be_mixed() -> Result<(), Box<dyn Error>> {
    let (_repository, loaded) = support::loaded_example()?;
    let finalized = finalize(&loaded)?;
    let selection = select_policy_profiles(
        finalized.graph(),
        EvaluationMode::PullRequest,
        Some(&request()?),
        None,
    )?;
    let applicability = context(&finalized)?;
    let universe = derive_required_universe(&finalized, &selection, &applicability, None)?;
    // A status map for a different release context names coordinates outside
    // this universe and is rejected rather than silently ignored.
    let foreign: BTreeMap<_, _> = universe
        .coordinates()
        .iter()
        .take(1)
        .cloned()
        .map(|mut key| {
            key.release_context = Some(eqm_domain::Sha256Digest::hash_content(b"foreign"));
            (key, FacetStatus::Satisfied)
        })
        .collect();
    assert_eq!(
        CompletePolicyResolution::new(universe, foreign),
        Err(ConformanceError::UnexpectedCoordinate)
    );
    Ok(())
}
