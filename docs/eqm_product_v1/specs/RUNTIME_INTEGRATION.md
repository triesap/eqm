# Rust and Kotlin Multiplatform integration

## Required scope (R01–R03, R07–R10, R48)

Initial EQM runs in developer tooling and CI. The Rust product CLI and the desktop/mobile KMP apps are targets under test. Test adapters and selected runtime observation producers bridge native tools to EQM's normalized evidence. These adapters do not make the business app depend on EQM's internal crates.

Actual product repositories, build tasks, platform minimums, desktop JVM/native choice, device matrix, provider APIs and compatibility windows were not supplied. Discover them read-only where accessible and record them before implementing real adapters. Repository-local synthetic fixtures are appropriate for validating EQM, but cannot be labeled proof of actual mobile persistence/sync behavior.

Rust: link the shared EQM application service into EQM's own CLI. A product CLI needs a direct Rust dependency only for an actual local conformance feature; otherwise use it as a producer/consumer in test tooling. Keep EQM's package/binary distinct from the product's CLI.

KMP: represent shared source sets separately from shipped target outputs. `commonMain` changes invalidate all dependent outputs. Native tests retain their frameworks and explicit selectors. Adapter discovery is not permission to execute build scripts. Result normalization binds exact build/selection/attempt IDs and expected inventory; missing cases cannot vanish.

## Product contracts to author first

Shared invariants: correct account/task association, authorization boundaries, valid transitions, evidence revision identity and operation idempotency. Mobile-specific contracts: offline durable capture, interrupted photo/video/voice workflows, permission denial, editable annotations/revisions and resumable/recoverable uploads. Desktop-specific contracts: correct projections, visible stale/offline state, review history, conflict visibility and paginated large-account views. CLI contracts: inspection/export/automation behavior actually supported by the product. Cross-runtime scenarios link these roles and providers.

These are acceptance themes approved in the conversation, not invented endpoint names, storage models, UI designs, conflict algorithms or legal guarantees. The product owner must define the exact conflict, deletion, authorization and supported-format rules; EQM can then test them.

## Canonical first scenario

1. With synthetic authorized account/task fixtures, capture business evidence offline on mobile.
2. Edit the approved annotation/revision metadata; terminate the application after a save acknowledgment.
3. Restart and verify the acknowledged state survives.
4. Reconnect and retry interrupted synchronization.
5. Verify desktop reviews the intended revision exactly once under the correct task.
6. Verify CLI inspection/export observes the same approved logical revision.
7. Assert no duplicate logical record, cross-account disclosure or silent loss of acknowledged state.

Extend with account switching, permission revocation, concurrent edits, interrupted uploads, stale projections, backend upgrades and older supported clients. Do not promise a particular consistency algorithm; the contract specifies permitted offline observations and post-reconciliation invariants.

## Optional native embedding (R49)

Only activate for an explicitly required local evaluation feature. A desktop sidecar is a valid option for interactive local use or fault containment, not an automatic daemon. A read-only versioned report consumer may suffice. If a sidecar is chosen, use restricted IPC, version negotiation, bounded messages, cancellation and lifecycle ownership; no unauthenticated localhost default and no per-row process spawn.

If an in-process bridge is chosen, define a small Kotlin-facing abstraction and platform-specific implementations. UniFFI's documented Kotlin path uses JNA; it is not automatically a Kotlin/Native iOS implementation. Kotlin/Native C interop is an alternative boundary to evaluate. Select one coherent strategy with a demonstrated cross-platform slice, not multiple experimental bridges.

The bridge contract covers explicit allocation/free ownership, closed handles, cancellation/completion races, safe threading, errors, batch APIs, numeric/time mapping, supported ABI versions and bounded inputs. Never expose Rust memory layouts/collections over a stable ABI. Keep unsafe code isolated and reviewed; catch_unwind is not isolation from aborting panics. Product capture/media payloads stay in product/platform code, not large buffers through EQM evaluation APIs.

The optional sequence is in implementation/OPTIONAL_NATIVE_COMMIT_SEQUENCE.md. Do not mark it complete or activate it simply because the mandatory EQM core is approved.
