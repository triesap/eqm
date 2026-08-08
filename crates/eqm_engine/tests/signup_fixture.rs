//! End-to-end loading and policy derivation for the three-target signup fixture.

use eqm_domain::{DimensionId, PolicyId, ProfileId, Revision, SymbolicValueId};
use eqm_engine::{
    ApplicabilityContext, AuthorityOrigin, EvaluationMode, FragmentDigestMap, PolicyProfileRequest,
    PolicyRef, ProfileRequest, ScopeSubject, derive_obligations, expand_fragments, resolve_graph,
    select_policy_profiles,
};
use eqm_manifest::{canonicalize_fragment, load_workspace};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::Path;

fn copy_tree(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        if entry.file_name() == "GIT_HEAD.fixture" {
            fs::create_dir_all(destination.join(".git"))?;
            fs::copy(entry.path(), destination.join(".git/HEAD"))?;
        } else if entry.file_name() == "eqm.toml.fixture" {
            fs::copy(entry.path(), destination.join("eqm.toml"))?;
        } else if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &destination.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.join(entry.file_name()))?;
        }
    }
    Ok(())
}

#[test]
fn current_signup_fixture_resolves_and_derives_all_target_obligations() -> Result<(), Box<dyn Error>>
{
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/signup");
    let root = tempfile::tempdir()?;
    copy_tree(&source, root.path())?;
    let loaded = load_workspace(root.path(), None)?;
    assert_eq!(loaded.graph_input().bindings.len(), 6);
    assert_eq!(loaded.graph_input().targets.len(), 3);
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let selection_graph = graph.clone();
    let digests: FragmentDigestMap = graph
        .fragments()
        .iter()
        .map(|(key, fragment)| Ok((key.clone(), canonicalize_fragment(fragment)?.digest())))
        .collect::<Result<_, Box<dyn Error>>>()?;
    let finalized = expand_fragments(graph, &digests, loaded.source_map())?;
    let request = PolicyProfileRequest::new(
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
    )?;
    let selection = select_policy_profiles(
        &selection_graph,
        EvaluationMode::PullRequest,
        Some(&request),
        None,
    )?;
    let profile = selection_graph
        .profiles()
        .values()
        .next()
        .ok_or("profile")?;
    let applicability = ApplicabilityContext::new(
        profile,
        BTreeMap::from([(
            DimensionId::new("region")?,
            Some(SymbolicValueId::new("us")?),
        )]),
    )?;
    let obligations = derive_obligations(&finalized, &selection, &applicability, None)?;
    assert_eq!(
        obligations
            .obligations
            .keys()
            .filter(|key| matches!(key.subject, ScopeSubject::Target(_)))
            .count(),
        6
    );
    assert_eq!(
        obligations
            .obligations
            .keys()
            .filter(|key| matches!(key.subject, ScopeSubject::TargetSet(_)))
            .count(),
        1
    );
    assert!(obligations.unmatched_warnings.is_empty());
    Ok(())
}
