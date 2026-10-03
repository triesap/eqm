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
    use std::fs;

    const V1_WORKSPACE: &str = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/workspace.schema.json";
    const V1_LOCK: &str =
        "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/lock.schema.json";

    fn run_validate(root: &Path) -> Result<u8, Box<dyn Error>> {
        let ParseOutcome::Run(parsed) = parse(["validate", "--format", "json", "--no-progress"])?
        else {
            return Err("unexpected help".into());
        };
        Ok(execute(parsed, root)?.exit_code)
    }

    #[test]
    fn unsupported_workspace_and_lock_uris_fail_through_actual_preparation()
    -> Result<(), Box<dyn Error>> {
        // Control: the unmutated v1 workspace and lock validate successfully.
        let repository = crate::test_support::example_repository()?;
        assert_eq!(run_validate(repository.path())?, 0);

        // Workspace-only URI mutation to v3/wrong kind must reject through the
        // real CLI preparation boundary, not only a unit helper. The additive
        // v2 workspace family is accepted by its own reader.
        for mutation in [
            "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v3/manifest/workspace.schema.json",
            V1_LOCK,
        ] {
            let repository = crate::test_support::example_repository()?;
            let config = fs::read_to_string(repository.path().join("eqm.toml"))?;
            fs::write(
                repository.path().join("eqm.toml"),
                config.replace(V1_WORKSPACE, mutation),
            )?;
            assert_eq!(
                run_validate(repository.path())?,
                3,
                "accepted workspace schema {mutation}"
            );
        }

        // Lock-only URI mutation rejects too.
        for mutation in [
            "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/lock.schema.json",
            "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v3/manifest/lock.schema.json",
            V1_WORKSPACE,
        ] {
            let repository = crate::test_support::example_repository()?;
            let lock = fs::read_to_string(repository.path().join("eqm.lock"))?;
            fs::write(
                repository.path().join("eqm.lock"),
                lock.replace(V1_LOCK, mutation),
            )?;
            assert_eq!(
                run_validate(repository.path())?,
                3,
                "accepted lock schema {mutation}"
            );
        }
        Ok(())
    }

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

    #[test]
    fn asymmetric_fixture_is_admitted_and_cohort_failure_is_diagnosed() -> Result<(), Box<dyn Error>>
    {
        let repository = crate::test_support::asymmetric_repository()?;
        let root = repository.path();
        let ParseOutcome::Run(valid) = parse(["validate", "--format", "json", "--no-progress"])?
        else {
            return Err("unexpected help".into());
        };
        let valid = execute(valid, root)?;
        assert_eq!(valid.exit_code, 0);
        assert_eq!(valid.payload.json["result"]["valid"], true);
        assert_eq!(valid.payload.json["result"]["entity_counts"]["targets"], 3);

        // A mismatched authored cohort configuration is admitted as a diagnostic
        // rather than silently substituted by the graph projection.
        let cohort = root.join("eqm/contracts/cohort.n.toml");
        let text = fs::read_to_string(&cohort)?;
        let other = eqm_domain::Sha256Digest::hash_content(b"other");
        let mutated = text
            .lines()
            .map(|line| {
                if line.starts_with("provider_configuration_digest") {
                    format!("provider_configuration_digest = \"{other}\"")
                } else {
                    line.to_owned()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(&cohort, mutated)?;

        let ParseOutcome::Run(invalid) = parse(["validate", "--format", "json", "--no-progress"])?
        else {
            return Err("unexpected help".into());
        };
        let invalid = execute(invalid, root)?;
        assert_eq!(invalid.exit_code, 3);
        assert_eq!(invalid.payload.json["diagnostics"][0]["code"], "EQM-E0100");
        assert!(
            invalid.payload.json["diagnostics"][0]["message"]
                .as_str()
                .is_some_and(|message| message.contains("cohort"))
        );
        Ok(())
    }
}
