//! Bounded shell-free local process execution.

use crate::{InvocationBindings, ResolvedProgram, ResolvedRunner, substitute_argv};
use eqm_domain::{
    EnvironmentName, EnvironmentSource, RunnerBackend, Sha256Digest, WorkingDirectoryTemplate,
};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

/// Cloneable cooperative cancellation signal.
#[derive(Clone, Debug, Default)]
pub struct CancellationToken(Arc<AtomicBool>);

impl CancellationToken {
    /// Requests cancellation of the associated process execution.
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Release);
    }

    fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }
}

/// Trusted invocation data unavailable to authored manifests.
#[derive(Clone, Debug)]
pub struct LocalExecutionContext {
    /// Canonicalizable workspace root.
    pub workspace_root: PathBuf,
    /// Trusted fixed PATH used only by explicit `trusted_path` bindings.
    pub trusted_path: Box<str>,
    /// Values resolved by a trusted secret provider and never rendered.
    pub secrets: BTreeMap<EnvironmentName, Box<str>>,
    /// Cooperative cancellation token.
    pub cancellation: CancellationToken,
}

/// Terminal process classification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutionOutcome {
    /// Exit status was zero.
    Succeeded,
    /// Exit status was nonzero or unavailable.
    Failed(Option<i32>),
    /// Definition timeout elapsed.
    TimedOut,
    /// Cancellation was requested.
    Cancelled,
    /// Stdout or stderr exceeded its independent cap.
    OutputLimitExceeded,
}

/// Bounded retained process output and terminal state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionReport {
    /// Terminal classification.
    pub outcome: ExecutionOutcome,
    /// Bounded stdout prefix.
    pub stdout: Vec<u8>,
    /// Bounded stderr prefix with configured secret values redacted.
    pub stderr: Vec<u8>,
    /// Whether either stream exceeded its cap.
    pub output_truncated: bool,
}

/// Executes one resolved `local` runner without a shell.
pub fn execute_local_process(
    runner: &ResolvedRunner,
    bindings: &InvocationBindings,
    context: &LocalExecutionContext,
) -> Result<ExecutionReport, LocalExecutionError> {
    if runner.definition().backend() != RunnerBackend::Local {
        return Err(LocalExecutionError::WrongBackend);
    }
    let workspace = fs::canonicalize(&context.workspace_root)
        .map_err(|_| LocalExecutionError::InvalidWorkspace)?;
    let target_root = confined_existing(bindings.target_root(), &workspace)?;
    let result_parent = bindings
        .result_path()
        .parent()
        .ok_or(LocalExecutionError::PathEscape)?;
    let result_parent = confined_existing(result_parent, &workspace)?;
    let program = match runner.program() {
        ResolvedProgram::Repository { path, digest } => {
            let program = confined_existing(&workspace.join(path.as_str()), &workspace)?;
            let bytes = fs::read(&program).map_err(|_| LocalExecutionError::ProgramUnavailable)?;
            if Sha256Digest::hash_content(&bytes) != *digest {
                return Err(LocalExecutionError::ProgramDigestMismatch);
            }
            program
        }
        ResolvedProgram::Locked { .. } => return Err(LocalExecutionError::ProgramUnavailable),
    };
    let cwd = match runner.definition().cwd() {
        WorkingDirectoryTemplate::TargetRoot => target_root,
        WorkingDirectoryTemplate::Repository(path) => {
            confined_existing(&workspace.join(path.as_str()), &workspace)?
        }
        WorkingDirectoryTemplate::ResultPath => result_parent,
    };
    let argv = substitute_argv(runner.definition(), bindings)
        .map_err(|_| LocalExecutionError::InvalidArguments)?;
    let mut command = Command::new(program);
    command
        .args(argv)
        .current_dir(cwd)
        .env_clear()
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for binding in runner.definition().environment().values() {
        let value = match binding.source() {
            EnvironmentSource::Literal(value) => value.as_str(),
            EnvironmentSource::TrustedPath => context.trusted_path.as_ref(),
            EnvironmentSource::CanonicalLocale => "C.UTF-8",
            EnvironmentSource::UtcTimezone => "UTC",
        };
        command.env(binding.name().as_str(), value);
    }
    if runner.definition().secrets().len() != context.secrets.len() {
        return Err(LocalExecutionError::SecretMismatch);
    }
    for binding in runner.definition().secrets().values() {
        let value = context
            .secrets
            .get(binding.name())
            .ok_or(LocalExecutionError::SecretMismatch)?;
        command.env(binding.name().as_str(), value.as_ref());
    }
    configure_process_group(&mut command);
    let mut child = command
        .spawn()
        .map_err(|_| LocalExecutionError::SpawnFailed)?;
    let stdout = child
        .stdout
        .take()
        .ok_or(LocalExecutionError::PipeUnavailable)?;
    let stderr = child
        .stderr
        .take()
        .ok_or(LocalExecutionError::PipeUnavailable)?;
    let cap = usize::try_from(runner.definition().limits().max_output_bytes().get())
        .map_err(|_| LocalExecutionError::OutputLimitInvalid)?;
    let stdout_reader = thread::spawn(move || read_bounded(stdout, cap));
    let stderr_reader = thread::spawn(move || read_bounded(stderr, cap));
    let deadline = Instant::now()
        .checked_add(Duration::from_millis(
            runner.definition().limits().timeout().get(),
        ))
        .ok_or(LocalExecutionError::TimeoutInvalid)?;
    let (status, forced) = loop {
        if context.cancellation.is_cancelled() {
            terminate_process_tree(&mut child);
            break (child.wait().ok(), Some(ExecutionOutcome::Cancelled));
        }
        if Instant::now() >= deadline {
            terminate_process_tree(&mut child);
            break (child.wait().ok(), Some(ExecutionOutcome::TimedOut));
        }
        match child.try_wait() {
            Ok(Some(status)) => break (Some(status), None),
            Ok(None) => thread::sleep(Duration::from_millis(5)),
            Err(_) => {
                terminate_process_tree(&mut child);
                break (child.wait().ok(), Some(ExecutionOutcome::Failed(None)));
            }
        }
    };
    let mut stdout = join_reader(stdout_reader)?;
    let mut stderr = join_reader(stderr_reader)?;
    redact(
        &mut stdout.bytes,
        context.secrets.values().map(AsRef::as_ref),
    );
    redact(
        &mut stderr.bytes,
        context.secrets.values().map(AsRef::as_ref),
    );
    let output_truncated = stdout.truncated || stderr.truncated;
    let outcome = if output_truncated {
        ExecutionOutcome::OutputLimitExceeded
    } else if let Some(forced) = forced {
        forced
    } else {
        classify_status(status)
    };
    Ok(ExecutionReport {
        outcome,
        stdout: stdout.bytes,
        stderr: stderr.bytes,
        output_truncated,
    })
}

fn confined_existing(path: &Path, root: &Path) -> Result<PathBuf, LocalExecutionError> {
    let path = fs::canonicalize(path).map_err(|_| LocalExecutionError::PathUnavailable)?;
    if path == root || path.starts_with(root) {
        Ok(path)
    } else {
        Err(LocalExecutionError::PathEscape)
    }
}

#[cfg(unix)]
fn configure_process_group(command: &mut Command) {
    use std::os::unix::process::CommandExt as _;
    command.process_group(0);
}

#[cfg(not(unix))]
fn configure_process_group(_command: &mut Command) {}

fn terminate_process_tree(child: &mut std::process::Child) {
    #[cfg(unix)]
    {
        let group = format!("-{}", child.id());
        let _ = Command::new("/bin/kill")
            .args(["-KILL", group.as_str()])
            .env_clear()
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    let _ = child.kill();
}

#[derive(Debug)]
struct BoundedOutput {
    bytes: Vec<u8>,
    truncated: bool,
}

fn read_bounded(mut reader: impl Read, cap: usize) -> io::Result<BoundedOutput> {
    let mut retained = Vec::with_capacity(cap.min(64 * 1024));
    let mut buffer = [0_u8; 8 * 1024];
    let mut truncated = false;
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        let remaining = cap.saturating_sub(retained.len());
        let kept = remaining.min(count);
        retained.extend_from_slice(&buffer[..kept]);
        truncated |= kept < count;
    }
    Ok(BoundedOutput {
        bytes: retained,
        truncated,
    })
}

fn join_reader(
    handle: thread::JoinHandle<io::Result<BoundedOutput>>,
) -> Result<BoundedOutput, LocalExecutionError> {
    handle
        .join()
        .map_err(|_| LocalExecutionError::OutputReadFailed)?
        .map_err(|_| LocalExecutionError::OutputReadFailed)
}

fn redact<'a>(bytes: &mut Vec<u8>, secrets: impl Iterator<Item = &'a str>) {
    let mut text = String::from_utf8_lossy(bytes).into_owned();
    for secret in secrets.filter(|value| !value.is_empty()) {
        text = text.replace(secret, "[REDACTED]");
    }
    *bytes = text.into_bytes();
}

fn classify_status(status: Option<ExitStatus>) -> ExecutionOutcome {
    match status {
        Some(status) if status.success() => ExecutionOutcome::Succeeded,
        Some(status) => ExecutionOutcome::Failed(status.code()),
        None => ExecutionOutcome::Failed(None),
    }
}

/// Local execution setup or supervision failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalExecutionError {
    /// Runner uses a different backend.
    WrongBackend,
    /// Workspace cannot be canonicalized.
    InvalidWorkspace,
    /// A required confined path does not exist.
    PathUnavailable,
    /// A path resolves outside the workspace.
    PathEscape,
    /// Executable cannot be used by this backend.
    ProgramUnavailable,
    /// Repository executable content differs from its resolved digest.
    ProgramDigestMismatch,
    /// Invocation arguments are invalid.
    InvalidArguments,
    /// Secret values do not exactly match declared bindings.
    SecretMismatch,
    /// Process could not be created.
    SpawnFailed,
    /// Child pipes were unavailable.
    PipeUnavailable,
    /// Output cap cannot be represented safely.
    OutputLimitInvalid,
    /// Timeout cannot be represented safely.
    TimeoutInvalid,
    /// A bounded output reader failed.
    OutputReadFailed,
}

impl Display for LocalExecutionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for LocalExecutionError {}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use crate::{RunnerResolutionAuthority, resolve_runner};
    use eqm_domain::{
        ArgumentTemplate, DurationMillis, Extensions, PositiveCount, RepoPath, Revision,
        RunnerDefinition, RunnerId, RunnerLimits, RunnerProgram, SelectorText,
    };
    use std::collections::BTreeSet;
    use std::error::Error;

    fn fixture(
        source: &str,
        args: Vec<ArgumentTemplate>,
        timeout: u64,
        cap: u64,
    ) -> Result<(tempfile::TempDir, ResolvedRunner, InvocationBindings), Box<dyn Error>> {
        let root = tempfile::tempdir()?;
        fs::create_dir_all(root.path().join("target"))?;
        fs::create_dir_all(root.path().join("results"))?;
        let program_path = source.strip_prefix('/').ok_or("absolute source required")?;
        let program_repo_path = RepoPath::new(program_path)?;
        let digest = Sha256Digest::hash_content(&fs::read(source)?);
        let definition = RunnerDefinition::new(
            RunnerId::new("runner.tests")?,
            Revision::new(1)?,
            vec!["owner://team/platform".parse()?],
            RunnerBackend::Local,
            RunnerProgram::Repository(program_repo_path.clone()),
            args,
            None,
            Vec::new(),
            Vec::new(),
            RunnerLimits::new(
                DurationMillis::new(timeout)?,
                PositiveCount::new(cap)?,
                None,
            )?,
            Vec::new(),
            Extensions::default(),
        )?;
        let authority = RunnerResolutionAuthority {
            id: RunnerId::new("runner.tests")?,
            revision: Revision::new(1)?,
            backends: BTreeSet::from([RunnerBackend::Local]),
            repository_programs: BTreeMap::from([(program_repo_path, digest)]),
            backend_guarantees: BTreeMap::from([(RunnerBackend::Local, BTreeSet::new())]),
            maximum_timeout: DurationMillis::new(timeout)?,
            maximum_output_bytes: PositiveCount::new(cap)?,
            maximum_concurrency: PositiveCount::ONE,
        };
        let runner = resolve_runner(&definition, &authority)?;
        let bindings = InvocationBindings::new(
            root.path().join("target"),
            "{}",
            root.path().join("results/result.json"),
        )?;
        Ok((root, runner, bindings))
    }

    fn context(_root: &Path) -> LocalExecutionContext {
        LocalExecutionContext {
            workspace_root: PathBuf::from("/"),
            trusted_path: "/usr/bin:/bin".into(),
            secrets: BTreeMap::new(),
            cancellation: CancellationToken::default(),
        }
    }

    #[test]
    fn success_failure_timeout_and_cap_are_classified() -> Result<(), Box<dyn Error>> {
        let (root, runner, bindings) = fixture(
            "/bin/echo",
            vec![ArgumentTemplate::Literal(SelectorText::new("hello")?)],
            1_000,
            128,
        )?;
        let success = execute_local_process(&runner, &bindings, &context(root.path()))?;
        assert_eq!(success.outcome, ExecutionOutcome::Succeeded);
        assert_eq!(success.stdout, b"hello\n");

        let (root, runner, bindings) = fixture("/usr/bin/false", Vec::new(), 1_000, 128)?;
        assert_eq!(
            execute_local_process(&runner, &bindings, &context(root.path()))?.outcome,
            ExecutionOutcome::Failed(Some(1))
        );

        let (root, runner, bindings) = fixture(
            "/bin/sleep",
            vec![ArgumentTemplate::Literal(SelectorText::new("1")?)],
            20,
            128,
        )?;
        assert_eq!(
            execute_local_process(&runner, &bindings, &context(root.path()))?.outcome,
            ExecutionOutcome::TimedOut
        );

        let (root, runner, bindings) = fixture("/usr/bin/yes", Vec::new(), 1_000, 32)?;
        let capped = execute_local_process(&runner, &bindings, &context(root.path()))?;
        assert_eq!(capped.outcome, ExecutionOutcome::OutputLimitExceeded);
        assert_eq!(capped.stdout.len(), 32);
        Ok(())
    }

    #[test]
    fn cancellation_is_terminal() -> Result<(), Box<dyn Error>> {
        let (root, runner, bindings) = fixture(
            "/bin/sleep",
            vec![ArgumentTemplate::Literal(SelectorText::new("1")?)],
            2_000,
            128,
        )?;
        let execution_context = context(root.path());
        let token = execution_context.cancellation.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(20));
            token.cancel();
        });
        assert_eq!(
            execute_local_process(&runner, &bindings, &execution_context)?.outcome,
            ExecutionOutcome::Cancelled
        );
        Ok(())
    }
}
