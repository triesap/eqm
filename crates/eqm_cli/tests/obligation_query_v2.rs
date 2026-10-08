//! RA14 contribution: actual process success/error JSON versus its real schema.
#[path = "../src/test_support.rs"]
#[allow(dead_code)] // The shared helper also serves other fixture families.
mod test_support;
use eqm_protocol::{ObligationsReportV2Dto, RESULT_SCHEMA_V2};
use jsonschema::{Draft, Validator};
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

fn validator() -> Result<Validator, Box<dyn Error>> {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../schemas/v2/protocol/result.schema.json");
    let schema: Value = serde_json::from_slice(&fs::read(path)?)?;
    Ok(jsonschema::options()
        .with_draft(Draft::Draft202012)
        .should_validate_formats(true)
        .build(&schema)?)
}
fn snapshot(
    root: &Path,
) -> Result<std::collections::BTreeMap<std::path::PathBuf, Vec<u8>>, Box<dyn Error>> {
    fn walk(
        root: &Path,
        path: &Path,
        out: &mut std::collections::BTreeMap<std::path::PathBuf, Vec<u8>>,
    ) -> Result<(), Box<dyn Error>> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                walk(root, &entry.path(), out)?;
            } else {
                out.insert(
                    entry.path().strip_prefix(root)?.to_path_buf(),
                    fs::read(entry.path())?,
                );
            }
        }
        Ok(())
    }
    let mut result = std::collections::BTreeMap::new();
    walk(root, root, &mut result)?;
    Ok(result)
}
fn invoke(
    root: &Path,
    args: &[&str],
    expected_exit: i32,
    v: &Validator,
) -> Result<Value, Box<dyn Error>> {
    let before = snapshot(root)?;
    let output = Command::new(env!("CARGO_BIN_EXE_eqm"))
        .current_dir(root)
        .env_clear()
        .args(args)
        .args(["--format", "json", "--no-progress"])
        .output()?;
    assert_eq!(
        output.status.code(),
        Some(expected_exit),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        snapshot(root)?,
        before,
        "query process changed workspace or invoked runner sentinel"
    );
    let value: Value = serde_json::from_slice(&output.stdout)?;
    if !v.is_valid(&value) {
        return Err(format!(
            "schema failures: {:?}",
            v.iter_errors(&value)
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
        )
        .into());
    }
    assert_eq!(value["schema"], RESULT_SCHEMA_V2.to_string());
    ObligationsReportV2Dto::from_json(&output.stdout)?;
    Ok(value)
}
#[test]
fn real_success_filtered_and_execution_error_outputs_validate() -> Result<(), Box<dyn Error>> {
    let root = test_support::example_repository()?;
    fs::create_dir_all(root.path().join(".eqm/evidence"))?;
    fs::write(
        root.path().join(".eqm/evidence/poison.json"),
        b"\xffnot evidence",
    )?;
    for program in ["apps/android/gradlew", "apps/ios/bin/run-xctest"] {
        let path = root.path().join(program);
        fs::create_dir_all(path.parent().ok_or("sentinel parent")?)?;
        fs::write(
            &path,
            b"#!/bin/sh\nprintf invoked > runner-invoked\nexit 97\n",
        )?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o755))?;
        }
    }
    let v = validator()?;
    let all = invoke(root.path(), &["obligations"], 0, &v)?;
    assert_eq!(all["outcome"], "success");
    boundary_controls(&all, &v)?;
    assert_eq!(
        all["result"]["denominator"],
        serde_json::json!({"required":4,"unresolved":1})
    );
    let filtered = invoke(
        root.path(),
        &["obligations", "--target", "android", "--status", "pending"],
        0,
        &v,
    )?;
    assert_eq!(
        filtered["result"]["denominator"],
        all["result"]["denominator"]
    );
    assert_eq!(filtered["result"]["returned_count"], 2);
    let invalid = invoke(root.path(), &["obligations", "--status", "missing"], 6, &v)?;
    assert_eq!(invalid["error"]["code"], "invalid_request");
    assert!(invalid.get("result").is_none());
    let authority = invoke(
        root.path(),
        &["--policy", "missing.policy@1", "obligations"],
        6,
        &v,
    )?;
    assert_eq!(authority["error"]["code"], "invalid_authority");
    fs::write(
        root.path()
            .join("eqm/contracts/auth.signup.identifier.toml"),
        "schema = \"unsupported\"\n",
    )?;
    let malformed = invoke(root.path(), &["obligations"], 6, &v)?;
    assert_eq!(malformed["error"]["code"], "invalid_authority");
    assert_eq!(
        malformed["error"]["source"],
        "eqm/contracts/auth.signup.identifier.toml"
    );
    let unavailable = tempfile::tempdir()?;
    let unavailable = invoke(unavailable.path(), &["obligations"], 6, &v)?;
    assert_eq!(unavailable["error"]["code"], "unavailable_workspace");
    Ok(())
}

fn boundary_controls(original: &Value, v: &Validator) -> Result<(), Box<dyn Error>> {
    let required = original["result"]["records"]
        .as_array()
        .ok_or("records")?
        .iter()
        .position(|r| r["kind"] == "required")
        .ok_or("required")?;
    let unresolved = original["result"]["records"]
        .as_array()
        .ok_or("records")?
        .iter()
        .position(|r| r["kind"] == "unresolved")
        .ok_or("unresolved")?;
    let mut cases = Vec::new();
    let mut x = original.clone();
    x["schema"] = serde_json::json!(eqm_protocol::RESULT_SCHEMA.to_string());
    cases.push(("v1 marker", x, false));
    let mut x = original.clone();
    x["unrecognized_authority"] = serde_json::json!(true);
    cases.push(("unknown root", x, false));
    let mut x = original.clone();
    x["context"]["release_context"] = Value::Null;
    cases.push(("explicit absence null", x, false));
    let mut x = original.clone();
    x["context"]["evaluated_at"] = serde_json::json!("2026-01-02T03:04:05.000Z");
    cases.push(("redundant precision", x, false));
    let mut x = original.clone();
    x["result"]["records"][required]["state"] = serde_json::json!("not_applicable");
    cases.push(("required NA", x, false));
    let mut x = original.clone();
    x["result"]["records"][unresolved]["state"] = serde_json::json!("pending");
    cases.push(("unresolved pending", x, false));
    let mut x = original.clone();
    x["result"]["records"][required]["coordinate"]["strength"]["minimum_count"] =
        serde_json::json!(0);
    cases.push(("zero count", x, false));
    let mut x = original.clone();
    x["result"]["records"][required]["coordinate"]["strength"]["maximum_age_ms"] =
        serde_json::json!("18446744073709551615");
    cases.push(("age beyond duration", x, false));
    let mut x = original.clone();
    x["context"]["policy"]["revision"] = serde_json::json!(0);
    cases.push(("zero revision", x, false));
    let mut x = original.clone();
    x["result"]["denominator"]["required"] = serde_json::json!(100001);
    cases.push(("record bound", x, false));
    let mut x = original.clone();
    let duplicate = x["result"]["records"][required].clone();
    x["result"]["records"]
        .as_array_mut()
        .ok_or("records")?
        .push(duplicate);
    cases.push(("duplicate record", x, false));
    let mut x = original.clone();
    x["result"]["returned_count"] = serde_json::json!(0);
    cases.push(("count arithmetic", x, true));
    let mut x = original.clone();
    x["context"]["policy"]["revision"] = serde_json::json!(2);
    cases.push(("foreign context", x, true));
    let mut x = original.clone();
    x["result"]["records"][required]["reference"] =
        serde_json::json!(format!("eqm:v2:obligation:sha256:{}", "0".repeat(64)));
    cases.push(("wrong reference digest", x, true));
    for (field, value) in [
        ("unit", serde_json::json!("foreign.unit")),
        ("target", serde_json::json!("foreign")),
        ("statuses", serde_json::json!(["unknown"])),
    ] {
        let mut x = original.clone();
        x["result"]["filters"][field] = value;
        cases.push(("inconsistent presentation filter", x, true));
    }
    let mut x = original.clone();
    x["result"]["denominator"]["required"] = serde_json::json!(5);
    x["result"]["presentation_filtered"] = serde_json::json!(true);
    cases.push(("empty filter omission", x, true));
    for (name, value, schema_valid) in cases {
        assert_eq!(v.is_valid(&value), schema_valid, "{name}: schema");
        assert!(
            ObligationsReportV2Dto::from_json(&serde_json::to_vec(&value)?).is_err(),
            "{name}: strict reader"
        );
    }
    // Raw token/member checks cannot be proven by a JSON value validator.
    let raw = serde_json::to_string(original)?;
    let exponent = raw.replacen("\"required\":4", "\"required\":4e0", 1);
    assert_ne!(raw, exponent);
    assert!(v.is_valid(&serde_json::from_str::<Value>(&exponent)?));
    assert!(ObligationsReportV2Dto::from_json(exponent.as_bytes()).is_err());
    let duplicate = format!("{{\"schema\":\"{}\",{}", RESULT_SCHEMA_V2, &raw[1..]);
    assert!(ObligationsReportV2Dto::from_json(duplicate.as_bytes()).is_err());
    let error = eqm_protocol::ObligationsReportV2Dto::Error {
        schema: eqm_protocol::QuerySchemaV2,
        tool_version: eqm_protocol::ToolVersionDto,
        command: eqm_protocol::ObligationQueryCommand::Obligations,
        error: eqm_protocol::ObligationQueryFailureV2 {
            code: eqm_protocol::ObligationQueryErrorCode::ResourceLimit,
            message: "é".repeat(4096),
            source: None,
        },
    };
    let bytes = error.to_json()?;
    assert!(v.is_valid(&serde_json::from_slice(&bytes)?));
    ObligationsReportV2Dto::from_json(&bytes)?;
    let mut value: Value = serde_json::from_slice(&bytes)?;
    value["error"]["message"] = serde_json::json!("é".repeat(4097));
    assert!(!v.is_valid(&value));
    assert!(ObligationsReportV2Dto::from_json(&serde_json::to_vec(&value)?).is_err());
    Ok(())
}
