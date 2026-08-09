//! Current-schema workspace and authority scaffolding with rollback-safe writes.

use super::CommandExecution;
use crate::cli::{CommandName, ParsedCli};
use crate::renderer::OutputPayload;
use crate::session::{SessionRequest, prepare};
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_domain::{
    BindingId, CapabilityId, FragmentId, JourneyId, PolicyId, ProfileId, RepoPath, RunnerId,
    SchemaKind, SchemaUri, Sha256Digest, SurfaceId, UtcInstant, WaiverId,
};
use eqm_manifest::{format_manifest, select_workspace_config};
use eqm_protocol::{
    CommandIdentity, EvaluationModeDto, FileChangeDto, InitResultDto, InvocationContextDto,
    NewResultDto, ReportEnvelope,
};
use std::collections::BTreeSet;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

const WORKSPACE: &str = "schema = \"https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/workspace.schema.json\"\ncontract_sources = [\"eqm/contracts/**/*.toml\"]\nbinding_sources = [\"eqm/bindings/**/*.toml\"]\npolicy_sources = [\"eqm/policies/**/*.toml\"]\nprofile_sources = [\"eqm/profiles/**/*.toml\"]\nrunner_sources = [\"eqm/runners/**/*.toml\"]\nwaiver_sources = [\"eqm/waivers/**/*.toml\"]\n";
const LOCK: &str = "schema = \"https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/lock.schema.json\"\nversion = 1\n";

/// Initializes a new empty current-schema EQM workspace.
pub fn init(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    let dry_run = flag(&parsed, "--dry-run");
    let relative = parsed.command.operands.first().map_or(".", String::as_str);
    let destination = confined_destination(start, relative)?;
    if destination.join("eqm.toml").exists() || destination.join("eqm.lock").exists() {
        return Err("EQM workspace already exists at the destination".into());
    }
    if destination.exists() && !destination.is_dir() {
        return Err("initialization destination is not a directory".into());
    }
    let changes = BTreeSet::from([change("eqm.lock", LOCK), change("eqm.toml", WORKSPACE)]);
    let mut written = BTreeSet::new();
    if !dry_run {
        let created_directory = !destination.exists();
        if created_directory {
            fs::create_dir(&destination)?;
        }
        let config = destination.join("eqm.toml");
        let lock = destination.join("eqm.lock");
        if let Err(error) = create_new_file(&config, WORKSPACE.as_bytes()) {
            if created_directory {
                let _ = fs::remove_dir(&destination);
            }
            return Err(error.into());
        }
        if let Err(error) = create_new_file(&lock, LOCK.as_bytes()) {
            let _ = fs::remove_file(&config);
            if created_directory {
                let _ = fs::remove_dir(&destination);
            }
            return Err(error.into());
        }
        let request = SessionRequest::new(Default::default(), CommandName::Init);
        if let Err(error) = prepare(&request, &destination) {
            let _ = fs::remove_file(&lock);
            let _ = fs::remove_file(&config);
            if created_directory {
                let _ = fs::remove_dir(&destination);
            }
            return Err(format!("initialized workspace did not finalize: {error}").into());
        }
        written.extend(["eqm.lock".to_owned(), "eqm.toml".to_owned()]);
    }
    mutation_response(
        CommandIdentity::Init,
        parsed.global.offline,
        dry_run,
        changes,
        written,
    )
}

/// Creates one unused current-schema authority and rolls it back if finalization fails.
pub fn new(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    let dry_run = flag(&parsed, "--dry-run");
    let kind = parsed
        .command
        .operands
        .first()
        .ok_or("authority kind required")?;
    let id = parsed
        .command
        .operands
        .get(1)
        .ok_or("authority id required")?;
    validate_id(kind, id)?;
    let explicit = parsed
        .global
        .config
        .as_ref()
        .map(|path| RepoPath::new(path.to_string_lossy().replace('\\', "/")))
        .transpose()?;
    let config = select_workspace_config(start, explicit.as_ref())?;
    let relative = authority_path(kind, id)?;
    let absolute = config.repository_root().join(relative.as_str());
    if absolute.exists() {
        return Err("authority destination already exists".into());
    }
    reject_symlink_parent(config.repository_root(), &absolute)?;
    let source = format_manifest(&template(kind, id)?)?;
    let changes = BTreeSet::from([change(relative.as_str(), &source)]);
    let mut written = BTreeSet::new();
    if !dry_run {
        let parent = absolute.parent().ok_or("authority parent")?;
        fs::create_dir_all(parent)?;
        create_new_file(&absolute, source.as_bytes())?;
        let request = SessionRequest::new(parsed.global.clone(), CommandName::New);
        if let Err(error) = prepare(&request, start) {
            fs::remove_file(&absolute)?;
            return Err(format!("new authority did not finalize: {error}").into());
        }
        written.insert(relative.to_string());
    }
    mutation_response(
        CommandIdentity::New,
        parsed.global.offline,
        dry_run,
        changes,
        written,
    )
}

fn mutation_response(
    command: CommandIdentity,
    offline: bool,
    dry_run: bool,
    changes: BTreeSet<FileChangeDto>,
    written: BTreeSet<String>,
) -> Result<CommandExecution, Box<dyn Error>> {
    let context = InvocationContextDto::<(), ()>::new(
        EvaluationModeDto::Development,
        Vec::new(),
        None,
        None,
        offline,
        evaluated_at()?,
    )?;
    let (json, human) = match command {
        CommandIdentity::Init => {
            let result = InitResultDto {
                kind: command,
                dry_run,
                changes,
                written,
            };
            let envelope = ReportEnvelope::new(command, None, context, Some(result), Vec::new())?;
            (
                serde_json::from_slice(&envelope.to_json()?)?,
                "workspace scaffold planned".to_owned(),
            )
        }
        CommandIdentity::New => {
            let result = NewResultDto {
                kind: command,
                dry_run,
                changes,
                written,
            };
            let envelope = ReportEnvelope::new(command, None, context, Some(result), Vec::new())?;
            (
                serde_json::from_slice(&envelope.to_json()?)?,
                "authority scaffold planned".to_owned(),
            )
        }
        _ => return Err("invalid mutation command".into()),
    };
    Ok(CommandExecution {
        payload: OutputPayload {
            human,
            json,
            sarif: None,
            markdown: None,
        },
        exit_code: 0,
    })
}

fn confined_destination(start: &Path, relative: &str) -> Result<PathBuf, Box<dyn Error>> {
    let root = start.canonicalize()?;
    let relative = if relative == "." {
        None
    } else {
        Some(RepoPath::new(relative)?)
    };
    let destination = relative
        .as_ref()
        .map_or_else(|| root.clone(), |value| root.join(value.as_str()));
    let parent = destination
        .parent()
        .ok_or("initialization parent")?
        .canonicalize()?;
    if !parent.starts_with(&root) && destination != root {
        return Err("initialization destination escaped the current directory".into());
    }
    reject_symlink_parent(&root, &destination)?;
    Ok(destination)
}

fn reject_symlink_parent(root: &Path, path: &Path) -> Result<(), Box<dyn Error>> {
    let relative = path.strip_prefix(root)?;
    let mut current = root.to_path_buf();
    for component in relative.components() {
        current.push(component);
        if fs::symlink_metadata(&current).is_ok_and(|value| value.file_type().is_symlink()) {
            return Err("mutation path contains a symlink".into());
        }
    }
    Ok(())
}

fn create_new_file(path: &Path, bytes: &[u8]) -> Result<(), std::io::Error> {
    let parent = path
        .parent()
        .ok_or_else(|| std::io::Error::other("missing parent"))?;
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    temporary.write_all(bytes)?;
    temporary.flush()?;
    temporary.as_file().sync_all()?;
    temporary
        .persist_noclobber(path)
        .map_err(|error| error.error)?;
    fs::File::open(parent)?.sync_all()?;
    Ok(())
}

fn authority_path(kind: &str, id: &str) -> Result<RepoPath, Box<dyn Error>> {
    let directory = match kind {
        "capability" | "journey" | "surface" | "fragment" => "contracts",
        "binding" => "bindings",
        "policy" => "policies",
        "profile" => "profiles",
        "runner" => "runners",
        "waiver" => "waivers",
        _ => return Err("unsupported authority kind".into()),
    };
    Ok(RepoPath::new(format!("eqm/{directory}/{id}.toml"))?)
}

fn validate_id(kind: &str, id: &str) -> Result<(), Box<dyn Error>> {
    match kind {
        "capability" => {
            id.parse::<CapabilityId>()?;
        }
        "journey" => {
            id.parse::<JourneyId>()?;
        }
        "surface" => {
            id.parse::<SurfaceId>()?;
        }
        "fragment" => {
            id.parse::<FragmentId>()?;
        }
        "binding" => {
            id.parse::<BindingId>()?;
        }
        "policy" => {
            id.parse::<PolicyId>()?;
        }
        "profile" => {
            id.parse::<ProfileId>()?;
        }
        "runner" => {
            id.parse::<RunnerId>()?;
        }
        "waiver" => {
            id.parse::<WaiverId>()?;
        }
        _ => return Err("unsupported authority kind".into()),
    }
    Ok(())
}

fn template(kind: &str, id: &str) -> Result<String, Box<dyn Error>> {
    let schema = SchemaUri::new(kind.parse::<SchemaKind>()?).to_string();
    let parent = id.rsplit_once('.').map(|(value, _)| value).unwrap_or(id);
    let source = match kind {
        "capability" => format!(
            "schema = \"{schema}\"\nid = \"{id}\"\ntitle = \"{id}\"\nstatus = \"draft\"\nowners = [\"owner://team/eqm\"]\n"
        ),
        "journey" => format!(
            "schema = \"{schema}\"\nid = \"{id}\"\nrevision = 1\ntitle = \"{id}\"\ncapability = \"{parent}\"\nstatus = \"draft\"\nrisk_class = \"low\"\nowners = [\"owner://team/eqm\"]\nsurfaces = []\n"
        ),
        "surface" => format!(
            "schema = \"{schema}\"\nid = \"{id}\"\nrevision = 1\ntitle = \"{id}\"\njourney = \"{parent}\"\nstatus = \"draft\"\nowners = [\"owner://team/eqm\"]\nrequirements = []\nfragments = []\n"
        ),
        "fragment" => format!(
            "schema = \"{schema}\"\nid = \"{id}\"\nrevision = 1\ntitle = \"{id}\"\nrisk_class = \"low\"\nowners = [\"owner://team/eqm\"]\nrequirements = []\n"
        ),
        "binding" => format!(
            "schema = \"{schema}\"\nid = \"{id}\"\nrevision = 1\nowners = [\"owner://team/eqm\"]\ntarget = \"default\"\nunit = \"{parent}\"\nartifacts = []\n"
        ),
        "policy" => format!(
            "schema = \"{schema}\"\nid = \"{id}\"\nrevision = 1\ntitle = \"{id}\"\nowners = [\"owner://team/eqm\"]\nprofiles = []\nrequired_targets = []\nrules = []\n"
        ),
        "profile" => format!(
            "schema = \"{schema}\"\nid = \"{id}\"\nrevision = 1\ntitle = \"{id}\"\nowners = [\"owner://team/eqm\"]\n\n[[dimensions]]\nid = \"environment\"\nvalues = [\"default\"]\n\n[defaults]\nenvironment = \"default\"\n"
        ),
        "runner" => format!(
            "schema = \"{schema}\"\nid = \"{id}\"\nrevision = 1\nowners = [\"owner://team/eqm\"]\nbackend = \"local\"\nprogram = \"scripts/eqm-runner\"\nargs = []\ntimeout_ms = 60000\nmax_output_bytes = 1048576\n"
        ),
        "waiver" => format!(
            "schema = \"{schema}\"\nid = \"{id}\"\nrevision = 1\nowners = [\"owner://team/eqm\"]\npolicy = \"default\"\nreason = \"Documented temporary exception.\"\nissue = \"issue://EQM-1\"\napprovers = [\"owner://role/contract_approver\"]\nstarts_on = \"2099-01-01\"\nexpires_on = \"2099-01-02\"\ncontrols = [\"behavior\"]\n\n[scope]\ntarget = \"default\"\nunit = \"{parent}\"\nrequirement = \"{parent}#required\"\nfacets = [\"behavior\"]\n\n[scope.profiles]\n"
        ),
        _ => return Err("unsupported authority kind".into()),
    };
    Ok(source)
}

fn change(path: &str, bytes: &str) -> FileChangeDto {
    FileChangeDto {
        path: path.to_owned(),
        action: "create".to_owned(),
        before_digest: None,
        after_digest: Some(Sha256Digest::hash_content(bytes.as_bytes()).to_string()),
    }
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
    use crate::cli::{GlobalOptions, ParsedCommand};
    use std::collections::BTreeMap;

    #[test]
    fn init_dry_run_and_write_are_current_collision_safe_and_valid() -> Result<(), Box<dyn Error>> {
        let root = tempfile::tempdir()?;
        fs::create_dir(root.path().join(".git"))?;
        let parsed = |dry_run| ParsedCli {
            global: GlobalOptions::default(),
            command: ParsedCommand {
                name: CommandName::Init,
                operands: Vec::new(),
                options: if dry_run {
                    BTreeMap::from([("--dry-run".to_owned(), vec![None])])
                } else {
                    BTreeMap::new()
                },
            },
        };
        assert_eq!(init(parsed(true), root.path())?.exit_code, 0);
        assert!(!root.path().join("eqm.toml").exists());
        assert_eq!(init(parsed(false), root.path())?.exit_code, 0);
        assert!(
            prepare(
                &SessionRequest::new(Default::default(), CommandName::Validate),
                root.path()
            )
            .is_ok()
        );
        assert!(init(parsed(false), root.path()).is_err());
        Ok(())
    }

    #[test]
    fn every_new_kind_has_current_schema_without_placeholder_digests() -> Result<(), Box<dyn Error>>
    {
        for kind in [
            "capability",
            "journey",
            "surface",
            "fragment",
            "binding",
            "policy",
            "profile",
            "runner",
            "waiver",
        ] {
            let value = template(
                kind,
                match kind {
                    "capability" => "account.create",
                    "journey" => "account.create.signup",
                    "surface" => "account.create.signup.email",
                    "fragment" => "auth.otp",
                    "binding" => "binding.web.signup",
                    "policy" => "policy.release",
                    "profile" => "profile.default",
                    "runner" => "runner.local",
                    _ => "waiver.signup",
                },
            )?;
            assert!(format_manifest(&value)?.contains(&format!("/{kind}.schema.json")));
            assert!(!value.contains("sha256:0000"));
        }
        Ok(())
    }

    #[test]
    fn new_supports_dry_run_collision_checks_and_failure_rollback() -> Result<(), Box<dyn Error>> {
        let root = tempfile::tempdir()?;
        fs::create_dir(root.path().join(".git"))?;
        init(
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
        let parsed = |kind: &str, id: &str, dry_run: bool| ParsedCli {
            global: GlobalOptions::default(),
            command: ParsedCommand {
                name: CommandName::New,
                operands: vec![kind.to_owned(), id.to_owned()],
                options: if dry_run {
                    BTreeMap::from([("--dry-run".to_owned(), vec![None])])
                } else {
                    BTreeMap::new()
                },
            },
        };
        let capability = root.path().join("eqm/contracts/account.create.toml");
        new(parsed("capability", "account.create", true), root.path())?;
        assert!(!capability.exists());
        new(parsed("capability", "account.create", false), root.path())?;
        assert!(capability.exists());
        assert!(new(parsed("capability", "account.create", false), root.path()).is_err());

        let journey = root.path().join("eqm/contracts/missing.create.signup.toml");
        assert!(
            new(
                parsed("journey", "missing.create.signup", false),
                root.path()
            )
            .is_err()
        );
        assert!(!journey.exists());
        Ok(())
    }
}
