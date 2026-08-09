//! End-to-end graph-resolution fixtures.

mod support;

use eqm_domain::validate_diagnostic_registry;
use eqm_engine::{ResolutionError, resolution_diagnostics, resolve_graph};
use std::error::Error;

#[test]
fn valid_graph_resolves_deterministically() -> Result<(), Box<dyn Error>> {
    let (_repository, loaded) = support::loaded_example()?;
    let first = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    let second = resolve_graph(loaded.graph_input().clone(), loaded.source_map())?;
    assert_eq!(first, second);
    assert_eq!(first.capabilities().len(), 1);
    assert_eq!(first.bindings().len(), 2);
    Ok(())
}

#[test]
fn duplicates_and_dangling_references_are_stable_and_source_linked() -> Result<(), Box<dyn Error>> {
    let (_repository, loaded) = support::loaded_example()?;
    let mut duplicate = loaded.graph_input().clone();
    duplicate
        .capabilities
        .push(duplicate.capabilities[0].clone());
    let error = resolve_graph(duplicate, loaded.source_map())
        .err()
        .ok_or("duplicate accepted")?;
    assert!(matches!(error, ResolutionError::Findings(_)));
    assert_eq!(error.diagnostics()[0].code().to_string(), "EQM-E0300");
    assert!(error.diagnostics()[0].source().is_some());

    let mut dangling = loaded.graph_input().clone();
    dangling.capabilities.clear();
    let first = resolve_graph(dangling.clone(), loaded.source_map())
        .err()
        .ok_or("dangling reference accepted")?;
    let second = resolve_graph(dangling, loaded.source_map())
        .err()
        .ok_or("dangling reference accepted")?;
    assert_eq!(first.diagnostics(), second.diagnostics());
    assert_eq!(first.diagnostics()[0].code().to_string(), "EQM-E0301");
    assert!(first.diagnostics()[0].source().is_some());
    Ok(())
}

#[test]
fn resolution_diagnostic_registry_is_complete_and_sorted() -> Result<(), Box<dyn Error>> {
    let registry = resolution_diagnostics()?;
    validate_diagnostic_registry(&registry)?;
    assert_eq!(registry[0].code.to_string(), "EQM-E0300");
    assert_eq!(registry[1].code.to_string(), "EQM-E0301");
    assert_eq!(registry[2].code.to_string(), "EQM-E0302");
    assert_eq!(registry[3].code.to_string(), "EQM-E0303");
    assert_eq!(registry[4].code.to_string(), "EQM-E0304");
    assert_eq!(registry[5].code.to_string(), "EQM-E0305");
    Ok(())
}
