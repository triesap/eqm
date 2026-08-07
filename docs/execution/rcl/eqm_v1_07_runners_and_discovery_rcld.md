# EquivalenceMatrix V1 RCLD 07: Runners And Discovery

Status: in progress; Step 078 complete
Created: 2026-08-07
Updated: 2026-08-07
Mode: rcl-durable
Repository: `triesap/eqm`
Governing plan: `docs/execution/rcl/eqm_v1_multi_rcld.md`
Depends on: RCLD 06
Current checkpoint: `step_079`

## Purpose

Implement the sole process-execution boundary, immutable generated evidence
storage, pinned adapter invocation, inventory validation, reconciliation wiring,
and representative discovery fixtures.

## Scope Boundary

Only `eqm_runner` launches processes. Local process controls do not claim
generic sandboxing. Container support validates and uses only guarantees the
backend can enforce. CI-delegated execution imports trusted results and does
not execute locally. Adapters are pinned out-of-process programs, never dynamic
libraries, WASM plugins, or scripts embedded in manifests.

## Definition Of Green

- No manifest or binding can inject a shell command.
- Every process uses an argv array, confined cwd, empty-plus-allowlist
  environment, explicit timeout, bounded output, and cancellation cleanup.
- Typed placeholder substitution preserves one argument per value.
- Runner and adapter definitions have deterministic digests.
- Normalized results validate before immutable digest-named writes.
- Adapter failure and partial completeness remain explicit unknown/partial
  facts.
- Web, iOS, and Android discovery fixtures reconcile deterministically.

## Verification Lane

Run the standard locked workspace lane plus fake runner/adapter integration,
command-injection, environment, cwd/symlink, timeout, cancellation, output
flood, atomic-write, inventory, and framework fixture tests.

## Checkpoint Map

| Step | Scope | Definition of green | Commit intent |
| --- | --- | --- | --- |
| `step_077` | Resolve approved runner definitions into executable configurations and canonical digests. | Equivalent definitions hash identically; backend/resource/authority mismatches fail. | `feat(runner): resolve runner definitions` |
| `step_078` | Substitute typed selector, result_path, and target_root placeholders into argv elements. | Metacharacters remain literal argument data; unknown/duplicate-forbidden placeholders fail. | `feat(runner): substitute argv placeholders` |
| `step_079` | Implement bounded `local_process` execution with environment, cwd, timeout, output cap, and process-tree cancellation policy. | Fake processes prove success/failure/timeout/cancellation/cap behavior with no shell invocation. | `feat(runner): implement local process backend` |
| `step_080` | Read and validate current `eqm.test_result/v1` normalized output. | Passed/failed/skipped/filtered/retry/count/schema/limit fixtures classify exactly. | `feat(runner): read normalized test results` |
| `step_081` | Atomically persist immutable digest-named evidence below `.eqm/results/`. | Existing identical write is idempotent; collision/mismatch fails; partial writes and symlink escapes are prevented. | `feat(runner): write evidence results` |
| `step_082` | Validate and execute only digest-pinned container definitions with approved enforceable guarantees. | Floating images and unsupported network/filesystem/isolation claims fail; backend behavior is explicitly tested or unavailable. | `feat(runner): validate container backend` |
| `step_083` | Import CI-delegated results with exact CI reference, subject, producer, trust, and signature metadata. | Missing/insufficient trust, subject mismatch, replay, and schema failures are rejected without local execution. | `feat(runner): add ci delegated backend` |
| `step_084` | Add consolidated runner adversarial security tests. | Injection, environment leak, cwd escape, symlink, timeout, cancellation, output flood, and secret-redaction cases pass. | `test(runner): add security tests` |
| `step_085` | Invoke digest-pinned adapters through bounded JSON stdin/stdout. | Fake adapter success, malformed output, timeout, output cap, pin mismatch, and nonzero exit cases classify correctly. | `feat(discovery): invoke adapters` |
| `step_086` | Validate inventory schema, subject, target, completeness, entries, ordering, and limits. | Complete/partial/best-effort and malformed/duplicate/wrong-target fixtures pass. | `feat(discovery): validate inventories` |
| `step_087` | Feed validated inventories into pure exposure reconciliation. | Declared/discovered/unknown integration fixtures remain deterministic and preserve completeness. | `feat(discovery): wire reconciliation` |
| `step_088` | Add minimal SvelteKit filesystem discovery fixture. | Route inventory, exclusions, dynamic routes, ordering, and reconciliation tests pass. | `test(discovery): add sveltekit fixture` |
| `step_089` | Add SwiftUI build-export inventory fixture. | Current schema, subject, navigation entries, completeness, and reconciliation pass without parsing Swift source heuristically. | `test(discovery): add swiftui fixture` |
| `step_090` | Add Jetpack Compose build-export inventory fixture. | Current schema, subject, navigation entries, completeness, and reconciliation pass without parsing Kotlin source heuristically. | `test(discovery): add compose fixture` |

## Generated-State Rules

- Evidence results are generated state, not authored metadata.
- Adapter inventories are returned to the caller and are not persisted unless
  the caller explicitly requests a generated-state destination.
- Temporary files use confined generated-state directories and are cleaned on
  success and failure.
- Logs are capped and redacted before any diagnostic or evidence attachment.

## Reconciliation Rules

- Any claimed isolation guarantee must have a backend enforcement test.
- Unsupported container execution may be reported as unavailable; it may not
  silently fall back to `local_process`.
- RCLD 08 begins only after runner and adapter adversarial gates are green.

## Checkpoint Ledger

| Step range | Status | Result |
| --- | --- | --- |
| `step_077` | complete | Runner definitions resolve only under exact ID/revision, backend, resource, guarantee, and executable-digest authority; prepared programs retain verified repository or immutable locked identity, while a length-delimited canonical semantic projection produces stable definition digests independent of set/map insertion order and display-only extensions |
| `step_078` | complete | Invocation bindings require absolute UTF-8 NUL-free target/result paths and a bounded compact JSON-object selector; each typed placeholder replaces exactly one argv element, shell metacharacters remain literal data, and repeated execution-sensitive placeholders fail before launch |
| `step_079`-`step_090` | pending | - |
