//! Repository automation for EquivalenceMatrix.

use flate2::{Compression, GzBuilder};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::fs::{self, File};
use std::io::{self};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Output};
use tar::{Builder, Header};
use tempfile::{NamedTempFile, TempDir};

const NIGHTLY: &str = "nightly-2026-07-16";
const SCHEMA_COUNT: usize = 22;
const FUZZ_TARGETS: [&str; 7] = [
    "toml",
    "protocol",
    "adapter",
    "inventory",
    "evidence",
    "canonicalization",
    "graph",
];

fn main() -> ExitCode {
    match run(env::args_os().skip(1)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("xtask: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: impl IntoIterator<Item = OsString>) -> Result<()> {
    let mut args = args.into_iter();
    let Some(command) = args.next() else {
        print_help();
        return Ok(());
    };
    let trailing = args.collect::<Vec<_>>();

    match command.to_str() {
        Some("check") if trailing.is_empty() => check(),
        Some("verify") if trailing.is_empty() => verify(),
        Some("schemas") => schemas(&trailing),
        Some("test") => test_lane(&trailing),
        Some("benchmark") if trailing.is_empty() => benchmark(),
        Some("dist") if trailing.len() == 1 => package(Path::new(&trailing[0])).map(|_| ()),
        Some("help" | "--help" | "-h") if trailing.is_empty() => {
            print_help();
            Ok(())
        }
        Some(value) => Err(Error::usage(format!("unsupported arguments for `{value}`"))),
        None => Err(Error::usage("command must be valid UTF-8")),
    }
}

fn check() -> Result<()> {
    check_generated_state()?;
    check_no_legacy_names(false, &[repository_root()])?;
    check_schemas()?;
    check_schema_parity()?;
    cargo(["fmt", "--all", "--check"])?;
    cargo(["check", "--workspace", "--all-targets", "--locked"])?;
    cargo(["test", "--workspace", "--all-targets", "--locked"])?;
    cargo([
        "clippy",
        "--workspace",
        "--all-targets",
        "--locked",
        "--",
        "-D",
        "warnings",
    ])?;
    cargo(["doc", "--workspace", "--no-deps", "--locked"])?;
    check_end_to_end()?;
    git(["diff", "--check"])
}

fn verify() -> Result<()> {
    check()?;
    check_security_matrix()?;
    cargo(["audit", "--deny", "warnings"])?;
    cargo(["deny", "check"])?;
    coverage()?;
    mutation()?;
    fuzz()?;
    benchmark()?;
    check_package()?;
    check_generated_state()
}

fn schemas(args: &[OsString]) -> Result<()> {
    match args {
        [command] if command == "generate" => {
            generate_schemas(&repository_root().join("schemas/v1"))
        }
        [command] if command == "check" => check_schemas(),
        _ => Err(Error::usage(
            "schemas requires exactly one of `generate` or `check`",
        )),
    }
}

fn test_lane(args: &[OsString]) -> Result<()> {
    let [lane] = args else {
        return Err(Error::usage(
            "test requires one of `security`, `coverage`, `mutation`, or `fuzz`",
        ));
    };
    match lane.to_str() {
        Some("security") => check_security_matrix(),
        Some("coverage") => coverage(),
        Some("mutation") => mutation(),
        Some("fuzz") => fuzz(),
        _ => Err(Error::usage(format!(
            "unsupported test lane `{}`",
            lane.to_string_lossy()
        ))),
    }
}

fn check_generated_state() -> Result<()> {
    let root = repository_root();
    if root.join(".eqm").exists() {
        return Err(Error::message(
            "generated .eqm state must not exist during repository verification",
        ));
    }
    let output = capture("git", ["ls-files", ".eqm/**", "target/**"])?;
    if !output.stdout.is_empty() {
        return Err(Error::message("generated state is tracked by Git"));
    }
    let ignored = Command::new("git")
        .args(["check-ignore", "-q", ".eqm/"])
        .current_dir(&root)
        .status()
        .map_err(|source| Error::spawn("git check-ignore -q .eqm/", source))?;
    if !ignored.success() {
        return Err(Error::message("root .eqm state is not ignored by Git"));
    }
    Ok(())
}

fn check_no_legacy_names(include_negative: bool, roots: &[PathBuf]) -> Result<()> {
    let forbidden = [
        ["Feature", "Matrix"].concat(),
        ["fm", "tx"].concat(),
        ["FM", "TX"].concat(),
        [".fm", "tx"].concat(),
        ["fm", "tx.toml"].concat(),
    ];
    for root in roots {
        for path in files_below(root)? {
            let relative = path.strip_prefix(repository_root()).unwrap_or(&path);
            let relative_text = relative.to_string_lossy();
            if relative_text.starts_with(".git/")
                || relative_text.starts_with("target/")
                || (!include_negative
                    && relative_text.starts_with("tests/fixtures/no_legacy/negative/"))
            {
                continue;
            }
            let Ok(contents) = fs::read_to_string(&path) else {
                continue;
            };
            if let Some(name) = forbidden
                .iter()
                .find(|name| contents.contains(name.as_str()))
            {
                return Err(Error::message(format!(
                    "forbidden compatibility identifier `{name}` detected in {}",
                    path.display()
                )));
            }
        }
    }
    Ok(())
}

fn generate_schemas(output: &Path) -> Result<()> {
    execute(
        "cargo",
        [
            OsStr::new("run"),
            OsStr::new("--quiet"),
            OsStr::new("--locked"),
            OsStr::new("-p"),
            OsStr::new("eqm_manifest"),
            OsStr::new("--bin"),
            OsStr::new("generate_manifest_schemas"),
            OsStr::new("--"),
            output.join("manifest").as_os_str(),
        ],
    )?;
    execute(
        "cargo",
        [
            OsStr::new("run"),
            OsStr::new("--quiet"),
            OsStr::new("--locked"),
            OsStr::new("-p"),
            OsStr::new("eqm_protocol"),
            OsStr::new("--bin"),
            OsStr::new("generate_protocol_schemas"),
            OsStr::new("--"),
            output.join("protocol").as_os_str(),
        ],
    )
}

fn check_schemas() -> Result<()> {
    let temporary = TempDir::new().map_err(Error::io)?;
    let generated = temporary.path().join("schemas/v1");
    generate_schemas(&generated)?;
    compare_trees(&repository_root().join("schemas/v1"), &generated)
}

fn check_schema_parity() -> Result<()> {
    let root = repository_root();
    let schema_root = root.join("schemas/v1");
    let mut known = BTreeSet::new();
    for path in files_below(&schema_root)? {
        if path.extension() != Some(OsStr::new("json")) {
            continue;
        }
        let document: Value =
            serde_json::from_reader(File::open(&path).map_err(Error::io)?).map_err(Error::json)?;
        let id = document.get("$id").and_then(Value::as_str).ok_or_else(|| {
            Error::message(format!("schema has no string $id: {}", path.display()))
        })?;
        if !known.insert(id.to_owned()) {
            return Err(Error::message(format!("duplicate schema $id: {id}")));
        }
    }
    if known.len() != SCHEMA_COUNT {
        return Err(Error::message(format!(
            "expected {SCHEMA_COUNT} schemas, found {}",
            known.len()
        )));
    }
    for scan_root in [
        root.join("examples/android-ios"),
        root.join("tests/fixtures/signup"),
    ] {
        for path in files_below(&scan_root)? {
            let Ok(contents) = fs::read_to_string(&path) else {
                continue;
            };
            for token in contents.split(|character: char| {
                character.is_whitespace() || matches!(character, '"' | '\'' | ',' | ']' | '}')
            }) {
                let token = token.trim_end_matches('\\');
                if token
                    .starts_with("https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/")
                    && !known.contains(token)
                {
                    return Err(Error::message(format!(
                        "unknown schema URI in {}: {token}",
                        path.display()
                    )));
                }
            }
        }
    }
    println!("schema-parity schemas={SCHEMA_COUNT} status=ok");
    Ok(())
}

fn check_end_to_end() -> Result<()> {
    cargo([
        "test",
        "-p",
        "eqm",
        "--locked",
        "renderer::tests::reviewed_signup_goldens_cover_the_public_surface_and_are_byte_stable",
    ])?;
    cargo([
        "test",
        "-p",
        "eqm",
        "--locked",
        "commands::mcp::tests::stdio_handshake_lists_and_calls_are_json_only",
    ])?;
    cargo([
        "test",
        "-p",
        "eqm",
        "--locked",
        "commands::release_check::tests::parsed_release_cli_exercises_pass_fail_and_unknown_with_exact_inputs",
    ])?;
    cargo([
        "test",
        "-p",
        "eqm_runner",
        "--test",
        "signup_fixture",
        "--locked",
    ])?;
    println!("end-to-end: CLI, MCP, release outcomes, and three-target fixture passed");
    Ok(())
}

fn check_security_matrix() -> Result<()> {
    let root = repository_root();
    let matrix =
        fs::read_to_string(root.join("tests/security/adversarial-cases.tsv")).map_err(Error::io)?;
    let mut lines = matrix.lines();
    if lines.next() != Some("case\tsource\tpackage\ttarget\ttest") {
        return Err(Error::message("invalid security matrix header"));
    }
    let mut count = 0;
    for line in lines {
        let fields = line.split('\t').collect::<Vec<_>>();
        let [case_name, source, package, target, test_name] = fields.as_slice() else {
            return Err(Error::message(format!(
                "invalid security matrix row: {line}"
            )));
        };
        let source_text = fs::read_to_string(root.join(source)).map_err(Error::io)?;
        let short_name = test_name.rsplit("::").next().unwrap_or(test_name);
        if !source_text.contains(&format!("fn {short_name}(")) {
            return Err(Error::message(format!(
                "security test not found: {test_name}"
            )));
        }
        let mut args = vec!["test", "-p", package];
        match target.split_once(':') {
            None if *target == "lib" => args.push("--lib"),
            Some(("test", name)) => args.extend(["--test", name]),
            Some(("bin", name)) => args.extend(["--bin", name]),
            _ => {
                return Err(Error::message(format!(
                    "unsupported security target: {target}"
                )));
            }
        }
        args.extend(["--locked", test_name, "--", "--exact", "--nocapture"]);
        let output = capture("cargo", args)?;
        let log = String::from_utf8_lossy(&output.stdout).to_string()
            + &String::from_utf8_lossy(&output.stderr);
        if !log.contains(&format!("test {test_name} ... ok"))
            || log.contains("sensitive-value")
            || log.contains("secret://vault/token")
        {
            return Err(Error::message(format!("security case failed: {case_name}")));
        }
        println!("security-case name={case_name} status=ok");
        count += 1;
    }
    if count != 12 {
        return Err(Error::message(format!(
            "expected 12 security cases, found {count}"
        )));
    }
    println!("security-matrix cases={count} status=ok");
    Ok(())
}

fn coverage() -> Result<()> {
    let report = NamedTempFile::new().map_err(Error::io)?;
    let nightly = format!("+{NIGHTLY}");
    execute(
        "cargo",
        [
            OsStr::new(&nightly),
            OsStr::new("llvm-cov"),
            OsStr::new("-p"),
            OsStr::new("eqm_engine"),
            OsStr::new("--all-targets"),
            OsStr::new("--locked"),
            OsStr::new("--branch"),
            OsStr::new("--json"),
            OsStr::new("--output-path"),
            report.path().as_os_str(),
        ],
    )?;
    let value: Value =
        serde_json::from_reader(report.reopen().map_err(Error::io)?).map_err(Error::json)?;
    let files = value
        .pointer("/data/0/files")
        .and_then(Value::as_array)
        .ok_or_else(|| Error::message("coverage report has no files"))?;
    let core = [
        "applicability.rs",
        "conformance.rs",
        "coverage.rs",
        "equivalence.rs",
        "exposure.rs",
        "freshness.rs",
        "matrix.rs",
        "monotonicity.rs",
        "release.rs",
    ];
    let mut line_count = 0;
    let mut line_covered = 0;
    let mut branch_count = 0;
    let mut branch_covered = 0;
    for file in files {
        let filename = file
            .get("filename")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if !filename.contains("eqm_engine/src/")
            || !core.iter().any(|name| filename.ends_with(name))
        {
            continue;
        }
        line_count += json_u64(file, "/summary/lines/count")?;
        line_covered += json_u64(file, "/summary/lines/covered")?;
        branch_count += json_u64(file, "/summary/branches/count")?;
        branch_covered += json_u64(file, "/summary/branches/covered")?;
    }
    enforce_ratio("line", line_covered, line_count, 90)?;
    enforce_ratio("branch", branch_covered, branch_count, 85)?;
    println!(
        "core coverage: lines {line_covered}/{line_count}, branches {branch_covered}/{branch_count}"
    );
    Ok(())
}

fn mutation() -> Result<()> {
    let root = repository_root();
    let output_root = root.join("mutants.out");
    if output_root.exists() {
        return Err(Error::message(format!(
            "mutation output already exists: {}",
            output_root.display()
        )));
    }
    let target = env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("target"))
        .join("mutants-critical");
    let status = Command::new("cargo")
        .args([
            "mutants",
            "-p",
            "eqm_engine",
            "--file",
            "crates/eqm_engine/src/{conformance,monotonicity,release}.rs",
            "--in-place",
            "--timeout",
            "60",
            "--minimum-test-timeout",
            "20",
            "--colors",
            "never",
        ])
        .env("CARGO_TARGET_DIR", target)
        .current_dir(&root)
        .status()
        .map_err(|source| Error::spawn("cargo mutants", source))?;
    let result = inspect_mutation(&output_root, status.success());
    if output_root.exists() {
        fs::remove_dir_all(&output_root).map_err(Error::io)?;
    }
    result
}

fn inspect_mutation(root: &Path, command_succeeded: bool) -> Result<()> {
    let outcomes: Value =
        serde_json::from_reader(File::open(root.join("outcomes.json")).map_err(Error::io)?)
            .map_err(Error::json)?;
    if outcomes
        .pointer("/outcomes/0/summary")
        .and_then(Value::as_str)
        != Some("Success")
    {
        return Err(Error::message("mutation run did not report Success"));
    }
    let generated = serde_json::from_reader::<_, Value>(
        File::open(root.join("mutants.json")).map_err(Error::io)?,
    )
    .map_err(Error::json)?
    .as_array()
    .map(Vec::len)
    .ok_or_else(|| Error::message("mutants.json is not an array"))?;
    let caught = line_count(&root.join("caught.txt"))?;
    let missed = line_count(&root.join("missed.txt"))?;
    let timed_out = line_count(&root.join("timeout.txt"))?;
    let unviable = line_count(&root.join("unviable.txt"))?;
    if generated != caught + missed + timed_out + unviable {
        return Err(Error::message(
            "mutation classifications do not cover generated mutants",
        ));
    }
    let viable = caught + missed + timed_out;
    let killed =
        u64::try_from(caught + timed_out).map_err(|error| Error::message(error.to_string()))?;
    let viable_ratio = u64::try_from(viable).map_err(|error| Error::message(error.to_string()))?;
    enforce_ratio("mutation", killed, viable_ratio, 80)?;
    if !command_succeeded && missed == 0 && timed_out == 0 {
        return Err(Error::message(
            "mutation runner failed without a classified survivor",
        ));
    }
    println!(
        "critical mutation: killed {}/{viable}, missed {missed}, unviable {unviable}",
        caught + timed_out
    );
    Ok(())
}

fn fuzz() -> Result<()> {
    let campaigns = TempDir::new().map_err(Error::io)?;
    let nightly = format!("+{NIGHTLY}");
    for target in FUZZ_TARGETS {
        let root = campaigns.path().join(target);
        let corpus = root.join("corpus");
        let artifacts = root.join("artifacts");
        fs::create_dir_all(&corpus).map_err(Error::io)?;
        fs::create_dir_all(&artifacts).map_err(Error::io)?;
        let prefix = format!("-artifact_prefix={}/", artifacts.display());
        let mut command = Command::new("cargo");
        command
            .args([nightly.as_str(), "fuzz", "run", target])
            .arg(&corpus)
            .args(["--", "-runs=1000", "-timeout=10", &prefix])
            .current_dir(repository_root().join("tools/fuzz"));
        execute_command(command)?;
    }
    println!("fuzz smoke: 7 production targets x 1000 runs passed");
    Ok(())
}

fn benchmark() -> Result<()> {
    cargo([
        "run",
        "--release",
        "--package",
        "eqm-benchmarks",
        "--locked",
    ])
}

fn check_package() -> Result<()> {
    let temporary = TempDir::new().map_err(Error::io)?;
    let first = package(&temporary.path().join("first"))?;
    let second = package(&temporary.path().join("second"))?;
    if fs::read(&first).map_err(Error::io)? != fs::read(&second).map_err(Error::io)? {
        return Err(Error::message(
            "distribution archives are not byte-identical",
        ));
    }
    println!("package: two byte-identical archives with SBOM and provenance inputs");
    Ok(())
}

fn package(output: &Path) -> Result<PathBuf> {
    fs::create_dir_all(output).map_err(Error::io)?;
    cargo(["build", "--release", "--locked", "-p", "eqm"])?;
    let root = repository_root();
    let metadata: Value = serde_json::from_slice(
        &capture("cargo", ["metadata", "--format-version", "1", "--locked"])?.stdout,
    )
    .map_err(Error::json)?;
    let target_directory = metadata
        .get("target_directory")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::message("Cargo metadata omitted target_directory"))?;
    let version = metadata
        .get("packages")
        .and_then(Value::as_array)
        .and_then(|packages| {
            packages
                .iter()
                .find(|package| package.get("name").and_then(Value::as_str) == Some("eqm"))
        })
        .and_then(|package| package.get("version"))
        .and_then(Value::as_str)
        .ok_or_else(|| Error::message("Cargo metadata omitted the eqm version"))?;
    let rustc = String::from_utf8(capture("rustc", ["-vV"])?.stdout)
        .map_err(|error| Error::message(error.to_string()))?;
    let host = rustc
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .ok_or_else(|| Error::message("rustc -vV omitted the host triple"))?;
    let package_name = format!("eqm-{version}-{host}");
    let staging = TempDir::new().map_err(Error::io)?;
    let package_root = staging.path().join(&package_name);
    fs::create_dir_all(package_root.join("bin")).map_err(Error::io)?;
    copy_file(
        Path::new(target_directory).join("release/eqm"),
        package_root.join("bin/eqm"),
    )?;
    copy_tree(&root.join("schemas"), &package_root.join("schemas"))?;
    for name in ["README.md", "LICENSE-APACHE", "LICENSE-MIT"] {
        copy_file(root.join(name), package_root.join(name))?;
    }
    let packages = metadata
        .get("packages")
        .and_then(Value::as_array)
        .ok_or_else(|| Error::message("Cargo metadata omitted packages"))?;
    let sbom_packages = packages
        .iter()
        .map(|package| {
            json!({
                "licenseDeclared": package.get("license").cloned().unwrap_or(Value::Null),
                "name": package.get("name").cloned().unwrap_or(Value::Null),
                "versionInfo": package.get("version").cloned().unwrap_or(Value::Null),
            })
        })
        .collect::<Vec<_>>();
    write_json(
        &package_root.join("SBOM.spdx.json"),
        &json!({
            "name": "eqm", "packages": sbom_packages, "spdxVersion": "SPDX-2.3"
        }),
    )?;
    let commit = String::from_utf8(capture("git", ["rev-parse", "HEAD"])?.stdout)
        .map_err(|error| Error::message(error.to_string()))?;
    let lock_digest = sha256_file(&root.join("Cargo.lock"))?;
    write_json(
        &package_root.join("provenance-inputs.json"),
        &json!({
            "builder": "local-dry-run", "cargo_lock_sha256": lock_digest,
            "production_signature": false, "source_commit": commit.trim()
        }),
    )?;
    check_no_legacy_names(true, std::slice::from_ref(&package_root))?;
    let archive = output.join(format!("{package_name}.tar.gz"));
    write_archive(&archive, staging.path(), &package_name)?;
    let digest = sha256_file(&archive)?;
    fs::write(
        archive.with_extension("gz.sha256"),
        format!(
            "{digest}  {}\n",
            archive.file_name().unwrap_or_default().to_string_lossy()
        ),
    )
    .map_err(Error::io)?;
    println!("{}", archive.display());
    Ok(archive)
}

fn write_archive(destination: &Path, staging: &Path, package_name: &str) -> Result<()> {
    let gzip = GzBuilder::new().mtime(0).write(
        File::create(destination).map_err(Error::io)?,
        Compression::best(),
    );
    let mut archive = Builder::new(gzip);
    archive.mode(tar::HeaderMode::Deterministic);
    let root = staging.join(package_name);
    let mut files = files_below(&root)?;
    files.sort();
    for source in files {
        let relative = source
            .strip_prefix(staging)
            .map_err(|error| Error::message(error.to_string()))?;
        let mut header = Header::new_gnu();
        let metadata = fs::metadata(&source).map_err(Error::io)?;
        header.set_size(metadata.len());
        header.set_mode(if relative.ends_with("bin/eqm") {
            0o755
        } else {
            0o644
        });
        header.set_mtime(0);
        header.set_uid(0);
        header.set_gid(0);
        header.set_cksum();
        archive
            .append_data(
                &mut header,
                relative,
                File::open(&source).map_err(Error::io)?,
            )
            .map_err(Error::io)?;
    }
    archive
        .into_inner()
        .map_err(Error::io)?
        .finish()
        .map_err(Error::io)?;
    Ok(())
}

fn files_below(root: &Path) -> Result<Vec<PathBuf>> {
    if root.is_file() {
        return Ok(vec![root.to_path_buf()]);
    }
    let mut pending = vec![root.to_path_buf()];
    let mut files = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory).map_err(Error::io)? {
            let path = entry.map_err(Error::io)?.path();
            if path.is_dir() {
                if path.file_name() == Some(OsStr::new(".git"))
                    || path.file_name() == Some(OsStr::new("target"))
                {
                    continue;
                }
                pending.push(path);
            } else if path.is_file() {
                files.push(path);
            }
        }
    }
    files.sort();
    Ok(files)
}

fn compare_trees(expected: &Path, actual: &Path) -> Result<()> {
    let relative = |root: &Path| -> Result<Vec<PathBuf>> {
        files_below(root)?
            .into_iter()
            .map(|path| {
                path.strip_prefix(root)
                    .map(Path::to_path_buf)
                    .map_err(|error| Error::message(error.to_string()))
            })
            .collect()
    };
    let expected_files = relative(expected)?;
    let actual_files = relative(actual)?;
    if expected_files != actual_files {
        return Err(Error::message(
            "generated schema inventory differs from committed schemas",
        ));
    }
    for path in expected_files {
        if fs::read(expected.join(&path)).map_err(Error::io)?
            != fs::read(actual.join(&path)).map_err(Error::io)?
        {
            return Err(Error::message(format!(
                "generated schema differs: {}",
                path.display()
            )));
        }
    }
    Ok(())
}

fn copy_tree(source: &Path, destination: &Path) -> Result<()> {
    for path in files_below(source)? {
        let relative = path
            .strip_prefix(source)
            .map_err(|error| Error::message(error.to_string()))?;
        copy_file(&path, destination.join(relative))?;
    }
    Ok(())
}

fn copy_file(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> Result<()> {
    let destination = destination.as_ref();
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).map_err(Error::io)?;
    }
    fs::copy(source, destination).map_err(Error::io)?;
    Ok(())
}

fn write_json(path: &Path, value: &Value) -> Result<()> {
    let mut bytes = serde_json::to_vec_pretty(value).map_err(Error::json)?;
    bytes.push(b'\n');
    fs::write(path, bytes).map_err(Error::io)
}

fn sha256_file(path: &Path) -> Result<String> {
    let mut file = File::open(path).map_err(Error::io)?;
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher).map_err(Error::io)?;
    Ok(format!("{:x}", hasher.finalize()))
}

fn line_count(path: &Path) -> Result<usize> {
    Ok(fs::read_to_string(path).map_err(Error::io)?.lines().count())
}

fn json_u64(value: &Value, pointer: &str) -> Result<u64> {
    value
        .pointer(pointer)
        .and_then(Value::as_u64)
        .ok_or_else(|| Error::message(format!("JSON report omitted {pointer}")))
}

fn enforce_ratio(name: &str, covered: u64, count: u64, percent: u64) -> Result<()> {
    if count == 0 || covered * 100 < count * percent {
        return Err(Error::message(format!(
            "{name} threshold failed: {covered}/{count} < {percent}%"
        )));
    }
    Ok(())
}

fn cargo<const N: usize>(args: [&str; N]) -> Result<()> {
    execute("cargo", args)
}

fn git<const N: usize>(args: [&str; N]) -> Result<()> {
    execute("git", args)
}

fn execute<I, S>(program: &str, args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new(program);
    command.args(args).current_dir(repository_root());
    execute_command(command)
}

fn execute_command(mut command: Command) -> Result<()> {
    let display = format!("{command:?}");
    let status = command
        .status()
        .map_err(|source| Error::spawn(display.clone(), source))?;
    if status.success() {
        Ok(())
    } else {
        Err(Error::status(display, status.code()))
    }
}

fn capture<I, S>(program: &str, args: I) -> Result<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new(program);
    command.args(args).current_dir(repository_root());
    let display = format!("{command:?}");
    let output = command
        .output()
        .map_err(|source| Error::spawn(display.clone(), source))?;
    if output.status.success() {
        Ok(output)
    } else {
        Err(Error::status(display, output.status.code()))
    }
}

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn print_help() {
    println!(
        "EquivalenceMatrix repository tasks\n\n\
         Usage: cargo xtask <COMMAND>\n\n\
         Commands:\n\
           check                   Run the contributor verification gate\n\
           verify                  Run the complete release-candidate gate\n\
           schemas generate       Regenerate committed schemas\n\
           schemas check          Verify committed schemas are current\n\
           test security          Run the adversarial security matrix\n\
           test coverage          Enforce core coverage thresholds\n\
           test mutation          Enforce critical mutation thresholds\n\
           test fuzz              Run bounded production fuzz targets\n\
           benchmark              Run the production-scale benchmark\n\
           dist <OUTPUT>           Build an unsigned reproducible archive"
    );
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum Error {
    Message(String),
    Spawn { display: String, source: io::Error },
    Status { display: String, code: Option<i32> },
}

impl Error {
    fn usage(message: impl Into<String>) -> Self {
        Self::Message(format!("{}; run `cargo xtask help`", message.into()))
    }

    fn message(message: impl Into<String>) -> Self {
        Self::Message(message.into())
    }

    fn io(source: io::Error) -> Self {
        Self::Message(source.to_string())
    }

    fn json(source: serde_json::Error) -> Self {
        Self::Message(source.to_string())
    }

    fn spawn(display: impl Into<String>, source: io::Error) -> Self {
        Self::Spawn {
            display: display.into(),
            source,
        }
    }

    fn status(display: impl Into<String>, code: Option<i32>) -> Self {
        Self::Status {
            display: display.into(),
            code,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Message(message) => formatter.write_str(message),
            Self::Spawn { display, source } => {
                write!(formatter, "failed to start {display}: {source}")
            }
            Self::Status {
                display,
                code: Some(code),
            } => write!(formatter, "{display} exited with status {code}"),
            Self::Status {
                display,
                code: None,
            } => write!(formatter, "{display} terminated by a signal"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repository_root_contains_the_workspace_manifest() {
        assert!(repository_root().join("Cargo.toml").is_file());
    }

    #[test]
    fn unsupported_command_is_a_usage_error() {
        assert!(run([OsString::from("unknown")]).is_err());
    }

    #[test]
    fn incomplete_test_lane_is_a_usage_error() {
        assert!(run([OsString::from("test")]).is_err());
    }
}
