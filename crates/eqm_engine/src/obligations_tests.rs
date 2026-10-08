//! Corruption controls start with real checked derivation, never public forgeries.

#[path = "../tests/support/mod.rs"]
mod support;

use super::*;
use crate::{MonotonicChange, MonotonicityError};
use std::fs;

#[derive(Clone, Copy)]
enum Condition {
    Always,
    Requirement,
    Participation,
}

fn checked(unknown: bool) -> Result<RequiredUniverse, Box<dyn Error>> {
    checked_context(
        if unknown {
            Condition::Requirement
        } else {
            Condition::Always
        },
        if unknown { None } else { Some("us") },
        true,
    )
}

fn checked_context(
    conditional: Condition,
    region: Option<&str>,
    bindings: bool,
) -> Result<RequiredUniverse, Box<dyn Error>> {
    let (_root, loaded) = support::loaded_example_with(|root| {
        let path = root.join("eqm/policies/consumer.critical_flow.toml");
        fs::write(
            &path,
            fs::read_to_string(&path)?
                .replace("trusted_ci", "signed_ci")
                .replace("maximum_age = 86400000", "maximum_age = 60000")
                .replace("minimum_count = 1", "minimum_count = 2"),
        )?;
        if matches!(conditional, Condition::Requirement) {
            let path = root.join("eqm/contracts/auth.signup.identifier.toml");
            let text = fs::read_to_string(&path)?;
            fs::write(
                path,
                format!(
                    "{text}\n[requirements.applicability]\ndimension = \"region\"\noperator = \"eq\"\nvalue = \"eu\"\n"
                ),
            )?;
        }
        if matches!(conditional, Condition::Participation) {
            let path = root.join("eqm/contracts/participation.signup.identifier.toml");
            fs::write(
                &path,
                fs::read_to_string(&path)?.replace(
                    "always = true",
                    "dimension = \"region\"\noperator = \"eq\"\nvalue = \"eu\"",
                ),
            )?;
        }
        Ok(())
    })?;
    let graph = crate::resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let digests: crate::FragmentDigestMap = graph
        .fragments()
        .iter()
        .map(|(k, f)| Ok((k.clone(), eqm_manifest::canonicalize_fragment(f)?.digest())))
        .collect::<Result<_, Box<dyn Error>>>()?;
    let finalized = crate::expand_fragments(graph, &digests, loaded.source_map())?;
    let request = crate::PolicyProfileRequest::new(
        crate::AuthorityOrigin::TrustedInvocation,
        crate::PolicyRef::new("consumer.critical_flow".parse()?, Revision::new(1)?),
        vec![crate::ProfileRequest::new(
            "audience.default".parse()?,
            Revision::new(1)?,
            vec![("region".parse()?, region.map(str::parse).transpose()?)],
        )?],
    )?;
    let selection = crate::select_policy_profiles(
        finalized.graph(),
        crate::EvaluationMode::PullRequest,
        Some(&request),
        None,
    )?;
    let profile = finalized
        .graph()
        .profiles()
        .values()
        .next()
        .ok_or("profile")?;
    let values = selection
        .profiles()
        .values()
        .next()
        .ok_or("selected")?
        .values();
    let applicability = ApplicabilityContext::from_profiles(vec![(profile, values)])?;
    Ok(if bindings {
        support::derive_fixture_universe(&finalized, &selection, &applicability, None)?
    } else {
        derive_required_universe(&finalized, &selection, &applicability, None)?
    })
}

#[test]
fn omission_and_independent_strength_weakening_are_detected() -> Result<(), Box<dyn Error>> {
    let baseline = checked(false)?;
    assert_eq!(
        enforce_required_universe_strengthening(&baseline, &baseline)?,
        MonotonicChange::Unchanged
    );
    let key = baseline
        .coordinates
        .iter()
        .next()
        .ok_or("coordinate")?
        .clone();
    let mut omitted = baseline.clone();
    omitted.coordinates.remove(&key);
    omitted.context.obligations.remove(&key);
    assert_eq!(
        enforce_required_universe_strengthening(&omitted, &baseline),
        Err(MonotonicityError::Weakening)
    );
    assert!(!baseline.is_subset_of(&omitted));
    for axis in 0..3 {
        let mut candidate = baseline.clone();
        let strength = &mut candidate
            .context
            .obligations
            .get_mut(&key)
            .ok_or("obligation")?
            .strength;
        match axis {
            0 => strength.minimum_trust = TrustLevel::TrustedCi,
            1 => strength.maximum_age = DurationMillis::new(60_001)?,
            _ => strength.minimum_count = PositiveCount::ONE,
        }
        assert_eq!(
            enforce_required_universe_strengthening(&candidate, &baseline),
            Err(MonotonicityError::Weakening),
            "axis={axis}"
        );
        assert!(!baseline.is_subset_of(&candidate));
    }
    let mut incoherent = baseline.clone();
    incoherent.coordinates.remove(&key);
    assert_eq!(
        enforce_required_universe_strengthening(&incoherent, &baseline),
        Err(MonotonicityError::InvalidPreparedInput)
    );
    Ok(())
}

#[test]
fn unresolved_omission_or_unknown_to_na_substitution_never_strengthens()
-> Result<(), Box<dyn Error>> {
    let baseline = checked(true)?;
    assert_eq!(baseline.unresolved.len(), 2);
    let coordinate = baseline
        .unresolved
        .iter()
        .next()
        .ok_or("unresolved")?
        .clone();
    assert_eq!(coordinate.reason, UnresolvedReason::UnknownApplicability);
    for substitute in [false, true] {
        let mut candidate = baseline.clone();
        candidate.unresolved.remove(&coordinate);
        if substitute {
            let mut changed = coordinate.clone();
            changed.reason = UnresolvedReason::NotApplicable;
            candidate.unresolved.insert(changed);
        }
        assert_eq!(
            enforce_required_universe_strengthening(&candidate, &baseline),
            Err(MonotonicityError::Weakening)
        );
        assert!(!baseline.is_subset_of(&candidate));
    }
    Ok(())
}

#[test]
fn mode_origin_scope_and_expected_selection_contexts_are_not_interchangeable()
-> Result<(), Box<dyn Error>> {
    let baseline = checked(false)?;
    let mut mode = baseline.clone();
    mode.context.mode = crate::EvaluationMode::Release;
    let mut origin = baseline.clone();
    origin.context.origin = crate::AuthorityOrigin::ProtectedBaseline;
    let mut scope = baseline.clone();
    scope.context.scope_bindings = RequirementScenarioBindings::new(Vec::new())?;
    let selection = crate::bind_expected_selections(baseline.clone(), BTreeMap::new())?;
    for candidate in [mode, origin, scope, selection] {
        assert_eq!(
            enforce_required_universe_strengthening(&candidate, &baseline),
            Err(MonotonicityError::IncomparableAuthority)
        );
        assert!(!baseline.is_subset_of(&candidate));
    }
    Ok(())
}

#[test]
fn real_false_and_missing_scope_reasons_must_stay_visible() -> Result<(), Box<dyn Error>> {
    for (baseline, reason) in [
        (
            checked_context(Condition::Participation, None, true)?,
            UnresolvedReason::UnknownParticipation,
        ),
        (
            checked_context(Condition::Requirement, Some("us"), true)?,
            UnresolvedReason::NotApplicable,
        ),
        (
            checked_context(Condition::Always, Some("us"), false)?,
            UnresolvedReason::MissingScopeAuthority,
        ),
    ] {
        let coordinate = baseline
            .unresolved
            .iter()
            .find(|c| c.reason == reason)
            .ok_or("expected real unresolved coordinate")?
            .clone();
        assert_eq!(
            enforce_required_universe_strengthening(&baseline, &baseline)?,
            MonotonicChange::Unchanged
        );
        let mut deleted = baseline.clone();
        deleted.unresolved.remove(&coordinate);
        assert_eq!(
            enforce_required_universe_strengthening(&deleted, &baseline),
            Err(MonotonicityError::Weakening)
        );
        assert!(!baseline.is_subset_of(&deleted));
    }
    Ok(())
}
