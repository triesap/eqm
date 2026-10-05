//! Fragment expansion and finalized canonicalization integration fixtures.

use eqm_engine::{FragmentDigestMap, expand_fragments, resolve_graph};
use eqm_manifest::{canonicalize_fragment, canonicalize_graph, load_workspace};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

fn source_root() -> Result<PathBuf, Box<dyn Error>> {
    Ok(Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or("workspace root unavailable")?
        .join("examples/android-ios"))
}

fn digest_map(graph: &eqm_domain::WorkspaceGraph) -> Result<FragmentDigestMap, Box<dyn Error>> {
    graph
        .fragments()
        .iter()
        .map(|(key, fragment)| Ok((key.clone(), canonicalize_fragment(fragment)?.digest())))
        .collect()
}

fn copy_tree(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let target = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

fn workspace() -> Result<tempfile::TempDir, Box<dyn Error>> {
    let root = source_root()?;
    let directory = tempfile::tempdir()?;
    fs::create_dir(directory.path().join(".git"))?;
    for file in ["eqm.toml", "eqm.lock"] {
        fs::copy(root.join(file), directory.path().join(file))?;
    }
    copy_tree(&root.join("eqm"), &directory.path().join("eqm"))?;
    Ok(directory)
}

fn replace(path: PathBuf, before: &str, after: &str) -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string(&path)?;
    if !source.contains(before) {
        return Err(format!("fixture text not found in {}", path.display()).into());
    }
    fs::write(path, source.replacen(before, after, 1))?;
    Ok(())
}

#[test]
fn exact_pins_expand_before_finalized_canonicalization() -> Result<(), Box<dyn Error>> {
    let repository = workspace()?;
    let loaded = load_workspace(repository.path(), None)?;
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let digests = digest_map(&graph)?;
    let finalized = expand_fragments(graph, &digests, loaded.source_map())?;
    let otp = finalized
        .graph()
        .surfaces()
        .get(&"account.create.signup.otp".parse()?)
        .ok_or("OTP surface unavailable")?;
    assert_eq!(otp.requirements().len(), 2);
    assert!(
        otp.requirements()
            .contains_key(&"six_decimal_digits".parse()?)
    );
    let first = canonicalize_graph(&finalized)?;
    let second = canonicalize_graph(&finalized)?;
    assert_eq!(first, second);
    Ok(())
}

#[test]
fn pin_mismatch_and_requirement_collision_fail_closed() -> Result<(), Box<dyn Error>> {
    let repository = workspace()?;
    replace(
        repository.path().join("eqm/contracts/auth.signup.otp.toml"),
        "sha256:f4bd7d44c2fc00b13840d5f1d4b04826d6830e698359e4e468ce51d931e36378",
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    )?;
    let loaded = load_workspace(repository.path(), None)?;
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let error = expand_fragments(graph.clone(), &digest_map(&graph)?, loaded.source_map())
        .err()
        .ok_or("mismatched fragment pin accepted")?;
    assert_eq!(error.diagnostics()[0].code().to_string(), "EQM-E0304");

    let repository = workspace()?;
    replace(
        repository.path().join("eqm/contracts/auth.signup.otp.toml"),
        "id = \"valid_code_advances\"",
        "id = \"six_decimal_digits\"",
    )?;
    let loaded = load_workspace(repository.path(), None)?;
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let error = expand_fragments(graph.clone(), &digest_map(&graph)?, loaded.source_map())
        .err()
        .ok_or("fragment requirement override accepted")?;
    assert_eq!(error.diagnostics()[0].code().to_string(), "EQM-E0305");
    Ok(())
}

#[test]
fn semantic_mutation_changes_final_digest_but_source_layout_does_not() -> Result<(), Box<dyn Error>>
{
    let baseline_repository = workspace()?;
    let baseline = load_workspace(baseline_repository.path(), None)?;
    let graph = resolve_graph(baseline.graph_input().clone(), baseline.source_map())?;
    let finalized = expand_fragments(graph.clone(), &digest_map(&graph)?, baseline.source_map())?;
    let baseline_digest = canonicalize_graph(&finalized)?.digest();
    assert_eq!(
        baseline_digest.to_string(),
        "sha256:4d3252cd506168e9dda01f213a6b4bdf8e0b1441a686fa114341a79f7758f6b3"
    );

    let repository = workspace()?;
    replace(
        repository.path().join("eqm/contracts/account.create.toml"),
        "title = \"Create account\"",
        "title = \"Create a customer account\"",
    )?;
    let loaded = load_workspace(repository.path(), None)?;
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let finalized = expand_fragments(graph.clone(), &digest_map(&graph)?, loaded.source_map())?;
    assert_ne!(canonicalize_graph(&finalized)?.digest(), baseline_digest);

    let repository = workspace()?;
    let fragment_path = repository.path().join("eqm/contracts/auth.otp_entry.toml");
    let fragment = fs::read_to_string(&fragment_path)?;
    fs::write(
        &fragment_path,
        format!(
            "# layout-only comment\n{}",
            fragment.replace("title =", "title    =")
        ),
    )?;
    let loaded = load_workspace(repository.path(), None)?;
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let finalized = expand_fragments(graph.clone(), &digest_map(&graph)?, loaded.source_map())?;
    assert_eq!(canonicalize_graph(&finalized)?.digest(), baseline_digest);
    Ok(())
}

/// Returns the exact inherited risk of the expanded OTP fragment requirement.
fn expanded_otp_risk(
    repository: &tempfile::TempDir,
) -> Result<eqm_domain::RiskClass, Box<dyn Error>> {
    let loaded = load_workspace(repository.path(), None)?;
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let finalized = expand_fragments(graph.clone(), &digest_map(&graph)?, loaded.source_map())?;
    finalized
        .graph()
        .surfaces()
        .get(&"account.create.signup.otp".parse()?)
        .ok_or("OTP surface missing")?
        .requirements()
        .get(&"six_decimal_digits".parse()?)
        .ok_or("requirement missing")?
        .risk_class()
        .ok_or_else(|| "expanded requirement has no risk".into())
}

/// Re-pins the OTP fragment to its mutated canonical digest.
fn repin_otp_fragment(repository: &tempfile::TempDir) -> Result<(), Box<dyn Error>> {
    let loaded = load_workspace(repository.path(), None)?;
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let digest = digest_map(&graph)?
        .get(&(
            eqm_domain::FragmentId::new("auth.otp_entry")?,
            eqm_domain::Revision::new(1)?,
        ))
        .cloned()
        .ok_or("fragment digest missing")?;
    replace(
        repository.path().join("eqm/contracts/auth.signup.otp.toml"),
        "sha256:f4bd7d44c2fc00b13840d5f1d4b04826d6830e698359e4e468ce51d931e36378",
        &digest.to_string(),
    )?;
    Ok(())
}

#[test]
fn expanded_fragment_requirements_inherit_the_stronger_risk() -> Result<(), Box<dyn Error>> {
    // The fragment default (high) is stronger than the destination journey
    // default (low), so the copied requirement inherits high.
    let repository = workspace()?;
    replace(
        repository.path().join("eqm/contracts/auth.signup.toml"),
        "risk_class = \"high\"",
        "risk_class = \"low\"",
    )?;
    assert_eq!(expanded_otp_risk(&repository)?, eqm_domain::RiskClass::High);

    // The journey default (critical) is stronger than the fragment default
    // (high), so the copied requirement inherits critical.
    let repository = workspace()?;
    replace(
        repository.path().join("eqm/contracts/auth.signup.toml"),
        "risk_class = \"high\"",
        "risk_class = \"critical\"",
    )?;
    assert_eq!(
        expanded_otp_risk(&repository)?,
        eqm_domain::RiskClass::Critical
    );

    // An explicit requirement risk between the fragment default (high) and the
    // stronger journey default (critical) rejects after expansion. Pre-expansion
    // validation admits it because the fragment default alone still allows high.
    let repository = workspace()?;
    replace(
        repository.path().join("eqm/contracts/auth.signup.toml"),
        "risk_class = \"high\"",
        "risk_class = \"critical\"",
    )?;
    let fragment_path = repository.path().join("eqm/contracts/auth.otp_entry.toml");
    let text = fs::read_to_string(&fragment_path)?;
    fs::write(&fragment_path, format!("{text}\nrisk_class = \"high\"\n"))?;
    repin_otp_fragment(&repository)?;
    let loaded = load_workspace(repository.path(), None)?;
    let graph = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let error = expand_fragments(graph.clone(), &digest_map(&graph)?, loaded.source_map())
        .err()
        .ok_or("lowered inherited risk accepted")?;
    assert_eq!(error.diagnostics()[0].code().to_string(), "EQM-E0305");

    // An explicit requirement at the inherited ceiling is retained.
    let repository = workspace()?;
    replace(
        repository.path().join("eqm/contracts/auth.signup.toml"),
        "risk_class = \"high\"",
        "risk_class = \"critical\"",
    )?;
    let fragment_path = repository.path().join("eqm/contracts/auth.otp_entry.toml");
    let text = fs::read_to_string(&fragment_path)?;
    fs::write(
        &fragment_path,
        format!("{text}\nrisk_class = \"critical\"\n"),
    )?;
    repin_otp_fragment(&repository)?;
    assert_eq!(
        expanded_otp_risk(&repository)?,
        eqm_domain::RiskClass::Critical
    );
    Ok(())
}
