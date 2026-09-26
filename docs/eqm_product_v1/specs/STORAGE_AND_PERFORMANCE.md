# Persistence, lifecycle and performance

## Storage model (R35–R37)

Use immutable content-addressed objects and a transactional local metadata/index store. The approved initial index direction is SQLite. Keep trusted release storage outside candidate-writable mounts; development `.eqm/` remains ignored generated state. Do not assume a developer-owned directory is a hostile-worker security boundary.

Publication protocol: write/validate in protected staging; durably install the immutable object; commit the index transaction referencing installed objects; expose the committed record. A crash before indexing can leave an orphan. A visible completed record must never reference bytes that were not safely published. Test the actual file/directory durability guarantees on supported OS/filesystem combinations.

Handle concurrent directory initialization, duplicate installation, transaction contention and idempotency. Distinguish raw-object collision, equivalent semantic transport and conflicting attempt identity. Immutable objects do not prevent legitimate extra raw encodings, nor permit contradictory attempts to overwrite one another.

Store enough immutable manifests/receipts and metadata to rebuild indexes without minting trust or losing conflicts. Validate referenced objects when reopening/replaying. A missing or corrupt object produces a bounded explicit unavailable result, not fabricated history. Use short read snapshots for coherent decisions; avoid long-lived transactions across native test execution.

## SQLite operational contract (R36)

Pin and audit an implementation containing SQLite's documented WAL-reset corruption fix. The upstream fix is in 3.51.3 and later or documented backports; this is a fix floor, not an instruction to pin that old release forever. Verify the actually linked runtime, not only a wrapper crate's version. WAL is for a local host, not network-filesystem or cross-device synchronization. One writer at a time requires bounded contention handling and explicit checkpoint ownership. Decide/test durability settings, transaction atomicity, busy behavior, crash recovery and backups.

Configuration/version evidence belongs in the implementation ADR and build reports. Current library/feature names and pin versions must be discovered and audited before dependency commits, with explicit lockfile changes.

## Retention and GC (R37)

Define classes for temporary local work, long-lived release evidence, original signed payloads, verification material, logs and attachments. Exact periods require owner policy. Immutable does not mean retain forever. GC respects running acquisitions, active readers, snapshots, archive references and release decisions; use reachability/leases appropriate to the store and test races. Reclaim orphan staging only after proving it is not live.

Prefer synthetic product data for conformance tests. Restrict sensitive attachments/log access and do not put actual business media or account secrets into public fixtures. Encryption/access policies are an operator decision, not an unapproved cryptographic subsystem to invent in this module.

## Historical replay versus current acceptance (R38)

Replay uses original input/evaluator/trust/time identities to reproduce the historical decision. Current acceptance checks present authorized trust/freshness/revocation/deployment context. Do not rewrite history on key rotation or expiry, and do not treat historical acceptance as perpetual permission. Missing archived material or unsupported evaluator versions are explicit replay limitations.

## Incremental evaluation (R39–R43)

Stage caches separately: parsing/canonical projections, prepared graph/obligations, verified inputs and time-sensitive decisions. Keys cover relevant subject/authority/profile/specification/binding/runner/adapter/trust/evaluator identities. Record validity boundaries for time-based inputs and invalidate on trust/configuration/authority changes. Do not use wall-clock buckets that permit stale success across a boundary.

Build reverse dependencies from source components and shared KMP source sets to targets, and from contracts/profiles/runners/adapters/trust to obligations. Handle renames/deletions and unclassified paths conservatively. `verify --affected` must use the result rather than simply loading a baseline. A cache/index miss expands or recomputes work; it does not remove obligations.

Batch tests by compatible runner/target/device/selection. Multiple requirements may reference one execution without counting it as independent evidence repeatedly. Bounded scheduling must respect device exclusivity, resources, cancellation and stable result ordering. Do not start a Gradle/Xcode process for every requirement or a native call for every displayed cell.

## Measurement

Measure cold preparation including canonicalization, derivation, evidence lookup, verification, full evaluation, serialization, storage contention/recovery and optional native overhead, plus warm/delta paths. Use realistic histories, profile combinations, cross-target scenarios, large matrices and host distributions. Record sample methodology, percentiles and peak memory on declared hardware. The baseline benchmark constants are not measured achievements from this conversation.

Keep a simple reference evaluation path. Property/differential tests compare optimized semantics to the reference for identical frozen inputs. Do not create a second independently evolving policy engine in the benchmark. Optimize bottlenecks only after correctness, with evidence of benefit and no relaxed validation.
