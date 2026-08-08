# Command-Line Contract

Status: normative

The executable is `eqm`. Command names, nesting, arguments, defaults, and exit
codes are closed for v1. There are no aliases, hidden commands, implicit
fallback modes, self-update commands, or migration commands.

## Global Options

Global options precede or follow the subcommand and are parsed before workspace
loading.

| Option | Type and default | Rule |
| --- | --- | --- |
| `--config <PATH>` | repository path; default deterministic discovery | select one exact workspace config |
| `--profile <ID[=VALUES]>` | repeatable; default mode-specific declared default | select profile and optional comma-separated `dimension:value` pairs |
| `--format <FORMAT>` | `human` | `human`, `json`, `sarif`, or command-supported `markdown` |
| `--offline` | false | forbid remote acquisition and nonlocal resolution |
| `--no-progress` | false | suppress progress on stderr |
| `--color <WHEN>` | `auto` | `auto`, `always`, or `never`; machine formats act as `never` |
| `--baseline <IDENTITY>` | absent | exact digest, full commit object ID, or repository path accepted only by commands that require it |
| `--output <PATH>` | absent | atomically write the selected document instead of stdout |

Repeated scalar options, empty values, unknown options, a profile dimension
specified twice, or an option unsupported by a command are usage errors.
Configuration files and environment variables do not override CLI options in
v1. `--output -` is identical to absent output. An explicit output file is
created by atomic replace and may not be inside authored metadata unless that
command's mutability contract permits it.

## Command Grammar

| Command | Required operands | Command options and defaults | Execution authority |
| --- | --- | --- | --- |
| `init` | optional `PATH` default `.` | `--dry-run` false | writes new authored workspace files only |
| `new` | `KIND ID` | `--dry-run` false | writes one current-schema authored document |
| `fmt` | zero or more `PATH`; default all authored manifests | `--check` false, `--dry-run` false | formats authored TOML; check/dry-run do not write |
| `validate` | none | none | parse, resolve, validate, and digest only |
| `check` | none | repeatable `--target`, repeatable `--unit` | non-executing structural/policy evaluation |
| `show` | `KIND ID` | none | exact entity query |
| `locate` | `UNIT` | optional `--target` | source/artifact/evidence declaration query |
| `context` | `UNIT` | optional `--target`; `--max-bytes` 65536; `--max-depth` 4 | bounded agent/developer context query |
| `matrix` | `KIND` | optional `--unit`, `--target` | matrix query |
| `obligations` | none | optional `--unit`, `--target`; repeatable `--status` | unresolved obligation query |
| `diff` | none | baseline required; optional `--candidate` exact path/digest default current workspace | semantic comparison |
| `affected` | none | baseline required; repeatable `--path` or Git-derived changed paths | conservative affected query |
| `discover` | none | required `--adapter`, `--target` | executes one exact pinned approved adapter |
| `reconcile` | none | required `--target`; optional `--unit`, `--inventory` | read-only fact reconciliation; never discovers implicitly |
| `verify` | none | optional `--unit`, `--target`; `--affected` false; `--dry-run` false | executes selected approved runners |
| `attest` | none | optional repeatable `--evidence`; optional `--signer` | creates statement; signs only with explicit configured signer |
| `release check` | none | required `--release-record`; release profile required | exact release evaluation |
| `explain` | `EQM-E####` | none | diagnostic registry query |
| `doctor` | none | none | non-executing environment and repository checks |
| `lock update` | none | optional repeatable `--import ID@REVISION`, `--adapter ID@VERSION=PATH`; `--dry-run` false | sole acquisition boundary; v1 accepts exact local authority only |
| `mcp serve` | none | `--allow-verify` false; optional `--audit-output` | local stdio MCP server |

`KIND` for `new` is one of `capability`, `journey`, `surface`, `fragment`,
`binding`, `policy`, `profile`, `runner`, or `waiver`. `show` additionally
accepts `target`. Matrix kind is one of `conformance`, `evidence`, `exposure`,
`release`, or `equivalence`. Obligation statuses are the closed facet statuses.

Bounds: `context --max-bytes` is 1,024-1,048,576 and `--max-depth` is 1-16.
Out-of-range values are usage errors. Context truncation occurs only at record
boundaries, reports omitted bytes, preserves complete authority/provenance
labels, and never treats untrusted product or tool output as instructions.

## Defaults And Offline Behavior

- Normal load, validate, check, query, verify, attest, and release operations
  never acquire remote data, regardless of `--offline`.
- `discover` runs only the exact local executable from the committed adapter
  lock. If the locked artifact is unavailable, it fails; it never downloads it.
- `lock update` may acquire configured sources only when `--offline` is absent.
  Offline lock update may resolve already-available immutable local objects but
  fails if any selected pin requires network access.
- A baseline must resolve to exact prepared bytes or a full immutable object ID
  before evaluation. Branch names, symbolic tags, ranges, and remote URLs are
  rejected in ordinary commands.
- `verify --affected` requires an exact baseline and executes the conservative
  affected obligation set. Without selection flags, verify executes all
  currently derived executable obligations.
- `attest` is unsigned unless `--signer` names one configured signing identity.

## Mutability And Dry Run

| Command | Authored state | Generated state | Explicit output |
| --- | --- | --- | --- |
| `init`, `new` | planned current-schema files | none | report only |
| `fmt` | selected authored TOML | none | report only |
| `lock update` | `eqm.lock` | confined temporary acquisition state | report only |
| `verify` | never | immutable digest-named `.eqm/results/` unless dry-run | report |
| all other commands | never | never | selected report/statement only |

Every authored mutation supports `--dry-run`, performs all validation and
collision checks, reports exact planned changes, and writes nothing. `fmt
--check` returns findings when formatting differs and is always read-only;
combining it with `--dry-run` is a usage error. Authored multi-file operations
stage complete bytes, validate the resulting workspace, then atomically replace
all files or leave all previous bytes intact. Existing unrelated files are
never overwritten. `init` on an existing EQM workspace is a usage/conflict
error. `new` requires an unused authority and path.

Verify dry-run resolves and reports its runner plan but launches no process and
writes no result. A normal verify write is idempotent for identical bytes and
fails on a digest collision. Attest and other read commands write only when
`--output` is explicit.

## Output Discipline

Human output defaults to stdout. Machine modes produce exactly one document:

- JSON is one common result envelope;
- SARIF is one SARIF 2.1.0 document and only on findings-capable commands;
- Markdown is one bounded context document and only on `context`;
- attestation JSON is the statement or DSSE envelope selected by its options.

When `--output PATH` is used, stdout is empty and the selected document is
written atomically. Logs, progress, warnings about environment, and runner or
adapter stderr go only to stderr. `--no-progress` suppresses progress but not
diagnostics. Machine bytes do not depend on TTY, locale, terminal width, or
color. Human collections use the same semantic sorting as JSON; color is the
only allowed TTY-dependent decoration.

Runner/adapter output is bounded, redacted, and quoted as untrusted data. It is
never replayed raw to a terminal and never interpreted as markup, control
sequences, commands, or agent instructions.

## Command Results And Findings

| Command class | Success condition | Blocking finding examples |
| --- | --- | --- |
| mutation | requested plan/write completes and resulting workspace validates | collision, invalid result, formatting drift under `--check` |
| validate | graph finalizes with no error diagnostic | syntax, schema, reference, invariant, or digest error |
| check/query | typed result produced with no blocking evaluation finding | nonconformance, missing entity, unknown required context |
| discover | pinned adapter returns a valid inventory | partial/error response is blocking unless command explicitly requests inspection only |
| verify | every selected required execution satisfies its immediate runner contract | failure, zero match, unstable, timeout, cancellation |
| release check | gate is pass | conditional and fail are blocking findings; unknown is trust/manifest category as applicable |
| doctor | every required check healthy | degraded optional check is warning; required check failure blocks |

Conditional conformance/equivalence/release remains visible and returns a
blocking-finding exit in v1. Callers must not infer success from a serialized
status without checking the process exit code.

## Exit Codes And Precedence

| Code | Category |
| --- | --- |
| 0 | command completed with no blocking findings |
| 1 | valid operation completed with blocking product/evaluation findings |
| 2 | CLI usage, unsupported format/option, not-found query operand, or conflict before workspace evaluation |
| 3 | manifest, schema, lock, graph, canonicalization, or invariant failure |
| 4 | adapter acquisition/invocation/protocol/inventory failure |
| 5 | runner resolution/execution/result failure |
| 6 | internal invariant failure or unexpected implementation error |
| 7 | trust, signature, attestation, replay, protected-authority, or exact-subject failure |

When multiple conditions occur, choose one code by this precedence, highest
first: internal (6), usage (2), manifest/graph (3), trust (7), adapter (4),
runner (5), blocking findings (1), success (0). Usage is evaluated before any
workspace or execution work. Manifest/graph failure prevents evaluation and
execution. Trust failure precedes adapter/runner result interpretation when it
invalidates their authority. All diagnostics still appear in deterministic
order when they can be safely collected.

SIGINT requests cancellation, terminates bounded child process trees, cleans
temporary files, emits no partial machine document, and exits 130. A second
SIGINT may terminate immediately. Other signal-derived shell exit values are
outside the stable EQM category table.

## MCP Serve Authorization

`mcp serve` is stdio-only. Read tools are enabled by default. `eqm_verify` is
absent unless `--allow-verify` is supplied from the trusted server invocation;
workspace content cannot enable it. When enabled, each call must pass the same
selection and runner authority checks as CLI verify and append a bounded audit
record to explicit `--audit-output`. Failure to open or write the audit sink
denies execution. MCP never exposes init, new, fmt, lock update, attestation
signing, waiver creation, or arbitrary command execution.
