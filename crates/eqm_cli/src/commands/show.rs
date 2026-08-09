//! Exact semantic entity query.

use super::{CommandExecution, source_location};
use crate::cli::ParsedCli;
use crate::renderer::OutputPayload;
use crate::session::{PreparedSession, SessionRequest, prepare};
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_domain::{Diagnostic, DiagnosticCode, Severity, UtcInstant};
use eqm_engine::diagnostic_registry;
use eqm_manifest::{
    project_binding, project_capability, project_fragment, project_journey, project_policy,
    project_profile, project_runner, project_surface, project_target, project_waiver,
};
use eqm_protocol::{
    CommandIdentity, DiagnosticDto, EvaluationModeDto, InvocationContextDto, ReportEnvelope,
    SarifLogDto, ShowResultDto,
};
use serde_json::Value;
use std::error::Error;
use std::path::Path;
use std::time::SystemTime;

#[derive(Clone, Debug, PartialEq)]
struct Match {
    source_key: String,
    entity: Value,
}

/// Returns one exact semantic entity without exposing parser-only fields.
pub fn execute(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    let offline = parsed.global.offline;
    let entity_kind = parsed.command.operands[0].clone();
    let entity_id = parsed.command.operands[1].clone();
    let request = SessionRequest::new(parsed.global, parsed.command.name);
    let session = prepare(&request, start)?;
    let matches = query(&session, &entity_kind, &entity_id);
    if matches.len() != 1 {
        return unresolved(
            offline,
            session.workspace_digest(),
            &entity_kind,
            &entity_id,
            matches.len(),
        );
    }
    let found = &matches[0];
    let source_path = session
        .source_map()
        .get(found.source_key.as_str())
        .ok_or("entity source mapping")?;
    let result = ShowResultDto {
        kind: CommandIdentity::Show,
        entity_kind: entity_kind.clone(),
        entity_id: entity_id.clone(),
        source: source_location(source_path)?,
        entity: found.entity.clone(),
    };
    let envelope = ReportEnvelope::new(
        CommandIdentity::Show,
        Some(session.workspace_digest()),
        context(offline)?,
        Some(result),
        Vec::new(),
    )?;
    Ok(CommandExecution {
        payload: OutputPayload {
            human: format!(
                "{entity_kind} {entity_id}\nsource: {source_path}\n{}",
                serde_json::to_string_pretty(&found.entity)?
            ),
            json: serde_json::from_slice(&envelope.to_json()?)?,
            sarif: None,
            markdown: None,
        },
        exit_code: 0,
    })
}

fn query(session: &PreparedSession, kind: &str, id: &str) -> Vec<Match> {
    let graph = session.finalized().graph();
    match kind {
        "capability" => graph
            .capabilities()
            .values()
            .filter(|value| value.id().as_str() == id)
            .map(|value| Match {
                source_key: format!("capability:{}", value.id()),
                entity: project_capability(value),
            })
            .collect(),
        "journey" => graph
            .journeys()
            .values()
            .filter(|value| value.id().as_str() == id)
            .map(|value| Match {
                source_key: format!("journey:{}", value.id()),
                entity: project_journey(value),
            })
            .collect(),
        "surface" => graph
            .surfaces()
            .values()
            .filter(|value| value.id().as_str() == id)
            .map(|value| Match {
                source_key: format!("surface:{}", value.id()),
                entity: project_surface(value),
            })
            .collect(),
        "fragment" => graph
            .fragments()
            .values()
            .filter(|value| value.id().as_str() == id)
            .map(|value| Match {
                source_key: format!("fragment:{}@{}", value.id(), value.revision()),
                entity: project_fragment(value),
            })
            .collect(),
        "target" => graph
            .targets()
            .values()
            .filter(|value| value.id().as_str() == id)
            .map(|value| Match {
                source_key: format!("target:{}", value.id()),
                entity: project_target(value),
            })
            .collect(),
        "binding" => graph
            .bindings()
            .values()
            .filter(|value| value.id().as_str() == id)
            .map(|value| Match {
                source_key: format!("binding:{}", value.id()),
                entity: project_binding(value),
            })
            .collect(),
        "policy" => graph
            .policies()
            .values()
            .filter(|value| value.id().as_str() == id)
            .map(|value| Match {
                source_key: format!("policy:{}@{}", value.id(), value.revision()),
                entity: project_policy(value),
            })
            .collect(),
        "profile" => graph
            .profiles()
            .values()
            .filter(|value| value.id().as_str() == id)
            .map(|value| Match {
                source_key: format!("profile:{}@{}", value.id(), value.revision()),
                entity: project_profile(value),
            })
            .collect(),
        "runner" => graph
            .runners()
            .values()
            .filter(|value| value.id().as_str() == id)
            .map(|value| Match {
                source_key: format!("runner:{}@{}", value.id(), value.revision()),
                entity: project_runner(value),
            })
            .collect(),
        "waiver" => graph
            .waivers()
            .values()
            .filter(|value| value.id().as_str() == id)
            .map(|value| Match {
                source_key: format!("waiver:{}@{}", value.id(), value.revision()),
                entity: project_waiver(value),
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn unresolved(
    offline: bool,
    digest: eqm_domain::Sha256Digest,
    kind: &str,
    id: &str,
    count: usize,
) -> Result<CommandExecution, Box<dyn Error>> {
    let detail = if count == 0 {
        "was not found".to_owned()
    } else {
        format!("matched {count} revisions")
    };
    let diagnostic = Diagnostic::new(
        DiagnosticCode::from_number(1).ok_or("diagnostic code")?,
        Severity::Error,
        format!("{kind} `{id}` {detail}"),
        None,
        Vec::new(),
        Some("Correct the entity kind or ID, or retain exactly one matching revision.".into()),
    )?;
    let envelope = ReportEnvelope::<ShowResultDto<Value>, (), ()>::new(
        CommandIdentity::Show,
        Some(digest),
        context(offline)?,
        None,
        vec![DiagnosticDto::from_domain(&diagnostic)],
    )?;
    Ok(CommandExecution {
        payload: OutputPayload {
            human: diagnostic.to_string(),
            json: serde_json::from_slice(&envelope.to_json()?)?,
            sarif: Some(serde_json::to_value(SarifLogDto::from_diagnostics(
                &[diagnostic],
                &diagnostic_registry()?,
            ))?),
            markdown: None,
        },
        exit_code: 2,
    })
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
    fn every_approved_kind_is_exact_source_located_and_missing_is_typed()
    -> Result<(), Box<dyn Error>> {
        let repository = crate::test_support::example_repository()?;
        let root = repository.path();
        for (kind, id) in [
            ("capability", "account.create"),
            ("journey", "account.create.signup"),
            ("surface", "account.create.signup.identifier"),
            ("fragment", "auth.otp_entry"),
            ("target", "android"),
            ("binding", "binding.android.auth_signup"),
            ("policy", "consumer.critical_flow"),
            ("profile", "audience.default"),
            ("runner", "runner.android"),
            ("waiver", "waiver.android.signup_email"),
        ] {
            let ParseOutcome::Run(parsed) =
                parse(["show", kind, id, "--format", "json", "--no-progress"])?
            else {
                return Err("unexpected help".into());
            };
            let result = execute(parsed, root)?;
            assert_eq!(result.exit_code, 0, "{kind} {id}");
            assert_eq!(result.payload.json["result"]["entity_kind"], kind);
            assert_eq!(result.payload.json["result"]["entity_id"], id);
            assert!(
                result.payload.json["result"]["source"]["uri"]
                    .as_str()
                    .is_some_and(|value| value.starts_with("file:"))
            );
            assert_eq!(result.payload.json["result"]["entity"]["id"], id);
        }

        let ParseOutcome::Run(parsed) = parse([
            "show",
            "target",
            "missing",
            "--format",
            "json",
            "--no-progress",
        ])?
        else {
            return Err("unexpected help".into());
        };
        let result = execute(parsed, root)?;
        assert_eq!(result.exit_code, 2);
        assert!(result.payload.json["result"].is_null());
        assert_eq!(result.payload.json["diagnostics"][0]["code"], "EQM-E0001");
        Ok(())
    }
}
