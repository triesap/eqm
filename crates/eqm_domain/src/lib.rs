//! Pure validated domain types for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod diagnostic;

pub use diagnostic::{
    Diagnostic, DiagnosticBuildError, DiagnosticCode, DiagnosticDescriptor,
    DiagnosticRegistryError, Severity, SourceLocation, SourceName, SourcePosition,
    validate_diagnostic_registry,
};
