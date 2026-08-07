//! Bounded normalized test-result decoding into validated domain values.

use eqm_domain::{
    EvidenceAttachment, EvidenceAttempt, EvidenceCounts, EvidenceSelector, ExecutionPayload,
    HttpMethod, PositiveCount, ReleaseChannel, SelectorText, Sha256Digest, UtcInstant,
};
use eqm_protocol::{EvidenceDtoError, EvidenceSelectorDto, TestResultDto};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

const MAX_RESULT_BYTES: usize = 16 * 1024 * 1024;
const MAX_ATTACHMENTS: usize = 1_000;
const MAX_ATTACHMENT_BYTES: u64 = 1024 * 1024 * 1024;

/// Fully validated normalized runner output.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizedTestResult {
    selector: EvidenceSelector,
    execution: ExecutionPayload,
    attachments: BTreeMap<Box<str>, EvidenceAttachment>,
}

impl NormalizedTestResult {
    /// Returns the exact provider-neutral selector.
    #[must_use]
    pub const fn selector(&self) -> &EvidenceSelector {
        &self.selector
    }

    /// Returns validated attempts, counts, and time window.
    #[must_use]
    pub const fn execution(&self) -> &ExecutionPayload {
        &self.execution
    }

    /// Returns attachments in unique name order.
    #[must_use]
    pub const fn attachments(&self) -> &BTreeMap<Box<str>, EvidenceAttachment> {
        &self.attachments
    }
}

/// Reads one complete current-schema normalized result under the fixed byte bound.
pub fn read_test_result(bytes: &[u8]) -> Result<NormalizedTestResult, TestResultReadError> {
    if bytes.len() > MAX_RESULT_BYTES {
        return Err(TestResultReadError::TooLarge);
    }
    let dto = TestResultDto::from_json(bytes).map_err(TestResultReadError::Protocol)?;
    let selector = convert_selector(dto.selector)?;
    let attempts = dto
        .attempts
        .into_iter()
        .map(|attempt| {
            EvidenceAttempt::new(
                PositiveCount::new(attempt.number)
                    .map_err(|_| TestResultReadError::InvalidAttempt)?,
                attempt
                    .outcome
                    .parse()
                    .map_err(|_| TestResultReadError::InvalidAttempt)?,
                attempt
                    .started_at
                    .parse()
                    .map_err(|_| TestResultReadError::InvalidAttempt)?,
                attempt
                    .finished_at
                    .parse()
                    .map_err(|_| TestResultReadError::InvalidAttempt)?,
                attempt
                    .message
                    .map(SelectorText::new)
                    .transpose()
                    .map_err(|_| TestResultReadError::InvalidAttempt)?,
            )
            .map_err(|_| TestResultReadError::InvalidAttempt)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let counts = EvidenceCounts::new(
        dto.counts.selected,
        dto.counts.passed,
        dto.counts.failed,
        dto.counts.skipped,
        dto.counts.filtered,
        dto.counts.quarantined,
    )
    .map_err(|_| TestResultReadError::InvalidCounts)?;
    let started_at: UtcInstant = dto
        .started_at
        .parse()
        .map_err(|_| TestResultReadError::InvalidAttempt)?;
    let finished_at: UtcInstant = dto
        .finished_at
        .parse()
        .map_err(|_| TestResultReadError::InvalidAttempt)?;
    let execution = ExecutionPayload::new(attempts, counts, started_at, finished_at)
        .map_err(|_| TestResultReadError::InvalidAttempt)?;
    if dto.attachments.len() > MAX_ATTACHMENTS {
        return Err(TestResultReadError::AttachmentLimit);
    }
    let mut total_size = 0_u64;
    let mut attachments = BTreeMap::new();
    for attachment in dto.attachments {
        total_size = total_size
            .checked_add(attachment.size)
            .ok_or(TestResultReadError::AttachmentLimit)?;
        if total_size > MAX_ATTACHMENT_BYTES {
            return Err(TestResultReadError::AttachmentLimit);
        }
        let name = SelectorText::new(attachment.name)
            .map_err(|_| TestResultReadError::InvalidAttachment)?;
        let item = EvidenceAttachment::new(
            name.clone(),
            SelectorText::new(attachment.media_type)
                .map_err(|_| TestResultReadError::InvalidAttachment)?,
            attachment
                .digest
                .parse::<Sha256Digest>()
                .map_err(|_| TestResultReadError::InvalidAttachment)?,
            attachment.size,
        );
        if attachments.insert(name.as_str().into(), item).is_some() {
            return Err(TestResultReadError::DuplicateAttachment);
        }
    }
    Ok(NormalizedTestResult {
        selector,
        execution,
        attachments,
    })
}

fn convert_selector(value: EvidenceSelectorDto) -> Result<EvidenceSelector, TestResultReadError> {
    let text = |value| SelectorText::new(value).map_err(|_| TestResultReadError::InvalidSelector);
    Ok(match value {
        EvidenceSelectorDto::Symbol { name, language } => EvidenceSelector::Symbol {
            name: text(name)?,
            language: language.map(text).transpose()?,
        },
        EvidenceSelectorDto::Route { path, method } => EvidenceSelector::Route {
            path: text(path)?,
            method: method
                .map(|value| {
                    value
                        .parse::<HttpMethod>()
                        .map_err(|_| TestResultReadError::InvalidSelector)
                })
                .transpose()?,
        },
        EvidenceSelectorDto::Test {
            framework,
            test_id,
            suite,
        } => EvidenceSelector::Test {
            framework: text(framework)?,
            test_id: text(test_id)?,
            suite: suite.map(text).transpose()?,
        },
        EvidenceSelectorDto::Inventory {
            record_type,
            key,
            value,
        } => EvidenceSelector::Inventory {
            record_type: text(record_type)?,
            key: text(key)?,
            value: value.map(text).transpose()?,
        },
        EvidenceSelectorDto::Snapshot {
            snapshot_id,
            variant,
        } => EvidenceSelector::Snapshot {
            snapshot_id: text(snapshot_id)?,
            variant: variant.map(text).transpose()?,
        },
        EvidenceSelectorDto::Release { channel } => EvidenceSelector::Release {
            channel: channel
                .parse::<ReleaseChannel>()
                .map_err(|_| TestResultReadError::InvalidSelector)?,
        },
    })
}

/// Normalized-result decoding or semantic validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TestResultReadError {
    /// Input exceeded 16 MiB before decoding.
    TooLarge,
    /// Public protocol decoding or base validation failed.
    Protocol(EvidenceDtoError),
    /// Selector did not satisfy its typed domain grammar.
    InvalidSelector,
    /// Attempts, messages, numbering, outcomes, or time windows were invalid.
    InvalidAttempt,
    /// Count totals were inconsistent.
    InvalidCounts,
    /// Attachment metadata was invalid.
    InvalidAttachment,
    /// Attachment names were repeated.
    DuplicateAttachment,
    /// Attachment record or total-size bound was exceeded.
    AttachmentLimit,
}

impl Display for TestResultReadError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for TestResultReadError {}

#[cfg(test)]
mod tests {
    use super::*;
    use eqm_domain::{AttemptAggregate, AttemptOutcome};

    fn result(attempts: &str, counts: &str) -> Vec<u8> {
        format!(
            r#"{{"schema":"https://schemas.equivalencematrix.dev/v1/test-result","selector":{{"kind":"test","framework":"cargo","test_id":"suite::case","suite":null}},"attempts":[{attempts}],"counts":{counts},"started_at":"2026-08-07T12:00:00Z","finished_at":"2026-08-07T12:00:03Z","attachments":[]}}"#
        )
        .into_bytes()
    }

    fn attempt(number: u64, outcome: &str) -> String {
        format!(
            r#"{{"number":{number},"outcome":"{outcome}","started_at":"2026-08-07T12:00:00Z","finished_at":"2026-08-07T12:00:01Z","message":null}}"#
        )
    }

    #[test]
    fn pass_failure_skip_filter_and_retry_history_classify_exactly() -> Result<(), Box<dyn Error>> {
        let passed = read_test_result(&result(
            &attempt(1, "passed"),
            r#"{"selected":1,"passed":1,"failed":0,"skipped":0,"filtered":0,"quarantined":0}"#,
        ))?;
        assert_eq!(
            passed.execution().aggregate(PositiveCount::ONE),
            AttemptAggregate::Satisfied
        );
        let failed = read_test_result(&result(
            &attempt(1, "failed"),
            r#"{"selected":1,"passed":0,"failed":1,"skipped":0,"filtered":0,"quarantined":0}"#,
        ))?;
        assert_eq!(
            failed.execution().aggregate(PositiveCount::ONE),
            AttemptAggregate::Failed
        );
        for (field, outcome) in [("skipped", "skipped"), ("filtered", "filtered")] {
            let counts = format!(
                r#"{{"selected":1,"passed":0,"failed":0,"skipped":{},"filtered":{},"quarantined":0}}"#,
                u8::from(field == "skipped"),
                u8::from(field == "filtered")
            );
            let missing = read_test_result(&result(&attempt(1, outcome), &counts))?;
            assert_eq!(
                missing.execution().aggregate(PositiveCount::ONE),
                AttemptAggregate::Missing
            );
        }
        let retry = read_test_result(&result(
            &format!("{},{}", attempt(1, "failed"), attempt(2, "passed")),
            r#"{"selected":1,"passed":1,"failed":0,"skipped":0,"filtered":0,"quarantined":0}"#,
        ))?;
        assert_eq!(
            retry.execution().aggregate(PositiveCount::ONE),
            AttemptAggregate::Unstable
        );
        assert_eq!(
            retry.execution().attempts()[0].outcome(),
            AttemptOutcome::Failed
        );
        Ok(())
    }

    #[test]
    fn schema_counts_limits_and_semantics_fail_closed() {
        let inconsistent = result(
            &attempt(1, "passed"),
            r#"{"selected":2,"passed":1,"failed":0,"skipped":0,"filtered":0,"quarantined":0}"#,
        );
        assert!(matches!(
            read_test_result(&inconsistent),
            Err(TestResultReadError::Protocol(
                EvidenceDtoError::InvalidCounts
            ))
        ));
        let oversized = vec![b' '; MAX_RESULT_BYTES + 1];
        assert_eq!(
            read_test_result(&oversized),
            Err(TestResultReadError::TooLarge)
        );
        let invalid_selector = String::from_utf8(result(
            &attempt(1, "passed"),
            r#"{"selected":1,"passed":1,"failed":0,"skipped":0,"filtered":0,"quarantined":0}"#,
        ))
        .map(|value| value.replace("cargo", ""));
        assert!(invalid_selector.is_ok_and(|value| matches!(
            read_test_result(value.as_bytes()),
            Err(TestResultReadError::InvalidSelector)
        )));
    }
}
