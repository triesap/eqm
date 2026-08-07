//! Conservative affected analysis from exact Git or explicit paths.

use super::{CommandExecution, diff, evaluation};
use crate::cli::ParsedCli;
use crate::renderer::OutputPayload;
use crate::session::{SessionRequest, prepare};
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_domain::{RepoPath, TargetId, UnitId, UtcInstant};
use eqm_engine::{AffectedIndexes, ChangedFile, analyze_affected_set, classify_diffs};
use eqm_protocol::{
    AffectedResultDto, CommandIdentity, EvaluationModeDto, InvocationContextDto, ReportEnvelope,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::path::Path;
use std::process::Command;
use std::time::SystemTime;

/// Computes an exact or conservative affected set without executing product code.
pub fn execute(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    let offline = parsed.global.offline;
    let profiles = parsed.global.profiles.clone();
    let baseline_identity = parsed.global.baseline.clone().ok_or("baseline required")?;
    let explicit = option_values(&parsed, "--path");
    let request = SessionRequest::new(parsed.global, parsed.command.name);
    let current = prepare(&request, start)?;
    let baseline = diff::prepare_identity(&baseline_identity, &current, start)?;
    let paths = if explicit.is_empty() {
        if !diff::is_full_object_id(&baseline_identity) {
            return Err("Git-derived affected analysis requires a full commit object ID".into());
        }
        git_changed_paths(start, &baseline_identity)?
    } else {
        explicit
    };
    let (_, obligations) = evaluation::derive(&current, &profiles)?;
    let indexes = indexes(&current, &obligations);
    let changed_files = paths
        .iter()
        .map(|path| {
            let path = RepoPath::new(path)?;
            Ok(ChangedFile {
                target: classify_target(current.finalized().graph(), &path),
                path: path.to_string().into_boxed_str(),
            })
        })
        .collect::<Result<BTreeSet<_>, Box<dyn Error>>>()?;
    let semantic = classify_diffs(&diff::projection(&baseline), &diff::projection(&current));
    let affected = analyze_affected_set(&indexes, &changed_files, &semantic);
    let result = AffectedResultDto {
        kind: CommandIdentity::Affected,
        baseline_digest: baseline.workspace_digest().to_string(),
        changed_paths: paths,
        units: affected.units.iter().map(ToString::to_string).collect(),
        obligations: affected
            .obligations
            .iter()
            .filter_map(|key| obligations.obligations.get(key))
            .map(evaluation::obligation_id)
            .collect(),
        conservative: affected.conservative,
    };
    let envelope = ReportEnvelope::new(
        CommandIdentity::Affected,
        Some(current.workspace_digest()),
        context(offline)?,
        Some(result),
        Vec::new(),
    )?;
    Ok(CommandExecution {
        payload: OutputPayload {
            human: format!("{} affected units", affected.units.len()),
            json: serde_json::from_slice(&envelope.to_json()?)?,
            sarif: None,
            markdown: None,
        },
        exit_code: 0,
    })
}

fn indexes(
    session: &crate::session::PreparedSession,
    derived: &eqm_engine::ObligationDerivation,
) -> AffectedIndexes {
    let graph = session.finalized().graph();
    let all_obligations = derived.obligations.keys().cloned().collect::<BTreeSet<_>>();
    let all_units = all_obligations
        .iter()
        .map(|key| key.unit.clone())
        .collect::<BTreeSet<_>>();
    let mut unit_obligations = BTreeMap::<UnitId, BTreeSet<_>>::new();
    for key in &all_obligations {
        unit_obligations
            .entry(key.unit.clone())
            .or_default()
            .insert(key.clone());
    }
    let mut target_units = BTreeMap::<TargetId, BTreeSet<UnitId>>::new();
    let mut artifact_units = BTreeMap::<Box<str>, BTreeSet<UnitId>>::new();
    for binding in graph.bindings().values() {
        target_units
            .entry(binding.target().clone())
            .or_default()
            .insert(binding.unit().clone());
        for artifact in binding.artifacts().values().values() {
            artifact_units
                .entry(artifact.path().to_string().into_boxed_str())
                .or_default()
                .insert(binding.unit().clone());
        }
    }
    let mut unit_dependents = BTreeMap::<UnitId, BTreeSet<UnitId>>::new();
    for journey in graph.journeys().values() {
        for transition in journey.transitions() {
            if let (Ok(from), Ok(to)) = (
                UnitId::new(transition.from().as_str()),
                UnitId::new(transition.to().as_str()),
            ) {
                unit_dependents.entry(from).or_default().insert(to);
            }
        }
    }
    AffectedIndexes {
        all_units,
        all_obligations,
        unit_dependents,
        unit_obligations,
        target_units,
        artifact_units,
        semantic_units: BTreeMap::new(),
        semantic_obligations: BTreeMap::new(),
    }
}

fn classify_target(graph: &eqm_domain::WorkspaceGraph, path: &RepoPath) -> Option<TargetId> {
    graph
        .targets()
        .values()
        .filter(|target| {
            path.as_str() == target.root().as_str()
                || path
                    .as_str()
                    .strip_prefix(target.root().as_str())
                    .is_some_and(|suffix| suffix.starts_with('/'))
        })
        .max_by_key(|target| target.root().as_str().len())
        .map(|target| target.id().clone())
}

fn git_changed_paths(
    repository: &Path,
    baseline: &str,
) -> Result<BTreeSet<String>, Box<dyn Error>> {
    let mut paths = command_lines(
        Command::new("git")
            .args(["diff", "--name-only", "--no-renames", baseline, "--"])
            .current_dir(repository),
    )?;
    paths.extend(command_lines(
        Command::new("git")
            .args(["ls-files", "--others", "--exclude-standard"])
            .current_dir(repository),
    )?);
    Ok(paths)
}

fn command_lines(command: &mut Command) -> Result<BTreeSet<String>, Box<dyn Error>> {
    let output = command.output()?;
    if !output.status.success() {
        return Err("Git changed-file acquisition failed".into());
    }
    let stdout = String::from_utf8(output.stdout)?;
    stdout
        .lines()
        .map(|line| {
            RepoPath::new(line)
                .map(|path| path.to_string())
                .map_err(|error| Box::new(error) as Box<dyn Error>)
        })
        .collect()
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
    use std::fs;

    #[test]
    fn git_acquisition_includes_dirty_and_untracked_paths() -> Result<(), Box<dyn Error>> {
        let directory = tempfile::tempdir()?;
        let run = |arguments: &[&str]| -> Result<(), Box<dyn Error>> {
            let status = Command::new("git")
                .args(arguments)
                .current_dir(directory.path())
                .status()?;
            if status.success() {
                Ok(())
            } else {
                Err("git fixture".into())
            }
        };
        run(&["init", "-q"])?;
        fs::write(directory.path().join("tracked.txt"), "before\n")?;
        run(&["add", "tracked.txt"])?;
        run(&[
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "commit",
            "-qm",
            "fixture",
        ])?;
        let baseline = String::from_utf8(
            Command::new("git")
                .args(["rev-parse", "HEAD"])
                .current_dir(directory.path())
                .output()?
                .stdout,
        )?
        .trim()
        .to_owned();
        fs::write(directory.path().join("tracked.txt"), "after\n")?;
        fs::write(directory.path().join("untracked.txt"), "new\n")?;
        assert_eq!(
            git_changed_paths(directory.path(), &baseline)?,
            BTreeSet::from(["tracked.txt".to_owned(), "untracked.txt".to_owned()])
        );
        Ok(())
    }
}
