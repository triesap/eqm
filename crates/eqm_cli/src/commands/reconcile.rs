//! Read-only reconciliation of authored exposure declarations and prepared inventory.

use super::{CommandExecution, discover, evaluation};
use crate::cli::ParsedCli;
use crate::renderer::OutputPayload;
use crate::session::{PreparedSession, SessionRequest, prepare};
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_domain::{IntendedExposureState, RepoPath, SelectorText, TargetId, UtcInstant};
use eqm_engine::{
    ConformanceFact, ExpectedExposure, ExposureComparison, ExposureReconciliation,
    ObservedExposure, TruthValue, evaluate_applicability, reconcile_exposure,
};
use eqm_protocol::{
    ADAPTER_REQUEST_SCHEMA, ADAPTER_RESPONSE_SCHEMA, AdapterLimitsDto, AdapterOperationDto,
    AdapterRequestDto, AdapterResponseDto, AdapterStatusDto, CommandIdentity, EvaluationModeDto,
    ExposureComparisonDto, ExposureFactDto, InventoryDto, InvocationContextDto, ReconcileResultDto,
    ReportEnvelope,
};
use eqm_runner::{
    InventoryExposureInput, InventoryObservation, reconcile_inventory_exposure,
    validate_inventory_response,
};
use std::collections::BTreeSet;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

/// Reconciles prepared declarations and optional inventory without discovery.
pub fn execute(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    let offline = parsed.global.offline;
    let profiles = parsed.global.profiles.clone();
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
    let (selection, _) = evaluation::derive(&session, &profiles)?;
    let applicability = evaluation::applicability_context(&session, &selection)?;
    let observation = inventory_path
        .as_deref()
        .map(|path| load_inventory(&session, path, &target_id))
        .transpose()?;
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
            let reconciled = if let Some(observation) = observation.as_ref() {
                reconcile_inventory_exposure(
                    input,
                    observation,
                    &SelectorText::new(kind)?,
                    &SelectorText::new(key)?,
                )
            } else {
                reconcile_exposure(eqm_engine::ExposureFacts {
                    expected,
                    declared: input.declared,
                    discovered: ObservedExposure::Unknown,
                    enabled: input.enabled,
                    released: input.released,
                    conformant: input.conformant,
                })
            };
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
        Vec::new(),
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

fn load_inventory(
    session: &PreparedSession,
    path: &str,
    target: &TargetId,
) -> Result<InventoryObservation, Box<dyn Error>> {
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
    let inventory: InventoryDto = serde_json::from_slice(&bytes)?;
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
    let limits = definition.limits();
    let request = AdapterRequestDto {
        schema: ADAPTER_REQUEST_SCHEMA.to_string(),
        request_id: "reconcile-prepared-inventory".to_owned(),
        adapter: definition.id().as_str().to_owned(),
        adapter_digest: definition.digest().to_string(),
        operation: AdapterOperationDto::Discover,
        subject: inventory.subject.clone(),
        target: target.as_str().to_owned(),
        target_root: session.repository_root().to_string_lossy().into_owned(),
        limits: AdapterLimitsDto {
            timeout_ms: limits.timeout().get(),
            max_input_bytes: limits.max_input_bytes().get(),
            max_output_bytes: limits.max_output_bytes().get(),
            max_entries: limits.max_entries().get(),
            max_depth: limits.max_depth().get(),
        },
    };
    let status = if inventory.completeness == "complete" {
        AdapterStatusDto::Ok
    } else {
        AdapterStatusDto::Partial
    };
    validate_inventory_response(
        &definition,
        &request,
        AdapterResponseDto {
            schema: ADAPTER_RESPONSE_SCHEMA.to_string(),
            request_id: request.request_id.clone(),
            adapter: request.adapter.clone(),
            adapter_digest: request.adapter_digest.clone(),
            status,
            inventory: Some(inventory),
            diagnostics: Vec::new(),
        },
    )
    .map_err(Into::into)
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
            freshness: if source == "not_prepared" {
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
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let before = fs::read_dir(root.join(".eqm"))
            .ok()
            .map(|entries| entries.count());
        let ParseOutcome::Run(parsed) = parse([
            "reconcile",
            "--target",
            "web",
            "--format",
            "json",
            "--no-progress",
        ])?
        else {
            return Err("unexpected help".into());
        };
        let execution = execute(parsed, &root)?;
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
}
