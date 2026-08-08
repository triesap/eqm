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
    InvocationContextDto, ReleaseCheckResultDto, ReleaseRecordDto, ReportEnvelope,
};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

/// Evaluates one exact release record against current policy and prepared evidence.
pub fn execute(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
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
    let runtime_digest = common_runtime_digest(&matching);
    let now = evaluated_at()?;
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
    let facets = obligation_statuses(&obligations, record.target(), &matching, now);
    let conformance = evaluate_target_conformance(record.target(), &facets, true)?;
    let release_facets = facets.values().copied().collect::<Vec<_>>();
    let effective_trust = matching
        .iter()
        .map(|value| value.claimed_trust.parse::<TrustLevel>())
        .chain(std::iter::once(Ok(record.claimed_trust())))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .min();
    let context_complete = !matching.is_empty()
        && runtime_digest.is_some()
        && matching.iter().all(|value| {
            value.contract_digest == session.workspace_digest().to_string()
                && value.policy_digest == session.workspace_digest().to_string()
                && value.profile_values == profile_values
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

fn obligation_statuses(
    derived: &eqm_engine::ObligationDerivation,
    target: &TargetId,
    evidence: &[&EvidenceResultDto],
    evaluated_at: UtcInstant,
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
            });
            let mut statuses = candidates
                .map(|value| {
                    evidence_status(
                        value,
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

fn evidence_status(
    value: &EvidenceResultDto,
    minimum: TrustLevel,
    maximum_age: DurationMillis,
    evaluated_at: UtcInstant,
) -> FacetStatus {
    let Ok(trust) = value.claimed_trust.parse::<TrustLevel>() else {
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
}
