//! Explicit invocation of one digest-pinned, locally installed adapter.

use super::CommandExecution;
use crate::cli::ParsedCli;
use crate::renderer::OutputPayload;
use crate::session::{PreparedSession, SessionRequest, prepare};
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_domain::{
    AdapterDefinition, AdapterId, AdapterLimits, Diagnostic, DiagnosticCode, DurationMillis,
    InventoryCompleteness, PositiveCount, RepositoryIdentity, Severity, Sha256Digest, TargetId,
    UtcInstant,
};
use eqm_protocol::{
    ADAPTER_REQUEST_SCHEMA, AdapterLimitsDto, AdapterOperationDto, CommandIdentity, DiagnosticDto,
    DiscoverResultDto, EvaluationModeDto, EvidenceSubjectDto, InventoryDto, InvocationContextDto,
    ReportEnvelope, ScopeSubjectDto,
};
use eqm_runner::{
    AdapterExecutionAuthority, CancellationToken, invoke_adapter, validate_inventory_response,
};
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

const TIMEOUT_MS: u64 = 30_000;
const MAX_INPUT_BYTES: u64 = 4 * 1024 * 1024;
const MAX_OUTPUT_BYTES: u64 = 16 * 1024 * 1024;
const MAX_ENTRIES: u64 = 250_000;
const MAX_DEPTH: u64 = 64;

/// Invokes exactly one committed adapter pin without acquisition or persistence.
pub fn execute(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    let offline = parsed.global.offline;
    let adapter = option(&parsed, "--adapter")
        .ok_or("adapter required")?
        .to_owned();
    let target = option(&parsed, "--target")
        .ok_or("target required")?
        .to_owned();
    let request = SessionRequest::new(parsed.global, parsed.command.name);
    let session = prepare(&request, start)?;
    let graph = session.finalized().graph();
    let adapter_id = AdapterId::new(adapter.as_str())?;
    let matches = graph
        .adapter_locks()
        .values()
        .filter(|lock| lock.id == adapter_id)
        .collect::<Vec<_>>();
    if matches.len() != 1 {
        return adapter_failure(
            &session,
            offline,
            format!("adapter `{adapter}` did not resolve to exactly one committed pin"),
        );
    }
    let lock = matches[0];
    let target_id = TargetId::new(target.as_str())?;
    let Some(target_authority) = graph.targets().get(&target_id) else {
        return adapter_failure(
            &session,
            offline,
            format!("target `{target}` was not found"),
        );
    };
    let definition = definition(lock)?;
    let target_root = session
        .repository_root()
        .join(target_authority.root().as_str());
    let target_root = match target_root.canonicalize() {
        Ok(path) => path,
        Err(_) => {
            return adapter_failure(
                &session,
                offline,
                format!("adapter target root for `{target}` is unavailable"),
            );
        }
    };
    let request = adapter_request(&session, &definition, &target_id, &target_root)?;
    let authority = AdapterExecutionAuthority {
        executable: installed_executable(session.repository_root(), definition.digest()),
        repository_root: session.repository_root().to_path_buf(),
        cancellation: CancellationToken::default(),
    };
    let response = match invoke_adapter(&definition, &request, &authority) {
        Ok(response) => response,
        Err(error) => {
            return adapter_failure(
                &session,
                offline,
                format!("adapter `{adapter}` invocation failed: {error}"),
            );
        }
    };
    let response_diagnostics = response.diagnostics.clone();
    let observation = match validate_inventory_response(&definition, &request, response) {
        Ok(observation) => observation,
        Err(error) => {
            return adapter_failure(
                &session,
                offline,
                format!("adapter `{adapter}` inventory failed validation: {error}"),
            );
        }
    };
    let Some(inventory) = observation.inventory().cloned() else {
        return adapter_failure(
            &session,
            offline,
            format!("adapter `{adapter}` returned an error response"),
        );
    };
    let blocking = observation.completeness() != InventoryCompleteness::Complete;
    let mut diagnostics = response_diagnostics;
    if blocking {
        diagnostics.push(DiagnosticDto::from_domain(&adapter_diagnostic(format!(
            "adapter `{adapter}` returned {} inventory",
            observation.completeness()
        ))?));
    }
    diagnostics.sort_unstable();
    let result = DiscoverResultDto {
        kind: CommandIdentity::Discover,
        adapter,
        target,
        inventory,
    };
    render(
        &session,
        offline,
        Some(result),
        diagnostics,
        if blocking { 4 } else { 0 },
    )
}

pub(super) fn definition(
    lock: &eqm_domain::AdapterLockIdentity,
) -> Result<AdapterDefinition, Box<dyn Error>> {
    Ok(AdapterDefinition::new(
        lock.id.clone(),
        lock.version.clone(),
        lock.source.clone(),
        lock.digest,
        lock.protocol,
        InventoryCompleteness::Complete,
        AdapterLimits::new(
            DurationMillis::new(TIMEOUT_MS)?,
            PositiveCount::new(MAX_INPUT_BYTES)?,
            PositiveCount::new(MAX_OUTPUT_BYTES)?,
            PositiveCount::new(MAX_ENTRIES)?,
            PositiveCount::new(MAX_DEPTH)?,
        )?,
    )?)
}

fn adapter_request(
    session: &PreparedSession,
    definition: &AdapterDefinition,
    target: &TargetId,
    target_root: &Path,
) -> Result<eqm_protocol::AdapterRequestDto, Box<dyn Error>> {
    let repository = repository_identity(session.repository_root())?;
    let source_commit = git_output(session.repository_root(), &["rev-parse", "HEAD"])?;
    let request_seed = format!(
        "{}\0{}\0{}",
        session.workspace_digest(),
        definition.digest(),
        target
    );
    Ok(eqm_protocol::AdapterRequestDto {
        schema: ADAPTER_REQUEST_SCHEMA.to_string(),
        request_id: format!(
            "discover-{}",
            Sha256Digest::hash_content(request_seed.as_bytes())
        ),
        adapter: definition.id().as_str().to_owned(),
        adapter_digest: definition.digest().to_string(),
        operation: AdapterOperationDto::Discover,
        subject: EvidenceSubjectDto {
            repository: repository.as_str().to_owned(),
            repository_id_digest: Sha256Digest::hash_content(repository.as_str().as_bytes())
                .to_string(),
            scope: ScopeSubjectDto::Target {
                target: target.as_str().to_owned(),
            },
            source_commit,
            build_id: None,
            artifact_digest: None,
            target_configuration_digest: session.workspace_digest().to_string(),
        },
        target: target.as_str().to_owned(),
        target_root: target_root.to_string_lossy().into_owned(),
        limits: AdapterLimitsDto {
            timeout_ms: TIMEOUT_MS,
            max_input_bytes: MAX_INPUT_BYTES,
            max_output_bytes: MAX_OUTPUT_BYTES,
            max_entries: MAX_ENTRIES,
            max_depth: MAX_DEPTH,
        },
    })
}

fn repository_identity(root: &Path) -> Result<RepositoryIdentity, Box<dyn Error>> {
    let remote = git_output(root, &["remote", "get-url", "origin"])?;
    let normalized = if let Some(path) = remote.strip_prefix("git@github.com:") {
        format!("https://github.com/{}", path.trim_end_matches(".git"))
    } else {
        remote.trim_end_matches(".git").to_owned()
    };
    Ok(normalized.parse()?)
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

fn installed_executable(root: &Path, digest: Sha256Digest) -> PathBuf {
    root.join(".eqm")
        .join("adapters")
        .join(digest.to_string().trim_start_matches("sha256:"))
}

fn adapter_failure(
    session: &PreparedSession,
    offline: bool,
    message: String,
) -> Result<CommandExecution, Box<dyn Error>> {
    let diagnostic = adapter_diagnostic(message)?;
    render::<InventoryDto>(
        session,
        offline,
        None,
        vec![DiagnosticDto::from_domain(&diagnostic)],
        4,
    )
}

fn adapter_diagnostic(message: String) -> Result<Diagnostic, Box<dyn Error>> {
    Ok(Diagnostic::new(
        DiagnosticCode::from_number(700).ok_or("diagnostic code")?,
        Severity::Error,
        message,
        None,
        Vec::new(),
        Some("Install the exact committed adapter pin locally or correct its response.".into()),
    )?)
}

fn render<I: serde::Serialize>(
    session: &PreparedSession,
    offline: bool,
    result: Option<DiscoverResultDto<I>>,
    diagnostics: Vec<DiagnosticDto>,
    exit_code: u8,
) -> Result<CommandExecution, Box<dyn Error>> {
    let human = if exit_code == 0 {
        "adapter discovery completed".to_owned()
    } else {
        "adapter discovery failed".to_owned()
    };
    let envelope = ReportEnvelope::new(
        CommandIdentity::Discover,
        Some(session.workspace_digest()),
        context(offline)?,
        result,
        diagnostics,
    )?;
    Ok(CommandExecution {
        payload: OutputPayload {
            human,
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

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use crate::cli::{ParseOutcome, parse};
    use std::fs;
    use std::os::unix::fs::PermissionsExt as _;

    #[test]
    fn fake_adapter_is_invoked_only_through_its_exact_local_pin() -> Result<(), Box<dyn Error>> {
        let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let directory = tempfile::tempdir()?;
        let root = directory.path();
        fs::copy(source.join("eqm.toml"), root.join("eqm.toml"))?;
        copy_tree(&source.join("eqm"), &root.join("eqm"))?;
        fs::create_dir_all(root.join("apps/web"))?;
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
        let lock_path = root.join("eqm.lock");
        let script = b"#!/bin/sh\nexit 9\n";
        let digest = Sha256Digest::hash_content(script);
        let executable = installed_executable(root, digest);
        fs::create_dir_all(executable.parent().ok_or("adapter parent")?)?;
        fs::write(&executable, script)?;
        fs::set_permissions(&executable, fs::Permissions::from_mode(0o700))?;
        let lock = format!(
            "schema = \"https://schemas.equivalencematrix.dev/v1/lock\"\nversion = 1\n\n[[adapters]]\nid = \"adapter.test\"\nversion = \"1.0.0\"\nsource = \"https://example.com/adapters/test\"\nresolved = \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"\ndigest = \"{digest}\"\nprotocol = 1\n"
        );
        fs::write(&lock_path, lock)?;
        (|| {
            let ParseOutcome::Run(parsed) = parse([
                "discover",
                "--adapter",
                "adapter.test",
                "--target",
                "web",
                "--offline",
                "--format",
                "json",
                "--no-progress",
            ])?
            else {
                return Err("unexpected help".into());
            };
            let execution = execute(parsed, root)?;
            assert_eq!(execution.exit_code, 4);
            assert_eq!(
                execution.payload.json["diagnostics"][0]["code"],
                "EQM-E0700"
            );
            assert!(execution.payload.json["result"].is_null());
            fs::write(&executable, b"#!/bin/sh\nexit 0\n")?;
            let ParseOutcome::Run(parsed) =
                parse(["discover", "--adapter", "adapter.test", "--target", "web"])?
            else {
                return Err("unexpected help".into());
            };
            let mismatch = execute(parsed, root)?;
            assert_eq!(mismatch.exit_code, 4);
            Ok::<_, Box<dyn Error>>(())
        })()
    }

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
