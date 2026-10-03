//! S038: strict v2 subject-manifest protocol decoding.

use eqm_protocol::{
    CanonicalSet, SUBJECT_MANIFEST_SCHEMA_V2, SubjectManifestEntryV2Dto, SubjectManifestV2Dto,
    WireUint,
};
use std::error::Error;

const REPOSITORY: &str = "https://example.com/team/project";
const COMMIT: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn entry() -> SubjectManifestEntryV2Dto {
    SubjectManifestEntryV2Dto {
        occurrence: "phone_one".to_owned(),
        repository: REPOSITORY.to_owned(),
        source_commit: COMMIT.to_owned(),
        source_content_digest: Some(
            eqm_domain::Sha256Digest::hash_content(b"source-content").to_string(),
        ),
        target_configuration_digest: Some(
            eqm_domain::TargetConfigurationDigest::hash_content(b"config").to_string(),
        ),
        provider_configuration_digest: None,
        build_id: Some("build.one".to_owned()),
        artifact_digest: Some(eqm_domain::Sha256Digest::hash_content(b"artifact").to_string()),
    }
}

fn manifest() -> Result<SubjectManifestV2Dto, Box<dyn Error>> {
    Ok(SubjectManifestV2Dto {
        schema: SUBJECT_MANIFEST_SCHEMA_V2.to_string(),
        id: eqm_domain::Sha256Digest::hash_content(b"manifest").to_string(),
        revision: WireUint::new(1),
        entries: CanonicalSet::from_members(vec![entry()])?,
    })
}

fn parse(value: &SubjectManifestV2Dto) -> Result<SubjectManifestV2Dto, Box<dyn Error>> {
    Ok(SubjectManifestV2Dto::from_json(&serde_json::to_vec(
        value,
    )?)?)
}

#[test]
fn valid_subject_manifest_round_trips_strictly() -> Result<(), Box<dyn Error>> {
    let parsed = parse(&manifest()?)?;
    assert_eq!(parsed, manifest()?);
    assert_eq!(parsed.revision.get(), 1);
    // Absent optional members are omitted, never encoded as null.
    let text = serde_json::to_string(&parsed)?;
    assert!(!text.contains("null"));
    assert!(text.contains("source_content_digest"));
    Ok(())
}

#[test]
fn configuration_authority_absence_and_both_present_reject() -> Result<(), Box<dyn Error>> {
    let neither = SubjectManifestEntryV2Dto {
        target_configuration_digest: None,
        provider_configuration_digest: None,
        ..entry()
    };
    let mut value = manifest()?;
    value.entries = CanonicalSet::from_members(vec![neither])?;
    assert!(parse(&value).is_err());

    let both = SubjectManifestEntryV2Dto {
        provider_configuration_digest: Some(
            eqm_domain::Sha256Digest::hash_content(b"provider").to_string(),
        ),
        ..entry()
    };
    value.entries = CanonicalSet::from_members(vec![both])?;
    assert!(parse(&value).is_err());
    Ok(())
}

#[test]
fn wrong_schema_and_unknown_field_fail_closed() -> Result<(), Box<dyn Error>> {
    let mut wrong = manifest()?;
    wrong.schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/protocol/subject-manifest.schema.json".to_owned();
    assert!(parse(&wrong).is_err());

    let text = serde_json::to_string(&manifest()?)?
        .replace("\"revision\":1", "\"revision\":1,\"bogus\":true");
    assert!(SubjectManifestV2Dto::from_json(text.as_bytes()).is_err());
    Ok(())
}
