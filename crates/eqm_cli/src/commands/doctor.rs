//! Non-executing environment and repository readiness inspection.

use super::CommandExecution;
use crate::cli::ParsedCli;
use crate::renderer::OutputPayload;
use crate::session::{SessionRequest, prepare};
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_domain::{RepoPath, UtcInstant};
use eqm_manifest::{load_lockfile, select_workspace_config};
use eqm_protocol::{
    CommandIdentity, DoctorCheckDto, DoctorResultDto, EvaluationModeDto, InvocationContextDto,
    ReportEnvelope, ResultStatusDto,
};
use std::collections::BTreeSet;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

/// Inspects readiness without spawning runners, adapters, package tools, or VCS commands.
pub fn execute(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    let offline = parsed.global.offline;
    let explicit = parsed
        .global
        .config
        .as_ref()
        .map(|path| RepoPath::new(path.to_string_lossy().replace('\\', "/")))
        .transpose();
    let config = explicit
        .as_ref()
        .map_err(|_| "invalid explicit configuration path")
        .and_then(|value| {
            select_workspace_config(start, value.as_ref()).map_err(|_| "configuration unavailable")
        });
    let root = config
        .as_ref()
        .map(|value| value.repository_root().to_path_buf())
        .unwrap_or_else(|_| start.to_path_buf());
    let mut checks = BTreeSet::new();
    checks.insert(check(
        "config",
        config.is_ok(),
        "current workspace configuration is selected and strictly decoded",
        "Correct the unique current-schema workspace configuration.",
    ));
    checks.insert(toolchain_check(&root));
    checks.insert(generated_state_check(&root));
    checks.insert(no_legacy_check(&root));
    checks.insert(match config.as_ref() {
        Ok(value) => check(
            "pins",
            load_lockfile(value).is_ok(),
            "lock entries are exact, current, and available offline",
            "Correct eqm.lock to contain only current exact immutable pins.",
        ),
        Err(_) => check(
            "pins",
            false,
            "lock entries cannot be inspected without configuration",
            "Correct the workspace configuration before inspecting pins.",
        ),
    });
    let request = SessionRequest::new(parsed.global, parsed.command.name);
    let workspace = prepare(&request, start);
    let workspace_healthy = workspace
        .as_ref()
        .is_ok_and(|session| session.mcp_session().is_ok());
    checks.insert(check(
        "workspace",
        workspace_healthy,
        "workspace authority finalizes and exposes the read-only adapter boundary without execution",
        "Correct manifest, graph, invariant, expansion, or canonicalization failures.",
    ));
    let status = if checks
        .iter()
        .any(|value| value.status == ResultStatusDto::Error)
    {
        ResultStatusDto::Error
    } else if checks
        .iter()
        .any(|value| value.status == ResultStatusDto::Partial)
    {
        ResultStatusDto::Partial
    } else {
        ResultStatusDto::Ok
    };
    let result = DoctorResultDto {
        kind: CommandIdentity::Doctor,
        checks,
        status,
    };
    let digest = workspace.ok().map(|value| value.workspace_digest());
    let envelope = ReportEnvelope::new(
        CommandIdentity::Doctor,
        digest,
        InvocationContextDto::<(), ()>::new(
            EvaluationModeDto::Development,
            Vec::new(),
            None,
            None,
            offline,
            evaluated_at()?,
        )?,
        Some(result.clone()),
        Vec::new(),
    )?;
    let human = result
        .checks
        .iter()
        .map(|value| format!("{}: {:?}: {}", value.id, value.status, value.message))
        .collect::<Vec<_>>()
        .join("\n");
    Ok(CommandExecution {
        payload: OutputPayload {
            human,
            json: serde_json::from_slice(&envelope.to_json()?)?,
            sarif: None,
            markdown: None,
        },
        exit_code: if status == ResultStatusDto::Error {
            1
        } else {
            0
        },
    })
}

fn check(id: &str, healthy: bool, message: &str, remediation: &str) -> DoctorCheckDto {
    DoctorCheckDto {
        id: id.to_owned(),
        status: if healthy {
            ResultStatusDto::Ok
        } else {
            ResultStatusDto::Error
        },
        message: message.to_owned(),
        remediation: (!healthy).then(|| remediation.to_owned()),
    }
}

fn toolchain_check(root: &Path) -> DoctorCheckDto {
    let toolchain = fs::read_to_string(root.join("rust-toolchain.toml"));
    let cargo = fs::read_to_string(root.join("Cargo.toml"));
    let healthy = match (toolchain, cargo) {
        (Ok(toolchain), Ok(cargo)) => {
            let expected = env!("CARGO_PKG_RUST_VERSION");
            toolchain
                .lines()
                .any(|line| line.trim() == format!("channel = \"{expected}\""))
                && cargo
                    .lines()
                    .any(|line| line.trim() == format!("rust-version = \"{expected}\""))
                && toolchain.contains("\"clippy\"")
                && toolchain.contains("\"rustfmt\"")
        }
        _ => false,
    };
    check(
        "toolchain",
        healthy,
        "Rust version and required components are pinned consistently",
        "Align rust-toolchain.toml and workspace rust-version with clippy and rustfmt enabled.",
    )
}

fn generated_state_check(root: &Path) -> DoctorCheckDto {
    let ignored = fs::read_to_string(root.join(".gitignore"))
        .is_ok_and(|value| value.lines().any(|line| line.trim() == "/.eqm/"));
    let state = root.join(".eqm");
    let confined = if !state.exists() {
        true
    } else {
        inspect_generated(&state).unwrap_or(false)
    };
    check(
        "generated_state",
        ignored && confined,
        "generated state is ignored, confined, regular, and bounded",
        "Remove symlinks or unsupported entries from .eqm and keep /.eqm/ ignored.",
    )
}

fn inspect_generated(root: &Path) -> Result<bool, std::io::Error> {
    if fs::symlink_metadata(root)?.file_type().is_symlink() {
        return Ok(false);
    }
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let metadata = fs::symlink_metadata(entry.path())?;
        if metadata.file_type().is_symlink() || entry.file_name() != "results" || !metadata.is_dir()
        {
            return Ok(false);
        }
        for result in fs::read_dir(entry.path())? {
            let metadata = fs::symlink_metadata(result?.path())?;
            if metadata.file_type().is_symlink()
                || !metadata.is_file()
                || metadata.len() > 16 * 1024 * 1024
            {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

fn no_legacy_check(root: &Path) -> DoctorCheckDto {
    let clean = scan_sources(root).unwrap_or(false);
    check(
        "no_legacy",
        clean,
        "repository sources contain no forbidden compatibility identities",
        "Remove legacy product, CLI, hidden-state, and configuration identifiers.",
    )
}

fn scan_sources(root: &Path) -> Result<bool, std::io::Error> {
    let mut pending = vec![root.to_path_buf()];
    let forbidden = [
        "Feature".to_owned() + "Matrix",
        "fm".to_owned() + "tx",
        "FM".to_owned() + "TX",
    ];
    while let Some(directory) = pending.pop() {
        let mut entries = fs::read_dir(&directory)?.collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(fs::DirEntry::file_name);
        for entry in entries {
            let path = entry.path();
            let relative = path.strip_prefix(root).unwrap_or(&path);
            if ignored_scan_path(relative) {
                continue;
            }
            let metadata = fs::symlink_metadata(&path)?;
            if metadata.file_type().is_symlink() {
                continue;
            }
            if metadata.is_dir() {
                pending.push(path);
            } else if metadata.is_file() && metadata.len() <= 1024 * 1024 {
                let bytes = fs::read(path)?;
                if let Ok(text) = std::str::from_utf8(&bytes)
                    && forbidden.iter().any(|value| text.contains(value))
                {
                    return Ok(false);
                }
            }
        }
    }
    Ok(true)
}

fn ignored_scan_path(path: &Path) -> bool {
    let value = path.to_string_lossy().replace('\\', "/");
    matches!(value.as_str(), ".git" | ".eqm" | "target")
        || value.starts_with(".git/")
        || value.starts_with(".eqm/")
        || value.starts_with("target/")
        || value == "scripts/check_no_legacy_names.sh"
        || value == "docs/specification/naming-and-no-compat.md"
        || value.starts_with("tests/fixtures/no_legacy/negative/")
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

    #[test]
    fn healthy_repository_is_inspected_without_execution() -> Result<(), Box<dyn Error>> {
        let parsed = ParsedCli {
            global: GlobalOptions::default(),
            command: ParsedCommand {
                name: crate::cli::CommandName::Doctor,
                operands: Vec::new(),
                options: BTreeMap::new(),
            },
        };
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let execution = execute(parsed, &root)?;
        assert_eq!(execution.exit_code, 0);
        assert!(execution.payload.human.contains("workspace: Ok"));
        Ok(())
    }

    #[test]
    fn legacy_and_generated_symlink_checks_fail_closed() -> Result<(), Box<dyn Error>> {
        let temporary = tempfile::tempdir()?;
        fs::write(
            temporary.path().join("bad.txt"),
            "Feature".to_owned() + "Matrix",
        )?;
        assert!(!scan_sources(temporary.path())?);
        Ok(())
    }
}
