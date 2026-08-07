//! Current unresolved obligation query.

use super::{CommandExecution, evaluation};
use crate::cli::ParsedCli;
use crate::renderer::OutputPayload;
use crate::session::{SessionRequest, prepare};
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_domain::UtcInstant;
use eqm_engine::ScopeSubject;
use eqm_protocol::{
    CommandIdentity, EvaluationModeDto, InvocationContextDto, ObligationsResultDto, ReportEnvelope,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::path::Path;
use std::time::SystemTime;

/// Lists current unresolved obligations without reading or executing evidence.
pub fn execute(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    let offline = parsed.global.offline;
    let profiles = parsed.global.profiles.clone();
    let unit = option_value(&parsed, "--unit").map(ToOwned::to_owned);
    let target = option_value(&parsed, "--target").map(ToOwned::to_owned);
    let statuses = option_values(&parsed, "--status");
    let request = SessionRequest::new(parsed.global, parsed.command.name);
    let session = prepare(&request, start)?;
    let (_, derived) = evaluation::derive(&session, &profiles)?;
    let include_missing = statuses.is_empty() || statuses.contains("missing");
    let obligations = if include_missing {
        derived
            .obligations
            .values()
            .filter(|value| {
                unit.as_deref()
                    .is_none_or(|unit| value.key.unit.as_str() == unit)
                    && target
                        .as_deref()
                        .is_none_or(|target| subject_matches(&value.key.subject, target))
            })
            .map(evaluation::obligation_dto)
            .collect::<Result<BTreeSet<_>, _>>()?
    } else {
        BTreeSet::new()
    };
    let mut filters = BTreeMap::new();
    if let Some(unit) = &unit {
        filters.insert("unit".to_owned(), unit.clone());
    }
    if let Some(target) = &target {
        filters.insert("target".to_owned(), target.clone());
    }
    if !statuses.is_empty() {
        filters.insert(
            "status".to_owned(),
            statuses.iter().cloned().collect::<Vec<_>>().join(","),
        );
    }
    let count = obligations.len();
    let result = ObligationsResultDto {
        kind: CommandIdentity::Obligations,
        filters,
        obligations,
    };
    let envelope = ReportEnvelope::new(
        CommandIdentity::Obligations,
        Some(session.workspace_digest()),
        context(offline)?,
        Some(result),
        Vec::new(),
    )?;
    Ok(CommandExecution {
        payload: OutputPayload {
            human: format!("{count} unresolved obligations"),
            json: serde_json::from_slice(&envelope.to_json()?)?,
            sarif: None,
            markdown: None,
        },
        exit_code: 0,
    })
}

fn subject_matches(subject: &ScopeSubject, target: &str) -> bool {
    match subject {
        ScopeSubject::Target(value) => value.as_str() == target,
        ScopeSubject::TargetSet(values) => values.iter().any(|value| value.as_str() == target),
        ScopeSubject::Provider(_) => true,
    }
}

fn option_value<'a>(parsed: &'a ParsedCli, name: &str) -> Option<&'a str> {
    parsed.command.options.get(name)?.first()?.as_deref()
}

fn option_values(parsed: &ParsedCli, name: &str) -> BTreeSet<String> {
    parsed
        .command
        .options
        .get(name)
        .into_iter()
        .flatten()
        .filter_map(Clone::clone)
        .collect()
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
    fn missing_and_noncurrent_status_filters_are_stable() -> Result<(), Box<dyn Error>> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let ParseOutcome::Run(missing) = parse([
            "obligations",
            "--unit",
            "account.create.signup.identifier",
            "--target",
            "web",
            "--status",
            "missing",
            "--format",
            "json",
            "--no-progress",
        ])?
        else {
            return Err("unexpected help".into());
        };
        let missing = execute(missing, &root)?;
        assert!(
            !missing.payload.json["result"]["obligations"]
                .as_array()
                .ok_or("obligations")?
                .is_empty()
        );
        assert!(
            missing.payload.json["result"]["obligations"]
                .as_array()
                .ok_or("obligations")?
                .iter()
                .all(|value| value["status"] == "missing")
        );

        let ParseOutcome::Run(stale) = parse([
            "obligations",
            "--status",
            "stale",
            "--format",
            "json",
            "--no-progress",
        ])?
        else {
            return Err("unexpected help".into());
        };
        let stale = execute(stale, &root)?;
        assert!(
            stale.payload.json["result"]["obligations"]
                .as_array()
                .ok_or("obligations")?
                .is_empty()
        );
        Ok(())
    }
}
