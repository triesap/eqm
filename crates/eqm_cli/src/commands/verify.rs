//! Explicit bounded evidence runner execution and immutable result persistence.

use super::{CommandExecution, diff, evaluation};
use crate::cli::ParsedCli;
use crate::renderer::OutputPayload;
use crate::session::{PreparedSession, SessionRequest, prepare};
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_domain::{
    AttemptAggregate, EvidenceKind, RunnerBackend, RunnerProgram, Sha256Digest, TrustLevel,
    UtcInstant,
};
use eqm_protocol::{
    AttachmentDto, CommandIdentity, EvaluationModeDto, EvidencePayloadDto, EvidenceResultDto,
    EvidenceSelectorDto, FacetStatusDto, InvocationContextDto, ProfileValueDto, ReportEnvelope,
    VerifyResultDto,
};
use eqm_runner::{
    CancellationToken, InvocationBindings, LocalExecutionContext, RunnerResolutionAuthority,
    execute_local_process, persist_evidence_result, read_test_result, resolve_runner,
};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

/// Executes selected executable evidence specifications or reports a dry-run plan.
pub fn execute(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    let offline = parsed.global.offline;
    let profiles = parsed.global.profiles.clone();
    let unit = option(&parsed, "--unit").map(str::to_owned);
    let target = option(&parsed, "--target").map(str::to_owned);
    let dry_run = parsed.command.options.contains_key("--dry-run");
    let affected = parsed.command.options.contains_key("--affected");
    if affected && parsed.global.baseline.is_none() {
        return Err("verify --affected requires an exact baseline".into());
    }
    let request = SessionRequest::new(parsed.global, parsed.command.name);
    let session = prepare(&request, start)?;
    if affected {
        let baseline = parsed_baseline(&session, start, &baseline_identity(&request)?)?;
        let _ = baseline.workspace_digest();
    }
    let (selection, obligations) = evaluation::derive(&session, &profiles)?;
    let selected_profiles = profile_values(&selection)?;
    let mut plan = Vec::new();
    for binding in session
        .finalized()
        .graph()
        .bindings()
        .values()
        .filter(|binding| {
            unit.as_deref()
                .is_none_or(|value| binding.unit().as_str() == value)
                && target
                    .as_deref()
                    .is_none_or(|value| binding.target().as_str() == value)
        })
    {
        for specification in binding
            .evidence()
            .values()
            .filter(|specification| specification.runner().is_some())
        {
            plan.push((binding, specification));
        }
    }
    if dry_run {
        let result = VerifyResultDto {
            kind: CommandIdentity::Verify,
            selection: plan
                .iter()
                .map(|(binding, specification)| format!("{}:{}", binding.id(), specification.id()))
                .collect::<BTreeSet<_>>(),
            evidence_results: BTreeSet::<String>::new(),
            summary: BTreeMap::new(),
        };
        return render(&session, offline, result, 0);
    }
    let work = work_directory(session.repository_root())?;
    let mut results = BTreeSet::new();
    let mut summary = BTreeMap::new();
    let mut failed = false;
    let mut trust_failed = false;
    for (binding, specification) in plan {
        let runner_id = specification.runner().ok_or("executable runner")?;
        let definitions = session
            .finalized()
            .graph()
            .runners()
            .values()
            .filter(|definition| definition.id() == runner_id)
            .collect::<Vec<_>>();
        if definitions.len() != 1 {
            return Err(format!("runner `{runner_id}` did not resolve uniquely").into());
        }
        let definition = definitions[0];
        let resolved = resolve_runner(
            definition,
            &runner_authority(session.repository_root(), definition)?,
        )?;
        let target_authority = session
            .finalized()
            .graph()
            .targets()
            .get(binding.target())
            .ok_or("binding target")?;
        let target_root = session
            .repository_root()
            .join(target_authority.root().as_str())
            .canonicalize()?;
        let result_file = tempfile::NamedTempFile::new_in(&work)?;
        let selector = specification.selector().ok_or("executable selector")?;
        let selector_json = serde_json::to_string(&EvidenceSelectorDto::from(selector))?;
        let bindings = InvocationBindings::new(
            target_root,
            &selector_json,
            result_file.path().to_path_buf(),
        )?;
        let report = execute_local_process(
            &resolved,
            &bindings,
            &LocalExecutionContext {
                workspace_root: session.repository_root().to_path_buf(),
                trusted_path: "/usr/bin:/bin".into(),
                secrets: BTreeMap::new(),
                cancellation: CancellationToken::default(),
            },
        )?;
        if !matches!(report.outcome, eqm_runner::ExecutionOutcome::Succeeded) {
            return Err(format!("runner `{runner_id}` failed with {:?}", report.outcome).into());
        }
        let normalized = read_test_result(&fs::read(result_file.path())?)?;
        if normalized.selector() != selector {
            return Err("runner result selector mismatch".into());
        }
        let minimum = specification
            .minimum_count()
            .unwrap_or(eqm_domain::PositiveCount::ONE);
        let insufficient_trust = obligations.obligations.values().any(|obligation| {
            obligation.key.unit == *binding.unit()
                && specification
                    .requirements()
                    .contains(&obligation.key.requirement)
                && specification.facets().contains(&obligation.key.facet)
                && matches!(
                    &obligation.key.subject,
                    eqm_engine::ScopeSubject::Target(target) if target == binding.target()
                )
                && obligation.strength.minimum_trust > TrustLevel::UntrustedLocal
        });
        trust_failed |= insufficient_trust;
        let status = if insufficient_trust {
            FacetStatusDto::Unknown
        } else {
            match normalized.execution().aggregate(minimum) {
                AttemptAggregate::Satisfied => FacetStatusDto::Satisfied,
                AttemptAggregate::Failed => FacetStatusDto::Failed,
                AttemptAggregate::Unstable => FacetStatusDto::Unstable,
                AttemptAggregate::Missing => FacetStatusDto::Missing,
                AttemptAggregate::Unknown => FacetStatusDto::Unknown,
            }
        };
        failed |= status != FacetStatusDto::Satisfied;
        *summary.entry(status).or_insert(0) += 1;
        let bytes = evidence_bytes(
            &session,
            binding,
            specification,
            resolved.digest(),
            &selected_profiles,
            &normalized,
        )?;
        let outcome = persist_evidence_result(session.repository_root(), &bytes)?;
        results.insert(outcome.digest.to_string());
    }
    let result = VerifyResultDto {
        kind: CommandIdentity::Verify,
        selection: BTreeSet::from_iter(results.iter().map(|digest| format!("evidence:{digest}"))),
        evidence_results: results,
        summary,
    };
    render(
        &session,
        offline,
        result,
        if trust_failed {
            7
        } else if failed {
            1
        } else {
            0
        },
    )
}

fn baseline_identity(request: &SessionRequest) -> Result<String, Box<dyn Error>> {
    request
        .global
        .baseline
        .clone()
        .ok_or_else(|| "verify --affected requires an exact baseline".into())
}

fn parsed_baseline(
    session: &PreparedSession,
    start: &Path,
    identity: &str,
) -> Result<PreparedSession, Box<dyn Error>> {
    diff::prepare_identity(identity, session, start)
}

fn runner_authority(
    root: &Path,
    definition: &eqm_domain::RunnerDefinition,
) -> Result<RunnerResolutionAuthority, Box<dyn Error>> {
    let mut repository_programs = BTreeMap::new();
    if let RunnerProgram::Repository(path) = definition.program() {
        let absolute = root.join(path.as_str());
        let metadata = fs::symlink_metadata(&absolute)?;
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err("runner program must be a regular repository file".into());
        }
        repository_programs.insert(
            path.clone(),
            Sha256Digest::hash_content(&fs::read(absolute)?),
        );
    }
    Ok(RunnerResolutionAuthority {
        id: definition.id().clone(),
        revision: definition.revision(),
        backends: BTreeSet::from([RunnerBackend::Local]),
        repository_programs,
        backend_guarantees: BTreeMap::from([(
            RunnerBackend::Local,
            definition.guarantees().clone(),
        )]),
        maximum_timeout: definition.limits().timeout(),
        maximum_output_bytes: definition.limits().max_output_bytes(),
        maximum_concurrency: definition.limits().max_concurrency(),
    })
}

fn evidence_bytes(
    session: &PreparedSession,
    binding: &eqm_domain::Binding,
    specification: &eqm_domain::EvidenceSpecification,
    runner_digest: Sha256Digest,
    profiles: &[ProfileValueDto],
    normalized: &eqm_runner::NormalizedTestResult,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let repository = repository_identity(session.repository_root())?;
    let source_commit = git_output(session.repository_root(), &["rev-parse", "HEAD"])?;
    let coordinate = format!("{}:{}", binding.id(), specification.id());
    let coordinate_digest = Sha256Digest::hash_content(coordinate.as_bytes());
    let payload = match specification.kind() {
        EvidenceKind::StructuralCheck => EvidencePayloadDto::StructuralCheck {
            execution: normalized.execution().into(),
        },
        EvidenceKind::Test => EvidencePayloadDto::Test {
            execution: normalized.execution().into(),
        },
        EvidenceKind::Snapshot => EvidencePayloadDto::Snapshot {
            execution: normalized.execution().into(),
        },
        _ => return Err("non-executable evidence reached runner execution".into()),
    };
    let mut value = serde_json::to_value(EvidenceResultDto {
        schema: eqm_protocol::EVIDENCE_RESULT_SCHEMA.to_string(),
        id: String::new(),
        subject: eqm_protocol::EvidenceSubjectDto {
            repository: repository.clone(),
            repository_id_digest: Sha256Digest::hash_content(repository.as_bytes()).to_string(),
            scope: eqm_protocol::ScopeSubjectDto::Target {
                target: binding.target().as_str().to_owned(),
            },
            source_commit,
            build_id: None,
            artifact_digest: None,
            target_configuration_digest: session.workspace_digest().to_string(),
        },
        target: binding.target().as_str().to_owned(),
        unit: binding.unit().as_str().to_owned(),
        requirements: specification
            .requirements()
            .iter()
            .map(ToString::to_string)
            .collect(),
        facets: specification
            .facets()
            .iter()
            .map(ToString::to_string)
            .collect(),
        kind: specification.kind().to_string(),
        evidence_spec_digest: coordinate_digest.to_string(),
        contract_digest: session.workspace_digest().to_string(),
        binding_digest: coordinate_digest.to_string(),
        policy_digest: session.workspace_digest().to_string(),
        runner_digest: Some(runner_digest.to_string()),
        adapter_digest: None,
        runtime_facts_digest: None,
        release_record_digest: None,
        profile_values: profiles.to_vec(),
        producer: "producer://local/eqm/v1".to_owned(),
        claimed_trust: "untrusted_local".to_owned(),
        observed_at: evaluated_at()?.to_string(),
        payload,
        attachments: normalized
            .attachments()
            .values()
            .map(AttachmentDto::from)
            .collect(),
        result_digest: String::new(),
    })?;
    {
        let object = value.as_object_mut().ok_or("evidence object")?;
        object.remove("id");
        object.remove("result_digest");
    }
    let digest = Sha256Digest::hash_content(&serde_json_canonicalizer::to_vec(&value)?).to_string();
    let object = value.as_object_mut().ok_or("evidence object")?;
    object.insert("id".to_owned(), Value::String(digest.clone()));
    object.insert("result_digest".to_owned(), Value::String(digest));
    Ok(serde_json::to_vec(&value)?)
}

fn profile_values(
    selection: &eqm_engine::SelectedPolicyProfiles<'_>,
) -> Result<Vec<ProfileValueDto>, Box<dyn Error>> {
    let mut values = Vec::new();
    for selected in selection.profiles().values() {
        for (dimension, value) in selected.values() {
            values.push(ProfileValueDto::from_parts(
                selected.id(),
                selected.revision(),
                dimension,
                value.as_ref().ok_or("unknown selected profile value")?,
            ));
        }
    }
    values.sort_unstable();
    Ok(values)
}

fn work_directory(root: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let generated = root.join(".eqm");
    let work = generated.join("work");
    fs::create_dir_all(&work)?;
    if fs::symlink_metadata(&generated)?.file_type().is_symlink()
        || fs::symlink_metadata(&work)?.file_type().is_symlink()
    {
        return Err("unsafe generated work directory".into());
    }
    Ok(work)
}

fn repository_identity(root: &Path) -> Result<String, Box<dyn Error>> {
    let remote = git_output(root, &["remote", "get-url", "origin"])?;
    let normalized = if let Some(path) = remote.strip_prefix("git@github.com:") {
        format!("https://github.com/{}", path.trim_end_matches(".git"))
    } else {
        remote.trim_end_matches(".git").to_owned()
    };
    let _: eqm_domain::RepositoryIdentity = normalized.parse()?;
    Ok(normalized)
}

fn git_output(root: &Path, arguments: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(root)
        .output()?;
    if !output.status.success() {
        return Err("Git identity acquisition failed".into());
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}

fn render<S: serde::Serialize, E: serde::Serialize>(
    session: &PreparedSession,
    offline: bool,
    result: VerifyResultDto<S, E>,
    exit_code: u8,
) -> Result<CommandExecution, Box<dyn Error>> {
    let envelope = ReportEnvelope::new(
        CommandIdentity::Verify,
        Some(session.workspace_digest()),
        context(offline)?,
        Some(result),
        Vec::new(),
    )?;
    Ok(CommandExecution {
        payload: OutputPayload {
            human: if exit_code == 0 {
                "verification completed"
            } else {
                "verification found blocking evidence"
            }
            .to_owned(),
            json: serde_json::from_slice(&envelope.to_json()?)?,
            sarif: None,
            markdown: None,
        },
        exit_code,
    })
}

fn option<'a>(parsed: &'a ParsedCli, name: &str) -> Option<&'a str> {
    parsed
        .command
        .options
        .get(name)
        .and_then(|values| values.first())
        .and_then(Option::as_deref)
}

fn evaluated_at() -> Result<UtcInstant, Box<dyn Error>> {
    let value: DateTime<Utc> = SystemTime::now().into();
    Ok(value.to_rfc3339_opts(SecondsFormat::Secs, true).parse()?)
}

fn context(offline: bool) -> Result<InvocationContextDto<(), ()>, Box<dyn Error>> {
    Ok(InvocationContextDto::new(
        EvaluationModeDto::Development,
        Vec::new(),
        None,
        None,
        offline,
        evaluated_at()?,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{ParseOutcome, parse};

    #[test]
    fn dry_run_returns_an_exact_plan_without_execution_or_writes() -> Result<(), Box<dyn Error>> {
        let repository = crate::test_support::example_repository()?;
        let root = repository.path();
        let ParseOutcome::Run(parsed) =
            parse(["verify", "--dry-run", "--format", "json", "--no-progress"])?
        else {
            return Err("unexpected help".into());
        };
        let execution = execute(parsed, root)?;
        assert_eq!(execution.exit_code, 0);
        assert!(
            !execution.payload.json["result"]["selection"]
                .as_array()
                .ok_or("selection")?
                .is_empty()
        );
        assert!(!root.join(".eqm").exists());
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn fake_runner_writes_one_valid_immutable_result() -> Result<(), Box<dyn Error>> {
        use std::os::unix::fs::PermissionsExt as _;

        let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/android-ios");
        let directory = tempfile::tempdir()?;
        let root = directory.path();
        fs::copy(source.join("eqm.toml"), root.join("eqm.toml"))?;
        fs::copy(source.join("eqm.lock"), root.join("eqm.lock"))?;
        copy_tree(&source.join("eqm"), &root.join("eqm"))?;
        fs::create_dir_all(root.join("apps/android"))?;
        let script = root.join("apps/android/gradlew");
        fs::write(
            &script,
            r##"#!/bin/sh
/bin/cat > "$5" <<'JSON'
{"schema":"https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/protocol/test-result.schema.json","selector":{"kind":"test","framework":"junit","test_id":"signupIdentifierDefaultsToEmail","suite":null},"attempts":[{"number":1,"outcome":"passed","started_at":"2026-08-07T12:00:00Z","finished_at":"2026-08-07T12:00:01Z","message":null}],"counts":{"selected":1,"passed":1,"failed":0,"skipped":0,"filtered":0,"quarantined":0},"started_at":"2026-08-07T12:00:00Z","finished_at":"2026-08-07T12:00:01Z","attachments":[]}
JSON
"##,
        )?;
        fs::set_permissions(&script, fs::Permissions::from_mode(0o700))?;
        git(root, &["init", "-q"])?;
        git(
            root,
            &[
                "remote",
                "add",
                "origin",
                "git@github.com:example/project.git",
            ],
        )?;
        git(root, &["add", "."])?;
        git(
            root,
            &[
                "-c",
                "user.name=Fixture",
                "-c",
                "user.email=fixture@example.invalid",
                "commit",
                "-qm",
                "fixture",
            ],
        )?;
        let ParseOutcome::Run(parsed) = parse([
            "verify",
            "--unit",
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
        let execution = execute(parsed, root)?;
        assert_eq!(execution.exit_code, 7);
        let results = fs::read_dir(root.join(".eqm/results"))?.collect::<Result<Vec<_>, _>>()?;
        assert_eq!(results.len(), 1);
        let bytes = fs::read(results[0].path())?;
        assert!(EvidenceResultDto::from_json(&bytes).is_ok());
        Ok(())
    }

    #[cfg(unix)]
    fn copy_tree(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(destination)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let target = destination.join(entry.file_name());
            if entry.file_type()?.is_dir() {
                copy_tree(&entry.path(), &target)?;
            } else {
                fs::copy(entry.path(), target)?;
            }
        }
        Ok(())
    }

    #[cfg(unix)]
    fn git(root: &Path, arguments: &[&str]) -> Result<(), Box<dyn Error>> {
        if Command::new("git")
            .args(arguments)
            .current_dir(root)
            .status()?
            .success()
        {
            Ok(())
        } else {
            Err("Git fixture command failed".into())
        }
    }
}
