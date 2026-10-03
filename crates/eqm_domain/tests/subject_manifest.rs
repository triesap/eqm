//! S038: exact multi-participant subject manifests and completeness.

use eqm_domain::{
    OccurrenceId, Revision, SelectorText, Sha256Digest, SourceCommit, SubjectAssignment,
    SubjectManifest, SubjectManifestError, TargetConfigurationDigest,
};
use std::error::Error;

fn repository() -> Result<eqm_domain::RepositoryIdentity, Box<dyn Error>> {
    Ok("https://example.com/team/project".parse()?)
}

fn complete(value: &str) -> Result<SubjectAssignment, Box<dyn Error>> {
    Ok(SubjectAssignment::new(
        OccurrenceId::new(value)?,
        repository()?,
        "a".repeat(40).parse::<SourceCommit>()?,
        Some(Sha256Digest::hash_content(b"source-content")),
        Some(TargetConfigurationDigest::hash_content(b"config")),
        None,
        Some(SelectorText::new("build.one")?),
        Some(Sha256Digest::hash_content(b"artifact")),
    )?)
}

#[test]
fn complete_target_subject_passes_trusted_completeness() -> Result<(), Box<dyn Error>> {
    let assignment = complete("phone_one")?;
    assert!(assignment.is_complete());
    assert_eq!(
        assignment.source_content(),
        Some(Sha256Digest::hash_content(b"source-content"))
    );
    assert_eq!(
        assignment.target_configuration(),
        Some(TargetConfigurationDigest::hash_content(b"config"))
    );
    let manifest = SubjectManifest::new(
        SelectorText::new("signup.subjects")?,
        Revision::new(1)?,
        vec![assignment],
    )?;
    assert!(manifest.require_complete().is_ok());
    Ok(())
}

#[test]
fn a_target_digest_alone_never_satisfies_missing_source_or_build() -> Result<(), Box<dyn Error>> {
    let assignment = SubjectAssignment::new(
        OccurrenceId::new("phone_one")?,
        repository()?,
        "a".repeat(40).parse::<SourceCommit>()?,
        None,
        Some(TargetConfigurationDigest::hash_content(b"config")),
        None,
        None,
        None,
    )?;
    assert!(!assignment.is_complete());
    let manifest = SubjectManifest::new(
        SelectorText::new("signup.subjects")?,
        Revision::new(1)?,
        vec![assignment],
    )?;
    assert_eq!(
        manifest.require_complete(),
        Err(SubjectManifestError::IncompleteSubject)
    );
    Ok(())
}

#[test]
fn configuration_authority_and_duplicate_occurrence_reject() -> Result<(), Box<dyn Error>> {
    // Both target and provider configuration present.
    assert_eq!(
        SubjectAssignment::new(
            OccurrenceId::new("phone_one")?,
            repository()?,
            "a".repeat(40).parse::<SourceCommit>()?,
            None,
            Some(TargetConfigurationDigest::hash_content(b"config")),
            Some(Sha256Digest::hash_content(b"provider")),
            None,
            None,
        ),
        Err(SubjectManifestError::ConfigurationAuthority)
    );
    // A provider assignment is distinct from a target assignment.
    let provider = SubjectAssignment::new(
        OccurrenceId::new("mailer")?,
        repository()?,
        "a".repeat(40).parse::<SourceCommit>()?,
        Some(Sha256Digest::hash_content(b"source-content")),
        None,
        Some(Sha256Digest::hash_content(b"provider")),
        Some(SelectorText::new("build.one")?),
        Some(Sha256Digest::hash_content(b"artifact")),
    )?;
    assert!(provider.provider_configuration().is_some());

    let first = complete("phone_one")?;
    assert_eq!(
        SubjectManifest::new(
            SelectorText::new("signup.subjects")?,
            Revision::new(1)?,
            vec![first.clone(), first],
        ),
        Err(SubjectManifestError::DuplicateAssignment)
    );
    Ok(())
}

#[test]
fn empty_manifest_rejects() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        SubjectManifest::new(
            SelectorText::new("signup.subjects")?,
            Revision::new(1)?,
            Vec::new(),
        ),
        Err(SubjectManifestError::AssignmentsRequired)
    );
    Ok(())
}
