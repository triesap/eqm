//! Adversarial integration coverage for the local process boundary.

#![cfg(unix)]

use eqm_domain::{
    ArgumentTemplate, DurationMillis, EnvironmentName, Extensions, PositiveCount, RepoPath,
    Revision, RunnerBackend, RunnerDefinition, RunnerId, RunnerLimits, RunnerProgram,
    SecretBinding, SecretProviderRef, SelectorText,
};
use eqm_runner::{
    CancellationToken, ExecutionOutcome, InvocationBindings, LocalExecutionContext,
    LocalExecutionError, ResolvedRunner, RunnerResolutionAuthority, execute_local_process,
    resolve_runner,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

fn runner(
    program: &str,
    args: Vec<ArgumentTemplate>,
    timeout: u64,
    cap: u64,
    secrets: Vec<SecretBinding>,
) -> Result<ResolvedRunner, Box<dyn Error>> {
    let path = RepoPath::new(program.strip_prefix('/').ok_or("absolute program")?)?;
    let digest = eqm_domain::Sha256Digest::hash_content(&fs::read(program)?);
    let definition = RunnerDefinition::new(
        RunnerId::new("runner.security")?,
        Revision::new(1)?,
        vec!["owner://team/security".parse()?],
        RunnerBackend::Local,
        RunnerProgram::Repository(path.clone()),
        args,
        None,
        Vec::new(),
        secrets,
        RunnerLimits::new(
            DurationMillis::new(timeout)?,
            PositiveCount::new(cap)?,
            None,
        )?,
        Vec::new(),
        Extensions::default(),
    )?;
    Ok(resolve_runner(
        &definition,
        &RunnerResolutionAuthority {
            id: RunnerId::new("runner.security")?,
            revision: Revision::new(1)?,
            backends: BTreeSet::from([RunnerBackend::Local]),
            repository_programs: BTreeMap::from([(path, digest)]),
            backend_guarantees: BTreeMap::from([(RunnerBackend::Local, BTreeSet::new())]),
            maximum_timeout: DurationMillis::new(timeout)?,
            maximum_output_bytes: PositiveCount::new(cap)?,
            maximum_concurrency: PositiveCount::ONE,
        },
    )?)
}

fn bindings(root: &Path) -> Result<InvocationBindings, Box<dyn Error>> {
    fs::create_dir_all(root.join("target"))?;
    fs::create_dir_all(root.join("results"))?;
    Ok(InvocationBindings::new(
        root.join("target"),
        "{}",
        root.join("results/result.json"),
    )?)
}

fn context(
    workspace_root: PathBuf,
    secrets: BTreeMap<EnvironmentName, Box<str>>,
) -> LocalExecutionContext {
    LocalExecutionContext {
        workspace_root,
        trusted_path: "/usr/bin:/bin".into(),
        secrets,
        cancellation: CancellationToken::default(),
    }
}

#[test]
fn injection_is_literal_and_inherited_environment_is_absent() -> Result<(), Box<dyn Error>> {
    let root = tempfile::tempdir()?;
    let marker = root.path().join("injected");
    let payload = format!("$(touch {}) ; `false` | cat", marker.display());
    let echo = runner(
        "/bin/echo",
        vec![ArgumentTemplate::Literal(SelectorText::new(
            payload.as_str(),
        )?)],
        1_000,
        4_096,
        Vec::new(),
    )?;
    let report = execute_local_process(
        &echo,
        &bindings(root.path())?,
        &context(PathBuf::from("/"), BTreeMap::new()),
    )?;
    assert_eq!(report.outcome, ExecutionOutcome::Succeeded);
    assert_eq!(String::from_utf8(report.stdout)?.trim(), payload);
    assert!(!marker.exists());

    let environment = runner("/usr/bin/env", Vec::new(), 1_000, 16_384, Vec::new())?;
    let report = execute_local_process(
        &environment,
        &bindings(root.path())?,
        &context(PathBuf::from("/"), BTreeMap::new()),
    )?;
    let output = String::from_utf8(report.stdout)?;
    assert!(!output.contains("HOME="));
    assert!(!output.contains("PATH="));
    Ok(())
}

#[test]
fn cwd_escape_and_symlink_escape_fail_before_spawn() -> Result<(), Box<dyn Error>> {
    let workspace = tempfile::tempdir()?;
    let outside = tempfile::tempdir()?;
    let definition = RunnerDefinition::new(
        RunnerId::new("runner.security")?,
        Revision::new(1)?,
        vec!["owner://team/security".parse()?],
        RunnerBackend::Local,
        RunnerProgram::Repository(RepoPath::new("program")?),
        Vec::new(),
        None,
        Vec::new(),
        Vec::new(),
        RunnerLimits::new(DurationMillis::new(1_000)?, PositiveCount::new(128)?, None)?,
        Vec::new(),
        Extensions::default(),
    )?;
    let runner = resolve_runner(
        &definition,
        &RunnerResolutionAuthority {
            id: RunnerId::new("runner.security")?,
            revision: Revision::new(1)?,
            backends: BTreeSet::from([RunnerBackend::Local]),
            repository_programs: BTreeMap::from([(
                RepoPath::new("program")?,
                eqm_domain::Sha256Digest::hash_content(b"program"),
            )]),
            backend_guarantees: BTreeMap::from([(RunnerBackend::Local, BTreeSet::new())]),
            maximum_timeout: DurationMillis::new(1_000)?,
            maximum_output_bytes: PositiveCount::new(128)?,
            maximum_concurrency: PositiveCount::ONE,
        },
    )?;
    fs::create_dir_all(workspace.path().join("results"))?;
    let escaped = InvocationBindings::new(
        outside.path().to_path_buf(),
        "{}",
        workspace.path().join("results/result.json"),
    )?;
    assert_eq!(
        execute_local_process(
            &runner,
            &escaped,
            &context(workspace.path().to_path_buf(), BTreeMap::new())
        ),
        Err(LocalExecutionError::PathEscape)
    );
    std::os::unix::fs::symlink(outside.path(), workspace.path().join("target"))?;
    let symlinked = InvocationBindings::new(
        workspace.path().join("target"),
        "{}",
        workspace.path().join("results/result.json"),
    )?;
    assert_eq!(
        execute_local_process(
            &runner,
            &symlinked,
            &context(workspace.path().to_path_buf(), BTreeMap::new())
        ),
        Err(LocalExecutionError::PathEscape)
    );
    Ok(())
}

#[test]
fn timeout_cancellation_and_output_flood_are_terminal() -> Result<(), Box<dyn Error>> {
    let root = tempfile::tempdir()?;
    let sleep = runner(
        "/bin/sleep",
        vec![ArgumentTemplate::Literal(SelectorText::new("1")?)],
        20,
        128,
        Vec::new(),
    )?;
    assert_eq!(
        execute_local_process(
            &sleep,
            &bindings(root.path())?,
            &context(PathBuf::from("/"), BTreeMap::new())
        )?
        .outcome,
        ExecutionOutcome::TimedOut
    );

    let sleep = runner(
        "/bin/sleep",
        vec![ArgumentTemplate::Literal(SelectorText::new("1")?)],
        2_000,
        128,
        Vec::new(),
    )?;
    let execution_context = context(PathBuf::from("/"), BTreeMap::new());
    let token = execution_context.cancellation.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(20));
        token.cancel();
    });
    assert_eq!(
        execute_local_process(&sleep, &bindings(root.path())?, &execution_context)?.outcome,
        ExecutionOutcome::Cancelled
    );

    let flood = runner("/usr/bin/yes", Vec::new(), 1_000, 64, Vec::new())?;
    let report = execute_local_process(
        &flood,
        &bindings(root.path())?,
        &context(PathBuf::from("/"), BTreeMap::new()),
    )?;
    assert_eq!(report.outcome, ExecutionOutcome::OutputLimitExceeded);
    assert_eq!(report.stdout.len(), 64);
    Ok(())
}

#[test]
fn declared_secrets_are_redacted_from_retained_output() -> Result<(), Box<dyn Error>> {
    let root = tempfile::tempdir()?;
    let name = EnvironmentName::new("TOKEN")?;
    let runner = runner(
        "/usr/bin/env",
        Vec::new(),
        1_000,
        16_384,
        vec![SecretBinding::new(
            name.clone(),
            SecretProviderRef::new("secret://vault/token")?,
        )],
    )?;
    let report = execute_local_process(
        &runner,
        &bindings(root.path())?,
        &context(
            PathBuf::from("/"),
            BTreeMap::from([(name, Box::from("sensitive-value"))]),
        ),
    )?;
    let output = String::from_utf8(report.stdout)?;
    assert!(!output.contains("sensitive-value"));
    assert!(output.contains("TOKEN=[REDACTED]"));
    Ok(())
}
