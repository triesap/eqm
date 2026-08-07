//! Non-executing structural, policy, and obligation checks.

use super::CommandExecution;
use crate::cli::ParsedCli;
use crate::renderer::OutputPayload;
use crate::session::{SessionRequest, prepare};
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_domain::{ArtifactRole, Diagnostic, DiagnosticCode, RepoPath, Severity, UtcInstant};
use eqm_engine::{
    ApplicabilityContext, AuthorityOrigin, EvaluationMode, PolicyProfileRequest, PolicyRef,
    ProfileRequest, RepositoryEntry, RepositoryEntryKind, RepositoryView, ScopeSubject,
    derive_obligations, diagnostic_registry, evaluate_structure, select_policy_profiles,
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
        let target = graph
            .targets()
            .get(binding.target())
            .ok_or("binding target")?;
        let view = repository_view(session.repository_root(), binding)?;
        for finding in evaluate_structure(binding, target, &view, false).findings {
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

    let policy = graph.policies().values().next().ok_or("policy required")?;
    let profile_requests = policy
        .profiles()
        .iter()
        .map(|id| {
            let ((_, revision), _) = graph
                .profiles()
                .iter()
                .find(|((candidate, _), _)| candidate == id)
                .ok_or("profile required")?;
            Ok(ProfileRequest::new(id.clone(), *revision, Vec::new())?)
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    let selection_request = PolicyProfileRequest::new(
        AuthorityOrigin::CandidateLocal,
        PolicyRef::new(policy.id().clone(), policy.revision()),
        profile_requests,
    )?;
    let selection = select_policy_profiles(
        graph,
        EvaluationMode::Development,
        None,
        Some(&selection_request),
    )?;
    let selected = selection
        .profiles()
        .values()
        .next()
        .ok_or("selected profile required")?;
    let profile = graph
        .profiles()
        .get(&(selected.id().clone(), selected.revision()))
        .ok_or("profile required")?;
    let applicability = ApplicabilityContext::new(profile, selected.values().clone())?;
    let obligations = derive_obligations(session.finalized(), &selection, &applicability, None)?;
    for obligation in obligations.obligations.values().filter(|obligation| {
        (units.is_empty() || units.contains(obligation.key.unit.as_str()))
            && (targets.is_empty() || subject_matches(&obligation.key.subject, &targets))
    }) {
        let id = obligation_id(obligation);
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

fn obligation_id(obligation: &eqm_engine::Obligation) -> String {
    let subject = match &obligation.key.subject {
        ScopeSubject::Target(value) => format!("target:{value}"),
        ScopeSubject::Provider(value) => format!("provider:{value}"),
        ScopeSubject::TargetSet(values) => format!(
            "targets:{}",
            values
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",")
        ),
    };
    format!(
        "{}@{}:{}:{}:{}:{}",
        obligation.key.policy,
        obligation.key.policy_revision.get(),
        obligation.key.unit,
        obligation.key.requirement,
        subject,
        obligation.key.facet
    )
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
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let before = fs::read_dir(root.join(".eqm"))
            .ok()
            .map(|entries| entries.count());
        let ParseOutcome::Run(parsed) = parse(["check", "--format", "json", "--no-progress"])?
        else {
            return Err("unexpected help".into());
        };
        let first = execute(parsed.clone(), &root)?;
        let second = execute(parsed, &root)?;
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
}
