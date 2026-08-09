//! Binding source, artifact, and evidence declaration query.

use super::{CommandExecution, source_location};
use crate::cli::ParsedCli;
use crate::renderer::OutputPayload;
use crate::session::{SessionRequest, prepare};
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_domain::{Diagnostic, DiagnosticCode, Severity, UtcInstant};
use eqm_engine::diagnostic_registry;
use eqm_protocol::{
    CommandIdentity, DiagnosticDto, EvaluationModeDto, InvocationContextDto, LocateResultDto,
    LocationDto, ReportEnvelope, SarifLogDto,
};
use serde_json::Value;
use std::collections::BTreeSet;
use std::error::Error;
use std::path::Path;
use std::time::SystemTime;

/// Locates all declarations for one unit and an optional exact target.
pub fn execute(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    let offline = parsed.global.offline;
    let unit = parsed.command.operands[0].clone();
    let target = option_value(&parsed, "--target").map(ToOwned::to_owned);
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
        return unresolved(
            offline,
            session.workspace_digest(),
            &unit,
            target.as_deref(),
        );
    }

    let mut locations = BTreeSet::new();
    for binding in bindings {
        let source_path = session
            .source_map()
            .get(format!("binding:{}", binding.id()).as_str())
            .ok_or("binding source mapping")?;
        let source = source_location(source_path)?;
        locations.insert(LocationDto {
            role: "source".to_owned(),
            path: source_path.to_string(),
            symbol: Some(binding.id().to_string()),
            source: Some(source.clone()),
        });
        for artifact in binding.artifacts().values().values() {
            locations.insert(LocationDto {
                role: format!("artifact/{}", artifact.role().as_str()),
                path: artifact.path().to_string(),
                symbol: artifact.symbol().map(ToString::to_string),
                source: Some(source.clone()),
            });
        }
        for evidence in binding.evidence().values() {
            locations.insert(LocationDto {
                role: format!("evidence/{}", evidence.kind().as_str()),
                path: source_path.to_string(),
                symbol: Some(evidence.id().to_string()),
                source: Some(source.clone()),
            });
        }
    }
    let result = LocateResultDto {
        kind: CommandIdentity::Locate,
        unit: unit.clone(),
        target: target.clone(),
        locations,
    };
    let envelope = ReportEnvelope::new(
        CommandIdentity::Locate,
        Some(session.workspace_digest()),
        context(offline)?,
        Some(result),
        Vec::new(),
    )?;
    let json: Value = serde_json::from_slice(&envelope.to_json()?)?;
    let count = json["result"]["locations"].as_array().map_or(0, Vec::len);
    Ok(CommandExecution {
        payload: OutputPayload {
            human: format!("located {count} declarations for {unit}"),
            json,
            sarif: None,
            markdown: None,
        },
        exit_code: 0,
    })
}

fn unresolved(
    offline: bool,
    digest: eqm_domain::Sha256Digest,
    unit: &str,
    target: Option<&str>,
) -> Result<CommandExecution, Box<dyn Error>> {
    let coordinate = target.map_or_else(|| unit.to_owned(), |value| format!("{unit} on {value}"));
    let diagnostic = Diagnostic::new(
        DiagnosticCode::from_number(1).ok_or("diagnostic code")?,
        Severity::Error,
        format!("unit `{coordinate}` has no binding authority"),
        None,
        Vec::new(),
        Some("Correct the unit or target query operand.".into()),
    )?;
    let envelope = ReportEnvelope::<LocateResultDto, (), ()>::new(
        CommandIdentity::Locate,
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

fn option_value<'a>(parsed: &'a ParsedCli, name: &str) -> Option<&'a str> {
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
    fn signup_locations_are_sorted_source_linked_and_missing_is_typed() -> Result<(), Box<dyn Error>>
    {
        let repository = crate::test_support::example_repository()?;
        let root = repository.path();
        let ParseOutcome::Run(parsed) = parse([
            "locate",
            "account.create.signup.identifier",
            "--target",
            "android",
            "--format",
            "json",
            "--no-progress",
        ])?
        else {
            return Err("unexpected help".into());
        };
        let result = execute(parsed, root)?;
        assert_eq!(result.exit_code, 0);
        let locations = result.payload.json["result"]["locations"]
            .as_array()
            .ok_or("locations")?;
        assert_eq!(locations.len(), 3);
        assert!(locations.iter().any(|value| value["role"] == "source"));
        assert!(locations.iter().any(|value| {
            value["path"] == "apps/android/app/src/main/java/com/example/signup/SignupScreen.kt"
        }));
        assert!(locations.iter().all(|value| {
            value["source"]["uri"] == "file:eqm/bindings/android.auth_signup.toml"
        }));

        let ParseOutcome::Run(parsed) = parse([
            "locate",
            "account.create.missing",
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
