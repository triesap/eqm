//! Typed one-value-per-argument runner substitution.

use eqm_domain::{ArgumentTemplate, RunnerDefinition};
use serde_json::Value;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::path::{Path, PathBuf};

const MAX_SELECTOR_BYTES: usize = 1024 * 1024;

/// Exact invocation-scoped placeholder values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvocationBindings {
    target_root: PathBuf,
    selector_json: Box<str>,
    result_path: PathBuf,
}

impl InvocationBindings {
    /// Validates confined absolute paths and compacts one JSON object selector.
    pub fn new(
        target_root: PathBuf,
        selector_json: &str,
        result_path: PathBuf,
    ) -> Result<Self, SubstitutionError> {
        validate_path(&target_root)?;
        validate_path(&result_path)?;
        let selector: Value =
            serde_json::from_str(selector_json).map_err(|_| SubstitutionError::InvalidSelector)?;
        if !selector.is_object() {
            return Err(SubstitutionError::InvalidSelector);
        }
        let selector_json =
            serde_json::to_string(&selector).map_err(|_| SubstitutionError::InvalidSelector)?;
        if selector_json.len() > MAX_SELECTOR_BYTES {
            return Err(SubstitutionError::SelectorTooLarge);
        }
        Ok(Self {
            target_root,
            selector_json: selector_json.into(),
            result_path,
        })
    }

    /// Returns the confined target root.
    #[must_use]
    pub fn target_root(&self) -> &Path {
        &self.target_root
    }

    /// Returns compact validated selector JSON.
    #[must_use]
    pub fn selector_json(&self) -> &str {
        &self.selector_json
    }

    /// Returns the confined result path.
    #[must_use]
    pub fn result_path(&self) -> &Path {
        &self.result_path
    }
}

/// Substitutes typed placeholders without parsing or invoking a shell.
pub fn substitute_argv(
    definition: &RunnerDefinition,
    bindings: &InvocationBindings,
) -> Result<Vec<String>, SubstitutionError> {
    let mut target_roots = 0_u8;
    let mut selectors = 0_u8;
    let mut result_paths = 0_u8;
    let mut argv = Vec::with_capacity(definition.args().len());
    for argument in definition.args() {
        let value = match argument {
            ArgumentTemplate::Literal(value) => value.as_str().to_owned(),
            ArgumentTemplate::TargetRoot => {
                target_roots = target_roots.saturating_add(1);
                path_text(bindings.target_root())?
            }
            ArgumentTemplate::SelectorJson => {
                selectors = selectors.saturating_add(1);
                bindings.selector_json().to_owned()
            }
            ArgumentTemplate::ResultPath => {
                result_paths = result_paths.saturating_add(1);
                path_text(bindings.result_path())?
            }
        };
        argv.push(value);
    }
    if target_roots > 1 || selectors > 1 || result_paths > 1 {
        return Err(SubstitutionError::DuplicatePlaceholder);
    }
    Ok(argv)
}

fn validate_path(path: &Path) -> Result<(), SubstitutionError> {
    if !path.is_absolute() || path.as_os_str().is_empty() {
        return Err(SubstitutionError::InvalidPath);
    }
    path_text(path).map(|_| ())
}

fn path_text(path: &Path) -> Result<String, SubstitutionError> {
    let value = path.to_str().ok_or(SubstitutionError::InvalidPath)?;
    if value.as_bytes().contains(&0) {
        return Err(SubstitutionError::InvalidPath);
    }
    Ok(value.to_owned())
}

/// Typed substitution failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubstitutionError {
    /// A path was relative, non-UTF-8, empty, or contained NUL.
    InvalidPath,
    /// Selector was not one valid JSON object.
    InvalidSelector,
    /// Compact selector exceeded its fixed bound.
    SelectorTooLarge,
    /// One execution-sensitive placeholder occurred more than once.
    DuplicatePlaceholder,
}

impl Display for SubstitutionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for SubstitutionError {}

#[cfg(test)]
mod tests {
    use super::*;
    use eqm_domain::{
        DurationMillis, Extensions, PositiveCount, RepoPath, Revision, RunnerBackend, RunnerId,
        RunnerLimits, RunnerProgram, SelectorText,
    };
    use std::error::Error;

    fn definition(args: Vec<ArgumentTemplate>) -> Result<RunnerDefinition, Box<dyn Error>> {
        Ok(RunnerDefinition::new(
            RunnerId::new("runner.tests")?,
            Revision::new(1)?,
            vec!["owner://team/platform".parse()?],
            RunnerBackend::Local,
            RunnerProgram::Repository(RepoPath::new("tools/test-runner")?),
            args,
            None,
            Vec::new(),
            Vec::new(),
            RunnerLimits::new(
                DurationMillis::new(30_000)?,
                PositiveCount::new(1_024)?,
                None,
            )?,
            Vec::new(),
            Extensions::default(),
        )?)
    }

    #[test]
    fn metacharacters_remain_literal_single_arguments() -> Result<(), Box<dyn Error>> {
        let runner = definition(vec![
            ArgumentTemplate::Literal(SelectorText::new("--selector")?),
            ArgumentTemplate::SelectorJson,
            ArgumentTemplate::TargetRoot,
            ArgumentTemplate::ResultPath,
        ])?;
        let bindings = InvocationBindings::new(
            PathBuf::from("/tmp/root;touch injected"),
            r#"{"query":"$(touch nope); `false` | cat"}"#,
            PathBuf::from("/tmp/result && false"),
        )?;
        let argv = substitute_argv(&runner, &bindings)?;
        assert_eq!(argv.len(), 4);
        assert_eq!(argv[1], r#"{"query":"$(touch nope); `false` | cat"}"#);
        assert_eq!(argv[2], "/tmp/root;touch injected");
        assert_eq!(argv[3], "/tmp/result && false");
        Ok(())
    }

    #[test]
    fn duplicate_and_invalid_values_fail_before_execution() -> Result<(), Box<dyn Error>> {
        let duplicate = definition(vec![
            ArgumentTemplate::SelectorJson,
            ArgumentTemplate::SelectorJson,
        ])?;
        let bindings = InvocationBindings::new(
            PathBuf::from("/tmp/root"),
            "{}",
            PathBuf::from("/tmp/result"),
        )?;
        assert_eq!(
            substitute_argv(&duplicate, &bindings),
            Err(SubstitutionError::DuplicatePlaceholder)
        );
        assert_eq!(
            InvocationBindings::new(
                PathBuf::from("relative"),
                "[]",
                PathBuf::from("/tmp/result")
            ),
            Err(SubstitutionError::InvalidPath)
        );
        assert_eq!(
            InvocationBindings::new(
                PathBuf::from("/tmp/root"),
                "[]",
                PathBuf::from("/tmp/result")
            ),
            Err(SubstitutionError::InvalidSelector)
        );
        Ok(())
    }
}
