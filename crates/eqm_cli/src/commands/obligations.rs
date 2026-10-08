//! Complete pre-evidence obligation query, with explicit v2 wire/error records.
use super::{CommandExecution, evaluation};
use crate::cli::{OutputFormat, ParsedCli};
use crate::renderer::OutputPayload;
use crate::session::{SessionError, SessionRequest, prepare};
use eqm_application::{QueryPreparationError, prepare_obligation_query};
use eqm_protocol::*;
use std::error::Error;
use std::path::Path;

struct QueryCommandError {
    code: ObligationQueryErrorCode,
    message: String,
    source: Option<String>,
}
fn failed(code: ObligationQueryErrorCode, error: impl std::fmt::Display) -> QueryCommandError {
    QueryCommandError {
        code,
        message: error.to_string().chars().take(4096).collect(),
        source: None,
    }
}
/// Renders complete scope without reading/evaluating evidence or writing workspace state.
pub fn execute(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    if !matches!(
        parsed.global.format,
        OutputFormat::Human | OutputFormat::Json
    ) {
        return Err("obligations supports human and json output".into());
    }
    match execute_query(parsed, start) {
        Ok(value) => Ok(value),
        Err(error) => {
            let human = format!("obligations error {:?}: {}", error.code, error.message);
            let report = ObligationsReportV2Dto::Error {
                schema: QuerySchemaV2,
                tool_version: ToolVersionDto,
                command: ObligationQueryCommand::Obligations,
                error: ObligationQueryFailureV2 {
                    code: error.code,
                    message: error.message,
                    source: error.source,
                },
            };
            Ok(CommandExecution {
                payload: OutputPayload {
                    human,
                    json: serde_json::from_slice(&report.to_json()?)?,
                    sarif: None,
                    markdown: None,
                },
                exit_code: 6,
            })
        }
    }
}
fn execute_query(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, QueryCommandError> {
    let invalid = |e| failed(ObligationQueryErrorCode::InvalidRequest, e);
    let statuses = parsed
        .command
        .options
        .get("--status")
        .into_iter()
        .flatten()
        .filter_map(|v| v.as_deref())
        .map(|value| match value {
            "pending" => Ok(ObligationQueryState::Pending),
            "unknown" => Ok(ObligationQueryState::Unknown),
            "not_applicable" => Ok(ObligationQueryState::NotApplicable),
            _ => Err(invalid(
                "pre-evidence status must be pending, unknown or not_applicable",
            )),
        })
        .collect::<Result<Vec<_>, _>>()?;
    let filters = ObligationQueryFiltersV2 {
        unit: option_value(&parsed, "--unit").map(ToOwned::to_owned),
        target: option_value(&parsed, "--target").map(ToOwned::to_owned),
        statuses: CanonicalSet::from_members(statuses)
            .map_err(|e| failed(ObligationQueryErrorCode::InvalidRequest, e))?,
    };
    filters
        .validate()
        .map_err(|e| failed(ObligationQueryErrorCode::InvalidRequest, e))?;
    let offline = parsed.global.offline;
    let policy = parsed.global.policy.clone();
    let profiles = parsed.global.profiles.clone();
    let scenarios = parsed.global.scenarios.clone();
    let request = SessionRequest::new(parsed.global, parsed.command.name);
    let session = prepare(&request, start).map_err(|e| {
        let code = match &e {
            SessionError::ConfigPath(_) => ObligationQueryErrorCode::InvalidRequest,
            SessionError::Load(error) => load_error_code(error),
            SessionError::Resolve(_) | SessionError::Expand(_) | SessionError::Cohort(_) => {
                ObligationQueryErrorCode::InvalidAuthority
            }
            _ => ObligationQueryErrorCode::InternalFailure,
        };
        let source = match &e {
            SessionError::Load(eqm_manifest::LoadError::Validation(v)) => {
                v.source().map(ToString::to_string)
            }
            _ => None,
        };
        let mut failure = failed(code, e);
        failure.source = source;
        failure
    })?;
    let (_, derived) =
        evaluation::derive_with_policy(&session, policy.as_deref(), &profiles, &scenarios)
            .map_err(|e| failed(ObligationQueryErrorCode::InvalidAuthority, e))?;
    let time = session
        .evaluation_time()
        .to_string()
        .parse()
        .map_err(|e| failed(ObligationQueryErrorCode::InternalFailure, e))?;
    let prepared =
        prepare_obligation_query(session.finalized(), derived.identities(), time, offline)
            .map_err(projection_error)?;
    // The complete view must fit its actual CLI framing before a filter can
    // hide work and make an otherwise oversized result appear available.
    query_payload(
        &prepared
            .view(ObligationQueryFiltersV2 {
                unit: None,
                target: None,
                statuses: CanonicalSet::empty(),
            })
            .map_err(projection_error)?,
        String::new(),
    )?;
    let report = prepared.view(filters).map_err(projection_error)?;
    let human = match &report {
        ObligationsReportV2Dto::Success { result, .. } => format!(
            "{} required coordinates; {} unresolved before evidence; {} shown",
            result.denominator.required.get(),
            result.denominator.unresolved.get(),
            result.returned_count.get()
        ),
        ObligationsReportV2Dto::Error { .. } => {
            return Err(failed(
                ObligationQueryErrorCode::InternalFailure,
                "query returned an unexpected error form",
            ));
        }
    };
    Ok(CommandExecution {
        payload: query_payload(&report, human)?,
        exit_code: 0,
    })
}
fn query_payload(
    report: &ObligationsReportV2Dto,
    human: String,
) -> Result<OutputPayload, QueryCommandError> {
    let bytes = report.to_json().map_err(|e| {
        failed(
            if e == ObligationQueryError::Limits {
                ObligationQueryErrorCode::ResourceLimit
            } else {
                ObligationQueryErrorCode::InternalFailure
            },
            e,
        )
    })?;
    let payload = OutputPayload {
        human,
        json: serde_json::from_slice(&bytes)
            .map_err(|e| failed(ObligationQueryErrorCode::InternalFailure, e))?,
        sarif: None,
        markdown: None,
    };
    let rendered = crate::renderer::render(&payload, OutputFormat::Json)
        .map_err(|e| failed(ObligationQueryErrorCode::InternalFailure, e))?;
    if rendered.bytes().len() > MAX_OBLIGATION_QUERY_BYTES {
        return Err(failed(
            ObligationQueryErrorCode::ResourceLimit,
            "complete rendered query exceeds byte ceiling",
        ));
    }
    Ok(payload)
}

fn projection_error(error: QueryPreparationError) -> QueryCommandError {
    let code = match &error {
        QueryPreparationError::Limit
        | QueryPreparationError::Protocol(ObligationQueryError::Limits) => {
            ObligationQueryErrorCode::ResourceLimit
        }
        QueryPreparationError::ForeignGraph
        | QueryPreparationError::MissingIdentity
        | QueryPreparationError::Scope(_) => ObligationQueryErrorCode::InvalidAuthority,
        _ => ObligationQueryErrorCode::InternalFailure,
    };
    failed(code, error)
}
fn option_value<'a>(parsed: &'a ParsedCli, name: &str) -> Option<&'a str> {
    parsed.command.options.get(name)?.first()?.as_deref()
}

fn load_error_code(error: &eqm_manifest::LoadError) -> ObligationQueryErrorCode {
    use eqm_manifest::{ConfigError, DiscoveryError, LoadError, LockError, ValidationErrorKind};
    match error {
        LoadError::Config(
            ConfigError::StartUnavailable
            | ConfigError::RepositoryNotFound
            | ConfigError::ConfigUnavailable
            | ConfigError::Discovery,
        )
        | LoadError::Discovery(DiscoveryError::Filesystem)
        | LoadError::Lock(LockError::Unavailable) => ObligationQueryErrorCode::UnavailableWorkspace,
        LoadError::Validation(v) => match v.kind() {
            ValidationErrorKind::Filesystem => ObligationQueryErrorCode::UnavailableWorkspace,
            ValidationErrorKind::TooManyDocuments
            | ValidationErrorKind::TotalBytesExceeded
            | ValidationErrorKind::NestingExceeded
            | ValidationErrorKind::StringExceeded
            | ValidationErrorKind::ContainerExceeded => ObligationQueryErrorCode::ResourceLimit,
            _ => ObligationQueryErrorCode::InvalidAuthority,
        },
        _ => ObligationQueryErrorCode::InvalidAuthority,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{ParseOutcome, parse};
    #[test]
    fn valid_complete_envelope_reserves_actual_renderer_framing() -> Result<(), Box<dyn Error>> {
        let root = crate::test_support::example_repository()?;
        let ParseOutcome::Run(parsed) =
            parse(["obligations", "--format", "json", "--no-progress"])?
        else {
            return Err("help".into());
        };
        let actual = execute(parsed, root.path())?;
        let mut report: ObligationsReportV2Dto = serde_json::from_value(actual.payload.json)?;
        let ObligationsReportV2Dto::Success { result, .. } = &mut report else {
            return Err("success".into());
        };
        let template = result
            .records
            .iter()
            .find_map(|r| match r {
                ObligationViewV2Dto::Required { coordinate, .. } => Some(coordinate.clone()),
                _ => None,
            })
            .ok_or("coordinate")?;
        let record_size = serde_json::to_vec(&ObligationViewV2Dto::Required {
            state: ObligationQueryState::Pending,
            reference: template.reference()?,
            coordinate: template.clone(),
        })?
        .len()
            + 1;
        let count = (MAX_OBLIGATION_QUERY_BYTES - 4096) / record_size;
        let mut records = Vec::new();
        for index in 0..count {
            let mut coordinate = template.clone();
            coordinate.unit = format!(
                "boundary.unit{index:06}{}",
                "a".repeat(template.unit.len().checked_sub(19).ok_or("unit length")?)
            );
            if index == 0 {
                coordinate.policy_selection.policy.extensions.insert(
                    "org.eqm.boundary".into(),
                    eqm_protocol::schema_graph_v2::ExtensionValueV2Dto::String("a".into()),
                );
            }
            records.push(ObligationViewV2Dto::Required {
                state: ObligationQueryState::Pending,
                reference: coordinate.reference()?,
                coordinate,
            });
        }
        result.denominator = ObligationQueryDenominatorV2 {
            required: WireUint::new(count as u64),
            unresolved: WireUint::new(0),
        };
        result.returned_count = WireUint::new(count as u64);
        result.filters = ObligationQueryFiltersV2 {
            unit: None,
            target: None,
            statuses: CanonicalSet::empty(),
        };
        result.presentation_filtered = false;
        result.records = CanonicalSet::from_members(records.clone())?;
        let unpadded = report.to_json()?.len();
        let padding = MAX_OBLIGATION_QUERY_BYTES
            .checked_sub(unpadded + 1)
            .ok_or("base envelope too large")?;
        assert!(padding < 16 * 1024);
        for extra in [0, 1] {
            let ObligationViewV2Dto::Required {
                coordinate,
                reference,
                ..
            } = &mut records[0]
            else {
                return Err("required".into());
            };
            coordinate.policy_selection.policy.extensions.insert(
                "org.eqm.boundary".into(),
                eqm_protocol::schema_graph_v2::ExtensionValueV2Dto::String(
                    "a".repeat(padding + 1 + extra),
                ),
            );
            *reference = coordinate.reference()?;
            let ObligationsReportV2Dto::Success { result, .. } = &mut report else {
                return Err("success".into());
            };
            result.records = CanonicalSet::from_members(records.clone())?;
            let bytes = report.to_json()?;
            assert_eq!(bytes.len(), MAX_OBLIGATION_QUERY_BYTES - 1 + extra);
            ObligationsReportV2Dto::from_json(&bytes)?;
            match query_payload(&report, String::new()) {
                Ok(payload) => {
                    assert_eq!(extra, 0);
                    let rendered = crate::renderer::render(&payload, OutputFormat::Json)?;
                    assert_eq!(rendered.bytes().len(), MAX_OBLIGATION_QUERY_BYTES);
                    ObligationsReportV2Dto::from_json(rendered.bytes())?;
                }
                Err(error) => {
                    assert_eq!(extra, 1);
                    assert_eq!(error.code, ObligationQueryErrorCode::ResourceLimit);
                }
            }
        }
        Ok(())
    }
    #[test]
    fn pending_is_truthful_and_evaluated_status_filters_reject() -> Result<(), Box<dyn Error>> {
        let repository = crate::test_support::example_repository()?;
        let ParseOutcome::Run(pending) = parse([
            "obligations",
            "--unit",
            "account.create.signup.identifier",
            "--target",
            "android",
            "--status",
            "pending",
            "--format",
            "json",
            "--no-progress",
        ])?
        else {
            return Err("help".into());
        };
        let pending = execute(pending, repository.path())?;
        assert_eq!(pending.exit_code, 0);
        let records = pending.payload.json["result"]["records"]
            .as_array()
            .ok_or("records")?;
        assert!(!records.is_empty());
        assert!(records.iter().all(|r| r["state"] == "pending"));
        let ParseOutcome::Run(missing) = parse([
            "obligations",
            "--status",
            "missing",
            "--format",
            "json",
            "--no-progress",
        ])?
        else {
            return Err("help".into());
        };
        let missing = execute(missing, repository.path())?;
        assert_eq!(missing.exit_code, 6);
        assert_eq!(missing.payload.json["outcome"], "error");
        assert_eq!(missing.payload.json["error"]["code"], "invalid_request");
        Ok(())
    }
}
