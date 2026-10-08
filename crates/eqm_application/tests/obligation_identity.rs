//! S054 real immutable graph preparation and complete reference boundaries.
use eqm_application::{
    ObligationIdentityError, PolicyReference, PolicySelectionRequest, PreparedObligationIdentities,
    PreparedSession, ProfileSelection, SessionRequest, prepare, prepare_expected_selections,
    prepare_obligation_identities, prepare_obligations_from_request,
};
use eqm_domain::*;
use eqm_engine::{EvaluationMode, ExpectedSelectionMap, ScopeSubject};
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

fn identities(
    session: &PreparedSession,
    input: &PolicySelectionRequest,
    release: Option<Sha256Digest>,
) -> Result<PreparedObligationIdentities, Box<dyn Error>> {
    let prepared = prepare_obligations_from_request(
        session.finalized(),
        EvaluationMode::Development,
        input,
        release,
    )?;
    let expected = prepare_expected_selections(session.finalized(), &prepared, None, None)?;
    Ok(prepare_obligation_identities(
        session.finalized(),
        &expected,
    )?)
}
fn manifest_map(
    prepared: &eqm_application::PreparedObligations<'_>,
    session: &PreparedSession,
    cases: &[&str],
) -> Result<ExpectedSelectionMap, Box<dyn Error>> {
    let mut map = BTreeMap::new();
    for key in prepared.universe().coordinates() {
        if key.requirement.as_str() != "account.create.signup.identifier#email_default"
            || key.facet != Facet::Behavior
        {
            continue;
        }
        let ScopeSubject::Target(target) = &key.subject else {
            continue;
        };
        let binding = session
            .finalized()
            .graph()
            .bindings()
            .values()
            .find(|binding| binding.target() == target)
            .ok_or("binding absent")?;
        let specification = binding
            .evidence()
            .values()
            .next()
            .ok_or("specification absent")?;
        let source = ExpectedSelectionSource {
            binding: binding.id().clone(),
            binding_revision: binding.revision(),
            binding_digest: eqm_manifest::canonicalize_binding_v2(binding)?.digest(),
            specification: specification.id().clone(),
            specification_digest: eqm_manifest::canonicalize_evidence_spec_v2(specification)?
                .digest(),
            plan: PlanId::new("plan.expected")?,
            campaign: CampaignId::new("campaign.selected")?,
        };
        map.insert(
            key.clone(),
            vec![ExpectedSelectionManifest::new(
                source,
                EvidenceKind::Test,
                SelectionCountUnit::LogicalTestCase,
                cases
                    .iter()
                    .map(|case| LogicalCaseId::new(*case))
                    .collect::<Result<Vec<_>, _>>()?,
                vec![],
                false,
            )?],
        );
    }
    assert_eq!(map.len(), 2);
    Ok(map)
}
#[test]
fn one_immutable_session_repeats_complete_references_without_time_or_effect_inputs()
-> Result<(), Box<dyn Error>> {
    let (_root, session) = fixture()?;
    let first = identities(&session, &request()?, None)?;
    let second = identities(&session, &request()?, None)?;
    assert!(!first.identities().is_empty());
    assert_eq!(first.identities(), second.identities());
    assert_eq!(
        first.identities().len(),
        first.universe().coordinates().len()
    );
    let refs: std::collections::BTreeSet<_> = first
        .identities()
        .values()
        .map(|value| value.reference())
        .collect();
    assert_eq!(refs.len(), first.identities().len());
    assert!(
        refs.iter()
            .all(|value| value.starts_with("eqm:v2:obligation:sha256:"))
    );
    let (other_root, _) = fixture_with(|_| Ok(()))?;
    let later = prepare(
        &SessionRequest::new(None, "2027-02-03T04:05:06Z".parse()?),
        other_root.path(),
    )?;
    assert_eq!(
        first.identities(),
        identities(&later, &request()?, None)?.identities()
    );
    Ok(())
}
#[test]
fn authored_formatting_preserves_identity_but_semantic_policy_changes_do_not()
-> Result<(), Box<dyn Error>> {
    let (_root, session) = fixture()?;
    let original = identities(&session, &request()?, None)?;
    let (_root, formatted) = fixture_with(|root| {
        let path = root.join("eqm/policies/consumer.critical_flow.toml");
        let text = fs::read_to_string(&path)?;
        fs::write(
            path,
            format!("# presentation-only source formatting\n{text}\n"),
        )?;
        Ok(())
    })?;
    assert_eq!(
        original.identities(),
        identities(&formatted, &request()?, None)?.identities()
    );
    let (_root, stronger) = fixture_with(|root| {
        let path = root.join("eqm/policies/consumer.critical_flow.toml");
        let text = fs::read_to_string(&path)?;
        assert!(text.contains("minimum_count = 1"));
        fs::write(path, text.replace("minimum_count = 1", "minimum_count = 2"))?;
        Ok(())
    })?;
    let changed = identities(&stronger, &request()?, None)?;
    assert_eq!(
        original.universe().coordinates(),
        changed.universe().coordinates()
    );
    assert!(
        original
            .universe()
            .obligations()
            .values()
            .all(|value| value.strength.minimum_count.get() == 1)
    );
    assert!(
        changed
            .universe()
            .obligations()
            .values()
            .all(|value| value.strength.minimum_count.get() == 2)
    );
    for key in original.universe().coordinates() {
        assert_ne!(
            original.identities()[key].digest(),
            changed.identities()[key].digest()
        );
    }
    // A foreign graph cannot be combined with a same-labelled expected bundle.
    let prepared = prepare_obligations_from_request(
        session.finalized(),
        EvaluationMode::Development,
        &request()?,
        None,
    )?;
    let expected = prepare_expected_selections(session.finalized(), &prepared, None, None)?;
    assert!(matches!(
        prepare_obligation_identities(stronger.finalized(), &expected),
        Err(ObligationIdentityError::Graph)
    ));
    Ok(())
}
#[test]
fn selected_profile_and_release_context_changes_never_collide() -> Result<(), Box<dyn Error>> {
    let (_root, session) = fixture()?;
    let baseline = identities(&session, &request()?, None)?;
    let other = PolicySelectionRequest::development(
        Some(PolicyReference::new(
            PolicyId::new("consumer.critical_flow")?,
            Some(Revision::new(1)?),
        )),
        vec![ProfileSelection::new(
            ProfileId::new("audience.default")?,
            Some(Revision::new(1)?),
            vec![(
                DimensionId::new("region")?,
                Some(SymbolicValueId::new("eu")?),
            )],
        )],
    );
    let changed = identities(&session, &other, None)?;
    for (key, value) in baseline.identities() {
        let other_key = changed
            .identities()
            .keys()
            .find(|other| {
                other.unit == key.unit
                    && other.requirement == key.requirement
                    && other.subject == key.subject
                    && other.facet == key.facet
            })
            .ok_or("matching coordinate")?;
        assert_ne!(value.digest(), changed.identities()[other_key].digest());
    }
    let first = identities(
        &session,
        &request()?,
        Some(Sha256Digest::hash_content(b"release.one")),
    )?;
    let second = identities(
        &session,
        &request()?,
        Some(Sha256Digest::hash_content(b"release.two")),
    )?;
    for value in first.identities().values() {
        assert!(
            !baseline
                .identities()
                .values()
                .any(|other| other.digest() == value.digest())
        );
        assert!(
            !second
                .identities()
                .values()
                .any(|other| other.digest() == value.digest())
        );
    }
    Ok(())
}
#[test]
fn actual_component_checked_expectations_change_identity_without_observed_results()
-> Result<(), Box<dyn Error>> {
    let (_root, session) = fixture()?;
    let prepared = prepare_obligations_from_request(
        session.finalized(),
        EvaluationMode::Development,
        &request()?,
        None,
    )?;
    let missing = prepare_expected_selections(session.finalized(), &prepared, None, None)?;
    let missing = prepare_obligation_identities(session.finalized(), &missing)?;
    let first_map = manifest_map(&prepared, &session, &["case.a"])?;
    let second_map = manifest_map(&prepared, &session, &["case.a", "case.b"])?;
    let first = prepare_expected_selections(
        session.finalized(),
        &prepared,
        Some(first_map.clone()),
        None,
    )?;
    let second =
        prepare_expected_selections(session.finalized(), &prepared, Some(second_map), None)?;
    let first = prepare_obligation_identities(session.finalized(), &first)?;
    let second = prepare_obligation_identities(session.finalized(), &second)?;
    for key in first_map.keys() {
        assert_ne!(
            missing.identities()[key].digest(),
            first.identities()[key].digest()
        );
        assert_ne!(
            first.identities()[key].digest(),
            second.identities()[key].digest()
        );
    }
    for key in prepared
        .universe()
        .coordinates()
        .iter()
        .filter(|key| !first_map.contains_key(*key))
    {
        assert_eq!(
            missing.identities()[key].digest(),
            first.identities()[key].digest()
        );
    }
    Ok(())
}

#[test]
fn actual_profile_revision_unknown_and_target_configuration_are_bound() -> Result<(), Box<dyn Error>>
{
    let (_root, session) = fixture()?;
    let original = identities(&session, &request()?, None)?;
    let (_root, revision) = fixture_with(|root| {
        let path = root.join("eqm/profiles/audience.default.toml");
        let text = fs::read_to_string(&path)?;
        assert!(text.contains("revision = 1"));
        fs::write(path, text.replace("revision = 1", "revision = 2"))?;
        Ok(())
    })?;
    let revised_request = PolicySelectionRequest::development(
        Some(PolicyReference::new(
            PolicyId::new("consumer.critical_flow")?,
            Some(Revision::new(1)?),
        )),
        vec![ProfileSelection::new(
            ProfileId::new("audience.default")?,
            Some(Revision::new(2)?),
            vec![(
                DimensionId::new("region")?,
                Some(SymbolicValueId::new("us")?),
            )],
        )],
    );
    let revised = identities(&revision, &revised_request, None)?;
    assert_eq!(revised.identities().len(), original.identities().len());
    assert!(revised.identities().values().all(|value| {
        !original
            .identities()
            .values()
            .any(|old| value.digest() == old.digest())
    }));

    let unknown_request = PolicySelectionRequest::development(
        Some(PolicyReference::new(
            PolicyId::new("consumer.critical_flow")?,
            Some(Revision::new(1)?),
        )),
        vec![ProfileSelection::new(
            ProfileId::new("audience.default")?,
            Some(Revision::new(1)?),
            vec![(DimensionId::new("region")?, None)],
        )],
    );
    let unknown = identities(&session, &unknown_request, None)?;
    assert!(!unknown.identities().is_empty());
    for value in unknown.identities().values() {
        let projected: serde_json::Value = serde_json::from_slice(value.bytes())?;
        assert_eq!(
            projected["policy_selection"]["profiles"][0]["values"][0]["state"],
            "unknown"
        );
        assert!(
            projected["policy_selection"]["profiles"][0]["values"][0]
                .get("value")
                .is_none()
        );
        assert!(
            !original
                .identities()
                .values()
                .any(|old| value.digest() == old.digest())
        );
    }
    let (_root, configured) = fixture_with(|root| {
        let path = root.join("eqm.toml");
        let text = fs::read_to_string(&path)?;
        assert!(text.contains("framework = \"compose\""));
        fs::write(
            path,
            text.replace("framework = \"compose\"", "framework = \"compose_custom\""),
        )?;
        Ok(())
    })?;
    let configured = identities(&configured, &request()?, None)?;
    assert_eq!(
        original.universe().coordinates(),
        configured.universe().coordinates()
    );
    assert!(
        original
            .identities()
            .iter()
            .all(|(key, old)| old.digest() != configured.identities()[key].digest())
    );
    Ok(())
}

#[test]
fn actual_graph_excludes_display_but_preserves_semantic_extension_types()
-> Result<(), Box<dyn Error>> {
    let (_root, session) = fixture()?;
    let original = identities(&session, &request()?, None)?;
    let (_root, displayed) = fixture_with(|root| {
        let path = root.join("eqm/policies/consumer.critical_flow.toml");
        let text = fs::read_to_string(&path)?;
        fs::write(
            path,
            format!(
                "{text}\n[extensions.\"dev.equivalencematrix.display\"]\nnote = \"presentation\"\n"
            ),
        )?;
        Ok(())
    })?;
    assert_eq!(
        original.identities(),
        identities(&displayed, &request()?, None)?.identities()
    );
    let mut results = Vec::new();
    for value in ["9007199254740992", "\"9007199254740992\""] {
        let (_root, changed) = fixture_with(|root| {
            let path = root.join("eqm/policies/consumer.critical_flow.toml");
            let text = fs::read_to_string(&path)?;
            fs::write(
                path,
                format!("{text}\n[extensions.\"dev.example.semantic\"]\nvalue = {value}\n"),
            )?;
            Ok(())
        })?;
        let result = identities(&changed, &request()?, None)?;
        assert_eq!(
            original.universe().coordinates(),
            result.universe().coordinates()
        );
        results.push(result);
    }
    for (key, value) in original.identities() {
        assert_ne!(value.digest(), results[0].identities()[key].digest());
        assert_ne!(value.digest(), results[1].identities()[key].digest());
        assert_ne!(
            results[0].identities()[key].digest(),
            results[1].identities()[key].digest()
        );
    }
    Ok(())
}

#[test]
fn actual_exact_scenario_scope_retains_repeated_occurrences_and_edges() -> Result<(), Box<dyn Error>>
{
    let (_root, session) = fixture_with(|root| {
        for revision in [1, 2] {
            fs::write(
                root.join(format!("eqm/contracts/scenario.{revision}.toml")),
                format!(
                    r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/scenario.schema.json"
id = "scenario.identity"
revision = {revision}
[[participants]]
id = "phone_one"
role = "edge"
[participants.subject]
target = "android"
[[participants]]
id = "phone_two"
role = "observer"
[participants.subject]
target = "android"
[[participants]]
id = "ios_actor"
role = "edge"
[participants.subject]
target = "ios"
[[dependencies]]
from = "phone_two"
to = "phone_one"
"#
                ),
            )?;
        }
        Ok(())
    })?;
    let mut scene_refs = Vec::new();
    for revision in [1, 2] {
        let input = request()?.with_scope_bindings(RequirementScenarioBindings::new(vec![(
            FullRequirementId::new("account.create.signup.otp#valid_code_advances")?,
            ScenarioRef::new(
                ScenarioId::new("scenario.identity")?,
                Revision::new(revision)?,
            ),
        )])?);
        let result = identities(&session, &input, None)?;
        let values = result
            .identities()
            .iter()
            .filter(|(key, _)| matches!(key.subject, ScopeSubject::Scenario(_)))
            .collect::<Vec<_>>();
        assert!(!values.is_empty());
        let mut refs = std::collections::BTreeSet::new();
        for (_, value) in values {
            let wire: serde_json::Value = serde_json::from_slice(value.bytes())?;
            assert_eq!(wire["subject"]["kind"], "scenario");
            assert_eq!(
                wire["subject"]["scenario"]["participants"]
                    .as_array()
                    .ok_or("actors")?
                    .len(),
                3
            );
            assert_eq!(
                wire["subject"]["scenario"]["dependencies"]
                    .as_array()
                    .ok_or("edges")?
                    .len(),
                1
            );
            assert_eq!(
                wire["subject"]["selected_targets"]
                    .as_array()
                    .ok_or("targets")?
                    .len(),
                2
            );
            refs.insert(value.reference());
        }
        scene_refs.push(refs);
    }
    assert!(scene_refs[0].is_disjoint(&scene_refs[1]));
    Ok(())
}

#[test]
fn actual_provider_scope_binds_transitive_endpoint_configurations_and_false_depender()
-> Result<(), Box<dyn Error>> {
    let (root, session) = fixture_with(|root| {
        fs::write(
            root.join("eqm/contracts/provider.identity.toml"),
            r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/provider.schema.json"
id = "provider.identity"
configuration_digest = "sha256:071ca2227754705837aa3ef9748ed59e9f8a015fd765c42f391a4cbc271c6d5e"
owners = ["owner://team/providers"]
"#,
        )?;
        fs::create_dir_all(root.join("apps/bridge"))?;
        let path = root.join("eqm.toml");
        let text = fs::read_to_string(&path)?
            .replace(
                "[targets.android]",
                "[targets.android]\ndepends_on = [\"bridge\"]",
            )
            .replace(
                "[targets.ios]",
                "[targets.ios]\nproviders = [\"provider.identity\"]",
            );
        fs::write(
            path,
            format!(
                "{text}\n[targets.bridge]\nroot = \"apps/bridge\"\nplatform = \"bridge\"\nframework = \"fixture\"\nowners = [\"owner://team/bridge\"]\ndepends_on = [\"ios\"]\n"
            ),
        )?;
        let path = root.join("eqm/contracts/auth.signup.otp.toml");
        let text = fs::read_to_string(&path)?;
        assert!(text.contains("scope = \"end_to_end\""));
        fs::write(
            path,
            text.replace(
                "scope = \"end_to_end\"",
                "scope = \"shared_provider\"\nprovider = \"provider.identity\"",
            ),
        )?;
        let path = root.join("eqm/contracts/participation.signup.otp_advance.toml");
        let text = fs::read_to_string(&path)?;
        fs::write(
            &path,
            text.replace(
                "targets = [\"android\", \"ios\"]",
                "targets = [\"android\"]",
            )
            .replace("always = true", "always = false"),
        )?;
        fs::write(
            root.join("eqm/contracts/participation.ios_only.toml"),
            text.replace(
                "id = \"participation.signup.otp_advance\"",
                "id = \"participation.ios_only\"",
            )
            .replace("targets = [\"android\", \"ios\"]", "targets = [\"ios\"]"),
        )?;
        Ok(())
    })?;
    let original = identities(&session, &request()?, None)?;
    let providers = original
        .identities()
        .iter()
        .filter(|(key, _)| matches!(key.subject, ScopeSubject::CheckedProvider(_)))
        .collect::<Vec<_>>();
    assert!(!providers.is_empty());
    for (_, value) in &providers {
        let wire: serde_json::Value = serde_json::from_slice(value.bytes())?;
        let subject = &wire["subject"];
        assert_eq!(subject["kind"], "provider");
        assert_eq!(subject["dependers"].as_array().ok_or("dependers")?.len(), 2);
        assert_eq!(
            subject["assignments"]
                .as_array()
                .ok_or("assignments")?
                .len(),
            1
        );
        assert_eq!(
            subject["dependencies"]
                .as_array()
                .ok_or("dependency edges")?
                .len(),
            2
        );
        assert_eq!(
            subject["dependency_configurations"]
                .as_array()
                .ok_or("endpoint configurations")?
                .len(),
            3
        );
        let states = subject["participation"].as_array().ok_or("participation")?;
        assert_eq!(states.len(), 2);
        assert!(
            states
                .iter()
                .any(|value| value["target"] == "android" && value["state"] == "false")
        );
        assert!(
            states
                .iter()
                .any(|value| value["target"] == "ios" && value["state"] == "true")
        );
    }
    let path = root.path().join("eqm.toml");
    let text = fs::read_to_string(&path)?;
    assert!(text.contains("framework = \"fixture\""));
    fs::write(
        path,
        text.replace("framework = \"fixture\"", "framework = \"changed_bridge\""),
    )?;
    let changed = prepare(
        &SessionRequest::new(None, "2026-01-02T03:04:05Z".parse()?),
        root.path(),
    )?;
    let changed = identities(&changed, &request()?, None)?;
    for (key, value) in providers {
        let other = &changed.identities()[key];
        assert_ne!(value.digest(), other.digest());
        let old: serde_json::Value = serde_json::from_slice(value.bytes())?;
        let new: serde_json::Value = serde_json::from_slice(other.bytes())?;
        assert_ne!(
            old["subject"]["dependency_configurations"],
            new["subject"]["dependency_configurations"]
        );
        assert_eq!(
            old["subject"]["dependencies"],
            new["subject"]["dependencies"]
        );
    }
    Ok(())
}
