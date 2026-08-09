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
        "sha256:edaea5fb3e53ebac478e1916ac360b137ad281bfed2e8914dc67c5eb8b39eb78"
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
