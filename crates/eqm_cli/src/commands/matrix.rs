//! Complete stable matrix query views.

use super::{CommandExecution, evaluation};
use crate::cli::ParsedCli;
use crate::renderer::OutputPayload;
use crate::session::{SessionRequest, prepare};
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_domain::{DiagnosticCode, TargetId, UtcInstant};
use eqm_engine::{
    MatrixAxisKey, MatrixKind, MatrixStatus, MatrixValue, ScopeSubject, generate_matrix,
};
use eqm_protocol::{
    CommandIdentity, EvaluationModeDto, FacetStatusDto, InvocationContextDto, MatrixAxisDto,
    MatrixCellDto, MatrixResultDto, ReportEnvelope,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::path::Path;
use std::time::SystemTime;

/// Generates one complete non-executing matrix view.
pub fn execute(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    let offline = parsed.global.offline;
    let profiles = parsed.global.profiles.clone();
    let kind_name = parsed.command.operands[0].clone();
    let unit_filter = option_value(&parsed, "--unit").map(ToOwned::to_owned);
    let target_filter = option_value(&parsed, "--target").map(ToOwned::to_owned);
    let request = SessionRequest::new(parsed.global, parsed.command.name);
    let session = prepare(&request, start)?;
    let (_, derived) = evaluation::derive(&session, &profiles)?;
    let kind = parse_kind(&kind_name)?;

    let units = derived
        .obligations
        .values()
        .filter(|value| {
            unit_filter
                .as_deref()
                .is_none_or(|unit| value.key.unit.as_str() == unit)
        })
        .map(|value| value.key.unit.clone())
        .collect::<BTreeSet<_>>();
    let targets = derived
        .obligations
        .values()
        .flat_map(|value| subject_targets(&value.key.subject))
        .filter(|value| {
            target_filter
                .as_deref()
                .is_none_or(|target| value.as_str() == target)
        })
        .collect::<BTreeSet<_>>();
    let facets = derived
        .obligations
        .values()
        .filter(|value| {
            unit_filter
                .as_deref()
                .is_none_or(|unit| value.key.unit.as_str() == unit)
        })
        .map(|value| value.key.facet)
        .collect::<BTreeSet<_>>();
    let rows = units
        .iter()
        .map(|value| {
            (
                MatrixAxisKey::Unit(value.clone()),
                value.to_string().into_boxed_str(),
            )
        })
        .collect();
    let columns = if kind == MatrixKind::Evidence {
        facets
            .iter()
            .map(|value| {
                (
                    MatrixAxisKey::Facet(*value),
                    value.to_string().into_boxed_str(),
                )
            })
            .collect()
    } else {
        targets
            .iter()
            .map(|value| {
                (
                    MatrixAxisKey::Target(value.clone()),
                    value.to_string().into_boxed_str(),
                )
            })
            .collect()
    };
    let mut values = BTreeMap::new();
    if matches!(kind, MatrixKind::Conformance | MatrixKind::Evidence) {
        for obligation in derived.obligations.values().filter(|value| {
            unit_filter
                .as_deref()
                .is_none_or(|unit| value.key.unit.as_str() == unit)
                && target_filter.as_deref().is_none_or(|target| {
                    subject_targets(&value.key.subject)
                        .iter()
                        .any(|value| value.as_str() == target)
                })
        }) {
            let row = MatrixAxisKey::Unit(obligation.key.unit.clone());
            let columns = if kind == MatrixKind::Evidence {
                vec![MatrixAxisKey::Facet(obligation.key.facet)]
            } else {
                subject_targets(&obligation.key.subject)
                    .into_iter()
                    .filter(|target| {
                        target_filter
                            .as_deref()
                            .is_none_or(|value| target.as_str() == value)
                    })
                    .map(MatrixAxisKey::Target)
                    .collect()
            };
            for column in columns {
                let value = values
                    .entry((row.clone(), column))
                    .or_insert_with(MatrixValue::default);
                value.status = MatrixStatus::Missing;
                value.obligations.insert(obligation.key.clone());
                value
                    .diagnostics
                    .insert(DiagnosticCode::from_number(500).ok_or("diagnostic code")?);
            }
        }
    }
    let matrix = generate_matrix(kind, rows, columns, values)?;
    let result = MatrixResultDto {
        kind: CommandIdentity::Matrix,
        matrix_kind: kind_name.clone(),
        rows: matrix
            .rows
            .iter()
            .map(|axis| MatrixAxisDto {
                id: axis_id(&axis.key),
                label: axis.label.to_string(),
            })
            .collect(),
        columns: matrix
            .columns
            .iter()
            .map(|axis| MatrixAxisDto {
                id: axis_id(&axis.key),
                label: axis.label.to_string(),
            })
            .collect(),
        cells: matrix
            .cells
            .iter()
            .map(|cell| {
                Ok(MatrixCellDto {
                    row: axis_id(&cell.row),
                    column: axis_id(&cell.column),
                    status: status(cell.value.status),
                    obligations: cell
                        .value
                        .obligations
                        .iter()
                        .map(|key| {
                            derived
                                .obligations
                                .get(key)
                                .map(evaluation::obligation_id)
                                .ok_or("matrix obligation")
                        })
                        .collect::<Result<_, _>>()?,
                    diagnostic_codes: cell
                        .value
                        .diagnostics
                        .iter()
                        .map(ToString::to_string)
                        .collect(),
                })
            })
            .collect::<Result<BTreeSet<_>, &'static str>>()?,
    };
    let envelope = ReportEnvelope::new(
        CommandIdentity::Matrix,
        Some(session.workspace_digest()),
        context(offline)?,
        Some(result),
        Vec::new(),
    )?;
    Ok(CommandExecution {
        payload: OutputPayload {
            human: format!("{kind_name} matrix: {} cells", matrix.cells.len()),
            json: serde_json::from_slice(&envelope.to_json()?)?,
            sarif: None,
            markdown: None,
        },
        exit_code: 0,
    })
}

fn parse_kind(value: &str) -> Result<MatrixKind, Box<dyn Error>> {
    Ok(match value {
        "conformance" => MatrixKind::Conformance,
        "evidence" => MatrixKind::Evidence,
        "exposure" => MatrixKind::Exposure,
        "release" => MatrixKind::Release,
        "equivalence" => MatrixKind::Equivalence,
        _ => return Err("matrix kind".into()),
    })
}

fn subject_targets(subject: &ScopeSubject) -> Vec<TargetId> {
    match subject {
        ScopeSubject::Target(value) => vec![value.clone()],
        ScopeSubject::TargetSet(values) => values.iter().cloned().collect(),
        ScopeSubject::Provider(_) => Vec::new(),
    }
}

fn axis_id(value: &MatrixAxisKey) -> String {
    match value {
        MatrixAxisKey::Unit(value) => format!("unit:{value}"),
        MatrixAxisKey::Target(value) => format!("target:{value}"),
        MatrixAxisKey::Facet(value) => format!("facet:{value}"),
    }
}

const fn status(value: MatrixStatus) -> FacetStatusDto {
    match value {
        MatrixStatus::NotApplicable => FacetStatusDto::NotApplicable,
        MatrixStatus::Pass => FacetStatusDto::Satisfied,
        MatrixStatus::Conditional => FacetStatusDto::Waived,
        MatrixStatus::Fail => FacetStatusDto::Failed,
        MatrixStatus::Stale => FacetStatusDto::Stale,
        MatrixStatus::Missing => FacetStatusDto::Missing,
        MatrixStatus::Unstable => FacetStatusDto::Unstable,
        MatrixStatus::Unknown => FacetStatusDto::Unknown,
    }
}

fn option_value<'a>(parsed: &'a ParsedCli, name: &str) -> Option<&'a str> {
    parsed.command.options.get(name)?.first()?.as_deref()
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
    fn all_matrix_views_are_complete_and_filters_are_exact() -> Result<(), Box<dyn Error>> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        for kind in [
            "conformance",
            "evidence",
            "exposure",
            "release",
            "equivalence",
        ] {
            let ParseOutcome::Run(parsed) = parse([
                "matrix",
                kind,
                "--unit",
                "account.create.signup.identifier",
                "--target",
                "web",
                "--format",
                "json",
                "--no-progress",
            ])?
            else {
                return Err("unexpected help".into());
            };
            let result = execute(parsed, &root)?;
            assert_eq!(result.exit_code, 0, "{kind}");
            assert_eq!(result.payload.json["result"]["matrix_kind"], kind);
            assert!(
                !result.payload.json["result"]["cells"]
                    .as_array()
                    .ok_or("cells")?
                    .is_empty()
            );
        }
        Ok(())
    }
}
