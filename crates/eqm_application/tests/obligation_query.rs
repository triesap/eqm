//! Real complete pre-evidence preparation and presentation causality.
use eqm_application::*;
use eqm_domain::*;
use eqm_engine::EvaluationMode;
use eqm_protocol::*;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::Path;

fn copy(source: &Path, target: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(target)?;
    for entry in fs::read_dir(source)? {
        let e = entry?;
        let to = target.join(e.file_name());
        if e.file_type()?.is_dir() {
            copy(&e.path(), &to)?;
        } else {
            fs::copy(e.path(), to)?;
        }
    }
    Ok(())
}
fn fixture(
    mutate: impl FnOnce(&Path) -> Result<(), Box<dyn Error>>,
) -> Result<(tempfile::TempDir, PreparedSession), Box<dyn Error>> {
    let root = tempfile::tempdir()?;
    copy(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/android-ios"),
        root.path(),
    )?;
    fs::create_dir(root.path().join(".git"))?;
    mutate(root.path())?;
    let session = prepare(
        &SessionRequest::new(None, "2026-01-02T03:04:05.123456789Z".parse()?),
        root.path(),
    )?;
    Ok((root, session))
}
fn ids(
    session: &PreparedSession,
    region: Option<&str>,
) -> Result<PreparedObligationIdentities, Box<dyn Error>> {
    let request = PolicySelectionRequest::development(
        Some(PolicyReference::new(
            "consumer.critical_flow".parse()?,
            Some(Revision::new(1)?),
        )),
        vec![eqm_application::ProfileSelection::new(
            "audience.default".parse()?,
            Some(Revision::new(1)?),
            vec![("region".parse()?, region.map(str::parse).transpose()?)],
        )],
    );
    let prepared = prepare_obligations_from_request(
        session.finalized(),
        EvaluationMode::Development,
        &request,
        None,
    )?;
    let expected = prepare_expected_selections(session.finalized(), &prepared, None, None)?;
    Ok(prepare_obligation_identities(
        session.finalized(),
        &expected,
    )?)
}
fn filters() -> ObligationQueryFiltersV2 {
    ObligationQueryFiltersV2 {
        unit: None,
        target: None,
        statuses: CanonicalSet::empty(),
    }
}
fn json(report: ObligationsReportV2Dto) -> Result<serde_json::Value, Box<dyn Error>> {
    let bytes = report.to_json()?;
    assert_eq!(ObligationsReportV2Dto::from_json(&bytes)?.to_json()?, bytes);
    Ok(serde_json::from_slice(&bytes)?)
}
fn snapshot(root: &Path) -> Result<BTreeMap<std::path::PathBuf, Vec<u8>>, Box<dyn Error>> {
    fn walk(
        root: &Path,
        path: &Path,
        out: &mut BTreeMap<std::path::PathBuf, Vec<u8>>,
    ) -> Result<(), Box<dyn Error>> {
        for e in fs::read_dir(path)? {
            let e = e?;
            if e.file_type()?.is_dir() {
                walk(root, &e.path(), out)?;
            } else {
                out.insert(
                    e.path().strip_prefix(root)?.to_path_buf(),
                    fs::read(e.path())?,
                );
            }
        }
        Ok(())
    }
    let mut out = BTreeMap::new();
    walk(root, root, &mut out)?;
    Ok(out)
}
#[test]
fn complete_literal_coordinates_pending_scope_and_filter_denominator_are_preserved()
-> Result<(), Box<dyn Error>> {
    let (root, session) = fixture(|root| {
        fs::create_dir_all(root.join(".eqm/evidence"))?;
        fs::write(
            root.join(".eqm/evidence/unreadable-result.json"),
            b"\xffnot evidence",
        )?;
        Ok(())
    })?;
    let before = snapshot(root.path())?;
    let identities = ids(&session, Some("us"))?;
    let retained = identities.clone();
    let prepared = prepare_obligation_query(
        session.finalized(),
        &identities,
        "2026-01-02T03:04:05.123456789Z".parse()?,
        true,
    )?;
    let all = json(prepared.view(filters())?)?;
    assert_eq!(
        all["context"]["evaluated_at"],
        "2026-01-02T03:04:05.123456789Z"
    );
    assert_eq!(all["context"]["offline"], true);
    assert_eq!(all["context"]["policy"]["id"], "consumer.critical_flow");
    assert_eq!(all["context"]["policy"]["revision"], 1);
    assert_eq!(all["context"]["profiles"][0]["id"], "audience.default");
    assert_eq!(all["context"]["profiles"][0]["values"][0]["value"], "us");
    assert_eq!(
        all["result"]["denominator"],
        serde_json::json!({"required":4,"unresolved":1})
    );
    assert_eq!(all["result"]["returned_count"], 5);
    assert_eq!(all["result"]["presentation_filtered"], false);
    let records = all["result"]["records"].as_array().ok_or("records")?;
    let mut expected = vec![
        ("account.create.signup.identifier#email_default", "android"),
        ("account.create.signup.identifier#email_default", "ios"),
        ("account.create.signup.otp#six_decimal_digits", "android"),
        ("account.create.signup.otp#six_decimal_digits", "ios"),
    ];
    let mut actual=records.iter().filter(|r|r["kind"]=="required").map(|r|{
        assert_eq!(r["state"],"pending");assert_eq!(r["coordinate"]["facet"],"behavior");assert_eq!(r["coordinate"]["strength"],serde_json::json!({"minimum_level":"required","minimum_trust":"trusted_ci","maximum_age_ms":86400000,"minimum_count":1}));assert!(r.get("evidence").is_none());assert!(r.get("waiver").is_none());
        (r["coordinate"]["requirement"].as_str().unwrap_or("absent"),r["coordinate"]["subject"]["target"].as_str().unwrap_or("absent"))
    }).collect::<Vec<_>>();
    expected.sort();
    actual.sort();
    assert_eq!(actual, expected);
    let unresolved = records
        .iter()
        .find(|r| r["kind"] == "unresolved")
        .ok_or("missing scope record")?;
    assert_eq!(
        unresolved["requirement"],
        "account.create.signup.otp#valid_code_advances"
    );
    assert_eq!(unresolved["reason"], "missing_scope_authority");
    assert_eq!(unresolved["state"], "unknown");
    assert!(unresolved.get("subject").is_none());
    let mut f = filters();
    f.target = Some("android".into());
    let android = json(prepared.view(f)?)?;
    assert_eq!(
        android["result"]["denominator"],
        all["result"]["denominator"]
    );
    assert_eq!(android["result"]["returned_count"], 3);
    let mut f = filters();
    f.target = Some("foreign".into());
    let foreign = json(prepared.view(f)?)?;
    assert_eq!(foreign["result"]["returned_count"], 1);
    assert_eq!(
        foreign["result"]["records"][0]["reason"],
        "missing_scope_authority"
    );
    for record in android["result"]["records"]
        .as_array()
        .ok_or("filtered records")?
    {
        assert!(records.contains(record));
    }
    assert_eq!(&identities, &retained);
    assert_eq!(prepared.universe(), retained.universe());
    assert_eq!(snapshot(root.path())?, before);
    Ok(())
}
#[test]
fn actual_unknown_false_and_unmatched_reasons_never_disappear() -> Result<(), Box<dyn Error>> {
    for (condition, region, expected) in [
        ("requirement", None, "unknown_applicability"),
        ("requirement", Some("us"), "not_applicable"),
        ("participation", None, "unknown_participation"),
        ("unmatched", Some("us"), "unmatched_policy"),
    ] {
        let (_root, session) = fixture(|root| {
            let path = root.join("eqm/contracts/auth.signup.identifier.toml");
            let text = fs::read_to_string(&path)?;
            if condition == "requirement" {
                fs::write(
                    &path,
                    format!(
                        "{text}\n[requirements.applicability]\ndimension = \"region\"\noperator = \"eq\"\nvalue = \"eu\"\n"
                    ),
                )?;
            }
            if condition == "unmatched" {
                fs::write(
                    &path,
                    text.replace("level = \"required\"", "level = \"recommended\""),
                )?;
            }
            if condition == "participation" {
                let path = root.join("eqm/contracts/participation.signup.identifier.toml");
                fs::write(
                    &path,
                    fs::read_to_string(&path)?.replace(
                        "always = true",
                        "dimension = \"region\"\noperator = \"eq\"\nvalue = \"eu\"",
                    ),
                )?;
            }
            Ok(())
        })?;
        let identities = ids(&session, region)?;
        let prepared = prepare_obligation_query(
            session.finalized(),
            &identities,
            "2026-01-02T03:04:05Z".parse()?,
            false,
        )?;
        let all = json(prepared.view(filters())?)?;
        assert_eq!(
            all["result"]["denominator"],
            serde_json::json!({"required":2,"unresolved":3})
        );
        let matching = all["result"]["records"]
            .as_array()
            .ok_or("records")?
            .iter()
            .filter(|r| r["requirement"] == "account.create.signup.identifier#email_default")
            .collect::<Vec<_>>();
        assert_eq!(matching.len(), 2);
        assert!(matching.iter().all(|r| r["reason"] == expected));
        let state = if expected == "not_applicable" {
            "not_applicable"
        } else {
            "unknown"
        };
        assert!(matching.iter().all(|r| r["state"] == state));
        if region.is_none() {
            assert_eq!(
                all["context"]["profiles"][0]["values"][0],
                serde_json::json!({"dimension":"region","state":"unknown"})
            );
        }
        let mut f = filters();
        f.statuses = CanonicalSet::from_members(vec![if state == "not_applicable" {
            ObligationQueryState::NotApplicable
        } else {
            ObligationQueryState::Unknown
        }])?;
        let selected = json(prepared.view(f)?)?;
        assert_eq!(
            selected["result"]["denominator"],
            all["result"]["denominator"]
        );
    }
    Ok(())
}
#[test]
fn foreign_semantic_graph_rejects_even_when_visible_coordinates_match() -> Result<(), Box<dyn Error>>
{
    let (_root, first) = fixture(|_| Ok(()))?;
    let original = ids(&first, Some("us"))?;
    let (_root, changed) = fixture(|root| {
        let path = root.join("eqm.toml");
        fs::write(
            &path,
            fs::read_to_string(&path)?
                .replace("framework = \"compose\"", "framework = \"compose_changed\""),
        )?;
        Ok(())
    })?;
    assert_eq!(
        original.universe().coordinates(),
        ids(&changed, Some("us"))?.universe().coordinates()
    );
    assert!(matches!(
        prepare_obligation_query(
            changed.finalized(),
            &original,
            "2026-01-02T03:04:05Z".parse()?,
            false
        ),
        Err(QueryPreparationError::ForeignGraph)
    ));
    Ok(())
}
