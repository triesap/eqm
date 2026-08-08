//! Target artifact and normalized evidence checks for the signup corpus.

use eqm_domain::EvidenceSelector;
use eqm_runner::read_test_result;
use std::error::Error;
use std::fs;
use std::path::Path;

#[test]
fn web_artifact_and_normalized_evidence_are_real_and_current() -> Result<(), Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/signup");
    let route = fs::read_to_string(root.join("targets/web/src/routes/signup/+page.svelte"))?;
    assert!(route.contains("type=\"email\""));
    assert!(route.contains("bind:value"));
    let result = read_test_result(&fs::read(
        root.join("targets/web/evidence/identifier.json"),
    )?)?;
    assert!(matches!(
        result.selector(),
        EvidenceSelector::Test { framework, .. } if framework.as_str() == "vitest"
    ));
    assert_eq!(result.execution().attempts().len(), 1);
    Ok(())
}
