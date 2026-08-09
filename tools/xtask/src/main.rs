//! Repository automation for EquivalenceMatrix.

use std::env;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    match run(env::args_os().skip(1)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("xtask: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: impl IntoIterator<Item = OsString>) -> Result<(), Error> {
    let mut args = args.into_iter();
    let Some(command) = args.next() else {
        print_help();
        return Ok(());
    };
    let trailing = args.collect::<Vec<_>>();

    match command.to_str() {
        Some("check") if trailing.is_empty() => check(),
        Some("verify") if trailing.is_empty() => shell_script("scripts/verify.sh", &[]),
        Some("schemas") => schemas(&trailing),
        Some("test") => test_lane(&trailing),
        Some("benchmark") if trailing.is_empty() => {
            shell_script("scripts/check_performance.sh", &[])
        }
        Some("dist") if trailing.len() == 1 => {
            shell_script("scripts/package_release.sh", &trailing)
        }
        Some("help" | "--help" | "-h") if trailing.is_empty() => {
            print_help();
            Ok(())
        }
        Some(value) => Err(Error::Usage(format!("unsupported arguments for `{value}`"))),
        None => Err(Error::Usage("command must be valid UTF-8".to_owned())),
    }
}

fn check() -> Result<(), Error> {
    shell_script("scripts/test_no_legacy_names.sh", &[])?;
    shell_script("scripts/check_schemas.sh", &[])?;
    shell_script("scripts/check_schema_parity.sh", &[])?;
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
    shell_script("scripts/check_end_to_end.sh", &[])?;
    git(["diff", "--check"])
}

fn schemas(args: &[OsString]) -> Result<(), Error> {
    match args {
        [command] if command == "generate" => shell_script("scripts/generate_schemas.sh", &[]),
        [command] if command == "check" => shell_script("scripts/check_schemas.sh", &[]),
        _ => Err(Error::Usage(
            "schemas requires exactly one of `generate` or `check`".to_owned(),
        )),
    }
}

fn test_lane(args: &[OsString]) -> Result<(), Error> {
    let [lane] = args else {
        return Err(Error::Usage(
            "test requires one of `security`, `coverage`, `mutation`, or `fuzz`".to_owned(),
        ));
    };
    let script = match lane.to_str() {
        Some("security") => "scripts/check_security_matrix.sh",
        Some("coverage") => "scripts/check_core_coverage.sh",
        Some("mutation") => "scripts/check_critical_mutation.sh",
        Some("fuzz") => "scripts/check_fuzz_smoke.sh",
        _ => {
            return Err(Error::Usage(format!(
                "unsupported test lane `{}`",
                lane.to_string_lossy()
            )));
        }
    };
    shell_script(script, &[])
}

fn cargo<const N: usize>(args: [&str; N]) -> Result<(), Error> {
    execute("cargo", args)
}

fn git<const N: usize>(args: [&str; N]) -> Result<(), Error> {
    execute("git", args)
}

fn shell_script(script: &str, args: &[OsString]) -> Result<(), Error> {
    let mut command = Command::new("bash");
    command
        .arg(script)
        .args(args)
        .current_dir(repository_root());
    execute_command(command)
}

fn execute<I, S>(program: &str, args: I) -> Result<(), Error>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new(program);
    command.args(args).current_dir(repository_root());
    execute_command(command)
}

fn execute_command(mut command: Command) -> Result<(), Error> {
    let display = format!("{command:?}");
    let status = command.status().map_err(|source| Error::Spawn {
        display: display.clone(),
        source,
    })?;
    if status.success() {
        Ok(())
    } else {
        Err(Error::Status {
            display,
            code: status.code(),
        })
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
           dist <OUTPUT>           Build an unsigned distribution archive"
    );
}

#[derive(Debug)]
enum Error {
    Usage(String),
    Spawn {
        display: String,
        source: std::io::Error,
    },
    Status {
        display: String,
        code: Option<i32>,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(message) => write!(formatter, "{message}; run `cargo xtask help`"),
            Self::Spawn { display, source } => {
                write!(formatter, "failed to start {display}: {source}")
            }
            Self::Status { display, code } => match code {
                Some(code) => write!(formatter, "{display} exited with status {code}"),
                None => write!(formatter, "{display} terminated by a signal"),
            },
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
        assert!(matches!(
            run([OsString::from("unknown")]),
            Err(Error::Usage(_))
        ));
    }

    #[test]
    fn incomplete_test_lane_is_a_usage_error() {
        assert!(matches!(
            run([OsString::from("test")]),
            Err(Error::Usage(_))
        ));
    }
}
