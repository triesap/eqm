//! Exact baseline-to-candidate semantic graph comparison.

use super::CommandExecution;
use crate::cli::{CommandName, GlobalOptions, ParsedCli};
use crate::renderer::OutputPayload;
use crate::session::{PreparedSession, SessionRequest, prepare};
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_domain::{Revision, UtcInstant};
use eqm_engine::{
    SemanticChangeKind, SemanticCoordinate, SemanticFieldClass, SemanticProjection, SemanticValue,
    classify_diffs,
};
use eqm_manifest::{
    project_binding, project_capability, project_fragment, project_journey, project_policy,
    project_profile, project_runner, project_surface, project_target, project_waiver,
};
use eqm_protocol::{
    CommandIdentity, DiffResultDto, EvaluationModeDto, InvocationContextDto, ReportEnvelope,
    SemanticChangeDto,
};
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

/// Compares exact prepared semantic graphs without evaluating or executing product code.
pub fn execute(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    let offline = parsed.global.offline;
    let baseline_identity = parsed.global.baseline.clone().ok_or("baseline required")?;
    let candidate_identity = option_value(&parsed, "--candidate").map(ToOwned::to_owned);
    let request = SessionRequest::new(parsed.global, parsed.command.name);
    let current = prepare(&request, start)?;
    let baseline = prepare_identity(&baseline_identity, &current, start)?;
    let candidate = candidate_identity
        .as_deref()
        .map(|identity| prepare_identity(identity, &current, start))
        .transpose()?
        .unwrap_or_else(|| current.clone());
    let changes = classify_diffs(&projection(&baseline), &projection(&candidate))
        .into_iter()
        .map(|change| {
            Ok(SemanticChangeDto {
                unit: change.coordinate.unit.map(|value| value.to_string()),
                requirement: change.coordinate.requirement.map(|value| value.to_string()),
                target: change.coordinate.target.map(|value| value.to_string()),
                facet: change.coordinate.facet.map(|value| value.to_string()),
                kind: change_kind(change.kind).to_owned(),
                field: change.coordinate.field.to_string(),
                before: change.before.map(semantic_value).transpose()?,
                after: change.after.map(semantic_value).transpose()?,
            })
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    let count = changes.len();
    let result = DiffResultDto {
        kind: CommandIdentity::Diff,
        baseline_digest: baseline.workspace_digest().to_string(),
        candidate_digest: candidate.workspace_digest().to_string(),
        changes,
    };
    let envelope = ReportEnvelope::new(
        CommandIdentity::Diff,
        Some(candidate.workspace_digest()),
        context(offline)?,
        Some(result),
        Vec::new(),
    )?;
    Ok(CommandExecution {
        payload: OutputPayload {
            human: format!("{count} semantic changes"),
            json: serde_json::from_slice(&envelope.to_json()?)?,
            sarif: None,
            markdown: None,
        },
        exit_code: 0,
    })
}

pub(super) fn prepare_identity(
    identity: &str,
    current: &PreparedSession,
    repository: &Path,
) -> Result<PreparedSession, Box<dyn Error>> {
    if identity.starts_with("sha256:") {
        return if identity == current.workspace_digest().to_string() {
            Ok(current.clone())
        } else {
            Err("semantic digest is exact but its prepared bytes are unavailable".into())
        };
    }
    let path = PathBuf::from(identity);
    if path.exists() {
        let start = if path.is_file() {
            path.parent().ok_or("baseline path parent")?
        } else {
            path.as_path()
        };
        return Ok(prepare(
            &SessionRequest::new(GlobalOptions::default(), CommandName::Diff),
            start,
        )?);
    }
    if !is_full_object_id(identity) {
        return Err(
            "baseline identity must be an exact digest, path, or full commit object ID".into(),
        );
    }
    prepare_commit(identity, repository)
}

fn prepare_commit(identity: &str, repository: &Path) -> Result<PreparedSession, Box<dyn Error>> {
    let temporary = tempfile::tempdir()?;
    let archive = temporary.path().join("repository.tar");
    let verified = Command::new("git")
        .args(["cat-file", "-e", &format!("{identity}^{{commit}}")])
        .current_dir(repository)
        .status()?;
    if !verified.success() {
        return Err("full commit object ID is unavailable locally".into());
    }
    let archived = Command::new("git")
        .args(["archive", "--format=tar", "-o"])
        .arg(&archive)
        .arg(identity)
        .current_dir(repository)
        .status()?;
    if !archived.success() {
        return Err("commit archive failed".into());
    }
    let extracted = Command::new("tar")
        .args(["-xf"])
        .arg(&archive)
        .args(["-C"])
        .arg(temporary.path())
        .status()?;
    if !extracted.success() {
        return Err("commit archive extraction failed".into());
    }
    fs::create_dir(temporary.path().join(".git"))?;
    Ok(prepare(
        &SessionRequest::new(GlobalOptions::default(), CommandName::Diff),
        temporary.path(),
    )?)
}

pub(super) fn is_full_object_id(value: &str) -> bool {
    matches!(value.len(), 40 | 64) && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

pub(super) fn projection(session: &PreparedSession) -> SemanticProjection {
    let graph = session.finalized().graph();
    let mut result = SemanticProjection::new();
    for value in graph.capabilities().values() {
        insert(
            &mut result,
            SemanticFieldClass::Entity,
            "capability",
            value.id().as_str(),
            None,
            project_capability(value),
        );
    }
    for value in graph.journeys().values() {
        insert(
            &mut result,
            SemanticFieldClass::Entity,
            "journey",
            value.id().as_str(),
            Some(value.revision()),
            project_journey(value),
        );
    }
    for value in graph.surfaces().values() {
        insert(
            &mut result,
            SemanticFieldClass::Requirement,
            "surface",
            value.id().as_str(),
            Some(value.revision()),
            project_surface(value),
        );
    }
    for value in graph.fragments().values() {
        insert(
            &mut result,
            SemanticFieldClass::Requirement,
            "fragment",
            value.id().as_str(),
            Some(value.revision()),
            project_fragment(value),
        );
    }
    for value in graph.targets().values() {
        insert(
            &mut result,
            SemanticFieldClass::Target,
            "target",
            value.id().as_str(),
            None,
            project_target(value),
        );
    }
    for value in graph.bindings().values() {
        insert(
            &mut result,
            SemanticFieldClass::Evidence,
            "binding",
            value.id().as_str(),
            Some(value.revision()),
            project_binding(value),
        );
    }
    for value in graph.policies().values() {
        insert(
            &mut result,
            SemanticFieldClass::OrderedPolicy,
            "policy",
            value.id().as_str(),
            Some(value.revision()),
            project_policy(value),
        );
    }
    for value in graph.profiles().values() {
        insert(
            &mut result,
            SemanticFieldClass::Entity,
            "profile",
            value.id().as_str(),
            Some(value.revision()),
            project_profile(value),
        );
    }
    for value in graph.runners().values() {
        insert(
            &mut result,
            SemanticFieldClass::Evidence,
            "runner",
            value.id().as_str(),
            Some(value.revision()),
            project_runner(value),
        );
    }
    for value in graph.waivers().values() {
        insert(
            &mut result,
            SemanticFieldClass::Waiver,
            "waiver",
            value.id().as_str(),
            Some(value.revision()),
            project_waiver(value),
        );
    }
    result
}

fn insert(
    projection: &mut SemanticProjection,
    class: SemanticFieldClass,
    kind: &str,
    id: &str,
    revision: Option<Revision>,
    value: Value,
) {
    let revision = revision.map_or_else(String::new, |value| format!("@{value}"));
    projection.insert(
        SemanticCoordinate {
            unit: None,
            requirement: None,
            target: None,
            facet: None,
            class,
            field: format!("{kind}:{id}{revision}").into_boxed_str(),
        },
        SemanticValue::Opaque(value.to_string().into_boxed_str()),
    );
}

fn semantic_value(value: SemanticValue) -> Result<Value, Box<dyn Error>> {
    Ok(match value {
        SemanticValue::Opaque(value) => serde_json::from_str(&value)?,
        SemanticValue::Strength(value) => Value::from(value),
    })
}

const fn change_kind(value: SemanticChangeKind) -> &'static str {
    match value {
        SemanticChangeKind::Strengthened => "strengthened",
        SemanticChangeKind::Weakened => "weakened",
        SemanticChangeKind::Added => "added",
        SemanticChangeKind::Removed => "removed",
        SemanticChangeKind::Evidence => "evidence",
        SemanticChangeKind::Waiver => "waiver",
        SemanticChangeKind::Exposure => "exposure",
        SemanticChangeKind::Nonnormative => "nonnormative",
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
    fn exact_path_and_digest_baselines_are_unchanged_and_floating_is_rejected()
    -> Result<(), Box<dyn Error>> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let identity = root.to_string_lossy().to_string();
        let ParseOutcome::Run(parsed) = parse([
            "diff",
            "--baseline",
            &identity,
            "--format",
            "json",
            "--no-progress",
        ])?
        else {
            return Err("unexpected help".into());
        };
        let result = execute(parsed, &root)?;
        assert_eq!(result.exit_code, 0);
        assert!(
            result.payload.json["result"]["changes"]
                .as_array()
                .ok_or("changes")?
                .is_empty()
        );
        let current = prepare(
            &SessionRequest::new(GlobalOptions::default(), CommandName::Diff),
            &root,
        )?;
        assert_eq!(
            prepare_identity(&current.workspace_digest().to_string(), &current, &root)?
                .workspace_digest(),
            current.workspace_digest()
        );
        assert!(prepare_identity("master", &current, &root).is_err());
        Ok(())
    }
}
