//! Pure validated domain types for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod diagnostic;
mod schema;

pub use diagnostic::{
    Diagnostic, DiagnosticBuildError, DiagnosticCode, DiagnosticDescriptor,
    DiagnosticRegistryError, Severity, SourceLocation, SourceName, SourcePosition,
    validate_diagnostic_registry,
};
pub use schema::{SchemaKind, SchemaParseError, SchemaUri, SchemaVersion, ToolVersion};
