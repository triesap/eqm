//! Read-only reconciliation of authored exposure declarations and prepared inventory.

use super::{CommandExecution, discover, evaluation};
use crate::cli::ParsedCli;
use crate::renderer::OutputPayload;
use crate::session::{PreparedSession, SessionRequest, prepare};
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_application::{prepare_inventory_facts, reconcile_prepared_inventory_exposure};
use eqm_domain::{IntendedExposureState, RepoPath, SelectorText, TargetId, UtcInstant};
use eqm_engine::{
    ConformanceFact, ExpectedExposure, ExposureComparison, ExposureReconciliation,
    ObservedExposure, TruthValue, evaluate_applicability,
};
use eqm_protocol::{
    CommandIdentity, EvaluationModeDto, ExposureComparisonDto, ExposureFactDto, InventoryDto,
    InvocationContextDto, ReconcileResultDto, ReportEnvelope,
};
use eqm_runner::{InventoryExposureInput, InventoryObservation, validate_inventory_claim};
use std::collections::BTreeSet;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

/// Reconciles prepared declarations and optional inventory without discovery.
pub fn execute(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    let offline = parsed.global.offline;
    let profiles = parsed.global.profiles.clone();
    let scenarios = parsed.global.scenarios.clone();
    let policy = parsed.global.policy.clone();
    let target = option(&parsed, "--target")
        .ok_or("target required")?
        .to_owned();
    let unit = option(&parsed, "--unit").map(str::to_owned);
    let inventory_path = option(&parsed, "--inventory").map(str::to_owned);
    let request = SessionRequest::new(parsed.global, parsed.command.name);
    let session = prepare(&request, start)?;
    let target_id = TargetId::new(target.as_str())?;
    if !session
        .finalized()
        .graph()
        .targets()
        .contains_key(&target_id)
    {
        return Err(format!("target `{target}` was not found").into());
    }
    let (selection, derived) =
        evaluation::derive_with_policy(&session, policy.as_deref(), &profiles, &scenarios)?;
    let applicability =
        eqm_application::applicability_context(session.finalized().graph(), &selection)?;
    let observation = inventory_path
        .as_deref()
        .map(|path| load_inventory(&session, path, &target_id))
        .transpose()?;
    // Imported subject/completeness remain claims. No independent expected
    // subject, configured time/trust policy or verifier is available here.
    let prepared_inventory = prepare_inventory_facts(
        observation.as_ref().map(|loaded| &loaded.observation),
        observation.as_ref().map(|loaded| loaded.bytes.as_slice()),
        None,
        None,
        session.evaluation_time(),
        None,
    );
    let mut facts = BTreeSet::new();
    let mut comparisons = BTreeSet::new();
    for binding in session
        .finalized()
        .graph()
        .bindings()
        .values()
        .filter(|binding| {
            binding.target() == &target_id
                && unit
                    .as_deref()
                    .is_none_or(|unit| binding.unit().as_str() == unit)
        })
    {
        for exposure in binding.exposures() {
            let truth = evaluate_applicability(exposure.applicability(), &applicability)?;
            let expected = match (truth, exposure.state()) {
                (TruthValue::True, IntendedExposureState::Required) => ExpectedExposure::Required,
                (TruthValue::True, IntendedExposureState::Prohibited) => {
                    ExpectedExposure::Prohibited
                }
                (TruthValue::False | TruthValue::Unknown, _) => ExpectedExposure::Unknown,
            };
            let (kind, key) = exposure.route().map_or_else(
                || ("surface", exposure.surface().as_str()),
                |route| ("route", route.as_str()),
            );
            let input = InventoryExposureInput {
                expected,
                declared: ObservedExposure::True,
                enabled: ObservedExposure::Unknown,
                released: ObservedExposure::Unknown,
                conformant: ConformanceFact::Unknown,
            };
            let reconciled = reconcile_prepared_inventory_exposure(
                input,
                &prepared_inventory,
                &SelectorText::new(kind)?,
                &SelectorText::new(key)?,
            );
            append_records(
                &mut facts,
                &mut comparisons,
                &format!("{}:{}", binding.unit(), exposure.surface()),
                &reconciled,
                observation.is_some(),
            );
        }
    }
    let result = ReconcileResultDto {
        kind: CommandIdentity::Reconcile,
        target,
        unit,
        facts,
        comparisons,
    };
    let envelope = ReportEnvelope::new(
        CommandIdentity::Reconcile,
        Some(session.workspace_digest()),
        context(offline)?,
        Some(result),
        evaluation::scope_diagnostics(&derived)?
            .iter()
            .map(eqm_protocol::DiagnosticDto::from_domain)
            .collect(),
    )?;
    Ok(CommandExecution {
        payload: OutputPayload {
            human: "exposure reconciliation completed".to_owned(),
            json: serde_json::from_slice(&envelope.to_json()?)?,
            sarif: None,
            markdown: None,
        },
        exit_code: 0,
    })
}

struct LoadedInventory {
    observation: InventoryObservation,
    bytes: Vec<u8>,
}

fn load_inventory(
    session: &PreparedSession,
    path: &str,
    target: &TargetId,
) -> Result<LoadedInventory, Box<dyn Error>> {
    let relative = RepoPath::new(path)?;
    let candidate = session.repository_root().join(relative.as_str());
    let metadata = fs::symlink_metadata(&candidate)?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err("inventory must be a regular repository file".into());
    }
    let absolute = candidate.canonicalize()?;
    if !absolute.starts_with(session.repository_root().canonicalize()?) {
        return Err("inventory escaped repository confinement".into());
    }
    let bytes = fs::read(absolute)?;
    if bytes.len() > 16 * 1024 * 1024 {
        return Err("inventory exceeds the protocol bound".into());
    }
    let inventory = InventoryDto::from_json(&bytes)?;
    let adapter = inventory.adapter.parse()?;
    let digest = inventory.adapter_digest.parse()?;
    let matches = session
        .finalized()
        .graph()
        .adapter_locks()
        .values()
        .filter(|lock| lock.id == adapter && lock.digest == digest)
        .collect::<Vec<_>>();
    if matches.len() != 1 {
        return Err("inventory adapter did not resolve to one exact lock pin".into());
    }
    let definition = discover::definition(matches[0])?;
    let observation = validate_inventory_claim(&definition, target, inventory)?;
    Ok(LoadedInventory { observation, bytes })
}

fn append_records(
    facts: &mut BTreeSet<ExposureFactDto>,
    comparisons: &mut BTreeSet<ExposureComparisonDto>,
    coordinate: &str,
    value: &ExposureReconciliation,
    has_inventory: bool,
) {
    let values = [
        (
            "expected",
            expected(value.facts.expected),
            "contract_policy",
            "repository_authority",
        ),
        (
            "declared",
            observed(value.facts.declared),
            "binding",
            "repository_authority",
        ),
        (
            "discovered",
            observed(value.facts.discovered),
            if has_inventory {
                "inventory"
            } else {
                "not_prepared"
            },
            if has_inventory {
                "untrusted_adapter_output"
            } else {
                "unknown"
            },
        ),
        (
            "enabled",
            observed(value.facts.enabled),
            "not_prepared",
            "unknown",
        ),
        (
            "released",
            observed(value.facts.released),
            "not_prepared",
            "unknown",
        ),
        (
            "conformant",
            conformance(value.facts.conformant),
            "not_prepared",
            "unknown",
        ),
    ];
    for (name, value, source, trust) in values {
        facts.insert(ExposureFactDto {
            name: format!("{coordinate}:{name}"),
            value: value.to_owned(),
            source: source.to_owned(),
            freshness: if source == "not_prepared" || name == "discovered" {
                "unknown"
            } else {
                "current"
            }
            .to_owned(),
            effective_trust: trust.to_owned(),
        });
    }
    for (fact, fact_value, result) in [
        ("declared", value.facts.declared, value.declared),
        ("discovered", value.facts.discovered, value.discovered),
        ("enabled", value.facts.enabled, value.enabled),
        ("released", value.facts.released, value.released),
    ] {
        comparisons.insert(ExposureComparisonDto {
            fact: format!("{coordinate}:{fact}"),
            expected: expected(value.facts.expected).to_owned(),
            observed: observed(fact_value).to_owned(),
            result: comparison(result).to_owned(),
        });
    }
}

const fn expected(value: ExpectedExposure) -> &'static str {
    match value {
        ExpectedExposure::Required => "required",
        ExpectedExposure::Prohibited => "prohibited",
        ExpectedExposure::Unknown => "unknown",
    }
}

const fn observed(value: ObservedExposure) -> &'static str {
    match value {
        ObservedExposure::True => "true",
        ObservedExposure::False => "false",
        ObservedExposure::Unknown => "unknown",
    }
}

const fn conformance(value: ConformanceFact) -> &'static str {
    match value {
        ConformanceFact::True => "true",
        ConformanceFact::Conditional => "conditional",
        ConformanceFact::False => "false",
        ConformanceFact::Unknown => "unknown",
    }
}

const fn comparison(value: ExposureComparison) -> &'static str {
    match value {
        ExposureComparison::Match => "match",
        ExposureComparison::Mismatch => "mismatch",
        ExposureComparison::Unknown => "unknown",
    }
}

fn option<'a>(parsed: &'a ParsedCli, name: &str) -> Option<&'a str> {
    parsed
        .command
        .options
        .get(name)
        .and_then(|values| values.first())
        .and_then(Option::as_deref)
}

fn context(offline: bool) -> Result<InvocationContextDto<(), ()>, Box<dyn Error>> {
    let value: DateTime<Utc> = SystemTime::now().into();
    let instant: UtcInstant = value.to_rfc3339_opts(SecondsFormat::Secs, true).parse()?;
    Ok(InvocationContextDto::new(
        EvaluationModeDto::Development,
        Vec::new(),
        None,
        None,
        offline,
        instant,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{ParseOutcome, parse};

    #[test]
    fn declarations_reconcile_without_implicit_discovery() -> Result<(), Box<dyn Error>> {
        let repository = crate::test_support::example_repository()?;
        let root = repository.path();
        let before = fs::read_dir(root.join(".eqm"))
            .ok()
            .map(|entries| entries.count());
        let ParseOutcome::Run(parsed) = parse([
            "reconcile",
            "--target",
            "android",
            "--format",
            "json",
            "--no-progress",
        ])?
        else {
            return Err("unexpected help".into());
        };
        let execution = execute(parsed, root)?;
        assert_eq!(execution.exit_code, 0);
        assert_eq!(execution.payload.json["command"], "reconcile");
        assert!(
            execution.payload.json["result"]["facts"]
                .as_array()
                .is_some_and(|facts| facts.iter().any(|fact| {
                    fact["name"]
                        .as_str()
                        .is_some_and(|name| name.ends_with(":discovered"))
                        && fact["value"] == "unknown"
                }))
        );
        let after = fs::read_dir(root.join(".eqm"))
            .ok()
            .map(|entries| entries.count());
        assert_eq!(before, after);
        Ok(())
    }

    #[test]
    fn duplicate_inventory_attributes_fail_at_locked_adapter_reconcile_ingress()
    -> Result<(), Box<dyn Error>> {
        let repository = crate::test_support::example_repository()?;
        let root = repository.path();
        let adapter_digest = eqm_domain::Sha256Digest::hash_content(b"adapter-fixture").to_string();
        let mut lock = fs::read_to_string(root.join("eqm.lock"))?;
        lock.push_str(&format!(
            "\n[[adapters]]\nid = \"adapter.test\"\nversion = \"1.0.0\"\nsource = \"https://example.com/adapters/test\"\nresolved = \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"\ndigest = \"{adapter_digest}\"\nprotocol = 1\n"
        ));
        fs::write(root.join("eqm.lock"), lock)?;
        let request = SessionRequest::new(Default::default(), crate::cli::CommandName::Reconcile);
        let session = prepare(&request, root)?;
        let target = TargetId::new("android")?;

        // A fully valid locked inventory. `build_id`/`artifact_digest` are
        // present as null because the v1 subject DTO serializes absent options
        // as null, and the canonical digest is computed over that form.
        let mut inventory = serde_json::json!({
            "schema": eqm_protocol::INVENTORY_SCHEMA.to_string(),
            "adapter": "adapter.test",
            "adapter_digest": adapter_digest,
            "subject": {
                "repository": "https://example.com/product/android",
                "repository_id_digest": "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "scope": {"kind": "target", "target": "android"},
                "source_commit": "a".repeat(40),
                "build_id": serde_json::Value::Null,
                "artifact_digest": serde_json::Value::Null,
                "target_configuration_digest": "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
            },
            "target": "android",
            "generated_at": "2026-08-07T12:00:00Z",
            "completeness": "complete",
            "entries": [{
                "kind": "route",
                "key": "home",
                "attributes": {"enabled": {"type": "boolean", "value": true}},
                "source": "src/home.ts"
            }],
            "diagnostics": []
        });
        let digest =
            eqm_domain::Sha256Digest::hash_content(&serde_json_canonicalizer::to_vec(&inventory)?)
                .to_string();
        inventory["inventory_digest"] = serde_json::json!(digest);
        let valid_text = serde_json::to_string(&inventory)?;
        fs::write(root.join("inventory.valid.json"), &valid_text)?;

        // The positive control genuinely succeeds at the locked-adapter ingress.
        let valid = load_inventory(&session, "inventory.valid.json", &target)?;
        assert!(valid.observation.inventory().is_some());
        assert!(!valid.observation.can_prove_absence());

        // The actual consuming command must not infer current verified product
        // facts from either a complete absent claim or a present exact route.
        let exposure = session
            .finalized()
            .graph()
            .bindings()
            .values()
            .filter(|binding| binding.target() == &target)
            .flat_map(|binding| binding.exposures())
            .next()
            .ok_or("exposure")?;
        let (kind, key) = exposure.route().map_or_else(
            || ("surface", exposure.surface().as_str()),
            |route| ("route", route.as_str()),
        );
        for present in [false, true] {
            let mut claim = inventory.clone();
            claim["entries"] = if present {
                serde_json::json!([{
                    "kind": kind, "key": key, "attributes": {}, "source": "src/home.ts"
                }])
            } else {
                serde_json::json!([])
            };
            claim
                .as_object_mut()
                .ok_or("claim")?
                .remove("inventory_digest");
            claim["inventory_digest"] = serde_json::json!(
                eqm_domain::Sha256Digest::hash_content(&serde_json_canonicalizer::to_vec(&claim)?)
                    .to_string()
            );
            fs::write(
                root.join("inventory.consume.json"),
                serde_json::to_vec(&claim)?,
            )?;
            let crate::cli::ParseOutcome::Run(parsed) = crate::cli::parse([
                "reconcile",
                "--target",
                "android",
                "--inventory",
                "inventory.consume.json",
                "--format",
                "json",
                "--no-progress",
            ])?
            else {
                return Err("parse".into());
            };
            let result = execute(parsed, root)?;
            let facts = result.payload.json["result"]["facts"]
                .as_array()
                .ok_or("facts")?;
            let discovered = facts
                .iter()
                .filter(|fact| {
                    fact["name"]
                        .as_str()
                        .is_some_and(|name| name.ends_with(":discovered"))
                })
                .collect::<Vec<_>>();
            assert!(!discovered.is_empty());
            assert!(discovered.iter().all(|fact| fact["value"] == "unknown"
                && fact["freshness"] == "unknown"
                && fact["effective_trust"] == "untrusted_adapter_output"));
            assert!(!root.join(".eqm").exists());
        }

        // The mutant differs only by a second, differently-valued `enabled`
        // member, so it must fail as an ambiguous duplicate parse at the same
        // ingress rather than as some unrelated later error.
        let duplicate_text = valid_text.replace(
            "\"enabled\":{\"type\":\"boolean\",\"value\":true}",
            "\"enabled\":{\"type\":\"boolean\",\"value\":true},\"enabled\":{\"type\":\"boolean\",\"value\":false}",
        );
        assert_ne!(
            duplicate_text, valid_text,
            "duplicate mutation did not apply"
        );
        fs::write(root.join("inventory.duplicate.json"), duplicate_text)?;
        let duplicate = load_inventory(&session, "inventory.duplicate.json", &target);
        assert_eq!(
            duplicate.err().map(|error| error.to_string()).as_deref(),
            Some("Json")
        );
        Ok(())
    }
}
