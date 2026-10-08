//! Inherited risk must survive expansion and select actual policy obligations.

mod support;

use eqm_domain::{DimensionId, Facet, PolicyId, ProfileId, Revision, SymbolicValueId, TrustLevel};
use eqm_engine::{
    ApplicabilityContext, AuthorityOrigin, EvaluationMode, FragmentDigestMap, PolicyProfileRequest,
    PolicyRef, ProfileRequest, expand_fragments, resolve_graph, select_policy_profiles,
};
use eqm_manifest::{canonicalize_fragment, load_workspace};
use std::error::Error;
use std::fs;

fn risk_fixture(
    journey: &str,
    fragment: &str,
    explicit: Option<&str>,
    expected: &str,
) -> Result<(tempfile::TempDir, eqm_manifest::LoadedWorkspace), Box<dyn Error>> {
    support::loaded_example_with(|root| {
        let path = root.join("eqm/contracts/auth.signup.toml");
        fs::write(
            &path,
            fs::read_to_string(&path)?.replace(
                "risk_class = \"high\"",
                &format!("risk_class = \"{journey}\""),
            ),
        )?;
        let path = root.join("eqm/contracts/auth.otp_entry.toml");
        let mut text = fs::read_to_string(&path)?.replace(
            "risk_class = \"high\"",
            &format!("risk_class = \"{fragment}\""),
        );
        if let Some(risk) = explicit {
            text.push_str(&format!("\nrisk_class = \"{risk}\"\n"));
        }
        fs::write(&path, text)?;
        // Repin the deliberately authored fixture before expansion. This never
        // removes the pin check and is not production acquisition authority.
        let loaded = load_workspace(root, None)?;
        let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
        let fragment = graph.fragments().values().next().ok_or("fragment")?;
        let digest = canonicalize_fragment(fragment)?.digest();
        let path = root.join("eqm/contracts/auth.signup.otp.toml");
        fs::write(
            &path,
            fs::read_to_string(&path)?.replace(
                "sha256:f4bd7d44c2fc00b13840d5f1d4b04826d6830e698359e4e468ce51d931e36378",
                &digest.to_string(),
            ),
        )?;
        let path = root.join("eqm/policies/consumer.critical_flow.toml");
        let mut text=fs::read_to_string(&path)?.replace("risk_classes = [\"high\", \"critical\"]","requirements = [\"account.create.signup.identifier#email_default\", \"account.create.signup.otp#valid_code_advances\"]");
        text.push_str(&format!("\n[[rules]]\nminimum_level = \"required\"\nfacets = [\"behavior\"]\nminimum_trust = \"signed_ci\"\nmaximum_age = 60000\nminimum_count = 2\n[rules.selector]\nrequirements = [\"account.create.signup.otp#six_decimal_digits\"]\nrisk_classes = [\"{expected}\"]\n"));
        fs::write(path, text)?;
        Ok(())
    })
}

#[test]
fn literal_risk_table_controls_final_policy_matching() -> Result<(), Box<dyn Error>> {
    for (journey, fragment, explicit, expected) in [
        ("low", "high", None, "high"),
        ("high", "low", None, "high"),
        ("high", "critical", None, "critical"),
        ("critical", "high", None, "critical"),
        ("critical", "high", Some("critical"), "critical"),
        ("high", "high", Some("critical"), "critical"),
    ] {
        let (_root, loaded) = risk_fixture(journey, fragment, explicit, expected)?;
        let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
        let digests: FragmentDigestMap = graph
            .fragments()
            .iter()
            .map(|(k, f)| Ok((k.clone(), canonicalize_fragment(f)?.digest())))
            .collect::<Result<_, Box<dyn Error>>>()?;
        let finalized = expand_fragments(graph, &digests, loaded.source_map())?;
        let requirement = finalized
            .graph()
            .surfaces()
            .get(&"account.create.signup.otp".parse()?)
            .ok_or("surface")?
            .requirements()
            .get(&"six_decimal_digits".parse()?)
            .ok_or("requirement")?;
        assert_eq!(
            requirement.risk_class(),
            Some(expected.parse()?),
            "journey={journey} fragment={fragment} explicit={explicit:?}"
        );
        let request = PolicyProfileRequest::new(
            AuthorityOrigin::TrustedInvocation,
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
            finalized.graph(),
            EvaluationMode::PullRequest,
            Some(&request),
            None,
        )?;
        let profile = finalized
            .graph()
            .profiles()
            .values()
            .next()
            .ok_or("profile")?;
        let selected = selection
            .profiles()
            .values()
            .next()
            .ok_or("selected profile")?;
        let applicability =
            ApplicabilityContext::from_profiles(vec![(profile, selected.values())])?;
        let universe =
            support::derive_fixture_universe(&finalized, &selection, &applicability, None)?;
        assert_eq!(universe.coordinates().len(), 5);
        assert!(universe.unresolved().is_empty());
        let copied = universe
            .obligations()
            .values()
            .filter(|o| o.key.requirement.local() == "six_decimal_digits")
            .collect::<Vec<_>>();
        assert_eq!(copied.len(), 2);
        for obligation in copied {
            assert_eq!(obligation.key.facet, Facet::Behavior);
            assert_eq!(obligation.strength.minimum_trust, TrustLevel::SignedCi);
            assert_eq!(obligation.strength.maximum_age.get(), 60_000);
            assert_eq!(obligation.strength.minimum_count.get(), 2);
        }
    }
    Ok(())
}

#[test]
fn explicit_risk_below_inherited_ceiling_rejects_before_derivation() -> Result<(), Box<dyn Error>> {
    let (_root, loaded) = risk_fixture("critical", "high", Some("high"), "critical")?;
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let digests: FragmentDigestMap = graph
        .fragments()
        .iter()
        .map(|(k, f)| Ok((k.clone(), canonicalize_fragment(f)?.digest())))
        .collect::<Result<_, Box<dyn Error>>>()?;
    let error = expand_fragments(graph, &digests, loaded.source_map())
        .err()
        .ok_or("weaker inherited risk accepted")?;
    assert_eq!(error.diagnostics()[0].code().to_string(), "EQM-E0305");
    Ok(())
}
