# Architecture and code-health contract

## Structure (R04–R06)

Use a modular Rust application, not a rewrite and not a fleet of microservices.

```text
CLI / MCP / optional native transport
                  |
          eqm_application
       explicit use-case orchestration
          |                 |
  pure eqm_engine       host adapters
          |             manifest / store / trust / runner
       eqm_domain

Versioned protocol DTOs map boundary data to validated domain inputs.
```

`eqm_domain` owns validated values and local invariants. `eqm_engine` owns pure resolution, obligation derivation, semantic evaluation, comparison and deterministic explanations. `eqm_manifest` owns authored TOML/discovery/canonical projection and explicit conversion. `eqm_protocol` owns versioned JSON/SARIF and transport shapes; deserialization is not verification. `eqm_runner` owns authorized process supervision. `eqm_mcp` is a permission-limited adapter. The CLI package name is `eqm`, located in `crates/eqm_cli`.

Add `eqm_application` as the shared effectful use-case boundary. Initial planning allocation A01 places `authority`, `storage`, `trust`, `acquisition`, `evaluation`, and `query` modules inside it where appropriate. Split additional crates only when dependency isolation or privilege/packaging needs prove useful. These module names are navigation suggestions to be reconciled with actual code, not pre-existing symbols.

The application layer may orchestrate manifest/protocol/runner adapters, but it must not depend on CLI or MCP. Avoid a cycle where moving PreparedSession also moves its CLI-owned MCP-conversion helper into the application crate. Transport mapping stays outside the pure evaluator. No domain/engine dependency may introduce I/O implicitly.

## Two distinct operations (R05)

Acquisition is effectful: explicit source inspection, authorized worker execution, authorized import or permitted trust-material retrieval. Preparation captures the results into immutable validated input records. Evaluation is pure over that frozen input and an injected time value. Querying a stored evaluation is not acquisition. Missing inputs produce explicit non-success rather than secretly fetching or running tools.

Use narrow interfaces where external behavior changes: evidence reader/writer, signature verifier, authority loader, source snapshot provider, executor, time provider at the host boundary. Do not create a trait for every local struct. Avoid a universal manager with unrestricted access to all capabilities.

## Snapshot construction (R11, R22)

The engine's prepared input must have an enumerated required universe. A validated completeness boundary accounts for each coordinate as evaluated, explicitly not applicable, or unresolved. An immutable decision records that universe, exact input references, explanatory reasons and summary. Neither a public boolean nor deserializing an internal type may assert completeness, verified trust, or release authority.

A complete decision may contain failed, missing or unknown facts. The public API must preserve this distinction. Evaluation summaries are derived views, not mutable fields that callers can independently override.

## Code conventions (R50–R51)

Use validated newtypes for IDs, digest domains, revisions, subjects and counts. Prefer explicit enums/state machines for lifecycle distinctions and typed contextual errors with sources and machine codes. Avoid large constructor argument lists by validated parameter objects when they improve correctness. Do not use stringly typed truth, status, trust or scope values in domain logic.

Keep inputs and decisions immutable; share read-only data rather than repeatedly cloning entire graphs. Preserve deterministic ordering. No ambient current directory, environment inheritance, global mutable state, implicit network access, hidden runtime, or internal SystemTime reads in pure evaluation.

Retain the repo's panic/unwrap/expect/unsafe restrictions in production. Tests may use repository-approved patterns, but test conveniences must not enter production factories or features. Do not add Default/Deserialize/public field escape hatches to completeness, trust, or authorization types. Newtypes and private constructors prevent accidents, not malicious code with equal process privilege.

Keep the pure core unsafe-free. Any unavoidable native/OS bridge must be narrowly isolated and documented, with safe external wrappers considered first. Update repository instructions and lint scopes explicitly before introducing an exception; never remove the workspace unsafe rule globally. A process boundary, not catch_unwind, provides crash isolation when required.

## Concurrency and lifecycle

Pure evaluation remains independently callable without an async runtime. Host I/O may be asynchronous where actual integration warrants it; do not mandate an unchosen runtime. Tasks have owners, bounded queues, explicit cancellation, and shutdown joining/cleanup. Do not detach background work that can outlive authority or expose half-published results. Capture publication/commit semantics for cancellation races rather than guessing them from timing.

## Incremental migration of the implementation

Create validated internal primitives and fixtures before routing production entry points. Replace duplicated CLI decision paths with the application service one command at a time. Keep the existing fail-closed release path until the new protected pipeline is fully connected. Remove old evaluators after equivalence/regression tests, not before. Do not maintain alternate production truth calculators behind convenience flags.

Feature flags, if needed, isolate optional host capabilities and native packaging. They never disable validation or turn absent security functionality into pass. Enumerate actual supported combinations and test them; do not invent a broad feature matrix before inspecting Cargo manifests.

## DEV-006 projection boundary allocation

S019 adds narrow validated domain projection carriers PolicySelectionInputV2 and
SelectedProfileInputV2; they certify local structural validity, not trust. Existing
engine SelectedProfile/SelectedPolicyProfiles and selection algorithms remain in
place. eqm_manifest serializes the carriers and finalized contract graph into
typed v2 component identities. eqm_application assembles them from one prepared
session and the existing selection request API. No manifest-to-engine/protocol
dependency or engine serialization/I/O dependency is introduced. The governing
DEV-006 amendment defines exact invariants and independent implementation slices.
