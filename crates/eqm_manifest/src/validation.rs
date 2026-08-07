//! Exact-current schema dispatch and bounded strict DTO decoding.

use crate::dto::{
    BindingDto, CapabilityDto, FragmentDto, JourneyDto, PolicyDto, ProfileDto, RunnerDto,
    SurfaceDto, WaiverDto,
};
use crate::{DiscoveredSource, SourceClass, parse_toml};
use eqm_domain::{
    ExtensionKey, ExtensionNamespace, ExtensionValue, Extensions, RepoPath, SchemaKind, SchemaUri,
    SourceName,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::path::Path;

const MAX_TOTAL_BYTES: u64 = 64 * 1024 * 1024;
const MAX_DOCUMENTS: usize = 10_000;
const MAX_DEPTH: usize = 64;
const MAX_STRING_BYTES: usize = 1024 * 1024;
const MAX_CONTAINER_ENTRIES: usize = 100_000;

/// One strictly decoded authored document.
#[derive(Clone, Debug, PartialEq)]
pub enum DocumentDto {
    /// Capability authority.
    Capability(CapabilityDto),
    /// Journey authority.
    Journey(JourneyDto),
    /// Surface authority.
    Surface(SurfaceDto),
    /// Fragment authority.
    Fragment(FragmentDto),
    /// Binding authority.
    Binding(BindingDto),
    /// Policy authority.
    Policy(PolicyDto),
    /// Profile authority.
    Profile(ProfileDto),
    /// Runner authority.
    Runner(RunnerDto),
    /// Waiver authority.
    Waiver(WaiverDto),
}

impl DocumentDto {
    fn authority_key(&self) -> Box<str> {
        match self {
            Self::Capability(dto) => format!("capability:{}", dto.id),
            Self::Journey(dto) => format!("journey:{}", dto.id),
            Self::Surface(dto) => format!("surface:{}", dto.id),
            Self::Fragment(dto) => format!("fragment:{}", dto.id),
            Self::Binding(dto) => format!("binding:{}\0{}", dto.target, dto.unit),
            Self::Policy(dto) => format!("policy:{}", dto.id),
            Self::Profile(dto) => format!("profile:{}", dto.id),
            Self::Runner(dto) => format!("runner:{}", dto.id),
            Self::Waiver(dto) => format!("waiver:{}", dto.id),
        }
        .into_boxed_str()
    }
}

/// A strict DTO paired with its stable repository source identity.
#[derive(Clone, Debug, PartialEq)]
pub struct ValidatedDocument {
    source: RepoPath,
    document: DocumentDto,
}

impl ValidatedDocument {
    pub(crate) fn new(source: RepoPath, document: DocumentDto) -> Self {
        Self { source, document }
    }

    /// Returns the repository-relative source path.
    #[must_use]
    pub const fn source(&self) -> &RepoPath {
        &self.source
    }

    /// Returns the decoded authored document.
    #[must_use]
    pub const fn document(&self) -> &DocumentDto {
        &self.document
    }
}

/// Decodes discovered sources with aggregate limits and duplicate rejection.
pub fn decode_sources(
    repository_root: &Path,
    sources: &[DiscoveredSource],
) -> Result<Vec<ValidatedDocument>, ValidationError> {
    if sources.len() > MAX_DOCUMENTS {
        return Err(ValidationError::new(
            ValidationErrorKind::TooManyDocuments,
            None,
        ));
    }
    let mut total_bytes = 0_u64;
    let mut authorities = BTreeSet::new();
    let mut documents = Vec::with_capacity(sources.len());
    for source in sources {
        let path = repository_root.join(source.path().as_str());
        let metadata = fs::metadata(&path)
            .map_err(|_| ValidationError::at(ValidationErrorKind::Filesystem, source.path()))?;
        total_bytes = total_bytes.checked_add(metadata.len()).ok_or_else(|| {
            ValidationError::at(ValidationErrorKind::TotalBytesExceeded, source.path())
        })?;
        if total_bytes > MAX_TOTAL_BYTES {
            return Err(ValidationError::at(
                ValidationErrorKind::TotalBytesExceeded,
                source.path(),
            ));
        }
        let bytes = fs::read(path)
            .map_err(|_| ValidationError::at(ValidationErrorKind::Filesystem, source.path()))?;
        let source_name = SourceName::new(source.path().as_str()).map_err(|_| {
            ValidationError::at(ValidationErrorKind::InvalidSourceName, source.path())
        })?;
        let parsed = parse_toml(source_name, &bytes)
            .map_err(|_| ValidationError::at(ValidationErrorKind::InvalidToml, source.path()))?;
        validate_table(parsed.root(), 1)
            .map_err(|kind| ValidationError::at(kind, source.path()))?;
        validate_extensions(parsed.root())
            .map_err(|kind| ValidationError::at(kind, source.path()))?;
        let document = dispatch(parsed.text(), source.class())
            .map_err(|kind| ValidationError::at(kind, source.path()))?;
        if !authorities.insert(document.authority_key()) {
            return Err(ValidationError::at(
                ValidationErrorKind::DuplicateAuthority,
                source.path(),
            ));
        }
        documents.push(ValidatedDocument::new(source.path().clone(), document));
    }
    Ok(documents)
}

fn dispatch(text: &str, class: SourceClass) -> Result<DocumentDto, ValidationErrorKind> {
    let table: toml::Table = toml::from_str(text).map_err(|_| ValidationErrorKind::InvalidToml)?;
    let schema = table
        .get("schema")
        .and_then(toml::Value::as_str)
        .ok_or(ValidationErrorKind::MissingSchema)?;
    let schema: SchemaUri = schema
        .parse()
        .map_err(|_| ValidationErrorKind::WrongSchema)?;
    let kind = schema.kind();
    let allowed = matches!(
        (class, kind),
        (
            SourceClass::Contract,
            SchemaKind::Capability
                | SchemaKind::Journey
                | SchemaKind::Surface
                | SchemaKind::Fragment
        ) | (SourceClass::Binding, SchemaKind::Binding)
            | (SourceClass::Policy, SchemaKind::Policy)
            | (SourceClass::Profile, SchemaKind::Profile)
            | (SourceClass::Runner, SchemaKind::Runner)
            | (SourceClass::Waiver, SchemaKind::Waiver)
    );
    if !allowed {
        return Err(ValidationErrorKind::WrongSourceClass);
    }
    macro_rules! decode {
        ($type:ty, $variant:ident) => {
            toml::from_str::<$type>(text)
                .map(DocumentDto::$variant)
                .map_err(|_| ValidationErrorKind::InvalidFields)
        };
    }
    match kind {
        SchemaKind::Capability => decode!(CapabilityDto, Capability),
        SchemaKind::Journey => decode!(JourneyDto, Journey),
        SchemaKind::Surface => decode!(SurfaceDto, Surface),
        SchemaKind::Fragment => decode!(FragmentDto, Fragment),
        SchemaKind::Binding => decode!(BindingDto, Binding),
        SchemaKind::Policy => decode!(PolicyDto, Policy),
        SchemaKind::Profile => decode!(ProfileDto, Profile),
        SchemaKind::Runner => decode!(RunnerDto, Runner),
        SchemaKind::Waiver => decode!(WaiverDto, Waiver),
        _ => Err(ValidationErrorKind::WrongSchema),
    }
}

fn validate_value(value: &toml::Value, depth: usize) -> Result<(), ValidationErrorKind> {
    if depth > MAX_DEPTH {
        return Err(ValidationErrorKind::NestingExceeded);
    }
    match value {
        toml::Value::String(value) if value.len() > MAX_STRING_BYTES => {
            Err(ValidationErrorKind::StringExceeded)
        }
        toml::Value::Array(values) => {
            if values.len() > MAX_CONTAINER_ENTRIES {
                return Err(ValidationErrorKind::ContainerExceeded);
            }
            for value in values {
                validate_value(value, depth + 1)?;
            }
            Ok(())
        }
        toml::Value::Table(values) => validate_table(values, depth),
        _ => Ok(()),
    }
}

fn validate_table(table: &toml::Table, depth: usize) -> Result<(), ValidationErrorKind> {
    if depth > MAX_DEPTH {
        return Err(ValidationErrorKind::NestingExceeded);
    }
    if table.len() > MAX_CONTAINER_ENTRIES {
        return Err(ValidationErrorKind::ContainerExceeded);
    }
    for (key, value) in table {
        if key.len() > MAX_STRING_BYTES {
            return Err(ValidationErrorKind::StringExceeded);
        }
        validate_value(value, depth + 1)?;
    }
    Ok(())
}

fn validate_extensions(table: &toml::Table) -> Result<(), ValidationErrorKind> {
    for (key, value) in table {
        if key == "extensions" {
            let values = value
                .as_table()
                .ok_or(ValidationErrorKind::InvalidExtensions)?;
            let converted = values
                .iter()
                .map(|(namespace, value)| {
                    Ok((
                        ExtensionNamespace::new(namespace.as_str())
                            .map_err(|_| ValidationErrorKind::InvalidExtensions)?,
                        extension_value(value)?,
                    ))
                })
                .collect::<Result<BTreeMap<_, _>, ValidationErrorKind>>()?;
            Extensions::new(converted).map_err(|_| ValidationErrorKind::InvalidExtensions)?;
        }
        match value {
            toml::Value::Table(nested) => validate_extensions(nested)?,
            toml::Value::Array(values) => {
                for value in values {
                    if let Some(nested) = value.as_table() {
                        validate_extensions(nested)?;
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn extension_value(value: &toml::Value) -> Result<ExtensionValue, ValidationErrorKind> {
    match value {
        toml::Value::Boolean(value) => Ok(ExtensionValue::Boolean(*value)),
        toml::Value::Integer(value) => Ok(ExtensionValue::Integer(*value)),
        toml::Value::String(value) => ExtensionValue::string(value.as_str())
            .map_err(|_| ValidationErrorKind::InvalidExtensions),
        toml::Value::Array(values) => values
            .iter()
            .map(extension_value)
            .collect::<Result<Vec<_>, _>>()
            .map(ExtensionValue::Array),
        toml::Value::Table(values) => values
            .iter()
            .map(|(key, value)| {
                Ok((
                    ExtensionKey::new(key.as_str())
                        .map_err(|_| ValidationErrorKind::InvalidExtensions)?,
                    extension_value(value)?,
                ))
            })
            .collect::<Result<BTreeMap<_, _>, _>>()
            .map(ExtensionValue::Object),
        toml::Value::Float(_) | toml::Value::Datetime(_) => {
            Err(ValidationErrorKind::InvalidExtensions)
        }
    }
}

/// Source-associated strict validation failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationError {
    kind: ValidationErrorKind,
    source: Option<RepoPath>,
}

impl ValidationError {
    fn new(kind: ValidationErrorKind, source: Option<RepoPath>) -> Self {
        Self { kind, source }
    }
    fn at(kind: ValidationErrorKind, source: &RepoPath) -> Self {
        Self::new(kind, Some(source.clone()))
    }
    /// Returns the stable error classification.
    #[must_use]
    pub const fn kind(&self) -> ValidationErrorKind {
        self.kind
    }
    /// Returns the repository source when one document caused the error.
    #[must_use]
    pub const fn source(&self) -> Option<&RepoPath> {
        self.source.as_ref()
    }
}

impl Display for ValidationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        if let Some(source) = &self.source {
            write!(formatter, "{}: {:?}", source, self.kind)
        } else {
            write!(formatter, "{:?}", self.kind)
        }
    }
}

impl Error for ValidationError {}

/// Stable strict-validation error classification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationErrorKind {
    /// More than 10,000 documents were supplied.
    TooManyDocuments,
    /// Aggregate authored bytes exceeded 64 MiB.
    TotalBytesExceeded,
    /// Input could not be read or inspected.
    Filesystem,
    /// Repository path could not become a source identity.
    InvalidSourceName,
    /// TOML was malformed, oversized, or not UTF-8.
    InvalidToml,
    /// TOML nesting exceeded 64 levels.
    NestingExceeded,
    /// A string exceeded 1 MiB.
    StringExceeded,
    /// An array or table exceeded 100,000 entries.
    ContainerExceeded,
    /// The schema field was absent or not a string.
    MissingSchema,
    /// The schema was not an exact current authored schema.
    WrongSchema,
    /// The document schema was not allowed in its configured source class.
    WrongSourceClass,
    /// Strict DTO decoding rejected fields or shapes.
    InvalidFields,
    /// Extension namespace, key, type, or bounds were invalid.
    InvalidExtensions,
    /// A semantic authority appeared more than once.
    DuplicateAuthority,
}

#[cfg(test)]
mod tests {
    use super::*;

    const CAPABILITY: &str = r#"schema = "https://schemas.equivalencematrix.dev/v1/capability"
id = "account.create"
title = "Account creation"
status = "active"
owners = ["owner://team/accounts"]
"#;

    fn source(path: &str) -> Result<DiscoveredSource, Box<dyn Error>> {
        Ok(DiscoveredSource::new(
            SourceClass::Contract,
            RepoPath::new(path)?,
        ))
    }

    #[test]
    fn exact_current_schema_decodes_and_retains_source() -> Result<(), Box<dyn Error>> {
        let repository = tempfile::tempdir()?;
        fs::create_dir(repository.path().join("eqm"))?;
        fs::write(repository.path().join("eqm/capability.toml"), CAPABILITY)?;
        let documents = decode_sources(repository.path(), &[source("eqm/capability.toml")?])?;
        assert_eq!(documents[0].source().as_str(), "eqm/capability.toml");
        assert!(matches!(
            documents[0].document(),
            DocumentDto::Capability(_)
        ));
        Ok(())
    }

    #[test]
    fn old_future_foreign_and_cross_class_schemas_fail() -> Result<(), Box<dyn Error>> {
        for (schema, kind) in [
            (
                "https://schemas.equivalencematrix.dev/v0/capability",
                ValidationErrorKind::WrongSchema,
            ),
            (
                "https://schemas.equivalencematrix.dev/v2/capability",
                ValidationErrorKind::WrongSchema,
            ),
            (
                "https://example.com/v1/capability",
                ValidationErrorKind::WrongSchema,
            ),
            (
                "https://schemas.equivalencematrix.dev/v1/policy",
                ValidationErrorKind::WrongSourceClass,
            ),
        ] {
            let repository = tempfile::tempdir()?;
            fs::write(
                repository.path().join("document.toml"),
                CAPABILITY.replace(
                    "https://schemas.equivalencematrix.dev/v1/capability",
                    schema,
                ),
            )?;
            let error = decode_sources(repository.path(), &[source("document.toml")?])
                .err()
                .ok_or("schema unexpectedly accepted")?;
            assert_eq!(error.kind(), kind);
            assert_eq!(error.source().map(RepoPath::as_str), Some("document.toml"));
        }
        Ok(())
    }

    #[test]
    fn unknown_fields_extensions_and_duplicates_fail_closed() -> Result<(), Box<dyn Error>> {
        let repository = tempfile::tempdir()?;
        for (name, body) in [
            ("unknown.toml", format!("{CAPABILITY}legacy = true\n")),
            (
                "extension.toml",
                format!("{CAPABILITY}[extensions.Bad.Namespace]\nvalue = 1\n"),
            ),
        ] {
            fs::write(repository.path().join(name), body)?;
        }
        let unknown = decode_sources(repository.path(), &[source("unknown.toml")?])
            .err()
            .ok_or("unknown field accepted")?;
        assert_eq!(unknown.kind(), ValidationErrorKind::InvalidFields);
        let extension = decode_sources(repository.path(), &[source("extension.toml")?])
            .err()
            .ok_or("invalid extension accepted")?;
        assert_eq!(extension.kind(), ValidationErrorKind::InvalidExtensions);

        fs::write(repository.path().join("a.toml"), CAPABILITY)?;
        fs::write(repository.path().join("b.toml"), CAPABILITY)?;
        let duplicate = decode_sources(repository.path(), &[source("a.toml")?, source("b.toml")?])
            .err()
            .ok_or("duplicate authority accepted")?;
        assert_eq!(duplicate.kind(), ValidationErrorKind::DuplicateAuthority);
        Ok(())
    }

    #[test]
    fn oversized_document_fails_with_its_source() -> Result<(), Box<dyn Error>> {
        let repository = tempfile::tempdir()?;
        fs::write(
            repository.path().join("large.toml"),
            vec![b'a'; 4 * 1024 * 1024 + 1],
        )?;
        let error = decode_sources(repository.path(), &[source("large.toml")?])
            .err()
            .ok_or("oversized document accepted")?;
        assert_eq!(error.kind(), ValidationErrorKind::InvalidToml);
        assert_eq!(error.source().map(RepoPath::as_str), Some("large.toml"));
        Ok(())
    }
}
