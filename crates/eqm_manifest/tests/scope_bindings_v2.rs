//! S052-A canonical scope-binding identity, independent literal bytes/digest.

use eqm_domain::{
    FullRequirementId, RequirementScenarioBindings, RequirementScenarioBindingsDigest, Revision,
    ScenarioId, ScenarioRef, Sha256Digest,
};
use eqm_manifest::canonicalize_scope_bindings_v2;
use std::error::Error;

fn binding(
    requirement: &str,
    scenario: &str,
    revision: u64,
) -> Result<(FullRequirementId, ScenarioRef), Box<dyn Error>> {
    Ok((
        FullRequirementId::new(requirement)?,
        ScenarioRef::new(ScenarioId::new(scenario)?, Revision::new(revision)?),
    ))
}

const REQUIREMENT: &str = "account.create.signup.otp#valid_code_advances";
const LITERAL: &[u8] = br#"{"bindings":[{"requirement":"account.create.signup.otp#valid_code_advances","scenario":{"id":"signup.flow","revision":1}}]}"#;

#[test]
fn exact_literal_bytes_and_independent_digest_match() -> Result<(), Box<dyn Error>> {
    let input = RequirementScenarioBindings::new(vec![binding(REQUIREMENT, "signup.flow", 1)?])?;
    let projected = canonicalize_scope_bindings_v2(&input)?;
    assert_eq!(projected.bytes(), LITERAL);
    // Computed independently with SHA-256(label || zero || literal), not from
    // the projection/hash implementation under test.
    assert_eq!(
        projected.digest().to_string(),
        "sha256:07b4a7ad1d48dd466a3ab4e88df41b778e0d66c6be0fdf3add6c9d42532ab6ae"
    );
    assert_ne!(
        projected.digest().as_digest(),
        Sha256Digest::hash_content(LITERAL)
    );
    assert_ne!(
        projected.digest().as_digest(),
        eqm_domain::ScenarioDigest::hash_content(LITERAL).as_digest()
    );
    assert_eq!(
        projected.digest(),
        RequirementScenarioBindingsDigest::hash_content(LITERAL)
    );
    Ok(())
}

#[test]
fn insertion_order_is_invariant_but_every_binding_axis_changes_identity()
-> Result<(), Box<dyn Error>> {
    let first = binding(REQUIREMENT, "signup.flow", 1)?;
    let second = binding("account.create.signup.otp#another_rule", "signup.flow", 1)?;
    let ordered = canonicalize_scope_bindings_v2(&RequirementScenarioBindings::new(vec![
        first.clone(),
        second.clone(),
    ])?)?;
    let reversed = canonicalize_scope_bindings_v2(&RequirementScenarioBindings::new(vec![
        second,
        first.clone(),
    ])?)?;
    assert_eq!(ordered, reversed);
    assert_eq!(ordered.bytes()[0], b'{');
    let original = canonicalize_scope_bindings_v2(&RequirementScenarioBindings::new(vec![first])?)?;
    for changed in [
        binding("account.create.signup.otp#another_rule", "signup.flow", 1)?,
        binding(REQUIREMENT, "signup.other", 1)?,
        binding(REQUIREMENT, "signup.flow", 2)?,
    ] {
        let mutated =
            canonicalize_scope_bindings_v2(&RequirementScenarioBindings::new(vec![changed])?)?;
        assert_ne!(original.digest(), mutated.digest());
    }
    assert_ne!(ordered.digest(), original.digest());
    Ok(())
}

#[test]
fn unsafe_integer_revision_uses_exact_decimal_string_and_empty_map_stays_empty()
-> Result<(), Box<dyn Error>> {
    let large = canonicalize_scope_bindings_v2(&RequirementScenarioBindings::new(vec![binding(
        REQUIREMENT,
        "signup.flow",
        u64::MAX,
    )?])?)?;
    assert_eq!(large.bytes(), br#"{"bindings":[{"requirement":"account.create.signup.otp#valid_code_advances","scenario":{"id":"signup.flow","revision":"18446744073709551615"}}]}"#);
    let empty = canonicalize_scope_bindings_v2(&RequirementScenarioBindings::new(Vec::new())?)?;
    assert_eq!(empty.bytes(), br#"{"bindings":[]}"#);
    assert_ne!(empty.digest(), large.digest());
    Ok(())
}

#[test]
fn revision_safe_integer_boundary_has_exact_wire_forms() -> Result<(), Box<dyn Error>> {
    for (revision, literal) in [
        (9_007_199_254_740_991, br#"{"bindings":[{"requirement":"account.create.signup.otp#valid_code_advances","scenario":{"id":"signup.flow","revision":9007199254740991}}]}"#.as_slice()),
        (9_007_199_254_740_992, br#"{"bindings":[{"requirement":"account.create.signup.otp#valid_code_advances","scenario":{"id":"signup.flow","revision":"9007199254740992"}}]}"#.as_slice()),
    ] {
        let projected = canonicalize_scope_bindings_v2(&RequirementScenarioBindings::new(vec![
            binding(REQUIREMENT, "signup.flow", revision)?,
        ])?)?;
        assert_eq!(projected.bytes(), literal);
    }
    Ok(())
}
