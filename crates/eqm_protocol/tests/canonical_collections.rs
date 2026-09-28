//! S016 canonical collection and authored-text enforcement at real v2 ingress.
//!
//! These tests drive the actual `CanonicalSet` boundary through the concrete v2
//! DTOs: valid permutations are accepted and produce identical canonical bytes,
//! duplicates are rejected before a lossy collection can erase them, ordered
//! sequences keep order and multiplicity, object members sort by complete
//! canonical member bytes rather than derived Rust `Ord`, authored metadata is
//! rejected when it is not NFC, and the runtime-facts digest binds the canonical
//! semantic set projection rather than the received array order.

use eqm_domain::Sha256Digest;
use eqm_protocol::{
    AttemptV2Dto, CanonicalSet, EvidenceResultV2Dto, EvidenceSelectorV2Dto, ExecutionPayloadV2Dto,
    InventoryV2Dto, RuntimeFactsV2Dto, V2DtoError,
};
use serde_json::{Value, json};
use std::error::Error;

fn digest(seed: &str) -> String {
    Sha256Digest::hash_content(seed.as_bytes()).to_string()
}

fn zeros() -> String {
    format!("sha256:{}", "0".repeat(64))
}

fn ffs() -> String {
    format!("sha256:{}", "f".repeat(64))
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
        "kind": "release_record",
        "evidence_spec_digest": digest("spec"),
        "contract_digest": digest("contract"),
        "binding_digest": digest("binding"),
        "policy_digest": digest("policy"),
        "profile_values": [],
        "producer": "producer://ci/runner/1",
        "claimed_trust": "signed_ci",
        "observed_at": "2026-01-02T03:04:05Z",
        "payload": {"kind": "release_record", "release_record_digest": digest("release")},
        "attachments": [],
        "result_digest": id,
    })
}

fn parse_evidence(value: &Value) -> Result<EvidenceResultV2Dto, V2DtoError> {
    EvidenceResultV2Dto::from_json(&serde_json::to_vec(value).map_err(|_| V2DtoError::Json)?)
}

#[test]
fn semantic_set_permutations_are_equal_and_duplicates_reject() -> Result<(), Box<dyn Error>> {
    let mut first = evidence_result();
    first["requirements"] = json!(["unit.test#r18", "unit.test#r19"]);
    first["facets"] = json!(["behavior", "structure"]);
    let mut second = evidence_result();
    second["requirements"] = json!(["unit.test#r19", "unit.test#r18"]);
    second["facets"] = json!(["structure", "behavior"]);

    let first = parse_evidence(&first)?;
    let second = parse_evidence(&second)?;
    assert_eq!(
        first, second,
        "permutations must yield the same semantic view"
    );
    assert_eq!(
        serde_json::to_vec(&first)?,
        serde_json::to_vec(&second)?,
        "permutations must yield identical canonical bytes"
    );
    assert_eq!(
        first.requirements.as_slice(),
        &["unit.test#r18".to_owned(), "unit.test#r19".to_owned()]
    );

    // Nested exact duplicates reject before the set is collected.
    let mut duplicated = evidence_result();
    duplicated["requirements"] = json!(["unit.test#r18", "unit.test#r18"]);
    assert!(parse_evidence(&duplicated).is_err());

    // Duplicate attachment names are a logical-key collision even when the
    // remaining attachment fields differ.
    let mut attachments = evidence_result();
    attachments["attachments"] = json!([
        {"name": "report.txt", "media_type": "text/plain", "digest": zeros(), "size": 1},
        {"name": "report.txt", "media_type": "text/plain", "digest": ffs(), "size": 2},
    ]);
    assert!(parse_evidence(&attachments).is_err());
    Ok(())
}

#[test]
fn object_members_sort_by_canonical_bytes_not_derived_ord() -> Result<(), Box<dyn Error>> {
    let mut value = evidence_result();
    // Derived `Ord` for AttachmentV2Dto compares `name` first, so "a" would
    // precede "b". Canonical JCS bytes compare `digest` first, so the all-zero
    // digest member precedes the all-`f` digest member.
    value["attachments"] = json!([
        {"name": "a", "media_type": "text/plain", "digest": ffs(), "size": 1},
        {"name": "b", "media_type": "text/plain", "digest": zeros(), "size": 2},
    ]);
    let parsed = parse_evidence(&value)?;
    let rendered: Value = serde_json::from_slice(&serde_json::to_vec(&parsed)?)?;
    assert_eq!(
        rendered["attachments"][0]["name"], "b",
        "canonical object ordering must follow member bytes, not derived Ord"
    );
    Ok(())
}

#[test]
fn ordered_sequences_preserve_order_and_multiplicity() -> Result<(), Box<dyn Error>> {
    let attempt = |number: u64| {
        json!({
            "number": number,
            "outcome": "passed",
            "started_at": "2026-01-02T03:04:05Z",
            "finished_at": "2026-01-02T03:04:06Z",
        })
    };
    let payload: ExecutionPayloadV2Dto = serde_json::from_value(json!({
        "attempts": [attempt(2), attempt(1), attempt(2)],
        "counts": {"selected": 0, "passed": 0, "failed": 0, "skipped": 0, "filtered": 0, "quarantined": 0},
        "started_at": "2026-01-02T03:04:05Z",
        "finished_at": "2026-01-02T03:04:06Z",
    }))?;
    let numbers: Vec<u64> = payload
        .attempts
        .iter()
        .map(|attempt: &AttemptV2Dto| attempt.number.get())
        .collect();
    assert_eq!(numbers, vec![2, 1, 2], "attempt order and multiplicity");

    let location = |line: u32| json!({"uri": "src/a.rs", "start": {"line": line, "column": 0}, "end": {"line": line, "column": 0}});
    let inventory = json!({
        "schema": eqm_protocol::INVENTORY_SCHEMA_V2.to_string(),
        "adapter": "adapter.test",
        "adapter_digest": digest("adapter"),
        "subject": subject(),
        "target": "web",
        "generated_at": "2026-01-02T03:04:05Z",
        "completeness": "complete",
        "entries": [],
        "diagnostics": [{
            "code": "adapter.one",
            "severity": "warning",
            "message": "observed",
            "related": [location(3), location(1), location(3)],
        }],
        "inventory_digest": digest("inventory"),
    });
    let parsed: InventoryV2Dto = InventoryV2Dto::from_json(&serde_json::to_vec(&inventory)?)?;
    let lines: Vec<u32> = parsed.diagnostics[0]
        .related
        .iter()
        .map(|entry| entry.start.line)
        .collect();
    assert_eq!(
        lines,
        vec![3, 1, 3],
        "related-location order and multiplicity"
    );
    Ok(())
}

#[test]
fn authored_metadata_rejects_non_nfc_without_normalizing() -> Result<(), Box<dyn Error>> {
    // `SelectorText` is Unicode-capable but must be exact NFC: attachment names
    // and build IDs demonstrate the authored-text rule directly. Typed IDs such
    // as `UnitId` are narrower than `SelectorText` and reject non-ASCII input on
    // grammar, not on normalization.
    let attachment = |name: &str| {
        let mut value = evidence_result();
        value["attachments"] = json!([
            {"name": name, "media_type": "text/plain", "digest": zeros(), "size": 1},
        ]);
        value
    };
    assert_eq!(
        parse_evidence(&attachment("cafe\u{0301}.txt")),
        Err(V2DtoError::InvalidVocabulary)
    );
    assert!(parse_evidence(&attachment("café.txt")).is_ok());

    let mut decomposed_build = evidence_result();
    decomposed_build["subject"]["build_id"] = json!("build-cafe\u{0301}");
    assert_eq!(
        parse_evidence(&decomposed_build),
        Err(V2DtoError::InvalidSubject)
    );
    let mut composed_build = evidence_result();
    composed_build["subject"]["build_id"] = json!("build-café");
    assert!(parse_evidence(&composed_build).is_ok());

    // Typed identifiers keep their own grammar: a decomposed `UnitId` and a
    // non-ASCII `UnitId` both reject, and so does a single-segment unit.
    for unit in ["cafe\u{0301}", "café", "single"] {
        let mut value = evidence_result();
        value["unit"] = json!(unit);
        assert_eq!(parse_evidence(&value), Err(V2DtoError::InvalidVocabulary));
    }

    assert_eq!(
        EvidenceSelectorV2Dto::Symbol {
            name: "cafe\u{0301}".to_owned(),
            language: None,
        }
        .validate(),
        Err(V2DtoError::InvalidVocabulary)
    );
    assert!(
        EvidenceSelectorV2Dto::Symbol {
            name: "café".to_owned(),
            language: None,
        }
        .validate()
        .is_ok()
    );
    // The selector helper reuses the closed vocabularies, not an NFC predicate:
    // an NFC `HttpMethod`/`ReleaseChannel` spelling outside the vocabulary fails.
    assert_eq!(
        EvidenceSelectorV2Dto::Route {
            path: "/signup".to_owned(),
            method: Some("head".to_owned()),
        }
        .validate(),
        Err(V2DtoError::InvalidVocabulary)
    );
    assert_eq!(
        EvidenceSelectorV2Dto::Release {
            channel: "nightly".to_owned(),
        }
        .validate(),
        Err(V2DtoError::InvalidVocabulary)
    );
    Ok(())
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
            {"surface": "account.create.signup.identifier", "dimension": "availability", "key": "a", "value": {"type": "boolean", "value": true}},
            {"surface": "account.create.signup.identifier", "dimension": "availability", "key": "b", "value": {"type": "boolean", "value": false}},
        ],
        "producer": "producer://runtime/fixture/snapshot-1",
        "claimed_trust": "signed_ci",
        "facts_digest": "",
    })
}

/// Producer-side digest over the canonical semantic set projection, mirroring
/// the enforced boundary: unordered arrays are canonically ordered before JCS.
fn canonical_facts_digest(document: &mut Value) -> Result<String, Box<dyn Error>> {
    document
        .as_object_mut()
        .ok_or("runtime facts object")?
        .remove("facts_digest");
    for key in ["facts", "profile_values"] {
        if let Some(Value::Array(items)) = document.get_mut(key) {
            let mut encoded: Vec<Value> = std::mem::take(items);
            encoded.sort_by_cached_key(|item| {
                serde_json_canonicalizer::to_vec(item).unwrap_or_default()
            });
            *items = encoded;
        }
    }
    Ok(Sha256Digest::hash_content(&serde_json_canonicalizer::to_vec(document)?).to_string())
}

#[test]
fn runtime_facts_digest_binds_canonical_semantic_set_projection() -> Result<(), Box<dyn Error>> {
    let mut document = runtime_facts();
    let facts = canonical_facts_digest(&mut document)?;
    document["facts_digest"] = json!(facts);

    let bytes = serde_json::to_vec(&document)?;
    let parsed = RuntimeFactsV2Dto::from_json(&bytes)?;
    assert_eq!(
        parsed
            .facts
            .iter()
            .map(|fact| fact.key.as_str())
            .collect::<Vec<_>>(),
        vec!["a", "b"]
    );

    // A permutation of the same set keeps the digest valid because the boundary
    // binds the canonical projection rather than the received order.
    let mut permuted = document.clone();
    let facts = permuted["facts"].as_array_mut().ok_or("facts array")?;
    facts.reverse();
    assert!(RuntimeFactsV2Dto::from_json(&serde_json::to_vec(&permuted)?).is_ok());

    // Exact duplicate members reject at deserialization.
    let mut duplicated = document.clone();
    let facts = duplicated["facts"].as_array().ok_or("facts array")?;
    let first = facts[0].clone();
    duplicated["facts"] = json!([first.clone(), first]);
    let digest = canonical_facts_digest(&mut duplicated)?;
    duplicated["facts_digest"] = json!(digest);
    assert!(RuntimeFactsV2Dto::from_json(&serde_json::to_vec(&duplicated)?).is_err());

    // A shared logical key with a different value is rejected by validation.
    let mut conflicting = document.clone();
    conflicting["facts"][1]["key"] = json!("a");
    let digest = canonical_facts_digest(&mut conflicting)?;
    conflicting["facts_digest"] = json!(digest);
    assert_eq!(
        RuntimeFactsV2Dto::from_json(&serde_json::to_vec(&conflicting)?),
        Err(V2DtoError::InvalidOrdering)
    );
    Ok(())
}

#[test]
fn canonical_set_construction_is_order_independent() -> Result<(), Box<dyn Error>> {
    let first = CanonicalSet::from_members(vec!["b".to_owned(), "a".to_owned()])?;
    let second = CanonicalSet::from_members(vec!["a".to_owned(), "b".to_owned()])?;
    assert_eq!(first, second);
    Ok(())
}
