# EquivalenceMatrix V1 RCLD 08: CLI Query Surface

Status: in progress; Step 101 complete
Created: 2026-08-07
Updated: 2026-08-07
Mode: rcl-durable
Repository: `triesap/eqm`
Governing plan: `docs/execution/rcl/eqm_v1_multi_rcld.md`
Depends on: RCLD 07
Current checkpoint: `step_102`

## Purpose

Implement the stable CLI parser, prepared workspace session, output discipline,
and all validation/query/discovery/reconciliation commands that do not execute
evidence runners or mutate authored metadata.

## Scope Boundary

This RCLD may run explicitly approved adapters only through `discover`.
`validate`, `check`, `show`, `locate`, `context`, `matrix`, `obligations`,
`diff`, `affected`, and `reconcile` do not execute runners or adapters.
`discover` returns data without implicit persistence. Authored mutations,
evidence execution, attestations, and release checks remain RCLD 09.

## Definition Of Green

- The exact approved command and global-option surface appears in help.
- CLI orchestration prepares one validated/resolved/canonical workspace session
  reused across commands.
- Machine stdout is exactly one selected JSON, SARIF, or Markdown document;
  logs and progress use stderr.
- Human output is deterministic apart from explicitly TTY-only decoration.
- Exit-code precedence follows the public contract.
- Context is bounded and labels untrusted data separately from procedural
  authority.
- Read-only commands do not dirty the workspace.

## Verification Lane

Run the standard locked workspace lane plus command help snapshots, machine
stdout/stderr isolation, exit-code tables, temp-workspace integration tests,
output goldens, no-execution assertions, and clean-worktree checks.

## Checkpoint Map

| Step | Scope | Definition of green | Commit intent |
| --- | --- | --- | --- |
| `step_091` | Define CLI parser, global options, subcommands, prepared session boundary, and help. | Command/option names and nesting match authority; invalid usage returns code 2; help snapshots pass. | `feat(cli): implement command skeleton` |
| `step_092` | Implement human/JSON/SARIF/Markdown rendering, atomic explicit output, stderr logging, progress, and color policy. | Machine stdout contains one document and no log bytes; deterministic goldens and TTY behavior pass. | `feat(cli): implement output renderer` |
| `step_093` | Implement `validate` through load, resolve, invariants, fragment expansion, and final workspace digest. | Valid/invalid workspaces return exact envelopes and exit codes 0/1/3 without execution or writes. | `feat(cli): implement validate` |
| `step_094` | Implement non-executing `check` for structure, policy, and obligations. | Runner/adapter spies prove no execution; findings and exit codes match fixtures. | `feat(cli): implement check` |
| `step_095` | Implement `show` for all approved entity kinds. | Exact/not-found/ambiguous and deterministic output goldens pass. | `feat(cli): implement show` |
| `step_096` | Implement `locate` for source, artifacts, and evidence declarations by unit/target. | Signup and error fixtures return repository-relative, source-located results. | `feat(cli): implement locate` |
| `step_097` | Implement bounded `context` with filters, max bytes/depth, authority, trust, source, obligations, evidence, findings, and waivers. | Bounds are hard, truncation is explicit, untrusted prose is labeled, and goldens pass. | `feat(cli): implement context` |
| `step_098` | Implement conformance, evidence, exposure, release, and equivalence `matrix`. | All matrix modes and invalid mode/profile cases match stable goldens. | `feat(cli): implement matrix` |
| `step_099` | Implement unresolved obligation reporting. | Missing/stale/failed/unstable/waived/unknown filters and stable ordering match goldens. | `feat(cli): implement obligations` |
| `step_100` | Implement exact baseline/candidate semantic `diff`. | Ambiguous/floating baseline is rejected; classified diff and unchanged cases match fixtures. | `feat(cli): implement diff` |
| `step_101` | Implement Git changed-file acquisition plus conservative affected analysis. | Temporary Git fixtures, dirty patch handling, unmapped changes, and explicit baseline tests pass. | `feat(cli): implement affected` |
| `step_102` | Implement explicit approved-adapter `discover` with pin/offline enforcement and no implicit persistence. | Fake adapter CLI tests, offline/pin failures, exact output, and clean-worktree assertions pass. | `feat(cli): implement discover` |
| `step_103` | Implement read-only declaration/inventory `reconcile` without implicit discovery. | Goldens preserve unknown/completeness and adapter spy proves no execution. | `feat(cli): implement reconcile` |

## Output And Exit Rules

- Explicit `--output` uses atomic replace and never duplicates machine stdout.
- Usage errors precede loading; manifest/graph errors precede evaluation;
  adapter/runner/trust failures use their dedicated categories exactly as
  specified.
- `--offline` forbids remote resolution and acquisition but does not disable
  explicitly pinned local adapters unless the command contract says so.

## Reconciliation Rules

- Commands share orchestration rather than duplicating load/evaluation logic.
- Any new command or alias requires new authority and is outside this RCLD.
- RCLD 09 begins only after every RCLD 08 command leaves its fixture workspace
  clean unless explicit output was requested.

## Checkpoint Ledger

| Step range | Status | Result |
| --- | --- | --- |
| `step_091` | complete | The executable has a closed table-driven parser for every approved global option, command, nested command, operand count, command option, required option, enum, profile selection, and context bound; usage is validated before session preparation, deterministic help covers the exact surface, invalid use exits 2, and one immutable session request boundary captures later workspace orchestration inputs |
| `step_092` | complete | One renderer selects deterministic human, compact JSON, SARIF 2.1.0, or bounded Markdown bytes with exactly one terminal newline; command/format compatibility fails during usage parsing, machine formats disable color, progress and escaped logs are stderr-only, `--no-progress` preserves diagnostics, and explicit output durably atomically replaces a nonsymlink destination while stdout remains empty |
| `step_093` | complete | `validate` performs deterministic config selection, strict workspace discovery/decode/conversion/lock binding, graph resolution, invariant validation, exact fragment-digest preparation and expansion, and canonical semantic graph hashing in one immutable session; success emits a typed current result envelope with stable entity counts and digest, preparation failure emits a manifest diagnostic and exit 3, and neither path executes or writes workspace state |
| `step_094` | complete | `check` reuses one finalized session, confines direct filesystem inspection to declared artifacts, evaluates typed structure without process execution, selects the exact development policy/profile defaults, derives applicability-aware obligations, reports missing evidence and structural failures through registered diagnostics and typed counts/findings, applies unit/target filters, returns blocking exit 1, and leaves generated state unchanged |
| `step_095` | complete | `show` resolves every approved entity kind from one finalized graph, emits the exact canonical semantic projection plus its repository-relative authority source, rejects missing or multi-revision matches with registered `EQM-E0001` and exit 2, and returns deterministic typed envelopes without execution or writes |
| `step_096` | complete | `locate` resolves an exact unit and optional target across binding authority, returns deterministically sorted source, artifact, and evidence declaration locations with repository-relative paths and manifest source spans, and reports missing coordinates through typed `EQM-E0001` query failure without execution or writes |
| `step_097` | complete | `context` combines exact graph authority, complete profile-aware obligations, binding product paths, evidence declarations, findings, and relevant waivers under explicit provenance/trust labels; depth and byte bounds truncate only complete records with visible omission accounting, Markdown remains bounded, and no runner or adapter executes |
| `step_098` | complete | All five matrix modes use complete stable typed axes and Cartesian cells; conformance and evidence views expose current missing obligations and diagnostics while exposure, release, and equivalence preserve explicit unknown state until trusted observations exist, and exact unit/target/profile filters apply without execution |
| `step_099` | complete | `obligations` reports the current complete missing-evidence set as stable typed coordinates with exact strength, policy, profile, unit, requirement, scope subject, and facet fields; unit, target, and every closed status filter are deterministic, with statuses absent from current prepared evidence returning an explicit empty result |
| `step_100` | complete | `diff` prepares exact repository paths, available semantic digests, or full immutable local commit objects, rejects floating or unavailable identities, projects every finalized semantic authority into classified fields, and returns stable directional changes or an exact unchanged result without evaluation or execution |
| `step_101` | complete | `affected` acquires tracked, staged, unstaged, and untracked paths against an exact full local commit or accepts explicit normalized paths; finalized artifact, target, transition, unit, and obligation reverse indexes drive precise expansion while unmapped files and unclassified semantic changes conservatively select the complete affected set |
| `step_102`-`step_103` | pending | - |
