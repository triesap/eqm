//! Stable transactional formatting of authored TOML authority.

use super::CommandExecution;
use crate::cli::ParsedCli;
use crate::renderer::OutputPayload;
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_domain::{RepoPath, Sha256Digest, UtcInstant};
use eqm_manifest::{discover_sources, format_manifest, select_workspace_config};
use eqm_protocol::{
    CommandIdentity, EvaluationModeDto, FileChangeDto, FmtResultDto, InvocationContextDto,
    ReportEnvelope,
};
use std::collections::BTreeSet;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

struct PlannedFile {
    relative: RepoPath,
    absolute: PathBuf,
    original: Vec<u8>,
    formatted: Vec<u8>,
}

/// Formats explicit paths or every authored workspace TOML document.
pub fn execute(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    let check_only = flag(&parsed, "--check");
    let dry_run = flag(&parsed, "--dry-run");
    if check_only && dry_run {
        return Err("fmt --check and --dry-run are mutually exclusive".into());
    }
    let explicit = parsed
        .global
        .config
        .as_ref()
        .map(|path| RepoPath::new(path.to_string_lossy().replace('\\', "/")))
        .transpose()?;
    let config = select_workspace_config(start, explicit.as_ref())?;
    let paths = selected_paths(&parsed, &config)?;
    let mut plan = Vec::new();
    for relative in paths {
        let absolute = config.repository_root().join(relative.as_str());
        reject_symlink_components(config.repository_root(), &absolute)?;
        let metadata = fs::metadata(&absolute)?;
        if !metadata.is_file() || metadata.len() > 16 * 1024 * 1024 {
            return Err("format input must be a bounded regular file".into());
        }
        let original = fs::read(&absolute)?;
        let source = std::str::from_utf8(&original)?;
        let formatted = format_manifest(source)?.into_bytes();
        plan.push(PlannedFile {
            relative,
            absolute,
            original,
            formatted,
        });
    }
    let changed = plan
        .iter()
        .filter(|value| value.original != value.formatted)
        .collect::<Vec<_>>();
    let changes = changed
        .iter()
        .map(|value| FileChangeDto {
            path: value.relative.to_string(),
            action: "format".to_owned(),
            before_digest: Some(Sha256Digest::hash_content(&value.original).to_string()),
            after_digest: Some(Sha256Digest::hash_content(&value.formatted).to_string()),
        })
        .collect::<BTreeSet<_>>();
    let mut written = BTreeSet::new();
    if !check_only && !dry_run && !changed.is_empty() {
        transactional_replace(&changed)?;
        written.extend(changed.iter().map(|value| value.relative.to_string()));
    }
    let result = FmtResultDto {
        kind: CommandIdentity::Fmt,
        dry_run: dry_run || check_only,
        changes,
        written,
    };
    let envelope = ReportEnvelope::new(
        CommandIdentity::Fmt,
        None,
        InvocationContextDto::<(), ()>::new(
            EvaluationModeDto::Development,
            Vec::new(),
            None,
            None,
            parsed.global.offline,
            evaluated_at()?,
        )?,
        Some(result),
        Vec::new(),
    )?;
    Ok(CommandExecution {
        payload: OutputPayload {
            human: if changed.is_empty() {
                "all selected files are formatted".to_owned()
            } else {
                format!("{} selected file(s) require formatting", changed.len())
            },
            json: serde_json::from_slice(&envelope.to_json()?)?,
            sarif: None,
            markdown: None,
        },
        exit_code: if check_only && !changed.is_empty() {
            1
        } else {
            0
        },
    })
}

fn selected_paths(
    parsed: &ParsedCli,
    config: &eqm_manifest::WorkspaceConfig,
) -> Result<BTreeSet<RepoPath>, Box<dyn Error>> {
    if !parsed.command.operands.is_empty() {
        return parsed
            .command
            .operands
            .iter()
            .map(|value| {
                let path = RepoPath::new(value)?;
                if !path.as_str().ends_with(".toml") {
                    return Err("fmt accepts TOML paths only".into());
                }
                Ok(path)
            })
            .collect();
    }
    let mut paths = discover_sources(config)?
        .into_iter()
        .map(|value| value.path().clone())
        .collect::<BTreeSet<_>>();
    let config_relative = config
        .config_path()
        .strip_prefix(config.repository_root())?;
    paths.insert(RepoPath::new(
        config_relative.to_string_lossy().replace('\\', "/"),
    )?);
    let lock = config.dto().lockfile.as_deref().unwrap_or("eqm.lock");
    paths.insert(RepoPath::new(lock)?);
    Ok(paths)
}

fn transactional_replace(files: &[&PlannedFile]) -> Result<(), Box<dyn Error>> {
    let mut staged = Vec::with_capacity(files.len());
    for file in files {
        let parent = file.absolute.parent().ok_or("format parent")?;
        let permissions = fs::metadata(&file.absolute)?.permissions();
        let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
        temporary.write_all(&file.formatted)?;
        temporary.flush()?;
        temporary.as_file().sync_all()?;
        temporary.as_file().set_permissions(permissions)?;
        staged.push((*file, temporary));
    }
    let mut replaced: Vec<&PlannedFile> = Vec::new();
    for (file, temporary) in staged {
        if let Err(error) = temporary.persist(&file.absolute) {
            for previous in replaced.iter().rev() {
                atomic_restore(previous)?;
            }
            return Err(error.error.into());
        }
        replaced.push(file);
    }
    let parents = replaced
        .iter()
        .filter_map(|value| value.absolute.parent())
        .collect::<BTreeSet<_>>();
    for parent in parents {
        fs::File::open(parent)?.sync_all()?;
    }
    Ok(())
}

fn atomic_restore(file: &PlannedFile) -> Result<(), Box<dyn Error>> {
    let parent = file.absolute.parent().ok_or("format parent")?;
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    temporary.write_all(&file.original)?;
    temporary.flush()?;
    temporary.as_file().sync_all()?;
    temporary.persist(&file.absolute)?;
    fs::File::open(parent)?.sync_all()?;
    Ok(())
}

fn reject_symlink_components(root: &Path, path: &Path) -> Result<(), Box<dyn Error>> {
    let relative = path.strip_prefix(root)?;
    let mut current = root.to_path_buf();
    for component in relative.components() {
        current.push(component);
        if fs::symlink_metadata(&current).is_ok_and(|value| value.file_type().is_symlink()) {
            return Err("format path contains a symlink".into());
        }
    }
    Ok(())
}

fn flag(parsed: &ParsedCli, name: &str) -> bool {
    parsed.command.options.contains_key(name)
}

fn evaluated_at() -> Result<UtcInstant, Box<dyn Error>> {
    let value: DateTime<Utc> = SystemTime::now().into();
    Ok(value.to_rfc3339_opts(SecondsFormat::Secs, true).parse()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{CommandName, GlobalOptions, ParsedCommand};
    use crate::commands::init_new;
    use std::collections::BTreeMap;

    fn parsed(check: bool, dry_run: bool) -> ParsedCli {
        let mut options = BTreeMap::new();
        if check {
            options.insert("--check".to_owned(), vec![None]);
        }
        if dry_run {
            options.insert("--dry-run".to_owned(), vec![None]);
        }
        ParsedCli {
            global: GlobalOptions::default(),
            command: ParsedCommand {
                name: CommandName::Fmt,
                operands: Vec::new(),
                options,
            },
        }
    }

    #[test]
    fn check_dry_run_write_and_idempotence_preserve_comments() -> Result<(), Box<dyn Error>> {
        let root = tempfile::tempdir()?;
        fs::create_dir(root.path().join(".git"))?;
        init_new::init(
            ParsedCli {
                global: GlobalOptions::default(),
                command: ParsedCommand {
                    name: CommandName::Init,
                    operands: Vec::new(),
                    options: BTreeMap::new(),
                },
            },
            root.path(),
        )?;
        let config = root.path().join("eqm.toml");
        let mut source = fs::read_to_string(&config)?;
        source.insert_str(0, "# retained\n");
        source.push('\n');
        fs::write(&config, &source)?;
        assert_eq!(execute(parsed(true, false), root.path())?.exit_code, 1);
        assert_eq!(fs::read_to_string(&config)?, source);
        assert_eq!(execute(parsed(false, true), root.path())?.exit_code, 0);
        assert_eq!(fs::read_to_string(&config)?, source);
        assert_eq!(execute(parsed(false, false), root.path())?.exit_code, 0);
        assert!(fs::read_to_string(&config)?.starts_with("# retained\n"));
        assert_eq!(execute(parsed(true, false), root.path())?.exit_code, 0);
        Ok(())
    }
}
