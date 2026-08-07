//! Comment-preserving, semantics-safe formatting for authored TOML manifests.

use eqm_domain::RepoPath;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use toml_edit::DocumentMut;

/// Filesystem behavior for a formatting operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormatMode {
    /// Replace changed files atomically.
    Write,
    /// Report whether formatting would be required without writing.
    Check,
    /// Produce formatted contents without writing.
    DryRun,
}

/// Result of formatting one manifest file.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormatOutcome {
    /// Repository-relative source path.
    pub path: RepoPath,
    /// Whether the original bytes differed from the formatted bytes.
    pub changed: bool,
    /// Whether this operation replaced the file.
    pub written: bool,
    /// Formatted contents, present only in dry-run mode.
    pub output: Option<Box<str>>,
}

/// Formats TOML while preserving comments, ordering, quoting, and literal contents.
pub fn format_manifest(source: &str) -> Result<String, FormatError> {
    let document = source
        .parse::<DocumentMut>()
        .map_err(|error| FormatError::Syntax(error.to_string().into_boxed_str()))?;
    let mut formatted = document.to_string();
    while formatted.ends_with("\n\n") {
        formatted.pop();
    }
    if !formatted.ends_with('\n') {
        formatted.push('\n');
    }

    let before = toml::from_str::<toml::Value>(source)
        .map_err(|error| FormatError::Syntax(error.to_string().into_boxed_str()))?;
    let after = toml::from_str::<toml::Value>(&formatted)
        .map_err(|error| FormatError::Syntax(error.to_string().into_boxed_str()))?;
    if before != after {
        return Err(FormatError::SemanticChange);
    }
    Ok(formatted)
}

/// Formats one confined repository file according to `mode`.
pub fn format_manifest_file(
    repository_root: &Path,
    path: &RepoPath,
    mode: FormatMode,
) -> Result<FormatOutcome, FormatError> {
    let absolute = repository_root.join(path.as_str());
    reject_symlink_components(repository_root, &absolute)?;
    let original = fs::read_to_string(&absolute).map_err(FormatError::Io)?;
    let formatted = format_manifest(&original)?;
    let changed = original != formatted;
    let written = changed && mode == FormatMode::Write;
    if written {
        atomic_replace(&absolute, formatted.as_bytes())?;
    }
    Ok(FormatOutcome {
        path: path.clone(),
        changed,
        written,
        output: (mode == FormatMode::DryRun).then(|| formatted.into_boxed_str()),
    })
}

fn reject_symlink_components(root: &Path, path: &Path) -> Result<(), FormatError> {
    let relative = path
        .strip_prefix(root)
        .map_err(|_| FormatError::OutsideRepository)?;
    let mut current = PathBuf::from(root);
    for component in relative.components() {
        current.push(component);
        if fs::symlink_metadata(&current).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
            return Err(FormatError::Symlink(current));
        }
    }
    Ok(())
}

fn atomic_replace(path: &Path, bytes: &[u8]) -> Result<(), FormatError> {
    let parent = path.parent().ok_or(FormatError::OutsideRepository)?;
    let permissions = fs::metadata(path).map_err(FormatError::Io)?.permissions();
    let mut temporary = tempfile::NamedTempFile::new_in(parent).map_err(FormatError::Io)?;
    temporary.write_all(bytes).map_err(FormatError::Io)?;
    temporary.flush().map_err(FormatError::Io)?;
    temporary.as_file().sync_all().map_err(FormatError::Io)?;
    temporary
        .as_file()
        .set_permissions(permissions)
        .map_err(FormatError::Io)?;
    temporary
        .persist(path)
        .map_err(|error| FormatError::Io(error.error))?;
    fs::File::open(parent)
        .and_then(|directory| directory.sync_all())
        .map_err(FormatError::Io)
}

/// Manifest formatting failure.
#[derive(Debug)]
pub enum FormatError {
    /// The input was not valid TOML.
    Syntax(Box<str>),
    /// Formatting would have changed decoded TOML values.
    SemanticChange,
    /// The resolved path escaped the repository root.
    OutsideRepository,
    /// A path component was a symbolic link.
    Symlink(PathBuf),
    /// Filesystem input/output failed.
    Io(io::Error),
}

impl Display for FormatError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax(message) => write!(formatter, "invalid TOML: {message}"),
            Self::SemanticChange => {
                formatter.write_str("formatting changed decoded TOML semantics")
            }
            Self::OutsideRepository => formatter.write_str("format path is outside repository"),
            Self::Symlink(path) => write!(
                formatter,
                "format path contains symlink: {}",
                path.display()
            ),
            Self::Io(error) => write!(formatter, "format I/O failed: {error}"),
        }
    }
}

impl Error for FormatError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GOLDEN: &str = "# workspace\nschema = \"https://schemas.eqm.dev/workspace/v1\" # exact\n\n[extensions.acme]\nnote = \"kept\"\n";

    #[test]
    fn preserves_comments_and_is_idempotent() -> Result<(), FormatError> {
        let once = format_manifest(GOLDEN)?;
        assert_eq!(once, GOLDEN);
        assert_eq!(format_manifest(&once)?, once);
        Ok(())
    }

    #[test]
    fn adds_exactly_one_terminal_newline_without_semantic_change() -> Result<(), FormatError> {
        let source = "value = '''line one\nline two'''";
        let formatted = format_manifest(source)?;
        assert_eq!(formatted, format!("{source}\n"));
        assert_eq!(
            toml::from_str::<toml::Value>(source),
            toml::from_str::<toml::Value>(&formatted)
        );
        Ok(())
    }

    #[test]
    fn modes_write_only_when_requested() -> Result<(), Box<dyn Error>> {
        let directory = tempfile::tempdir()?;
        let path = RepoPath::new("manifest.toml")?;
        let absolute = directory.path().join(path.as_str());
        fs::write(&absolute, "value = 1")?;

        let check = format_manifest_file(directory.path(), &path, FormatMode::Check)?;
        assert!(check.changed);
        assert!(!check.written);
        assert_eq!(fs::read_to_string(&absolute)?, "value = 1");

        let dry_run = format_manifest_file(directory.path(), &path, FormatMode::DryRun)?;
        assert_eq!(dry_run.output.as_deref(), Some("value = 1\n"));
        assert_eq!(fs::read_to_string(&absolute)?, "value = 1");

        let write = format_manifest_file(directory.path(), &path, FormatMode::Write)?;
        assert!(write.written);
        assert_eq!(fs::read_to_string(&absolute)?, "value = 1\n");
        assert!(!format_manifest_file(directory.path(), &path, FormatMode::Write)?.changed);
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn rejects_symlink_targets() -> Result<(), Box<dyn Error>> {
        use std::os::unix::fs::symlink;

        let directory = tempfile::tempdir()?;
        fs::write(directory.path().join("real.toml"), "value = 1")?;
        symlink("real.toml", directory.path().join("link.toml"))?;
        let error = format_manifest_file(
            directory.path(),
            &RepoPath::new("link.toml")?,
            FormatMode::Write,
        )
        .err()
        .ok_or("expected symlink rejection")?;
        assert!(matches!(error, FormatError::Symlink(_)));
        Ok(())
    }
}
