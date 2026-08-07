//! Atomic immutable content-addressed evidence persistence.

use eqm_domain::Sha256Digest;
use eqm_protocol::{EvidenceDtoError, EvidenceResultDto};
use serde_json::Value;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, File};
use std::io::Write as _;
use std::path::{Path, PathBuf};

const MAX_RESULT_BYTES: usize = 16 * 1024 * 1024;

/// Result of an immutable evidence write.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceWriteOutcome {
    /// Validated evidence result digest.
    pub digest: Sha256Digest,
    /// Final digest-named path.
    pub path: PathBuf,
    /// Whether this call installed new bytes.
    pub written: bool,
}

/// Validates and atomically persists one evidence result below `.eqm/results`.
pub fn persist_evidence_result(
    repository_root: &Path,
    bytes: &[u8],
) -> Result<EvidenceWriteOutcome, EvidenceWriteError> {
    if bytes.len() > MAX_RESULT_BYTES {
        return Err(EvidenceWriteError::TooLarge);
    }
    let dto = EvidenceResultDto::from_json(bytes).map_err(EvidenceWriteError::Protocol)?;
    let digest = dto
        .result_digest
        .parse::<Sha256Digest>()
        .map_err(|_| EvidenceWriteError::InvalidDigest)?;
    if evidence_payload_digest(bytes)? != digest {
        return Err(EvidenceWriteError::DigestMismatch);
    }
    let root = fs::canonicalize(repository_root).map_err(|_| EvidenceWriteError::InvalidRoot)?;
    let generated = create_confined_directory(&root, ".eqm")?;
    let results = create_confined_directory(&generated, "results")?;
    let hex = dto
        .result_digest
        .strip_prefix("sha256:")
        .ok_or(EvidenceWriteError::InvalidDigest)?;
    let destination = results.join(format!("{hex}.json"));
    if destination.exists() {
        return existing_outcome(destination, bytes, digest);
    }

    let mut temporary = tempfile::NamedTempFile::new_in(&results)
        .map_err(|_| EvidenceWriteError::TemporaryWrite)?;
    set_private_permissions(temporary.as_file()).map_err(|_| EvidenceWriteError::TemporaryWrite)?;
    temporary
        .write_all(bytes)
        .and_then(|()| temporary.flush())
        .and_then(|()| temporary.as_file().sync_all())
        .map_err(|_| EvidenceWriteError::TemporaryWrite)?;
    match temporary.persist_noclobber(&destination) {
        Ok(_) => {
            sync_directory(&results)?;
            Ok(EvidenceWriteOutcome {
                digest,
                path: destination,
                written: true,
            })
        }
        Err(_) if destination.exists() => existing_outcome(destination, bytes, digest),
        Err(_) => Err(EvidenceWriteError::AtomicInstall),
    }
}

fn evidence_payload_digest(bytes: &[u8]) -> Result<Sha256Digest, EvidenceWriteError> {
    let mut value: Value =
        serde_json::from_slice(bytes).map_err(|_| EvidenceWriteError::InvalidDigest)?;
    let object = value
        .as_object_mut()
        .ok_or(EvidenceWriteError::InvalidDigest)?;
    object.remove("id");
    object.remove("result_digest");
    let canonical =
        serde_json_canonicalizer::to_vec(&value).map_err(|_| EvidenceWriteError::InvalidDigest)?;
    Ok(Sha256Digest::hash_content(&canonical))
}

fn create_confined_directory(parent: &Path, name: &str) -> Result<PathBuf, EvidenceWriteError> {
    let path = parent.join(name);
    match fs::symlink_metadata(&path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            return Err(EvidenceWriteError::UnsafeDestination);
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir(&path).map_err(|_| EvidenceWriteError::UnsafeDestination)?;
            set_private_directory_permissions(&path)?;
        }
        Err(_) => return Err(EvidenceWriteError::UnsafeDestination),
    }
    let canonical = fs::canonicalize(&path).map_err(|_| EvidenceWriteError::UnsafeDestination)?;
    if canonical.starts_with(parent) {
        Ok(canonical)
    } else {
        Err(EvidenceWriteError::UnsafeDestination)
    }
}

fn existing_outcome(
    destination: PathBuf,
    bytes: &[u8],
    digest: Sha256Digest,
) -> Result<EvidenceWriteOutcome, EvidenceWriteError> {
    let metadata =
        fs::symlink_metadata(&destination).map_err(|_| EvidenceWriteError::UnsafeDestination)?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(EvidenceWriteError::UnsafeDestination);
    }
    let existing = fs::read(&destination).map_err(|_| EvidenceWriteError::UnsafeDestination)?;
    if existing != bytes {
        return Err(EvidenceWriteError::Collision);
    }
    Ok(EvidenceWriteOutcome {
        digest,
        path: destination,
        written: false,
    })
}

#[cfg(unix)]
fn set_private_permissions(file: &File) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt as _;
    file.set_permissions(fs::Permissions::from_mode(0o600))
}

#[cfg(not(unix))]
fn set_private_permissions(_file: &File) -> std::io::Result<()> {
    Ok(())
}

#[cfg(unix)]
fn set_private_directory_permissions(path: &Path) -> Result<(), EvidenceWriteError> {
    use std::os::unix::fs::PermissionsExt as _;
    fs::set_permissions(path, fs::Permissions::from_mode(0o700))
        .map_err(|_| EvidenceWriteError::UnsafeDestination)
}

#[cfg(not(unix))]
fn set_private_directory_permissions(_path: &Path) -> Result<(), EvidenceWriteError> {
    Ok(())
}

fn sync_directory(path: &Path) -> Result<(), EvidenceWriteError> {
    File::open(path)
        .and_then(|directory| directory.sync_all())
        .map_err(|_| EvidenceWriteError::AtomicInstall)
}

/// Immutable evidence persistence failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceWriteError {
    /// Input exceeded 16 MiB.
    TooLarge,
    /// Closed evidence protocol validation failed.
    Protocol(EvidenceDtoError),
    /// Result digest syntax or canonicalization was invalid.
    InvalidDigest,
    /// Claimed digest did not cover the canonical preceding fields.
    DigestMismatch,
    /// Repository root was unavailable.
    InvalidRoot,
    /// Generated destination was a symlink, wrong type, or escaped confinement.
    UnsafeDestination,
    /// Temporary file creation or durable write failed.
    TemporaryWrite,
    /// Atomic no-clobber installation failed.
    AtomicInstall,
    /// Existing digest-named payload differs from the validated bytes.
    Collision,
}

impl Display for EvidenceWriteError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for EvidenceWriteError {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::error::Error;

    fn evidence() -> Result<Vec<u8>, Box<dyn Error>> {
        let mut value = json!({
            "schema": "https://schemas.equivalencematrix.dev/v1/evidence-result",
            "subject": {
                "repository": "https://example.com/team/project",
                "repository_id_digest": "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "scope": {"kind": "target", "target": "web"},
                "source_commit": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "build_id": null,
                "artifact_digest": null,
                "target_configuration_digest": "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
            },
            "target": "web",
            "unit": "account.create",
            "requirements": ["account.create#works"],
            "facets": ["behavior"],
            "kind": "release_record",
            "evidence_spec_digest": "sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
            "contract_digest": "sha256:dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
            "binding_digest": "sha256:eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
            "policy_digest": "sha256:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            "runner_digest": null,
            "adapter_digest": null,
            "runtime_facts_digest": null,
            "release_record_digest": "sha256:1111111111111111111111111111111111111111111111111111111111111111",
            "profile_values": [],
            "producer": "producer://release/system/v1",
            "claimed_trust": "signed_ci",
            "observed_at": "2026-08-07T12:00:00Z",
            "payload": {"kind": "release_record", "release_record_digest": "sha256:1111111111111111111111111111111111111111111111111111111111111111"},
            "attachments": []
        });
        let canonical = serde_json_canonicalizer::to_vec(&value)?;
        let digest = Sha256Digest::hash_content(&canonical).to_string();
        value["id"] = Value::String(digest.clone());
        value["result_digest"] = Value::String(digest);
        Ok(serde_json::to_vec(&value)?)
    }

    #[test]
    fn write_is_atomic_idempotent_and_collision_safe() -> Result<(), Box<dyn Error>> {
        let root = tempfile::tempdir()?;
        let bytes = evidence()?;
        let first = persist_evidence_result(root.path(), &bytes)?;
        assert!(first.written);
        assert_eq!(fs::read(&first.path)?, bytes);
        let second = persist_evidence_result(root.path(), &bytes)?;
        assert!(!second.written);
        fs::write(&first.path, b"different")?;
        assert_eq!(
            persist_evidence_result(root.path(), &bytes),
            Err(EvidenceWriteError::Collision)
        );
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn symlink_destination_and_digest_mismatch_fail_without_partial_write()
    -> Result<(), Box<dyn Error>> {
        use std::os::unix::fs::symlink;
        let root = tempfile::tempdir()?;
        let outside = tempfile::tempdir()?;
        symlink(outside.path(), root.path().join(".eqm"))?;
        assert_eq!(
            persist_evidence_result(root.path(), &evidence()?),
            Err(EvidenceWriteError::UnsafeDestination)
        );

        let clean = tempfile::tempdir()?;
        let mut value: Value = serde_json::from_slice(&evidence()?)?;
        value["target"] = Value::String("ios".to_owned());
        let mismatched = serde_json::to_vec(&value)?;
        assert_eq!(
            persist_evidence_result(clean.path(), &mismatched),
            Err(EvidenceWriteError::DigestMismatch)
        );
        assert!(!clean.path().join(".eqm").exists());
        Ok(())
    }
}
