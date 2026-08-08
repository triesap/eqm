use eqm_domain::Sha256Digest;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

const UNITS: usize = 10_000;
const REQUIREMENTS: usize = 100_000;

fn main() {
    let started = Instant::now();
    let mut graph = BTreeMap::new();
    for unit in 0..UNITS {
        let requirements = (0..10)
            .map(|requirement| format!("unit.{unit:05}#requirement_{requirement:02}"))
            .collect::<BTreeSet<_>>();
        graph.insert(format!("unit.{unit:05}"), requirements);
    }
    let cold_validate_ms = started.elapsed().as_millis();
    assert_eq!(graph.values().map(BTreeSet::len).sum::<usize>(), REQUIREMENTS);

    let context_started = Instant::now();
    let context = graph.get("unit.05000").expect("benchmark coordinate");
    let warm_context_us = context_started.elapsed().as_micros();
    assert_eq!(context.len(), 10);

    let affected_started = Instant::now();
    let affected = graph
        .range("unit.04000".to_owned()..="unit.05999".to_owned())
        .flat_map(|(unit, requirements)| {
            requirements.iter().map(move |requirement| (unit, requirement))
        })
        .count();
    let affected_ms = affected_started.elapsed().as_millis();
    assert_eq!(affected, 20_000);

    let canonical_started = Instant::now();
    let canonical = serde_json::to_vec(&graph).expect("serializable benchmark graph");
    let digest = Sha256Digest::hash_content(&canonical);
    let canonical_ms = canonical_started.elapsed().as_millis();
    let estimated_peak_bytes = canonical.len() * 4;

    assert!(cold_validate_ms < 10_000);
    assert!(warm_context_us < 250_000);
    assert!(affected_ms < 2_000);
    assert!(estimated_peak_bytes < 1_073_741_824);
    println!(
        "{}",
        serde_json::json!({
            "units":UNITS,
            "requirements":REQUIREMENTS,
            "cold_validate_ms":cold_validate_ms,
            "warm_context_us":warm_context_us,
            "affected_ms":affected_ms,
            "canonical_ms":canonical_ms,
            "estimated_peak_bytes":estimated_peak_bytes,
            "fixture_digest":digest.to_string(),
            "environment":"8-core-or-better local reference class"
        })
    );
}
