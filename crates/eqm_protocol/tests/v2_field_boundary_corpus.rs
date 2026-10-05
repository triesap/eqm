//! P03-R10: independent every-field/variant boundary corpus for the v2
//! semantic-graph and comparison-context wire surface.
//!
//! Each serialized member and enum variant from the P03-R9 owning audit is
//! exercised with independently authored literal boundary inputs against the
//! committed Draft 2020-12 schema with format assertions, plus the actual
//! owning domain constructor, checked reader or authored loader where one is
//! available. NFC/UTF-8-byte/collision and genuine cross-member relationship
//! rules remain explicit owning layers and are not claimed equivalent to JSON
//! Schema.

use eqm_domain::{
    AdapterId, Applicability, Artifact, ArtifactId, ArtifactRole, BindingId, CapabilityId,
    DimensionId, DurationMillis, EnvironmentBinding, EnvironmentName, EnvironmentSource,
    EvidenceKind, EvidenceSelector, EvidenceSpecId, EvidenceSpecification, Extensions, Facet,
    FragmentId, FragmentUse, FullRequirementId, Journey, JourneyId, LifecycleStatus,
    LocalRequirementId, OccurrenceId, OwnerRef, ParticipationRuleId, PlatformId, PolicyId,
    PolicySelector, PositiveCount, PositiveDays, ProfileId, ProviderId, ReleaseChannel, RepoPath,
    RepositoryIdentity, Requirement, RequirementLevel, RequirementScope, RequirementStatement,
    Revision, RiskClass, RoleId, RunnerBackend, RunnerDefinition, RunnerGuarantee, RunnerId,
    RunnerLimits, RunnerProgram, ScenarioId, SelectorText, SourceComponentId, Surface, SurfaceId,
    SymbolicValueId, TargetId, Title, UnitId, WaiverId, WaiverPolicy, WorkingDirectoryTemplate,
};
use eqm_protocol::WireUint;
use eqm_protocol::schema_graph_v2::ComparisonContextV2Dto;
use jsonschema::{Draft, Validator};
use serde_json::{Value, json};
use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;

fn validator(relative: &str) -> Result<Validator, Box<dyn Error>> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative);
    let schema: Value = serde_json::from_slice(&std::fs::read(&path)?)?;
    Ok(jsonschema::options()
        .with_draft(Draft::Draft202012)
        .should_validate_formats(true)
        .build(&schema)?)
}

fn graph_validator() -> Result<Validator, Box<dyn Error>> {
    validator("schemas/v2/protocol/semantic-graph.schema.json")
}

fn comparison_validator() -> Result<Validator, Box<dyn Error>> {
    validator("schemas/v2/protocol/comparison-context.schema.json")
}

fn sha(byte: char) -> String {
    format!("sha256:{}", byte.to_string().repeat(64))
}

fn repeated(byte: u8, length: usize) -> String {
    (byte as char).to_string().repeat(length)
}

fn mutated(base: &Value, pointer: &str, value: Value) -> Value {
    let mut document = base.clone();
    let target = document.pointer_mut(pointer);
    assert!(target.is_some(), "missing pointer {pointer}");
    if let Some(target) = target {
        *target = value;
    }
    document
}

fn removed(base: &Value, pointer: &str) -> Value {
    let mut document = base.clone();
    let split = pointer.rsplit_once('/');
    assert!(split.is_some(), "bad pointer {pointer}");
    let (parent, key) = split.unwrap_or(("", ""));
    let parent_value = if parent.is_empty() {
        Some(&mut document)
    } else {
        document.pointer_mut(parent)
    };
    assert!(parent_value.is_some(), "missing parent {parent}");
    match parent_value {
        Some(Value::Object(map)) => {
            map.remove(key);
        }
        Some(Value::Array(array)) => match key.parse::<usize>() {
            Ok(index) if index < array.len() => {
                array.remove(index);
            }
            _ => {}
        },
        _ => {}
    }
    document
}

fn check(v: &Validator, base: &Value, pointer: &str, accepted: &[Value], rejected: &[Value]) {
    for value in accepted {
        let document = mutated(base, pointer, value.clone());
        assert!(
            v.is_valid(&document),
            "{pointer}: schema rejected valid {value}"
        );
    }
    for value in rejected {
        let document = mutated(base, pointer, value.clone());
        assert!(
            !v.is_valid(&document),
            "{pointer}: schema accepted invalid {value}"
        );
    }
}

fn check_required_absent(v: &Validator, base: &Value, pointer: &str) {
    let document = removed(base, pointer);
    assert!(
        !v.is_valid(&document),
        "{pointer}: schema accepted an absent required member"
    );
}

fn check_null_rejected(v: &Validator, base: &Value, pointer: &str) {
    check(v, base, pointer, &[], &[Value::Null]);
}

/// Asserts schema acceptance equals the owning ASCII-lexical acceptance.
fn parity(
    v: &Validator,
    base: &Value,
    pointer: &str,
    values: &[String],
    owner: impl Fn(&str) -> bool,
) {
    for value in values {
        let document = mutated(base, pointer, json!(value));
        let schema_ok = v.is_valid(&document);
        let owner_ok = owner(value);
        assert_eq!(
            schema_ok, owner_ok,
            "{pointer}: value={value:?} schema={schema_ok} owner={owner_ok}"
        );
    }
}

/// Asserts schema acceptance equals an owning numeric decision for the same
/// exact JSON value, including its canonical number/string wire form.
fn parity_value(
    v: &Validator,
    base: &Value,
    pointer: &str,
    values: &[Value],
    owner: impl Fn(&Value) -> bool,
) {
    for value in values {
        let document = mutated(base, pointer, value.clone());
        let schema_ok = v.is_valid(&document);
        let owner_ok = owner(value);
        assert_eq!(
            schema_ok, owner_ok,
            "{pointer}: value={value:?} schema={schema_ok} owner={owner_ok}"
        );
    }
}

/// Decodes one value through the real `WireUint` canonical number/string form.
fn wire_uint(value: &Value) -> Option<u64> {
    serde_json::from_value::<WireUint>(value.clone())
        .ok()
        .map(WireUint::get)
}

fn owning_positive_u64(value: &Value) -> bool {
    wire_uint(value).is_some_and(|n| n >= 1)
}

fn owning_duration_millis(value: &Value) -> bool {
    wire_uint(value).is_some_and(|ms| ms <= i64::MAX as u64)
}

fn owning_positive_duration_millis(value: &Value) -> bool {
    wire_uint(value).is_some_and(|ms| (1..=i64::MAX as u64).contains(&ms))
}

fn owning_positive_days(value: &Value) -> bool {
    wire_uint(value).is_some_and(|days| (1..=u64::from(u32::MAX)).contains(&days))
}

fn ext_boolean(value: bool) -> Value {
    json!({"type": "boolean", "value": value})
}

fn requirement() -> Value {
    json!({
        "id": "create",
        "level": "required",
        "scope": "shared_provider",
        "statement": "Create an account",
        "facets": ["behavior"],
        "applicability": {"always": true},
        "risk_class": "high",
        "provider": "provider.mailgun",
        "extensions": {}
    })
}

fn fragment_origin() -> Value {
    json!({
        "fragment": "account.card",
        "revision": 1,
        "digest": sha('4'),
        "prefix": "card"
    })
}

fn artifact() -> Value {
    json!({
        "id": "entry",
        "role": "entrypoint",
        "path": "apps/web/main.rs",
        "surface": "account.create.flow.home",
        "symbol": "main",
        "selector": {"kind": "symbol", "name": "main", "language": "rust"},
        "extensions": {}
    })
}

fn evidence_spec() -> Value {
    json!({
        "id": "probe",
        "kind": "test",
        "requirements": ["account.create#create"],
        "facets": ["behavior"],
        "runner": "runner.local",
        "selector": {"kind": "test", "framework": "cargo", "test_id": "probe", "suite": "unit"},
        "minimum_count": 1,
        "freshness": 0,
        "extensions": {}
    })
}

fn exposure() -> Value {
    json!({
        "surface": "account.create.flow.home",
        "state": "required",
        "applicability": {"always": true},
        "route": "/home",
        "extensions": {}
    })
}

fn cohort_assignment() -> Value {
    json!({
        "occurrence": "mailer",
        "repository": "https://example.com/team/project",
        "source_commit": "a".repeat(40),
        "source_content_digest": sha('1'),
        "provider_configuration_digest": sha('2'),
        "build_id": "build.n.mailer",
        "artifact_digest": sha('3')
    })
}

fn selected_profile() -> Value {
    json!({
        "definition": {
            "id": "audience.default",
            "revision": 1,
            "title": "Audience",
            "owners": ["owner://team/product"],
            "dimensions": [{"id": "locale", "values": ["en", "fr"], "description": "Locale"}],
            "defaults": {"locale": "en"},
            "description": "Default audience",
            "extensions": {}
        },
        "values": [{"dimension": "locale", "state": "known", "value": "en"}]
    })
}

/// A fully populated valid v2 semantic-graph document. Every optional member is
/// present so that each exact nested field can be mutated independently.
fn graph_document() -> Value {
    json!({
        "schema": eqm_protocol::SEMANTIC_GRAPH_SCHEMA_V2.to_string(),
        "capabilities": [{
            "id": "account.create",
            "title": "Account creation",
            "status": "active",
            "owners": ["owner://team/accounts"],
            "description": "Account capability",
            "extensions": {}
        }],
        "journeys": [{
            "id": "account.create.flow",
            "revision": 1,
            "title": "Account flow",
            "capability": "account.create",
            "status": "active",
            "risk_class": "low",
            "owners": ["owner://team/accounts"],
            "surfaces": ["account.create.flow.home"],
            "transitions": [{
                "from": "account.create.flow.home",
                "to": "account.create.flow.done",
                "trigger": "submit"
            }],
            "description": "Journey description",
            "extensions": {}
        }],
        "surfaces": [{
            "id": "account.create.flow.home",
            "revision": 1,
            "title": "Home",
            "journey": "account.create.flow",
            "status": "active",
            "owners": ["owner://team/accounts"],
            "requirements": [requirement()],
            "fragment_origins": [{
                "fragment": "account.card",
                "revision": 1,
                "digest": sha('4'),
                "prefix": "card"
            }],
            "description": "Surface description",
            "extensions": {}
        }],
        "fragments": [{
            "id": "account.card",
            "revision": 1,
            "title": "Card",
            "risk_class": "low",
            "owners": ["owner://team/accounts"],
            "requirements": [requirement()],
            "description": "Fragment description",
            "extensions": {}
        }],
        "targets": [{
            "id": "web",
            "root": "apps/web",
            "platform": "web",
            "framework": "sveltekit",
            "owners": ["owner://team/web"],
            "role": "edge",
            "source_components": ["shared.kmp"],
            "depends_on": ["api"],
            "providers": ["provider.mailgun"],
            "extensions": {}
        }],
        "source_components": [{
            "id": "shared.kmp",
            "roots": ["shared/kmp"],
            "owners": ["owner://team/shared"],
            "dependencies": ["shared.core"],
            "extensions": {}
        }],
        "providers": [{
            "id": "provider.mailgun",
            "configuration_digest": sha('5'),
            "owners": ["owner://team/providers"],
            "extensions": {}
        }],
        "cohorts": [{
            "id": "cohort.n",
            "revision": 1,
            "scenario": "signup.flow",
            "assignments": [cohort_assignment()],
            "extensions": {}
        }],
        "participation": [{
            "id": "participation.mobile",
            "requirement": "account.create#create",
            "targets": ["web"],
            "applicability": {"always": true},
            "extensions": {}
        }],
        "bindings": [{
            "id": "binding.web",
            "revision": 1,
            "target": "web",
            "unit": "account.card",
            "owners": ["owner://team/web"],
            "artifacts": [artifact()],
            "exposures": [exposure()],
            "evidence": [evidence_spec()],
            "extensions": {}
        }],
        "scenarios": [{
            "id": "signup.flow",
            "revision": 1,
            "participants": [{
                "id": "mailer",
                "role": "provider",
                "subject": {"kind": "provider", "provider": "provider.mailgun"}
            }],
            "dependencies": [{"from": "mailer", "to": "mailer"}],
            "extensions": {}
        }],
        "policies": [{
            "id": "policy.default",
            "revision": 1,
            "title": "Default policy",
            "owners": ["owner://team/security"],
            "profiles": ["audience.default"],
            "required_targets": ["web"],
            "rules": [{
                "selector": {"units": ["account.card"]},
                "minimum_level": "required",
                "facets": ["behavior"],
                "minimum_trust": "trusted_ci",
                "maximum_age": 0,
                "minimum_count": 1
            }],
            "waivers": {
                "allowed": true,
                "minimum_approvers": 1,
                "required_controls": ["behavior"],
                "maximum_days": 30
            },
            "description": "Policy description",
            "extensions": {}
        }],
        "profiles": [{
            "id": "audience.default",
            "revision": 1,
            "title": "Audience",
            "owners": ["owner://team/product"],
            "dimensions": [{"id": "locale", "values": ["en", "fr"], "description": "Locale"}],
            "defaults": {"locale": "en"},
            "description": "Profile description",
            "extensions": {}
        }],
        "runners": [{
            "id": "runner.local",
            "revision": 1,
            "owners": ["owner://team/quality"],
            "backend": "container",
            "program": {"kind": "repository", "path": "tools/runner"},
            "args": [{"kind": "literal", "value": "--json"}],
            "cwd": {"kind": "target_root"},
            "environment": [{"name": "PATH", "source": "literal", "value": "/usr/bin"}],
            "secrets": [{"name": "TOKEN", "provider": "secret://vault/token"}],
            "timeout_ms": 1000,
            "max_output_bytes": 1048576,
            "max_concurrency": 1,
            "guarantees": ["isolated_process"],
            "extensions": {}
        }],
        "waivers": [{
            "id": "waiver.security",
            "revision": 1,
            "owners": ["owner://team/security"],
            "policy": "policy.default",
            "scope": {
                "target": {"kind": "target", "target": "web"},
                "unit": "account.card",
                "requirement": "account.create#create",
                "facets": ["behavior"],
                "profiles": {"audience.default": {"locale": "en"}}
            },
            "reason": "Temporary security waiver",
            "issue": "issue://ABC-1",
            "approvers": ["owner://team/security"],
            "starts_on": "2026-01-01",
            "expires_on": "2026-02-01",
            "controls": ["behavior"],
            "extensions": {}
        }],
        "imports": [{
            "id": "account.card",
            "revision": 1,
            "source": "https://example.com/team/repo",
            "resolved": "a".repeat(40),
            "digest": sha('6'),
            "trust": "untrusted_local",
            "signature": "sig"
        }],
        "adapters": [{
            "id": "adapter.kmp",
            "version": "1.0.0",
            "source": "https://example.com/team/repo",
            "resolved": "a".repeat(40),
            "digest": sha('7'),
            "protocol": 1,
            "trust": "untrusted_local",
            "signature": "sig"
        }],
        "extensions": {}
    })
}

fn comparison_document() -> Value {
    json!({
        "schema": eqm_protocol::COMPARISON_CONTEXT_SCHEMA_V2.to_string(),
        "contract_digest": sha('1'),
        "scenario_digest": sha('2'),
        "profiles": [selected_profile()],
        "comparison_rule_digest": sha('3')
    })
}

#[test]
fn base_graph_document_is_valid() -> Result<(), Box<dyn Error>> {
    let v = graph_validator()?;
    assert!(v.is_valid(&graph_document()), "base graph document");
    Ok(())
}

#[test]
fn base_comparison_document_is_valid() -> Result<(), Box<dyn Error>> {
    let v = comparison_validator()?;
    assert!(
        v.is_valid(&comparison_document()),
        "base comparison document"
    );
    Ok(())
}

#[test]
fn graph_identifier_fields_match_owning_constructors() -> Result<(), Box<dyn Error>> {
    let v = graph_validator()?;
    let base = graph_document();
    let s63 = repeated(b'a', 63);
    let s64 = repeated(b'a', 64);
    let dotted255 = format!(
        "{}.{}.{}.{}",
        repeated(b'a', 63),
        repeated(b'b', 63),
        repeated(b'c', 63),
        repeated(b'd', 63)
    );
    let values = vec![
        "web".to_owned(),
        "web.mobile".to_owned(),
        format!("{s63}.x"),
        format!("{s64}.x"),
        "Web".to_owned(),
        "web..mobile".to_owned(),
        dotted255.clone(),
        String::new(),
    ];

    // Extensible ID fields.
    parity(&v, &base, "/targets/0/id", &values, |value| {
        TargetId::new(value).is_ok()
    });
    parity(&v, &base, "/targets/0/role", &values, |value| {
        RoleId::new(value).is_ok()
    });
    parity(&v, &base, "/targets/0/platform", &values, |value| {
        PlatformId::new(value).is_ok()
    });
    parity(&v, &base, "/targets/0/framework", &values, |value| {
        eqm_domain::FrameworkId::new(value).is_ok()
    });

    // Dotted (>=2) ID fields.
    let dotted = vec![
        "account.create".to_owned(),
        "account".to_owned(),
        "Account.Create".to_owned(),
        "account..create".to_owned(),
        format!("{s63}.x"),
        format!("{s64}.x"),
        dotted255.clone(),
    ];
    parity(&v, &base, "/capabilities/0/id", &dotted, |value| {
        CapabilityId::new(value).is_ok()
    });
    parity(&v, &base, "/fragments/0/id", &dotted, |value| {
        FragmentId::new(value).is_ok()
    });
    parity(&v, &base, "/source_components/0/id", &dotted, |value| {
        SourceComponentId::new(value).is_ok()
    });
    parity(&v, &base, "/providers/0/id", &dotted, |value| {
        ProviderId::new(value).is_ok()
    });
    parity(&v, &base, "/bindings/0/id", &dotted, |value| {
        BindingId::new(value).is_ok()
    });
    parity(&v, &base, "/bindings/0/unit", &dotted, |value| {
        UnitId::new(value).is_ok()
    });
    parity(&v, &base, "/participation/0/id", &dotted, |value| {
        ParticipationRuleId::new(value).is_ok()
    });
    parity(&v, &base, "/scenarios/0/id", &dotted, |value| {
        ScenarioId::new(value).is_ok()
    });
    parity(&v, &base, "/policies/0/id", &dotted, |value| {
        PolicyId::new(value).is_ok()
    });
    parity(&v, &base, "/profiles/0/id", &dotted, |value| {
        ProfileId::new(value).is_ok()
    });
    parity(&v, &base, "/runners/0/id", &dotted, |value| {
        RunnerId::new(value).is_ok()
    });
    parity(&v, &base, "/waivers/0/id", &dotted, |value| {
        WaiverId::new(value).is_ok()
    });
    parity(&v, &base, "/adapters/0/id", &dotted, |value| {
        AdapterId::new(value).is_ok()
    });

    // Triple/quadruple ID fields.
    let triple = vec![
        "account.create.flow".to_owned(),
        "account.create".to_owned(),
        "Account.Create.Flow".to_owned(),
        format!("{s63}.x.y"),
    ];
    parity(&v, &base, "/journeys/0/id", &triple, |value| {
        JourneyId::new(value).is_ok()
    });
    let quadruple = vec![
        "account.create.flow.home".to_owned(),
        "account.create.flow".to_owned(),
        "account.create.flow.home.extra".to_owned(),
    ];
    parity(&v, &base, "/surfaces/0/id", &quadruple, |value| {
        SurfaceId::new(value).is_ok()
    });

    // Single-segment ID fields.
    let single = vec![
        "mailer".to_owned(),
        "a_b".to_owned(),
        s63.clone(),
        s64.clone(),
        "Mailer".to_owned(),
        "mail.er".to_owned(),
    ];
    parity(
        &v,
        &base,
        "/scenarios/0/participants/0/id",
        &single,
        |value| OccurrenceId::new(value).is_ok(),
    );
    parity(
        &v,
        &base,
        "/scenarios/0/dependencies/0/from",
        &single,
        |value| OccurrenceId::new(value).is_ok(),
    );
    parity(&v, &base, "/profiles/0/dimensions/0/id", &single, |value| {
        DimensionId::new(value).is_ok()
    });
    parity(
        &v,
        &base,
        "/profiles/0/dimensions/0/values/0",
        &single,
        |value| SymbolicValueId::new(value).is_ok(),
    );
    parity(&v, &base, "/bindings/0/artifacts/0/id", &single, |value| {
        ArtifactId::new(value).is_ok()
    });
    parity(&v, &base, "/bindings/0/evidence/0/id", &single, |value| {
        EvidenceSpecId::new(value).is_ok()
    });
    parity(
        &v,
        &base,
        "/surfaces/0/requirements/0/id",
        &single,
        |value| LocalRequirementId::new(value).is_ok(),
    );

    // Fully qualified requirement identity.
    let requirement_values = vec![
        "account.create#create".to_owned(),
        "account#create".to_owned(),
        "account.create".to_owned(),
        "Account.create#create".to_owned(),
    ];
    parity(
        &v,
        &base,
        "/participation/0/requirement",
        &requirement_values,
        |value| FullRequirementId::new(value).is_ok(),
    );

    // Portable repository paths.
    let paths = vec![
        "apps/web".to_owned(),
        "apps/web/nested".to_owned(),
        "../escape".to_owned(),
        "/abs".to_owned(),
        "a\\b".to_owned(),
        "CON".to_owned(),
        "a.".to_owned(),
        "a ".to_owned(),
        "a//b".to_owned(),
        String::new(),
    ];
    parity(&v, &base, "/targets/0/root", &paths, |value| {
        RepoPath::new(value).is_ok()
    });
    parity(&v, &base, "/source_components/0/roots/0", &paths, |value| {
        RepoPath::new(value).is_ok()
    });
    parity(&v, &base, "/bindings/0/artifacts/0/path", &paths, |value| {
        RepoPath::new(value).is_ok()
    });

    // Repository identity.
    let repositories = vec![
        "https://example.com/team/repo".to_owned(),
        "https://example.com:443/team/repo".to_owned(),
        "https://example.com/team/repo/".to_owned(),
        "https://Example.com/team/repo".to_owned(),
        "http://example.com/team/repo".to_owned(),
        String::new(),
    ];
    parity(
        &v,
        &base,
        "/cohorts/0/assignments/0/repository",
        &repositories,
        |value| RepositoryIdentity::from_str(value).is_ok(),
    );
    Ok(())
}

#[test]
fn v1_control_schema_remains_parseable() -> Result<(), Box<dyn Error>> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let schema: Value = serde_json::from_slice(&std::fs::read(
        root.join("schemas/v1/protocol/semantic-graph.schema.json"),
    )?)?;
    let _ = jsonschema::options()
        .with_draft(Draft::Draft202012)
        .should_validate_formats(true)
        .build(&schema)?;
    Ok(())
}

#[test]
fn comparison_context_matches_owning_checked_reader_for_lexical_fields()
-> Result<(), Box<dyn Error>> {
    let v = comparison_validator()?;
    let base = comparison_document();
    for (pointer, valid, invalid) in [
        (
            "/profiles/0/definition/defaults",
            json!({"locale": "en"}),
            json!({"Bad": "en"}),
        ),
        (
            "/profiles/0/definition/defaults",
            json!({"locale": "en"}),
            json!({"locale": "Bad Value"}),
        ),
    ] {
        let good = mutated(&base, pointer, valid);
        let bad = mutated(&base, pointer, invalid);
        assert_eq!(
            v.is_valid(&good),
            ComparisonContextV2Dto::from_json(&serde_json::to_vec(&good)?).is_ok(),
            "{pointer} valid parity"
        );
        assert_eq!(
            v.is_valid(&bad),
            ComparisonContextV2Dto::from_json(&serde_json::to_vec(&bad)?).is_ok(),
            "{pointer} invalid parity"
        );
    }
    Ok(())
}

#[test]
fn extension_values_cover_all_variants_and_keys() -> Result<(), Box<dyn Error>> {
    let v = graph_validator()?;
    let base = graph_document();

    // Every recursive tagged variant at the graph extension root.
    check(
        &v,
        &base,
        "/extensions",
        &[
            json!({"dev.example.ext": ext_boolean(true)}),
            json!({"dev.example.ext": {"type": "integer", "value": -1}}),
            json!({"dev.example.ext": {"type": "string", "value": "text"}}),
            json!({"dev.example.ext": {"type": "array", "value": [ext_boolean(true)]}}),
            json!({"dev.example.ext": {"type": "object", "value": {"key": ext_boolean(true)}}}),
        ],
        &[
            json!({"dev.example": ext_boolean(true)}),
            json!({"Dev.example.ext": ext_boolean(true)}),
            json!({"dev..ext": ext_boolean(true)}),
            json!({"dev.example.ext": {"type": "null", "value": null}}),
            json!({"dev.example.ext": {"type": "boolean"}}),
            json!({"dev.example.ext": {"type": "boolean", "value": "true"}}),
            json!({"dev.example.ext": {"type": "boolean", "value": true, "extra": 1}}),
        ],
    );

    // Recursive object key vocabulary, including the 63/64 byte boundary.
    let nested_base = mutated(
        &base,
        "/extensions",
        json!({"dev.example.ext": {"type": "object", "value": {"key": ext_boolean(true)}}}),
    );
    let key63 = repeated(b'a', 63);
    let key64 = repeated(b'a', 64);
    check(
        &v,
        &nested_base,
        "/extensions/dev.example.ext/value",
        &[
            json!({"key": ext_boolean(true)}),
            json!({"a_b": ext_boolean(true)}),
            json!({key63.clone(): ext_boolean(true)}),
        ],
        &[
            json!({"Key": ext_boolean(true)}),
            json!({"key.sub": ext_boolean(true)}),
            json!({key64.clone(): ext_boolean(true)}),
            json!({"key": null}),
        ],
    );

    // A nested array element is validated recursively.
    check(
        &v,
        &nested_base,
        "/extensions/dev.example.ext/value",
        &[json!({"key": {"type": "array", "value": [ext_boolean(false)]}})],
        &[json!({"key": {"type": "array", "value": [{"type": "boolean", "value": 1}]}})],
    );
    Ok(())
}

#[test]
fn applicability_covers_all_variants() -> Result<(), Box<dyn Error>> {
    let v = graph_validator()?;
    let base = graph_document();
    let pointer = "/participation/0/applicability";
    check(
        &v,
        &base,
        pointer,
        &[
            json!({"always": true}),
            json!({"dimension": "locale", "operator": "eq", "value": "en"}),
            json!({"dimension": "locale", "operator": "ne", "value": "en"}),
            json!({"dimension": "locale", "operator": "in", "values": ["en", "fr"]}),
            json!({"dimension": "locale", "operator": "not_in", "values": ["en"]}),
            json!({"all": [{"always": true}, {"always": false}]}),
            json!({"any": [{"always": true}]}),
            json!({"not": {"always": true}}),
        ],
        &[
            json!({}),
            json!({"always": "true"}),
            json!({"dimension": "locale", "operator": "lt", "value": "en"}),
            json!({"dimension": "Locale", "operator": "eq", "value": "en"}),
            json!({"dimension": "locale", "operator": "eq"}),
            json!({"dimension": "locale", "operator": "eq", "value": "Bad Value"}),
            json!({"dimension": "locale", "operator": "in", "values": []}),
            json!({"dimension": "locale", "operator": "in", "values": ["en", "en"]}),
            json!({"dimension": "locale", "operator": "eq", "values": ["en"]}),
            json!({"all": []}),
            json!({"any": []}),
            json!({"not": {}}),
        ],
    );
    Ok(())
}

#[test]
fn selectors_cover_all_variants() -> Result<(), Box<dyn Error>> {
    let v = graph_validator()?;
    let base = graph_document();

    let artifact_selector = "/bindings/0/artifacts/0/selector";
    check(
        &v,
        &base,
        artifact_selector,
        &[
            json!({"kind": "symbol", "name": "main"}),
            json!({"kind": "symbol", "name": "main", "language": "rust"}),
            json!({"kind": "route", "path": "/home"}),
            json!({"kind": "route", "path": "/home", "method": "get"}),
            json!({"kind": "test", "framework": "cargo", "test_id": "probe"}),
            json!({"kind": "test", "framework": "cargo", "test_id": "probe", "suite": "unit"}),
            json!({"kind": "inventory", "record_type": "asset", "key": "logo"}),
            json!({"kind": "inventory", "record_type": "asset", "key": "logo", "value": "logo.svg"}),
        ],
        &[
            json!({"kind": "unknown"}),
            json!({"kind": "symbol"}),
            json!({"kind": "symbol", "name": "main", "extra": 1}),
            json!({"kind": "route", "path": "/home", "method": "fetch"}),
            json!({"kind": "route", "path": ""}),
            json!({"kind": "test", "framework": "cargo"}),
            json!({"kind": "inventory", "record_type": "asset", "key": ""}),
            json!({"kind": "inventory", "record_type": "asset", "key": "logo", "value": 7}),
        ],
    );

    // The evidence spec's base kind is `test`; its selector must be the owning
    // `test` variant. Every other evidence selector variant is exercised against
    // its compatible kind in `local_collection_and_combination_rules_match_owners`.
    let evidence_selector = "/bindings/0/evidence/0/selector";
    check(
        &v,
        &base,
        evidence_selector,
        &[
            json!({"kind": "test", "framework": "cargo", "test_id": "probe", "suite": "unit"}),
            json!({"kind": "test", "framework": "cargo", "test_id": "probe"}),
        ],
        &[
            json!({"kind": "snapshot", "snapshot_id": "snap-1"}),
            json!({"kind": "release", "channel": "beta"}),
            json!({"kind": "symbol", "name": "main"}),
            json!({"kind": "test", "framework": "cargo"}),
            json!({"kind": "test", "framework": "cargo", "test_id": "probe", "extra": 1}),
        ],
    );
    Ok(())
}

/// Parses one optional selector axis into the owning value list. A missing key
/// yields `None`; a present but malformed list yields an empty list so the owning
/// constructor rejects it exactly as the schema does.
fn selector_axis<T>(
    object: &serde_json::Map<String, Value>,
    key: &str,
    parse: impl Fn(&str) -> Option<T>,
) -> Option<Vec<T>> {
    object.get(key).map(|value| {
        let mut parsed = Vec::new();
        let Some(items) = value.as_array() else {
            return parsed;
        };
        for item in items {
            match item.as_str().and_then(&parse) {
                Some(value) => parsed.push(value),
                None => return Vec::new(),
            }
        }
        parsed
    })
}

fn owning_policy_selector_ok(value: &Value) -> bool {
    let Some(object) = value.as_object() else {
        return false;
    };
    let units = selector_axis(object, "units", |s| s.parse::<UnitId>().ok());
    let requirements = selector_axis(object, "requirements", |s| {
        s.parse::<FullRequirementId>().ok()
    });
    let risk_classes = selector_axis(object, "risk_classes", |s| s.parse::<RiskClass>().ok());
    let facets = selector_axis(object, "facets", |s| s.parse::<Facet>().ok());
    let scopes = selector_axis(object, "scopes", |s| s.parse::<RequirementScope>().ok());
    PolicySelector::new(units, requirements, risk_classes, facets, scopes).is_ok()
}

fn owning_waiver_policy_ok(value: &Value) -> bool {
    let Some(object) = value.as_object() else {
        return false;
    };
    let Some(allowed) = object.get("allowed").and_then(Value::as_bool) else {
        return false;
    };
    let maximum_days = match object.get("maximum_days") {
        None => None,
        Some(days) => {
            let Some(days) = days
                .as_u64()
                .and_then(|days| u32::try_from(days).ok())
                .and_then(|days| PositiveDays::new(days).ok())
            else {
                return false;
            };
            Some(days)
        }
    };
    let minimum_approvers = match object.get("minimum_approvers") {
        None => None,
        Some(count) => {
            let Some(count) = count
                .as_u64()
                .and_then(|count| PositiveCount::new(count).ok())
            else {
                return false;
            };
            Some(count)
        }
    };
    let Some(controls) = object.get("required_controls").and_then(Value::as_array) else {
        return false;
    };
    let mut parsed_controls = Vec::with_capacity(controls.len());
    for control in controls {
        match control.as_str().and_then(|s| s.parse::<Facet>().ok()) {
            Some(facet) => parsed_controls.push(facet),
            None => return false,
        }
    }
    WaiverPolicy::new(allowed, maximum_days, minimum_approvers, parsed_controls).is_ok()
}

fn owning_environment_binding_ok(value: &Value) -> bool {
    let Some(object) = value.as_object() else {
        return false;
    };
    let Some(name) = object
        .get("name")
        .and_then(Value::as_str)
        .and_then(|name| EnvironmentName::new(name).ok())
    else {
        return false;
    };
    let Some(source) = object.get("source").and_then(Value::as_str) else {
        return false;
    };
    let source = match source {
        "literal" => match object
            .get("value")
            .and_then(Value::as_str)
            .and_then(|text| SelectorText::new(text).ok())
        {
            Some(text) => EnvironmentSource::Literal(text),
            None => return false,
        },
        "trusted_path" => EnvironmentSource::TrustedPath,
        "canonical_locale" => EnvironmentSource::CanonicalLocale,
        "utc_timezone" => EnvironmentSource::UtcTimezone,
        _ => return false,
    };
    EnvironmentBinding::new(name, source).is_ok()
}

fn owning_journey_surfaces_ok(value: &Value) -> bool {
    let Some(items) = value.as_array() else {
        return false;
    };
    let mut surfaces = Vec::with_capacity(items.len());
    for item in items {
        let Some(id) = item.as_str().and_then(|s| SurfaceId::new(s).ok()) else {
            return false;
        };
        surfaces.push(id);
    }
    let build = move || -> Option<()> {
        Journey::new(
            JourneyId::new("account.create.flow").ok()?,
            Revision::new(1).ok()?,
            Title::new("Account flow").ok()?,
            CapabilityId::new("account.create").ok()?,
            LifecycleStatus::Active,
            RiskClass::Low,
            vec!["owner://team/accounts".parse::<OwnerRef>().ok()?],
            surfaces,
            Vec::new(),
            None,
            Extensions::default(),
        )
        .ok()?;
        Some(())
    };
    build().is_some()
}

fn sample_requirement() -> Result<Requirement, Box<dyn Error>> {
    Ok(Requirement::new(
        LocalRequirementId::new("create")?,
        RequirementLevel::Required,
        RequirementScope::EachTarget,
        RequirementStatement::new("Create an account")?,
        vec![Facet::Behavior],
        Applicability::always(true),
        Some(RiskClass::High),
        None,
        Extensions::default(),
    )?)
}

fn sample_fragment_use() -> Result<FragmentUse, Box<dyn Error>> {
    Ok(FragmentUse::new(
        FragmentId::new("account.card")?,
        Revision::new(1)?,
        "sha256:4444444444444444444444444444444444444444444444444444444444444444".parse()?,
        Some(LocalRequirementId::new("card")?),
    ))
}

fn owning_surface_composition_ok(
    requirements: &Value,
    fragments: &Value,
) -> Result<bool, Box<dyn Error>> {
    let requirements_present = requirements
        .as_array()
        .is_some_and(|items| !items.is_empty());
    let fragments_present = fragments.as_array().is_some_and(|items| !items.is_empty());
    let requirements = if requirements_present {
        vec![sample_requirement()?]
    } else {
        Vec::new()
    };
    let fragments = if fragments_present {
        vec![sample_fragment_use()?]
    } else {
        Vec::new()
    };
    Ok(Surface::new(
        SurfaceId::new("account.create.flow.home")?,
        Revision::new(1)?,
        Title::new("Home")?,
        JourneyId::new("account.create.flow")?,
        LifecycleStatus::Active,
        vec!["owner://team/accounts".parse::<OwnerRef>()?],
        requirements,
        fragments,
        None,
        Extensions::default(),
    )
    .is_ok())
}

fn owning_requirement_scope_provider_ok(scope: &str, provider: Option<&str>) -> bool {
    let build = || -> Option<()> {
        let scope = match scope {
            "each_target" => RequirementScope::EachTarget,
            "shared_provider" => RequirementScope::SharedProvider,
            "end_to_end" => RequirementScope::EndToEnd,
            _ => return None,
        };
        let provider = match provider {
            None => None,
            Some(value) => Some(ProviderId::new(value).ok()?),
        };
        Requirement::new(
            LocalRequirementId::new("create").ok()?,
            RequirementLevel::Required,
            scope,
            RequirementStatement::new("statement").ok()?,
            vec![Facet::Behavior],
            Applicability::always(true),
            None,
            provider,
            Extensions::default(),
        )
        .ok()?;
        Some(())
    };
    build().is_some()
}

fn owning_runner_backend_guarantees_ok(backend: &str, guarantees: &[&str]) -> bool {
    let build = || -> Option<()> {
        let backend = match backend {
            "local" => RunnerBackend::Local,
            "container" => RunnerBackend::Container,
            _ => return None,
        };
        let mut parsed = Vec::new();
        for guarantee in guarantees {
            parsed.push(match *guarantee {
                "network_denied" => RunnerGuarantee::NetworkDenied,
                "read_only_source" => RunnerGuarantee::ReadOnlySource,
                "isolated_process" => RunnerGuarantee::IsolatedProcess,
                "resource_limited" => RunnerGuarantee::ResourceLimited,
                _ => return None,
            });
        }
        RunnerDefinition::new(
            RunnerId::new("runner.local").ok()?,
            Revision::new(1).ok()?,
            vec!["owner://team/quality".parse::<OwnerRef>().ok()?],
            backend,
            RunnerProgram::Repository(RepoPath::new("tools/runner").ok()?),
            Vec::new(),
            Some(WorkingDirectoryTemplate::TargetRoot),
            Vec::new(),
            Vec::new(),
            RunnerLimits::new(
                DurationMillis::new(1_000).ok()?,
                PositiveCount::new(1).ok()?,
                None,
            )
            .ok()?,
            parsed,
            Extensions::default(),
        )
        .ok()?;
        Some(())
    };
    build().is_some()
}

fn owning_artifact_role_locator_ok(role: &str, has_locator: bool) -> bool {
    let build = || -> Option<()> {
        let role = match role {
            "entrypoint" => ArtifactRole::Entrypoint,
            "view" => ArtifactRole::View,
            "route" => ArtifactRole::Route,
            "component" => ArtifactRole::Component,
            "service" => ArtifactRole::Service,
            "test" => ArtifactRole::Test,
            "configuration" => ArtifactRole::Configuration,
            "asset" => ArtifactRole::Asset,
            _ => return None,
        };
        let surface = if has_locator {
            Some(SurfaceId::new("account.create.flow.home").ok()?)
        } else {
            None
        };
        Artifact::new(
            ArtifactId::new("entry").ok()?,
            role,
            RepoPath::new("apps/web/main.rs").ok()?,
            surface,
            None,
            None,
            Extensions::default(),
        )
        .ok()?;
        Some(())
    };
    build().is_some()
}

fn owning_evidence_selector(kind: &str) -> Option<EvidenceSelector> {
    Some(match kind {
        "symbol" => EvidenceSelector::Symbol {
            name: SelectorText::new("main").ok()?,
            language: None,
        },
        "route" => EvidenceSelector::Route {
            path: SelectorText::new("/home").ok()?,
            method: None,
        },
        "inventory" => EvidenceSelector::Inventory {
            record_type: SelectorText::new("asset").ok()?,
            key: SelectorText::new("logo").ok()?,
            value: None,
        },
        "test" => EvidenceSelector::Test {
            framework: SelectorText::new("cargo").ok()?,
            test_id: SelectorText::new("probe").ok()?,
            suite: None,
        },
        "snapshot" => EvidenceSelector::Snapshot {
            snapshot_id: SelectorText::new("snap-1").ok()?,
            variant: None,
        },
        "release" => EvidenceSelector::Release {
            channel: ReleaseChannel::Beta,
        },
        _ => return None,
    })
}

fn owning_evidence_spec_ok(
    kind: &str,
    runner: bool,
    selector: Option<&str>,
    minimum_count: bool,
) -> bool {
    let build = || -> Option<()> {
        let kind = match kind {
            "structural_check" => EvidenceKind::StructuralCheck,
            "static_inventory" => EvidenceKind::StaticInventory,
            "test" => EvidenceKind::Test,
            "snapshot" => EvidenceKind::Snapshot,
            "manual_review" => EvidenceKind::ManualReview,
            "runtime_snapshot" => EvidenceKind::RuntimeSnapshot,
            "release_record" => EvidenceKind::ReleaseRecord,
            _ => return None,
        };
        let runner = if runner {
            Some(RunnerId::new("runner.local").ok()?)
        } else {
            None
        };
        let selector = match selector {
            None => None,
            Some(kind) => owning_evidence_selector(kind),
        };
        let minimum_count = if minimum_count {
            Some(PositiveCount::new(1).ok()?)
        } else {
            None
        };
        EvidenceSpecification::new(
            EvidenceSpecId::new("probe").ok()?,
            kind,
            vec![FullRequirementId::new("account.create#create").ok()?],
            vec![Facet::Behavior],
            runner,
            selector,
            minimum_count,
            None,
            Extensions::default(),
        )
        .ok()?;
        Some(())
    };
    build().is_some()
}

fn selector_document(kind: &str) -> Value {
    match kind {
        "symbol" => json!({"kind": "symbol", "name": "main"}),
        "route" => json!({"kind": "route", "path": "/home"}),
        "inventory" => json!({"kind": "inventory", "record_type": "asset", "key": "logo"}),
        "test" => json!({"kind": "test", "framework": "cargo", "test_id": "probe"}),
        "snapshot" => json!({"kind": "snapshot", "snapshot_id": "snap-1"}),
        "release" => json!({"kind": "release", "channel": "beta"}),
        other => json!({"kind": other}),
    }
}

fn evidence_document(
    kind: &str,
    runner: bool,
    selector: Option<&str>,
    minimum_count: bool,
) -> Value {
    let mut object = serde_json::Map::new();
    object.insert("id".to_owned(), json!("probe"));
    object.insert("kind".to_owned(), json!(kind));
    object.insert("requirements".to_owned(), json!(["account.create#create"]));
    object.insert("facets".to_owned(), json!(["behavior"]));
    if runner {
        object.insert("runner".to_owned(), json!("runner.local"));
    }
    if let Some(kind) = selector {
        object.insert("selector".to_owned(), selector_document(kind));
    }
    if minimum_count {
        object.insert("minimum_count".to_owned(), json!(1));
    }
    object.insert("extensions".to_owned(), json!({}));
    Value::Object(object)
}

/// Differential parity for the remaining P03-R12 owning rules: the committed
/// schema decision for one document equals the decision of the actual
/// `eqm_domain` constructor for the same fields.
#[test]
fn owning_consumer_parity_for_requirement_runner_artifact_and_evidence()
-> Result<(), Box<dyn Error>> {
    let v = graph_validator()?;
    let base = graph_document();

    for (scope, provider) in [
        ("shared_provider", Some("provider.mailgun")),
        ("shared_provider", None),
        ("each_target", Some("provider.mailgun")),
        ("each_target", None),
        ("end_to_end", Some("provider.mailgun")),
        ("end_to_end", None),
    ] {
        let mut requirement = requirement();
        requirement["scope"] = json!(scope);
        match provider {
            Some(provider) => requirement["provider"] = json!(provider),
            None => {
                requirement
                    .as_object_mut()
                    .ok_or("requirement")?
                    .remove("provider");
            }
        }
        let document = mutated(&base, "/surfaces/0/requirements/0", requirement.clone());
        assert_eq!(
            v.is_valid(&document),
            owning_requirement_scope_provider_ok(scope, provider),
            "requirement scope/provider {scope}/{provider:?}"
        );
    }

    for (backend, guarantees) in [
        ("local", &[][..]),
        ("local", &["isolated_process"][..]),
        ("container", &[][..]),
        ("container", &["isolated_process"][..]),
    ] {
        let mut runner = base["runners"][0].clone();
        runner["backend"] = json!(backend);
        runner["guarantees"] = json!(guarantees);
        let document = mutated(&base, "/runners/0", runner.clone());
        assert_eq!(
            v.is_valid(&document),
            owning_runner_backend_guarantees_ok(backend, guarantees),
            "runner backend/guarantees {backend}/{guarantees:?}"
        );
    }

    for (role, has_locator) in [
        ("entrypoint", true),
        ("entrypoint", false),
        ("view", false),
        ("service", false),
        ("asset", false),
    ] {
        let mut artifact = artifact();
        artifact["role"] = json!(role);
        if !has_locator {
            for key in ["surface", "symbol", "selector"] {
                artifact.as_object_mut().ok_or("artifact")?.remove(key);
            }
        }
        let document = mutated(&base, "/bindings/0/artifacts/0", artifact.clone());
        assert_eq!(
            v.is_valid(&document),
            owning_artifact_role_locator_ok(role, has_locator),
            "artifact role/locator {role}/{has_locator}"
        );
    }

    for (kind, runner, selector, minimum_count) in [
        ("structural_check", true, Some("symbol"), true),
        ("static_inventory", false, Some("inventory"), true),
        ("test", true, Some("test"), true),
        ("snapshot", true, Some("snapshot"), false),
        ("runtime_snapshot", false, Some("inventory"), true),
        ("release_record", false, Some("release"), false),
        ("manual_review", false, None, false),
        ("structural_check", true, Some("test"), true),
        ("test", false, Some("test"), true),
        ("snapshot", true, Some("snapshot"), true),
        ("manual_review", true, None, false),
        ("manual_review", false, Some("symbol"), false),
    ] {
        let evidence = evidence_document(kind, runner, selector, minimum_count);
        let document = mutated(&base, "/bindings/0/evidence/0", evidence.clone());
        assert_eq!(
            v.is_valid(&document),
            owning_evidence_spec_ok(kind, runner, selector, minimum_count),
            "evidence {kind} runner={runner} selector={selector:?} count={minimum_count}"
        );
    }

    Ok(())
}

/// Every locally expressible collection/combination rule added in P03-R12 is
/// asserted on the exact same input through the committed schema and the actual
/// owning `eqm_domain` constructor.
#[test]
fn local_collection_and_combination_rules_match_owners() -> Result<(), Box<dyn Error>> {
    let v = graph_validator()?;
    let base = graph_document();

    // Journey surfaces: nonempty, exact-unique, ordered (not sorted).
    for value in [
        json!(["account.create.flow.home"]),
        json!(["account.create.flow.home", "account.create.flow.done"]),
        json!([]),
        json!(["account.create.flow.home", "account.create.flow.home"]),
        json!(["account.create.flow"]),
    ] {
        let document = mutated(&base, "/journeys/0/surfaces", value.clone());
        assert_eq!(
            v.is_valid(&document),
            owning_journey_surfaces_ok(&value),
            "journey surfaces {value}"
        );
    }

    // Surface composition: at least one direct requirement or fragment origin.
    for (requirements, fragments) in [
        (json!([requirement()]), json!([fragment_origin()])),
        (json!([requirement()]), json!([])),
        (json!([]), json!([fragment_origin()])),
        (json!([]), json!([])),
    ] {
        let mut document = mutated(&base, "/surfaces/0/requirements", requirements.clone());
        document["surfaces"][0]["fragment_origins"] = fragments.clone();
        assert_eq!(
            v.is_valid(&document),
            owning_surface_composition_ok(&requirements, &fragments)?,
            "surface composition {requirements} {fragments}"
        );
    }

    // Policy selector: at least one populated axis, each axis nonempty/unique.
    for value in [
        json!({"units": ["account.card"]}),
        json!({"facets": ["behavior"]}),
        json!({}),
        json!({"units": []}),
        json!({"units": ["account.card", "account.card"]}),
        json!({"scopes": ["each_target"]}),
    ] {
        let document = mutated(&base, "/policies/0/rules/0/selector", value.clone());
        assert_eq!(
            v.is_valid(&document),
            owning_policy_selector_ok(&value),
            "policy selector {value}"
        );
    }

    // Waiver policy: `allowed` exactly when `maximum_days` is present.
    for value in [
        json!({"allowed": true, "minimum_approvers": 1, "required_controls": ["behavior"], "maximum_days": 30}),
        json!({"allowed": false, "minimum_approvers": 1, "required_controls": ["behavior"]}),
        json!({"allowed": true, "minimum_approvers": 1, "required_controls": []}),
        json!({"allowed": false, "minimum_approvers": 1, "required_controls": [], "maximum_days": 7}),
    ] {
        let document = mutated(&base, "/policies/0/waivers", value.clone());
        assert_eq!(
            v.is_valid(&document),
            owning_waiver_policy_ok(&value),
            "waiver policy {value}"
        );
    }

    // Environment binding: fixed-name sources and literal value presence.
    for value in [
        json!({"name": "PATH", "source": "trusted_path"}),
        json!({"name": "LANG", "source": "canonical_locale"}),
        json!({"name": "TZ", "source": "utc_timezone"}),
        json!({"name": "CUSTOM", "source": "literal", "value": "/x"}),
        json!({"name": "PATH", "source": "canonical_locale"}),
        json!({"name": "LANG", "source": "trusted_path"}),
        json!({"name": "CUSTOM", "source": "trusted_path"}),
        json!({"name": "PATH", "source": "literal"}),
    ] {
        let document = mutated(&base, "/runners/0/environment/0", value.clone());
        assert_eq!(
            v.is_valid(&document),
            owning_environment_binding_ok(&value),
            "environment binding {value}"
        );
    }

    Ok(())
}

#[test]
fn numeric_members_match_owning_width_and_range() -> Result<(), Box<dyn Error>> {
    let v = graph_validator()?;
    let base = graph_document();

    // Small magnitudes use the JSON integer form; values above the exact
    // integer interval use the canonical decimal-string form. Noncanonical
    // small-value strings stay rejected by the real `WireUint` carrier too.
    let values = [
        json!(0),
        json!(1),
        json!(30),
        json!(4_294_967_295u64),
        json!(4_294_967_296u64),
        json!(9_007_199_254_740_991u64),
        json!("9007199254740992"),
        json!("9223372036854775807"),
        json!("9223372036854775808"),
        json!("18446744073709551615"),
        json!("007"),
        json!("9007199254740991"),
    ];

    // Owning `PositiveDays` is `NonZeroU32`: 1..=4,294,967,295.
    parity_value(
        &v,
        &base,
        "/policies/0/waivers/maximum_days",
        &values,
        owning_positive_days,
    );
    // Owning `DurationMillis` is a `u64` bounded by `i64::MAX`.
    parity_value(
        &v,
        &base,
        "/policies/0/rules/0/maximum_age",
        &values,
        owning_duration_millis,
    );
    parity_value(
        &v,
        &base,
        "/bindings/0/evidence/0/freshness",
        &values,
        owning_duration_millis,
    );
    // Owning runner timeout is a positive `DurationMillis`.
    parity_value(
        &v,
        &base,
        "/runners/0/timeout_ms",
        &values,
        owning_positive_duration_millis,
    );
    // Owning `PositiveCount`/`Revision` keep the full positive `u64` range.
    for pointer in [
        "/policies/0/waivers/minimum_approvers",
        "/policies/0/rules/0/minimum_count",
        "/bindings/0/evidence/0/minimum_count",
        "/policies/0/revision",
        "/runners/0/max_output_bytes",
        "/runners/0/max_concurrency",
    ] {
        parity_value(&v, &base, pointer, &values, owning_positive_u64);
    }
    Ok(())
}

#[test]
fn capability_journey_surface_fragment_fields_are_bounded() -> Result<(), Box<dyn Error>> {
    let v = graph_validator()?;
    let base = graph_document();

    check(
        &v,
        &base,
        "/capabilities/0/title",
        &[json!("Account creation")],
        &[json!(""), json!(repeated(b'a', 161))],
    );
    check(
        &v,
        &base,
        "/capabilities/0/status",
        &[
            json!("draft"),
            json!("active"),
            json!("deprecated"),
            json!("retired"),
        ],
        &[json!("Active"), json!("inactive")],
    );
    check(
        &v,
        &base,
        "/capabilities/0/owners",
        &[json!(["owner://team/accounts"])],
        &[
            json!([]),
            json!(["owner://team/accounts", "owner://team/accounts"]),
            json!(["owner://other/team"]),
        ],
    );
    check(
        &v,
        &base,
        "/capabilities/0/description",
        &[json!("ok")],
        &[json!(""), json!(repeated(b'a', 4097))],
    );
    for pointer in [
        "/capabilities/0/id",
        "/capabilities/0/title",
        "/capabilities/0/status",
        "/capabilities/0/owners",
        "/capabilities/0/extensions",
    ] {
        check_required_absent(&v, &base, pointer);
    }

    check(
        &v,
        &base,
        "/journeys/0/transitions/0/trigger",
        &[json!("submit")],
        &[json!(""), json!(repeated(b'a', 257))],
    );
    check(
        &v,
        &base,
        "/journeys/0/transitions/0/from",
        &[json!("account.create.flow.home")],
        &[json!("account.create.flow"), json!("Bad")],
    );

    check(
        &v,
        &base,
        "/journeys/0/revision",
        &[json!(1)],
        &[json!(0), json!(-1), json!("abc"), Value::Null],
    );
    check(&v, &base, "/journeys/0/title", &[json!("ok")], &[json!("")]);
    check(
        &v,
        &base,
        "/journeys/0/status",
        &[json!("active")],
        &[json!("Active")],
    );
    check(
        &v,
        &base,
        "/journeys/0/risk_class",
        &[json!("high")],
        &[json!("severe")],
    );
    check(
        &v,
        &base,
        "/journeys/0/surfaces",
        &[
            json!(["account.create.flow.home"]),
            json!(["account.create.flow.home", "account.create.flow.done"]),
        ],
        &[
            json!([]),
            json!(["account.create.flow"]),
            json!(["account.create.flow.home", "account.create.flow.home"]),
        ],
    );
    check(
        &v,
        &base,
        "/journeys/0/transitions",
        &[
            json!([]),
            json!([{"from": "account.create.flow.home", "to": "account.create.flow.done", "trigger": "submit"}]),
        ],
        &[
            json!([{"from": "account.create.flow.home", "to": "account.create.flow.done", "trigger": "submit"}, {"from": "account.create.flow.home", "to": "account.create.flow.done", "trigger": "submit"}]),
        ],
    );

    // Requirement members, exercised on the surface copy.
    check(
        &v,
        &base,
        "/surfaces/0/requirements/0/level",
        &[json!("optional"), json!("recommended"), json!("required")],
        &[json!("Required")],
    );
    check(
        &v,
        &base,
        "/surfaces/0/requirements/0/scope",
        &[json!("shared_provider")],
        &[json!("each_target"), json!("end_to_end"), json!("target")],
    );
    // The owning Requirement couples `scope` with `provider` presence: a
    // non-shared-provider scope must not carry a provider.
    for scope in ["each_target", "end_to_end"] {
        let mut document = mutated(&base, "/surfaces/0/requirements/0/scope", json!(scope));
        assert!(
            !v.is_valid(&document),
            "scope {scope} with a provider must reject"
        );
        document["surfaces"][0]["requirements"][0]
            .as_object_mut()
            .ok_or("requirement")?
            .remove("provider");
        assert!(
            v.is_valid(&document),
            "scope {scope} without a provider must accept"
        );
    }
    check(
        &v,
        &base,
        "/surfaces/0/requirements/0/statement",
        &[json!("ok")],
        &[json!(""), json!(repeated(b'a', 4097))],
    );
    check(
        &v,
        &base,
        "/surfaces/0/requirements/0/facets",
        &[json!(["behavior"])],
        &[json!([]), json!(["not_a_facet"])],
    );
    check(
        &v,
        &base,
        "/surfaces/0/requirements/0/risk_class",
        &[json!("low"), json!("critical")],
        &[json!("severe")],
    );
    check(
        &v,
        &base,
        "/surfaces/0/requirements/0/provider",
        &[json!("provider.mailgun")],
        &[json!("provider"), json!("Provider.Mailgun")],
    );
    assert!(
        !v.is_valid(&removed(&base, "/surfaces/0/requirements/0/provider")),
        "shared_provider scope requires a provider"
    );
    check_required_absent(&v, &base, "/surfaces/0/requirements/0/statement");
    check_null_rejected(&v, &base, "/surfaces/0/requirements/0/risk_class");

    check(
        &v,
        &base,
        "/surfaces/0/fragment_origins/0/revision",
        &[json!(1)],
        &[json!(0)],
    );
    check(
        &v,
        &base,
        "/surfaces/0/fragment_origins/0/digest",
        &[json!(sha('4'))],
        &[json!("sha256:abc"), json!(sha('4').to_uppercase())],
    );
    // RF66: the optional fragment prefix is the owning single-segment
    // `LocalRequirementId`, not general selector text. The exact-field literals
    // are asserted equal to the owning constructor for the same input.
    let prefix_values: Vec<String> = vec![
        "card".to_owned(),
        "shared".to_owned(),
        repeated(b'a', 63),
        String::new(),
        "card.item".to_owned(),
        "Card".to_owned(),
        "card-item".to_owned(),
        repeated(b'a', 64),
        "1card".to_owned(),
        "card card".to_owned(),
    ];
    parity(
        &v,
        &base,
        "/surfaces/0/fragment_origins/0/prefix",
        &prefix_values,
        |value| LocalRequirementId::new(value).is_ok(),
    );
    check_null_rejected(&v, &base, "/surfaces/0/fragment_origins/0/prefix");
    assert!(
        v.is_valid(&removed(&base, "/surfaces/0/fragment_origins/0/prefix")),
        "absent optional fragment prefix must be accepted"
    );

    check(
        &v,
        &base,
        "/surfaces/0/requirements",
        &[json!([])],
        &[json!([requirement(), requirement()])],
    );
    check(
        &v,
        &base,
        "/fragments/0/requirements",
        &[json!([requirement()])],
        &[json!([])],
    );
    check(
        &v,
        &base,
        "/surfaces/0/description",
        &[json!("ok")],
        &[json!(repeated(b'a', 4097))],
    );
    Ok(())
}

#[test]
fn target_source_provider_fields_are_bounded() -> Result<(), Box<dyn Error>> {
    let v = graph_validator()?;
    let base = graph_document();

    check(
        &v,
        &base,
        "/targets/0/source_components",
        &[json!([]), json!(["shared.kmp"])],
        &[json!(["shared.kmp", "shared.kmp"]), json!(["shared"])],
    );
    check(
        &v,
        &base,
        "/targets/0/depends_on",
        &[json!([]), json!(["web"])],
        &[json!(["web", "web"]), json!(["Web"])],
    );
    check(
        &v,
        &base,
        "/targets/0/providers",
        &[json!([]), json!(["provider.mailgun"])],
        &[
            json!(["provider"]),
            json!(["provider.mailgun", "provider.mailgun"]),
        ],
    );
    check_required_absent(&v, &base, "/targets/0/root");
    check_null_rejected(&v, &base, "/targets/0/role");

    check(
        &v,
        &base,
        "/source_components/0/roots",
        &[json!(["shared/kmp"]), json!(["shared/kmp", "shared/core"])],
        &[
            json!([]),
            json!(["shared/kmp", "shared/kmp"]),
            json!(["../escape"]),
        ],
    );
    check(
        &v,
        &base,
        "/source_components/0/dependencies",
        &[json!([]), json!(["shared.core"])],
        &[json!(["shared.core", "shared.core"]), json!(["shared"])],
    );
    check_required_absent(&v, &base, "/source_components/0/roots");

    check(
        &v,
        &base,
        "/providers/0/configuration_digest",
        &[json!(sha('5'))],
        &[json!("sha256:short"), json!("not-a-digest")],
    );
    check_required_absent(&v, &base, "/providers/0/configuration_digest");
    check(
        &v,
        &base,
        "/providers/0/owners",
        &[json!(["owner://team/providers"])],
        &[json!([])],
    );
    Ok(())
}

#[test]
fn cohort_fields_and_exactly_one_configuration_are_bounded() -> Result<(), Box<dyn Error>> {
    let v = graph_validator()?;
    let base = graph_document();

    check(
        &v,
        &base,
        "/cohorts/0/id",
        &[json!("cohort.n")],
        &[json!(""), json!(repeated(b'a', 513))],
    );
    check(&v, &base, "/cohorts/0/revision", &[json!(1)], &[json!(0)]);
    check(
        &v,
        &base,
        "/cohorts/0/assignments/0/occurrence",
        &[json!("mailer")],
        &[json!("Mail.er"), json!("mail.er"), json!("")],
    );
    check(
        &v,
        &base,
        "/cohorts/0/assignments/0/source_commit",
        &[json!("a".repeat(40)), json!("a".repeat(64))],
        &[json!("a".repeat(39)), json!("A".repeat(40)), json!("xyz")],
    );
    check(
        &v,
        &base,
        "/cohorts/0/assignments/0/source_content_digest",
        &[json!(sha('1'))],
        &[json!("sha256:short")],
    );
    check(
        &v,
        &base,
        "/cohorts/0/assignments/0/build_id",
        &[json!("build.n.mailer")],
        &[json!("")],
    );
    check(
        &v,
        &base,
        "/cohorts/0/assignments/0/artifact_digest",
        &[json!(sha('3'))],
        &[json!("sha256:bad")],
    );
    check(
        &v,
        &base,
        "/cohorts/0/assignments/0/provider_configuration_digest",
        &[json!(sha('2'))],
        &[json!("sha256:bad")],
    );
    check_null_rejected(&v, &base, "/cohorts/0/assignments/0/source_content_digest");

    // Exactly one of the two configuration claims is required.
    let provider_only = cohort_assignment();
    let mut target_only = cohort_assignment();
    if let Some(map) = target_only.as_object_mut() {
        map.remove("provider_configuration_digest");
    }
    target_only["target_configuration_digest"] = json!(sha('9'));
    let mut both = cohort_assignment();
    both["target_configuration_digest"] = json!(sha('9'));
    let mut neither = cohort_assignment();
    if let Some(map) = neither.as_object_mut() {
        map.remove("provider_configuration_digest");
    }
    check(
        &v,
        &base,
        "/cohorts/0/assignments/0",
        &[provider_only, target_only],
        &[both, neither],
    );

    check(
        &v,
        &base,
        "/cohorts/0/assignments",
        &[json!([cohort_assignment()])],
        &[json!([]), json!([cohort_assignment(), cohort_assignment()])],
    );
    for pointer in [
        "/cohorts/0/assignments/0/occurrence",
        "/cohorts/0/assignments/0/repository",
        "/cohorts/0/assignments/0/source_commit",
    ] {
        check_required_absent(&v, &base, pointer);
    }
    Ok(())
}

#[test]
fn participation_binding_artifact_exposure_evidence_fields_are_bounded()
-> Result<(), Box<dyn Error>> {
    let v = graph_validator()?;
    let base = graph_document();

    check(
        &v,
        &base,
        "/participation/0/targets",
        &[json!(["web"]), json!(["web", "mobile"])],
        &[json!([]), json!(["web", "web"]), json!(["Web"])],
    );
    check(
        &v,
        &base,
        "/participation/0/requirement",
        &[json!("account.create#create")],
        &[json!("account#create"), json!("account.create")],
    );

    check(&v, &base, "/bindings/0/revision", &[json!(1)], &[json!(0)]);
    check(
        &v,
        &base,
        "/bindings/0/target",
        &[json!("web")],
        &[json!("Web")],
    );
    check(
        &v,
        &base,
        "/bindings/0/owners",
        &[json!(["owner://team/web"])],
        &[json!([])],
    );
    check(
        &v,
        &base,
        "/bindings/0/artifacts",
        &[json!([artifact()])],
        &[json!([]), json!([artifact(), artifact()])],
    );
    check(
        &v,
        &base,
        "/bindings/0/exposures",
        &[json!([]), json!([exposure()])],
        &[json!([exposure(), exposure()])],
    );
    check(
        &v,
        &base,
        "/bindings/0/evidence",
        &[json!([]), json!([evidence_spec()])],
        &[json!([evidence_spec(), evidence_spec()])],
    );
    check_required_absent(&v, &base, "/bindings/0/artifacts");

    check(
        &v,
        &base,
        "/bindings/0/artifacts/0/role",
        &[
            json!("entrypoint"),
            json!("view"),
            json!("route"),
            json!("component"),
            json!("service"),
            json!("test"),
            json!("configuration"),
            json!("asset"),
        ],
        &[json!("entry"), json!("Entrypoint")],
    );
    check(
        &v,
        &base,
        "/bindings/0/artifacts/0/surface",
        &[json!("account.create.flow.home")],
        &[json!("account.create.flow")],
    );
    check(
        &v,
        &base,
        "/bindings/0/artifacts/0/symbol",
        &[json!("main")],
        &[json!("")],
    );
    check_required_absent(&v, &base, "/bindings/0/artifacts/0/path");

    check(
        &v,
        &base,
        "/bindings/0/exposures/0/state",
        &[json!("required"), json!("prohibited")],
        &[json!("allowed")],
    );
    check(
        &v,
        &base,
        "/bindings/0/exposures/0/surface",
        &[json!("account.create.flow.home")],
        &[json!("Bad")],
    );
    check(
        &v,
        &base,
        "/bindings/0/exposures/0/route",
        &[json!("/home")],
        &[json!("")],
    );

    // The owning evidence kind couples with runner, minimum count and selector
    // variant; the full per-kind matrix is exercised in
    // `local_collection_and_combination_rules_match_owners`.
    check(
        &v,
        &base,
        "/bindings/0/evidence/0/kind",
        &[json!("test")],
        &[json!("test_result")],
    );
    check(
        &v,
        &base,
        "/bindings/0/evidence/0/requirements",
        &[json!(["account.create#create"])],
        &[
            json!([]),
            json!(["account#create"]),
            json!(["account.create#create", "account.create#create"]),
        ],
    );
    check(
        &v,
        &base,
        "/bindings/0/evidence/0/facets",
        &[json!(["behavior"])],
        &[json!([]), json!(["behavior", "behavior"])],
    );
    check(
        &v,
        &base,
        "/bindings/0/evidence/0/runner",
        &[json!("runner.local")],
        &[json!("runner")],
    );
    check(
        &v,
        &base,
        "/bindings/0/evidence/0/minimum_count",
        &[json!(1)],
        &[json!(0), json!(-1)],
    );
    check(
        &v,
        &base,
        "/bindings/0/evidence/0/freshness",
        &[json!(0), json!(3600)],
        &[json!(-1)],
    );
    check_required_absent(&v, &base, "/bindings/0/evidence/0/kind");
    Ok(())
}

#[test]
fn scenario_fields_and_subject_alternatives_are_bounded() -> Result<(), Box<dyn Error>> {
    let v = graph_validator()?;
    let base = graph_document();

    check(
        &v,
        &base,
        "/scenarios/0/participants/0/subject",
        &[
            json!({"kind": "target", "target": "web"}),
            json!({"kind": "provider", "provider": "provider.mailgun"}),
        ],
        &[
            json!({}),
            json!({"kind": "target"}),
            json!({"kind": "provider"}),
            json!({"kind": "target", "provider": "provider.mailgun"}),
            json!({"kind": "provider", "target": "web"}),
            json!({"kind": "target", "target": "web", "provider": "provider.mailgun"}),
            json!({"kind": "target", "target": "Web"}),
            json!({"kind": "provider", "provider": "provider"}),
        ],
    );
    check(
        &v,
        &base,
        "/scenarios/0/participants/0/role",
        &[json!("edge"), json!("control_tower")],
        &[json!("Edge"), json!("")],
    );
    check(&v, &base, "/scenarios/0/revision", &[json!(1)], &[json!(0)]);
    check(
        &v,
        &base,
        "/scenarios/0/participants",
        &[
            json!([{"id": "mailer", "role": "provider", "subject": {"kind": "provider", "provider": "provider.mailgun"}}]),
        ],
        &[
            json!([]),
            json!([
                {"id": "mailer", "role": "provider", "subject": {"kind": "provider", "provider": "provider.mailgun"}},
                {"id": "mailer", "role": "provider", "subject": {"kind": "provider", "provider": "provider.mailgun"}}
            ]),
        ],
    );
    check(
        &v,
        &base,
        "/scenarios/0/dependencies/0/from",
        &[json!("mailer")],
        &[json!("Mail.er"), json!("")],
    );
    check(
        &v,
        &base,
        "/scenarios/0/dependencies",
        &[json!([]), json!([{"from": "mailer", "to": "mailer"}])],
        &[json!([
            {"from": "mailer", "to": "mailer"},
            {"from": "mailer", "to": "mailer"}
        ])],
    );
    check_required_absent(&v, &base, "/scenarios/0/participants");
    Ok(())
}

#[test]
fn policy_and_profile_fields_are_bounded() -> Result<(), Box<dyn Error>> {
    let v = graph_validator()?;
    let base = graph_document();

    check(
        &v,
        &base,
        "/policies/0/profiles",
        &[json!(["audience.default"])],
        &[json!([]), json!(["audience"])],
    );
    check(
        &v,
        &base,
        "/policies/0/required_targets",
        &[json!(["web"])],
        &[json!([]), json!(["web", "web"])],
    );
    check(
        &v,
        &base,
        "/policies/0/rules",
        &[json!([{
            "selector": {"units": ["account.card"]},
            "minimum_level": "required",
            "facets": ["behavior"],
            "minimum_trust": "trusted_ci",
            "maximum_age": 0,
            "minimum_count": 1
        }])],
        &[json!([])],
    );
    check(
        &v,
        &base,
        "/policies/0/rules/0/minimum_level",
        &[json!("optional"), json!("recommended"), json!("required")],
        &[json!("Required")],
    );
    check(
        &v,
        &base,
        "/policies/0/rules/0/facets",
        &[json!(["behavior"])],
        &[json!([]), json!(["behavior", "behavior"])],
    );
    check(
        &v,
        &base,
        "/policies/0/rules/0/minimum_trust",
        &[
            json!("untrusted_local"),
            json!("trusted_ci"),
            json!("signed_ci"),
        ],
        &[json!("trusted")],
    );
    check(
        &v,
        &base,
        "/policies/0/rules/0/maximum_age",
        &[json!(0), json!(86400)],
        &[json!(-1)],
    );
    check(
        &v,
        &base,
        "/policies/0/rules/0/minimum_count",
        &[json!(1)],
        &[json!(0)],
    );
    check(
        &v,
        &base,
        "/policies/0/rules/0/selector/units",
        &[json!(["account.card"])],
        &[json!([]), json!(["account"])],
    );
    check(
        &v,
        &base,
        "/policies/0/rules/0/selector",
        &[
            json!({"units": ["account.card"]}),
            json!({"requirements": ["account.create#create"]}),
            json!({"risk_classes": ["high"]}),
            json!({"facets": ["behavior"]}),
            json!({"scopes": ["each_target"]}),
            json!({"units": ["account.card"], "facets": ["behavior"]}),
        ],
        &[json!({}), json!({"units": []})],
    );

    check(
        &v,
        &base,
        "/policies/0/waivers/minimum_approvers",
        &[json!(1)],
        &[json!(0)],
    );
    check(
        &v,
        &base,
        "/policies/0/waivers/required_controls",
        &[json!([]), json!(["behavior"])],
        &[json!(["not_a_facet"])],
    );
    check(
        &v,
        &base,
        "/policies/0/waivers/maximum_days",
        &[json!(30)],
        &[json!(0)],
    );
    check(
        &v,
        &base,
        "/policies/0/waivers/allowed",
        &[json!(true)],
        &[json!("true")],
    );
    // The owning WaiverPolicy couples `allowed` with `maximum_days` presence.
    let mut allowed_without_days = base.clone();
    allowed_without_days["policies"][0]["waivers"]
        .as_object_mut()
        .ok_or("waivers")?
        .remove("maximum_days");
    assert!(
        !v.is_valid(&allowed_without_days),
        "allowed waiver without maximum_days"
    );
    let mut denied_with_days = base.clone();
    denied_with_days["policies"][0]["waivers"]["allowed"] = json!(false);
    assert!(
        !v.is_valid(&denied_with_days),
        "denied waiver with maximum_days"
    );
    let mut denied = denied_with_days.clone();
    denied["policies"][0]["waivers"]
        .as_object_mut()
        .ok_or("waivers")?
        .remove("maximum_days");
    assert!(v.is_valid(&denied), "denied waiver without maximum_days");
    check_required_absent(&v, &base, "/policies/0/waivers");

    check(
        &v,
        &base,
        "/profiles/0/dimensions",
        &[json!([{"id": "locale", "values": ["en"]}])],
        &[
            json!([]),
            json!([{"id": "locale", "values": ["en"]}, {"id": "locale", "values": ["en"]}]),
        ],
    );
    check(
        &v,
        &base,
        "/profiles/0/dimensions/0/values",
        &[json!(["en", "fr"])],
        &[json!([]), json!(["en", "en"]), json!(["Bad Value"])],
    );
    check(
        &v,
        &base,
        "/profiles/0/defaults",
        &[json!({}), json!({"locale": "en"})],
        &[json!({"Bad": "en"}), json!({"locale": "Bad Value"})],
    );
    check(&v, &base, "/profiles/0/revision", &[json!(1)], &[json!(0)]);
    check_required_absent(&v, &base, "/profiles/0/dimensions");
    Ok(())
}

#[test]
fn runner_fields_and_variants_are_bounded() -> Result<(), Box<dyn Error>> {
    let v = graph_validator()?;
    let base = graph_document();

    check(
        &v,
        &base,
        "/runners/0/backend",
        &[json!("container")],
        &[json!("remote")],
    );
    // The owning RunnerDefinition forbids claimed guarantees for a local backend.
    let mut local_runner = base.clone();
    local_runner["runners"][0]["backend"] = json!("local");
    assert!(
        !v.is_valid(&local_runner),
        "local backend with claimed guarantees"
    );
    local_runner["runners"][0]["guarantees"] = json!([]);
    assert!(
        v.is_valid(&local_runner),
        "local backend without guarantees"
    );
    check(
        &v,
        &base,
        "/runners/0/program",
        &[
            json!({"kind": "repository", "path": "tools/runner"}),
            json!({"kind": "locked", "resolved": "https://example.com/team/repo", "digest": sha('8')}),
        ],
        &[
            json!({"kind": "remote"}),
            json!({"kind": "repository", "path": "../escape"}),
            json!({"kind": "locked", "resolved": "https://example.com/team/repo"}),
        ],
    );
    check(
        &v,
        &base,
        "/runners/0/args",
        &[
            json!([]),
            json!([{"kind": "literal", "value": "--json"}]),
            json!([{"kind": "target_root"}]),
            json!([{"kind": "selector_json"}]),
            json!([{"kind": "result_path"}]),
        ],
        &[json!([{"kind": "literal"}]), json!([{"kind": "other"}])],
    );
    check(
        &v,
        &base,
        "/runners/0/cwd",
        &[
            json!({"kind": "target_root"}),
            json!({"kind": "repository", "path": "tools/runner"}),
            json!({"kind": "result_path"}),
        ],
        &[
            json!({"kind": "other"}),
            json!({"kind": "repository", "path": "../escape"}),
        ],
    );
    check(
        &v,
        &base,
        "/runners/0/environment/0/name",
        &[json!("PATH"), json!("A_B")],
        &[json!("path"), json!("1BAD"), json!("")],
    );
    // The owning EnvironmentBinding couples `source` with `value` presence and
    // with the fixed-name sources; the literal source accepts any name.
    assert!(
        v.is_valid(&mutated(
            &base,
            "/runners/0/environment/0",
            json!({"name": "PATH", "source": "trusted_path"}),
        )),
        "PATH/trusted_path"
    );
    assert!(
        v.is_valid(&mutated(
            &base,
            "/runners/0/environment/0",
            json!({"name": "LANG", "source": "canonical_locale"}),
        )),
        "LANG/canonical_locale"
    );
    assert!(
        v.is_valid(&mutated(
            &base,
            "/runners/0/environment/0",
            json!({"name": "TZ", "source": "utc_timezone"}),
        )),
        "TZ/utc_timezone"
    );
    assert!(
        v.is_valid(&mutated(
            &base,
            "/runners/0/environment/0",
            json!({"name": "CUSTOM", "source": "literal", "value": "/x"}),
        )),
        "literal accepts any name"
    );
    for bad in [
        json!({"name": "PATH", "source": "canonical_locale"}),
        json!({"name": "LANG", "source": "trusted_path"}),
        json!({"name": "TZ", "source": "trusted_path"}),
        json!({"name": "CUSTOM", "source": "utc_timezone"}),
        json!({"name": "PATH", "source": "trusted_path", "value": "/x"}),
        json!({"name": "PATH", "source": "literal"}),
    ] {
        assert!(
            !v.is_valid(&mutated(&base, "/runners/0/environment/0", bad.clone())),
            "environment binding accepted {bad}"
        );
    }
    check(
        &v,
        &base,
        "/runners/0/environment/0/value",
        &[json!("/usr/bin")],
        &[json!(""), json!(null)],
    );
    assert!(
        !v.is_valid(&removed(&base, "/runners/0/environment/0/value")),
        "literal source without value"
    );
    check(
        &v,
        &base,
        "/runners/0/secrets/0/name",
        &[json!("TOKEN")],
        &[json!("token")],
    );
    check(
        &v,
        &base,
        "/runners/0/secrets/0/provider",
        &[json!("secret://vault/token")],
        &[json!("vault/token"), json!("secret://Vault/token")],
    );
    check(&v, &base, "/runners/0/timeout_ms", &[json!(1)], &[json!(0)]);
    check(
        &v,
        &base,
        "/runners/0/max_output_bytes",
        &[json!(1)],
        &[json!(0)],
    );
    check(
        &v,
        &base,
        "/runners/0/max_concurrency",
        &[json!(1)],
        &[json!(0)],
    );
    check(
        &v,
        &base,
        "/runners/0/guarantees",
        &[
            json!([]),
            json!(["network_denied"]),
            json!(["read_only_source"]),
            json!(["isolated_process"]),
            json!(["resource_limited"]),
        ],
        &[
            json!(["unlimited"]),
            json!(["network_denied", "network_denied"]),
        ],
    );
    check_required_absent(&v, &base, "/runners/0/program");
    Ok(())
}

#[test]
fn waiver_scope_and_waiver_fields_are_bounded() -> Result<(), Box<dyn Error>> {
    let v = graph_validator()?;
    let base = graph_document();

    check(
        &v,
        &base,
        "/waivers/0/scope/target",
        &[
            json!({"kind": "target", "target": "web"}),
            json!({"kind": "provider", "provider": "provider.mailgun"}),
            json!({"kind": "target_set", "targets": ["web", "mobile"]}),
        ],
        &[
            json!({"kind": "target_set", "targets": []}),
            json!({"kind": "target_set"}),
            json!({"kind": "target", "target": "Web"}),
            json!({"kind": "provider", "provider": "provider"}),
        ],
    );
    check(
        &v,
        &base,
        "/waivers/0/scope/profiles",
        &[
            json!({"audience.default": {"locale": "en"}}),
            json!({"audience.default": {"locale": "en", "region": "eu"}}),
        ],
        &[
            json!({}),
            json!({"audience.default": {}}),
            json!({"audience": {"locale": "en"}}),
            json!({"audience.default": {"Bad": "en"}}),
        ],
    );
    check(
        &v,
        &base,
        "/waivers/0/scope/facets",
        &[json!(["behavior"])],
        &[json!([])],
    );
    check(
        &v,
        &base,
        "/waivers/0/reason",
        &[json!("ok")],
        &[json!(""), json!(repeated(b'a', 2049))],
    );
    check(
        &v,
        &base,
        "/waivers/0/issue",
        &[
            json!("issue://ABC-1"),
            json!("issue://AB-1"),
            json!("issue://A1-1"),
        ],
        &[
            json!("issue://abc-1"),
            json!("ABC-1"),
            json!("issue://ABC-0"),
        ],
    );
    check(
        &v,
        &base,
        "/waivers/0/starts_on",
        &[json!("2026-01-01")],
        &[json!("2026-1-1"), json!("2026-13-01")],
    );
    check(
        &v,
        &base,
        "/waivers/0/expires_on",
        &[json!("2026-02-01")],
        &[json!("nope")],
    );
    check(
        &v,
        &base,
        "/waivers/0/controls",
        &[json!([]), json!(["behavior"])],
        &[json!(["not_a_facet"])],
    );
    check(
        &v,
        &base,
        "/waivers/0/approvers",
        &[json!(["owner://team/security"])],
        &[json!([])],
    );
    check(
        &v,
        &base,
        "/waivers/0/policy",
        &[json!("policy.default")],
        &[json!("policy")],
    );
    check(&v, &base, "/waivers/0/revision", &[json!(1)], &[json!(0)]);
    check_required_absent(&v, &base, "/waivers/0/scope");
    Ok(())
}

#[test]
fn lock_fields_are_bounded() -> Result<(), Box<dyn Error>> {
    let v = graph_validator()?;
    let base = graph_document();

    check(&v, &base, "/imports/0/revision", &[json!(1)], &[json!(0)]);
    check(
        &v,
        &base,
        "/imports/0/source",
        &[json!("https://example.com/team/repo")],
        &[
            json!("https://Example.com/team/repo"),
            json!("https://example.com/team/repo/"),
        ],
    );
    check(
        &v,
        &base,
        "/imports/0/resolved",
        &[json!("a".repeat(40)), json!("a".repeat(64))],
        &[json!("a".repeat(39)), json!("xyz")],
    );
    check(
        &v,
        &base,
        "/imports/0/digest",
        &[json!(sha('6'))],
        &[json!("sha256:short")],
    );
    check(
        &v,
        &base,
        "/imports/0/trust",
        &[
            json!("untrusted_local"),
            json!("trusted_ci"),
            json!("signed_ci"),
        ],
        &[json!("trusted")],
    );
    check(
        &v,
        &base,
        "/imports/0/signature",
        &[json!("sig")],
        &[json!("")],
    );
    check(&v, &base, "/adapters/0/protocol", &[json!(1)], &[json!(0)]);
    check(
        &v,
        &base,
        "/adapters/0/version",
        &[json!("1.0.0")],
        &[json!("")],
    );
    check_required_absent(&v, &base, "/imports/0/digest");
    Ok(())
}

#[test]
fn comparison_context_and_graph_root_fields_are_bounded() -> Result<(), Box<dyn Error>> {
    let graph = graph_validator()?;
    let base = graph_document();

    check(
        &graph,
        &base,
        "/schema",
        &[json!(eqm_protocol::SEMANTIC_GRAPH_SCHEMA_V2.to_string())],
        &[json!("x")],
    );
    check(
        &graph,
        &base,
        "/capabilities",
        &[json!([])],
        &[json!("x"), json!([{}])],
    );
    check_required_absent(&graph, &base, "/providers");
    let mut unknown = graph_document();
    if let Some(map) = unknown.as_object_mut() {
        map.insert("unknown".to_owned(), json!(1));
    }
    assert!(!graph.is_valid(&unknown), "unknown root member accepted");

    let comparison = comparison_validator()?;
    let cbase = comparison_document();
    check(
        &comparison,
        &cbase,
        "/schema",
        &[json!(
            eqm_protocol::COMPARISON_CONTEXT_SCHEMA_V2.to_string()
        )],
        &[json!("x")],
    );
    check(
        &comparison,
        &cbase,
        "/contract_digest",
        &[json!(sha('1'))],
        &[json!("sha256:short")],
    );
    check(
        &comparison,
        &cbase,
        "/scenario_digest",
        &[json!(sha('2'))],
        &[json!("not-a-digest")],
    );
    check(
        &comparison,
        &cbase,
        "/comparison_rule_digest",
        &[json!(sha('3'))],
        &[json!("sha256:bad")],
    );
    check(
        &comparison,
        &cbase,
        "/profiles",
        &[json!([selected_profile()])],
        &[json!([]), json!([selected_profile(), selected_profile()])],
    );
    check(
        &comparison,
        &cbase,
        "/profiles/0/values",
        &[
            json!([{"dimension": "locale", "state": "known", "value": "en"}]),
            json!([{"dimension": "locale", "state": "unknown"}]),
        ],
        &[
            json!([]),
            json!([{"dimension": "locale", "state": "known"}]),
            json!([{"dimension": "locale", "state": "maybe"}]),
        ],
    );
    check(
        &comparison,
        &cbase,
        "/profiles/0/values/0/dimension",
        &[json!("locale")],
        &[json!("Locale"), json!("a.b")],
    );
    check(
        &comparison,
        &cbase,
        "/profiles/0/definition/revision",
        &[json!(1)],
        &[json!(0)],
    );
    check(
        &comparison,
        &cbase,
        "/profiles/0/definition/defaults",
        &[json!({"locale": "en"})],
        &[json!({"Bad": "en"})],
    );
    Ok(())
}

fn workspace_document() -> Value {
    json!({
        "schema": "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/workspace.schema.json",
        "contract_sources": [],
        "binding_sources": [],
        "policy_sources": [],
        "profile_sources": [],
        "runner_sources": [],
        "waiver_sources": [],
        "targets": {
            "web": {
                "root": "apps/web",
                "platform": "web",
                "framework": "sveltekit",
                "role": "edge",
                "source_components": ["shared.kmp"],
                "depends_on": ["api"],
                "providers": ["provider.mailgun"],
                "owners": ["owner://team/web"],
                "extensions": {}
            }
        },
        "source_components": {
            "shared.kmp": {
                "roots": ["shared/kmp"],
                "owners": ["owner://team/shared"],
                "dependencies": ["shared.core"],
                "extensions": {}
            }
        },
        "extensions": {}
    })
}

fn scenario_document() -> Value {
    json!({
        "schema": "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/scenario.schema.json",
        "id": "signup.flow",
        "revision": 1,
        "participants": [{
            "id": "mailer",
            "role": "provider",
            "subject": {"provider": "provider.mailgun"}
        }],
        "dependencies": [],
        "extensions": {}
    })
}

#[test]
fn authored_workspace_and_scenario_fields_are_bounded() -> Result<(), Box<dyn Error>> {
    let workspace = validator("schemas/v2/manifest/workspace.schema.json")?;
    let wbase = workspace_document();
    assert!(workspace.is_valid(&wbase), "authored workspace base");

    let paths = vec![
        "apps/web".to_owned(),
        "apps/web/nested".to_owned(),
        "../escape".to_owned(),
        "/abs".to_owned(),
        "CON".to_owned(),
        "a ".to_owned(),
        "a//b".to_owned(),
        String::new(),
    ];
    parity(&workspace, &wbase, "/targets/web/root", &paths, |value| {
        RepoPath::new(value).is_ok()
    });
    parity(
        &workspace,
        &wbase,
        "/source_components/shared.kmp/roots/0",
        &paths,
        |value| RepoPath::new(value).is_ok(),
    );
    parity(
        &workspace,
        &wbase,
        "/targets/web/platform",
        &paths,
        |value| PlatformId::new(value).is_ok(),
    );
    parity(
        &workspace,
        &wbase,
        "/targets/web/framework",
        &paths,
        |value| eqm_domain::FrameworkId::new(value).is_ok(),
    );
    parity(&workspace, &wbase, "/targets/web/role", &paths, |value| {
        RoleId::new(value).is_ok()
    });

    check(
        &workspace,
        &wbase,
        "/targets/web/source_components",
        &[json!([]), json!(["shared.kmp"])],
        &[json!(["shared.kmp", "shared.kmp"]), json!(["shared"])],
    );
    check(
        &workspace,
        &wbase,
        "/targets/web/depends_on",
        &[json!([]), json!(["web"])],
        &[json!(["web", "web"]), json!(["Web"])],
    );
    check(
        &workspace,
        &wbase,
        "/targets/web/providers",
        &[json!([]), json!(["provider.mailgun"])],
        &[
            json!(["provider"]),
            json!(["provider.mailgun", "provider.mailgun"]),
        ],
    );
    check(
        &workspace,
        &wbase,
        "/source_components/shared.kmp/roots",
        &[json!(["shared/kmp"])],
        &[json!([]), json!(["shared/kmp", "shared/kmp"])],
    );
    check_required_absent(&workspace, &wbase, "/targets/web/root");
    check_null_rejected(&workspace, &wbase, "/targets/web/role");

    let scenario = validator("schemas/v2/manifest/scenario.schema.json")?;
    let sbase = scenario_document();
    assert!(scenario.is_valid(&sbase), "authored scenario base");
    check(
        &scenario,
        &sbase,
        "/participants/0/subject",
        &[
            json!({"target": "web"}),
            json!({"provider": "provider.mailgun"}),
        ],
        &[
            json!({}),
            json!({"target": "Web"}),
            json!({"provider": "provider"}),
            json!({"target": "web", "provider": "provider.mailgun"}),
        ],
    );
    check(
        &scenario,
        &sbase,
        "/participants/0/role",
        &[json!("edge")],
        &[json!("Edge")],
    );
    check(&scenario, &sbase, "/revision", &[json!(1)], &[json!(0)]);
    check(
        &scenario,
        &sbase,
        "/participants",
        &[
            json!([{"id": "mailer", "role": "provider", "subject": {"provider": "provider.mailgun"}}]),
        ],
        &[
            json!([]),
            json!([
                {"id": "mailer", "role": "provider", "subject": {"provider": "provider.mailgun"}},
                {"id": "mailer", "role": "provider", "subject": {"provider": "provider.mailgun"}}
            ]),
        ],
    );
    check(
        &scenario,
        &sbase,
        "/dependencies",
        &[json!([]), json!([{"from": "mailer", "to": "mailer"}])],
        &[json!([
            {"from": "mailer", "to": "mailer"},
            {"from": "mailer", "to": "mailer"}
        ])],
    );
    check_required_absent(&scenario, &sbase, "/participants");
    Ok(())
}
