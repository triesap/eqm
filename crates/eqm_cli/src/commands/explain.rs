//! Stable workspace-independent diagnostic registry query.

use super::CommandExecution;
use crate::cli::ParsedCli;
use crate::renderer::OutputPayload;
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_domain::{DiagnosticCode, UtcInstant};
use eqm_engine::explain_diagnostic;
use eqm_protocol::{
    CommandIdentity, EvaluationModeDto, ExplainResultDto, InvocationContextDto, ReportEnvelope,
};
use std::error::Error;
use std::time::SystemTime;

/// Explains one exact allocated and registered diagnostic code.
pub fn execute(parsed: ParsedCli) -> Result<CommandExecution, Box<dyn Error>> {
    let raw = parsed
        .command
        .operands
        .first()
        .ok_or("diagnostic code required")?;
    let code = raw.parse::<DiagnosticCode>();
    let descriptor = code
        .ok()
        .and_then(|value| explain_diagnostic(value).ok().flatten());
    let result = descriptor.map(|value| ExplainResultDto {
        kind: CommandIdentity::Explain,
        code: value.code.to_string(),
        title: value.title.to_owned(),
        authority: value.authority.to_owned(),
        explanation: value.explanation.to_owned(),
        remediation: value.remediation.to_owned(),
    });
    let evaluated_at = evaluated_at()?;
    let envelope = ReportEnvelope::new(
        CommandIdentity::Explain,
        None,
        InvocationContextDto::<(), ()>::new(
            EvaluationModeDto::Development,
            Vec::new(),
            None,
            None,
            parsed.global.offline,
            evaluated_at,
        )?,
        result.clone(),
        Vec::new(),
    )?;
    let human = result.as_ref().map_or_else(
        || format!("diagnostic not found: {raw}"),
        |value| {
            format!(
                "{}: {}\nauthority: {}\n{}\nremediation: {}",
                value.code, value.title, value.authority, value.explanation, value.remediation
            )
        },
    );
    Ok(CommandExecution {
        payload: OutputPayload {
            human,
            json: serde_json::from_slice(&envelope.to_json()?)?,
            sarif: None,
            markdown: None,
        },
        exit_code: if result.is_some() { 0 } else { 2 },
    })
}

fn evaluated_at() -> Result<UtcInstant, Box<dyn Error>> {
    let value: DateTime<Utc> = SystemTime::now().into();
    Ok(value.to_rfc3339_opts(SecondsFormat::Secs, true).parse()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{GlobalOptions, ParsedCommand};
    use std::collections::BTreeMap;

    fn parsed(code: &str) -> ParsedCli {
        ParsedCli {
            global: GlobalOptions::default(),
            command: ParsedCommand {
                name: crate::cli::CommandName::Explain,
                operands: vec![code.to_owned()],
                options: BTreeMap::new(),
            },
        }
    }

    #[test]
    fn every_registered_code_is_explainable_and_unknown_is_usage() -> Result<(), Box<dyn Error>> {
        for descriptor in eqm_engine::diagnostic_registry()? {
            let execution = execute(parsed(&descriptor.code.to_string()))?;
            assert_eq!(execution.exit_code, 0);
            assert!(execution.payload.human.contains(descriptor.title));
        }
        assert_eq!(execute(parsed("EQM-E0400"))?.exit_code, 2);
        assert_eq!(execute(parsed("invalid"))?.exit_code, 2);
        Ok(())
    }
}
