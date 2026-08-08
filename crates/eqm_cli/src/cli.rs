//! Closed v1 command-line grammar and usage validation.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

/// Stable usage-error exit code.
pub const USAGE_EXIT: u8 = 2;

/// Parsed global output format.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum OutputFormat {
    /// Deterministic human output.
    #[default]
    Human,
    /// Common JSON result envelope.
    Json,
    /// SARIF 2.1.0 findings document.
    Sarif,
    /// Bounded context Markdown.
    Markdown,
}

/// Parsed color policy.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ColorWhen {
    /// Decorate only an interactive human terminal.
    #[default]
    Auto,
    /// Always decorate human output.
    Always,
    /// Never decorate output.
    Never,
}

/// Global options, independent of workspace content.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GlobalOptions {
    /// Exact configuration path, or deterministic discovery when absent.
    pub config: Option<PathBuf>,
    /// Repeatable profile selections.
    pub profiles: Vec<String>,
    /// Selected output format.
    pub format: OutputFormat,
    /// Whether nonlocal resolution is forbidden.
    pub offline: bool,
    /// Whether progress is suppressed.
    pub no_progress: bool,
    /// Human color policy.
    pub color: ColorWhen,
    /// Exact baseline identity.
    pub baseline: Option<String>,
    /// Explicit output path; `-` normalizes to stdout.
    pub output: Option<PathBuf>,
}

/// One closed command identity.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CommandName {
    Init,
    New,
    Fmt,
    Validate,
    Check,
    Show,
    Locate,
    Context,
    Matrix,
    Obligations,
    Diff,
    Affected,
    Discover,
    Reconcile,
    Verify,
    Attest,
    ReleaseCheck,
    Explain,
    Doctor,
    LockUpdate,
    McpServe,
}

impl CommandName {
    /// Returns the stable machine command identity.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Init => "init",
            Self::New => "new",
            Self::Fmt => "fmt",
            Self::Validate => "validate",
            Self::Check => "check",
            Self::Show => "show",
            Self::Locate => "locate",
            Self::Context => "context",
            Self::Matrix => "matrix",
            Self::Obligations => "obligations",
            Self::Diff => "diff",
            Self::Affected => "affected",
            Self::Discover => "discover",
            Self::Reconcile => "reconcile",
            Self::Verify => "verify",
            Self::Attest => "attest",
            Self::ReleaseCheck => "release_check",
            Self::Explain => "explain",
            Self::Doctor => "doctor",
            Self::LockUpdate => "lock_update",
            Self::McpServe => "mcp_serve",
        }
    }
}

/// Parsed command operands and exact option values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedCommand {
    /// Command identity.
    pub name: CommandName,
    /// Positional operands in declared order.
    pub operands: Vec<String>,
    /// Command options by exact long name; flags contain `None`.
    pub options: BTreeMap<String, Vec<Option<String>>>,
}

/// A fully parsed invocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedCli {
    /// Global options.
    pub global: GlobalOptions,
    /// Selected command.
    pub command: ParsedCommand,
}

/// Parser result that does not require loading a workspace.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseOutcome {
    /// A valid invocation.
    Run(ParsedCli),
    /// Deterministic help was requested.
    Help,
}

/// A stable usage error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsageError(String);

impl Display for UsageError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for UsageError {}

#[derive(Clone, Copy)]
struct OptionSpec {
    name: &'static str,
    takes_value: bool,
    repeatable: bool,
}

const GLOBALS: &[OptionSpec] = &[
    OptionSpec {
        name: "--config",
        takes_value: true,
        repeatable: false,
    },
    OptionSpec {
        name: "--profile",
        takes_value: true,
        repeatable: true,
    },
    OptionSpec {
        name: "--format",
        takes_value: true,
        repeatable: false,
    },
    OptionSpec {
        name: "--offline",
        takes_value: false,
        repeatable: false,
    },
    OptionSpec {
        name: "--no-progress",
        takes_value: false,
        repeatable: false,
    },
    OptionSpec {
        name: "--color",
        takes_value: true,
        repeatable: false,
    },
    OptionSpec {
        name: "--baseline",
        takes_value: true,
        repeatable: false,
    },
    OptionSpec {
        name: "--output",
        takes_value: true,
        repeatable: false,
    },
];

const DRY_RUN: OptionSpec = OptionSpec {
    name: "--dry-run",
    takes_value: false,
    repeatable: false,
};
const TARGET: OptionSpec = OptionSpec {
    name: "--target",
    takes_value: true,
    repeatable: true,
};
const UNIT: OptionSpec = OptionSpec {
    name: "--unit",
    takes_value: true,
    repeatable: false,
};

/// Parses the exact v1 command grammar without touching repository state.
pub fn parse<I, S>(arguments: I) -> Result<ParseOutcome, UsageError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let arguments = arguments
        .into_iter()
        .map(Into::into)
        .collect::<Vec<String>>();
    if arguments
        .iter()
        .any(|value| value == "--help" || value == "-h")
    {
        return if arguments.len() == 1 {
            Ok(ParseOutcome::Help)
        } else {
            Err(usage("--help must be used alone"))
        };
    }
    let mut command_index = 0;
    while command_index < arguments.len() {
        let token = &arguments[command_index];
        if let Some(spec) = GLOBALS.iter().find(|spec| spec.name == token) {
            command_index += if spec.takes_value { 2 } else { 1 };
        } else if token.starts_with('-') {
            return Err(usage(format!("unknown global option `{token}`")));
        } else {
            break;
        }
    }
    if command_index >= arguments.len() {
        return Err(usage("a command is required"));
    }
    let (name, nested, min_operands, max_operands, options): (
        CommandName,
        usize,
        usize,
        Option<usize>,
        Vec<OptionSpec>,
    ) = match arguments[command_index].as_str() {
        "init" => (CommandName::Init, 0, 0, Some(1), vec![DRY_RUN]),
        "new" => (CommandName::New, 0, 2, Some(2), vec![DRY_RUN]),
        "fmt" => (
            CommandName::Fmt,
            0,
            0,
            None,
            vec![
                OptionSpec {
                    name: "--check",
                    takes_value: false,
                    repeatable: false,
                },
                DRY_RUN,
            ],
        ),
        "validate" => (CommandName::Validate, 0, 0, Some(0), vec![]),
        "check" => (
            CommandName::Check,
            0,
            0,
            Some(0),
            vec![
                TARGET,
                OptionSpec {
                    name: "--unit",
                    takes_value: true,
                    repeatable: true,
                },
            ],
        ),
        "show" => (CommandName::Show, 0, 2, Some(2), vec![]),
        "locate" => (
            CommandName::Locate,
            0,
            1,
            Some(1),
            vec![OptionSpec {
                name: "--target",
                takes_value: true,
                repeatable: false,
            }],
        ),
        "context" => (
            CommandName::Context,
            0,
            1,
            Some(1),
            vec![
                OptionSpec {
                    name: "--target",
                    takes_value: true,
                    repeatable: false,
                },
                OptionSpec {
                    name: "--max-bytes",
                    takes_value: true,
                    repeatable: false,
                },
                OptionSpec {
                    name: "--max-depth",
                    takes_value: true,
                    repeatable: false,
                },
            ],
        ),
        "matrix" => (
            CommandName::Matrix,
            0,
            1,
            Some(1),
            vec![
                UNIT,
                OptionSpec {
                    name: "--target",
                    takes_value: true,
                    repeatable: false,
                },
            ],
        ),
        "obligations" => (
            CommandName::Obligations,
            0,
            0,
            Some(0),
            vec![
                UNIT,
                OptionSpec {
                    name: "--target",
                    takes_value: true,
                    repeatable: false,
                },
                OptionSpec {
                    name: "--status",
                    takes_value: true,
                    repeatable: true,
                },
            ],
        ),
        "diff" => (
            CommandName::Diff,
            0,
            0,
            Some(0),
            vec![OptionSpec {
                name: "--candidate",
                takes_value: true,
                repeatable: false,
            }],
        ),
        "affected" => (
            CommandName::Affected,
            0,
            0,
            Some(0),
            vec![OptionSpec {
                name: "--path",
                takes_value: true,
                repeatable: true,
            }],
        ),
        "discover" => (
            CommandName::Discover,
            0,
            0,
            Some(0),
            vec![
                OptionSpec {
                    name: "--adapter",
                    takes_value: true,
                    repeatable: false,
                },
                OptionSpec {
                    name: "--target",
                    takes_value: true,
                    repeatable: false,
                },
            ],
        ),
        "reconcile" => (
            CommandName::Reconcile,
            0,
            0,
            Some(0),
            vec![
                OptionSpec {
                    name: "--target",
                    takes_value: true,
                    repeatable: false,
                },
                UNIT,
                OptionSpec {
                    name: "--inventory",
                    takes_value: true,
                    repeatable: false,
                },
            ],
        ),
        "verify" => (
            CommandName::Verify,
            0,
            0,
            Some(0),
            vec![
                UNIT,
                OptionSpec {
                    name: "--target",
                    takes_value: true,
                    repeatable: false,
                },
                OptionSpec {
                    name: "--affected",
                    takes_value: false,
                    repeatable: false,
                },
                DRY_RUN,
            ],
        ),
        "attest" => (
            CommandName::Attest,
            0,
            0,
            Some(0),
            vec![
                OptionSpec {
                    name: "--evidence",
                    takes_value: true,
                    repeatable: true,
                },
                OptionSpec {
                    name: "--signer",
                    takes_value: true,
                    repeatable: false,
                },
            ],
        ),
        "release" => (
            CommandName::ReleaseCheck,
            1,
            0,
            Some(0),
            vec![OptionSpec {
                name: "--release-record",
                takes_value: true,
                repeatable: false,
            }],
        ),
        "explain" => (CommandName::Explain, 0, 1, Some(1), vec![]),
        "doctor" => (CommandName::Doctor, 0, 0, Some(0), vec![]),
        "lock" => (
            CommandName::LockUpdate,
            1,
            0,
            Some(0),
            vec![
                OptionSpec {
                    name: "--import",
                    takes_value: true,
                    repeatable: true,
                },
                OptionSpec {
                    name: "--adapter",
                    takes_value: true,
                    repeatable: true,
                },
                DRY_RUN,
            ],
        ),
        "mcp" => (
            CommandName::McpServe,
            1,
            0,
            Some(0),
            vec![
                OptionSpec {
                    name: "--allow-verify",
                    takes_value: false,
                    repeatable: false,
                },
                OptionSpec {
                    name: "--audit-output",
                    takes_value: true,
                    repeatable: false,
                },
            ],
        ),
        value => return Err(usage(format!("unknown command `{value}`"))),
    };
    let mut global_values = BTreeMap::<String, Vec<Option<String>>>::new();
    let mut command_values = BTreeMap::<String, Vec<Option<String>>>::new();
    let mut operands = Vec::new();
    let mut index = 0;
    while index < arguments.len() {
        if index == command_index {
            index += 1;
            continue;
        }
        if nested == 1 && index == command_index + 1 {
            let required = match name {
                CommandName::ReleaseCheck => "check",
                CommandName::LockUpdate => "update",
                CommandName::McpServe => "serve",
                _ => return Err(usage("invalid nested command parser state")),
            };
            if arguments[index] != required {
                return Err(usage(format!(
                    "expected `{}` after `{}`",
                    required, arguments[command_index]
                )));
            }
            index += 1;
            continue;
        }
        let token = &arguments[index];
        if token.starts_with('-') {
            let (target, spec) = if let Some(spec) = GLOBALS.iter().find(|spec| spec.name == token)
            {
                (&mut global_values, *spec)
            } else if let Some(spec) = options.iter().find(|spec| spec.name == token) {
                (&mut command_values, *spec)
            } else {
                return Err(usage(format!(
                    "unsupported option `{token}` for {}",
                    name.as_str()
                )));
            };
            if !spec.repeatable && target.contains_key(spec.name) {
                return Err(usage(format!("option `{}` may not be repeated", spec.name)));
            }
            let value = if spec.takes_value {
                index += 1;
                let value = arguments
                    .get(index)
                    .ok_or_else(|| usage(format!("option `{}` requires a value", spec.name)))?;
                if value.is_empty()
                    || (value.starts_with('-') && !(spec.name == "--output" && value == "-"))
                {
                    return Err(usage(format!(
                        "option `{}` requires a nonempty value",
                        spec.name
                    )));
                }
                Some(value.clone())
            } else {
                None
            };
            target.entry(spec.name.to_owned()).or_default().push(value);
        } else if index < command_index {
            return Err(usage(format!(
                "unexpected operand `{token}` before command"
            )));
        } else {
            operands.push(token.clone());
        }
        index += 1;
    }
    if nested == 1 && command_index + 1 >= arguments.len() {
        return Err(usage("nested command is required"));
    }
    if operands.len() < min_operands || max_operands.is_some_and(|maximum| operands.len() > maximum)
    {
        return Err(usage(format!(
            "invalid operand count for {}",
            name.as_str()
        )));
    }
    validate_command(name, &operands, &command_values, &global_values)?;
    let global = build_globals(global_values)?;
    validate_format(name, global.format)?;
    Ok(ParseOutcome::Run(ParsedCli {
        global,
        command: ParsedCommand {
            name,
            operands,
            options: command_values,
        },
    }))
}

fn validate_format(name: CommandName, format: OutputFormat) -> Result<(), UsageError> {
    match format {
        OutputFormat::Markdown if name != CommandName::Context => {
            Err(usage("markdown format is supported only by context"))
        }
        OutputFormat::Sarif
            if !matches!(
                name,
                CommandName::Validate
                    | CommandName::Check
                    | CommandName::Verify
                    | CommandName::ReleaseCheck
                    | CommandName::Doctor
            ) =>
        {
            Err(usage(format!(
                "sarif format is unsupported for {}",
                name.as_str()
            )))
        }
        _ => Ok(()),
    }
}

fn validate_command(
    name: CommandName,
    operands: &[String],
    options: &BTreeMap<String, Vec<Option<String>>>,
    globals: &BTreeMap<String, Vec<Option<String>>>,
) -> Result<(), UsageError> {
    let required = match name {
        CommandName::Discover => &["--adapter", "--target"][..],
        CommandName::Reconcile => &["--target"][..],
        CommandName::ReleaseCheck => &["--release-record"][..],
        CommandName::Diff | CommandName::Affected => &["--baseline"][..],
        _ => &[],
    };
    for option in required {
        if !options.contains_key(*option) && !globals.contains_key(*option) {
            return Err(usage(format!("{} requires `{option}`", name.as_str())));
        }
    }
    if globals.contains_key("--baseline")
        && !matches!(
            name,
            CommandName::Diff | CommandName::Affected | CommandName::Verify
        )
    {
        return Err(usage(format!(
            "--baseline is unsupported for {}",
            name.as_str()
        )));
    }
    if options.contains_key("--check") && options.contains_key("--dry-run") {
        return Err(usage("fmt --check cannot be combined with --dry-run"));
    }
    if name == CommandName::McpServe
        && options.contains_key("--allow-verify") != options.contains_key("--audit-output")
    {
        return Err(usage(
            "mcp serve requires --allow-verify and --audit-output together",
        ));
    }
    if name == CommandName::New
        && !matches!(
            operands[0].as_str(),
            "capability"
                | "journey"
                | "surface"
                | "fragment"
                | "binding"
                | "policy"
                | "profile"
                | "runner"
                | "waiver"
        )
    {
        return Err(usage("invalid new kind"));
    }
    if name == CommandName::Show
        && !matches!(
            operands[0].as_str(),
            "capability"
                | "journey"
                | "surface"
                | "fragment"
                | "binding"
                | "policy"
                | "profile"
                | "runner"
                | "waiver"
                | "target"
        )
    {
        return Err(usage("invalid show kind"));
    }
    if name == CommandName::Matrix
        && !matches!(
            operands[0].as_str(),
            "conformance" | "evidence" | "exposure" | "release" | "equivalence"
        )
    {
        return Err(usage("invalid matrix kind"));
    }
    for (option, minimum, maximum) in [
        ("--max-bytes", 1_024u64, 1_048_576u64),
        ("--max-depth", 1, 16),
    ] {
        if let Some(value) = one(options, option) {
            let number = value
                .parse::<u64>()
                .map_err(|_| usage(format!("`{option}` must be an integer")))?;
            if number < minimum || number > maximum {
                return Err(usage(format!("`{option}` is out of range")));
            }
        }
    }
    Ok(())
}

fn build_globals(
    values: BTreeMap<String, Vec<Option<String>>>,
) -> Result<GlobalOptions, UsageError> {
    let profiles = many(&values, "--profile");
    for profile in &profiles {
        let mut dimensions = BTreeSet::new();
        if let Some((id, assignments)) = profile.split_once('=') {
            if id.is_empty() || assignments.is_empty() {
                return Err(usage("profile selection is empty"));
            }
            for assignment in assignments.split(',') {
                let (dimension, value) = assignment
                    .split_once(':')
                    .ok_or_else(|| usage("profile values use dimension:value"))?;
                if dimension.is_empty() || value.is_empty() || !dimensions.insert(dimension) {
                    return Err(usage("profile dimensions must be nonempty and unique"));
                }
            }
        }
    }
    let format = match one(&values, "--format").unwrap_or("human") {
        "human" => OutputFormat::Human,
        "json" => OutputFormat::Json,
        "sarif" => OutputFormat::Sarif,
        "markdown" => OutputFormat::Markdown,
        _ => return Err(usage("invalid --format")),
    };
    let color = match one(&values, "--color").unwrap_or("auto") {
        "auto" => ColorWhen::Auto,
        "always" => ColorWhen::Always,
        "never" => ColorWhen::Never,
        _ => return Err(usage("invalid --color")),
    };
    Ok(GlobalOptions {
        config: one(&values, "--config").map(PathBuf::from),
        profiles,
        format,
        offline: values.contains_key("--offline"),
        no_progress: values.contains_key("--no-progress"),
        color,
        baseline: one(&values, "--baseline").map(str::to_owned),
        output: one(&values, "--output")
            .filter(|value| *value != "-")
            .map(PathBuf::from),
    })
}

fn one<'a>(values: &'a BTreeMap<String, Vec<Option<String>>>, name: &str) -> Option<&'a str> {
    values.get(name)?.first()?.as_deref()
}
fn many(values: &BTreeMap<String, Vec<Option<String>>>, name: &str) -> Vec<String> {
    values
        .get(name)
        .into_iter()
        .flatten()
        .filter_map(Clone::clone)
        .collect()
}
fn usage(message: impl Into<String>) -> UsageError {
    UsageError(message.into())
}

/// Returns deterministic top-level help.
pub const fn help() -> &'static str {
    "EquivalenceMatrix\n\nUsage: eqm [GLOBAL OPTIONS] <COMMAND> [ARGS] [OPTIONS]\n\nCommands:\n  init  new  fmt  validate  check  show  locate  context\n  matrix  obligations  diff  affected  discover  reconcile\n  verify  attest  release check  explain  doctor  lock update  mcp serve\n\nGlobal options:\n  --config <PATH>  --profile <ID[=VALUES]>  --format <FORMAT>\n  --offline  --no-progress  --color <WHEN>  --baseline <IDENTITY>\n  --output <PATH>\n"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_command_and_nested_identity_parses() -> Result<(), UsageError> {
        let cases = [
            (vec!["init"], CommandName::Init),
            (vec!["new", "surface", "signup"], CommandName::New),
            (vec!["fmt"], CommandName::Fmt),
            (vec!["validate"], CommandName::Validate),
            (vec!["check"], CommandName::Check),
            (vec!["show", "target", "web"], CommandName::Show),
            (vec!["locate", "signup"], CommandName::Locate),
            (vec!["context", "signup"], CommandName::Context),
            (vec!["matrix", "conformance"], CommandName::Matrix),
            (vec!["obligations"], CommandName::Obligations),
            (
                vec![
                    "--baseline",
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                    "diff",
                ],
                CommandName::Diff,
            ),
            (
                vec![
                    "affected",
                    "--baseline",
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                ],
                CommandName::Affected,
            ),
            (
                vec!["discover", "--adapter", "adapter.test", "--target", "web"],
                CommandName::Discover,
            ),
            (vec!["reconcile", "--target", "web"], CommandName::Reconcile),
            (vec!["verify"], CommandName::Verify),
            (vec!["attest"], CommandName::Attest),
            (
                vec!["release", "check", "--release-record", "record.json"],
                CommandName::ReleaseCheck,
            ),
            (vec!["explain", "EQM-E0300"], CommandName::Explain),
            (vec!["doctor"], CommandName::Doctor),
            (vec!["lock", "update"], CommandName::LockUpdate),
            (vec!["mcp", "serve"], CommandName::McpServe),
        ];
        for (arguments, expected) in cases {
            let ParseOutcome::Run(parsed) = parse(arguments)? else {
                return Err(usage("unexpected help"));
            };
            assert_eq!(parsed.command.name, expected);
        }
        Ok(())
    }

    #[test]
    fn usage_errors_precede_session_preparation() {
        for arguments in [
            vec!["validate", "--config", ""],
            vec!["fmt", "--check", "--dry-run"],
            vec!["context", "unit", "--max-bytes", "100"],
            vec!["discover", "--target", "web"],
            vec!["validate", "--baseline", "abc"],
            vec!["new", "unknown", "id"],
            vec!["--profile", "release=cohort:a,cohort:b", "validate"],
            vec!["mcp", "serve", "--allow-verify"],
            vec!["mcp", "serve", "--audit-output", "audit.jsonl"],
        ] {
            assert!(parse(arguments).is_err());
        }
    }

    #[test]
    fn global_options_work_before_and_after_commands() -> Result<(), UsageError> {
        let ParseOutcome::Run(parsed) = parse([
            "--offline",
            "context",
            "signup",
            "--format",
            "markdown",
            "--output",
            "-",
        ])?
        else {
            return Err(usage("unexpected help"));
        };
        assert!(parsed.global.offline);
        assert_eq!(parsed.global.format, OutputFormat::Markdown);
        assert_eq!(parsed.global.output, None);
        Ok(())
    }
}
