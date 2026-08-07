//! Bounded digest-pinned JSON adapter invocation.

use crate::CancellationToken;
use crate::execution::{
    configure_process_group, join_reader, read_bounded_signaled, terminate_process_tree,
};
use eqm_domain::{AdapterDefinition, Sha256Digest};
use eqm_protocol::{AdapterDtoError, AdapterRequestDto, AdapterResponseDto};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

const MAX_ADAPTER_STDERR: usize = 1024 * 1024;

/// Trusted local executable and confinement authority for one locked adapter.
#[derive(Clone, Debug)]
pub struct AdapterExecutionAuthority {
    /// Exact executable installed by operator authority.
    pub executable: PathBuf,
    /// Repository root used only to confine the request target root.
    pub repository_root: PathBuf,
    /// Cooperative cancellation signal.
    pub cancellation: CancellationToken,
}

/// Invokes one exact locked adapter with one JSON request on stdin.
pub fn invoke_adapter(
    definition: &AdapterDefinition,
    request: &AdapterRequestDto,
    authority: &AdapterExecutionAuthority,
) -> Result<AdapterResponseDto, AdapterInvocationError> {
    let request_bytes = serde_json::to_vec(request).map_err(|_| AdapterInvocationError::Request)?;
    let validated = AdapterRequestDto::from_json(&request_bytes)
        .map_err(|_| AdapterInvocationError::Request)?;
    validate_request(definition, &validated)?;
    if request_bytes.len()
        > usize::try_from(definition.limits().max_input_bytes().get())
            .map_err(|_| AdapterInvocationError::Request)?
    {
        return Err(AdapterInvocationError::Request);
    }
    let repository = fs::canonicalize(&authority.repository_root)
        .map_err(|_| AdapterInvocationError::Confinement)?;
    let target = fs::canonicalize(Path::new(&request.target_root))
        .map_err(|_| AdapterInvocationError::Confinement)?;
    if !target.starts_with(&repository) {
        return Err(AdapterInvocationError::Confinement);
    }
    let executable = fs::canonicalize(&authority.executable)
        .map_err(|_| AdapterInvocationError::ExecutableUnavailable)?;
    let executable_metadata = fs::symlink_metadata(&authority.executable)
        .map_err(|_| AdapterInvocationError::ExecutableUnavailable)?;
    if executable_metadata.file_type().is_symlink() || !executable_metadata.is_file() {
        return Err(AdapterInvocationError::ExecutableUnavailable);
    }
    let executable_bytes =
        fs::read(&executable).map_err(|_| AdapterInvocationError::ExecutableUnavailable)?;
    if Sha256Digest::hash_content(&executable_bytes) != definition.digest() {
        return Err(AdapterInvocationError::PinMismatch);
    }

    let mut command = Command::new(executable);
    command
        .current_dir(target)
        .env_clear()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    configure_process_group(&mut command);
    let mut child = command.spawn().map_err(|_| AdapterInvocationError::Spawn)?;
    let mut stdin = child.stdin.take().ok_or(AdapterInvocationError::Spawn)?;
    stdin
        .write_all(&request_bytes)
        .map_err(|_| AdapterInvocationError::InputWrite)?;
    drop(stdin);
    let stdout = child.stdout.take().ok_or(AdapterInvocationError::Spawn)?;
    let stderr = child.stderr.take().ok_or(AdapterInvocationError::Spawn)?;
    let output_cap = usize::try_from(definition.limits().max_output_bytes().get())
        .map_err(|_| AdapterInvocationError::OutputLimit)?;
    let output_exceeded = Arc::new(AtomicBool::new(false));
    let stdout_exceeded = Arc::clone(&output_exceeded);
    let stderr_exceeded = Arc::clone(&output_exceeded);
    let stdout_reader =
        thread::spawn(move || read_bounded_signaled(stdout, output_cap, &stdout_exceeded));
    let stderr_reader =
        thread::spawn(move || read_bounded_signaled(stderr, MAX_ADAPTER_STDERR, &stderr_exceeded));
    let deadline = Instant::now()
        .checked_add(Duration::from_millis(definition.limits().timeout().get()))
        .ok_or(AdapterInvocationError::Timeout)?;
    let status = loop {
        if output_exceeded.load(Ordering::Acquire) {
            terminate_process_tree(&mut child);
            let _ = child.wait();
            let _ = join_reader(stdout_reader);
            let _ = join_reader(stderr_reader);
            return Err(AdapterInvocationError::OutputLimit);
        }
        if authority.cancellation.is_cancelled() {
            terminate_process_tree(&mut child);
            let _ = child.wait();
            return Err(AdapterInvocationError::Cancelled);
        }
        if Instant::now() >= deadline {
            terminate_process_tree(&mut child);
            let _ = child.wait();
            return Err(AdapterInvocationError::Timeout);
        }
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) => thread::sleep(Duration::from_millis(5)),
            Err(_) => {
                terminate_process_tree(&mut child);
                let _ = child.wait();
                return Err(AdapterInvocationError::Wait);
            }
        }
    };
    let stdout = join_reader(stdout_reader).map_err(|_| AdapterInvocationError::OutputRead)?;
    let stderr = join_reader(stderr_reader).map_err(|_| AdapterInvocationError::OutputRead)?;
    if stdout.truncated || stderr.truncated {
        return Err(AdapterInvocationError::OutputLimit);
    }
    if !status.success() {
        return Err(AdapterInvocationError::NonzeroExit(status.code()));
    }
    let response =
        AdapterResponseDto::from_json(&stdout.bytes).map_err(AdapterInvocationError::Response)?;
    response
        .matches_request(request)
        .map_err(AdapterInvocationError::Response)?;
    Ok(response)
}

fn validate_request(
    definition: &AdapterDefinition,
    request: &AdapterRequestDto,
) -> Result<(), AdapterInvocationError> {
    let limits = definition.limits();
    if request.adapter != definition.id().as_str()
        || request.adapter_digest != definition.digest().to_string()
        || request.limits.timeout_ms != limits.timeout().get()
        || request.limits.max_input_bytes != limits.max_input_bytes().get()
        || request.limits.max_output_bytes != limits.max_output_bytes().get()
        || request.limits.max_entries != limits.max_entries().get()
        || request.limits.max_depth != limits.max_depth().get()
    {
        return Err(AdapterInvocationError::RequestMismatch);
    }
    Ok(())
}

/// Adapter setup, execution, or response classification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdapterInvocationError {
    /// Request could not be encoded or validate under its bound.
    Request,
    /// Request authority differs from the locked definition.
    RequestMismatch,
    /// Target root escaped repository confinement.
    Confinement,
    /// Configured executable was unavailable, a symlink, or not a file.
    ExecutableUnavailable,
    /// Installed executable content differed from its lock digest.
    PinMismatch,
    /// Process could not be created.
    Spawn,
    /// Complete request could not be written.
    InputWrite,
    /// Adapter exceeded its deadline.
    Timeout,
    /// Adapter was cancelled.
    Cancelled,
    /// Process supervision failed.
    Wait,
    /// Stdout or stderr exceeded its independent cap.
    OutputLimit,
    /// Bounded stream reading failed.
    OutputRead,
    /// Adapter returned a nonzero status.
    NonzeroExit(Option<i32>),
    /// Response was malformed, mismatched, or semantically inconsistent.
    Response(AdapterDtoError),
}

impl Display for AdapterInvocationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for AdapterInvocationError {}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use eqm_domain::{
        AdapterId, AdapterLimits, InventoryCompleteness, RepositoryIdentity, Revision, SelectorText,
    };
    use eqm_protocol::{
        ADAPTER_REQUEST_SCHEMA, AdapterLimitsDto, AdapterOperationDto, EvidenceSubjectDto,
        ScopeSubjectDto,
    };
    use std::error::Error;
    use std::os::unix::fs::PermissionsExt as _;

    fn script(root: &Path, body: &str) -> Result<PathBuf, Box<dyn Error>> {
        let path = root.join("adapter");
        fs::write(&path, format!("#!/bin/sh\n{body}\n"))?;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o700))?;
        Ok(path)
    }

    fn definition_fixture(executable: &Path) -> Result<AdapterDefinition, Box<dyn Error>> {
        Ok(AdapterDefinition::new(
            AdapterId::new("adapter.test")?,
            SelectorText::new("1.0.0")?,
            "https://example.com/adapters/test".parse::<RepositoryIdentity>()?,
            Sha256Digest::hash_content(&fs::read(executable)?),
            Revision::new(1)?,
            InventoryCompleteness::Complete,
            AdapterLimits::new(
                eqm_domain::DurationMillis::new(5_000)?,
                eqm_domain::PositiveCount::new(4 * 1024 * 1024)?,
                eqm_domain::PositiveCount::new(1_024)?,
                eqm_domain::PositiveCount::new(10)?,
                eqm_domain::PositiveCount::new(8)?,
            )?,
        )?)
    }

    fn request_fixture(definition: &AdapterDefinition, target: &Path) -> AdapterRequestDto {
        AdapterRequestDto {
            schema: ADAPTER_REQUEST_SCHEMA.to_string(),
            request_id: "request-1".to_owned(),
            adapter: definition.id().as_str().to_owned(),
            adapter_digest: definition.digest().to_string(),
            operation: AdapterOperationDto::Discover,
            subject: EvidenceSubjectDto {
                repository: "https://example.com/team/project".to_owned(),
                repository_id_digest:
                    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                        .to_owned(),
                scope: ScopeSubjectDto::Target {
                    target: "web".to_owned(),
                },
                source_commit: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned(),
                build_id: None,
                artifact_digest: None,
                target_configuration_digest:
                    "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
                        .to_owned(),
            },
            target: "web".to_owned(),
            target_root: target.to_string_lossy().into_owned(),
            limits: AdapterLimitsDto {
                timeout_ms: 5_000,
                max_input_bytes: 4 * 1024 * 1024,
                max_output_bytes: 1_024,
                max_entries: 10,
                max_depth: 8,
            },
        }
    }

    fn authority(executable: PathBuf, repository: &Path) -> AdapterExecutionAuthority {
        AdapterExecutionAuthority {
            executable,
            repository_root: repository.to_path_buf(),
            cancellation: CancellationToken::default(),
        }
    }

    #[test]
    fn success_malformed_nonzero_timeout_cap_and_pin_mismatch_classify()
    -> Result<(), Box<dyn Error>> {
        let root = tempfile::tempdir()?;
        let target = root.path().join("target");
        fs::create_dir(&target)?;
        let executable = script(
            root.path(),
            r#"input=$(/bin/cat)
digest=$(printf '%s' "$input" | /usr/bin/sed -n 's/.*"adapter_digest":"\([^"]*\)".*/\1/p')
printf '{"schema":"https://schemas.equivalencematrix.dev/v1/adapter-response","request_id":"request-1","adapter":"adapter.test","adapter_digest":"%s","status":"error","inventory":null,"diagnostics":[]}' "$digest""#,
        )?;
        let definition = definition_fixture(&executable)?;
        let initial_request = request_fixture(&definition, &target);
        let returned = invoke_adapter(
            &definition,
            &initial_request,
            &authority(executable.clone(), root.path()),
        )?;
        assert!(matches!(
            returned.status,
            eqm_protocol::AdapterStatusDto::Error
        ));

        let malformed = script(root.path(), "/bin/cat >/dev/null; printf '{'")?;
        let malformed_definition = definition_fixture(&malformed)?;
        assert!(matches!(
            invoke_adapter(
                &malformed_definition,
                &request_fixture(&malformed_definition, &target),
                &authority(malformed, root.path())
            ),
            Err(AdapterInvocationError::Response(_))
        ));
        let nonzero = script(root.path(), "/bin/cat >/dev/null; exit 7")?;
        let nonzero_definition = definition_fixture(&nonzero)?;
        assert_eq!(
            invoke_adapter(
                &nonzero_definition,
                &request_fixture(&nonzero_definition, &target),
                &authority(nonzero, root.path())
            ),
            Err(AdapterInvocationError::NonzeroExit(Some(7)))
        );
        let timeout = script(root.path(), "/bin/cat >/dev/null; /bin/sleep 6")?;
        let timeout_definition = definition_fixture(&timeout)?;
        assert_eq!(
            invoke_adapter(
                &timeout_definition,
                &request_fixture(&timeout_definition, &target),
                &authority(timeout, root.path())
            ),
            Err(AdapterInvocationError::Timeout)
        );
        let flood = script(root.path(), "/bin/cat >/dev/null; /usr/bin/yes")?;
        let flood_definition = definition_fixture(&flood)?;
        assert_eq!(
            invoke_adapter(
                &flood_definition,
                &request_fixture(&flood_definition, &target),
                &authority(flood.clone(), root.path())
            ),
            Err(AdapterInvocationError::OutputLimit)
        );
        fs::write(&flood, "changed")?;
        assert_eq!(
            invoke_adapter(
                &flood_definition,
                &request_fixture(&flood_definition, &target),
                &authority(flood, root.path())
            ),
            Err(AdapterInvocationError::PinMismatch)
        );
        Ok(())
    }
}
