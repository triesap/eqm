//! S016 canonical collection and authored-text enforcement at real v2 ingress.
//!
//! These tests drive the actual `CanonicalSet` boundary through the concrete v2
//! DTOs: valid permutations are accepted and produce identical canonical bytes,
//! duplicates and logical-key collisions are rejected before a lossy collection
//! can erase them, ordered sequences keep order and multiplicity, object members
//! sort by complete canonical member bytes rather than derived Rust `Ord`,
//! authored metadata is rejected when it is not exact NFC, and the runtime-facts
//! digest binds the canonical semantic set projection rather than the received
//! array order.

use eqm_domain::{RawObjectDigest, Sha256Digest};
use eqm_protocol::{
    AdapterResponseV2Dto, CanonicalSet, EvidencePayloadV2Dto, EvidenceResultV2Dto,
    EvidenceSelectorV2Dto, InventoryV2Dto, RuntimeFactsV2Dto, V2DtoError,
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

fn inventory() -> Value {
    json!({
        "schema": eqm_protocol::INVENTORY_SCHEMA_V2.to_string(),
        "adapter": "adapter.test",
        "adapter_digest": digest("adapter"),
        "subject": subject(),
        "target": "web",
        "generated_at": "2026-01-02T03:04:05Z",
        "completeness": "complete",
        "entries": [],
        "diagnostics": [],
        "inventory_digest": digest("inventory"),
    })
}

fn adapter_response(inventory: Value) -> Value {
    json!({
        "schema": eqm_protocol::ADAPTER_RESPONSE_SCHEMA_V2.to_string(),
        "request_id": "request-1",
        "adapter": "adapter.test",
        "adapter_digest": digest("adapter"),
        "status": "ok",
        "inventory": inventory,
        "diagnostics": [],
    })
}

fn parse_evidence(value: &Value) -> Result<EvidenceResultV2Dto, V2DtoError> {
    EvidenceResultV2Dto::from_json(&serde_json::to_vec(value).map_err(|_| V2DtoError::Json)?)
}

fn parse_inventory(value: &Value) -> Result<InventoryV2Dto, V2DtoError> {
    InventoryV2Dto::from_json(&serde_json::to_vec(value).map_err(|_| V2DtoError::Json)?)
}

fn parse_adapter_response(value: &Value) -> Result<AdapterResponseV2Dto, V2DtoError> {
    AdapterResponseV2Dto::from_json(&serde_json::to_vec(value).map_err(|_| V2DtoError::Json)?)
}

/// Returns the RFC 8785 canonical JSON bytes of one returned semantic DTO, so a
/// canonical-byte equality claim compares JCS of the parsed value rather than
/// relying on the incidentals of a normal `serde_json` map emission order.
fn jcs<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(serde_json_canonicalizer::to_vec(value)?)
}

fn diagnostic(code: &str) -> Value {
    json!({"code": code, "severity": "warning", "message": "observed", "related": []})
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
fn subject_target_sets_are_canonical_and_duplicate_free() -> Result<(), Box<dyn Error>> {
    let with_targets = |targets: &[&str]| {
        let mut value = evidence_result();
        value["subject"]["scope"] = json!({"kind": "target_set", "targets": targets});
        value
    };
    let first = parse_evidence(&with_targets(&["web", "ios"]))?;
    let second = parse_evidence(&with_targets(&["ios", "web"]))?;
    assert_eq!(first, second, "target-set permutations must agree");
    assert_eq!(serde_json::to_vec(&first)?, serde_json::to_vec(&second)?);

    // Exact duplicate members reject before the set is collected.
    assert!(parse_evidence(&with_targets(&["web", "web"])).is_err());
    // An empty scope violates the nonempty target-set rule.
    assert_eq!(
        parse_evidence(&with_targets(&[])),
        Err(V2DtoError::InvalidSubject)
    );
    Ok(())
}

#[test]
fn profile_values_are_canonical_and_conflicts_reject() -> Result<(), Box<dyn Error>> {
    let member = |dimension: &str, value: &str| json!({"profile": "account.profile", "revision": 1, "dimension": dimension, "value": value});
    let with_profiles = |profiles: Value| {
        let mut value = evidence_result();
        value["profile_values"] = profiles;
        value
    };
    let first = parse_evidence(&with_profiles(json!([
        member("locale", "en"),
        member("track", "beta")
    ])))?;
    let second = parse_evidence(&with_profiles(json!([
        member("track", "beta"),
        member("locale", "en")
    ])))?;
    assert_eq!(first, second, "profile permutations must agree");
    assert_eq!(serde_json::to_vec(&first)?, serde_json::to_vec(&second)?);

    // Duplicate (profile, dimension) with a different value is a logical-key
    // collision, not a second valid record.
    assert_eq!(
        parse_evidence(&with_profiles(json!([
            member("locale", "en"),
            member("locale", "fr")
        ]))),
        Err(V2DtoError::InvalidOrdering)
    );
    // Conflicting revisions for one profile reject.
    let conflicting = json!([
        {"profile": "account.profile", "revision": 1, "dimension": "locale", "value": "en"},
        {"profile": "account.profile", "revision": 2, "dimension": "track", "value": "beta"},
    ]);
    assert_eq!(
        parse_evidence(&with_profiles(conflicting)),
        Err(V2DtoError::InvalidOrdering)
    );
    Ok(())
}

#[test]
fn inventory_entries_are_canonical_and_duplicate_free() -> Result<(), Box<dyn Error>> {
    let entry = |kind: &str, key: &str, marker: &str| json!({"kind": kind, "key": key, "attributes": {"m": {"type": "text", "value": marker}}, "source": "src/file.kt"});
    let with_entries = |entries: Value| {
        let mut value = inventory();
        value["entries"] = entries;
        value
    };
    let first = parse_inventory(&with_entries(json!([
        entry("view", "a", "x"),
        entry("view", "b", "y"),
    ])))?;
    let second = parse_inventory(&with_entries(json!([
        entry("view", "b", "y"),
        entry("view", "a", "x"),
    ])))?;
    assert_eq!(first, second, "entry permutations must agree");
    assert_eq!(serde_json::to_vec(&first)?, serde_json::to_vec(&second)?);

    // Duplicate (kind, key) with different attributes is a logical-key collision.
    assert_eq!(
        parse_inventory(&with_entries(json!([
            entry("view", "a", "x"),
            entry("view", "a", "y"),
        ]))),
        Err(V2DtoError::InvalidOrdering)
    );
    Ok(())
}

#[test]
fn embedded_inventory_entries_are_canonical_and_duplicate_free() -> Result<(), Box<dyn Error>> {
    let entry = |kind: &str, key: &str, marker: &str| json!({"kind": kind, "key": key, "attributes": {"m": {"type": "text", "value": marker}}, "source": "src/file.kt"});
    let parse = |entries: Value| -> Result<AdapterResponseV2Dto, V2DtoError> {
        let mut value = inventory();
        value["entries"] = entries;
        let bytes = serde_json::to_vec(&adapter_response(value)).map_err(|_| V2DtoError::Json)?;
        AdapterResponseV2Dto::from_json(&bytes)
    };
    let first = parse(json!([entry("view", "a", "x"), entry("view", "b", "y")]))?;
    let second = parse(json!([entry("view", "b", "y"), entry("view", "a", "x")]))?;
    assert_eq!(first, second, "embedded entry permutations must agree");
    assert_eq!(serde_json::to_vec(&first)?, serde_json::to_vec(&second)?);
    assert_eq!(
        parse(json!([entry("view", "a", "x"), entry("view", "a", "y")])),
        Err(V2DtoError::InvalidOrdering)
    );
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

fn execution_payload(numbers: &[u64]) -> Value {
    let attempts: Vec<Value> = numbers
        .iter()
        .map(|number| {
            json!({
                "number": number,
                "outcome": "passed",
                "started_at": "2026-01-02T03:04:05Z",
                "finished_at": "2026-01-02T03:04:06Z",
            })
        })
        .collect();
    json!({
        "attempts": attempts,
        "counts": {"selected": 2, "passed": 2, "failed": 0, "skipped": 0, "filtered": 0, "quarantined": 0},
        "started_at": "2026-01-02T03:04:05Z",
        "finished_at": "2026-01-02T03:04:06Z",
    })
}

fn evidence_with_execution(numbers: &[u64]) -> Value {
    let mut value = evidence_result();
    value["kind"] = json!("test");
    value["payload"] = json!({"kind": "test", "execution": execution_payload(numbers)});
    value
}

#[test]
fn ordered_sequences_preserve_order_and_multiplicity() -> Result<(), Box<dyn Error>> {
    let parsed = parse_evidence(&evidence_with_execution(&[1, 2]))?;
    let EvidencePayloadV2Dto::Test { execution } = &parsed.payload else {
        return Err("expected a test execution payload".into());
    };
    let numbers: Vec<u64> = execution
        .attempts
        .iter()
        .map(|attempt| attempt.number.get())
        .collect();
    assert_eq!(numbers, vec![1, 2], "attempt order is preserved");

    let location = |line: u32| json!({"uri": "src/a.rs", "start": {"line": line, "column": 0}, "end": {"line": line, "column": 0}});
    let diagnostics = |related: Value| json!([{"code": "adapter.one", "severity": "warning", "message": "observed", "related": related}]);
    let with_diagnostics = |diagnostics: Value| {
        let mut value = inventory();
        value["diagnostics"] = diagnostics;
        value
    };

    let parsed = parse_inventory(&with_diagnostics(diagnostics(json!([
        location(3),
        location(1),
        location(3),
    ]))))?;
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

    // Diagnostics are an ordered list: reordering changes the canonical output
    // instead of being collapsed like a semantic set.
    let first = serde_json::to_vec(&parse_inventory(&with_diagnostics(diagnostics(json!([
        location(1),
        location(3)
    ]))))?)?;
    let second = serde_json::to_vec(&parse_inventory(&with_diagnostics(diagnostics(json!([
        location(3),
        location(1)
    ]))))?)?;
    assert_ne!(first, second, "diagnostic order must be significant");
    Ok(())
}

#[test]
fn attempt_numbering_is_enforced_at_evidence_result_root() -> Result<(), Box<dyn Error>> {
    assert!(parse_evidence(&evidence_with_execution(&[1, 2])).is_ok());
    for invalid in [&[2u64, 1][..], &[1, 1][..], &[1, 3][..], &[0][..]] {
        assert_eq!(
            parse_evidence(&evidence_with_execution(invalid)),
            Err(V2DtoError::InvalidAttempt),
            "attempt numbering {invalid:?} was accepted"
        );
    }
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

const CANONICAL_FACTS_PROJECTION: &str = r#"{"claimed_trust":"signed_ci","expires_at":"2026-01-03T03:04:05Z","facts":[{"dimension":"availability","key":"a","surface":"account.create.signup.identifier","value":{"type":"boolean","value":true}},{"dimension":"availability","key":"b","surface":"account.create.signup.identifier","value":{"type":"integer","value":42}}],"observed_at":"2026-01-02T03:04:05Z","producer":"producer://runtime/fixture/snapshot-1","profile_values":[{"dimension":"locale","profile":"account.profile","revision":1,"value":"en"},{"dimension":"track","profile":"account.profile","revision":1,"value":"beta"}],"provider":"runtime.fixture","schema":"https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/protocol/runtime-facts.schema.json","subject":{"repository":"https://example.com/team/project","repository_id_digest":"sha256:071ca2227754705837aa3ef9748ed59e9f8a015fd765c42f391a4cbc271c6d5e","scope":{"kind":"target","target":"web"},"source_commit":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","target_configuration_digest":"sha256:b79606fb3afea5bd1609ed40b622142f1c98125abcfe89a76a661b0e8e343910"},"target":"web"}"#;

const CANONICAL_FACTS_DIGEST: &str =
    "sha256:2277503dd67b1bef4b36a5ed3416fd249384e0c161474ce7fa5efffb53b690e0";

/// A record whose `facts_digest` is the fixed literal digest of the canonical
/// projection above. The literal values are reviewed constants, not a test copy
/// of the production array-sorting algorithm.
fn canonical_facts_record() -> Result<Value, Box<dyn Error>> {
    let mut record: Value = serde_json::from_str(CANONICAL_FACTS_PROJECTION)?;
    record["facts_digest"] = json!(CANONICAL_FACTS_DIGEST);
    Ok(record)
}

/// Recomputes a mutant's enclosing digest so a negative test fails on the
/// intended structural check rather than a digest mismatch. This mirrors the
/// boundary only for the mutant inputs; the accepted oracle above is literal.
fn canonical_facts_digest(document: &mut Value) -> Result<String, Box<dyn Error>> {
    document
        .as_object_mut()
        .ok_or("runtime facts object")?
        .remove("facts_digest");
    for key in ["facts", "profile_values"] {
        if let Some(Value::Array(items)) = document.get_mut(key) {
            let mut encoded: Vec<(Vec<u8>, Value)> = Vec::with_capacity(items.len());
            for item in std::mem::take(items) {
                // A serializer failure is surfaced, never hidden behind an empty
                // sort key that would silently accept an uncanonicalizable mutant.
                encoded.push((serde_json_canonicalizer::to_vec(&item)?, item));
            }
            encoded.sort_by(|left, right| left.0.cmp(&right.0));
            *items = encoded.into_iter().map(|(_, value)| value).collect();
        }
    }
    Ok(Sha256Digest::hash_content(&serde_json_canonicalizer::to_vec(document)?).to_string())
}

#[test]
fn literal_canonical_runtime_facts_bytes_and_digest() -> Result<(), Box<dyn Error>> {
    let record = canonical_facts_record()?;
    let parsed = RuntimeFactsV2Dto::from_json(&serde_json::to_vec(&record)?)?;
    assert_eq!(parsed.facts_digest, CANONICAL_FACTS_DIGEST);

    let mut projection = record.clone();
    projection
        .as_object_mut()
        .ok_or("runtime facts object")?
        .remove("facts_digest");
    let canonical = String::from_utf8(serde_json_canonicalizer::to_vec(&projection)?)?;
    assert_eq!(canonical, CANONICAL_FACTS_PROJECTION);
    assert_eq!(
        Sha256Digest::hash_content(CANONICAL_FACTS_PROJECTION.as_bytes()).to_string(),
        CANONICAL_FACTS_DIGEST
    );
    Ok(())
}

#[test]
fn runtime_facts_digest_binds_canonical_semantic_set_projection() -> Result<(), Box<dyn Error>> {
    let document = canonical_facts_record()?;
    let parsed = RuntimeFactsV2Dto::from_json(&serde_json::to_vec(&document)?)?;
    let fact_keys: Vec<&str> = parsed.facts.iter().map(|fact| fact.key.as_str()).collect();
    assert_eq!(fact_keys, vec!["a", "b"]);

    // A permutation of the same set keeps the literal digest valid because the
    // boundary binds the canonical projection rather than the received order.
    let mut permuted = document.clone();
    permuted["facts"]
        .as_array_mut()
        .ok_or("facts array")?
        .reverse();
    assert!(RuntimeFactsV2Dto::from_json(&serde_json::to_vec(&permuted)?).is_ok());

    // Exact duplicate members reject before collection.
    let mut duplicated = document.clone();
    let first = duplicated["facts"][0].clone();
    duplicated["facts"] = json!([first.clone(), first]);
    let digest = canonical_facts_digest(&mut duplicated)?;
    duplicated["facts_digest"] = json!(digest);
    assert!(RuntimeFactsV2Dto::from_json(&serde_json::to_vec(&duplicated)?).is_err());

    // A shared logical key with a different value rejects by validation.
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
fn runtime_facts_profile_values_are_canonical_and_revision_conflicts_reject()
-> Result<(), Box<dyn Error>> {
    let document = canonical_facts_record()?;
    let mut permuted = document.clone();
    permuted["facts"]
        .as_array_mut()
        .ok_or("facts array")?
        .reverse();
    permuted["profile_values"]
        .as_array_mut()
        .ok_or("profile array")?
        .reverse();
    let parsed = RuntimeFactsV2Dto::from_json(&serde_json::to_vec(&document)?)?;
    let permuted = RuntimeFactsV2Dto::from_json(&serde_json::to_vec(&permuted)?)?;
    assert_eq!(parsed, permuted, "profile permutations must agree");
    let dimensions: Vec<&str> = parsed
        .profile_values
        .iter()
        .map(|value| value.dimension.as_str())
        .collect();
    assert_eq!(dimensions, vec!["locale", "track"]);

    // A conflicting revision for one profile rejects even after the enclosing
    // digest is recomputed.
    let mut conflicting = document.clone();
    conflicting["profile_values"][1]["revision"] = json!(2);
    let digest = canonical_facts_digest(&mut conflicting)?;
    conflicting["facts_digest"] = json!(digest);
    assert_eq!(
        RuntimeFactsV2Dto::from_json(&serde_json::to_vec(&conflicting)?),
        Err(V2DtoError::InvalidOrdering)
    );
    Ok(())
}

#[test]
fn strict_ingress_preserves_raw_bytes_while_semantic_view_canonicalizes()
-> Result<(), Box<dyn Error>> {
    // A raw input with non-canonical formatting/array order and an escaped
    // separator that decodes to the same semantic value. The received bytes and
    // their raw-object identity must stay exact; only the separate semantic view
    // is canonicalized.
    let mut permuted = canonical_facts_record()?;
    permuted["facts"]
        .as_array_mut()
        .ok_or("facts array")?
        .reverse();
    permuted["profile_values"]
        .as_array_mut()
        .ok_or("profile array")?
        .reverse();
    let raw = String::from_utf8(serde_json::to_vec_pretty(&permuted)?)?
        .replace("team/project", "team\\u002fproject")
        .into_bytes();
    let original = raw.clone();
    let raw_digest = RawObjectDigest::hash_content(&raw).to_string();

    let parsed = RuntimeFactsV2Dto::from_json(&raw)?;
    assert_eq!(raw, original, "strict ingress must not rewrite input bytes");
    assert_eq!(
        RawObjectDigest::hash_content(&raw).to_string(),
        raw_digest,
        "raw-object identity must stay exact"
    );

    let fact_keys: Vec<&str> = parsed.facts.iter().map(|fact| fact.key.as_str()).collect();
    let dimensions: Vec<&str> = parsed
        .profile_values
        .iter()
        .map(|value| value.dimension.as_str())
        .collect();
    assert_eq!(fact_keys, vec!["a", "b"], "semantic facts are canonical");
    assert_eq!(dimensions, vec!["locale", "track"]);
    assert_eq!(
        parsed.subject.repository,
        "https://example.com/team/project"
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

#[test]
fn evidence_result_exact_duplicates_reject_for_each_set() -> Result<(), Box<dyn Error>> {
    // Valid controls through the same readers.
    let mut facets = evidence_result();
    facets["facets"] = json!(["behavior", "structure"]);
    let control = parse_evidence(&facets)?;
    let mut reversed = evidence_result();
    reversed["facets"] = json!(["structure", "behavior"]);
    assert_eq!(
        control,
        parse_evidence(&reversed)?,
        "facet permutations agree"
    );
    assert_eq!(jcs(&control)?, jcs(&parse_evidence(&reversed)?)?);

    // An exact duplicate facet reject before the set is collected.
    let mut duplicated_facets = evidence_result();
    duplicated_facets["facets"] = json!(["behavior", "behavior"]);
    assert_eq!(parse_evidence(&duplicated_facets), Err(V2DtoError::Json));

    // Two valid attachments in reversed order produce identical canonical
    // bytes; an exact duplicate rejects.
    let attachment = |name: &str, marker: &str| json!({"name": name, "media_type": "text/plain", "digest": digest(marker), "size": 1});
    let mut first = evidence_result();
    first["attachments"] = json!([attachment("a.txt", "one"), attachment("b.txt", "two")]);
    let mut second = evidence_result();
    second["attachments"] = json!([attachment("b.txt", "two"), attachment("a.txt", "one")]);
    let first = parse_evidence(&first)?;
    let second = parse_evidence(&second)?;
    assert_eq!(first, second, "attachment permutations agree");
    assert_eq!(jcs(&first)?, jcs(&second)?);

    let mut duplicated_attachments = evidence_result();
    duplicated_attachments["attachments"] =
        json!([attachment("a.txt", "one"), attachment("a.txt", "one")]);
    assert_eq!(
        parse_evidence(&duplicated_attachments),
        Err(V2DtoError::Json)
    );

    // An exact duplicate profile value rejects; permutation and logical-key
    // controls remain in `profile_values_are_canonical_and_conflicts_reject`.
    let member = |dimension: &str, value: &str| json!({"profile": "account.profile", "revision": 1, "dimension": dimension, "value": value});
    let mut duplicated_profiles = evidence_result();
    duplicated_profiles["profile_values"] = json!([member("locale", "en"), member("locale", "en")]);
    assert_eq!(parse_evidence(&duplicated_profiles), Err(V2DtoError::Json));
    Ok(())
}

#[test]
fn runtime_facts_profile_value_exact_duplicate_and_conflicting_value_reject()
-> Result<(), Box<dyn Error>> {
    let document = canonical_facts_record()?;

    // Valid control: the reviewed literal record parses.
    assert!(RuntimeFactsV2Dto::from_json(&serde_json::to_vec(&document)?).is_ok());

    // Two identical profile values are an exact duplicate member. The enclosing
    // digest is recomputed so the failure is the structural duplicate check, not
    // a stale digest.
    let mut duplicated = document.clone();
    let first = duplicated["profile_values"][0].clone();
    duplicated["profile_values"] = json!([first.clone(), first]);
    let digest = canonical_facts_digest(&mut duplicated)?;
    duplicated["facts_digest"] = json!(digest);
    assert_eq!(
        RuntimeFactsV2Dto::from_json(&serde_json::to_vec(&duplicated)?),
        Err(V2DtoError::Json)
    );

    // The same (profile, dimension) with a different value rejects, which is a
    // distinct branch from a byte-identical duplicate member.
    let mut conflicting = document.clone();
    conflicting["profile_values"][1] = json!({
        "profile": "account.profile",
        "revision": 1,
        "dimension": "locale",
        "value": "fr",
    });
    let digest = canonical_facts_digest(&mut conflicting)?;
    conflicting["facts_digest"] = json!(digest);
    assert_eq!(
        RuntimeFactsV2Dto::from_json(&serde_json::to_vec(&conflicting)?),
        Err(V2DtoError::InvalidOrdering)
    );
    Ok(())
}

#[test]
fn inventory_entry_exact_duplicates_reject_standalone_and_embedded() -> Result<(), Box<dyn Error>> {
    let entry = |kind: &str, key: &str, marker: &str| json!({"kind": kind, "key": key, "attributes": {"m": {"type": "text", "value": marker}}, "source": "src/file.kt"});

    // Standalone owning path: valid control and exact duplicate.
    let mut standalone = inventory();
    standalone["entries"] = json!([entry("view", "a", "x"), entry("view", "b", "y")]);
    assert!(parse_inventory(&standalone).is_ok());
    let mut duplicated_standalone = inventory();
    duplicated_standalone["entries"] = json!([entry("view", "a", "x"), entry("view", "a", "x")]);
    assert_eq!(
        parse_inventory(&duplicated_standalone),
        Err(V2DtoError::Json)
    );

    // Embedded owning path through AdapterResponseV2Dto: valid control and the
    // same exact duplicate rejects there too.
    let embedded = |entries: Value| {
        let mut value = inventory();
        value["entries"] = entries;
        adapter_response(value)
    };
    assert!(
        parse_adapter_response(&embedded(json!([
            entry("view", "a", "x"),
            entry("view", "b", "y")
        ])))
        .is_ok()
    );
    assert_eq!(
        parse_adapter_response(&embedded(json!([
            entry("view", "a", "x"),
            entry("view", "a", "x")
        ]))),
        Err(V2DtoError::Json)
    );
    Ok(())
}

#[test]
fn inventory_and_response_diagnostic_vectors_keep_order_and_multiplicity()
-> Result<(), Box<dyn Error>> {
    // The diagnostic vector itself is ordered, not only a diagnostic's related
    // locations. Two distinct diagnostics swapped in order must change the
    // canonical output.
    let with_inventory_diagnostics = |diagnostics: Value| {
        let mut value = inventory();
        value["diagnostics"] = diagnostics;
        value
    };
    let forward = parse_inventory(&with_inventory_diagnostics(json!([
        diagnostic("adapter.one"),
        diagnostic("adapter.two")
    ])))?;
    let reversed = parse_inventory(&with_inventory_diagnostics(json!([
        diagnostic("adapter.two"),
        diagnostic("adapter.one")
    ])))?;
    let codes: Vec<&str> = forward
        .diagnostics
        .iter()
        .map(|d| d.code.as_str())
        .collect();
    assert_eq!(codes, vec!["adapter.one", "adapter.two"], "order preserved");
    assert_ne!(
        jcs(&forward)?,
        jcs(&reversed)?,
        "swapping distinct diagnostics must change canonical output"
    );

    // Repeated identical diagnostic records retain multiplicity instead of being
    // collapsed like a semantic set.
    let repeated = parse_inventory(&with_inventory_diagnostics(json!([
        diagnostic("adapter.one"),
        diagnostic("adapter.one")
    ])))?;
    assert_eq!(repeated.diagnostics.len(), 2);
    assert_eq!(repeated.diagnostics[0].code, repeated.diagnostics[1].code);

    // The outer AdapterResponse diagnostic vector is independently ordered and
    // retains multiplicity.
    let mut first_response = adapter_response(inventory());
    first_response["diagnostics"] = json!([diagnostic("response.one"), diagnostic("response.two")]);
    let mut second_response = adapter_response(inventory());
    second_response["diagnostics"] =
        json!([diagnostic("response.two"), diagnostic("response.one")]);
    let first = parse_adapter_response(&first_response)?;
    let second = parse_adapter_response(&second_response)?;
    assert_ne!(jcs(&first)?, jcs(&second)?, "response order is significant");

    let mut repeated_response = adapter_response(inventory());
    repeated_response["diagnostics"] =
        json!([diagnostic("response.one"), diagnostic("response.one")]);
    let repeated_response = parse_adapter_response(&repeated_response)?;
    assert_eq!(repeated_response.diagnostics.len(), 2);
    Ok(())
}
