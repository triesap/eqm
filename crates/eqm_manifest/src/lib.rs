//! Authored manifest loading and canonicalization for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod config;
/// Strict source data-transfer objects for authored manifests.
pub mod dto;
mod parse;

pub use config::{ConfigError, WorkspaceConfig, select_workspace_config};
pub use parse::{ParseError, ParsedToml, TomlSyntaxError, parse_toml};
