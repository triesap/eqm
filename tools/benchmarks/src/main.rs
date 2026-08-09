//! Deterministic performance checks for EQM's production paths.

use eqm_domain::{TargetId, UnitId};
use eqm_engine::{
    AffectedIndexes, ChangedFile, FragmentDigestMap, analyze_affected_set, expand_fragments,
    resolve_graph,
};
use eqm_manifest::{canonicalize_fragment, canonicalize_graph, load_workspace};
use eqm_mcp::{McpResourceUri, PreparedMcpSession, read_resource};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::Write as _;
use std::fs;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const UNITS: usize = 10_000;
const SURFACES: usize = UNITS - 2;
const REQUIREMENTS: usize = 100_000;
const COLD_VALIDATE_LIMIT_MS: u64 = 10_000;
const WARM_CONTEXT_LIMIT_US: u64 = 250_000;
const AFFECTED_LIMIT_MS: u64 = 2_000;
const PEAK_MEMORY_LIMIT_BYTES: u64 = 1_073_741_824;

fn main() {
    if let Err(error) = run() {
        eprintln!("benchmark failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    if std::env::args().nth(1).as_deref() == Some("--worker") {
        return worker();
    }
    supervise_worker()
}

fn supervise_worker() -> Result<(), Box<dyn Error>> {
    let executable = std::env::current_exe()?;
    let mut child = Command::new(executable)
        .arg("--worker")
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;
    let mut peak_rss_bytes = 0_u64;
    loop {
        if let Some(resident) = resident_bytes(child.id())? {
            peak_rss_bytes = peak_rss_bytes.max(resident);
        }
        if child.try_wait()?.is_some() {
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }
    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err("production benchmark worker failed".into());
    }
    let mut metrics: Value = serde_json::from_slice(&output.stdout)?;
    if peak_rss_bytes == 0 {
        return Err("benchmark supervisor did not obtain an RSS sample".into());
    }
    metrics["actual_peak_bytes"] = json!(peak_rss_bytes);
    require_below(&metrics, "cold_validate_ms", COLD_VALIDATE_LIMIT_MS)?;
    require_below(&metrics, "warm_context_us", WARM_CONTEXT_LIMIT_US)?;
    require_below(&metrics, "affected_ms", AFFECTED_LIMIT_MS)?;
    if peak_rss_bytes >= PEAK_MEMORY_LIMIT_BYTES {
        return Err(format!(
            "actual peak memory {peak_rss_bytes} exceeded {PEAK_MEMORY_LIMIT_BYTES} bytes"
        )
        .into());
    }
    println!("{}", serde_json::to_string(&metrics)?);
    Ok(())
}

fn resident_bytes(pid: u32) -> Result<Option<u64>, Box<dyn Error>> {
    let output = Command::new("ps")
        .args(["-o", "rss=", "-p", &pid.to_string()])
        .output()?;
    if !output.status.success() {
        return Ok(None);
    }
    let text = String::from_utf8(output.stdout)?;
    let Some(kib) = text.trim().parse::<u64>().ok() else {
        return Ok(None);
    };
    Ok(Some(
        kib.checked_mul(1_024).ok_or("RSS byte count overflowed")?,
    ))
}

fn require_below(metrics: &Value, name: &str, limit: u64) -> Result<(), Box<dyn Error>> {
    let observed = metrics[name]
        .as_u64()
        .ok_or_else(|| format!("missing benchmark metric {name}"))?;
    if observed >= limit {
        Err(format!("{name} {observed} exceeded limit {limit}").into())
    } else {
        Ok(())
    }
}

fn worker() -> Result<(), Box<dyn Error>> {
    let repository = materialize_scale_fixture()?;
    let cold_started = Instant::now();
    let loaded = load_workspace(repository.path(), None)?;
    let source_map = loaded.source_map().clone();
    let graph = resolve_graph(loaded.into_graph_input(), &source_map)?;
    let fragment_digests: FragmentDigestMap = graph
        .fragments()
        .iter()
        .map(|(key, fragment)| {
            canonicalize_fragment(fragment).map(|value| (key.clone(), value.digest()))
        })
        .collect::<Result<_, _>>()?;
    let finalized = expand_fragments(graph, &fragment_digests, &source_map)?;
    let cold_validate_ms = duration_millis(cold_started.elapsed())?;

    let graph = finalized.graph();
    let unit_count = graph.capabilities().len() + graph.journeys().len() + graph.surfaces().len();
    let requirement_count = graph
        .surfaces()
        .values()
        .map(|surface| surface.requirements().len())
        .sum::<usize>();
    if unit_count != UNITS || requirement_count != REQUIREMENTS {
        return Err(format!(
            "fixture shape was {unit_count} units and {requirement_count} requirements"
        )
        .into());
    }

    let canonical_started = Instant::now();
    let canonical = canonicalize_graph(&finalized)?;
    let canonical_ms = duration_millis(canonical_started.elapsed())?;
    let session = PreparedMcpSession::new(
        repository.path(),
        &finalized,
        &source_map,
        canonical.digest(),
    )?;
    let context_uri = McpResourceUri::Context(UnitId::new("bench.scale.flow.unit05000")?);
    let evaluated_at = "2026-08-08T00:00:00Z".parse()?;
    let _ = read_resource(&session, &context_uri, evaluated_at)?;
    let context_started = Instant::now();
    let context = read_resource(&session, &context_uri, evaluated_at)?;
    let warm_context_us = duration_micros(context_started.elapsed())?;
    if !context.text.contains("bench.scale.flow.unit05000") {
        return Err("warm context did not return the requested production unit".into());
    }

    let all_units = graph
        .surfaces()
        .keys()
        .map(|surface| UnitId::new(surface.as_str()))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let target = TargetId::new("web")?;
    let indexes = AffectedIndexes {
        all_units: all_units.clone(),
        target_units: BTreeMap::from([(target.clone(), all_units)]),
        ..AffectedIndexes::default()
    };
    let changed_files = BTreeSet::from([ChangedFile {
        path: "targets/web/new-file".into(),
        target: Some(target),
    }]);
    let affected_started = Instant::now();
    let affected = analyze_affected_set(&indexes, &changed_files, &[]);
    let affected_ms = duration_millis(affected_started.elapsed())?;
    if affected.units.len() != SURFACES || !affected.conservative {
        return Err("production affected analysis was not conservatively complete".into());
    }

    let logical_cpus = std::thread::available_parallelism()?.get();
    println!(
        "{}",
        serde_json::to_string(&json!({
            "units": unit_count,
            "requirements": requirement_count,
            "authority_documents": UNITS,
            "cold_validate_ms": cold_validate_ms,
            "warm_context_us": warm_context_us,
            "affected_ms": affected_ms,
            "canonical_ms": canonical_ms,
            "canonical_bytes": canonical.bytes().len(),
            "fixture_digest": canonical.digest().to_string(),
            "logical_cpus": logical_cpus,
            "memory_measurement": "supervisor sampled child RSS with ps",
        }))?
    );
    thread::sleep(Duration::from_millis(250));
    Ok(())
}

fn materialize_scale_fixture() -> Result<tempfile::TempDir, Box<dyn Error>> {
    let repository = tempfile::tempdir()?;
    fs::create_dir(repository.path().join(".git"))?;
    let contracts = repository.path().join("eqm/contracts");
    fs::create_dir_all(&contracts)?;
    fs::write(
        repository.path().join("eqm.toml"),
        r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/workspace.schema.json"
contract_sources = ["eqm/contracts/*.toml"]
binding_sources = ["eqm/bindings/*.toml"]
policy_sources = ["eqm/policies/*.toml"]
profile_sources = ["eqm/profiles/*.toml"]
runner_sources = ["eqm/runners/*.toml"]
waiver_sources = ["eqm/waivers/*.toml"]
"#,
    )?;
    fs::write(
        repository.path().join("eqm.lock"),
        "schema = \"https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/lock.schema.json\"\nversion = 1\n",
    )?;
    fs::write(
        contracts.join("capability.toml"),
        r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/capability.schema.json"
id = "bench.scale"
title = "Scale benchmark"
status = "active"
owners = ["owner://team/performance"]
"#,
    )?;

    let mut journey = String::from(
        r#"schema = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/journey.schema.json"
id = "bench.scale.flow"
revision = 1
title = "Scale benchmark flow"
capability = "bench.scale"
status = "active"
risk_class = "low"
owners = ["owner://team/performance"]
surfaces = [
"#,
    );
    for surface in 0..SURFACES {
        writeln!(journey, "  \"bench.scale.flow.unit{surface:05}\",")?;
    }
    journey.push_str("]\n");
    fs::write(contracts.join("journey.toml"), journey)?;

    for surface in 0..SURFACES {
        let id = format!("bench.scale.flow.unit{surface:05}");
        let mut document = format!(
            "schema = \"https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/surface.schema.json\"\nid = \"{id}\"\nrevision = 1\ntitle = \"Scale unit {surface:05}\"\njourney = \"bench.scale.flow\"\nstatus = \"active\"\nowners = [\"owner://team/performance\"]\n"
        );
        let count = if surface == 0 { 30 } else { 10 };
        for requirement in 0..count {
            write!(
                document,
                "\n[[requirements]]\nid = \"requirement{requirement:02}\"\nlevel = \"required\"\nscope = \"each_target\"\nstatement = \"Scale requirement {requirement:02} for unit {surface:05}.\"\nfacets = [\"behavior\"]\n"
            )?;
        }
        fs::write(
            contracts.join(format!("surface_{surface:05}.toml")),
            document,
        )?;
    }
    Ok(repository)
}

fn duration_millis(duration: Duration) -> Result<u64, Box<dyn Error>> {
    Ok(u64::try_from(duration.as_millis())?)
}

fn duration_micros(duration: Duration) -> Result<u64, Box<dyn Error>> {
    Ok(u64::try_from(duration.as_micros())?)
}
