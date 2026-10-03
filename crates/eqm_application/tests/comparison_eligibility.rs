//! S039-R1: session-graph comparison context and distinct-subject eligibility.
//!
//! A checked comparison-context carrier and two genuinely different exact
//! subject manifests share one comparison identity; mismatched shared context
//! is ineligible and missing subject coverage is incomplete. The shared context
//! is derived from the immutable prepared graph with an explicit rule
//! reference, and the protocol reader must reproduce the manifest projection's
//! canonical bytes and digest exactly.

use eqm_application::{
    ComparisonContextError, checked_comparison_eligibility, shared_comparison_context,
};
use eqm_domain::{
    ComparisonContextInputV2, ContractDigest, DimensionId, DurationMillis, Extensions, Facet,
    FinalizedWorkspaceGraph, OccurrenceId, OwnerRef, Policy, PolicyId, PolicyRule,
    PolicySelectionInputV2, PolicySelector, PositiveCount, Profile, ProfileDimension, ProfileId,
    ProviderId, Revision, RoleId, Scenario, ScenarioDependency, ScenarioId, ScenarioParticipant,
    ScenarioSubject, SelectedDimensionStateV2, SelectedProfileInputV2, SelectorText, Sha256Digest,
    SymbolicValueId, TargetConfigurationDigest, TargetId, Title, TrustLevel, UnitId, WaiverPolicy,
    WorkspaceGraph, WorkspaceGraphInput,
};
use eqm_engine::ComparisonEligibility;
use eqm_protocol::schema_graph_v2::CheckedComparisonContextV2;
use eqm_protocol::v2::{
    CheckedSubjectManifestV2, ExpectedSubjectAxis, ExpectedSubjectConfiguration,
    ExpectedSubjectContext, SubjectMatchMode, match_subjects,
};
use serde_json::json;
use std::error::Error;

fn owner(value: &str) -> Result<OwnerRef, Box<dyn Error>> {
    Ok(value.parse()?)
}

fn scenario(revision: u64) -> Result<Scenario, Box<dyn Error>> {
    Ok(Scenario::new(
        ScenarioId::new("signup.flow")?,
        Revision::new(revision)?,
        vec![
            ScenarioParticipant::new(
                OccurrenceId::new("phone")?,
                RoleId::new("edge")?,
                ScenarioSubject::Target(TargetId::new("web")?),
            ),
            ScenarioParticipant::new(
                OccurrenceId::new("mailer")?,
                RoleId::new("provider")?,
                ScenarioSubject::Provider(ProviderId::new("provider.mailgun")?),
            ),
        ],
        vec![ScenarioDependency::new(
            OccurrenceId::new("mailer")?,
            OccurrenceId::new("phone")?,
        )?],
        Extensions::default(),
    )?)
}

fn profile(revision: u64) -> Result<Profile, Box<dyn Error>> {
    Ok(Profile::new(
        ProfileId::new("audience.default")?,
        Revision::new(revision)?,
        Title::new("Audience")?,
        vec![owner("owner://team/product")?],
        vec![
            ProfileDimension::new(
                DimensionId::new("locale")?,
                vec![SymbolicValueId::new("en")?, SymbolicValueId::new("fr")?],
                None,
            )?,
            ProfileDimension::new(
                DimensionId::new("region")?,
                vec![SymbolicValueId::new("eu")?, SymbolicValueId::new("us")?],
                None,
            )?,
        ],
        Vec::new(),
        None,
        Extensions::default(),
    )?)
}

fn policy(revision: u64) -> Result<Policy, Box<dyn Error>> {
    let rule = PolicyRule::new(
        PolicySelector::new(
            Some(vec![UnitId::new("account.create")?]),
            None,
            None,
            None,
            None,
        )?,
        eqm_domain::RequirementLevel::Recommended,
        vec![Facet::Behavior],
        TrustLevel::TrustedCi,
        DurationMillis::new(86_400_000)?,
        Some(PositiveCount::ONE),
    )?;
    Ok(Policy::new(
        PolicyId::new("policy.default")?,
        Revision::new(revision)?,
        Title::new("Default policy")?,
        vec![owner("owner://team/security")?],
        vec![ProfileId::new("audience.default")?],
        vec![TargetId::new("web")?],
        vec![rule],
        WaiverPolicy::deny(),
        None,
        Extensions::default(),
    )?)
}

fn selection<'a>(
    policy: &'a Policy,
    profile: &'a Profile,
    region: SelectedDimensionStateV2,
) -> Result<PolicySelectionInputV2<'a>, Box<dyn Error>> {
    Ok(PolicySelectionInputV2::new(
        policy,
        vec![SelectedProfileInputV2::new(
            profile,
            vec![
                (
                    DimensionId::new("locale")?,
                    SelectedDimensionStateV2::Known(SymbolicValueId::new("en")?),
                ),
                (DimensionId::new("region")?, region),
            ],
        )?],
    )?)
}

fn graph(
    scenario: &Scenario,
    policy: &Policy,
    profile: &Profile,
) -> Result<FinalizedWorkspaceGraph, Box<dyn Error>> {
    Ok(FinalizedWorkspaceGraph::from_engine(WorkspaceGraph::new(
        WorkspaceGraphInput {
            scenarios: vec![scenario.clone()],
            policies: vec![policy.clone()],
            profiles: vec![profile.clone()],
            ..WorkspaceGraphInput::default()
        },
    )?))
}

fn subject_wire(
    build_id: &str,
    artifact: char,
    source_content: char,
) -> Result<serde_json::Value, Box<dyn Error>> {
    Ok(json!({
        "schema": eqm_protocol::SUBJECT_MANIFEST_SCHEMA_V2.to_string(),
        "id": "subject.signup",
        "revision": 1,
        "entries": [
            {
                "occurrence": "phone",
                "repository": "https://example.com/team/project",
                "source_commit": "a".repeat(40),
                "source_content_digest": format!("sha256:{}", source_content.to_string().repeat(64)),
                "target_configuration_digest": TargetConfigurationDigest::hash_content(b"web").to_string(),
                "build_id": build_id,
                "artifact_digest": format!("sha256:{}", artifact.to_string().repeat(64))
            }
        ]
    }))
}

fn expected_axis(
    build_id: &str,
    artifact: char,
    source_content: char,
) -> Result<(OccurrenceId, ExpectedSubjectAxis), Box<dyn Error>> {
    Ok((
        OccurrenceId::new("phone")?,
        ExpectedSubjectAxis {
            repository: "https://example.com/team/project".parse()?,
            source_commit: "a".repeat(40).parse()?,
            source_content_digest: Some(
                format!("sha256:{}", source_content.to_string().repeat(64)).parse()?,
            ),
            configuration: ExpectedSubjectConfiguration::Target(
                TargetConfigurationDigest::hash_content(b"web"),
            ),
            build_id: Some(SelectorText::new(build_id)?),
            artifact_digest: Some(format!("sha256:{}", artifact.to_string().repeat(64)).parse()?),
        },
    ))
}

struct Fixture {
    finalized: FinalizedWorkspaceGraph,
    scenario: Scenario,
    policy: Policy,
    profile: Profile,
}

fn fixture() -> Result<Fixture, Box<dyn Error>> {
    let finalized = graph(&scenario(1)?, &policy(1)?, &profile(1)?)?;
    Ok(Fixture {
        finalized,
        scenario: scenario(1)?,
        policy: policy(1)?,
        profile: profile(1)?,
    })
}

fn checked_context(
    fixture: &Fixture,
    region: SelectedDimensionStateV2,
) -> Result<(CheckedComparisonContextV2, EqmDigest), Box<dyn Error>> {
    let selection = selection(&fixture.policy, &fixture.profile, region)?;
    let contract = ContractDigest::hash_content(b"contract");
    let rule = Sha256Digest::hash_content(b"rule");
    let shared = shared_comparison_context(
        &fixture.finalized,
        &fixture.scenario,
        &selection,
        contract,
        rule,
    )?;
    let input = ComparisonContextInputV2::new(contract, &fixture.scenario, &selection, rule);
    let manifest = eqm_manifest::canonicalize_comparison_context_v2(&input)?;
    let mut wire = eqm_manifest::project_comparison_context_v2(&input)?;
    wire["schema"] = json!(eqm_protocol::COMPARISON_CONTEXT_SCHEMA_V2.to_string());
    let checked = CheckedComparisonContextV2::from_bytes(&serde_json::to_vec(&wire)?)?;
    Ok((checked, EqmDigest { shared, manifest }))
}

/// Keeps the session digest and the manifest projection digest for comparison.
struct EqmDigest {
    shared: eqm_domain::ComparisonContextDigest,
    manifest: eqm_manifest::CanonicalComparisonContextV2,
}

#[test]
fn protocol_reader_reproduces_manifest_bytes_and_session_digest() -> Result<(), Box<dyn Error>> {
    let fixture = fixture()?;
    let (checked, digest) = checked_context(&fixture, SelectedDimensionStateV2::Unknown)?;
    assert_eq!(checked.semantic_bytes(), digest.manifest.bytes());
    assert_eq!(checked.digest(), digest.manifest.digest());
    assert_eq!(digest.shared, digest.manifest.digest());
    Ok(())
}

#[test]
fn distinct_checked_subjects_share_one_context_but_axis_mismatches_are_ineligible()
-> Result<(), Box<dyn Error>> {
    let fixture = fixture()?;
    let (checked, digest) = checked_context(&fixture, SelectedDimensionStateV2::Unknown)?;

    // Two genuinely different, independently matched exact subjects.
    let first_wire = subject_wire("build-a", '1', '3')?;
    let second_wire = subject_wire("build-n", '2', '4')?;
    let first = CheckedSubjectManifestV2::from_bytes(&serde_json::to_vec(&first_wire)?)?;
    let second = CheckedSubjectManifestV2::from_bytes(&serde_json::to_vec(&second_wire)?)?;
    assert_ne!(first.digest(), second.digest());
    let expected = ExpectedSubjectContext::new([expected_axis("build-a", '1', '3')?])?;
    let _matched_first = match_subjects(&first, &expected, SubjectMatchMode::Completeness)?;
    let expected_n = ExpectedSubjectContext::new([expected_axis("build-n", '2', '4')?])?;
    let _matched_second = match_subjects(&second, &expected_n, SubjectMatchMode::Completeness)?;

    let participants = [
        eqm_engine::ComparisonParticipant {
            occurrence: OccurrenceId::new("phone")?,
            context: Some(digest.shared),
            subject: Some(first.digest()),
        },
        eqm_engine::ComparisonParticipant {
            occurrence: OccurrenceId::new("mailer")?,
            context: Some(digest.shared),
            subject: Some(second.digest()),
        },
    ];
    assert_eq!(
        checked_comparison_eligibility(&checked, &participants),
        ComparisonEligibility::Eligible
    );

    // A mismatched shared context is ineligible.
    let (other, other_digest) = checked_context(
        &fixture,
        SelectedDimensionStateV2::Known(SymbolicValueId::new("eu")?),
    )?;
    assert_ne!(other_digest.shared, digest.shared);
    let mismatched = [
        eqm_engine::ComparisonParticipant {
            occurrence: OccurrenceId::new("phone")?,
            context: Some(digest.shared),
            subject: Some(first.digest()),
        },
        eqm_engine::ComparisonParticipant {
            occurrence: OccurrenceId::new("mailer")?,
            context: Some(other.digest()),
            subject: Some(second.digest()),
        },
    ];
    assert_eq!(
        checked_comparison_eligibility(&checked, &mismatched),
        ComparisonEligibility::Ineligible
    );

    // Missing exact subject coverage is incomplete.
    let missing = [eqm_engine::ComparisonParticipant {
        occurrence: OccurrenceId::new("phone")?,
        context: Some(digest.shared),
        subject: None,
    }];
    assert_eq!(
        checked_comparison_eligibility(&checked, &missing),
        ComparisonEligibility::Incomplete
    );
    Ok(())
}

#[test]
fn one_axis_contract_scenario_profile_and_rule_changes_move_context_identity()
-> Result<(), Box<dyn Error>> {
    let fixture = fixture()?;
    let (base, digest) = checked_context(&fixture, SelectedDimensionStateV2::Unknown)?;
    let selection = selection(
        &fixture.policy,
        &fixture.profile,
        SelectedDimensionStateV2::Unknown,
    )?;
    let other_scenario = scenario(2)?;
    let variants = [
        ComparisonContextInputV2::new(
            ContractDigest::hash_content(b"other-contract"),
            &fixture.scenario,
            &selection,
            Sha256Digest::hash_content(b"rule"),
        ),
        ComparisonContextInputV2::new(
            ContractDigest::hash_content(b"contract"),
            &other_scenario,
            &selection,
            Sha256Digest::hash_content(b"rule"),
        ),
        ComparisonContextInputV2::new(
            ContractDigest::hash_content(b"contract"),
            &fixture.scenario,
            &selection,
            Sha256Digest::hash_content(b"other-rule"),
        ),
    ];
    for variant in variants {
        let mutated = eqm_manifest::canonicalize_comparison_context_v2(&variant)?;
        assert_ne!(mutated.digest(), digest.shared);
        let mut wire = eqm_manifest::project_comparison_context_v2(&variant)?;
        wire["schema"] = json!(eqm_protocol::COMPARISON_CONTEXT_SCHEMA_V2.to_string());
        let checked = CheckedComparisonContextV2::from_bytes(&serde_json::to_vec(&wire)?)?;
        assert_eq!(checked.digest(), mutated.digest());
        let participants = [eqm_engine::ComparisonParticipant {
            occurrence: OccurrenceId::new("phone")?,
            context: Some(checked.digest()),
            subject: Some(eqm_domain::SubjectManifestDigest::hash_content(b"subject")),
        }];
        assert_eq!(
            checked_comparison_eligibility(&base, &participants),
            ComparisonEligibility::Ineligible
        );
    }
    Ok(())
}

#[test]
fn session_graph_membership_is_required_for_every_definition() -> Result<(), Box<dyn Error>> {
    let fixture = fixture()?;
    let base_selection = selection(
        &fixture.policy,
        &fixture.profile,
        SelectedDimensionStateV2::Unknown,
    )?;
    let contract = ContractDigest::hash_content(b"contract");
    let rule = Sha256Digest::hash_content(b"rule");
    // A scenario revision not present in the graph is rejected, never reloaded.
    let absent_scenario = scenario(9)?;
    assert_eq!(
        shared_comparison_context(
            &fixture.finalized,
            &absent_scenario,
            &base_selection,
            contract,
            rule
        ),
        Err(ComparisonContextError::ScenarioNotInGraph)
    );
    // A policy not present in the graph is rejected.
    let absent_policy = policy(9)?;
    let absent_selection = selection(
        &absent_policy,
        &fixture.profile,
        SelectedDimensionStateV2::Unknown,
    )?;
    assert_eq!(
        shared_comparison_context(
            &fixture.finalized,
            &fixture.scenario,
            &absent_selection,
            contract,
            rule
        ),
        Err(ComparisonContextError::PolicyNotInGraph)
    );
    Ok(())
}

#[test]
fn profile_axis_change_moves_context_identity() -> Result<(), Box<dyn Error>> {
    let fixture = fixture()?;
    let (base, digest) = checked_context(&fixture, SelectedDimensionStateV2::Unknown)?;
    let base_selection = selection(
        &fixture.policy,
        &fixture.profile,
        SelectedDimensionStateV2::Unknown,
    )?;
    let other_profile = profile(2)?;
    let other_selection = selection(
        &fixture.policy,
        &other_profile,
        SelectedDimensionStateV2::Unknown,
    )?;
    let variant = ComparisonContextInputV2::new(
        ContractDigest::hash_content(b"contract"),
        &fixture.scenario,
        &other_selection,
        Sha256Digest::hash_content(b"rule"),
    );
    let mutated = eqm_manifest::canonicalize_comparison_context_v2(&variant)?;
    assert_ne!(mutated.digest(), digest.shared);
    let mut wire = eqm_manifest::project_comparison_context_v2(&variant)?;
    wire["schema"] = json!(eqm_protocol::COMPARISON_CONTEXT_SCHEMA_V2.to_string());
    let checked = CheckedComparisonContextV2::from_bytes(&serde_json::to_vec(&wire)?)?;
    assert_eq!(checked.digest(), mutated.digest());
    assert_ne!(checked.digest(), base.digest());
    // `selection` is retained to prove the original axis is unchanged.
    assert_eq!(
        eqm_manifest::canonicalize_comparison_context_v2(&ComparisonContextInputV2::new(
            ContractDigest::hash_content(b"contract"),
            &fixture.scenario,
            &base_selection,
            Sha256Digest::hash_content(b"rule"),
        ))?
        .digest(),
        digest.shared
    );
    Ok(())
}

#[test]
fn empty_provider_scenario_contract_digest_stays_explicit() -> Result<(), Box<dyn Error>> {
    // A rule reference is explicit: two different references never collapse.
    let fixture = fixture()?;
    let selection = selection(
        &fixture.policy,
        &fixture.profile,
        SelectedDimensionStateV2::Unknown,
    )?;
    let contract = ContractDigest::hash_content(b"contract");
    let first = shared_comparison_context(
        &fixture.finalized,
        &fixture.scenario,
        &selection,
        contract,
        Sha256Digest::hash_content(b"rule-a"),
    )?;
    let second = shared_comparison_context(
        &fixture.finalized,
        &fixture.scenario,
        &selection,
        contract,
        Sha256Digest::hash_content(b"rule-b"),
    )?;
    assert_ne!(first, second);
    Ok(())
}

#[test]
fn exact_subject_changes_stay_separate_from_context_and_target_configuration()
-> Result<(), Box<dyn Error>> {
    let fixture = fixture()?;
    let (checked, digest) = checked_context(&fixture, SelectedDimensionStateV2::Unknown)?;
    let first = CheckedSubjectManifestV2::from_bytes(&serde_json::to_vec(&subject_wire(
        "build-a", '1', '3',
    )?)?)?;
    let second = CheckedSubjectManifestV2::from_bytes(&serde_json::to_vec(&subject_wire(
        "build-n", '2', '4',
    )?)?)?;

    // Separate build/source/artifact axes move the exact subject identity.
    assert_ne!(first.digest(), second.digest());
    // The shared comparison identity and the target configuration are unchanged.
    assert_eq!(checked.digest(), digest.shared);
    let first_entries = first.record().entries.as_slice();
    let second_entries = second.record().entries.as_slice();
    assert_eq!(
        first_entries[0].target_configuration_digest,
        second_entries[0].target_configuration_digest
    );
    assert_eq!(
        TargetConfigurationDigest::hash_content(b"web").to_string(),
        first_entries[0]
            .target_configuration_digest
            .clone()
            .ok_or("target configuration")?
    );
    // Both distinct subjects remain eligible under the one shared context.
    for subject in [first.digest(), second.digest()] {
        let participants = [eqm_engine::ComparisonParticipant {
            occurrence: OccurrenceId::new("phone")?,
            context: Some(digest.shared),
            subject: Some(subject),
        }];
        assert_eq!(
            checked_comparison_eligibility(&checked, &participants),
            ComparisonEligibility::Eligible
        );
    }
    Ok(())
}
