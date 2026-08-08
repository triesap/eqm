//! Exact release-subject gate over immutable records and prepared evidence.

use super::{CommandExecution, attest, evaluation};
use crate::cli::ParsedCli;
use crate::renderer::OutputPayload;
use crate::session::{PreparedSession, SessionRequest, prepare};
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_domain::{
    AppVersion, BuildNumber, DurationMillis, ProducerRef, ReleaseChannel, ReleaseRecord, RepoPath,
    Sha256Digest, SourceCommit, TargetId, TrustLevel, UtcInstant,
};
use eqm_engine::{
    FacetStatus, ReleaseCheck, ReleaseContext, ReleaseGateInput, ReleaseGateStatus, ReleaseSubject,
    ScopeSubject, TargetConformance, evaluate_release_gate, evaluate_target_conformance,
};
use eqm_protocol::{
    CommandIdentity, EvaluationModeDto, EvidencePayloadDto, EvidenceResultDto,
    InvocationContextDto, ReleaseCheckResultDto, ReleaseRecordDto, ReportEnvelope, RuntimeFactsDto,
    ScopeSubjectDto,
};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::SystemTime;

/// Evaluates one exact release record against current policy and prepared evidence.
pub fn execute(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    execute_with_authority(parsed, start, evaluated_at()?, &UnconfiguredTrust)
}

fn execute_with_authority(
    parsed: ParsedCli,
    start: &Path,
    now: UtcInstant,
    authority: &impl ReleaseTrustAuthority,
) -> Result<CommandExecution, Box<dyn Error>> {
    if parsed.global.profiles.is_empty() {
        return Err("release check requires an explicit release profile".into());
    }
    let offline = parsed.global.offline;
    let profiles = parsed.global.profiles.clone();
    let record_path = option(&parsed, "--release-record")
        .ok_or("release record required")?
        .to_owned();
    let request = SessionRequest::new(parsed.global, parsed.command.name);
    let session = prepare(&request, start)?;
    let (selection, obligations) = evaluation::derive(&session, &profiles)?;
    let (record_dto, record) = read_release_record(&session, &record_path)?;
    if !session
        .finalized()
        .graph()
        .targets()
        .contains_key(record.target())
    {
        return Err("release target is outside the prepared workspace".into());
    }
    let evidence = read_evidence_set(&session)?;
    let runtime_facts = read_runtime_facts(&session)?;
    let matching = evidence
        .iter()
        .filter(|value| {
            value.target == record.target().as_str()
                && value.subject.source_commit == record.source_commit().as_str()
                && value.release_record_digest.as_deref() == Some(&record_dto.record_digest)
        })
        .collect::<Vec<_>>();
    let profile_values = super::attest::selected_profile_values(&selection)?;
    let profile_digest = Sha256Digest::hash_content(&serde_json_canonicalizer::to_vec(
        &serde_json::to_value(&profile_values)?,
    )?);
    let evidence_set_digest = evidence_set_digest(&matching);
    let referenced_runtime_digest = common_runtime_digest(&matching);
    let runtime = exact_runtime_snapshot(
        &session,
        &record_dto,
        &profile_values,
        referenced_runtime_digest,
        &runtime_facts,
        now,
    )?;
    let runtime_digest = runtime
        .map(|value| value.facts_digest.parse::<Sha256Digest>())
        .transpose()?;
    let expected_context = ReleaseContext {
        contract_digest: session.workspace_digest(),
        policy_digest: session.workspace_digest(),
        profile_values_digest: profile_digest,
        evidence_set_digest,
        runtime_facts_digest: runtime_digest
            .unwrap_or_else(|| Sha256Digest::hash_content(b"missing")),
        trust_config_digest: attest::trust_config_digest(&session),
        release_record_digest: record.record_digest(),
        evaluated_at: now,
    };
    let facets = obligation_statuses(
        &obligations,
        record.target(),
        &matching,
        now,
        &EvidenceEvaluationContext {
            authority,
            session: &session,
            record: &record_dto,
            runtime_digest,
            profile_values: &profile_values,
        },
    );
    let conformance = evaluate_target_conformance(record.target(), &facets, true)?;
    let release_facets = facets.values().copied().collect::<Vec<_>>();
    let effective_trust = matching
        .iter()
        .filter_map(|value| effective_evidence_trust(value, authority))
        .chain(effective_record_trust(&record_dto, authority))
        .chain(runtime.and_then(|value| effective_runtime_trust(value, authority)))
        .min();
    let current_repository = repository_identity(session.repository_root())?;
    let current_commit = git_output(session.repository_root(), &["rev-parse", "HEAD"])?;
    let context_complete = !matching.is_empty()
        && runtime_digest.is_some()
        && record_dto.source_commit == current_commit
        && matching.iter().all(|value| {
            value.contract_digest == session.workspace_digest().to_string()
                && value.policy_digest == session.workspace_digest().to_string()
                && value.profile_values == profile_values
                && evidence_subject_matches(
                    value,
                    &record_dto,
                    &current_repository,
                    session.workspace_digest(),
                    runtime_digest,
                )
        });
    let waivers = BTreeSet::new();
    let input = ReleaseGateInput {
        expected_subject: ReleaseSubject::from(&record),
        observed_subject: Some(ReleaseSubject::from(&record)),
        release_record_verified: true,
        expected_context,
        observed_context: context_complete.then_some(expected_context),
        exposure: vec![if runtime_digest.is_some() {
            ReleaseCheck::Match
        } else {
            ReleaseCheck::Unknown
        }],
        conformance: Some(conformance),
        release_facets,
        effective_trust,
        waivers: waivers.clone(),
    };
    let status = evaluate_release_gate(&input);
    let result = ReleaseCheckResultDto {
        kind: CommandIdentity::ReleaseCheck,
        subject: record_dto,
        status: gate(status).to_owned(),
        conformance: target_conformance(conformance).to_owned(),
        equivalence: "unknown".to_owned(),
        exposure: BTreeSet::from([if runtime_digest.is_some() {
            "runtime_facts:match".to_owned()
        } else {
            "runtime_facts:unknown".to_owned()
        }]),
        waivers: waivers.iter().map(ToString::to_string).collect(),
    };
    let envelope = ReportEnvelope::new(
        CommandIdentity::ReleaseCheck,
        Some(session.workspace_digest()),
        context(offline, now)?,
        Some(result),
        Vec::new(),
    )?;
    Ok(CommandExecution {
        payload: OutputPayload {
            human: format!("release gate: {}", gate(status)),
            json: serde_json::from_slice(&envelope.to_json()?)?,
            sarif: None,
            markdown: None,
        },
        exit_code: match status {
            ReleaseGateStatus::Pass => 0,
            ReleaseGateStatus::Conditional | ReleaseGateStatus::Fail => 1,
            ReleaseGateStatus::Unknown => 7,
        },
    })
}

trait ReleaseTrustAuthority {
    fn evidence_trust(&self, evidence: &EvidenceResultDto) -> Option<TrustLevel>;
    fn record_trust(&self, record: &ReleaseRecordDto) -> Option<TrustLevel>;
    fn runtime_trust(&self, runtime: &RuntimeFactsDto) -> Option<TrustLevel>;
}

struct UnconfiguredTrust;

impl ReleaseTrustAuthority for UnconfiguredTrust {
    fn evidence_trust(&self, _evidence: &EvidenceResultDto) -> Option<TrustLevel> {
        Some(TrustLevel::UntrustedLocal)
    }

    fn record_trust(&self, _record: &ReleaseRecordDto) -> Option<TrustLevel> {
        Some(TrustLevel::UntrustedLocal)
    }

    fn runtime_trust(&self, _runtime: &RuntimeFactsDto) -> Option<TrustLevel> {
        Some(TrustLevel::UntrustedLocal)
    }
}

fn effective_evidence_trust(
    evidence: &EvidenceResultDto,
    authority: &impl ReleaseTrustAuthority,
) -> Option<TrustLevel> {
    let claimed = evidence.claimed_trust.parse::<TrustLevel>().ok()?;
    authority
        .evidence_trust(evidence)
        .map(|verified| verified.min(claimed))
}

fn effective_record_trust(
    record: &ReleaseRecordDto,
    authority: &impl ReleaseTrustAuthority,
) -> Option<TrustLevel> {
    let claimed = record.claimed_trust.parse::<TrustLevel>().ok()?;
    authority
        .record_trust(record)
        .map(|verified| verified.min(claimed))
}

fn effective_runtime_trust(
    runtime: &RuntimeFactsDto,
    authority: &impl ReleaseTrustAuthority,
) -> Option<TrustLevel> {
    let claimed = runtime.claimed_trust.parse::<TrustLevel>().ok()?;
    authority
        .runtime_trust(runtime)
        .map(|verified| verified.min(claimed))
}

fn read_release_record(
    session: &PreparedSession,
    path: &str,
) -> Result<(ReleaseRecordDto, ReleaseRecord), Box<dyn Error>> {
    let relative = RepoPath::new(path)?;
    let absolute = session.repository_root().join(relative.as_str());
    let bytes = confined_bytes(session, &absolute)?;
    let dto: ReleaseRecordDto = serde_json::from_slice(&bytes)?;
    if dto.schema != eqm_protocol::RELEASE_RECORD_SCHEMA.to_string() {
        return Err("release record schema mismatch".into());
    }
    let claimed: Sha256Digest = dto.record_digest.parse()?;
    let mut value: Value = serde_json::from_slice(&bytes)?;
    value
        .as_object_mut()
        .ok_or("release record object")?
        .remove("record_digest");
    if Sha256Digest::hash_content(&serde_json_canonicalizer::to_vec(&value)?) != claimed {
        return Err("release record digest mismatch".into());
    }
    let record = ReleaseRecord::new(
        TargetId::new(dto.target.as_str())?,
        AppVersion::new(dto.app_version.as_str())?,
        BuildNumber::new(dto.build_number.as_str())?,
        dto.source_commit.parse::<SourceCommit>()?,
        dto.artifact_digest.parse()?,
        dto.channel.parse::<ReleaseChannel>()?,
        dto.released_at.parse()?,
        dto.producer.parse::<ProducerRef>()?,
        dto.claimed_trust.parse::<TrustLevel>()?,
        claimed,
    );
    Ok((dto, record))
}

fn read_evidence_set(session: &PreparedSession) -> Result<Vec<EvidenceResultDto>, Box<dyn Error>> {
    let paths = attest::evidence_paths(session, &BTreeSet::new())?;
    paths
        .iter()
        .map(|path| attest::read_evidence(session, path))
        .collect()
}

fn read_runtime_facts(session: &PreparedSession) -> Result<Vec<RuntimeFactsDto>, Box<dyn Error>> {
    let root = session.repository_root().join(".eqm/runtime-facts");
    let metadata = match fs::symlink_metadata(&root) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error.into()),
    };
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err("runtime-facts root must be a confined directory".into());
    }
    let mut paths = fs::read_dir(&root)?
        .map(|entry| entry.map(|value| value.path()))
        .collect::<Result<Vec<_>, _>>()?;
    paths.sort();
    paths
        .iter()
        .map(|path| RuntimeFactsDto::from_json(&confined_bytes(session, path)?).map_err(Into::into))
        .collect()
}

struct EvidenceEvaluationContext<'a, A> {
    authority: &'a A,
    session: &'a PreparedSession,
    record: &'a ReleaseRecordDto,
    runtime_digest: Option<Sha256Digest>,
    profile_values: &'a [eqm_protocol::ProfileValueDto],
}

fn obligation_statuses<A: ReleaseTrustAuthority>(
    derived: &eqm_engine::ObligationDerivation,
    target: &TargetId,
    evidence: &[&EvidenceResultDto],
    evaluated_at: UtcInstant,
    context: &EvidenceEvaluationContext<'_, A>,
) -> BTreeMap<eqm_engine::ObligationKey, FacetStatus> {
    derived
        .obligations
        .iter()
        .filter(|(key, _)| matches!(&key.subject, ScopeSubject::Target(value) if value == target))
        .map(|(key, obligation)| {
            let candidates = evidence.iter().filter(|value| {
                value.unit == key.unit.as_str()
                    && value.requirements.contains(key.requirement.as_str())
                    && value.facets.contains(&key.facet.to_string())
                    && evidence_coordinate_matches(
                        context.session,
                        key,
                        value,
                        context.record,
                        context.runtime_digest,
                        context.profile_values,
                    )
            });
            let mut statuses = candidates
                .map(|value| {
                    evidence_status(
                        value,
                        effective_evidence_trust(value, context.authority),
                        obligation.strength.minimum_trust,
                        obligation.strength.maximum_age,
                        evaluated_at,
                    )
                })
                .collect::<Vec<_>>();
            if statuses.len() < obligation.strength.minimum_count.get() as usize {
                statuses.push(FacetStatus::Missing);
            }
            let status = aggregate_statuses(&statuses);
            (key.clone(), status)
        })
        .collect()
}

fn evidence_coordinate_matches(
    session: &PreparedSession,
    key: &eqm_engine::ObligationKey,
    evidence: &EvidenceResultDto,
    record: &ReleaseRecordDto,
    runtime_digest: Option<Sha256Digest>,
    profile_values: &[eqm_protocol::ProfileValueDto],
) -> bool {
    let eqm_engine::ScopeSubject::Target(target) = &key.subject else {
        return false;
    };
    let matching = session
        .finalized()
        .graph()
        .bindings()
        .values()
        .filter(|binding| binding.target() == target && binding.unit().as_str() == evidence.unit)
        .flat_map(|binding| {
            binding
                .evidence()
                .values()
                .filter(move |specification| {
                    specification.requirements().contains(&key.requirement)
                        && specification.facets().contains(&key.facet)
                        && specification.kind().to_string() == evidence.kind
                })
                .map(move |specification| {
                    Sha256Digest::hash_content(
                        format!("{}:{}", binding.id(), specification.id()).as_bytes(),
                    )
                })
        })
        .collect::<Vec<_>>();
    matching.len() == 1
        && evidence.evidence_spec_digest == matching[0].to_string()
        && evidence.binding_digest == matching[0].to_string()
        && evidence.contract_digest == session.workspace_digest().to_string()
        && evidence.policy_digest == session.workspace_digest().to_string()
        && evidence.profile_values == profile_values
        && evidence.release_record_digest.as_deref() == Some(record.record_digest.as_str())
        && evidence.runtime_facts_digest.as_deref()
            == runtime_digest.map(|value| value.to_string()).as_deref()
}

fn exact_runtime_snapshot<'a>(
    session: &PreparedSession,
    record: &ReleaseRecordDto,
    profile_values: &[eqm_protocol::ProfileValueDto],
    referenced_digest: Option<Sha256Digest>,
    snapshots: &'a [RuntimeFactsDto],
    evaluated_at: UtcInstant,
) -> Result<Option<&'a RuntimeFactsDto>, Box<dyn Error>> {
    let Some(referenced_digest) = referenced_digest else {
        return Ok(None);
    };
    let repository = repository_identity(session.repository_root())?;
    let repository_digest = Sha256Digest::hash_content(repository.as_bytes()).to_string();
    let matches = snapshots
        .iter()
        .filter(|snapshot| {
            let observed = snapshot.observed_at.parse::<UtcInstant>().ok();
            let expires = snapshot.expires_at.parse::<UtcInstant>().ok();
            snapshot.facts_digest == referenced_digest.to_string()
                && snapshot.target == record.target
                && snapshot.profile_values == profile_values
                && snapshot.subject.repository == repository
                && snapshot.subject.repository_id_digest == repository_digest
                && matches!(&snapshot.subject.scope, ScopeSubjectDto::Target { target } if target == &record.target)
                && snapshot.subject.source_commit == record.source_commit
                && snapshot.subject.build_id.as_deref() == Some(record.build_number.as_str())
                && snapshot.subject.artifact_digest.as_deref() == Some(record.artifact_digest.as_str())
                && snapshot.subject.target_configuration_digest == session.workspace_digest().to_string()
                && observed.is_some_and(|value| value <= evaluated_at)
                && expires.is_some_and(|value| evaluated_at < value)
        })
        .collect::<Vec<_>>();
    Ok((matches.len() == 1).then_some(matches[0]))
}

fn evidence_subject_matches(
    evidence: &EvidenceResultDto,
    record: &ReleaseRecordDto,
    repository: &str,
    workspace_digest: Sha256Digest,
    runtime_digest: Option<Sha256Digest>,
) -> bool {
    evidence.target == record.target
        && evidence.subject.repository == repository
        && evidence.subject.repository_id_digest
            == Sha256Digest::hash_content(repository.as_bytes()).to_string()
        && matches!(&evidence.subject.scope, ScopeSubjectDto::Target { target } if target == &record.target)
        && evidence.subject.source_commit == record.source_commit
        && evidence.subject.build_id.as_deref() == Some(record.build_number.as_str())
        && evidence.subject.artifact_digest.as_deref() == Some(record.artifact_digest.as_str())
        && evidence.subject.target_configuration_digest == workspace_digest.to_string()
        && evidence.runtime_facts_digest.as_deref()
            == runtime_digest.map(|value| value.to_string()).as_deref()
        && evidence.release_record_digest.as_deref() == Some(record.record_digest.as_str())
}

fn repository_identity(root: &Path) -> Result<String, Box<dyn Error>> {
    let remote = git_output(root, &["remote", "get-url", "origin"])?;
    let normalized = if let Some(path) = remote.strip_prefix("git@github.com:") {
        format!("https://github.com/{}", path.trim_end_matches(".git"))
    } else {
        remote.trim_end_matches(".git").to_owned()
    };
    let _: eqm_domain::RepositoryIdentity = normalized.parse()?;
    Ok(normalized)
}

fn git_output(root: &Path, arguments: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(root)
        .output()?;
    if !output.status.success() {
        return Err("Git identity acquisition failed".into());
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}

fn evidence_status(
    value: &EvidenceResultDto,
    effective_trust: Option<TrustLevel>,
    minimum: TrustLevel,
    maximum_age: DurationMillis,
    evaluated_at: UtcInstant,
) -> FacetStatus {
    let Some(trust) = effective_trust else {
        return FacetStatus::Unknown;
    };
    if trust < minimum {
        return FacetStatus::Unknown;
    }
    let Ok(observed_at) = value.observed_at.parse::<UtcInstant>() else {
        return FacetStatus::Unknown;
    };
    let observed_millis = i128::from(observed_at.unix_seconds()) * 1_000
        + i128::from(observed_at.subsec_nanos() / 1_000_000);
    let evaluated_millis = i128::from(evaluated_at.unix_seconds()) * 1_000
        + i128::from(evaluated_at.subsec_nanos() / 1_000_000);
    if observed_millis > evaluated_millis + 5 * 60 * 1_000 {
        return FacetStatus::Unknown;
    }
    if observed_millis + i128::from(maximum_age.get()) < evaluated_millis {
        return FacetStatus::Stale;
    }
    match &value.payload {
        EvidencePayloadDto::StructuralCheck { execution }
        | EvidencePayloadDto::Test { execution }
        | EvidencePayloadDto::Snapshot { execution }
            if execution.counts.failed > 0 =>
        {
            FacetStatus::Failed
        }
        EvidencePayloadDto::StructuralCheck { execution }
        | EvidencePayloadDto::Test { execution }
        | EvidencePayloadDto::Snapshot { execution }
            if execution.counts.passed == 0 =>
        {
            FacetStatus::Missing
        }
        _ => FacetStatus::Satisfied,
    }
}

fn aggregate_statuses(statuses: &[FacetStatus]) -> FacetStatus {
    let has = |status| statuses.contains(&status);
    if has(FacetStatus::Satisfied) && has(FacetStatus::Failed) {
        FacetStatus::Unstable
    } else if has(FacetStatus::Unknown) {
        FacetStatus::Unknown
    } else if has(FacetStatus::Failed) {
        FacetStatus::Failed
    } else if has(FacetStatus::Stale) {
        FacetStatus::Stale
    } else if has(FacetStatus::Missing) {
        FacetStatus::Missing
    } else if has(FacetStatus::Waived) {
        FacetStatus::Waived
    } else if has(FacetStatus::Satisfied) {
        FacetStatus::Satisfied
    } else {
        FacetStatus::NotApplicable
    }
}

fn evidence_set_digest(evidence: &[&EvidenceResultDto]) -> Sha256Digest {
    let joined = evidence
        .iter()
        .map(|value| value.result_digest.as_str())
        .collect::<Vec<_>>()
        .join("\0");
    Sha256Digest::hash_content(joined.as_bytes())
}

fn common_runtime_digest(evidence: &[&EvidenceResultDto]) -> Option<Sha256Digest> {
    let values = evidence
        .iter()
        .filter_map(|value| value.runtime_facts_digest.as_deref())
        .collect::<BTreeSet<_>>();
    (values.len() == 1)
        .then(|| values.first()?.parse().ok())
        .flatten()
}

fn confined_bytes(session: &PreparedSession, path: &Path) -> Result<Vec<u8>, Box<dyn Error>> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() || !metadata.is_file() || metadata.len() > 16 * 1024 * 1024
    {
        return Err("input must be a bounded regular file".into());
    }
    let canonical = path.canonicalize()?;
    if !canonical.starts_with(session.repository_root().canonicalize()?) {
        return Err("input escaped repository confinement".into());
    }
    Ok(fs::read(canonical)?)
}

const fn gate(value: ReleaseGateStatus) -> &'static str {
    match value {
        ReleaseGateStatus::Pass => "pass",
        ReleaseGateStatus::Conditional => "conditional",
        ReleaseGateStatus::Fail => "fail",
        ReleaseGateStatus::Unknown => "unknown",
    }
}

const fn target_conformance(value: TargetConformance) -> &'static str {
    match value {
        TargetConformance::Conformant => "conformant",
        TargetConformance::ConditionallyConformant => "conditionally_conformant",
        TargetConformance::Nonconformant => "nonconformant",
    }
}

fn option<'a>(parsed: &'a ParsedCli, name: &str) -> Option<&'a str> {
    parsed
        .command
        .options
        .get(name)
        .and_then(|values| values.first())
        .and_then(Option::as_deref)
}

fn evaluated_at() -> Result<UtcInstant, Box<dyn Error>> {
    let value: DateTime<Utc> = SystemTime::now().into();
    Ok(value.to_rfc3339_opts(SecondsFormat::Secs, true).parse()?)
}

fn context(offline: bool, at: UtcInstant) -> Result<InvocationContextDto<(), ()>, Box<dyn Error>> {
    Ok(InvocationContextDto::new(
        EvaluationModeDto::Release,
        Vec::new(),
        None,
        None,
        offline,
        at,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{ParseOutcome, parse};
    use serde_json::json;
    use std::fs;

    struct SyntheticSignedTrust;

    impl ReleaseTrustAuthority for SyntheticSignedTrust {
        fn evidence_trust(&self, _evidence: &EvidenceResultDto) -> Option<TrustLevel> {
            Some(TrustLevel::SignedCi)
        }

        fn record_trust(&self, _record: &ReleaseRecordDto) -> Option<TrustLevel> {
            Some(TrustLevel::SignedCi)
        }

        fn runtime_trust(&self, _runtime: &RuntimeFactsDto) -> Option<TrustLevel> {
            Some(TrustLevel::SignedCi)
        }
    }

    #[test]
    fn malformed_release_record_fails_before_gate_evaluation() -> Result<(), Box<dyn Error>> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let request =
            SessionRequest::new(Default::default(), crate::cli::CommandName::ReleaseCheck);
        let session = prepare(&request, &root)?;
        assert!(read_release_record(&session, "eqm.lock").is_err());
        Ok(())
    }

    #[test]
    fn facet_aggregation_preserves_terminal_and_ambiguous_outcomes() {
        assert_eq!(aggregate_statuses(&[]), FacetStatus::NotApplicable);
        assert_eq!(
            aggregate_statuses(&[FacetStatus::Satisfied, FacetStatus::Failed]),
            FacetStatus::Unstable
        );
        assert_eq!(
            aggregate_statuses(&[FacetStatus::Satisfied, FacetStatus::Unknown]),
            FacetStatus::Unknown
        );
        assert_eq!(
            aggregate_statuses(&[FacetStatus::Satisfied, FacetStatus::Missing]),
            FacetStatus::Missing
        );
    }

    #[test]
    fn claims_are_capped_by_independent_authority() -> Result<(), Box<dyn Error>> {
        let evidence: EvidenceResultDto = serde_json::from_value(json!({
            "schema":eqm_protocol::EVIDENCE_RESULT_SCHEMA.to_string(),
            "id":Sha256Digest::hash_content(b"result").to_string(),
            "subject":{"repository":"https://example.com/repo","repository_id_digest":Sha256Digest::hash_content(b"repo").to_string(),"scope":{"kind":"target","target":"web"},"source_commit":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","build_id":null,"artifact_digest":null,"target_configuration_digest":Sha256Digest::hash_content(b"config").to_string()},
            "target":"web","unit":"unit.one","requirements":["unit.one#works"],"facets":["behavior"],"kind":"release_record",
            "evidence_spec_digest":Sha256Digest::hash_content(b"spec").to_string(),"contract_digest":Sha256Digest::hash_content(b"contract").to_string(),"binding_digest":Sha256Digest::hash_content(b"binding").to_string(),"policy_digest":Sha256Digest::hash_content(b"policy").to_string(),
            "runner_digest":null,"adapter_digest":null,"runtime_facts_digest":null,"release_record_digest":null,"profile_values":[],"producer":"producer://fixture/test","claimed_trust":"signed_ci","observed_at":"2026-08-08T00:00:00Z","payload":{"kind":"release_record","release_record_digest":Sha256Digest::hash_content(b"release").to_string()},"attachments":[],"result_digest":Sha256Digest::hash_content(b"result").to_string()
        }))?;
        assert_eq!(
            effective_evidence_trust(&evidence, &UnconfiguredTrust),
            Some(TrustLevel::UntrustedLocal)
        );
        assert_eq!(
            effective_evidence_trust(&evidence, &SyntheticSignedTrust),
            Some(TrustLevel::SignedCi)
        );
        Ok(())
    }

    #[test]
    fn parsed_release_cli_exercises_pass_fail_and_unknown_with_exact_inputs()
    -> Result<(), Box<dyn Error>> {
        for (name, failed, claim, expected_status, expected_exit) in [
            ("pass", false, "signed_ci", "pass", 0),
            ("fail", true, "signed_ci", "fail", 1),
            ("unknown", false, "trusted_ci", "unknown", 7),
        ] {
            let repository = release_fixture(name, failed, claim)?;
            let parsed = release_invocation(name)?;
            let execution = execute_with_authority(
                parsed,
                repository.path(),
                "2026-08-08T12:00:00Z".parse()?,
                &SyntheticSignedTrust,
            )?;
            assert_eq!(
                execution.exit_code, expected_exit,
                "{name}: {}",
                execution.payload.json
            );
            assert_eq!(
                execution.payload.json["result"]["status"], expected_status,
                "{name}"
            );

            if name == "pass" {
                let unverified = execute_with_authority(
                    release_invocation(name)?,
                    repository.path(),
                    "2026-08-08T12:00:00Z".parse()?,
                    &UnconfiguredTrust,
                )?;
                assert_eq!(unverified.exit_code, 7);
                assert_eq!(unverified.payload.json["result"]["status"], "unknown");
            }
        }
        Ok(())
    }

    fn release_invocation(name: &str) -> Result<ParsedCli, Box<dyn Error>> {
        let path = format!("releases/{name}.generated.json");
        let ParseOutcome::Run(parsed) = parse([
            "release",
            "check",
            "--release-record",
            path.as_str(),
            "--profile",
            "audience.default",
            "--format",
            "json",
            "--no-progress",
        ])?
        else {
            return Err("unexpected help".into());
        };
        Ok(parsed)
    }

    fn release_fixture(
        name: &str,
        failed: bool,
        claim: &str,
    ) -> Result<tempfile::TempDir, Box<dyn Error>> {
        let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/signup");
        let repository = tempfile::tempdir()?;
        copy_fixture(&source, repository.path())?;
        git(repository.path(), &["init", "-q"])?;
        git(
            repository.path(),
            &[
                "remote",
                "add",
                "origin",
                "git@github.com:example/signup-fixture.git",
            ],
        )?;
        git(repository.path(), &["add", "."])?;
        git(
            repository.path(),
            &[
                "-c",
                "user.name=Fixture",
                "-c",
                "user.email=fixture@example.invalid",
                "commit",
                "-qm",
                "fixture",
            ],
        )?;

        let parsed = release_invocation(name)?;
        let request = SessionRequest::new(parsed.global, parsed.command.name);
        let session = prepare(&request, repository.path())?;
        let (selection, obligations) =
            evaluation::derive(&session, &["audience.default".to_owned()])?;
        let profile_values = attest::selected_profile_values(&selection)?;
        let profiles = serde_json::to_value(&profile_values)?;
        let commit = git_output(repository.path(), &["rev-parse", "HEAD"])?;
        let repository_identity = repository_identity(repository.path())?;
        let repository_digest =
            Sha256Digest::hash_content(repository_identity.as_bytes()).to_string();
        let workspace_digest = session.workspace_digest().to_string();
        let artifact_digest = Sha256Digest::hash_content(b"fixture-artifact").to_string();

        let mut record = json!({
            "schema":eqm_protocol::RELEASE_RECORD_SCHEMA.to_string(),
            "target":"web","app_version":"1.0.0","build_number":"42",
            "source_commit":commit,"artifact_digest":artifact_digest,"channel":"production",
            "released_at":"2026-08-08T00:00:00Z","producer":"producer://release/fixture/build-42",
            "claimed_trust":claim
        });
        let record_digest = seal_field(&mut record, "record_digest")?;
        fs::create_dir_all(repository.path().join("releases"))?;
        fs::write(
            repository
                .path()
                .join(format!("releases/{name}.generated.json")),
            serde_json::to_vec(&record)?,
        )?;

        let mut runtime = json!({
            "schema":eqm_protocol::RUNTIME_FACTS_SCHEMA.to_string(),
            "provider":"runtime.fixture",
            "subject":{
                "repository":repository_identity,"repository_id_digest":repository_digest,
                "scope":{"kind":"target","target":"web"},"source_commit":commit,
                "build_id":"42","artifact_digest":artifact_digest,
                "target_configuration_digest":workspace_digest
            },
            "target":"web","profile_values":profiles,
            "observed_at":"2026-08-08T00:00:00Z","expires_at":"2026-08-09T00:00:00Z",
            "facts":[{"surface":"account.create.signup.otp","dimension":"availability","key":"enabled","value":{"type":"boolean","value":true}}],
            "producer":"producer://runtime/fixture/snapshot-1","claimed_trust":claim
        });
        let runtime_digest = seal_field(&mut runtime, "facts_digest")?;
        let runtime_root = repository.path().join(".eqm/runtime-facts");
        fs::create_dir_all(&runtime_root)?;
        fs::write(
            runtime_root.join(format!(
                "{}.json",
                runtime_digest.trim_start_matches("sha256:")
            )),
            serde_json::to_vec(&runtime)?,
        )?;

        let (outcome, passed, failed_count) = if failed {
            ("failed", 0, 1)
        } else {
            ("passed", 1, 0)
        };
        let results = repository.path().join(".eqm/results");
        fs::create_dir_all(&results)?;
        let mut evidence_dtos = Vec::new();
        for (coordinate, unit, requirements, facets) in [
            (
                "binding.web.signup:identifier_behavior",
                "account.create.signup.identifier",
                vec!["account.create.signup.identifier#email_default"],
                vec!["behavior"],
            ),
            (
                "binding.web.signup_otp:otp_behavior",
                "account.create.signup.otp",
                vec!["account.create.signup.otp#six_decimal_digits"],
                vec!["accessibility", "behavior"],
            ),
        ] {
            let coordinate = Sha256Digest::hash_content(coordinate.as_bytes());
            let mut evidence = json!({
                "schema":eqm_protocol::EVIDENCE_RESULT_SCHEMA.to_string(),
                "subject":{
                    "repository":repository_identity,"repository_id_digest":repository_digest,
                    "scope":{"kind":"target","target":"web"},"source_commit":commit,
                    "build_id":"42","artifact_digest":artifact_digest,
                    "target_configuration_digest":workspace_digest
                },
                "target":"web","unit":unit,"requirements":requirements,"facets":facets,
                "kind":"test","evidence_spec_digest":coordinate.to_string(),
                "contract_digest":workspace_digest,"binding_digest":coordinate.to_string(),
                "policy_digest":workspace_digest,
                "runner_digest":Sha256Digest::hash_content(b"fixture-runner").to_string(),
                "adapter_digest":null,"runtime_facts_digest":runtime_digest,
                "release_record_digest":record_digest,"profile_values":profiles,
                "producer":"producer://ci/fixture/run-1","claimed_trust":claim,
                "observed_at":"2026-08-08T00:00:01Z",
                "payload":{"kind":"test","execution":{"attempts":[{"number":1,"outcome":outcome,"started_at":"2026-08-08T00:00:00Z","finished_at":"2026-08-08T00:00:01Z","message":null}],"counts":{"selected":1,"passed":passed,"failed":failed_count,"skipped":0,"filtered":0,"quarantined":0},"started_at":"2026-08-08T00:00:00Z","finished_at":"2026-08-08T00:00:01Z"}},
                "attachments":[]
            });
            let evidence_digest = seal_evidence(&mut evidence)?;
            let evidence_bytes = serde_json::to_vec(&evidence)?;
            fs::write(
                results.join(format!(
                    "{}.json",
                    evidence_digest.trim_start_matches("sha256:")
                )),
                &evidence_bytes,
            )?;
            evidence_dtos.push(EvidenceResultDto::from_json(&evidence_bytes)?);
        }
        let record_dto: ReleaseRecordDto = serde_json::from_value(record)?;
        let runtime_digest: Sha256Digest = runtime_digest.parse()?;
        for key in obligations.obligations.keys().filter(
            |key| matches!(&key.subject, ScopeSubject::Target(target) if target.as_str() == "web"),
        ) {
            if !evidence_dtos.iter().any(|evidence| {
                evidence_coordinate_matches(
                    &session,
                    key,
                    evidence,
                    &record_dto,
                    Some(runtime_digest),
                    &profile_values,
                )
            }) {
                return Err(format!("generated evidence did not match obligation {key:?}").into());
            }
        }
        Ok(repository)
    }

    fn seal_field(value: &mut Value, field: &str) -> Result<String, Box<dyn Error>> {
        let digest =
            Sha256Digest::hash_content(&serde_json_canonicalizer::to_vec(&*value)?).to_string();
        value
            .as_object_mut()
            .ok_or("object")?
            .insert(field.to_owned(), Value::String(digest.clone()));
        Ok(digest)
    }

    fn seal_evidence(value: &mut Value) -> Result<String, Box<dyn Error>> {
        let digest =
            Sha256Digest::hash_content(&serde_json_canonicalizer::to_vec(&*value)?).to_string();
        let object = value.as_object_mut().ok_or("object")?;
        object.insert("id".to_owned(), Value::String(digest.clone()));
        object.insert("result_digest".to_owned(), Value::String(digest.clone()));
        Ok(digest)
    }

    fn copy_fixture(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(destination)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            if entry.file_name() == "GIT_HEAD.fixture" {
                continue;
            }
            let name = if entry.file_name() == "eqm.toml.fixture" {
                "eqm.toml".into()
            } else {
                entry.file_name()
            };
            let target = destination.join(name);
            if entry.file_type()?.is_dir() {
                copy_fixture(&entry.path(), &target)?;
            } else {
                fs::copy(entry.path(), target)?;
            }
        }
        Ok(())
    }

    fn git(root: &Path, arguments: &[&str]) -> Result<(), Box<dyn Error>> {
        if Command::new("git")
            .args(arguments)
            .current_dir(root)
            .status()?
            .success()
        {
            Ok(())
        } else {
            Err("Git fixture command failed".into())
        }
    }
}
