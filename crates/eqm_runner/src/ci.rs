//! Exact trusted CI-delegated evidence import without local execution.

use crate::persistence::{EvidenceWriteError, validate_evidence_result_bytes};
use eqm_domain::{CiRunRef, ProducerRef, Sha256Digest, TrustLevel};
use eqm_protocol::{EvidenceResultDto, EvidenceSubjectDto};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

const ED25519_SIGNATURE_BYTES: usize = 64;
const ED25519_ALGORITHM: &str = "ed25519";

/// Bounded detached signature metadata supplied with a CI result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CiSignature {
    /// Configured signer identity.
    pub key_id: Box<str>,
    /// Exact approved signature algorithm identity.
    pub algorithm: Box<str>,
    /// Detached signature bytes.
    pub signature: Vec<u8>,
}

/// Successful verifier conclusion over exact input bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedCiSignature {
    /// Verified signer identity.
    pub key_id: Box<str>,
    /// Digest of the exact verified bytes.
    pub payload_digest: Sha256Digest,
    /// Effective trust established independently of the payload claim.
    pub effective_trust: TrustLevel,
}

/// External cryptographic verification boundary.
pub trait CiSignatureVerifier {
    /// Verifies one detached signature over exact result bytes.
    fn verify(
        &self,
        payload: &[u8],
        signature: &CiSignature,
    ) -> Result<VerifiedCiSignature, CiImportError>;
}

/// Exact CI import authority supplied independently from result data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CiImportAuthority {
    /// Immutable CI run reference.
    pub ci_run: CiRunRef,
    /// Exact expected evidence subject.
    pub subject: EvidenceSubjectDto,
    /// Exact expected producer identity.
    pub producer: ProducerRef,
    /// Minimum independently verified trust.
    pub minimum_trust: TrustLevel,
    /// Signer identities allowed for this import.
    pub trusted_signers: BTreeSet<Box<str>>,
}

/// Invocation-scoped replay state.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CiReplayGuard {
    seen_runs: BTreeMap<CiRunRef, Sha256Digest>,
}

/// Validated delegated evidence and verified CI context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CiDelegatedImport {
    /// Closed validated evidence DTO.
    pub evidence: EvidenceResultDto,
    /// Canonical evidence payload digest.
    pub evidence_digest: Sha256Digest,
    /// Exact immutable CI run.
    pub ci_run: CiRunRef,
    /// Independently verified signature conclusion.
    pub signature: VerifiedCiSignature,
}

/// Imports a delegated result without launching any process.
pub fn import_ci_delegated_result(
    bytes: &[u8],
    signature: &CiSignature,
    authority: &CiImportAuthority,
    verifier: &impl CiSignatureVerifier,
    replay: &mut CiReplayGuard,
) -> Result<CiDelegatedImport, CiImportError> {
    validate_signature_metadata(signature)?;
    let (evidence, evidence_digest) =
        validate_evidence_result_bytes(bytes).map_err(CiImportError::Evidence)?;
    if evidence.subject != authority.subject {
        return Err(CiImportError::SubjectMismatch);
    }
    if evidence.producer != authority.producer.as_str() {
        return Err(CiImportError::ProducerMismatch);
    }
    let verified = verifier.verify(bytes, signature)?;
    if verified.payload_digest != Sha256Digest::hash_content(bytes) {
        return Err(CiImportError::SignaturePayloadMismatch);
    }
    if verified.key_id != signature.key_id || !authority.trusted_signers.contains(&verified.key_id)
    {
        return Err(CiImportError::UntrustedSigner);
    }
    if verified.effective_trust < authority.minimum_trust {
        return Err(CiImportError::InsufficientTrust);
    }
    if replay.seen_runs.contains_key(&authority.ci_run) {
        return Err(CiImportError::Replay);
    }
    replay
        .seen_runs
        .insert(authority.ci_run.clone(), evidence_digest);
    Ok(CiDelegatedImport {
        evidence,
        evidence_digest,
        ci_run: authority.ci_run.clone(),
        signature: verified,
    })
}

fn validate_signature_metadata(signature: &CiSignature) -> Result<(), CiImportError> {
    let key_id = signature.key_id.as_ref();
    let valid_key_id = key_id.len() == 71
        && key_id.starts_with("sha256:")
        && key_id[7..]
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
    if !valid_key_id
        || signature.algorithm.as_ref() != ED25519_ALGORITHM
        || signature.signature.len() != ED25519_SIGNATURE_BYTES
    {
        return Err(CiImportError::InvalidSignatureMetadata);
    }
    Ok(())
}

/// CI delegated import failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CiImportError {
    /// Evidence envelope or canonical digest was invalid.
    Evidence(EvidenceWriteError),
    /// Signature metadata was empty, oversized, or unsafe.
    InvalidSignatureMetadata,
    /// Evidence subject differed from explicit CI authority.
    SubjectMismatch,
    /// Producer differed from explicit CI authority.
    ProducerMismatch,
    /// Cryptographic verification failed.
    SignatureInvalid,
    /// Verifier did not bind the exact result bytes.
    SignaturePayloadMismatch,
    /// Verified signer was not authorized.
    UntrustedSigner,
    /// Effective verified trust was below policy.
    InsufficientTrust,
    /// The immutable CI run was already imported.
    Replay,
}

impl Display for CiImportError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for CiImportError {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};
    use std::error::Error;

    fn evidence() -> Result<Vec<u8>, Box<dyn Error>> {
        let mut value = json!({
            "schema": "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/protocol/evidence-result.schema.json",
            "subject": {
                "repository": "https://example.com/team/project",
                "repository_id_digest": "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "scope": {"kind": "target", "target": "web"},
                "source_commit": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "build_id": "build-1",
                "artifact_digest": null,
                "target_configuration_digest": "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
            },
            "target": "web", "unit": "account.create",
            "requirements": ["account.create#works"], "facets": ["behavior"],
            "kind": "release_record",
            "evidence_spec_digest": "sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
            "contract_digest": "sha256:dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
            "binding_digest": "sha256:eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
            "policy_digest": "sha256:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            "runner_digest": null, "adapter_digest": null, "runtime_facts_digest": null,
            "release_record_digest": "sha256:1111111111111111111111111111111111111111111111111111111111111111",
            "profile_values": [], "producer": "producer://ci/actions/run-1",
            "claimed_trust": "signed_ci", "observed_at": "2026-08-07T12:00:00Z",
            "payload": {"kind": "release_record", "release_record_digest": "sha256:1111111111111111111111111111111111111111111111111111111111111111"},
            "attachments": []
        });
        let digest =
            Sha256Digest::hash_content(&serde_json_canonicalizer::to_vec(&value)?).to_string();
        value["id"] = Value::String(digest.clone());
        value["result_digest"] = Value::String(digest);
        Ok(serde_json::to_vec(&value)?)
    }

    struct Verifier {
        trust: TrustLevel,
        wrong_payload: bool,
    }

    impl CiSignatureVerifier for Verifier {
        fn verify(
            &self,
            payload: &[u8],
            signature: &CiSignature,
        ) -> Result<VerifiedCiSignature, CiImportError> {
            Ok(VerifiedCiSignature {
                key_id: signature.key_id.clone(),
                payload_digest: if self.wrong_payload {
                    Sha256Digest::hash_content(b"wrong")
                } else {
                    Sha256Digest::hash_content(payload)
                },
                effective_trust: self.trust,
            })
        }
    }

    fn inputs(bytes: &[u8]) -> Result<(CiSignature, CiImportAuthority), Box<dyn Error>> {
        let dto = EvidenceResultDto::from_json(bytes)?;
        Ok((
            CiSignature {
                key_id: "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                    .into(),
                algorithm: "ed25519".into(),
                signature: vec![1; 64],
            },
            CiImportAuthority {
                ci_run: "ci://github/repo/run-1".parse()?,
                subject: dto.subject,
                producer: "producer://ci/actions/run-1".parse()?,
                minimum_trust: TrustLevel::SignedCi,
                trusted_signers: BTreeSet::from([Box::from(
                    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                )]),
            },
        ))
    }

    #[test]
    fn exact_verified_import_succeeds_once_without_execution() -> Result<(), Box<dyn Error>> {
        let bytes = evidence()?;
        let (signature, authority) = inputs(&bytes)?;
        let verifier = Verifier {
            trust: TrustLevel::SignedCi,
            wrong_payload: false,
        };
        let mut replay = CiReplayGuard::default();
        let imported =
            import_ci_delegated_result(&bytes, &signature, &authority, &verifier, &mut replay)?;
        assert_eq!(imported.ci_run, authority.ci_run);
        assert_eq!(
            import_ci_delegated_result(&bytes, &signature, &authority, &verifier, &mut replay),
            Err(CiImportError::Replay)
        );
        Ok(())
    }

    #[test]
    fn trust_subject_signature_and_replay_inputs_fail_closed() -> Result<(), Box<dyn Error>> {
        let bytes = evidence()?;
        let (signature, mut authority) = inputs(&bytes)?;
        let mut replay = CiReplayGuard::default();
        let low = Verifier {
            trust: TrustLevel::UntrustedLocal,
            wrong_payload: false,
        };
        assert_eq!(
            import_ci_delegated_result(&bytes, &signature, &authority, &low, &mut replay),
            Err(CiImportError::InsufficientTrust)
        );
        let wrong = Verifier {
            trust: TrustLevel::SignedCi,
            wrong_payload: true,
        };
        assert_eq!(
            import_ci_delegated_result(&bytes, &signature, &authority, &wrong, &mut replay),
            Err(CiImportError::SignaturePayloadMismatch)
        );
        authority.subject.scope = eqm_protocol::ScopeSubjectDto::Target {
            target: "ios".to_owned(),
        };
        let valid = Verifier {
            trust: TrustLevel::SignedCi,
            wrong_payload: false,
        };
        assert_eq!(
            import_ci_delegated_result(&bytes, &signature, &authority, &valid, &mut replay),
            Err(CiImportError::SubjectMismatch)
        );
        Ok(())
    }

    #[test]
    fn unsupported_signature_profiles_fail_before_verifier() -> Result<(), Box<dyn Error>> {
        let bytes = evidence()?;
        let (mut signature, authority) = inputs(&bytes)?;
        let verifier = Verifier {
            trust: TrustLevel::SignedCi,
            wrong_payload: false,
        };
        let mut replay = CiReplayGuard::default();

        signature.algorithm = "rsa-pss".into();
        assert_eq!(
            import_ci_delegated_result(&bytes, &signature, &authority, &verifier, &mut replay),
            Err(CiImportError::InvalidSignatureMetadata)
        );
        signature.algorithm = ED25519_ALGORITHM.into();
        signature.key_id = "ci-key".into();
        assert_eq!(
            import_ci_delegated_result(&bytes, &signature, &authority, &verifier, &mut replay),
            Err(CiImportError::InvalidSignatureMetadata)
        );
        signature.key_id =
            "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into();
        signature.signature.pop();
        assert_eq!(
            import_ci_delegated_result(&bytes, &signature, &authority, &verifier, &mut replay),
            Err(CiImportError::InvalidSignatureMetadata)
        );
        Ok(())
    }
}
