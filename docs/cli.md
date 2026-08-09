# CLI reference

## Global options

Global options may appear before or after the command. Important options are
`--config PATH`, repeatable `--profile SELECTION`, `--format
human|json|sarif|markdown`, `--offline`, `--no-progress`, `--color
auto|always|never`, `--baseline ID`, and `--output PATH`. `--output -` means
stdout. Machine formats emit one deterministic document on stdout; progress
and diagnostics belong on stderr.

## Authoring commands

| Command | Purpose |
| --- | --- |
| `init [PATH] [--dry-run]` | Create the initial workspace files |
| `new KIND ID [--dry-run]` | Create one current-schema authored document |
| `fmt [PATH ...] [--check|--dry-run]` | Deterministically format manifests |
| `lock update [--import PIN] [--adapter PIN] [--dry-run]` | Review or write exact pins |

Only these commands mutate authored EQM metadata, and dry-run modes do not
write. Collision checks and writes are atomic.

## Read and analysis commands

| Command | Purpose |
| --- | --- |
| `validate` | Load, resolve, expand, and validate the semantic graph |
| `check [--target ID] [--unit ID]` | Evaluate prepared evidence without execution |
| `show KIND ID` | Return one exact typed entity and source |
| `locate UNIT [--target ID]` | Locate bound source, artifacts, and evidence |
| `context UNIT [--target ID] [--max-bytes N] [--max-depth N]` | Return bounded trust-labeled context |
| `matrix KIND [--unit ID] [--target ID]` | Render a deterministic matrix view |
| `obligations [--unit ID] [--target ID] [--status STATUS]` | List exact evidence obligations |
| `diff [--candidate ID]` | Compare exact baseline and candidate graphs |
| `affected [--path PATH]` | Derive conservative change impact |
| `explain EQM-CODE` | Explain a stable diagnostic and remediation |
| `doctor` | Inspect configuration, pins, names, generated state, and toolchain |

`--baseline` accepts exact authority, not a floating branch. An unmapped changed
path causes conservative expansion rather than a falsely narrow result.

## Observation and execution

| Command | Purpose |
| --- | --- |
| `discover --adapter ID --target ID` | Invoke one exact pinned inventory adapter |
| `reconcile --target ID [--unit ID] [--inventory PATH]` | Compare inventory with authored exposure |
| `verify [--unit ID] [--target ID] [--affected] [--dry-run]` | Plan or execute bounded evidence runners |
| `attest [--evidence PATH] [--signer PATH]` | Build an in-toto statement; signing is explicit |
| `release check --release-record PATH` | Evaluate one exact release subject |
| `mcp serve [--allow-verify --audit-output PATH]` | Serve the bounded MCP surface over stdio |

`discover`, non-dry-run `verify`, and an explicitly enabled MCP verify tool are
execution boundaries. Treat them as requiring separate authority. `attest`
without a signer remains explicitly unsigned.

## Output and exit behavior

JSON commands use the committed protocol schemas under `schemas/v1/protocol/`.
Consumers must preserve result categories instead of treating every nonzero
exit identically. The stable high-level meanings are success, conformance or
validation failure, usage error, unavailable/invalid external execution, and
release unknown. In particular, release pass, fail, and unknown are distinct;
inspect the structured envelope rather than guessing from prose.

Diagnostics have stable `EQM-` codes, severity, message, optional source and
related locations, remediation, and a canonical documentation path. Use `eqm
explain EQM-E0300` for a machine-accessible descriptor.
