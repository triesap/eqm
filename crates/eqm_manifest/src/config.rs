//! Repository-bound workspace configuration selection.

use crate::{ParseError, dto::WorkspaceDto, parse_toml};
use eqm_domain::{RepoPath, SchemaKind, SchemaUri, SourceName};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};

/// One selected and strictly decoded workspace configuration.
#[derive(Clone, Debug, PartialEq)]
pub struct WorkspaceConfig {
    repository_root: PathBuf,
    config_path: PathBuf,
    dto: WorkspaceDto,
}

impl WorkspaceConfig {
    /// Returns the canonical VCS root.
    #[must_use]
    pub fn repository_root(&self) -> &Path {
        &self.repository_root
    }
    /// Returns the canonical selected config path.
    #[must_use]
    pub fn config_path(&self) -> &Path {
        &self.config_path
    }
    /// Returns the strict source DTO.
    #[must_use]
    pub const fn dto(&self) -> &WorkspaceDto {
        &self.dto
    }
}

/// Selects the one workspace config within the nearest VCS boundary.
pub fn select_workspace_config(
    start: &Path,
    explicit: Option<&RepoPath>,
) -> Result<WorkspaceConfig, ConfigError> {
    let start = fs::canonicalize(start).map_err(|_| ConfigError::StartUnavailable)?;
    let start_dir = if start.is_file() {
        start.parent().ok_or(ConfigError::StartUnavailable)?
    } else {
        &start
    };
    let repository_root = find_repository_root(start_dir)?;
    let config_path = if let Some(relative) = explicit {
        let candidate = repository_root.join(relative.as_str());
        if fs::symlink_metadata(&candidate)
            .map_err(|_| ConfigError::ConfigUnavailable)?
            .file_type()
            .is_symlink()
        {
            return Err(ConfigError::SymlinkConfig);
        }
        let canonical = fs::canonicalize(candidate).map_err(|_| ConfigError::ConfigUnavailable)?;
        if !canonical.starts_with(&repository_root) {
            return Err(ConfigError::OutsideRepository);
        }
        canonical
    } else {
        let mut configs = Vec::new();
        find_configs(&repository_root, &repository_root, &mut configs)?;
        match configs.as_slice() {
            [] => return Err(ConfigError::ConfigUnavailable),
            [only] => only.clone(),
            _ => return Err(ConfigError::MultipleConfigs),
        }
    };
    let relative = config_path
        .strip_prefix(&repository_root)
        .map_err(|_| ConfigError::OutsideRepository)?;
    let source = SourceName::new(relative.to_string_lossy().replace('\\', "/"))
        .map_err(|_| ConfigError::InvalidSourceName)?;
    let bytes = fs::read(&config_path).map_err(|_| ConfigError::ConfigUnavailable)?;
    let parsed = parse_toml(source, &bytes).map_err(ConfigError::Parse)?;
    let dto: WorkspaceDto =
        toml::from_str(parsed.text()).map_err(|_| ConfigError::InvalidConfig)?;
    validate_declarations(&dto)?;
    Ok(WorkspaceConfig {
        repository_root,
        config_path,
        dto,
    })
}

fn find_repository_root(start: &Path) -> Result<PathBuf, ConfigError> {
    start
        .ancestors()
        .find(|path| path.join(".git").exists())
        .map(Path::to_path_buf)
        .ok_or(ConfigError::RepositoryNotFound)
}

fn find_configs(
    root: &Path,
    directory: &Path,
    configs: &mut Vec<PathBuf>,
) -> Result<(), ConfigError> {
    if directory != root && directory.join(".git").exists() {
        return Ok(());
    }
    let mut entries: Vec<_> = fs::read_dir(directory)
        .map_err(|_| ConfigError::Discovery)?
        .collect::<Result<_, _>>()
        .map_err(|_| ConfigError::Discovery)?;
    entries.sort_by_key(fs::DirEntry::file_name);
    for entry in entries {
        let name = entry.file_name();
        if name == ".git" || name == ".eqm" {
            continue;
        }
        let file_type = entry.file_type().map_err(|_| ConfigError::Discovery)?;
        if file_type.is_symlink() {
            if name == "eqm.toml" {
                return Err(ConfigError::SymlinkConfig);
            }
        } else if file_type.is_dir() {
            find_configs(root, &entry.path(), configs)?;
        } else if file_type.is_file() && name == "eqm.toml" {
            configs.push(fs::canonicalize(entry.path()).map_err(|_| ConfigError::Discovery)?);
        }
    }
    Ok(())
}

fn validate_declarations(dto: &WorkspaceDto) -> Result<(), ConfigError> {
    let schema: SchemaUri = dto.schema.parse().map_err(|_| ConfigError::InvalidConfig)?;
    if schema.kind() != SchemaKind::Workspace {
        return Err(ConfigError::InvalidConfig);
    }
    if dto.contract_sources.is_empty()
        || dto.binding_sources.is_empty()
        || dto.policy_sources.is_empty()
        || dto.profile_sources.is_empty()
        || dto.runner_sources.is_empty()
        || dto.waiver_sources.is_empty()
    {
        return Err(ConfigError::SourceClassRequired);
    }
    if dto
        .generated_root
        .as_deref()
        .is_some_and(|value| value != ".eqm")
    {
        return Err(ConfigError::InvalidGeneratedRoot);
    }
    Ok(())
}

/// Workspace selection or decoding failure.
#[derive(Debug)]
pub enum ConfigError {
    /// Start path was unavailable.
    StartUnavailable,
    /// No containing VCS boundary existed.
    RepositoryNotFound,
    /// Default or explicit config was unavailable.
    ConfigUnavailable,
    /// Default discovery found more than one config.
    MultipleConfigs,
    /// A config resolved outside the repository.
    OutsideRepository,
    /// A config path was a symlink.
    SymlinkConfig,
    /// Directory traversal failed.
    Discovery,
    /// Source-name conversion failed.
    InvalidSourceName,
    /// Bounded TOML parsing failed.
    Parse(ParseError),
    /// Strict workspace DTO decoding failed.
    InvalidConfig,
    /// One required source class was empty.
    SourceClassRequired,
    /// Generated root differed from `.eqm`.
    InvalidGeneratedRoot,
}

impl Display for ConfigError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    const CONFIG: &str = r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/workspace.schema.json"
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
        Ok(directory)
    }

    #[test]
    fn default_and_explicit_selection_stay_in_nearest_repository() -> Result<(), Box<dyn Error>> {
        let repository = repository()?;
        fs::create_dir_all(repository.path().join("src/nested"))?;
        let selected = select_workspace_config(&repository.path().join("src/nested"), None)?;
        assert_eq!(selected.config_path(), repository.path().join("eqm.toml"));
        let explicit = RepoPath::new("eqm.toml")?;
        assert_eq!(
            select_workspace_config(repository.path(), Some(&explicit))?.config_path(),
            selected.config_path()
        );
        Ok(())
    }

    #[test]
    fn duplicate_nested_configs_fail_deterministically() -> Result<(), Box<dyn Error>> {
        let repository = repository()?;
        fs::create_dir(repository.path().join("nested"))?;
        fs::write(repository.path().join("nested/eqm.toml"), CONFIG)?;
        assert!(matches!(
            select_workspace_config(repository.path(), None),
            Err(ConfigError::MultipleConfigs)
        ));
        Ok(())
    }

    #[test]
    fn missing_vcs_and_empty_source_classes_fail() -> Result<(), Box<dyn Error>> {
        let directory = tempfile::tempdir()?;
        assert!(matches!(
            select_workspace_config(directory.path(), None),
            Err(ConfigError::RepositoryNotFound)
        ));
        let repository = repository()?;
        fs::write(
            repository.path().join("eqm.toml"),
            CONFIG.replace(
                "contract_sources = [\"eqm/contracts/**/*.toml\"]",
                "contract_sources = []",
            ),
        )?;
        assert!(matches!(
            select_workspace_config(repository.path(), None),
            Err(ConfigError::SourceClassRequired)
        ));
        Ok(())
    }
}
