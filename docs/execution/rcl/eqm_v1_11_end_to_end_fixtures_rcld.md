# EquivalenceMatrix V1 RCLD 11: End-To-End Fixtures

Status: complete; Steps 117-122 complete
Created: 2026-08-07
Updated: 2026-08-08
Mode: rcl-durable
Repository: `triesap/eqm`
Governing plan: `docs/execution/rcl/eqm_v1_multi_rcld.md`
Depends on: RCLD 10
Current checkpoint: none

## Purpose

Prove the complete v1 system with a standalone signup workspace spanning
SvelteKit web, SwiftUI iOS, and Jetpack Compose Android, exact release-gate
subjects, and reviewed stable outputs in every public format.

## Scope Boundary

Fixtures are minimal test artifacts, not generated application code, production
framework integrations, or real organizational policy. Owners, trust roots,
build subjects, flags, and release identities are synthetic and visibly marked
for tests only.

## Definition Of Green

- The signup graph includes capability, journey, surfaces, exact-pinned
  fragment, atomic requirements, three target bindings, policy, profiles,
  runners, inventory, facts, evidence, waivers, and releases.
- Web, iOS, and Android artifacts are idiomatic independent fixture shapes.
- Pass, fail, stale, missing, unstable, unknown, waived, conditional, and trust
  cases are represented.
- Full CLI and engine evaluation produces stable reviewed human, JSON, SARIF,
  and Markdown goldens.
- Positive fixtures contain no placeholder digest or invalid current schema.

## Verification Lane

Run the standard locked workspace lane plus fixture loading, structure,
discovery, reconciliation, verification, conformance, equivalence, release,
CLI, MCP read-tool, golden, and deterministic repeated-output tests.

## Checkpoint Map

| Step | Scope | Definition of green | Commit intent |
| --- | --- | --- | --- |
| `step_117` | Add complete current-schema signup workspace with capability, journey, surfaces, fragment, requirements, targets, policies, profiles, runners, bindings, and real fragment digest. | Loader, graph, obligations, conformance, equivalence, and canonical digest tests pass. | `test(eqm): add signup fixture workspace` |
| `step_118` | Add minimal SvelteKit routes/artifacts, binding, inventory, and normalized evidence fixture. | Structure, discovery, reachability, evidence, and target conformance pass for web. | `test(eqm): add web fixture artifacts` |
| `step_119` | Add minimal SwiftUI flow/artifacts, build-export inventory, binding, and normalized evidence fixture. | Structure, inventory, reachability, evidence, and target conformance pass for iOS. | `test(eqm): add ios fixture artifacts` |
| `step_120` | Add minimal Compose flow/artifacts, build-export inventory, binding, and normalized evidence fixture. | Structure, inventory, reachability, evidence, and target conformance pass for Android. | `test(eqm): add android fixture artifacts` |
| `step_121` | Add exact release records, runtime facts, evidence, trust, waiver, and pass/fail/unknown release cases. | End-to-end release CLI and engine cases bind exact builds and match exit/status tables. | `test(eqm): add release gate fixture` |
| `step_122` | Add reviewed human, JSON, SARIF, and Markdown goldens for all public commands and representative MCP reads. | Locale/order/path normalization and repeated-output tests are byte-identical; updates require explicit review. | `test(eqm): add golden outputs` |

## Fixture Integrity Rules

- Every generated expected digest must also have an independently specified
  input/vector; the implementation cannot be the sole oracle.
- Synthetic signatures use committed test keys labeled non-production.
- Fixture paths are repository-relative and portable.
- No fixture imports a real application repository or contains an individual
  user identifier.

## Reconciliation Rules

- A fixture workaround may not weaken production validation.
- Framework artifacts remain minimal and are not maintained as example apps.
- RCLD 12 begins only after a clean repeated end-to-end run produces identical
  outputs.

## Checkpoint Ledger

| Step range | Status | Result |
| --- | --- | --- |
| `step_117` | complete | A standalone materialized signup corpus uses only current schemas and exact fragment pinning to define one capability, one journey, two surfaces, one fragment, atomic each-target and end-to-end requirements, three independently owned SvelteKit/SwiftUI/Compose targets, six complete identifier/OTP bindings, a closed profile, three-target policy, bounded runner, and an explicitly synthetic waiver. Resolution now recognizes exact fragment-derived requirement coordinates before expansion so bindings and waivers can cover them without weakening the exact pin check; isolated VCS loading, expansion, selection, and obligation derivation remain green. |
| `step_118` | complete | The web target contains a minimal idiomatic SvelteKit signup route whose email control and continuation state correspond to the binding, plus a current normalized Vitest evidence result with exact selector and one successful attempt; fixture tests parse the evidence through the production normalization boundary and assert the bound artifact shape, complementing the existing filesystem discovery/reconciliation corpus and aggregate structure/conformance lanes |
| `step_119` | complete | The iOS target contains an idiomatic SwiftUI form/navigation flow, a synthetic complete build-export inventory bound to the exact iOS target/build subject, and a current successful XCTest result whose selector matches the binding; fixture tests assert source shape, export target/completeness, and production evidence normalization alongside the existing SwiftUI inventory reconciliation lane |
| `step_120` | complete | The Android target contains a minimal state-driven Kotlin/Compose-shaped signup screen, a synthetic complete Compose build-export inventory bound to the exact Android target/build subject, and a current successful JUnit result matching the binding; fixture tests assert source state behavior, target/completeness, and production evidence normalization alongside the existing Compose reconciliation lane |
| `step_121` | complete | Exact checked-in release records retain independently recomputed canonical identities, while the exercised release fixture now parses real CLI invocations and materializes isolated Git repositories with exact source, build, artifact, profile, runtime-facts, evidence, policy, and release-record bindings. Strict runtime-facts parsing verifies size, schema, canonical digest, subject, expiry, ordering, and vocabulary. A test-only signed authority produces observed pass and fail outcomes, weaker verified trust produces unknown, and the same passing claims remain unknown through the production path with no configured external trust. Claimed trust is never promoted directly, and unsigned attestations make no conformance claim. |
| `step_122` | complete | The corrective suite executes all 20 non-MCP commands through the production dispatcher and compares each complete normalized JSON document and exit code byte-for-byte against the reviewed `commands.json` golden. Context human/JSON/Markdown and validate SARIF remain independently byte-compared, while `mcp serve` is exercised through real stdio initialize/list/read traffic. Fixed Git metadata and normalization limited to `evaluated_at` make consecutive runs byte-identical. |
