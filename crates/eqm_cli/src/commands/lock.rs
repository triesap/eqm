//! Explicit local exact-pin acquisition and deterministic lock replacement.

use super::{CommandExecution, attest};
use crate::cli::ParsedCli;
use crate::renderer::OutputPayload;
use crate::session::{SessionRequest, prepare};
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_domain::{
    AdapterId, FragmentId, RepoPath, Revision, SelectorText, Sha256Digest, SourceCommit, UtcInstant,
};
use eqm_manifest::{canonicalize_fragment, load_lockfile, select_workspace_config};
use eqm_protocol::{
    CommandIdentity, EvaluationModeDto, FileChangeDto, InvocationContextDto, LockUpdateResultDto,
    ReportEnvelope,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;

#[derive(Clone)]
struct ImportPin {
    id: String,
    revision: u64,
    source: String,
    resolved: String,
    digest: String,
    trust: Option<String>,
    signature: Option<String>,
}

#[derive(Clone)]
struct AdapterPin {
    id: String,
    version: String,
    source: String,
    resolved: String,
    digest: String,
    protocol: u64,
    trust: Option<String>,
    signature: Option<String>,
}

/// Updates selected exact local import and adapter pins only.
pub fn execute(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    let dry_run = parsed.command.options.contains_key("--dry-run");
    let request = SessionRequest::new(parsed.global.clone(), parsed.command.name);
    let session = prepare(&request, start)?;
    let explicit = parsed
        .global
        .config
        .as_ref()
        .map(|path| RepoPath::new(path.to_string_lossy().replace('\\', "/")))
        .transpose()?;
    let config = select_workspace_config(start, explicit.as_ref())?;
    let current = load_lockfile(&config)?;
    let source = attest::repository_identity(session.repository_root())?;
    let resolved = attest::git_output(session.repository_root(), &["rev-parse", "HEAD"])?;
    let _: SourceCommit = resolved.parse()?;
    let mut imports = current
        .imports()
        .values()
        .map(|value| {
            (
                value.id.to_string(),
                ImportPin {
                    id: value.id.to_string(),
                    revision: value.revision.get(),
                    source: value.source.as_str().to_owned(),
                    resolved: value.resolved.as_str().to_owned(),
                    digest: value.digest.to_string(),
                    trust: value.trust.map(|item| item.to_string()),
                    signature: value.signature.as_ref().map(ToString::to_string),
                },
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut adapters = current
        .adapters()
        .values()
        .map(|value| {
            (
                value.id.to_string(),
                AdapterPin {
                    id: value.id.to_string(),
                    version: value.version.to_string(),
                    source: value.source.as_str().to_owned(),
                    resolved: value.resolved.as_str().to_owned(),
                    digest: value.digest.to_string(),
                    protocol: value.protocol.get(),
                    trust: value.trust.map(|item| item.to_string()),
                    signature: value.signature.as_ref().map(ToString::to_string),
                },
            )
        })
        .collect::<BTreeMap<_, _>>();
    for specification in values(&parsed, "--import") {
        let (id, revision) = parse_import(specification)?;
        let fragment = session
            .finalized()
            .graph()
            .fragments()
            .get(&(id.clone(), revision))
            .ok_or("selected import is not exact local fragment authority")?;
        let digest = canonicalize_fragment(fragment)?.digest();
        imports.insert(
            id.to_string(),
            ImportPin {
                id: id.to_string(),
                revision: revision.get(),
                source: source.clone(),
                resolved: resolved.clone(),
                digest: digest.to_string(),
                trust: None,
                signature: None,
            },
        );
    }
    for specification in values(&parsed, "--adapter") {
        let (id, version, path) = parse_adapter(specification)?;
        let bytes = confined_program(session.repository_root(), &path)?;
        adapters.insert(
            id.to_string(),
            AdapterPin {
                id: id.to_string(),
                version: version.to_string(),
                source: source.clone(),
                resolved: resolved.clone(),
                digest: Sha256Digest::hash_content(&bytes).to_string(),
                protocol: 1,
                trust: None,
                signature: None,
            },
        );
    }
    let rendered = render_lock(imports.values(), adapters.values());
    let lock_relative = RepoPath::new(config.dto().lockfile.as_deref().unwrap_or("eqm.lock"))?;
    let lock_path = config.repository_root().join(lock_relative.as_str());
    reject_symlink_components(config.repository_root(), &lock_path)?;
    let original = fs::read(&lock_path)?;
    let changed = original != rendered.as_bytes();
    let changes = changed
        .then(|| FileChangeDto {
            path: lock_relative.to_string(),
            action: "update".to_owned(),
            before_digest: Some(Sha256Digest::hash_content(&original).to_string()),
            after_digest: Some(Sha256Digest::hash_content(rendered.as_bytes()).to_string()),
        })
        .into_iter()
        .collect::<BTreeSet<_>>();
    let mut written = BTreeSet::new();
    if changed && !dry_run {
        atomic_replace(&lock_path, rendered.as_bytes())?;
        if let Err(error) = load_lockfile(&config) {
            atomic_replace(&lock_path, &original)?;
            return Err(format!("updated lock did not validate: {error}").into());
        }
        written.insert(lock_relative.to_string());
    }
    let result = LockUpdateResultDto {
        kind: CommandIdentity::LockUpdate,
        dry_run,
        changes,
        written,
    };
    let envelope = ReportEnvelope::new(
        CommandIdentity::LockUpdate,
        Some(session.workspace_digest()),
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
            human: if changed {
                "lock update planned".to_owned()
            } else {
                "lock is current".to_owned()
            },
            json: serde_json::from_slice(&envelope.to_json()?)?,
            sarif: None,
            markdown: None,
        },
        exit_code: 0,
    })
}

fn parse_import(value: &str) -> Result<(FragmentId, Revision), Box<dyn Error>> {
    if value.contains(['/', '\\', ':']) {
        return Err("imports must use exact local ID@REVISION syntax".into());
    }
    let (id, revision) = value
        .rsplit_once('@')
        .ok_or("import requires ID@REVISION")?;
    Ok((id.parse()?, Revision::new(revision.parse()?)?))
}

fn parse_adapter(value: &str) -> Result<(AdapterId, SelectorText, RepoPath), Box<dyn Error>> {
    let (identity, path) = value
        .split_once('=')
        .ok_or("adapter requires ID@VERSION=PATH")?;
    let (id, version) = identity
        .rsplit_once('@')
        .ok_or("adapter requires ID@VERSION=PATH")?;
    if !immutable_version(version) {
        return Err("adapter version is floating".into());
    }
    Ok((
        id.parse()?,
        SelectorText::new(version)?,
        RepoPath::new(path)?,
    ))
}

fn immutable_version(value: &str) -> bool {
    !value.is_empty()
        && !value.chars().any(char::is_whitespace)
        && !value.contains(['*', '^', '~', '>', '<'])
        && !matches!(
            value.to_ascii_lowercase().as_str(),
            "main" | "master" | "head" | "latest"
        )
        && !value.starts_with("refs/heads/")
}

fn confined_program(root: &Path, path: &RepoPath) -> Result<Vec<u8>, Box<dyn Error>> {
    let absolute = root.join(path.as_str());
    reject_symlink_components(root, &absolute)?;
    let metadata = fs::metadata(&absolute)?;
    if !metadata.is_file() || metadata.len() > 64 * 1024 * 1024 {
        return Err("adapter artifact must be a bounded regular file".into());
    }
    Ok(fs::read(absolute)?)
}

fn render_lock<'a>(
    imports: impl Iterator<Item = &'a ImportPin>,
    adapters: impl Iterator<Item = &'a AdapterPin>,
) -> String {
    let mut output =
        "schema = \"https://schemas.equivalencematrix.dev/v1/lock\"\nversion = 1\n".to_owned();
    for value in imports {
        output.push_str(&format!(
            "\n[[imports]]\nid = {}\nrevision = {}\nsource = {}\nresolved = {}\ndigest = {}\n",
            quoted(&value.id),
            value.revision,
            quoted(&value.source),
            quoted(&value.resolved),
            quoted(&value.digest)
        ));
        optional(&mut output, "trust", value.trust.as_deref());
        optional(&mut output, "signature", value.signature.as_deref());
    }
    for value in adapters {
        output.push_str(&format!("\n[[adapters]]\nid = {}\nversion = {}\nsource = {}\nresolved = {}\ndigest = {}\nprotocol = {}\n", quoted(&value.id), quoted(&value.version), quoted(&value.source), quoted(&value.resolved), quoted(&value.digest), value.protocol));
        optional(&mut output, "trust", value.trust.as_deref());
        optional(&mut output, "signature", value.signature.as_deref());
    }
    output
}

fn quoted(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}
fn optional(output: &mut String, name: &str, value: Option<&str>) {
    if let Some(value) = value {
        output.push_str(&format!("{name} = {}\n", quoted(value)));
    }
}
fn values<'a>(parsed: &'a ParsedCli, name: &str) -> impl Iterator<Item = &'a str> {
    parsed
        .command
        .options
        .get(name)
        .into_iter()
        .flatten()
        .filter_map(Option::as_deref)
}

fn reject_symlink_components(root: &Path, path: &Path) -> Result<(), Box<dyn Error>> {
    let relative = path.strip_prefix(root)?;
    let mut current = root.to_path_buf();
    for component in relative.components() {
        current.push(component);
        if fs::symlink_metadata(&current).is_ok_and(|value| value.file_type().is_symlink()) {
            return Err("lock path contains a symlink".into());
        }
    }
    Ok(())
}

fn atomic_replace(path: &Path, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    let parent = path.parent().ok_or("lock parent")?;
    let permissions = fs::metadata(path)?.permissions();
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    temporary.write_all(bytes)?;
    temporary.flush()?;
    temporary.as_file().sync_all()?;
    temporary.as_file().set_permissions(permissions)?;
    temporary.persist(path)?;
    fs::File::open(parent)?.sync_all()?;
    Ok(())
}

fn evaluated_at() -> Result<UtcInstant, Box<dyn Error>> {
    let value: DateTime<Utc> = SystemTime::now().into();
    Ok(value.to_rfc3339_opts(SecondsFormat::Secs, true).parse()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_local_grammar_rejects_floating_and_renders_in_key_order() -> Result<(), Box<dyn Error>>
    {
        assert!(parse_import("auth.otp_entry@1").is_ok());
        assert!(parse_import("https://example.test/a@1").is_err());
        assert!(parse_adapter("adapter.test@1.2.3=scripts/run_example").is_ok());
        assert!(parse_adapter("adapter.test@latest=scripts/run_example").is_err());
        let first = ImportPin {
            id: "a.first".to_owned(),
            revision: 1,
            source: "https://example.test/a".to_owned(),
            resolved: "0".repeat(40),
            digest: Sha256Digest::hash_content(b"a").to_string(),
            trust: None,
            signature: None,
        };
        let second = ImportPin {
            id: "z.last".to_owned(),
            ..first.clone()
        };
        let imports = BTreeMap::from([(second.id.clone(), second), (first.id.clone(), first)]);
        let output = render_lock(imports.values(), std::iter::empty());
        assert!(output.find("a.first").ok_or("first")? < output.find("z.last").ok_or("last")?);
        Ok(())
    }
}
