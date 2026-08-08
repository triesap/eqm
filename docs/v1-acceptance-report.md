# EquivalenceMatrix v1 local acceptance report

> **Withdrawn:** a corrective completion audit found that this report
> overstated several verification lanes. It is retained as historical input
> while Step 134 is active and must not be used as acceptance evidence. A
> replacement report will be generated only after every corrective finding and
> mandatory lane passes through its production boundary.

Date: 2026-08-08

Acceptance candidate base: `1c439e78f34c1b0a5bd09136a8b6799b797ff90f`

Version: `0.1.0`

Toolchain: `rustc 1.97.1 (8bab26f4f 2026-07-14)`, Cargo `1.97.1`,
`aarch64-apple-darwin`, LLVM `22.1.6`

Root `Cargo.lock` SHA-256:
`dad80843a2635b2696536bc0d49d2459a21ef3a527831159c7c5b4d3b998a1fc`

## Scope and traceability

All 134 ordered checkpoints and 140 authority/requirement rows are implemented
and reconciled by `scripts/validate_authority.sh`. Direct evidence is carried
by the child RCLD ledgers, crate unit/integration tests, current generated
schemas, the standalone signup corpus, CLI/MCP tests, and the mandatory scripts
under `scripts/`. No compatibility ADR deviation was introduced; the approved
architecture adaptations remain ADRs 0001-0014.

Aggregate digests (sorted path plus file digest): schemas
`65cb9ef57487ba446d77359370e09f2a985ba1aaa4bb60d562b86fded72ae87a`;
signup fixture
`9f330b7fa25ffbf2171017ba85738524697fa6e61e06fe0d6c3b7b3bc29d6671`;
reviewed goldens
`284ae433378d078e864fcc61196e62c04a47a75515cb40b0f452b60d99e0b2c4`.

## Verification evidence

- `cargo extbuild run -- bash scripts/verify.sh`: passed authority (31
  checksums, 140 decisions, 134 checkpoints), security matrix (12 cases),
  current-only negative self-tests, byte-clean generation, 21-schema URI
  parity, formatting, locked all-target check/tests, clippy with warnings
  denied, docs, supply-chain gates, clean generated state, and diff checks.
- `cargo llvm-cov --workspace --all-targets --locked --summary-only`: passed;
  77.43% regions, 78.54% functions, 83.38% lines.
- Properties: seed `0x45514d313233`, 512 bounded cases; digest/order and
  non-success properties passed with the exhaustive monotonicity, waiver, and
  affected suites.
- Fuzz: nightly sanitizer build plus 1,000 runs each for TOML, protocol,
  adapter, inventory, evidence, canonicalization, and bounded graph; zero
  crash, panic, timeout, nondeterminism, or retained artifact.
- Mutation: 3,134-workspace mutant inventory; deterministic critical shard
  covering digest/release/MCP generated 14 mutants. The first run found two
  digest survivors; exact regressions were added. Rerun: 10 caught, 4 timed
  out (killed), 0 missed, 0 material survivors. This is a targeted critical
  campaign, not a claim that all 3,134 mutants were executed.
- Security: all 12 mapped adversarial cases passed, including traversal,
  symlink, injection, environment, limits, redaction, signature/replay/subject,
  immutable collision, and default-denied audited MCP execution.
- Supply chain: `cargo audit --deny warnings` reported no vulnerable/yanked
  dependency; `cargo deny check` passed advisories, bans, explicit
  MIT/Apache-2.0/Unicode-3.0/Unlicense policy, and crates.io-only sources. The
  transitive `syn` 2/3 duplication is visible as an allowed warning.
- Scale probe: 10,000 units/100,000 requirements; cold 7 ms, warm context 0
  microseconds at timer precision, affected 0 ms, canonicalization 11 ms,
  estimated peak 11,800,004 bytes; fixture digest
  `sha256:7a2fb6a703244c4707bf71c32d6d49a922bb709404b7a0eb927cae02ce840d21`.
- Signup/CLI/MCP/release: three-target graph and obligations, web/iOS/Android
  artifact/evidence/export cases, exact release records and 0/1/7 table,
  byte-stable output formats, pure current-version MCP framing, default denial,
  and audited authorized dry-run all passed.

## Package evidence

Two clean dry runs produced byte-identical, duplicate-free archives with 27
files: `bin/eqm`, 21 current schemas, README, dual licenses, SPDX 2.3 SBOM, and
provenance inputs. The package scanner found no forbidden identity. The
provenance binds the source commit and root lock digest and states
`production_signature: false`; candidate archive SHA-256 is
`a14a127a1bae077a5231f0660233d7df4d9072c869a1942c344157e57a08f695`. No
publisher, registry, signer, credential, or production provenance was selected.

## Remaining organizational inputs and non-claims

Real CODEOWNERS/security owner, protected baseline, trusted CI/signing
identity, pilot repository roots/build commands, evidence retention policy,
namespace control, and legal/name clearance remain open. Therefore this report
establishes local v1 implementation acceptance only. It does not claim package
publication, production release readiness, legal clearance, pilot validation,
organizational approval, complete discovery, application security, or a
production signature. No push, publication, signing, deployment, credential
change, or other remote mutation occurred.
