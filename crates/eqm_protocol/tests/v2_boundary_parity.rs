//! P03-R7: independent cross-layer v2 schema boundary corpus.
//!
//! Every P03-R6 audit row is exercised as valid/invalid boundary input through
//! the committed local Draft 2020-12 schemas with format assertions and, where
//! one exists, through the actual owning domain constructor, authored loader or
//! checked protocol reader. The tables assert parity only where the owning rule
//! is ASCII-lexical; Unicode byte/NFC/depth/node and raw-token layers are called
//! out explicitly and are never claimed equivalent to JSON Schema.

use eqm_domain::{
    DimensionId, ExtensionKey, ExtensionNamespace, FullRequirementId, OccurrenceId, ProfileId,
    ProviderId, RepoPath, RepositoryIdentity, RoleId, ScenarioId, SymbolicValueId, TargetId,
};
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

fn digest(byte: char) -> String {
    format!("sha256:{}", byte.to_string().repeat(64))
}

fn repeated(byte: u8, length: usize) -> String {
    (byte as char).to_string().repeat(length)
}

fn seg63() -> String {
    repeated(b'a', 63)
}

fn seg64() -> String {
    repeated(b'a', 64)
}

/// An exact 255-byte dotted ID whose every segment is at most 63 bytes.
fn dotted_255() -> String {
    format!(
        "{}.{}.{}.{}",
        repeated(b'a', 63),
        repeated(b'b', 63),
        repeated(b'c', 63),
        repeated(b'd', 63)
    )
}

/// An exact 256-byte dotted ID whose every segment is at most 63 bytes.
fn dotted_256() -> String {
    format!(
        "{}.{}.{}.{}.{}",
        repeated(b'a', 63),
        repeated(b'b', 63),
        repeated(b'c', 63),
        repeated(b'd', 62),
        repeated(b'e', 1)
    )
}

fn object_with(key: &str, value: Value) -> Value {
    let mut map = serde_json::Map::new();
    map.insert(key.to_owned(), value);
    Value::Object(map)
}

fn provider_document() -> Value {
    json!({
        "schema": "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/provider.schema.json",
        "id": "provider.mailgun",
        "configuration_digest": digest('1'),
        "owners": ["owner://team/providers"],
        "extensions": {}
    })
}

fn cohort_document() -> Value {
    json!({
        "schema": "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/cohort.schema.json",
        "id": "cohort.n",
        "revision": 1,
        "scenario": "signup.flow",
        "assignments": [{
            "occurrence": "mailer",
            "repository": "https://example.com/team/project",
            "source_commit": "a".repeat(40),
            "provider_configuration_digest": digest('4')
        }],
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

fn participation_document() -> Value {
    json!({
        "schema": "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/participation.schema.json",
        "id": "participation.mobile",
        "requirement": "account.create#create",
        "targets": ["web"],
        "applicability": {"always": true},
        "extensions": {}
    })
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
                "owners": ["owner://team/web"],
                "extensions": {}
            }
        },
        "source_components": {
            "shared.kmp": {
                "roots": ["shared/kmp"],
                "owners": ["owner://team/shared"],
                "extensions": {}
            }
        },
        "extensions": {}
    })
}

fn graph_document() -> Value {
    json!({
        "schema": eqm_protocol::SEMANTIC_GRAPH_SCHEMA_V2.to_string(),
        "capabilities": [],
        "journeys": [],
        "surfaces": [],
        "fragments": [],
        "targets": [{
            "id": "web",
            "root": "apps/web",
            "platform": "web",
            "framework": "sveltekit",
            "owners": ["owner://team/web"],
            "extensions": {}
        }],
        "source_components": [{
            "id": "shared.kmp",
            "roots": ["shared/kmp"],
            "owners": ["owner://team/shared"],
            "dependencies": [],
            "extensions": {}
        }],
        "providers": [{
            "id": "provider.mailgun",
            "configuration_digest": digest('1'),
            "owners": ["owner://team/providers"],
            "extensions": {}
        }],
        "cohorts": [{
            "id": "cohort.n",
            "revision": 1,
            "scenario": "signup.flow",
            "assignments": [{
                "occurrence": "mailer",
                "repository": "https://example.com/team/project",
                "source_commit": "a".repeat(40),
                "provider_configuration_digest": digest('4')
            }],
            "extensions": {}
        }],
        "participation": [{
            "id": "participation.mobile",
            "requirement": "account.create#create",
            "targets": ["web"],
            "applicability": {"always": true},
            "extensions": {}
        }],
        "bindings": [],
        "scenarios": [{
            "id": "signup.flow",
            "revision": 1,
            "participants": [{
                "id": "mailer",
                "role": "provider",
                "subject": {"kind": "provider", "provider": "provider.mailgun"}
            }],
            "dependencies": [],
            "extensions": {}
        }],
        "policies": [],
        "profiles": [],
        "runners": [],
        "waivers": [],
        "imports": [],
        "adapters": [],
        "extensions": {}
    })
}

fn comparison_document() -> Value {
    json!({
        "schema": eqm_protocol::COMPARISON_CONTEXT_SCHEMA_V2.to_string(),
        "contract_digest": digest('1'),
        "scenario_digest": digest('2'),
        "profiles": [{
            "definition": {
                "id": "audience.default",
                "revision": 1,
                "title": "Audience",
                "owners": ["owner://team/product"],
                "dimensions": [{"id": "locale", "values": ["en", "fr"]}],
                "defaults": {"locale": "en"},
                "extensions": {}
            },
            "values": [{"dimension": "locale", "state": "known", "value": "en"}]
        }],
        "comparison_rule_digest": digest('3')
    })
}

/// Asserts schema validity equals the owning ASCII-lexical acceptance for each
/// boundary value at one ingress.
fn parity(
    validator: &Validator,
    base: &Value,
    mutate: impl Fn(&mut Value, &str),
    owner: impl Fn(&str) -> bool,
    values: &[&str],
) {
    for value in values {
        let mut document = base.clone();
        mutate(&mut document, value);
        let schema_ok = validator.is_valid(&document);
        let owner_ok = owner(value);
        assert_eq!(
            schema_ok, owner_ok,
            "value={value:?} schema={schema_ok} owner={owner_ok}"
        );
    }
}

#[test]
fn dotted_identifier_bounds_match_owner() -> Result<(), Box<dyn Error>> {
    let validator = validator("schemas/v2/protocol/comparison-context.schema.json")?;
    let values = [
        "audience.default".to_owned(),
        format!("{}.x", seg63()),
        format!("{}.x", seg64()),
        dotted_255(),
        dotted_256(),
        "audience".to_owned(),
        "Audience.Default".to_owned(),
        "audience..default".to_owned(),
    ];
    let refs: Vec<&str> = values.iter().map(String::as_str).collect();
    parity(
        &validator,
        &comparison_document(),
        |document, value| document["profiles"][0]["definition"]["id"] = json!(value),
        |value| ProfileId::new(value).is_ok(),
        &refs,
    );
    Ok(())
}

#[test]
fn provider_and_scenario_identifier_bounds_match_owner() -> Result<(), Box<dyn Error>> {
    let values = [
        "provider.mailgun".to_owned(),
        "provider".to_owned(),
        format!("{}.x", seg63()),
        format!("{}.x", seg64()),
        dotted_255(),
        dotted_256(),
    ];
    let refs: Vec<&str> = values.iter().map(String::as_str).collect();

    let provider = validator("schemas/v2/manifest/provider.schema.json")?;
    parity(
        &provider,
        &provider_document(),
        |document, value| document["id"] = json!(value),
        |value| ProviderId::new(value).is_ok(),
        &refs,
    );

    let cohort = validator("schemas/v2/manifest/cohort.schema.json")?;
    parity(
        &cohort,
        &cohort_document(),
        |document, value| document["scenario"] = json!(value),
        |value| ScenarioId::new(value).is_ok(),
        &refs,
    );
    Ok(())
}

#[test]
fn extensible_identifier_bounds_match_owner() -> Result<(), Box<dyn Error>> {
    let graph = validator("schemas/v2/protocol/semantic-graph.schema.json")?;
    let values = [
        "web".to_owned(),
        "web.mobile".to_owned(),
        format!("{}.x", seg63()),
        format!("{}.x", seg64()),
        dotted_255(),
        dotted_256(),
        "Web".to_owned(),
        "web..mobile".to_owned(),
    ];
    let refs: Vec<&str> = values.iter().map(String::as_str).collect();
    parity(
        &graph,
        &graph_document(),
        |document, value| document["targets"][0]["id"] = json!(value),
        |value| TargetId::new(value).is_ok(),
        &refs,
    );
    parity(
        &graph,
        &graph_document(),
        |document, value| document["targets"][0]["role"] = json!(value),
        |value| RoleId::new(value).is_ok(),
        &refs,
    );
    Ok(())
}

#[test]
fn single_segment_identifier_bounds_match_owner() -> Result<(), Box<dyn Error>> {
    let values = [
        "mailer".to_owned(),
        "a_b".to_owned(),
        seg63(),
        seg64(),
        "Mailer".to_owned(),
        "mail.er".to_owned(),
    ];
    let refs: Vec<&str> = values.iter().map(String::as_str).collect();

    let cohort = validator("schemas/v2/manifest/cohort.schema.json")?;
    parity(
        &cohort,
        &cohort_document(),
        |document, value| document["assignments"][0]["occurrence"] = json!(value),
        |value| OccurrenceId::new(value).is_ok(),
        &refs,
    );

    let comparison = validator("schemas/v2/protocol/comparison-context.schema.json")?;
    parity(
        &comparison,
        &comparison_document(),
        |document, value| {
            document["profiles"][0]["definition"]["dimensions"][0]["id"] = json!(value)
        },
        |value| DimensionId::new(value).is_ok(),
        &refs,
    );
    parity(
        &comparison,
        &comparison_document(),
        |document, value| {
            document["profiles"][0]["definition"]["dimensions"][0]["values"] =
                json!([value, "other_value"]);
            document["profiles"][0]["values"] =
                json!([{"dimension": "locale", "state": "known", "value": value}]);
        },
        |value| SymbolicValueId::new(value).is_ok(),
        &refs,
    );
    Ok(())
}

#[test]
fn requirement_bounds_match_owner() -> Result<(), Box<dyn Error>> {
    let validator = validator("schemas/v2/manifest/participation.schema.json")?;
    let values = [
        "account.create#create".to_owned(),
        format!("{}#x", dotted_255()),
        format!("{}#x", dotted_256()),
        "account#create".to_owned(),
        "account.create#create.value".to_owned(),
        "Account.create#create".to_owned(),
    ];
    let refs: Vec<&str> = values.iter().map(String::as_str).collect();
    parity(
        &validator,
        &participation_document(),
        |document, value| document["requirement"] = json!(value),
        |value| FullRequirementId::new(value).is_ok(),
        &refs,
    );
    Ok(())
}

#[test]
fn repository_schema_matches_the_owning_parser() -> Result<(), Box<dyn Error>> {
    let validator = validator("schemas/v2/manifest/cohort.schema.json")?;
    let values = [
        "https://example.com/team/repo",
        "https://example.com:443/team/repo",
        "https://example.com/a",
        "https://example.com//team",
        "https://example_com/team",
        "https://example.com/team/repo?x=1",
        "https://example.com/team/repo#fragment",
        "https://example.com/team/repo@rev",
        "https://example.com/team/repo/",
        "https://example.com",
        "https://example.com/",
        "http://example.com/team/repo",
        "https://Example.com/team/repo",
        "https:///team/repo",
        "https://example.com/team repo",
    ];
    parity(
        &validator,
        &cohort_document(),
        |document, value| document["assignments"][0]["repository"] = json!(value),
        |value| RepositoryIdentity::from_str(value).is_ok(),
        &values,
    );
    Ok(())
}

#[test]
fn extension_namespace_and_recursive_keys_match_owner() -> Result<(), Box<dyn Error>> {
    let provider = validator("schemas/v2/manifest/provider.schema.json")?;
    let namespaces = [
        "dev.example.ext".to_owned(),
        "dev.example.ext.deep".to_owned(),
        format!("dev.{}.ext", seg63()),
        "dev.example".to_owned(),
        format!("dev.{}.ext", seg64()),
        "Dev.example.ext".to_owned(),
        "dev..ext".to_owned(),
    ];
    for namespace in &namespaces {
        let mut document = provider_document();
        document["extensions"] = object_with(namespace, object_with("key", json!(true)));
        let schema_ok = provider.is_valid(&document);
        let owner_ok = ExtensionNamespace::new(namespace.as_str()).is_ok();
        assert_eq!(
            schema_ok, owner_ok,
            "namespace={namespace:?} schema={schema_ok} owner={owner_ok}"
        );
    }

    let keys = [
        "key".to_owned(),
        "a_b".to_owned(),
        seg63(),
        seg64(),
        "Key".to_owned(),
        "key.sub".to_owned(),
    ];
    for key in &keys {
        let mut document = provider_document();
        document["extensions"] = object_with("dev.example.ext", object_with(key, json!(true)));
        let schema_ok = provider.is_valid(&document);
        let owner_ok = ExtensionKey::new(key.as_str()).is_ok();
        assert_eq!(
            schema_ok, owner_ok,
            "key={key:?} schema={schema_ok} owner={owner_ok}"
        );
    }
    Ok(())
}

#[test]
fn protocol_extension_namespace_and_recursive_keys_match_owner() -> Result<(), Box<dyn Error>> {
    let comparison = validator("schemas/v2/protocol/comparison-context.schema.json")?;
    let namespaces = [
        "dev.example.ext".to_owned(),
        "dev.example.ext.deep".to_owned(),
        "dev.example".to_owned(),
        "dev..ext".to_owned(),
    ];
    for namespace in &namespaces {
        let mut document = comparison_document();
        document["profiles"][0]["definition"]["extensions"] =
            object_with(namespace, json!({"type": "boolean", "value": true}));
        let schema_ok = comparison.is_valid(&document);
        let owner_ok = ExtensionNamespace::new(namespace.as_str()).is_ok();
        assert_eq!(schema_ok, owner_ok, "namespace={namespace:?}");
    }

    let key_63 = seg63();
    let key_64 = seg64();
    let keys = [
        "key",
        "a_b",
        key_63.as_str(),
        key_64.as_str(),
        "Key",
        "key.sub",
    ];
    for key in keys {
        let mut document = comparison_document();
        let nested = object_with(key, json!({"type": "boolean", "value": true}));
        document["profiles"][0]["definition"]["extensions"] = object_with(
            "dev.example.ext",
            json!({"type": "object", "value": nested}),
        );
        let schema_ok = comparison.is_valid(&document);
        let owner_ok = ExtensionKey::new(key).is_ok();
        assert_eq!(schema_ok, owner_ok, "key={key:?}");
    }
    Ok(())
}

#[test]
fn checked_reader_matches_schema_for_lexical_profile_defaults() -> Result<(), Box<dyn Error>> {
    let validator = validator("schemas/v2/protocol/comparison-context.schema.json")?;
    let cases = [
        (json!({"locale": "en"}), true),
        (json!({"Bad": "en"}), false),
        (json!({"locale": "Bad Value"}), false),
    ];
    for (defaults, expected) in cases {
        let mut document = comparison_document();
        document["profiles"][0]["definition"]["defaults"] = defaults;
        let bytes = serde_json::to_vec(&document)?;
        let schema_ok = validator.is_valid(&document);
        let reader_ok = ComparisonContextV2Dto::from_json(&bytes).is_ok();
        assert_eq!(
            schema_ok, reader_ok,
            "schema={schema_ok} reader={reader_ok}"
        );
        assert_eq!(reader_ok, expected, "expected owner acceptance");
    }
    Ok(())
}

#[test]
fn checked_reader_rejects_undeclared_default_as_semantic_layer() -> Result<(), Box<dyn Error>> {
    // The schema constrains only lexical vocabulary; the declared-default
    // relationship is an intentional semantic layer enforced by the reader.
    let validator = validator("schemas/v2/protocol/comparison-context.schema.json")?;
    let mut document = comparison_document();
    document["profiles"][0]["definition"]["defaults"] = json!({"locale": "en", "region": "eu"});
    let bytes = serde_json::to_vec(&document)?;
    assert!(validator.is_valid(&document));
    assert!(ComparisonContextV2Dto::from_json(&bytes).is_err());
    Ok(())
}

#[test]
fn checked_reader_rejects_recursive_unknown_extension_member() -> Result<(), Box<dyn Error>> {
    let mut document = comparison_document();
    document["profiles"][0]["definition"]["extensions"] = json!({
        "dev.example.ext": {"type": "boolean", "value": true, "extra": 1}
    });
    let bytes = serde_json::to_vec(&document)?;
    assert!(ComparisonContextV2Dto::from_json(&bytes).is_err());
    Ok(())
}

#[test]
fn authored_schemas_bound_map_keys_and_extensions() -> Result<(), Box<dyn Error>> {
    let workspace = validator("schemas/v2/manifest/workspace.schema.json")?;
    assert!(workspace.is_valid(&workspace_document()));

    let mut bad_target_key = workspace_document();
    let target = bad_target_key["targets"]
        .as_object_mut()
        .ok_or("targets")?
        .remove("web")
        .ok_or("web")?;
    bad_target_key["targets"]["Web"] = target;
    assert!(!workspace.is_valid(&bad_target_key));

    let mut bad_component_key = workspace_document();
    let component = bad_component_key["source_components"]
        .as_object_mut()
        .ok_or("components")?
        .remove("shared.kmp")
        .ok_or("shared.kmp")?;
    bad_component_key["source_components"]["shared"] = component;
    assert!(!workspace.is_valid(&bad_component_key));

    let mut bad_namespace = workspace_document();
    bad_namespace["extensions"] = object_with("dev.example", json!({"ok": true}));
    assert!(!workspace.is_valid(&bad_namespace));

    let scenario = validator("schemas/v2/manifest/scenario.schema.json")?;
    assert!(scenario.is_valid(&scenario_document()));
    let mut bad_dependency = scenario_document();
    bad_dependency["dependencies"] = json!([{"from": "mail.er", "to": "mailer"}]);
    assert!(!scenario.is_valid(&bad_dependency));
    Ok(())
}

#[test]
fn graph_schema_bounds_scenario_dependencies_and_extensions() -> Result<(), Box<dyn Error>> {
    let graph = validator("schemas/v2/protocol/semantic-graph.schema.json")?;
    assert!(graph.is_valid(&graph_document()));

    let mut bad_dependency = graph_document();
    bad_dependency["scenarios"][0]["dependencies"] = json!([{"from": "mailer", "to": "Bad"}]);
    assert!(!graph.is_valid(&bad_dependency));

    let mut bad_participant = graph_document();
    bad_participant["scenarios"][0]["participants"][0]["id"] = json!("mail.er");
    assert!(!graph.is_valid(&bad_participant));

    let mut bad_extension = graph_document();
    bad_extension["providers"][0]["extensions"] =
        object_with("dev.example", json!({"type": "boolean", "value": true}));
    assert!(!graph.is_valid(&bad_extension));

    let mut source_component = graph_document();
    source_component["source_components"][0]["id"] = json!("shared");
    assert!(!graph.is_valid(&source_component));
    Ok(())
}

#[test]
fn accepted_maximum_and_range_controls_are_not_overrestricted() -> Result<(), Box<dyn Error>> {
    let provider = validator("schemas/v2/manifest/provider.schema.json")?;
    let mut document = provider_document();
    document["id"] = json!(dotted_255());
    assert!(provider.is_valid(&document), "255-byte dotted id");

    let cohort = validator("schemas/v2/manifest/cohort.schema.json")?;
    let mut document = cohort_document();
    document["assignments"][0]["repository"] = json!("https://example.com:443/team/repo");
    assert!(cohort.is_valid(&document), "explicit-port repository");

    let workspace = validator("schemas/v2/manifest/workspace.schema.json")?;
    assert!(workspace.is_valid(&workspace_document()));

    let comparison = validator("schemas/v2/protocol/comparison-context.schema.json")?;
    assert!(comparison.is_valid(&comparison_document()));

    let graph = validator("schemas/v2/protocol/semantic-graph.schema.json")?;
    assert!(graph.is_valid(&graph_document()));
    Ok(())
}

#[test]
fn graph_scenario_subject_alternatives_match_owner() -> Result<(), Box<dyn Error>> {
    let graph = validator("schemas/v2/protocol/semantic-graph.schema.json")?;
    let s63 = seg63();
    let s64 = seg64();

    // The `target`-tagged alternative uses the owning extensible TargetId grammar.
    let target_values = [
        "web",
        "web.mobile",
        s63.as_str(),
        s64.as_str(),
        "Web",
        "web..mobile",
        "",
    ];
    parity(
        &graph,
        &graph_document(),
        |document, value| {
            document["scenarios"][0]["participants"][0]["subject"] =
                json!({"kind": "target", "target": value})
        },
        |value| TargetId::new(value).is_ok(),
        &target_values,
    );

    // The `provider`-tagged alternative uses the owning dotted ProviderId grammar.
    let provider_values = [
        "provider.mailgun",
        "provider",
        s63.as_str(),
        s64.as_str(),
        "Provider.Mailgun",
        "provider..mailgun",
        "",
    ];
    parity(
        &graph,
        &graph_document(),
        |document, value| {
            document["scenarios"][0]["participants"][0]["subject"] =
                json!({"kind": "provider", "provider": value})
        },
        |value| ProviderId::new(value).is_ok(),
        &provider_values,
    );

    // A missing or wrong discriminant is rejected independently of the value.
    for subject in [
        json!({}),
        json!({"kind": "target"}),
        json!({"kind": "provider"}),
        json!({"kind": "target", "provider": "provider.mailgun"}),
        json!({"kind": "provider", "target": "web"}),
        json!({"kind": "target", "target": "web", "provider": "provider.mailgun"}),
    ] {
        let mut document = graph_document();
        document["scenarios"][0]["participants"][0]["subject"] = subject.clone();
        assert!(
            !graph.is_valid(&document),
            "subject discriminant accepted {subject}"
        );
    }
    Ok(())
}

#[test]
fn graph_source_component_roots_match_owner_and_cardinality() -> Result<(), Box<dyn Error>> {
    let graph = validator("schemas/v2/protocol/semantic-graph.schema.json")?;
    assert!(graph.is_valid(&graph_document()), "valid positive");

    // Each single root value must agree with the owning `RepoPath` constructor.
    for value in [
        "shared/kmp",
        "shared/kmp/nested",
        "apps/café/view.swift",
        "../escape",
        "/abs",
        "a\\b",
        "CON",
        "a.",
        "a ",
        "a//b",
        "",
    ] {
        let mut document = graph_document();
        document["source_components"][0]["roots"] = json!([value]);
        let schema_ok = graph.is_valid(&document);
        let owner_ok = RepoPath::new(value).is_ok();
        assert_eq!(
            schema_ok, owner_ok,
            "root={value:?} schema={schema_ok} owner={owner_ok}"
        );
    }

    // Cardinality and exact-duplicate exclusion are separate collection rules.
    let mut empty = graph_document();
    empty["source_components"][0]["roots"] = json!([]);
    assert!(!graph.is_valid(&empty), "empty roots accepted");

    let mut duplicate = graph_document();
    duplicate["source_components"][0]["roots"] = json!(["shared/kmp", "shared/kmp"]);
    assert!(!graph.is_valid(&duplicate), "duplicate roots accepted");
    Ok(())
}

#[test]
fn authored_workspace_component_roots_match_owner_and_cardinality() -> Result<(), Box<dyn Error>> {
    let workspace = validator("schemas/v2/manifest/workspace.schema.json")?;
    assert!(workspace.is_valid(&workspace_document()), "valid positive");

    for value in [
        "shared/kmp",
        "shared/kmp/nested",
        "apps/café/view.swift",
        "../escape",
        "/abs",
        "a\\b",
        "CON",
        "a.",
        "a ",
        "a//b",
        "",
    ] {
        let mut document = workspace_document();
        document["source_components"]["shared.kmp"]["roots"] = json!([value]);
        let schema_ok = workspace.is_valid(&document);
        let owner_ok = RepoPath::new(value).is_ok();
        assert_eq!(
            schema_ok, owner_ok,
            "root={value:?} schema={schema_ok} owner={owner_ok}"
        );
    }

    let mut empty = workspace_document();
    empty["source_components"]["shared.kmp"]["roots"] = json!([]);
    assert!(!workspace.is_valid(&empty), "empty roots accepted");

    let mut duplicate = workspace_document();
    duplicate["source_components"]["shared.kmp"]["roots"] = json!(["shared/kmp", "shared/kmp"]);
    assert!(!workspace.is_valid(&duplicate), "duplicate roots accepted");
    Ok(())
}

#[test]
fn graph_capability_value_vocabularies_are_closed() -> Result<(), Box<dyn Error>> {
    let graph = validator("schemas/v2/protocol/semantic-graph.schema.json")?;
    let mut document = graph_document();
    document["capabilities"] = json!([{
        "id": "account.create",
        "title": "Account creation",
        "status": "active",
        "owners": ["owner://team/accounts"],
        "extensions": {}
    }]);
    assert!(graph.is_valid(&document), "valid capability positive");

    for bad_status in ["Active", "inactive", "removed", ""] {
        let mut mutated = document.clone();
        mutated["capabilities"][0]["status"] = json!(bad_status);
        assert!(!graph.is_valid(&mutated), "status accepted {bad_status:?}");
    }
    for bad_id in ["account", "Account.Create", "account..create"] {
        let mut mutated = document.clone();
        mutated["capabilities"][0]["id"] = json!(bad_id);
        assert!(!graph.is_valid(&mutated), "id accepted {bad_id:?}");
    }
    let long_title = repeated(b'a', 161);
    for bad_title in ["", long_title.as_str()] {
        let mut mutated = document.clone();
        mutated["capabilities"][0]["title"] = json!(bad_title);
        assert!(!graph.is_valid(&mutated), "title accepted {bad_title:?}");
    }
    let mut no_owners = document.clone();
    no_owners["capabilities"][0]["owners"] = json!([]);
    assert!(!graph.is_valid(&no_owners), "empty owners accepted");
    Ok(())
}

#[test]
fn authored_target_root_matches_portable_owner() -> Result<(), Box<dyn Error>> {
    let workspace = validator("schemas/v2/manifest/workspace.schema.json")?;
    assert!(workspace.is_valid(&workspace_document()), "valid positive");
    for value in [
        "apps/web",
        "apps/web/nested",
        "apps/café/view.swift",
        "../escape",
        "/abs",
        "a\\b",
        "CON",
        "a.",
        "a ",
        "",
    ] {
        let mut document = workspace_document();
        document["targets"]["web"]["root"] = json!(value);
        let schema_ok = workspace.is_valid(&document);
        let owner_ok = RepoPath::new(value).is_ok();
        assert_eq!(
            schema_ok, owner_ok,
            "root={value:?} schema={schema_ok} owner={owner_ok}"
        );
    }
    Ok(())
}

#[test]
fn authored_scenario_collections_reject_exact_duplicates() -> Result<(), Box<dyn Error>> {
    let scenario = validator("schemas/v2/manifest/scenario.schema.json")?;
    assert!(scenario.is_valid(&scenario_document()), "valid positive");

    let participant = scenario_document()["participants"][0].clone();
    let mut duplicate_participants = scenario_document();
    duplicate_participants["participants"] = json!([participant.clone(), participant]);
    assert!(
        !scenario.is_valid(&duplicate_participants),
        "duplicate participant accepted"
    );

    let dependency = json!({"from": "mailer", "to": "mailer"});
    let mut duplicate_dependencies = scenario_document();
    duplicate_dependencies["dependencies"] = json!([dependency.clone(), dependency]);
    assert!(
        !scenario.is_valid(&duplicate_dependencies),
        "duplicate dependency accepted"
    );

    let mut no_participants = scenario_document();
    no_participants["participants"] = json!([]);
    assert!(
        !scenario.is_valid(&no_participants),
        "empty participants accepted"
    );
    Ok(())
}

#[test]
fn v1_schema_compatibility_control_remains_parseable() -> Result<(), Box<dyn Error>> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for relative in [
        "schemas/v1/manifest/workspace.schema.json",
        "schemas/v1/protocol/semantic-graph.schema.json",
    ] {
        let schema: Value = serde_json::from_slice(&std::fs::read(root.join(relative))?)?;
        let _validator = jsonschema::options()
            .with_draft(Draft::Draft202012)
            .should_validate_formats(true)
            .build(&schema)?;
        assert!(
            schema["$id"]
                .as_str()
                .is_some_and(|id| id.contains("/schemas/v1/")),
            "v1 schema identity is preserved"
        );
    }
    Ok(())
}
