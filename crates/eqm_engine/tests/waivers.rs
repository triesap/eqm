//! Exact protected waiver evaluation fixtures.

mod support;

use eqm_domain::{
    DimensionId, Extensions, Facet, Policy, PolicyId, PositiveDays, ProfileId, Revision,
    SymbolicValueId, WaiverPolicy,
};
use eqm_engine::{
    ApplicabilityContext, AuthorityOrigin, EvaluationMode, FragmentDigestMap, PolicyProfileRequest,
    PolicyRef, ProfileRequest, WaivableStatus, WaiverEvaluation, WaiverInvalidReason,
    derive_obligations, evaluate_waivers, expand_fragments, resolve_graph, select_policy_profiles,
};
use eqm_manifest::canonicalize_fragment;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

#[test]
fn exact_valid_waiver_is_visible_but_never_satisfies_evidence() -> Result<(), Box<dyn Error>> {
    let (_repository, loaded) = support::loaded_example()?;
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let selection_graph = graph.clone();
    let digests: FragmentDigestMap = graph
        .fragments()
        .iter()
        .map(|(key, fragment)| Ok((key.clone(), canonicalize_fragment(fragment)?.digest())))
        .collect::<Result<_, Box<dyn Error>>>()?;
    let finalized = expand_fragments(graph, &digests, loaded.source_map())?;
    let request = PolicyProfileRequest::new(
        AuthorityOrigin::ProtectedBaseline,
        PolicyRef::new(PolicyId::new("consumer.critical_flow")?, Revision::new(1)?),
        vec![ProfileRequest::new(
            ProfileId::new("audience.default")?,
            Revision::new(1)?,
            vec![(
                DimensionId::new("region")?,
                Some(SymbolicValueId::new("us")?),
            )],
        )?],
    )?;
    let selection = select_policy_profiles(
        &selection_graph,
        EvaluationMode::Release,
        Some(&request),
        None,
    )?;
    let profile = selection_graph
        .profiles()
        .values()
        .next()
        .ok_or("profile missing")?;
    let context = ApplicabilityContext::new(
        profile,
        BTreeMap::from([(
            DimensionId::new("region")?,
            Some(SymbolicValueId::new("us")?),
        )]),
    )?;
    let obligations = derive_obligations(&finalized, &selection, &context, None)?;
    let obligation = obligations
        .obligations
        .keys()
        .find(|key| key.requirement.as_str().ends_with("#email_default"))
        .ok_or("email obligation missing")?;
    let waiver = finalized
        .graph()
        .waivers()
        .values()
        .next()
        .ok_or("waiver missing")?;
    let policy = selection.policy();
    let protected = waiver.approvers().clone();
    let controls = BTreeSet::from([Facet::Behavior]);
    assert_eq!(
        evaluate_waivers(
            &[waiver],
            policy,
            obligation,
            WaivableStatus::Missing,
            "2026-08-15".parse()?,
            &protected,
            &controls,
        ),
        WaiverEvaluation::Waived(waiver.id().clone())
    );
    let satisfied = evaluate_waivers(
        &[waiver],
        policy,
        obligation,
        WaivableStatus::Satisfied,
        "2026-08-15".parse()?,
        &protected,
        &controls,
    );
    assert!(matches!(satisfied, WaiverEvaluation::NotApplied(_)));
    if let WaiverEvaluation::NotApplied(reasons) = satisfied {
        assert!(reasons[waiver.id()].contains(&WaiverInvalidReason::Status));
    }
    let expired = evaluate_waivers(
        &[waiver],
        policy,
        obligation,
        WaivableStatus::Missing,
        "2026-08-31".parse()?,
        &BTreeSet::new(),
        &BTreeSet::new(),
    );
    if let WaiverEvaluation::NotApplied(reasons) = expired {
        assert!(reasons[waiver.id()].contains(&WaiverInvalidReason::Date));
        assert!(reasons[waiver.id()].contains(&WaiverInvalidReason::Approvers));
        assert!(reasons[waiver.id()].contains(&WaiverInvalidReason::UnsatisfiedControls));
    } else {
        return Err("invalid waiver applied".into());
    }
    let stricter = Policy::new(
        policy.id().clone(),
        policy.revision(),
        policy.title().clone(),
        policy.owners().iter().cloned().collect(),
        policy.profiles().iter().cloned().collect(),
        policy.required_targets().iter().cloned().collect(),
        policy.rules().to_vec(),
        WaiverPolicy::new(
            true,
            Some(PositiveDays::new(29)?),
            None,
            vec![Facet::Behavior],
        )?,
        policy.description().cloned(),
        Extensions::default(),
    )?;
    let duration = evaluate_waivers(
        &[waiver],
        &stricter,
        obligation,
        WaivableStatus::Failed,
        "2026-08-15".parse()?,
        &protected,
        &controls,
    );
    if let WaiverEvaluation::NotApplied(reasons) = duration {
        assert!(reasons[waiver.id()].contains(&WaiverInvalidReason::Duration));
    } else {
        return Err("overlong waiver applied".into());
    }
    assert!(matches!(
        evaluate_waivers(
            &[waiver, waiver],
            policy,
            obligation,
            WaivableStatus::Stale,
            "2026-08-15".parse()?,
            &protected,
            &controls,
        ),
        WaiverEvaluation::Ambiguous(_)
    ));
    Ok(())
}

#[test]
fn calendar_day_duration_is_exact_and_ordered() -> Result<(), Box<dyn Error>> {
    let start = "2026-08-01".parse::<eqm_domain::CalendarDate>()?;
    let expiry = "2026-08-31".parse::<eqm_domain::CalendarDate>()?;
    assert_eq!(start.days_until(expiry), Some(30));
    assert_eq!(expiry.days_until(start), None);
    Ok(())
}
