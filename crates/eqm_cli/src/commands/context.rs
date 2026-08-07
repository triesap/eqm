//! Bounded, trust-labeled developer and agent context query.

use super::{CommandExecution, evaluation};
use crate::cli::ParsedCli;
use crate::renderer::OutputPayload;
use crate::session::{SessionRequest, prepare};
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_domain::UtcInstant;
use eqm_manifest::{
    project_binding, project_capability, project_journey, project_policy, project_profile,
    project_surface, project_waiver,
};
use eqm_protocol::{
    CommandIdentity, ContextResultDto, EvaluationModeDto, FacetStatusDto, FindingDto,
    InvocationContextDto, ReportEnvelope,
};
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::error::Error;
use std::path::Path;
use std::time::SystemTime;

type ContextResult = ContextResultDto<Value, Value, Value, Value>;

/// Builds one bounded read-only context document.
pub fn execute(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    let offline = parsed.global.offline;
    let profiles = parsed.global.profiles.clone();
    let unit = parsed.command.operands[0].clone();
    let target = option_value(&parsed, "--target").map(ToOwned::to_owned);
    let maximum = number_option(&parsed, "--max-bytes", 65_536)?;
    let depth = number_option(&parsed, "--max-depth", 4)?;
    let request = SessionRequest::new(parsed.global, parsed.command.name);
    let session = prepare(&request, start)?;
    let graph = session.finalized().graph();
    let bindings = graph
        .bindings()
        .values()
        .filter(|binding| {
            binding.unit().as_str() == unit
                && target
                    .as_deref()
                    .is_none_or(|value| binding.target().as_str() == value)
        })
        .collect::<Vec<_>>();
    if bindings.is_empty() {
        return Err("context unit did not resolve to a binding".into());
    }
    let (selection, derived) = evaluation::derive(&session, &profiles)?;

    let mut authority_candidates = Vec::new();
    if let Some(surface) = graph
        .surfaces()
        .values()
        .find(|value| value.id().as_str() == unit)
    {
        let journey = graph.journeys().get(surface.journey()).ok_or("journey")?;
        let capability = graph
            .capabilities()
            .get(journey.capability())
            .ok_or("capability")?;
        authority_candidates.extend([
            (
                3usize,
                authority_record("capability", project_capability(capability)),
            ),
            (2, authority_record("journey", project_journey(journey))),
            (1, authority_record("surface", project_surface(surface))),
        ]);
    }
    authority_candidates.push((
        2,
        authority_record("policy", project_policy(selection.policy())),
    ));
    for selected in selection.profiles().values() {
        let profile = graph
            .profiles()
            .get(&(selected.id().clone(), selected.revision()))
            .ok_or("profile")?;
        authority_candidates.push((2, authority_record("profile", project_profile(profile))));
    }

    let mut product_records = Vec::new();
    let mut evidence_records = Vec::new();
    for binding in &bindings {
        authority_candidates.push((1, authority_record("binding", project_binding(binding))));
        for artifact in binding.artifacts().values().values() {
            product_records.push(json!({
                "trust": "untrusted_product_path",
                "target": binding.target().as_str(),
                "role": artifact.role().as_str(),
                "path": artifact.path().as_str(),
                "symbol": artifact.symbol().map(|value| value.as_str()),
            }));
        }
        for evidence in binding.evidence().values() {
            evidence_records.push(json!({
                "trust": "unverified_evidence_declaration",
                "target": binding.target().as_str(),
                "id": evidence.id().as_str(),
                "kind": evidence.kind().as_str(),
                "runner": evidence.runner().map(|value| value.as_str()),
                "requirements": evidence.requirements().iter().map(|value| value.as_str()).collect::<Vec<_>>(),
                "facets": evidence.facets().iter().map(|value| value.as_str()).collect::<Vec<_>>(),
            }));
        }
    }
    let obligations = derived
        .obligations
        .values()
        .filter(|value| {
            value.key.unit.as_str() == unit && subject_matches(value, target.as_deref())
        })
        .map(evaluation::obligation_dto)
        .collect::<Result<BTreeSet<_>, _>>()?;
    let findings = obligations
        .iter()
        .map(|obligation| FindingDto {
            diagnostic_code: "EQM-E0500".to_owned(),
            obligation: Some(obligation.id.clone()),
            status: FacetStatusDto::Missing,
            evidence: None,
            waiver: None,
        })
        .collect();
    let waiver_records = graph
        .waivers()
        .values()
        .filter(|waiver| waiver.scope().unit().as_str() == unit)
        .map(|waiver| authority_record("waiver", project_waiver(waiver)))
        .collect::<Vec<_>>();
    let mut depth_omitted = 0u64;
    let authority_records = authority_candidates
        .into_iter()
        .filter_map(|(distance, record)| {
            if distance <= depth {
                Some(record)
            } else {
                depth_omitted = depth_omitted.saturating_add(
                    serde_json::to_vec(&record)
                        .ok()
                        .and_then(|bytes| u64::try_from(bytes.len()).ok())
                        .unwrap_or(u64::MAX),
                );
                None
            }
        })
        .collect::<Vec<_>>();
    let mut result = ContextResult {
        kind: CommandIdentity::Context,
        unit: unit.clone(),
        target: target.clone(),
        authority: json!({"trust":"procedural_authority","records":authority_records}),
        product_data: json!({"trust":"untrusted_product_data","records":product_records}),
        obligations,
        evidence: json!({"trust":"unverified_evidence","records":evidence_records}),
        findings,
        waivers: json!({"trust":"protected_authority_candidate","records":waiver_records}),
        truncated: depth_omitted > 0,
        omitted_bytes: depth_omitted,
    };
    bound_result(&mut result, maximum.saturating_sub(32))?;
    let compact = serde_json::to_string(&result)?;
    let markdown = format!("# EQM Context\n\n```json\n{compact}\n```");
    if markdown.len() > maximum {
        return Err("bounded context markdown exceeded maximum".into());
    }
    let envelope = ReportEnvelope::new(
        CommandIdentity::Context,
        Some(session.workspace_digest()),
        invocation(offline)?,
        Some(result),
        Vec::new(),
    )?;
    Ok(CommandExecution {
        payload: OutputPayload {
            human: format!("context for {unit}"),
            json: serde_json::from_slice(&envelope.to_json()?)?,
            sarif: None,
            markdown: Some(markdown),
        },
        exit_code: 0,
    })
}

fn authority_record(kind: &str, entity: Value) -> Value {
    json!({"kind":kind,"provenance":"authored_semantic_authority","entity":entity})
}

fn subject_matches(obligation: &eqm_engine::Obligation, target: Option<&str>) -> bool {
    target.is_none_or(|target| match &obligation.key.subject {
        eqm_engine::ScopeSubject::Target(value) => value.as_str() == target,
        eqm_engine::ScopeSubject::TargetSet(values) => {
            values.iter().any(|value| value.as_str() == target)
        }
        eqm_engine::ScopeSubject::Provider(_) => true,
    })
}

fn bound_result(result: &mut ContextResult, maximum: usize) -> Result<(), Box<dyn Error>> {
    let mut omitted = result.omitted_bytes;
    while serde_json::to_vec(result)?.len() > maximum {
        let before = serde_json::to_vec(result)?.len();
        let removed = pop_record(&mut result.product_data)
            || pop_record(&mut result.evidence)
            || pop_record(&mut result.waivers)
            || result.findings.pop_last().is_some()
            || result.obligations.pop_last().is_some()
            || pop_record(&mut result.authority);
        if !removed {
            return Err("context labels exceed maximum bytes".into());
        }
        result.truncated = true;
        let after = serde_json::to_vec(result)?.len();
        omitted = omitted.saturating_add(u64::try_from(before.saturating_sub(after))?);
        result.omitted_bytes = omitted;
    }
    Ok(())
}

fn pop_record(section: &mut Value) -> bool {
    section
        .get_mut("records")
        .and_then(Value::as_array_mut)
        .and_then(Vec::pop)
        .is_some()
}

fn option_value<'a>(parsed: &'a ParsedCli, name: &str) -> Option<&'a str> {
    parsed.command.options.get(name)?.first()?.as_deref()
}

fn number_option(parsed: &ParsedCli, name: &str, default: usize) -> Result<usize, Box<dyn Error>> {
    option_value(parsed, name).map_or(Ok(default), |value| Ok(value.parse()?))
}

fn invocation(offline: bool) -> Result<InvocationContextDto<(), ()>, Box<dyn Error>> {
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
    fn context_is_bounded_trust_labeled_and_nonexecuting() -> Result<(), Box<dyn Error>> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let ParseOutcome::Run(parsed) = parse([
            "context",
            "account.create.signup.identifier",
            "--target",
            "web",
            "--max-bytes",
            "1024",
            "--max-depth",
            "4",
            "--format",
            "markdown",
            "--no-progress",
        ])?
        else {
            return Err("unexpected help".into());
        };
        let result = execute(parsed, &root)?;
        assert_eq!(result.exit_code, 0);
        let markdown = result.payload.markdown.ok_or("markdown")?;
        assert!(markdown.len() <= 1024);
        assert!(markdown.contains("procedural_authority"));
        assert!(markdown.contains("untrusted_product_data"));
        assert!(
            result.payload.json["result"]["truncated"]
                .as_bool()
                .is_some()
        );
        Ok(())
    }
}
