# EquivalenceMatrix v1 local acceptance report

Date: 2026-08-08

Acceptance candidate: `d735f76f00286b28db5938ac0a65bdd14c549c15`

Version: `0.1.0`

Toolchain: `rustc 1.97.1 (8bab26f4f 2026-07-14)`, Cargo `1.97.1`,
`aarch64-apple-darwin`, LLVM `22.1.6`; pinned coverage/fuzz toolchain
`nightly-2026-07-16` with Cargo `1.99.0-nightly`; `cargo-llvm-cov 0.8.7`,
`cargo-mutants 27.1.0`, `cargo-fuzz 0.13.2`, `cargo-audit 0.22.1`, and
`cargo-deny 0.19.8`.

Root `Cargo.lock` SHA-256:
`518ed82d69e46aadb73ebd4c0edef1cfb8c48da5d5ab923223ad6ee9d55b662e`

## Scope and traceability

`scripts/validate_authority.sh` reconciled 31 authority checksums, 140
decisions/requirements, 134 ordered checkpoints, and 140 traceability rows.
Direct evidence is carried by the child RCLD ledgers, crate tests, generated
schemas, the standalone signup corpus, real CLI/MCP entry-point goldens, and
the mandatory scripts under `scripts/`. The approved architecture adaptations
remain recorded in ADRs 0001-0014; no additional compatibility deviation was
introduced.

Aggregate digests use sorted paths plus each file's SHA-256. The 21 schemas
digest to
`65cb9ef57487ba446d77359370e09f2a985ba1aaa4bb60d562b86fded72ae87a`,
the 37-file signup fixture digests to
`913b2cbd33b13fe4349edf1c74e8d9679cc46e03adb7b0f51381c0bb7fc09cd5`,
and its seven reviewed golden files digest to
`f31908def2e2dc7682668b1721cb09debd75dca17cda8390204d911baa09759b`.

## Verification evidence

- `cargo extbuild run -- bash scripts/verify.sh` passed on the acceptance
  candidate. The aggregate ran authority and traceability validation, all 12
  mapped adversarial security cases, no-compat negative self-tests, byte-clean
  generation, 21-schema URI parity, locked formatting/check/test/clippy/docs,
  dependency and license gates, real end-to-end CLI/MCP/release fixtures,
  coverage, mutation, fuzz, performance/resource, reproducible packaging,
  clean generated-state, and diff checks.
- Real command coverage exercises all 20 non-MCP commands through the shared
  production dispatcher against deterministic fixtures. Complete normalized
  JSON bytes and exit codes are golden-tested; context human/JSON/Markdown,
  validate SARIF, and real MCP stdio responses are separately byte-tested.
- Properties used seed `0x45514d313233` for 512 bounded cases. Digest/order,
  non-success preservation, monotonicity, waiver, and affected-analysis
  properties passed.
- Fuzzing ran 1,000 iterations against each of seven production boundaries:
  TOML, protocol, adapter, inventory, evidence, canonicalization, and bounded
  graph. All 7,000 iterations passed without a crash, panic, timeout,
  nondeterminism failure, or retained artifact.
- The critical evaluation-core coverage gate measured `conformance`,
  `monotonicity`, and `release`: 1,205 of 1,242 lines (97.02%) and 146 of 162
  branches (90.12%), exceeding the hard 90% line and 85% branch thresholds.
- The critical mutation campaign generated 76 mutants: 69 were caught, one
  timed out and therefore counted as killed, six were unviable, and zero were
  missed. All 70 viable mutants were killed (100%), exceeding the hard 80%
  threshold.
- The 10,000-unit/100,000-requirement production benchmark measured 904 ms
  cold validation, 15 microseconds warm context, 1 ms affected recomputation,
  463 ms canonicalization of 20,950,097 bytes, and 367,738,880-byte sampled
  peak RSS on 12 logical CPUs. Its fixture digest was
  `sha256:f920d68ca74b5994fdfa51e38fa6303e587e80190ccd753113f591d21099f969`.
- `cargo audit --deny warnings` reported no vulnerable or yanked dependency.
  `cargo deny check` passed advisories, bans, license policy, and crates.io-only
  sources; the allowed `syn` 2/3 duplication remains visible as a warning.
- The full aggregate passed on macOS. Strict Clippy checks with all targets,
  the locked dependency graph, and warnings denied also passed locally for
  `x86_64-unknown-linux-gnu` and `x86_64-pc-windows-msvc`. The pinned CI
  workflow defines Linux, macOS, and Windows compilation, test, and lint jobs;
  no remote CI run is claimed by this local report.
- The three-target signup graph, web/iOS/Android obligations and artifacts,
  immutable evidence, exact release records, pass/fail/unknown release gates,
  output formats, pure current-version MCP framing, default denial, and audited
  authorized dry-run all passed through production entry points.

## Package evidence

The package gate produced two byte-identical, duplicate-free 27-file archives
containing `bin/eqm`, 21 current schemas, README, dual licenses, SPDX 2.3 SBOM,
and provenance inputs. The SBOM contains 71 package entries. A separate probe
of the same candidate produced archive SHA-256
`30f3c57d744730d2b5aa32a56748415523b12751a2d96ef3b29abb2a77e64d6b`.
The provenance binds source commit
`d735f76f00286b28db5938ac0a65bdd14c549c15`, the root lock digest above,
builder `local-dry-run`, and `production_signature: false`. The package scanner
found no forbidden compatibility identity.

## Remaining organizational inputs and non-claims

Real CODEOWNERS/security owner, a protected baseline, trusted CI/signing
identity, pilot repository roots/build commands, evidence retention policy,
namespace control, and legal/name clearance remain open. This report therefore
establishes local v1 implementation acceptance only. It does not claim package
publication, production release readiness, legal clearance, pilot validation,
organizational approval, complete discovery, application security, a remote CI
result, or a production signature. No push, publication, signing, deployment,
credential change, or other remote mutation occurred.
