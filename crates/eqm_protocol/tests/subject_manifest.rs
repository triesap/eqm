//! S038: strict v2 subject-manifest protocol decoding.

use eqm_protocol::{
    CanonicalSet, CheckedSubjectManifestV2, ExpectedSubjectAxis, ExpectedSubjectConfiguration,
    ExpectedSubjectContext, SUBJECT_MANIFEST_SCHEMA_V2, SubjectManifestEntryV2Dto,
    SubjectManifestV2Dto, SubjectMatchError, SubjectMatchMode, WireUint, match_subjects,
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

fn occurrence(value: &str) -> Result<eqm_domain::OccurrenceId, Box<dyn Error>> {
    Ok(eqm_domain::OccurrenceId::new(value)?)
}

fn entry_with(occurrence_: &str, seed: &str) -> SubjectManifestEntryV2Dto {
    SubjectManifestEntryV2Dto {
        occurrence: occurrence_.to_owned(),
        repository: REPOSITORY.to_owned(),
        source_commit: COMMIT.to_owned(),
        source_content_digest: Some(
            eqm_domain::Sha256Digest::hash_content(format!("source-{seed}").as_bytes()).to_string(),
        ),
        target_configuration_digest: Some(
            eqm_domain::TargetConfigurationDigest::hash_content(
                format!("config-{seed}").as_bytes(),
            )
            .to_string(),
        ),
        provider_configuration_digest: None,
        build_id: Some(format!("build.{seed}")),
        artifact_digest: Some(
            eqm_domain::Sha256Digest::hash_content(format!("artifact-{seed}").as_bytes())
                .to_string(),
        ),
    }
}

fn dto(entries: Vec<SubjectManifestEntryV2Dto>) -> Result<SubjectManifestV2Dto, Box<dyn Error>> {
    Ok(SubjectManifestV2Dto {
        schema: SUBJECT_MANIFEST_SCHEMA_V2.to_string(),
        id: "subject.signup".to_owned(),
        revision: WireUint::new(1),
        entries: CanonicalSet::from_members(entries)?,
    })
}

fn checked(
    entries: Vec<SubjectManifestEntryV2Dto>,
) -> Result<CheckedSubjectManifestV2, Box<dyn Error>> {
    Ok(CheckedSubjectManifestV2::from_bytes(&serde_json::to_vec(
        &dto(entries)?,
    )?)?)
}

fn axis(
    occurrence_: &str,
    seed: &str,
) -> Result<(eqm_domain::OccurrenceId, ExpectedSubjectAxis), Box<dyn Error>> {
    Ok((
        occurrence(occurrence_)?,
        ExpectedSubjectAxis {
            repository: REPOSITORY.parse()?,
            source_commit: COMMIT.parse()?,
            source_content_digest: Some(eqm_domain::Sha256Digest::hash_content(
                format!("source-{seed}").as_bytes(),
            )),
            configuration: ExpectedSubjectConfiguration::Target(
                eqm_domain::TargetConfigurationDigest::hash_content(
                    format!("config-{seed}").as_bytes(),
                ),
            ),
            build_id: Some(eqm_domain::SelectorText::new(format!("build.{seed}"))?),
            artifact_digest: Some(eqm_domain::Sha256Digest::hash_content(
                format!("artifact-{seed}").as_bytes(),
            )),
        },
    ))
}

fn base_context() -> Result<ExpectedSubjectContext, Box<dyn Error>> {
    Ok(ExpectedSubjectContext::new([
        axis("phone_one", "one")?,
        axis("phone_two", "two")?,
    ])?)
}

#[test]
fn occurrence_key_duplicate_rejects_before_domain_promotion() -> Result<(), Box<dyn Error>> {
    let value = dto(vec![
        entry_with("phone_one", "one"),
        entry_with("phone_one", "two"),
    ])?;
    assert!(SubjectManifestV2Dto::from_json(&serde_json::to_vec(&value)?).is_err());
    Ok(())
}

#[test]
fn literal_canonical_bytes_and_digest_match_independent_oracle() -> Result<(), Box<dyn Error>> {
    const BYTES: &str = r#"{"entries":[{"artifact_digest":"sha256:2e59c0ca52e152eba0357e4656bcfc4ff121e0c6069d1c7787f1465a211efb47","build_id":"build.one","occurrence":"phone_one","repository":"https://example.com/team/project","source_commit":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","source_content_digest":"sha256:bbe23121ca7ba39e8e00578fff2a60d30661ba124843298851f51d48276d7add","target_configuration_digest":"sha256:c760ab8f3136a60ef07d42e5bfb871b92c0efd569c5d490d2282cb6b2a3c8268"},{"artifact_digest":"sha256:8f2ebabafdd033675d2bc6aca26882b3153ff47416df4d7682b0798741b5ebe2","build_id":"build.two","occurrence":"phone_two","repository":"https://example.com/team/project","source_commit":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","source_content_digest":"sha256:0d586c1b439f5222feb54634f9a1a23e8d55db37dd91acd12a629967d74d13d0","target_configuration_digest":"sha256:ea21b37340a0daa4ffb35dc79fdf6854d1acb27b4372f1df91f859352e2b2e8d"}],"id":"subject.signup","revision":1}"#;
    const DIGEST: &str = "sha256:88865ade62e3ad8e69893b349dfb6950605e6b7671cc0999ac48dacf5fdc4cbb";
    let manifest = checked(vec![
        entry_with("phone_one", "one"),
        entry_with("phone_two", "two"),
    ])?;
    assert_eq!(std::str::from_utf8(manifest.semantic_bytes())?, BYTES);
    assert_eq!(manifest.digest().to_string(), DIGEST);
    assert_eq!(
        manifest.digest(),
        eqm_domain::SubjectManifestDigest::hash_content(manifest.semantic_bytes())
    );
    assert!(!manifest.semantic_bytes().contains(&b'\n'));
    Ok(())
}

#[test]
fn exact_match_promotes_two_occurrences_with_distinct_builds() -> Result<(), Box<dyn Error>> {
    let manifest = match_subjects(
        &checked(vec![
            entry_with("phone_one", "one"),
            entry_with("phone_two", "two"),
        ])?,
        &base_context()?,
        SubjectMatchMode::Completeness,
    )?;
    assert_eq!(manifest.assignments().len(), 2);
    assert!(manifest.require_complete().is_ok());
    assert_ne!(
        manifest.assignments()[0].build_id(),
        manifest.assignments()[1].build_id()
    );
    Ok(())
}

#[test]
fn every_axis_substitution_rejects() -> Result<(), Box<dyn Error>> {
    let checked = checked(vec![
        entry_with("phone_one", "one"),
        entry_with("phone_two", "two"),
    ])?;
    let (occurrence_two, base_two) = axis("phone_two", "two")?;
    let cases: [(ExpectedSubjectAxis, SubjectMatchError); 6] = [
        (
            ExpectedSubjectAxis {
                configuration: ExpectedSubjectConfiguration::Target(
                    eqm_domain::TargetConfigurationDigest::hash_content(b"wrong-config"),
                ),
                ..base_two.clone()
            },
            SubjectMatchError::ConfigurationMismatch(occurrence_two.clone()),
        ),
        (
            ExpectedSubjectAxis {
                repository: "https://example.com/team/other".parse()?,
                ..base_two.clone()
            },
            SubjectMatchError::RepositoryMismatch(occurrence_two.clone()),
        ),
        (
            ExpectedSubjectAxis {
                source_commit: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".parse()?,
                ..base_two.clone()
            },
            SubjectMatchError::CommitMismatch(occurrence_two.clone()),
        ),
        (
            ExpectedSubjectAxis {
                source_content_digest: Some(eqm_domain::Sha256Digest::hash_content(b"wrong")),
                ..base_two.clone()
            },
            SubjectMatchError::SourceContentMismatch(occurrence_two.clone()),
        ),
        (
            ExpectedSubjectAxis {
                build_id: Some(eqm_domain::SelectorText::new("build.wrong")?),
                ..base_two.clone()
            },
            SubjectMatchError::BuildMismatch(occurrence_two.clone()),
        ),
        (
            ExpectedSubjectAxis {
                artifact_digest: Some(eqm_domain::Sha256Digest::hash_content(b"wrong")),
                ..base_two.clone()
            },
            SubjectMatchError::ArtifactMismatch(occurrence_two.clone()),
        ),
    ];
    for (substituted, expected_error) in cases {
        let context = ExpectedSubjectContext::new([
            axis("phone_one", "one")?,
            (occurrence_two.clone(), substituted),
        ])?;
        assert_eq!(
            match_subjects(&checked, &context, SubjectMatchMode::Completeness),
            Err(expected_error)
        );
    }
    Ok(())
}

#[test]
fn missing_extra_and_wrong_kind_occurrences_reject() -> Result<(), Box<dyn Error>> {
    let both = checked(vec![
        entry_with("phone_one", "one"),
        entry_with("phone_two", "two"),
    ])?;
    // Missing scenario occurrence: the manifest omits phone_two.
    let missing_manifest = checked(vec![entry_with("phone_one", "one")])?;
    assert_eq!(
        match_subjects(
            &missing_manifest,
            &base_context()?,
            SubjectMatchMode::Completeness
        ),
        Err(SubjectMatchError::MissingOccurrence(occurrence(
            "phone_two"
        )?))
    );
    // Extra declared occurrence: the manifest adds phone_two the scenario lacks.
    let extra_context = ExpectedSubjectContext::new([axis("phone_one", "one")?])?;
    assert_eq!(
        match_subjects(&both, &extra_context, SubjectMatchMode::Completeness),
        Err(SubjectMatchError::ExtraOccurrence(occurrence("phone_two")?))
    );
    // Wrong kind: a target assignment cannot satisfy a provider expectation.
    let wrong_kind = ExpectedSubjectContext::new([
        axis("phone_one", "one")?,
        (
            occurrence("phone_two")?,
            ExpectedSubjectAxis {
                configuration: ExpectedSubjectConfiguration::Provider(
                    eqm_domain::ProviderConfigurationDigest::hash_content(b"config-two"),
                ),
                ..axis("phone_two", "two")?.1
            },
        ),
    ])?;
    assert_eq!(
        match_subjects(&both, &wrong_kind, SubjectMatchMode::Completeness),
        Err(SubjectMatchError::KindMismatch(occurrence("phone_two")?))
    );
    Ok(())
}

#[test]
fn development_absence_is_allowed_and_completeness_rejects() -> Result<(), Box<dyn Error>> {
    let incomplete = SubjectManifestEntryV2Dto {
        source_content_digest: None,
        build_id: None,
        artifact_digest: None,
        ..entry_with("phone_one", "one")
    };
    let checked = checked(vec![incomplete])?;
    let expected = ExpectedSubjectContext::new([(
        occurrence("phone_one")?,
        ExpectedSubjectAxis {
            repository: REPOSITORY.parse()?,
            source_commit: COMMIT.parse()?,
            source_content_digest: None,
            configuration: ExpectedSubjectConfiguration::Target(
                eqm_domain::TargetConfigurationDigest::hash_content(b"config-one"),
            ),
            build_id: None,
            artifact_digest: None,
        },
    )])?;
    assert!(match_subjects(&checked, &expected, SubjectMatchMode::Development).is_ok());
    assert_eq!(
        match_subjects(&checked, &expected, SubjectMatchMode::Completeness),
        Err(SubjectMatchError::IncompleteOccurrence(occurrence(
            "phone_one"
        )?))
    );
    Ok(())
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
