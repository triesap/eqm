//! Deterministic lexical discovery of authored manifest sources.

use crate::WorkspaceConfig;
use eqm_domain::RepoPath;
use globset::{GlobBuilder, GlobSet, GlobSetBuilder};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::path::{Component, Path, PathBuf};
use unicode_normalization::UnicodeNormalization;

/// One closed authored source class.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SourceClass {
    /// Capabilities, journeys, surfaces, and fragments.
    Contract,
    /// Target binding documents.
    Binding,
    /// Policy documents.
    Policy,
    /// Profile documents.
    Profile,
    /// Runner documents.
    Runner,
    /// Waiver documents.
    Waiver,
}

/// One repository-relative source selected for exactly one source class.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveredSource {
    class: SourceClass,
    path: RepoPath,
}

impl DiscoveredSource {
    pub(crate) fn new(class: SourceClass, path: RepoPath) -> Self {
        Self { class, path }
    }

    /// Returns the source class.
    #[must_use]
    pub const fn class(&self) -> SourceClass {
        self.class
    }

    /// Returns the normalized repository-relative path.
    #[must_use]
    pub const fn path(&self) -> &RepoPath {
        &self.path
    }
}

/// Expands every configured source class in stable repository-path order.
pub fn discover_sources(config: &WorkspaceConfig) -> Result<Vec<DiscoveredSource>, DiscoveryError> {
    let config_directory = config
        .config_path()
        .parent()
        .ok_or(DiscoveryError::InvalidConfigDirectory)?;
    let classes = [
        (
            SourceClass::Contract,
            config.dto().contract_sources.as_slice(),
        ),
        (
            SourceClass::Binding,
            config.dto().binding_sources.as_slice(),
        ),
        (SourceClass::Policy, config.dto().policy_sources.as_slice()),
        (
            SourceClass::Profile,
            config.dto().profile_sources.as_slice(),
        ),
        (SourceClass::Runner, config.dto().runner_sources.as_slice()),
        (SourceClass::Waiver, config.dto().waiver_sources.as_slice()),
    ];
    let matchers = classes
        .into_iter()
        .map(|(class, patterns)| Ok((class, build_matcher(patterns)?)))
        .collect::<Result<Vec<_>, DiscoveryError>>()?;

    let mut candidates = Vec::new();
    walk(config.repository_root(), config_directory, &mut candidates)?;
    candidates.sort();

    let mut portable_paths = BTreeMap::<Box<str>, RepoPath>::new();
    let mut discovered = Vec::new();
    for absolute in candidates {
        let relative_to_config = absolute
            .strip_prefix(config_directory)
            .map_err(|_| DiscoveryError::OutsideConfigDirectory)?;
        let match_path = slash_path(relative_to_config)?;
        let matching: Vec<_> = matchers
            .iter()
            .filter_map(|(class, matcher)| matcher.is_match(&match_path).then_some(*class))
            .collect();
        if matching.is_empty() {
            continue;
        }
        if matching.len() != 1 {
            return Err(DiscoveryError::MultipleSourceClasses);
        }
        if fs::symlink_metadata(&absolute)
            .map_err(|_| DiscoveryError::Filesystem)?
            .file_type()
            .is_symlink()
        {
            return Err(DiscoveryError::SymlinkSource);
        }
        let relative = absolute
            .strip_prefix(config.repository_root())
            .map_err(|_| DiscoveryError::OutsideRepository)?;
        let path = RepoPath::new(slash_path(relative)?).map_err(|_| DiscoveryError::InvalidPath)?;
        if let Some(existing) = portable_paths.insert(path.portable_collision_key(), path.clone())
            && existing != path
        {
            return Err(DiscoveryError::PortableCollision);
        }
        discovered.push(DiscoveredSource::new(matching[0], path));
    }
    discovered.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(discovered)
}

fn build_matcher(patterns: &[String]) -> Result<GlobSet, DiscoveryError> {
    let mut normalized = BTreeSet::new();
    for pattern in patterns {
        validate_pattern(pattern)?;
        normalized.insert(pattern);
    }
    let mut builder = GlobSetBuilder::new();
    for pattern in normalized {
        let glob = GlobBuilder::new(pattern)
            .literal_separator(true)
            .backslash_escape(false)
            .build()
            .map_err(|_| DiscoveryError::InvalidPattern)?;
        builder.add(glob);
    }
    builder.build().map_err(|_| DiscoveryError::InvalidPattern)
}

fn validate_pattern(pattern: &str) -> Result<(), DiscoveryError> {
    if pattern.is_empty()
        || pattern.len() > 1_024
        || pattern.starts_with('/')
        || pattern.starts_with("//")
        || pattern.contains('\\')
        || pattern.chars().any(char::is_control)
        || pattern.nfc().ne(pattern.chars())
    {
        return Err(DiscoveryError::InvalidPattern);
    }
    for (index, segment) in pattern.split('/').enumerate() {
        let bytes = segment.as_bytes();
        let drive =
            index == 0 && matches!(bytes, [letter, b':', ..] if letter.is_ascii_alphabetic());
        if segment.is_empty() || segment == "." || segment == ".." || drive {
            return Err(DiscoveryError::InvalidPattern);
        }
    }
    Ok(())
}

fn walk(root: &Path, directory: &Path, files: &mut Vec<PathBuf>) -> Result<(), DiscoveryError> {
    if directory != root && directory.join(".git").exists() {
        return Ok(());
    }
    let mut entries: Vec<_> = fs::read_dir(directory)
        .map_err(|_| DiscoveryError::Filesystem)?
        .collect::<Result<_, _>>()
        .map_err(|_| DiscoveryError::Filesystem)?;
    entries.sort_by_key(fs::DirEntry::file_name);
    for entry in entries {
        let name = entry.file_name();
        if name == ".git" || name == ".eqm" {
            continue;
        }
        let file_type = entry.file_type().map_err(|_| DiscoveryError::Filesystem)?;
        if file_type.is_dir() {
            walk(root, &entry.path(), files)?;
        } else if file_type.is_file() || file_type.is_symlink() {
            files.push(entry.path());
        }
    }
    Ok(())
}

fn slash_path(path: &Path) -> Result<String, DiscoveryError> {
    let mut segments = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(value) => segments.push(
                value
                    .to_str()
                    .ok_or(DiscoveryError::InvalidPath)?
                    .to_owned(),
            ),
            _ => return Err(DiscoveryError::InvalidPath),
        }
    }
    Ok(segments.join("/"))
}

/// Deterministic source-discovery failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiscoveryError {
    /// The selected config had no parent directory.
    InvalidConfigDirectory,
    /// A glob was non-portable, escaping, malformed, or too large.
    InvalidPattern,
    /// Filesystem enumeration or input failed.
    Filesystem,
    /// A discovered path was not normalized and portable.
    InvalidPath,
    /// A candidate escaped the config directory.
    OutsideConfigDirectory,
    /// A discovered file escaped the repository root.
    OutsideRepository,
    /// Authored metadata was a symlink.
    SymlinkSource,
    /// Two paths collide under portable comparison.
    PortableCollision,
    /// One file matched more than one source class.
    MultipleSourceClasses,
}

impl Display for DiscoveryError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for DiscoveryError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::select_workspace_config;

    const CONFIG: &str = r#"schema = "https://schemas.equivalencematrix.dev/v1/workspace"
contract_sources = ["eqm/contracts/**/*.toml"]
binding_sources = ["eqm/bindings/**/*.toml"]
policy_sources = ["eqm/policies/**/*.toml"]
profile_sources = ["eqm/profiles/**/*.toml"]
runner_sources = ["eqm/runners/**/*.toml"]
waiver_sources = ["eqm/waivers/**/*.toml"]
"#;

    fn repository() -> Result<tempfile::TempDir, Box<dyn Error>> {
        let directory = tempfile::tempdir()?;
        fs::create_dir(directory.path().join(".git"))?;
        fs::write(directory.path().join("eqm.toml"), CONFIG)?;
        for source in [
            "contracts",
            "bindings",
            "policies",
            "profiles",
            "runners",
            "waivers",
        ] {
            fs::create_dir_all(directory.path().join("eqm").join(source))?;
        }
        Ok(directory)
    }

    #[test]
    fn discovery_is_sorted_and_excludes_generated_and_nested_repositories()
    -> Result<(), Box<dyn Error>> {
        let repository = repository()?;
        fs::write(
            repository.path().join("eqm/contracts/z.toml"),
            "id = 'cap_z'",
        )?;
        fs::write(
            repository.path().join("eqm/contracts/a.toml"),
            "id = 'cap_a'",
        )?;
        fs::create_dir_all(repository.path().join(".eqm/contracts"))?;
        fs::write(
            repository.path().join(".eqm/contracts/ignored.toml"),
            "id = 'ignored'",
        )?;
        fs::create_dir_all(repository.path().join("nested/.git"))?;
        fs::create_dir_all(repository.path().join("nested/eqm/contracts"))?;
        fs::write(
            repository.path().join("nested/eqm/contracts/ignored.toml"),
            "id = 'nested'",
        )?;
        let config = select_workspace_config(repository.path(), None)?;
        let paths: Vec<_> = discover_sources(&config)?
            .into_iter()
            .map(|source| source.path().as_str().to_owned())
            .collect();
        assert_eq!(paths, ["eqm/contracts/a.toml", "eqm/contracts/z.toml"]);
        Ok(())
    }

    #[test]
    fn cross_class_matches_fail() -> Result<(), Box<dyn Error>> {
        let repository = repository()?;
        fs::write(
            repository.path().join("eqm.toml"),
            CONFIG.replace(
                "binding_sources = [\"eqm/bindings/**/*.toml\"]",
                "binding_sources = [\"eqm/contracts/**/*.toml\"]",
            ),
        )?;
        fs::write(
            repository.path().join("eqm/contracts/a.toml"),
            "id = 'cap_same'",
        )?;
        let config = select_workspace_config(repository.path(), None)?;
        assert_eq!(
            discover_sources(&config),
            Err(DiscoveryError::MultipleSourceClasses)
        );
        Ok(())
    }

    #[test]
    fn portable_collisions_and_symlink_sources_fail() -> Result<(), Box<dyn Error>> {
        let repository = repository()?;
        fs::write(
            repository.path().join("eqm/contracts/View.toml"),
            "id = 'cap_upper'",
        )?;
        fs::write(
            repository.path().join("eqm/contracts/view.toml"),
            "id = 'cap_lower'",
        )?;
        let config = select_workspace_config(repository.path(), None)?;
        let entry_count = fs::read_dir(repository.path().join("eqm/contracts"))?.count();
        if entry_count == 2 {
            assert_eq!(
                discover_sources(&config),
                Err(DiscoveryError::PortableCollision)
            );
        }

        fs::remove_file(repository.path().join("eqm/contracts/view.toml"))?;
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(
                repository.path().join("eqm/contracts/View.toml"),
                repository.path().join("eqm/contracts/link.toml"),
            )?;
            assert_eq!(
                discover_sources(&config),
                Err(DiscoveryError::SymlinkSource)
            );
        }
        Ok(())
    }

    #[test]
    fn invalid_patterns_fail_before_traversal() -> Result<(), Box<dyn Error>> {
        let repository = repository()?;
        fs::write(
            repository.path().join("eqm.toml"),
            CONFIG.replace("eqm/contracts/**/*.toml", "../outside/**/*.toml"),
        )?;
        let config = select_workspace_config(repository.path(), None)?;
        assert_eq!(
            discover_sources(&config),
            Err(DiscoveryError::InvalidPattern)
        );
        Ok(())
    }
}
