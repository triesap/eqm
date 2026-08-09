//! `validate` preparation, finalization, and reporting.

use super::CommandExecution;
use crate::cli::ParsedCli;
use crate::renderer::OutputPayload;
use crate::session::{SessionRequest, prepare};
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_domain::{Diagnostic, DiagnosticCode, Severity, UtcInstant};
use eqm_engine::diagnostic_registry;
use eqm_protocol::{
    CommandIdentity, DiagnosticDto, EvaluationModeDto, InvocationContextDto, ReportEnvelope,
    SarifLogDto, ValidateResultDto,
};
use serde_json::Value;
use std::collections::BTreeMap;
use std::error::Error;
use std::path::Path;
use std::time::SystemTime;

/// Runs validation without executing adapters/runners or writing state.
pub fn execute(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    let evaluated_at = evaluated_at()?;
    let offline = parsed.global.offline;
    let request = SessionRequest::new(parsed.global, parsed.command.name);
    match prepare(&request, start) {
        Ok(session) => {
            let graph = session.finalized().graph();
            let counts = BTreeMap::from([
                ("adapters".to_owned(), count(graph.adapters().len())?),
                ("bindings".to_owned(), count(graph.bindings().len())?),
                (
                    "capabilities".to_owned(),
                    count(graph.capabilities().len())?,
                ),
                ("fragments".to_owned(), count(graph.fragments().len())?),
                ("imports".to_owned(), count(graph.imports().len())?),
                ("journeys".to_owned(), count(graph.journeys().len())?),
                ("policies".to_owned(), count(graph.policies().len())?),
                ("profiles".to_owned(), count(graph.profiles().len())?),
                ("runners".to_owned(), count(graph.runners().len())?),
                ("surfaces".to_owned(), count(graph.surfaces().len())?),
                ("targets".to_owned(), count(graph.targets().len())?),
                ("waivers".to_owned(), count(graph.waivers().len())?),
            ]);
            let digest = session.workspace_digest();
            let result = ValidateResultDto {
                kind: CommandIdentity::Validate,
                valid: true,
                entity_counts: counts,
                graph_digest: digest.to_string(),
            };
            let envelope = ReportEnvelope::new(
                CommandIdentity::Validate,
                Some(digest),
                context(offline, evaluated_at)?,
                Some(result),
                Vec::new(),
            )?;
            let json: Value = serde_json::from_slice(&envelope.to_json()?)?;
            Ok(CommandExecution {
                payload: OutputPayload {
                    human: format!("valid\nworkspace_digest: {digest}"),
                    json,
                    sarif: Some(serde_json::to_value(SarifLogDto::from_diagnostics(
                        &[],
                        &diagnostic_registry()?,
                    ))?),
                    markdown: None,
                },
                exit_code: 0,
            })
        }
        Err(error) => {
            let diagnostic = Diagnostic::new(
                DiagnosticCode::from_number(100).ok_or("diagnostic code")?,
                Severity::Error,
                format!("workspace validation failed at {error}"),
                None,
                Vec::new(),
                Some("Correct the workspace authority and run validation again.".into()),
            )?;
            let envelope = ReportEnvelope::<ValidateResultDto, (), ()>::new(
                CommandIdentity::Validate,
                None,
                context(offline, evaluated_at)?,
                None,
                vec![DiagnosticDto::from_domain(&diagnostic)],
            )?;
            let json: Value = serde_json::from_slice(&envelope.to_json()?)?;
            Ok(CommandExecution {
                payload: OutputPayload {
                    human: format!("invalid\n{diagnostic}"),
                    json,
                    sarif: Some(serde_json::to_value(SarifLogDto::from_diagnostics(
                        &[diagnostic],
                        &diagnostic_registry()?,
                    ))?),
                    markdown: None,
                },
                exit_code: 3,
            })
        }
    }
}

fn context(
    offline: bool,
    evaluated_at: UtcInstant,
) -> Result<InvocationContextDto<(), ()>, Box<dyn Error>> {
    Ok(InvocationContextDto::new(
        EvaluationModeDto::Development,
        Vec::new(),
        None,
        None,
        offline,
        evaluated_at,
    )?)
}

fn evaluated_at() -> Result<UtcInstant, Box<dyn Error>> {
    let value: DateTime<Utc> = SystemTime::now().into();
    Ok(value.to_rfc3339_opts(SecondsFormat::Secs, true).parse()?)
}

fn count(value: usize) -> Result<u64, std::num::TryFromIntError> {
    u64::try_from(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{ParseOutcome, parse};

    #[test]
    fn valid_and_invalid_workspaces_return_exact_categories_without_writes()
    -> Result<(), Box<dyn Error>> {
        let repository = crate::test_support::example_repository()?;
        let root = repository.path();
        let ParseOutcome::Run(valid) = parse(["validate", "--format", "json", "--no-progress"])?
        else {
            return Err("unexpected help".into());
        };
        let valid = execute(valid, root)?;
        assert_eq!(valid.exit_code, 0);
        assert_eq!(valid.payload.json["command"], "validate");
        assert_eq!(valid.payload.json["result"]["valid"], true);
        assert!(
            valid.payload.json["workspace_digest"]
                .as_str()
                .is_some_and(|value| value.starts_with("sha256:"))
        );

        let ParseOutcome::Run(invalid) = parse([
            "validate",
            "--config",
            "missing.toml",
            "--format",
            "json",
            "--no-progress",
        ])?
        else {
            return Err("unexpected help".into());
        };
        let invalid = execute(invalid, root)?;
        assert_eq!(invalid.exit_code, 3);
        assert!(invalid.payload.json["result"].is_null());
        assert_eq!(invalid.payload.json["diagnostics"][0]["code"], "EQM-E0100");
        assert!(!root.join("missing.toml").exists());
        Ok(())
    }
}
