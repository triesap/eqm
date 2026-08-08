//! Deterministic output selection and atomic delivery.

use crate::cli::{ColorWhen, OutputFormat};
use serde_json::Value;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// All supported representations of one semantic command result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutputPayload {
    /// Deterministic human representation.
    pub human: String,
    /// Common JSON envelope representation.
    pub json: Value,
    /// Optional SARIF representation for findings-capable commands.
    pub sarif: Option<Value>,
    /// Optional bounded Markdown representation for context.
    pub markdown: Option<String>,
}

/// Rendered bytes and their machine-output classification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenderedDocument {
    bytes: Vec<u8>,
    machine: bool,
}

impl RenderedDocument {
    /// Returns exact rendered bytes, including one terminal newline.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns whether TTY-dependent decoration is forbidden.
    #[must_use]
    pub const fn is_machine(&self) -> bool {
        self.machine
    }
}

/// Output rendering or delivery failure.
#[derive(Debug)]
pub enum RenderError {
    /// The command did not support the selected representation.
    UnsupportedFormat,
    /// JSON serialization failed.
    Json(serde_json::Error),
    /// Output filesystem or stream operation failed.
    Io(io::Error),
    /// Explicit output was a symbolic link.
    SymlinkOutput,
}

impl Display for RenderError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedFormat => formatter.write_str("unsupported output format"),
            Self::Json(error) => write!(formatter, "JSON rendering failed: {error}"),
            Self::Io(error) => write!(formatter, "output failed: {error}"),
            Self::SymlinkOutput => {
                formatter.write_str("explicit output may not be a symbolic link")
            }
        }
    }
}

impl Error for RenderError {}
impl From<io::Error> for RenderError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
impl From<serde_json::Error> for RenderError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

/// Selects exactly one deterministic representation.
pub fn render(
    payload: &OutputPayload,
    format: OutputFormat,
) -> Result<RenderedDocument, RenderError> {
    let (mut bytes, machine) = match format {
        OutputFormat::Human => (payload.human.as_bytes().to_vec(), false),
        OutputFormat::Json => (serde_json::to_vec(&payload.json)?, true),
        OutputFormat::Sarif => (
            serde_json::to_vec(
                payload
                    .sarif
                    .as_ref()
                    .ok_or(RenderError::UnsupportedFormat)?,
            )?,
            true,
        ),
        OutputFormat::Markdown => (
            payload
                .markdown
                .as_ref()
                .ok_or(RenderError::UnsupportedFormat)?
                .as_bytes()
                .to_vec(),
            true,
        ),
    };
    while bytes.last() == Some(&b'\n') {
        bytes.pop();
    }
    bytes.push(b'\n');
    Ok(RenderedDocument { bytes, machine })
}

/// Resolves color without permitting color in machine output.
#[must_use]
pub const fn color_enabled(
    format: OutputFormat,
    policy: ColorWhen,
    stdout_is_terminal: bool,
) -> bool {
    if !matches!(format, OutputFormat::Human) {
        return false;
    }
    match policy {
        ColorWhen::Auto => stdout_is_terminal,
        ColorWhen::Always => true,
        ColorWhen::Never => false,
    }
}

/// Writes the selected document to stdout or atomically replaces an explicit output file.
pub fn deliver(
    document: &RenderedDocument,
    output: Option<&Path>,
    stdout: &mut dyn Write,
) -> Result<(), RenderError> {
    let Some(path) = output else {
        stdout.write_all(document.bytes())?;
        stdout.flush()?;
        return Ok(());
    };
    if fs::symlink_metadata(path).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
        return Err(RenderError::SymlinkOutput);
    }
    let parent = path
        .parent()
        .filter(|value| !value.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    temporary.as_file_mut().write_all(document.bytes())?;
    temporary.as_file_mut().sync_all()?;
    temporary
        .persist(path)
        .map_err(|error| RenderError::Io(error.error))?;
    fs::File::open(parent)?.sync_all()?;
    Ok(())
}

/// Stderr-only bounded progress and log sink.
pub struct StderrReporter<'a> {
    stream: &'a mut dyn Write,
    progress: bool,
}

impl<'a> StderrReporter<'a> {
    /// Creates a reporter; `no_progress` suppresses only progress records.
    pub fn new(stream: &'a mut dyn Write, no_progress: bool) -> Self {
        Self {
            stream,
            progress: !no_progress,
        }
    }

    /// Emits one progress line when enabled.
    pub fn progress(&mut self, message: &str) -> io::Result<()> {
        if self.progress {
            writeln!(self.stream, "progress: {}", escape_control(message))?;
        }
        Ok(())
    }

    /// Emits one diagnostic/log line regardless of progress policy.
    pub fn log(&mut self, message: &str) -> io::Result<()> {
        writeln!(self.stream, "log: {}", escape_control(message))
    }
}

fn escape_control(value: &str) -> String {
    value
        .chars()
        .flat_map(|character| match character {
            '\n' => "\\n".chars().collect::<Vec<_>>(),
            '\r' => "\\r".chars().collect(),
            '\t' => "\\t".chars().collect(),
            value if value.is_control() => "�".chars().collect(),
            value => vec![value],
        })
        .collect()
}

/// Resolves an output path for tests and callers without exposing host paths in documents.
#[must_use]
pub fn explicit_output_path(path: Option<PathBuf>) -> Option<PathBuf> {
    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{ParseOutcome, parse};
    use crate::commands;
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::Path;
    use std::process::Command;

    fn payload() -> OutputPayload {
        OutputPayload {
            human: "ok".to_owned(),
            json: serde_json::json!({"status":"ok"}),
            sarif: Some(serde_json::json!({"version":"2.1.0","runs":[]})),
            markdown: Some("# Context".to_owned()),
        }
    }

    #[test]
    fn every_representation_is_one_deterministic_document() -> Result<(), Box<dyn Error>> {
        for format in [
            OutputFormat::Human,
            OutputFormat::Json,
            OutputFormat::Sarif,
            OutputFormat::Markdown,
        ] {
            let first = render(&payload(), format)?;
            let second = render(&payload(), format)?;
            assert_eq!(first, second);
            assert_eq!(
                first.bytes().iter().filter(|byte| **byte == b'\n').count(),
                1
            );
            assert_eq!(first.is_machine(), format != OutputFormat::Human);
        }
        Ok(())
    }

    #[test]
    fn explicit_output_is_atomic_and_leaves_stdout_empty() -> Result<(), Box<dyn Error>> {
        let directory = tempfile::tempdir()?;
        let output = directory.path().join("result.json");
        let document = render(&payload(), OutputFormat::Json)?;
        let mut stdout = Vec::new();
        deliver(&document, Some(&output), &mut stdout)?;
        assert!(stdout.is_empty());
        assert_eq!(fs::read(output)?, document.bytes());
        assert_eq!(fs::read_dir(directory.path())?.count(), 1);
        Ok(())
    }

    #[test]
    fn progress_logs_and_color_obey_output_channels() -> Result<(), Box<dyn Error>> {
        let mut stderr = Vec::new();
        let mut reporter = StderrReporter::new(&mut stderr, true);
        reporter.progress("hidden")?;
        reporter.log("unsafe\u{1b}data")?;
        assert_eq!(String::from_utf8(stderr)?, "log: unsafe�data\n");
        assert!(color_enabled(OutputFormat::Human, ColorWhen::Auto, true));
        assert!(!color_enabled(OutputFormat::Json, ColorWhen::Always, true));
        Ok(())
    }

    #[test]
    fn reviewed_signup_goldens_cover_the_public_surface_and_are_byte_stable()
    -> Result<(), Box<dyn Error>> {
        let golden_root =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/signup/goldens");
        let index: Value = serde_json::from_slice(&fs::read(golden_root.join("index.json"))?)?;
        assert_eq!(index["commands"].as_array().map(Vec::len), Some(21));
        assert_eq!(index["mcp_reads"].as_array().map(Vec::len), Some(4));

        let command_golden = render_public_commands()?;
        if std::env::var_os("EQM_UPDATE_GOLDENS").is_some() {
            fs::write(golden_root.join("commands.json"), &command_golden)?;
        }
        assert_eq!(command_golden, fs::read(golden_root.join("commands.json"))?);

        let repository = materialized_signup_fixture()?;
        let ParseOutcome::Run(parsed) = parse([
            "context",
            "account.create.signup.identifier",
            "--max-bytes",
            "1024",
            "--no-progress",
        ])?
        else {
            return Err("unexpected help".into());
        };
        let mut context = commands::context::execute(parsed, repository.path())?.payload;
        context.json["context"]["evaluated_at"] = Value::String("<evaluated_at>".to_owned());
        for (format, name) in [
            (OutputFormat::Human, "context.human"),
            (OutputFormat::Json, "context.json"),
            (OutputFormat::Markdown, "context.md"),
        ] {
            let actual = render(&context, format)?;
            assert_eq!(actual.bytes(), fs::read(golden_root.join(name))?);
        }

        let ParseOutcome::Run(parsed) = parse(["validate", "--no-progress"])? else {
            return Err("unexpected help".into());
        };
        let validation = commands::validate::execute(parsed, repository.path())?.payload;
        let actual = render(&validation, OutputFormat::Sarif)?;
        assert_eq!(
            actual.bytes(),
            fs::read(golden_root.join("validate.sarif"))?
        );

        for name in ["context.json", "validate.sarif", "mcp-workspace.json"] {
            let bytes = fs::read(golden_root.join(name))?;
            let _: Value = serde_json::from_slice(&bytes)?;
        }
        Ok(())
    }

    fn render_public_commands() -> Result<Vec<u8>, Box<dyn Error>> {
        let repository = git_signup_fixture()?;
        let commit = git_output(repository.path(), &["rev-parse", "HEAD"])?;
        let release_repository =
            commands::release_check::tests::release_fixture("pass", false, "signed_ci")?;
        let empty = tempfile::tempdir()?;
        let cases = [
            ("init", vec!["init", "--dry-run"], empty.path()),
            (
                "new",
                vec![
                    "new",
                    "surface",
                    "account.create.signup.example",
                    "--dry-run",
                ],
                repository.path(),
            ),
            ("fmt", vec!["fmt", "--dry-run"], repository.path()),
            ("validate", vec!["validate"], repository.path()),
            ("check", vec!["check"], repository.path()),
            (
                "show",
                vec!["show", "surface", "account.create.signup.identifier"],
                repository.path(),
            ),
            (
                "locate",
                vec!["locate", "account.create.signup.identifier"],
                repository.path(),
            ),
            (
                "context",
                vec![
                    "context",
                    "account.create.signup.identifier",
                    "--max-bytes",
                    "1024",
                ],
                repository.path(),
            ),
            ("matrix", vec!["matrix", "conformance"], repository.path()),
            ("obligations", vec!["obligations"], repository.path()),
            (
                "diff",
                vec!["--baseline", commit.as_str(), "diff"],
                repository.path(),
            ),
            (
                "affected",
                vec![
                    "--baseline",
                    commit.as_str(),
                    "affected",
                    "--path",
                    "targets/web/src/routes/signup/+page.svelte",
                ],
                repository.path(),
            ),
            (
                "discover",
                vec![
                    "discover",
                    "--adapter",
                    "adapter.missing",
                    "--target",
                    "web",
                ],
                repository.path(),
            ),
            (
                "reconcile",
                vec!["reconcile", "--target", "web"],
                repository.path(),
            ),
            ("verify", vec!["verify", "--dry-run"], repository.path()),
            ("attest", vec!["attest"], release_repository.path()),
            (
                "release_check",
                vec![
                    "--profile",
                    "audience.default",
                    "release",
                    "check",
                    "--release-record",
                    "releases/pass.generated.json",
                ],
                release_repository.path(),
            ),
            ("explain", vec!["explain", "EQM-E0300"], repository.path()),
            ("doctor", vec!["doctor"], repository.path()),
            (
                "lock_update",
                vec!["lock", "update", "--dry-run"],
                repository.path(),
            ),
        ];
        let mut documents = BTreeMap::new();
        for (name, arguments, root) in cases {
            let ParseOutcome::Run(parsed) = parse(arguments)? else {
                return Err(format!("{name}: unexpected help").into());
            };
            let execution = commands::execute(parsed, root)?;
            let mut payload = execution.payload;
            normalize_volatile(&mut payload.json);
            let document = render(&payload, OutputFormat::Json)?;
            documents.insert(
                name,
                serde_json::json!({
                    "exit_code": execution.exit_code,
                    "document": String::from_utf8(document.bytes().to_vec())?,
                }),
            );
        }
        let mut bytes = serde_json::to_vec(&documents)?;
        bytes.push(b'\n');
        Ok(bytes)
    }

    fn normalize_volatile(value: &mut Value) {
        match value {
            Value::Array(values) => values.iter_mut().for_each(normalize_volatile),
            Value::Object(values) => {
                for (key, value) in values {
                    if key == "evaluated_at" {
                        *value = Value::String("<evaluated_at>".to_owned());
                    } else {
                        normalize_volatile(value);
                    }
                }
            }
            _ => {}
        }
    }

    fn git_signup_fixture() -> Result<tempfile::TempDir, Box<dyn Error>> {
        let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/signup");
        let repository = tempfile::tempdir()?;
        copy_git_fixture(&source, repository.path())?;
        git(repository.path(), &["init", "-q"])?;
        git(
            repository.path(),
            &[
                "remote",
                "add",
                "origin",
                "git@github.com:example/signup-fixture.git",
            ],
        )?;
        git(repository.path(), &["add", "."])?;
        git(
            repository.path(),
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
        Ok(repository)
    }

    fn copy_git_fixture(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(destination)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            if entry.file_name() == "GIT_HEAD.fixture" || entry.file_name() == "goldens" {
                continue;
            }
            let name = if entry.file_name() == "eqm.toml.fixture" {
                "eqm.toml".into()
            } else {
                entry.file_name()
            };
            let target = destination.join(name);
            if entry.file_type()?.is_dir() {
                copy_git_fixture(&entry.path(), &target)?;
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
            .env("GIT_AUTHOR_DATE", "2026-08-08T00:00:00Z")
            .env("GIT_COMMITTER_DATE", "2026-08-08T00:00:00Z")
            .status()?
            .success()
        {
            Ok(())
        } else {
            Err("Git fixture command failed".into())
        }
    }

    fn git_output(root: &Path, arguments: &[&str]) -> Result<String, Box<dyn Error>> {
        let output = Command::new("git")
            .args(arguments)
            .current_dir(root)
            .output()?;
        if !output.status.success() {
            return Err("Git fixture query failed".into());
        }
        Ok(String::from_utf8(output.stdout)?.trim().to_owned())
    }

    fn materialized_signup_fixture() -> Result<tempfile::TempDir, Box<dyn Error>> {
        let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/signup");
        let repository = tempfile::tempdir()?;
        copy_fixture(&source, repository.path())?;
        Ok(repository)
    }

    fn copy_fixture(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(destination)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            if entry.file_name() == "GIT_HEAD.fixture" {
                let git = destination.join(".git");
                fs::create_dir_all(&git)?;
                fs::copy(entry.path(), git.join("HEAD"))?;
                continue;
            }
            let name = if entry.file_name() == "eqm.toml.fixture" {
                "eqm.toml".into()
            } else {
                entry.file_name()
            };
            let target = destination.join(name);
            if entry.file_type()?.is_dir() {
                copy_fixture(&entry.path(), &target)?;
            } else {
                fs::copy(entry.path(), target)?;
            }
        }
        Ok(())
    }
}
