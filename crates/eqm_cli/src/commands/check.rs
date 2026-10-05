//! Non-executing structural, policy, and obligation checks.

use super::CommandExecution;
use super::evaluation;
use crate::cli::ParsedCli;
use crate::renderer::OutputPayload;
use crate::session::{SessionRequest, prepare};
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_domain::{ArtifactRole, Diagnostic, DiagnosticCode, RepoPath, Severity, UtcInstant};
use eqm_engine::{
    RepositoryEntry, RepositoryEntryKind, RepositoryView, ScopeSubject, diagnostic_registry,
    evaluate_structure_with_roots,
};
use eqm_protocol::{
    CheckResultDto, CommandIdentity, DiagnosticDto, EvaluationModeDto, FacetStatusDto, FindingDto,
    InvocationContextDto, ReportEnvelope, ResultStatusDto, SarifLogDto,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

/// Runs `check` without invoking any runner or adapter.
pub fn execute(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    let offline = parsed.global.offline;
    let profiles = parsed.global.profiles.clone();
    let policy = parsed.global.policy.clone();
    let units = option_values(&parsed, "--unit");
    let targets = option_values(&parsed, "--target");
    let request = SessionRequest::new(parsed.global, parsed.command.name);
    let session = prepare(&request, start)?;
    let graph = session.finalized().graph();
    let mut findings = BTreeSet::new();
    let mut diagnostics = Vec::new();
    let mut counts = BTreeMap::new();

    for binding in graph.bindings().values().filter(|binding| {
        (units.is_empty() || units.contains(binding.unit().as_str()))
            && (targets.is_empty() || targets.contains(binding.target().as_str()))
    }) {
        // Structural checking uses the checked graph's exact target/component
        // ownership closure, never an output-root-only view and never an
        // arbitrary caller-supplied root list.
        let roots = graph
            .owned_roots(binding.target())
            .ok_or("binding target")?;
        let view = repository_view(session.repository_root(), binding)?;
        for finding in evaluate_structure_with_roots(binding, &roots, &view, false).findings {
            let coordinate = format!("structure:{}:{}", binding.id(), finding.artifact);
            findings.insert(FindingDto {
                diagnostic_code: "EQM-E0200".to_owned(),
                obligation: Some(coordinate.clone()),
                status: FacetStatusDto::Failed,
                evidence: None,
                waiver: None,
            });
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::from_number(200).ok_or("diagnostic code")?,
                Severity::Error,
                format!("{coordinate} failed with {:?}", finding.kind),
                None,
                Vec::new(),
                Some("Correct the declared artifact structure.".into()),
            )?);
            *counts.entry(FacetStatusDto::Failed).or_insert(0) += 1;
        }
    }

    let (_, obligations) = evaluation::derive_with_policy(&session, policy.as_deref(), &profiles)?;
    for obligation in obligations.obligations.values().filter(|obligation| {
        (units.is_empty() || units.contains(obligation.key.unit.as_str()))
            && (targets.is_empty() || subject_matches(&obligation.key.subject, &targets))
    }) {
        let id = evaluation::obligation_id(obligation);
        findings.insert(FindingDto {
            diagnostic_code: "EQM-E0500".to_owned(),
            obligation: Some(id.clone()),
            status: FacetStatusDto::Missing,
            evidence: None,
            waiver: None,
        });
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::from_number(500).ok_or("diagnostic code")?,
            Severity::Error,
            format!("required evidence is missing for `{id}`"),
            None,
            Vec::new(),
            Some("Provide current trusted evidence for this exact obligation.".into()),
        )?);
        *counts.entry(FacetStatusDto::Missing).or_insert(0) += 1;
    }
    let status = if findings.is_empty() {
        ResultStatusDto::Ok
    } else {
        ResultStatusDto::Error
    };
    let result = CheckResultDto {
        kind: CommandIdentity::Check,
        status,
        obligation_counts: counts,
        findings,
    };
    let domain_diagnostics = diagnostics.clone();
    let envelope = ReportEnvelope::new(
        CommandIdentity::Check,
        Some(session.workspace_digest()),
        context(offline)?,
        Some(result),
        diagnostics.iter().map(DiagnosticDto::from_domain).collect(),
    )?;
    let json = serde_json::from_slice(&envelope.to_json()?)?;
    Ok(CommandExecution {
        payload: OutputPayload {
            human: if domain_diagnostics.is_empty() {
                "check passed".to_owned()
            } else {
                format!("check found {} blocking findings", domain_diagnostics.len())
            },
            json,
            sarif: Some(serde_json::to_value(SarifLogDto::from_diagnostics(
                &domain_diagnostics,
                &diagnostic_registry()?,
            ))?),
            markdown: None,
        },
        exit_code: if domain_diagnostics.is_empty() { 0 } else { 1 },
    })
}

fn repository_view(
    root: &Path,
    binding: &eqm_domain::Binding,
) -> Result<RepositoryView, Box<dyn Error>> {
    let mut view = RepositoryView::new();
    for artifact in binding.artifacts().values().values() {
        let absolute = root.join(artifact.path().as_str());
        let Ok(metadata) = fs::symlink_metadata(&absolute) else {
            continue;
        };
        let (kind, resolved) = if metadata.file_type().is_symlink() {
            let resolved = fs::canonicalize(&absolute)
                .ok()
                .and_then(|path| {
                    path.strip_prefix(root)
                        .ok()
                        .map(|value| value.to_path_buf())
                })
                .and_then(|path| RepoPath::new(path.to_string_lossy().replace('\\', "/")).ok());
            (RepositoryEntryKind::Symlink, resolved)
        } else if metadata.is_dir() {
            (RepositoryEntryKind::Directory, None)
        } else {
            (RepositoryEntryKind::File, None)
        };
        view.insert(
            artifact.path().clone(),
            RepositoryEntry {
                kind,
                resolved,
                roles: BTreeSet::<ArtifactRole>::from([artifact.role()]),
            },
        );
    }
    Ok(view)
}

fn option_values(parsed: &ParsedCli, name: &str) -> BTreeSet<String> {
    parsed
        .command
        .options
        .get(name)
        .into_iter()
        .flatten()
        .filter_map(Clone::clone)
        .collect()
}

fn subject_matches(subject: &ScopeSubject, targets: &BTreeSet<String>) -> bool {
    match subject {
        ScopeSubject::Target(target) => targets.contains(target.as_str()),
        ScopeSubject::TargetSet(values) => values
            .iter()
            .any(|target| targets.contains(target.as_str())),
        ScopeSubject::Provider(_) => true,
    }
}

fn context(offline: bool) -> Result<InvocationContextDto<(), ()>, Box<dyn Error>> {
    let value: DateTime<Utc> = SystemTime::now().into();
    let instant: UtcInstant = value.to_rfc3339_opts(SecondsFormat::Secs, true).parse()?;
    Ok(InvocationContextDto::new(
        EvaluationModeDto::Development,
        Vec::new(),
        None,
        None,
        offline,
        instant,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{ParseOutcome, parse};

    #[test]
    fn check_is_nonexecuting_and_reports_stable_missing_obligations() -> Result<(), Box<dyn Error>>
    {
        let repository = crate::test_support::example_repository()?;
        let root = repository.path();
        let before = fs::read_dir(root.join(".eqm"))
            .ok()
            .map(|entries| entries.count());
        let ParseOutcome::Run(parsed) = parse(["check", "--format", "json", "--no-progress"])?
        else {
            return Err("unexpected help".into());
        };
        let first = execute(parsed.clone(), root)?;
        let second = execute(parsed, root)?;
        assert_eq!(first.exit_code, 1);
        assert_eq!(first.payload.json["result"], second.payload.json["result"]);
        assert!(
            first.payload.json["result"]["obligation_counts"]["missing"]
                .as_u64()
                .is_some_and(|value| value > 0)
        );
        assert_eq!(
            before,
            fs::read_dir(root.join(".eqm"))
                .ok()
                .map(|entries| entries.count())
        );
        Ok(())
    }

    const V2_SHARED_WORKSPACE: &str = r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/workspace.schema.json"

contract_sources = ["eqm/contracts/**/*.toml"]
binding_sources = ["eqm/bindings/**/*.toml"]
policy_sources = ["eqm/policies/**/*.toml"]
profile_sources = ["eqm/profiles/**/*.toml"]
runner_sources = ["eqm/runners/**/*.toml"]
waiver_sources = ["eqm/waivers/**/*.toml"]

[targets.android]
root = "apps/android"
platform = "android"
framework = "compose"
source_components = ["shared.ui"]
owners = ["owner://team/android"]

[targets.ios]
root = "apps/ios"
platform = "ios"
framework = "swiftui"
owners = ["owner://team/ios"]

[source_components."shared.ui"]
roots = ["shared/ui"]
owners = ["owner://team/shared"]
dependencies = []
"#;

    fn shared_binding(path: &str) -> String {
        format!(
            r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/binding.schema.json"
id = "binding.android.shared"
revision = 1
owners = ["owner://team/android"]
target = "android"
unit = "account.create.signup.identifier"

[[artifacts]]
id = "shared_view"
role = "view"
path = "{path}"
surface = "account.create.signup.identifier"

[[exposures]]
surface = "account.create.signup.identifier"
state = "required"
route = "signup"

[[evidence]]
id = "shared_behavior"
kind = "test"
requirements = ["account.create.signup.identifier#email_default"]
facets = ["behavior"]
runner = "runner.android"

[evidence.selector]
kind = "test"
framework = "junit"
test_id = "sharedView"
"#
        )
    }

    fn shared_workspace(binding_path: &str) -> Result<tempfile::TempDir, Box<dyn Error>> {
        let repository = crate::test_support::example_repository()?;
        let root = repository.path();
        fs::write(root.join("eqm.toml"), V2_SHARED_WORKSPACE)?;
        fs::create_dir_all(root.join("shared/ui"))?;
        fs::write(root.join("shared/ui/Shared.kt"), b"// shared\n")?;
        // Replace the shipped android binding with one whose artifact lives in
        // the declared shared component root, keeping one binding per
        // (target, unit) coordinate.
        let _ = fs::remove_file(root.join("eqm/bindings/android.auth_signup.toml"));
        fs::write(
            root.join("eqm/bindings/android.shared.toml"),
            shared_binding(binding_path),
        )?;
        Ok(repository)
    }

    fn structural_findings(
        execution: &CommandExecution,
    ) -> Result<Vec<serde_json::Value>, Box<dyn Error>> {
        let findings = execution.payload.json["result"]["findings"]
            .as_array()
            .ok_or("findings")?
            .iter()
            .filter(|finding| finding["diagnostic_code"] == "EQM-E0200")
            .cloned()
            .collect();
        Ok(findings)
    }

    #[test]
    fn structural_check_accepts_declared_shared_artifact() -> Result<(), Box<dyn Error>> {
        let repository = shared_workspace("shared/ui/Shared.kt")?;
        let ParseOutcome::Run(parsed) = parse(["check", "--format", "json", "--no-progress"])?
        else {
            return Err("unexpected help".into());
        };
        let execution = execute(parsed, repository.path())?;
        let findings = structural_findings(&execution)?;
        // A declared shared artifact is owned by the target's component closure,
        // so it must not report OutsideTargetRoot/Missing.
        assert!(
            !findings
                .iter()
                .any(|finding| finding["obligation"]
                    == "structure:binding.android.shared:shared_view"),
            "declared shared artifact must pass structural ownership: {findings:?}"
        );
        Ok(())
    }

    #[test]
    fn structural_check_reports_missing_declared_shared_artifact() -> Result<(), Box<dyn Error>> {
        let repository = shared_workspace("shared/ui/Missing.kt")?;
        let ParseOutcome::Run(parsed) = parse(["check", "--format", "json", "--no-progress"])?
        else {
            return Err("unexpected help".into());
        };
        let execution = execute(parsed, repository.path())?;
        let findings = structural_findings(&execution)?;
        assert!(
            findings.iter().any(|finding| finding["obligation"]
                == "structure:binding.android.shared:shared_view"
                && finding["status"] == "failed"),
            "missing declared shared artifact must fail structurally: {findings:?}"
        );
        Ok(())
    }

    #[test]
    fn structural_check_rejects_undeclared_and_escaped_paths() -> Result<(), Box<dyn Error>> {
        // A path outside the checked closure is rejected at the real loading
        // boundary, not silently accepted by a structural helper.
        for path in ["unrelated/Path.kt", "../escape.kt"] {
            let repository = shared_workspace(path)?;
            let ParseOutcome::Run(parsed) = parse(["check", "--format", "json", "--no-progress"])?
            else {
                return Err("unexpected help".into());
            };
            assert!(
                execute(parsed, repository.path()).is_err(),
                "undeclared/escaped artifact path `{path}` must be rejected"
            );
        }
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn structural_check_reports_invalid_artifact_symlink() -> Result<(), Box<dyn Error>> {
        let repository = shared_workspace("shared/ui/link.kt")?;
        std::os::unix::fs::symlink(
            repository.path().join("shared/ui/Shared.kt"),
            repository.path().join("shared/ui/link.kt"),
        )?;
        let ParseOutcome::Run(parsed) = parse(["check", "--format", "json", "--no-progress"])?
        else {
            return Err("unexpected help".into());
        };
        let execution = execute(parsed, repository.path())?;
        let findings = structural_findings(&execution)?;
        assert!(
            findings
                .iter()
                .any(|finding| finding["obligation"]
                    == "structure:binding.android.shared:shared_view"),
            "a forbidden artifact symlink must fail structurally: {findings:?}"
        );
        Ok(())
    }
}
