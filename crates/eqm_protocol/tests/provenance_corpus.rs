//! S027: shared cross-domain provenance corpus at the real protocol boundary.
//!
//! The raw/semantic and bound-axis vectors come from the shared
//! `eqm_test_support::provenance` corpus; the literal semantic bytes and digest
//! are reviewed constants, and the raw identity is recomputed from the exact
//! bytes through its own domain.

use eqm_domain::{ContractDigest, RawObjectDigest, SemanticEvidenceDigest};
use eqm_protocol::CheckedEvidenceResultV2;
use eqm_test_support::{
    EvidenceAxis, STANDARD_EVIDENCE_SEMANTIC_BYTES, STANDARD_EVIDENCE_SEMANTIC_DIGEST, mutate_axis,
    standard_evidence_record,
};
use std::error::Error;

fn standard_bytes() -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(serde_json::to_vec(&standard_evidence_record()?)?)
}

fn seal(bytes: &[u8]) -> Result<CheckedEvidenceResultV2, Box<dyn Error>> {
    Ok(CheckedEvidenceResultV2::from_bytes(bytes)?)
}

#[test]
fn shared_evidence_corpus_matches_the_literal_semantic_oracle() -> Result<(), Box<dyn Error>> {
    let bytes = standard_bytes()?;
    let checked = seal(&bytes)?;
    assert_eq!(
        std::str::from_utf8(checked.semantic_bytes())?,
        STANDARD_EVIDENCE_SEMANTIC_BYTES
    );
    assert_eq!(
        checked.semantic_identity().to_string(),
        STANDARD_EVIDENCE_SEMANTIC_DIGEST
    );
    // The raw identity is recomputed from the exact bytes under its own domain
    // and stays distinct from the canonical semantic identity.
    assert_eq!(
        checked.raw_identity(),
        RawObjectDigest::hash_content(&bytes)
    );
    assert_ne!(
        checked.raw_identity().to_string(),
        checked.semantic_identity().to_string()
    );
    Ok(())
}

#[test]
fn shared_evidence_corpus_is_deterministic() -> Result<(), Box<dyn Error>> {
    let first = standard_bytes()?;
    let second = standard_bytes()?;
    assert_eq!(first, second);
    assert_eq!(
        seal(&first)?.semantic_identity(),
        seal(&second)?.semantic_identity()
    );
    Ok(())
}

#[test]
fn formatting_only_variants_preserve_semantic_identity() -> Result<(), Box<dyn Error>> {
    let baseline = seal(&standard_bytes()?)?;

    // Pretty-printed JSON is a whitespace-only change.
    let pretty = serde_json::to_vec_pretty(&standard_evidence_record()?)?;
    assert_ne!(pretty, standard_bytes()?);
    let pretty_checked = seal(&pretty)?;
    assert_eq!(
        pretty_checked.semantic_identity(),
        baseline.semantic_identity()
    );
    assert_ne!(pretty_checked.raw_identity(), baseline.raw_identity());

    // A Value round-trip reorders object members without changing semantics.
    let value: serde_json::Value = serde_json::from_slice(&standard_bytes()?)?;
    let reordered = serde_json::to_vec(&value)?;
    let reordered_checked = seal(&reordered)?;
    assert_eq!(
        reordered_checked.semantic_identity(),
        baseline.semantic_identity()
    );
    Ok(())
}

#[test]
fn every_bound_axis_changes_the_semantic_identity() -> Result<(), Box<dyn Error>> {
    let baseline = seal(&standard_bytes()?)?.semantic_identity();
    for axis in EvidenceAxis::ALL {
        let mutant = mutate_axis(&standard_evidence_record()?, axis)?;
        let bytes = serde_json::to_vec(&mutant)?;
        let checked = seal(&bytes)?;
        assert_ne!(
            checked.semantic_identity(),
            baseline,
            "axis {axis:?} did not move the semantic identity"
        );
    }
    Ok(())
}

#[test]
fn semantic_and_component_identities_are_domain_separated() -> Result<(), Box<dyn Error>> {
    let bytes = standard_bytes()?;
    let checked = seal(&bytes)?;
    let semantic = checked.semantic_identity();
    // The same semantic bytes under another domain produce a different identity.
    assert_ne!(
        semantic,
        SemanticEvidenceDigest::hash_content(&bytes),
        "the canonical semantic identity is not the raw hash"
    );
    assert_ne!(
        ContractDigest::hash_content(&bytes).to_string(),
        semantic.to_string()
    );
    Ok(())
}
