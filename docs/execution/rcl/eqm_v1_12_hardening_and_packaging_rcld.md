# EquivalenceMatrix V1 RCLD 12: Hardening And Packaging

Status: in progress; Step 123 complete
Created: 2026-08-07
Updated: 2026-08-08
Mode: rcl-durable
Repository: `triesap/eqm`
Governing plan: `docs/execution/rcl/eqm_v1_multi_rcld.md`
Depends on: RCLD 11
Current checkpoint: `step_124`

## Purpose

Prove the implementation's semantic invariants, malformed-input resilience,
security boundaries, scale targets, schema parity, and reproducible release
packaging before operational closure.

## Scope Boundary

This RCLD adds tests, fuzz targets, benchmarks, verification tooling, and
packaging automation. It does not publish crates, binaries, schemas, tags,
SBOMs, provenance, or releases. Production signing remains blocked until real
organizational identity and trust inputs exist.

## Definition Of Green

- Required semantic properties hold under generated/permuted inputs.
- Parsers and protocol boundaries have buildable bounded fuzz targets and
  recorded smoke campaigns.
- Traversal, symlink, injection, signature, replay, timeout, flood, exhaustion,
  and cross-repository substitution tests pass.
- Required performance and memory targets are measured on documented fixtures
  and environment.
- Generated schemas and examples are byte-for-byte current.
- Clean release packaging produces the expected binaries, checksums, SBOM, and
  provenance/signing inputs without publishing.

## Verification Lane

Run the standard locked workspace lane plus property, fuzz smoke, adversarial,
cross-platform, coverage, mutation, benchmark, schema, dependency/advisory,
license, SBOM, provenance, and clean-package lanes introduced here. Tool
versions and campaign commands must be pinned and documented.

## Checkpoint Map

| Step | Scope | Definition of green | Commit intent |
| --- | --- | --- | --- |
| `step_123` | Add properties for policy monotonicity, digest stability, waiver non-satisfaction, affected conservatism, ordering, and unknown non-success. | Seeded deterministic properties pass with replayable failure cases and meaningful generation bounds. | `test(eqm): add property tests` |
| `step_124` | Add fuzz targets for TOML, protocol, adapter, inventory, evidence, canonicalization, and bounded graph inputs. | Every target compiles; pinned smoke campaigns complete with zero crash, panic, timeout, or nondeterminism. | `test(eqm): add fuzz targets` |
| `step_125` | Add consolidated adversarial tests for paths, symlinks, collisions, argv, env, output, signatures, replay, subjects, limits, and substitution. | All threat-model cases fail closed with correct diagnostics and no secret/path leakage. | `test(eqm): add security tests` |
| `step_126` | Add deterministic benchmark fixtures and benches for cold validate, warm context, affected, canonicalization, and evaluation memory/time. | 10k units/100k requirements cold validate under 10s, warm context under 250ms, affected under 2s, and peak memory under 1GiB on the documented 8-core reference class; regressions are reported reproducibly. | `test(eqm): add performance benchmarks` |
| `step_127` | Add mandatory combined generated-schema, example, URI, and parity verification. | Clean regeneration has no diff; every positive/negative instance has the expected result; no failure is suppressed. | `test(eqm): verify generated schemas` |
| `step_128` | Add clean binary/package build scripts, checksums, SBOM, provenance statement inputs, signing hooks, and multi-platform release workflow definitions. | Dry-run packages are reproducible, contain only intended files, pass no-legacy/schema checks, and do not publish or claim a production signature. | `chore(eqm): add release packaging` |

## Evidence Requirements

Committed summaries must record tool versions, fixture/corpus digests, seeds,
duration, environment class, commands, results, exclusions, and unresolved
findings without committing large raw outputs. A material surviving mutation,
crash, nondeterminism, high-severity dependency issue, or missed performance
target blocks the checkpoint unless authority explicitly narrows the claim.

## Reconciliation Rules

- Benchmarks are optimized only after profiling and without changing canonical
  bytes or rejection behavior.
- Signing hooks cannot silently create or select credentials.
- RCLD 13 begins only after clean package, schema, security, and required scale
  gates are green.

## Checkpoint Ledger

| Step range | Status | Result |
| --- | --- | --- |
| `step_123` | complete | A deterministic 512-case property lane records seed `0x45514d313233`, generates bounded digest and ordering inputs with replayable case indices, proves content identity stability and insertion-order independence, and explicitly proves waived, unknown, missing, and unstable facets are never satisfied; it consolidates the existing exhaustive policy-strength monotonicity, waiver non-satisfaction, affected conservatism, canonical digest, and stable matrix/order integration suites without introducing nondeterministic generators |
| `step_124`-`step_128` | pending | - |
