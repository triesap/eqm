//! Complete stable diagnostic explanation registry.

use eqm_domain::{DiagnosticBuildError, DiagnosticCode, DiagnosticDescriptor, Severity};

/// Returns every emitted v1 diagnostic descriptor exactly once in code order.
pub fn diagnostic_registry() -> Result<Vec<DiagnosticDescriptor>, DiagnosticBuildError> {
    let code =
        |number| DiagnosticCode::from_number(number).ok_or(DiagnosticBuildError::InvalidCode);
    Ok(vec![
        DiagnosticDescriptor {
            code: code(1)?,
            severity: Severity::Error,
            title: "query operand did not resolve uniquely",
            authority: "docs/specification/cli.md",
            explanation: "An exact query found no authority or more than one authority for a required operand.",
            remediation: "Correct the query operand or narrow it to exactly one authority.",
        },
        DiagnosticDescriptor {
            code: code(100)?,
            severity: Severity::Error,
            title: "workspace preparation failed",
            authority: "docs/specification/cli.md",
            explanation: "The workspace could not complete manifest loading, graph resolution, invariant validation, expansion, or canonicalization.",
            remediation: "Correct the reported workspace authority and run validation again.",
        },
        DiagnosticDescriptor {
            code: code(200)?,
            severity: Severity::Error,
            title: "declared artifact structure failed",
            authority: "docs/specification/evaluation.md",
            explanation: "A declared artifact is missing, has the wrong path type or role, escapes its target root, uses an invalid symlink, or collides portably.",
            remediation: "Correct the artifact path and role within its declared target root.",
        },
        DiagnosticDescriptor {
            code: code(300)?,
            severity: Severity::Error,
            title: "duplicate graph authority",
            authority: "docs/specification/canonicalization.md",
            explanation: "Two inputs claim one semantic graph identity.",
            remediation: "Retain exactly one authority for the reported identity.",
        },
        DiagnosticDescriptor {
            code: code(301)?,
            severity: Severity::Error,
            title: "dangling graph reference",
            authority: "docs/specification/canonicalization.md",
            explanation: "An authored typed reference has no matching authority.",
            remediation: "Add the exact authority or correct the typed reference.",
        },
        DiagnosticDescriptor {
            code: code(302)?,
            severity: Severity::Error,
            title: "invalid graph relationship",
            authority: "docs/specification/manifest-contracts.md",
            explanation: "A resolved relationship violates hierarchy, membership, or lifecycle rules.",
            remediation: "Align parent references, membership, identifiers, and lifecycle state.",
        },
        DiagnosticDescriptor {
            code: code(303)?,
            severity: Severity::Error,
            title: "invalid risk inheritance",
            authority: "docs/specification/vocabularies.md",
            explanation: "A requirement lowers its journey or fragment risk authority.",
            remediation: "Retain inherited risk or raise the requirement risk class.",
        },
        DiagnosticDescriptor {
            code: code(304)?,
            severity: Severity::Error,
            title: "invalid fragment pin",
            authority: "docs/specification/manifest-contracts.md",
            explanation: "A fragment use does not match available semantic content exactly.",
            remediation: "Pin the exact fragment ID, revision, and canonical semantic digest.",
        },
        DiagnosticDescriptor {
            code: code(305)?,
            severity: Severity::Error,
            title: "fragment expansion collision",
            authority: "docs/specification/manifest-contracts.md",
            explanation: "Expansion would replace or duplicate a surface requirement identity.",
            remediation: "Choose a unique prefix or remove the conflicting requirement.",
        },
        DiagnosticDescriptor {
            code: code(500)?,
            severity: Severity::Error,
            title: "required evidence is missing",
            authority: "docs/specification/evaluation.md",
            explanation: "A derived obligation has no prepared evidence satisfying its exact coordinate.",
            remediation: "Provide current trusted evidence for the reported obligation.",
        },
        DiagnosticDescriptor {
            code: code(700)?,
            severity: Severity::Error,
            title: "adapter discovery failed",
            authority: "docs/specification/cli.md",
            explanation: "The exact committed adapter pin was unavailable, failed invocation, or returned an invalid or incomplete inventory.",
            remediation: "Install the exact pinned adapter locally and correct its protocol response.",
        },
    ])
}

/// Returns the complete explanation for one registered diagnostic code.
pub fn explain_diagnostic(
    code: DiagnosticCode,
) -> Result<Option<DiagnosticDescriptor>, DiagnosticBuildError> {
    let registry = diagnostic_registry()?;
    Ok(registry
        .binary_search_by_key(&code, |descriptor| descriptor.code)
        .ok()
        .map(|index| registry[index]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use eqm_domain::validate_diagnostic_registry;
    use std::error::Error;

    #[test]
    fn registry_is_complete_unique_live_and_explainable() -> Result<(), Box<dyn Error>> {
        let registry = diagnostic_registry()?;
        validate_diagnostic_registry(&registry)?;
        let emitted = [1, 100, 200, 300, 301, 302, 303, 304, 305, 500, 700]
            .into_iter()
            .map(|number| DiagnosticCode::from_number(number).ok_or("invalid emitted code"))
            .collect::<Result<Vec<_>, _>>()?;
        let registered = registry
            .iter()
            .map(|descriptor| descriptor.code)
            .collect::<Vec<_>>();
        assert_eq!(registered, emitted);
        for code in emitted {
            let descriptor = explain_diagnostic(code)?.ok_or("missing explanation")?;
            assert!(!descriptor.title.is_empty());
            assert!(!descriptor.authority.is_empty());
            assert!(!descriptor.explanation.is_empty());
            assert!(!descriptor.remediation.is_empty());
        }
        let unused = DiagnosticCode::from_number(400).ok_or("invalid unused code")?;
        assert!(explain_diagnostic(unused)?.is_none());
        Ok(())
    }
}
