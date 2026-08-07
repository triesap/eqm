# EquivalenceMatrix V1 RCLD 09: Verification And Mutation

Status: in progress; RCLD 08 complete
Created: 2026-08-07
Updated: 2026-08-07
Mode: rcl-durable
Repository: `triesap/eqm`
Governing plan: `docs/execution/rcl/eqm_v1_multi_rcld.md`
Depends on: RCLD 08
Current checkpoint: `step_104`

## Purpose

Complete the CLI with approved evidence execution, attestation, release checks,
diagnostic/environment inspection, authored scaffolding/formatting, and explicit
lock updates under the approved mutability model.

## Scope Boundary

`verify` writes only generated immutable evidence. `attest`, `release check`,
`explain`, and `doctor` are read-only unless an explicit output destination is
provided. `init`, `new`, `fmt`, and `lock update` are the only authored-file
mutators. No command publishes, signs with an undeclared identity, deploys,
creates waivers, or edits target application code.

## Definition Of Green

- Evidence execution is explicit, approved, bounded, and immutable.
- Attestations bind exact subjects and digests and do not overstate trust.
- Release checks use exact release subjects and explicit release profile.
- Every authored mutation supports dry-run and atomic writes.
- Lock update is the only normal command allowed to acquire remote pins.
- Doctor performs no untrusted execution.
- Every command preserves machine-output and exit-code contracts.

## Verification Lane

Run the standard locked workspace lane plus fake runner/CI trust integration,
attestation goldens, release end-to-end fixtures, dry-run/atomic-write/failure
tests, formatter idempotence, lock update offline/network fixtures, and
workspace cleanliness assertions.

## Checkpoint Map

| Step | Scope | Definition of green | Commit intent |
| --- | --- | --- | --- |
| `step_104` | Implement explicit selected/affected evidence `verify` and immutable `.eqm/results/` writes. | Fake runner success/failure/timeout/trust cases produce exact results and exit codes; authored files stay unchanged. | `feat(cli): implement verify` |
| `step_105` | Implement EQM predicate and in-toto statement emission from validated evidence. | Golden payload binds exact subject/digests; unsigned output is not labeled signed; explicit output is atomic. | `feat(cli): implement attest` |
| `step_106` | Implement exact-subject `release check` over evidence, facts, release records, policy, trust, freshness, and waivers. | Passing/failing/conditional/unknown end-to-end fixtures and code 7 trust failures pass. | `feat(cli): implement release check` |
| `step_107` | Implement stable diagnostic `explain`. | Every registered code is explainable; unknown code returns usage/not-found behavior defined by authority. | `feat(cli): implement explain` |
| `step_108` | Implement non-executing `doctor` for toolchain, workspace, config, generated-state policy, pins, and no-legacy readiness. | Spies prove no runner/adapter execution; healthy and degraded environment fixtures classify deterministically. | `feat(cli): implement doctor` |
| `step_109` | Implement current-schema `init` and `new` with dry-run, collision checks, and atomic writes. | Tempdir scaffolds are valid, idempotent where specified, rollback-safe, and contain no placeholder digest or legacy value. | `feat(cli): implement init and new` |
| `step_110` | Implement `fmt` and `fmt --check` over the comment-preserving backend. | Idempotence, dry-run/check, partial-failure rollback, explicit config, and stable file ordering pass. | `feat(cli): implement fmt` |
| `step_111` | Implement explicit `lock update` for local/exact imports and pinned adapters with offline behavior. | Dry-run, atomic replace, exact digest, floating-ref rejection, offline failure, and deterministic lock ordering pass. | `feat(cli): implement lock update` |

## Mutability Matrix

| Command | Authored state | Generated state | Remote acquisition |
| --- | --- | --- | --- |
| `verify` | never | `.eqm/results/` | never |
| `attest` | never | explicit output only | never |
| `release check` | never | explicit output only | never |
| `explain` | never | explicit output only | never |
| `doctor` | never | explicit output only | never |
| `init` / `new` | explicit, dry-run, atomic | no implicit state | never |
| `fmt` | explicit, dry-run/check, atomic | no implicit state | never |
| `lock update` | `eqm.lock`, dry-run, atomic | temporary confined state | explicit allowed operation |

## Reconciliation Rules

- A command that writes outside this matrix is a blocking contract violation.
- Real signing identities and production roots remain organizational inputs;
  fixtures use synthetic test roots labeled as such.
- RCLD 10 begins only after all mutation rollback and clean-worktree tests are
  green.

## Checkpoint Ledger

| Step range | Status | Result |
| --- | --- | --- |
| `step_104`-`step_111` | pending | - |
