//! S039: shared v2 comparison-context identity and `ComparisonContextDigest`.

use eqm_domain::{
    ComparisonContextInputV2, ContractDigest, DimensionId, DurationMillis, Extensions, Facet,
    FullRequirementId, OccurrenceId, OwnerRef, Policy, PolicyId, PolicyRule,
    PolicySelectionInputV2, PolicySelector, PositiveCount, Profile, ProfileDimension, ProfileId,
    ProviderId, Revision, RoleId, Scenario, ScenarioDependency, ScenarioId, ScenarioParticipant,
    ScenarioSubject, SelectedDimensionStateV2, SelectedProfileInputV2, Sha256Digest,
    SymbolicValueId, TargetId, Title, TrustLevel, UnitId, WaiverPolicy,
};
use eqm_manifest::{canonicalize_comparison_context_v2, project_comparison_context_v2};
use std::error::Error;

fn owner(value: &str) -> Result<OwnerRef, Box<dyn Error>> {
    Ok(value.parse()?)
}

fn scenario() -> Result<Scenario, Box<dyn Error>> {
    Ok(Scenario::new(
        ScenarioId::new("signup.flow")?,
        Revision::new(1)?,
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

fn profile() -> Result<Profile, Box<dyn Error>> {
    Ok(Profile::new(
        ProfileId::new("audience.default")?,
        Revision::new(1)?,
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

fn policy() -> Result<Policy, Box<dyn Error>> {
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
        Revision::new(1)?,
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

#[allow(clippy::too_many_arguments)]
fn context<'a>(
    contract: ContractDigest,
    scenario: &'a Scenario,
    selection: &'a PolicySelectionInputV2<'a>,
    rule: Sha256Digest,
) -> ComparisonContextInputV2<'a> {
    ComparisonContextInputV2::new(contract, scenario, selection, rule)
}

#[test]
fn literal_comparison_context_bytes_and_digest_match_independent_oracle()
-> Result<(), Box<dyn Error>> {
    let policy = policy()?;
    let profile = profile()?;
    let selection = selection(&policy, &profile, SelectedDimensionStateV2::Unknown)?;
    let scenario = scenario()?;
    let value = context(
        ContractDigest::hash_content(b"contract"),
        &scenario,
        &selection,
        Sha256Digest::hash_content(b"rule"),
    );
    let canonical = canonicalize_comparison_context_v2(&value)?;
    const BYTES: &str = r#"{"comparison_rule_digest":"sha256:7369949b47ef2c6ba26fbbee659cedc76123a75c17d321a52d8d1ef35e948042","contract_digest":"sha256:b0f7753090d6379ae6fed3d83f216edcc7631454d16bf5552f38c60e1ecd9dd0","profiles":[{"definition":{"defaults":{},"dimensions":[{"id":"locale","values":["en","fr"]},{"id":"region","values":["eu","us"]}],"extensions":{},"id":"audience.default","owners":["owner://team/product"],"revision":1,"title":"Audience"},"values":[{"dimension":"locale","state":"known","value":"en"},{"dimension":"region","state":"unknown"}]}],"scenario_digest":"sha256:67700e7f28e8fe95f8937bc7c17ca8f8d2fd24e0518bab8db9ca946feaba4b28"}"#;
    const DIGEST: &str = "sha256:7695f92b80d4c6d056e073f9257b8397a0e6cf2ece359f43c35d1c0525654ede";
    assert_eq!(std::str::from_utf8(canonical.bytes())?, BYTES);
    assert_eq!(canonical.digest().to_string(), DIGEST);
    assert_eq!(
        canonical.digest(),
        eqm_domain::ComparisonContextDigest::hash_content(canonical.bytes())
    );
    assert!(!canonical.bytes().ends_with(b"\n"));
    Ok(())
}

#[test]
fn distinct_subjects_share_one_comparison_context_but_exact_axes_stay_separate()
-> Result<(), Box<dyn Error>> {
    let policy = policy()?;
    let profile = profile()?;
    let selection = selection(&policy, &profile, SelectedDimensionStateV2::Unknown)?;
    let scenario = scenario()?;
    let base = context(
        ContractDigest::hash_content(b"contract"),
        &scenario,
        &selection,
        Sha256Digest::hash_content(b"rule"),
    );
    // The same shared axes produce one identity regardless of any exact subject
    // build/source/artifact assignment (which is not an input here).
    assert_eq!(
        canonicalize_comparison_context_v2(&base)?,
        canonicalize_comparison_context_v2(&context(
            ContractDigest::hash_content(b"contract"),
            &scenario,
            &selection,
            Sha256Digest::hash_content(b"rule"),
        ))?
    );
    // The projection embeds the independent scenario digest.
    let projected = project_comparison_context_v2(&base)?;
    assert_eq!(
        projected["scenario_digest"],
        serde_json::json!(
            eqm_manifest::canonicalize_scenario_v2(&scenario)?
                .digest()
                .to_string()
        )
    );
    assert_eq!(
        projected["contract_digest"],
        serde_json::json!(ContractDigest::hash_content(b"contract").to_string())
    );
    assert_eq!(
        projected["comparison_rule_digest"],
        serde_json::json!(Sha256Digest::hash_content(b"rule").to_string())
    );
    Ok(())
}

#[test]
fn contract_scenario_rule_and_profile_mutations_change_identity() -> Result<(), Box<dyn Error>> {
    let policy = policy()?;
    let profile = profile()?;
    let unknown = selection(&policy, &profile, SelectedDimensionStateV2::Unknown)?;
    let scenario = scenario()?;
    let base = canonicalize_comparison_context_v2(&context(
        ContractDigest::hash_content(b"contract"),
        &scenario,
        &unknown,
        Sha256Digest::hash_content(b"rule"),
    ))?;
    // Contract identity.
    assert_ne!(
        base.digest(),
        canonicalize_comparison_context_v2(&context(
            ContractDigest::hash_content(b"other"),
            &scenario,
            &unknown,
            Sha256Digest::hash_content(b"rule"),
        ))?
        .digest()
    );
    // Explicit rule authority reference.
    assert_ne!(
        base.digest(),
        canonicalize_comparison_context_v2(&context(
            ContractDigest::hash_content(b"contract"),
            &scenario,
            &unknown,
            Sha256Digest::hash_content(b"other-rule"),
        ))?
        .digest()
    );
    // Logical profile value: known vs unknown.
    let known = selection(
        &policy,
        &profile,
        SelectedDimensionStateV2::Known(SymbolicValueId::new("eu")?),
    )?;
    assert_ne!(
        base.digest(),
        canonicalize_comparison_context_v2(&context(
            ContractDigest::hash_content(b"contract"),
            &scenario,
            &known,
            Sha256Digest::hash_content(b"rule"),
        ))?
        .digest()
    );
    // Different scenario.
    let other_scenario = Scenario::new(
        ScenarioId::new("signup.flow")?,
        Revision::new(2)?,
        vec![ScenarioParticipant::new(
            OccurrenceId::new("phone")?,
            RoleId::new("edge")?,
            ScenarioSubject::Target(TargetId::new("web")?),
        )],
        Vec::new(),
        Extensions::default(),
    )?;
    assert_ne!(
        base.digest(),
        canonicalize_comparison_context_v2(&context(
            ContractDigest::hash_content(b"contract"),
            &other_scenario,
            &unknown,
            Sha256Digest::hash_content(b"rule"),
        ))?
        .digest()
    );
    Ok(())
}

#[test]
fn comparison_rule_authority_is_an_explicit_reference_not_an_invented_default()
-> Result<(), Box<dyn Error>> {
    let policy = policy()?;
    let profile = profile()?;
    let selection = selection(&policy, &profile, SelectedDimensionStateV2::Unknown)?;
    let scenario = scenario()?;
    // Two different explicit structural references yield different identities;
    // no implicit default or policy-digest substitution is applied.
    let first = canonicalize_comparison_context_v2(&context(
        ContractDigest::hash_content(b"contract"),
        &scenario,
        &selection,
        Sha256Digest::hash_content(b"rule-a"),
    ))?;
    let second = canonicalize_comparison_context_v2(&context(
        ContractDigest::hash_content(b"contract"),
        &scenario,
        &selection,
        Sha256Digest::hash_content(b"rule-b"),
    ))?;
    assert_ne!(first.digest(), second.digest());
    Ok(())
}

#[test]
fn empty_dependency_scenario_is_valid_context() -> Result<(), Box<dyn Error>> {
    let policy = policy()?;
    let profile = profile()?;
    let selection = selection(&policy, &profile, SelectedDimensionStateV2::Unknown)?;
    let single = Scenario::new(
        ScenarioId::new("signup.flow")?,
        Revision::new(1)?,
        vec![ScenarioParticipant::new(
            OccurrenceId::new("phone")?,
            RoleId::new("edge")?,
            ScenarioSubject::Target(TargetId::new("web")?),
        )],
        Vec::new(),
        Extensions::default(),
    )?;
    let _ = FullRequirementId::new("account.create#r")?;
    let _ = canonicalize_comparison_context_v2(&context(
        ContractDigest::hash_content(b"contract"),
        &single,
        &selection,
        Sha256Digest::hash_content(b"rule"),
    ))?;
    Ok(())
}
