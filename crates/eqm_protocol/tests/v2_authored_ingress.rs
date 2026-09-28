//! S016-R1: authored v2 metadata validation at every owning ingress.
//!
//! Each mapped family has a valid typed control and one-field mutants that
//! change only the field under test. Runtime-facts mutants recompute the
//! enclosing digest so a rejection is the authored-metadata check and not a
//! digest mismatch. Embedded inventory runs the same checks through the
//! adapter-response root as the standalone root.

use eqm_domain::Sha256Digest;
use eqm_protocol::{
    AdapterRequestV2Dto, AdapterResponseV2Dto, EvidenceResultV2Dto, InventoryV2Dto,
    RuntimeFactsV2Dto, V2DtoError,
};
use serde_json::{Value, json};
use std::error::Error;

fn digest(seed: &str) -> String {
    Sha256Digest::hash_content(seed.as_bytes()).to_string()
}

fn zeros() -> String {
    format!("sha256:{}", "0".repeat(64))
}

fn subject() -> Value {
    json!({
        "repository": "https://example.com/team/project",
        "repository_id_digest": digest("repo"),
        "scope": {"kind": "target", "target": "web"},
        "source_commit": "a".repeat(40),
        "target_configuration_digest": digest("config"),
    })
}

fn attempt() -> Value {
    json!({
        "number": 1,
        "outcome": "passed",
        "started_at": "2026-01-02T03:04:05Z",
        "finished_at": "2026-01-02T03:04:06Z",
    })
}

fn evidence_result() -> Value {
    let id = digest("identity");
    json!({
        "schema": eqm_protocol::EVIDENCE_RESULT_SCHEMA_V2.to_string(),
        "id": id,
        "subject": subject(),
        "target": "web",
        "unit": "unit.test",
        "requirements": ["unit.test#r18", "unit.test#r19"],
        "facets": ["behavior", "structure"],
        "kind": "test",
        "evidence_spec_digest": digest("spec"),
        "contract_digest": digest("contract"),
        "binding_digest": digest("binding"),
        "policy_digest": digest("policy"),
        "profile_values": [],
        "producer": "producer://ci/runner/1",
        "claimed_trust": "signed_ci",
        "observed_at": "2026-01-02T03:04:05Z",
        "payload": {
            "kind": "test",
            "execution": {
                "attempts": [attempt()],
                "counts": {"selected": 1, "passed": 1, "failed": 0, "skipped": 0, "filtered": 0, "quarantined": 0},
                "started_at": "2026-01-02T03:04:05Z",
                "finished_at": "2026-01-02T03:04:06Z",
            },
        },
        "attachments": [{"name": "report.txt", "media_type": "text/plain", "digest": zeros(), "size": 1}],
        "result_digest": id,
    })
}

fn runtime_facts() -> Value {
    json!({
        "schema": eqm_protocol::RUNTIME_FACTS_SCHEMA_V2.to_string(),
        "provider": "runtime.fixture",
        "subject": subject(),
        "target": "web",
        "profile_values": [],
        "observed_at": "2026-01-02T03:04:05Z",
        "expires_at": "2026-01-03T03:04:05Z",
        "facts": [
            {"surface": "account.create.signup.identifier", "dimension": "availability", "key": "version", "value": {"type": "boolean", "value": true}},
        ],
        "producer": "producer://runtime/fixture/snapshot-1",
        "claimed_trust": "signed_ci",
        "facts_digest": "",
    })
}

fn inventory() -> Value {
    json!({
        "schema": eqm_protocol::INVENTORY_SCHEMA_V2.to_string(),
        "adapter": "adapter.test",
        "adapter_digest": digest("adapter"),
        "subject": subject(),
        "target": "web",
        "generated_at": "2026-01-02T03:04:05Z",
        "completeness": "complete",
        "entries": [{
            "kind": "view",
            "key": "home",
            "attributes": {"framework": {"type": "text", "value": "compose"}},
            "source": "src/home.kt",
        }],
        "diagnostics": [{
            "code": "adapter.one",
            "severity": "warning",
            "message": "observed",
            "unit": "unit.test",
            "target": "web",
            "related": [],
        }],
        "inventory_digest": digest("inventory"),
    })
}

fn adapter_request() -> Value {
    json!({
        "schema": eqm_protocol::ADAPTER_REQUEST_SCHEMA_V2.to_string(),
        "request_id": "request-1",
        "adapter": "adapter.test",
        "adapter_digest": digest("adapter"),
        "operation": "discover",
        "subject": subject(),
        "target": "web",
        "target_root": "/tmp/target",
        "limits": {
            "timeout_ms": 1000,
            "max_input_bytes": 4194304,
            "max_output_bytes": 16777216,
            "max_entries": 10,
            "max_depth": 8,
        },
    })
}

fn adapter_response() -> Value {
    json!({
        "schema": eqm_protocol::ADAPTER_RESPONSE_SCHEMA_V2.to_string(),
        "request_id": "request-1",
        "adapter": "adapter.test",
        "adapter_digest": digest("adapter"),
        "status": "ok",
        "inventory": inventory(),
        "diagnostics": [],
    })
}

fn rehash_facts(document: &mut Value) -> Result<(), Box<dyn Error>> {
    // The fixture already holds one fact and no profile values, so this equals
    // the production canonical projection without a copy of its sorting step.
    document
        .as_object_mut()
        .ok_or("runtime facts object")?
        .remove("facts_digest");
    let digest =
        Sha256Digest::hash_content(&serde_json_canonicalizer::to_vec(document)?).to_string();
    document["facts_digest"] = json!(digest);
    Ok(())
}

fn parse_evidence(value: &Value) -> Result<EvidenceResultV2Dto, V2DtoError> {
    EvidenceResultV2Dto::from_json(&serde_json::to_vec(value).map_err(|_| V2DtoError::Json)?)
}

fn parse_inventory(value: &Value) -> Result<InventoryV2Dto, V2DtoError> {
    InventoryV2Dto::from_json(&serde_json::to_vec(value).map_err(|_| V2DtoError::Json)?)
}

fn parse_request(value: &Value) -> Result<AdapterRequestV2Dto, V2DtoError> {
    AdapterRequestV2Dto::from_json(&serde_json::to_vec(value).map_err(|_| V2DtoError::Json)?)
}

fn parse_response(value: &Value) -> Result<AdapterResponseV2Dto, V2DtoError> {
    AdapterResponseV2Dto::from_json(&serde_json::to_vec(value).map_err(|_| V2DtoError::Json)?)
}

#[test]
fn evidence_result_rejects_one_field_metadata_mutants() -> Result<(), Box<dyn Error>> {
    assert!(parse_evidence(&evidence_result()).is_ok());

    let cases: [(&str, &str, Value, V2DtoError); 14] = [
        (
            "target",
            "uppercase",
            json!("Bad-Target"),
            V2DtoError::InvalidVocabulary,
        ),
        (
            "unit",
            "single segment",
            json!("single"),
            V2DtoError::InvalidVocabulary,
        ),
        (
            "requirements",
            "unqualified",
            json!(["not-a-requirement"]),
            V2DtoError::InvalidVocabulary,
        ),
        (
            "facets",
            "unknown facet",
            json!(["nonsense"]),
            V2DtoError::InvalidVocabulary,
        ),
        (
            "kind",
            "unknown kind",
            json!("nonsense"),
            V2DtoError::InvalidVocabulary,
        ),
        (
            "claimed_trust",
            "unknown trust",
            json!("nonsense"),
            V2DtoError::InvalidVocabulary,
        ),
        (
            "producer",
            "bad producer",
            json!("producer://nobody"),
            V2DtoError::InvalidVocabulary,
        ),
        (
            "subject",
            "bad repository",
            json!({"repository": "not-a-repository"}),
            V2DtoError::InvalidSubject,
        ),
        (
            "attachments",
            "non-NFC name",
            json!([{"name": "cafe\u{0301}", "media_type": "text/plain", "digest": zeros(), "size": 1}]),
            V2DtoError::InvalidVocabulary,
        ),
        (
            "attachments",
            "empty media type",
            json!([{"name": "a", "media_type": "", "digest": zeros(), "size": 1}]),
            V2DtoError::InvalidVocabulary,
        ),
        (
            "attachments",
            "bad digest",
            json!([{"name": "a", "media_type": "text/plain", "digest": "sha256:bad", "size": 1}]),
            V2DtoError::InvalidDigest,
        ),
        (
            "profile_values",
            "bad profile",
            json!([{"profile": "single", "revision": 1, "dimension": "d", "value": "v"}]),
            V2DtoError::InvalidProfile,
        ),
        (
            "profile_values",
            "zero revision",
            json!([{"profile": "account.profile", "revision": 0, "dimension": "d", "value": "v"}]),
            V2DtoError::InvalidProfile,
        ),
        (
            "profile_values",
            "bad value",
            json!([{"profile": "account.profile", "revision": 1, "dimension": "d", "value": "Bad"}]),
            V2DtoError::InvalidProfile,
        ),
    ];
    for (field, note, replacement, expected) in cases {
        let mut value = evidence_result();
        if field == "subject" {
            value["subject"]["repository"] = replacement["repository"].clone();
        } else {
            value[field] = replacement;
        }
        assert_eq!(
            parse_evidence(&value),
            Err(expected),
            "{field} accepted {note}"
        );
    }

    // Manual review validates the reviewer and the outcome vocabulary.
    let mut review = evidence_result();
    review["kind"] = json!("manual_review");
    review["payload"] =
        json!({"kind": "manual_review", "outcome": "passed", "reviewer": "owner://team/review"});
    assert!(parse_evidence(&review).is_ok());
    let mut bad_reviewer = review.clone();
    bad_reviewer["payload"]["reviewer"] = json!("nonsense");
    assert_eq!(
        parse_evidence(&bad_reviewer),
        Err(V2DtoError::InvalidVocabulary)
    );
    let mut bad_outcome = review;
    bad_outcome["payload"]["outcome"] = json!("skipped");
    assert_eq!(
        parse_evidence(&bad_outcome),
        Err(V2DtoError::InvalidVocabulary)
    );
    Ok(())
}

#[test]
fn runtime_facts_rejects_one_field_metadata_mutants() -> Result<(), Box<dyn Error>> {
    let base = {
        let mut value = runtime_facts();
        rehash_facts(&mut value)?;
        value
    };
    let parse = |value: &Value| {
        RuntimeFactsV2Dto::from_json(&serde_json::to_vec(value).map_err(|_| V2DtoError::Json)?)
    };
    assert!(parse(&base).is_ok());

    let cases: [(&str, Value, V2DtoError); 10] = [
        ("provider", json!("Bad"), V2DtoError::InvalidSubject),
        ("target", json!("Bad-Target"), V2DtoError::InvalidSubject),
        (
            "subject_repository",
            json!("not-a-repository"),
            V2DtoError::InvalidSubject,
        ),
        (
            "subject_scope",
            json!("Bad-Target"),
            V2DtoError::InvalidSubject,
        ),
        (
            "fact_surface",
            json!("Bad-Surface"),
            V2DtoError::InvalidFact,
        ),
        (
            "fact_dimension",
            json!("Bad-Dimension"),
            V2DtoError::InvalidFact,
        ),
        ("fact_key", json!("cafe\u{0301}"), V2DtoError::InvalidFact),
        (
            "fact_symbol",
            json!({"type": "symbol", "value": "Bad-Symbol"}),
            V2DtoError::InvalidFact,
        ),
        (
            "fact_text",
            json!({"type": "text", "value": "cafe\u{0301}"}),
            V2DtoError::InvalidFact,
        ),
        (
            "profiles",
            json!([{"profile": "account.profile", "revision": 1, "dimension": "Bad-Dimension", "value": "v"}]),
            V2DtoError::InvalidProfile,
        ),
    ];
    for (field, replacement, expected) in cases {
        let mut value = base.clone();
        match field {
            "subject_repository" => value["subject"]["repository"] = replacement,
            "subject_scope" => value["subject"]["scope"]["target"] = replacement,
            "fact_surface" => value["facts"][0]["surface"] = replacement,
            "fact_dimension" => value["facts"][0]["dimension"] = replacement,
            "fact_key" => value["facts"][0]["key"] = replacement,
            "fact_symbol" | "fact_text" => value["facts"][0]["value"] = replacement,
            "profiles" => value["profile_values"] = replacement,
            _ => value[field] = replacement,
        }
        rehash_facts(&mut value)?;
        assert_eq!(parse(&value), Err(expected), "{field} accepted its mutant");
    }
    Ok(())
}

#[test]
fn inventory_rejects_one_field_metadata_mutants() -> Result<(), Box<dyn Error>> {
    assert!(parse_inventory(&inventory()).is_ok());

    let cases: [(&str, Value, V2DtoError); 11] = [
        ("adapter", json!("Bad"), V2DtoError::InvalidVocabulary),
        ("target", json!("Bad-Target"), V2DtoError::InvalidVocabulary),
        (
            "completeness",
            json!("nonsense"),
            V2DtoError::InvalidVocabulary,
        ),
        (
            "subject",
            json!("not-a-repository"),
            V2DtoError::InvalidSubject,
        ),
        ("entry_kind", json!(""), V2DtoError::InvalidEntry),
        ("entry_key", json!("cafe\u{0301}"), V2DtoError::InvalidEntry),
        (
            "entry_attribute_name",
            json!({"a\u{0301}": {"type": "boolean", "value": true}}),
            V2DtoError::InvalidEntry,
        ),
        (
            "entry_attribute_symbol",
            json!({"k": {"type": "symbol", "value": "Bad-Symbol"}}),
            V2DtoError::InvalidEntry,
        ),
        (
            "entry_attribute_text",
            json!({"k": {"type": "text", "value": "cafe\u{0301}"}}),
            V2DtoError::InvalidEntry,
        ),
        ("entry_source", json!("/absolute"), V2DtoError::InvalidEntry),
        (
            "diagnostic_unit",
            json!("single"),
            V2DtoError::InvalidVocabulary,
        ),
    ];
    for (field, replacement, expected) in cases {
        let mut value = inventory();
        match field {
            "subject" => value["subject"]["repository"] = replacement,
            "entry_kind" => value["entries"][0]["kind"] = replacement,
            "entry_key" => value["entries"][0]["key"] = replacement,
            "entry_attribute_name" | "entry_attribute_symbol" | "entry_attribute_text" => {
                value["entries"][0]["attributes"] = replacement;
            }
            "entry_source" => value["entries"][0]["source"] = replacement,
            "diagnostic_unit" => value["diagnostics"][0]["unit"] = replacement,
            _ => value[field] = replacement,
        }
        assert_eq!(
            parse_inventory(&value),
            Err(expected),
            "{field} accepted its mutant"
        );
    }

    let mut bad_target = inventory();
    bad_target["diagnostics"][0]["target"] = json!("Bad-Target");
    assert_eq!(
        parse_inventory(&bad_target),
        Err(V2DtoError::InvalidVocabulary)
    );
    Ok(())
}

#[test]
fn embedded_inventory_rejects_the_same_metadata_mutants() -> Result<(), Box<dyn Error>> {
    assert!(parse_response(&adapter_response()).is_ok());

    // The embedded inventory runs the standalone structural checks, so a
    // one-field mutation fails through the adapter-response root too.
    for (path, replacement) in [
        ("adapter", json!("Bad")),
        ("completeness", json!("nonsense")),
        ("entry_kind", json!("")),
        ("entry_source", json!("src/../escape")),
    ] {
        let mut value = adapter_response();
        match path {
            "adapter" => value["inventory"]["adapter"] = replacement,
            "completeness" => value["inventory"]["completeness"] = replacement,
            "entry_kind" => value["inventory"]["entries"][0]["kind"] = replacement,
            "entry_source" => value["inventory"]["entries"][0]["source"] = replacement,
            _ => unreachable!(),
        }
        assert!(
            parse_response(&value).is_err(),
            "embedded {path} mutant accepted"
        );
    }

    let mut bad_subject = adapter_response();
    bad_subject["inventory"]["subject"]["repository"] = json!("not-a-repository");
    assert_eq!(
        parse_response(&bad_subject),
        Err(V2DtoError::InvalidSubject)
    );
    Ok(())
}

#[test]
fn adapter_request_and_response_reject_metadata_mutants() -> Result<(), Box<dyn Error>> {
    assert!(parse_request(&adapter_request()).is_ok());
    assert!(parse_response(&adapter_response()).is_ok());

    for (field, replacement, expected) in [
        ("adapter", json!("Bad"), V2DtoError::InvalidVocabulary),
        ("target", json!("Bad-Target"), V2DtoError::InvalidVocabulary),
    ] {
        let mut value = adapter_request();
        value[field] = replacement;
        assert_eq!(parse_request(&value), Err(expected), "request {field}");
    }
    let mut bad_subject = adapter_request();
    bad_subject["subject"]["repository"] = json!("not-a-repository");
    assert_eq!(parse_request(&bad_subject), Err(V2DtoError::InvalidSubject));

    let mut bad_adapter = adapter_response();
    bad_adapter["adapter"] = json!("Bad");
    assert_eq!(
        parse_response(&bad_adapter),
        Err(V2DtoError::InvalidVocabulary)
    );
    Ok(())
}
