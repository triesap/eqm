//! Exact-subject in-toto attestation emission from validated evidence.

use super::{CommandExecution, evaluation};
use crate::cli::ParsedCli;
use crate::renderer::OutputPayload;
use crate::session::{PreparedSession, SessionRequest, prepare};
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_domain::{RepoPath, Sha256Digest, ToolVersion, UtcInstant};
use eqm_protocol::{
    AttestResultDto, AttestationPredicateDto, AttestationSubjectDto, CommandIdentity,
    EvaluationModeDto, EvidenceResultDto, EvidenceSubjectDto, InTotoStatementDto,
    InvocationContextDto, ProfileValueDto, ReportEnvelope, SubjectDigestDto,
};
use serde_json::Value;
use std::collections::BTreeSet;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

/// Emits one unsigned exact-subject statement from selected immutable evidence.
pub fn execute(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    if option(&parsed, "--signer").is_some() {
        return Err("no signer authority is configured for this workspace".into());
    }
    let offline = parsed.global.offline;
    let profiles = parsed.global.profiles.clone();
    let requested = option_values(&parsed, "--evidence");
    let request = SessionRequest::new(parsed.global, parsed.command.name);
    let session = prepare(&request, start)?;
    let (selection, _) = evaluation::derive(&session, &profiles)?;
    let paths = evidence_paths(&session, &requested)?;
    if paths.is_empty() {
        return Err("attestation requires at least one validated evidence result".into());
    }
    let evidence = paths
        .iter()
        .map(|path| read_evidence(&session, path))
        .collect::<Result<Vec<_>, _>>()?;
    let evaluation_subject = common_subject(&evidence)?;
    validate_subject_binding(&session, &evaluation_subject)?;
    let evidence_digests = evidence
        .iter()
        .map(|value| value.result_digest.clone())
        .collect::<BTreeSet<_>>();
    let policy_digest = common_required(
        evidence.iter().map(|value| value.policy_digest.as_str()),
        "policy digest",
    )?;
    if policy_digest != session.workspace_digest().to_string() {
        return Err("evidence policy digest does not bind the prepared workspace".into());
    }
    let runtime_facts_digest = common_optional(
        evidence
            .iter()
            .map(|value| value.runtime_facts_digest.as_deref()),
        "runtime facts digest",
    )?;
    let release_record_digest = common_optional(
        evidence
            .iter()
            .map(|value| value.release_record_digest.as_deref()),
        "release record digest",
    )?;
    let profile_values = selected_profile_values(&selection)?;
    if evidence
        .iter()
        .any(|value| value.profile_values != profile_values)
    {
        return Err("evidence profile values do not bind the prepared selection".into());
    }
    let conformance = conformance(&evidence);
    let targets = evidence
        .iter()
        .map(|value| value.target.as_str())
        .collect::<BTreeSet<_>>();
    let equivalence = if conformance == "conformant" && targets.len() > 1 {
        "equivalent"
    } else {
        "unknown"
    };
    let release_status = if release_record_digest.is_some() {
        if conformance == "conformant" {
            "pass"
        } else {
            "unknown"
        }
    } else {
        "not_applicable"
    };
    let subject_coordinates = evidence
        .iter()
        .map(|value| {
            let digest = value
                .subject
                .target_configuration_digest
                .strip_prefix("sha256:")
                .ok_or("target configuration digest")?;
            Ok((format!("target:{}", value.target), digest.to_owned()))
        })
        .collect::<Result<BTreeSet<_>, Box<dyn Error>>>()?;
    let subjects = subject_coordinates
        .into_iter()
        .map(|(name, sha256)| AttestationSubjectDto {
            name,
            digest: SubjectDigestDto { sha256 },
        })
        .collect();
    let statement = InTotoStatementDto::new(
        subjects,
        AttestationPredicateDto {
            tool_version: ToolVersion::CURRENT.as_str().to_owned(),
            command: "attest".to_owned(),
            workspace_digest: session.workspace_digest().to_string(),
            policy_digest,
            profile_values,
            evaluation_subject,
            evidence_digests,
            runtime_facts_digest,
            release_record_digest,
            trust_config_digest: trust_config_digest(&session).to_string(),
            evaluated_at: evaluated_at()?.to_string(),
            conformance: conformance.to_owned(),
            equivalence: equivalence.to_owned(),
            release_status: release_status.to_owned(),
            waivers: session
                .finalized()
                .graph()
                .waivers()
                .values()
                .map(|waiver| waiver.id().to_string())
                .collect(),
        },
    )?;
    let result = AttestResultDto {
        kind: CommandIdentity::Attest,
        statement,
        signed: false,
        signer: None,
    };
    let envelope = ReportEnvelope::new(
        CommandIdentity::Attest,
        Some(session.workspace_digest()),
        context(offline)?,
        Some(result),
        Vec::new(),
    )?;
    Ok(CommandExecution {
        payload: OutputPayload {
            human: "unsigned attestation emitted".to_owned(),
            json: serde_json::from_slice(&envelope.to_json()?)?,
            sarif: None,
            markdown: None,
        },
        exit_code: 0,
    })
}

pub(super) fn evidence_paths(
    session: &PreparedSession,
    requested: &BTreeSet<String>,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut paths = if requested.is_empty() {
        let root = session.repository_root().join(".eqm/results");
        match fs::read_dir(root) {
            Ok(entries) => entries
                .map(|entry| entry.map(|value| value.path()))
                .collect::<Result<Vec<_>, _>>()?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(error) => return Err(error.into()),
        }
    } else {
        requested
            .iter()
            .map(|value| {
                if let Some(hex) = value.strip_prefix("sha256:") {
                    if hex.len() != 64 || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
                        return Err("invalid evidence digest".into());
                    }
                    Ok(session
                        .repository_root()
                        .join(".eqm/results")
                        .join(format!("{hex}.json")))
                } else {
                    let path = RepoPath::new(value)?;
                    Ok(session.repository_root().join(path.as_str()))
                }
            })
            .collect::<Result<Vec<_>, Box<dyn Error>>>()?
    };
    paths.sort();
    paths.dedup();
    Ok(paths)
}

pub(super) fn read_evidence(
    session: &PreparedSession,
    path: &Path,
) -> Result<EvidenceResultDto, Box<dyn Error>> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() || !metadata.is_file() || metadata.len() > 16 * 1024 * 1024
    {
        return Err("evidence must be a bounded regular file".into());
    }
    let canonical = path.canonicalize()?;
    if !canonical.starts_with(session.repository_root().canonicalize()?) {
        return Err("evidence escaped repository confinement".into());
    }
    let bytes = fs::read(canonical)?;
    let value = EvidenceResultDto::from_json(&bytes)?;
    let mut canonical_value: Value = serde_json::from_slice(&bytes)?;
    let object = canonical_value.as_object_mut().ok_or("evidence object")?;
    object.remove("id");
    object.remove("result_digest");
    let digest = Sha256Digest::hash_content(&serde_json_canonicalizer::to_vec(&canonical_value)?);
    if digest.to_string() != value.result_digest {
        return Err("evidence result digest mismatch".into());
    }
    Ok(value)
}

fn common_subject(evidence: &[EvidenceResultDto]) -> Result<EvidenceSubjectDto, Box<dyn Error>> {
    let first = evidence.first().ok_or("evidence required")?.subject.clone();
    if evidence.iter().any(|value| value.subject != first) {
        return Err("evidence subjects do not match exactly".into());
    }
    Ok(first)
}

fn validate_subject_binding(
    session: &PreparedSession,
    subject: &EvidenceSubjectDto,
) -> Result<(), Box<dyn Error>> {
    let repository: eqm_domain::RepositoryIdentity = subject.repository.parse()?;
    let current_repository = repository_identity(session.repository_root())?;
    let current_commit = git_output(session.repository_root(), &["rev-parse", "HEAD"])?;
    if Sha256Digest::hash_content(repository.as_str().as_bytes()).to_string()
        != subject.repository_id_digest
        || subject.target_configuration_digest != session.workspace_digest().to_string()
        || repository.as_str() != current_repository
        || subject.source_commit != current_commit
    {
        return Err("evidence subject does not bind the prepared workspace".into());
    }
    let _: eqm_domain::SourceCommit = subject.source_commit.parse()?;
    let _: Sha256Digest = subject.target_configuration_digest.parse()?;
    Ok(())
}

pub(super) fn repository_identity(root: &Path) -> Result<String, Box<dyn Error>> {
    let remote = git_output(root, &["remote", "get-url", "origin"])?;
    let normalized = if let Some(path) = remote.strip_prefix("git@github.com:") {
        format!("https://github.com/{}", path.trim_end_matches(".git"))
    } else {
        remote.trim_end_matches(".git").to_owned()
    };
    let _: eqm_domain::RepositoryIdentity = normalized.parse()?;
    Ok(normalized)
}

pub(super) fn git_output(root: &Path, arguments: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(root)
        .output()?;
    if !output.status.success() {
        return Err("Git identity acquisition failed".into());
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}

fn common_required<'a>(
    mut values: impl Iterator<Item = &'a str>,
    name: &str,
) -> Result<String, Box<dyn Error>> {
    let first = values.next().ok_or("evidence required")?.to_owned();
    if values.any(|value| value != first) {
        return Err(format!("evidence {name} values differ").into());
    }
    Ok(first)
}

fn common_optional<'a>(
    mut values: impl Iterator<Item = Option<&'a str>>,
    name: &str,
) -> Result<Option<String>, Box<dyn Error>> {
    let first = values.next().ok_or("evidence required")?.map(str::to_owned);
    if values.any(|value| value != first.as_deref()) {
        return Err(format!("evidence {name} values differ").into());
    }
    Ok(first)
}

pub(super) fn selected_profile_values(
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

fn conformance(_evidence: &[EvidenceResultDto]) -> &'static str {
    // Evidence files carry claims, not verification authority. This command has
    // no configured CI transport or signature verifier in v1, so an unsigned
    // statement must not promote those claims into a conformance conclusion.
    "unknown"
}

pub(super) fn trust_config_digest(session: &PreparedSession) -> Sha256Digest {
    Sha256Digest::hash_content(
        format!("eqm:v1:trust-config\0{}", session.workspace_digest()).as_bytes(),
    )
}

fn option<'a>(parsed: &'a ParsedCli, name: &str) -> Option<&'a str> {
    parsed
        .command
        .options
        .get(name)
        .and_then(|values| values.first())
        .and_then(Option::as_deref)
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
    use serde_json::json;

    #[test]
    fn result_digest_validation_rejects_tampering() -> Result<(), Box<dyn Error>> {
        let directory = tempfile::tempdir()?;
        let path = directory.path().join("result.json");
        fs::write(&path, b"{}")?;
        let repository = crate::test_support::example_repository()?;
        let session_root = repository.path();
        let request = SessionRequest::new(Default::default(), crate::cli::CommandName::Attest);
        let session = prepare(&request, session_root)?;
        assert!(read_evidence(&session, &path).is_err());
        Ok(())
    }

    #[test]
    fn unsigned_statement_binds_exact_evidence_and_workspace() -> Result<(), Box<dyn Error>> {
        let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/android-ios");
        let directory = tempfile::tempdir()?;
        let root = directory.path();
        fs::copy(source.join("eqm.toml"), root.join("eqm.toml"))?;
        fs::copy(source.join("eqm.lock"), root.join("eqm.lock"))?;
        copy_tree(&source.join("eqm"), &root.join("eqm"))?;
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
        let request = SessionRequest::new(Default::default(), crate::cli::CommandName::Attest);
        let session = prepare(&request, root)?;
        let (selection, _) = evaluation::derive(&session, &[])?;
        let profiles = serde_json::to_value(selected_profile_values(&selection)?)?;
        let repository = repository_identity(root)?;
        let commit = git_output(root, &["rev-parse", "HEAD"])?;
        let digest = session.workspace_digest().to_string();
        let mut evidence = json!({
            "schema": eqm_protocol::EVIDENCE_RESULT_SCHEMA.to_string(),
            "subject": {
                "repository": repository,
                "repository_id_digest": Sha256Digest::hash_content(b"https://github.com/example/project").to_string(),
                "scope": {"kind":"target", "target":"android"},
                "source_commit": commit,
                "build_id": null,
                "artifact_digest": null,
                "target_configuration_digest": digest,
            },
            "target":"android",
            "unit":"account.create.signup.identifier",
            "requirements":["account.create.signup.identifier#email_default"],
            "facets":["release_presence"],
            "kind":"release_record",
            "evidence_spec_digest": Sha256Digest::hash_content(b"spec").to_string(),
            "contract_digest": session.workspace_digest().to_string(),
            "binding_digest": Sha256Digest::hash_content(b"binding").to_string(),
            "policy_digest": session.workspace_digest().to_string(),
            "runner_digest": null,
            "adapter_digest": null,
            "runtime_facts_digest": null,
            "release_record_digest": Sha256Digest::hash_content(b"release").to_string(),
            "profile_values": profiles,
            "producer":"producer://ci/fixture/v1",
            "claimed_trust":"trusted_ci",
            "observed_at":"2026-08-07T12:00:00Z",
            "payload":{"kind":"release_record","release_record_digest":Sha256Digest::hash_content(b"release").to_string()},
            "attachments":[]
        });
        let sealed = seal(&mut evidence)?;
        let result_root = root.join(".eqm/results");
        fs::create_dir_all(&result_root)?;
        fs::write(
            result_root.join(format!("{}.json", sealed.trim_start_matches("sha256:"))),
            serde_json::to_vec(&evidence)?,
        )?;
        let ParseOutcome::Run(parsed) = parse(["attest", "--format", "json", "--no-progress"])?
        else {
            return Err("unexpected help".into());
        };
        let execution = execute(parsed, root)?;
        assert_eq!(execution.exit_code, 0);
        assert_eq!(execution.payload.json["result"]["signed"], false);
        assert_eq!(
            execution.payload.json["result"]["statement"]["predicate"]["workspace_digest"],
            session.workspace_digest().to_string()
        );
        assert_eq!(
            execution.payload.json["result"]["statement"]["predicate"]["evidence_digests"][0],
            sealed
        );
        Ok(())
    }

    fn seal(value: &mut Value) -> Result<String, Box<dyn Error>> {
        let digest =
            Sha256Digest::hash_content(&serde_json_canonicalizer::to_vec(value)?).to_string();
        let object = value.as_object_mut().ok_or("evidence object")?;
        object.insert("id".to_owned(), Value::String(digest.clone()));
        object.insert("result_digest".to_owned(), Value::String(digest.clone()));
        Ok(digest)
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
