//! Authored manifest loading and canonicalization for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod config;
mod discovery;
/// Strict source data-transfer objects for authored manifests.
pub mod dto;
mod parse;
mod validation;

pub use config::{ConfigError, WorkspaceConfig, select_workspace_config};
pub use discovery::{DiscoveredSource, DiscoveryError, SourceClass, discover_sources};
pub use parse::{ParseError, ParsedToml, TomlSyntaxError, parse_toml};
pub use validation::{
    DocumentDto, ValidatedDocument, ValidationError, ValidationErrorKind, decode_sources,
};
